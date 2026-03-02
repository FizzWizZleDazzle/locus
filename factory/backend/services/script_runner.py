"""Script execution service — supports Python and Julia generators"""

import json
import os
import subprocess
import uuid
from concurrent.futures import ProcessPoolExecutor, as_completed
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Any, Tuple

from config import SCRIPTS_PYTHON_DIR, JULIA_PROJECT

# Python utils dir — problem_utils.py and svg_utils.py live here
_SCRIPTS_PYTHON_DIR = str(SCRIPTS_PYTHON_DIR)

# Julia project root
_JULIA_PROJECT = str(JULIA_PROJECT)
_JULIA_SYSIMAGE = os.path.join(_JULIA_PROJECT, "sysimage.so")


def _script_env() -> dict:
    """Env for script subprocesses: adds factory backend to PYTHONPATH."""
    env = os.environ.copy()
    env["PYTHONPATH"] = _SCRIPTS_PYTHON_DIR + os.pathsep + env.get("PYTHONPATH", "")
    return env


def _detect_language(path: Path) -> str:
    """Return 'julia' or 'python' based on file extension."""
    return "julia" if path.suffix == ".jl" else "python"


def _julia_cmd() -> list:
    """Base Julia command with sysimage and project flags if available."""
    cmd = ["julia"]
    if os.path.isfile(_JULIA_SYSIMAGE):
        cmd += [f"--sysimage={_JULIA_SYSIMAGE}"]
    cmd += [f"--project={_JULIA_PROJECT}"]
    return cmd


def _script_cmd(script_path: Path) -> list:
    """Return the subprocess command for running a script."""
    if _detect_language(script_path) == "julia":
        return _julia_cmd() + [script_path.name]
    return ["python3", script_path.name]


def run_script_once(script_path: Path, cwd: Path) -> Tuple[bool, Any]:
    """
    Run a script once and return (success, result_or_error)

    Returns:
        (True, problem_dict) on success
        (False, error_message) on failure
    """
    lang = _detect_language(script_path)
    timeout = 30 if lang == "julia" else 10

    try:
        result = subprocess.run(
            _script_cmd(script_path),
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=str(cwd),
            env=_script_env(),
        )

        if result.returncode != 0:
            return False, result.stderr

        try:
            # For Julia scripts, output may be JSONL; take first line
            stdout = result.stdout.strip()
            first_line = stdout.split('\n')[0] if stdout else ""
            problem = json.loads(first_line)
            problem.setdefault("answer_type", "expression")
            problem.setdefault("calculator_allowed", "none")
            return True, problem
        except json.JSONDecodeError as e:
            return False, f"Invalid JSON: {str(e)}"

    except subprocess.TimeoutExpired:
        return False, f"Script timeout ({timeout} seconds)"
    except Exception as e:
        return False, f"Execution error: {str(e)}"


def run_julia_batch(
    script_path: Path, count: int, cwd: Path
) -> Tuple[List[Dict], List[str]]:
    """
    Run a Julia script once with --count N, parse JSONL output.
    Returns (problems, errors).
    """
    cmd = _julia_cmd() + [script_path.name, "--count", str(count)]
    problems = []
    errors = []

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=max(30, count),  # scale timeout with count
            cwd=str(cwd),
            env=_script_env(),
        )

        if result.returncode != 0:
            errors.append(result.stderr[:200])
            return problems, errors

        for i, line in enumerate(result.stdout.strip().split('\n')):
            line = line.strip()
            if not line:
                continue
            try:
                problem = json.loads(line)
                problem.setdefault("answer_type", "expression")
                problem.setdefault("calculator_allowed", "none")
                problem["id"] = str(uuid.uuid4())
                problem["generated_at"] = datetime.utcnow().isoformat()
                problems.append(problem)
            except json.JSONDecodeError as e:
                errors.append(f"Line {i+1}: Invalid JSON: {str(e)}")

    except subprocess.TimeoutExpired:
        errors.append(f"Script timeout ({max(30, count)}s)")
    except Exception as e:
        errors.append(f"Execution error: {str(e)}")

    return problems, errors


def test_script_code(script_code: str, scripts_dir: Path, language: str = "python") -> Dict[str, Any]:
    """Test a script by running it once (used for inline testing)"""
    ext = ".jl" if language == "julia" else ".py"
    temp_name = f"temp_test{ext}"
    temp_path = scripts_dir / temp_name

    is_julia = language == "julia"
    timeout = 30 if is_julia else 10

    try:
        temp_path.write_text(script_code)

        if is_julia:
            cmd = _julia_cmd() + [temp_name]
        else:
            cmd = ["python3", temp_name]

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=str(scripts_dir),
            env=_script_env(),
        )

        if result.returncode != 0:
            return {
                "success": False,
                "error": result.stderr,
                "stdout": result.stdout,
            }

        try:
            # Take first line for JSONL compatibility
            stdout = result.stdout.strip()
            first_line = stdout.split('\n')[0] if stdout else ""
            problem = json.loads(first_line)

            required_fields = ["question_latex", "answer_key", "difficulty",
                             "main_topic", "subtopic", "grading_mode",
                             "solution_latex"]
            missing = [f for f in required_fields if f not in problem]

            if missing:
                return {
                    "success": False,
                    "error": f"Missing fields: {', '.join(missing)}",
                    "output": result.stdout,
                }

            # Add defaults for new fields (backwards compatibility)
            if "answer_type" not in problem:
                problem["answer_type"] = "expression"
            if "calculator_allowed" not in problem:
                problem["calculator_allowed"] = "none"

            # Validate answer_type value
            valid_answer_types = ["expression", "numeric", "set", "tuple", "list",
                                 "interval", "inequality", "equation", "boolean",
                                 "word", "matrix", "multi_part"]
            if problem["answer_type"] not in valid_answer_types:
                return {
                    "success": False,
                    "error": f"Invalid answer_type '{problem['answer_type']}'. Must be one of: {', '.join(valid_answer_types)}",
                    "output": result.stdout,
                }

            # Validate calculator_allowed value
            valid_calculator_levels = ["none", "scientific", "graphing", "cas"]
            if problem["calculator_allowed"] not in valid_calculator_levels:
                return {
                    "success": False,
                    "error": f"Invalid calculator_allowed '{problem['calculator_allowed']}'. Must be one of: {', '.join(valid_calculator_levels)}",
                    "output": result.stdout,
                }

            return {
                "success": True,
                "problem": problem,
                "message": "Script executed successfully"
            }

        except json.JSONDecodeError as e:
            return {
                "success": False,
                "error": f"Invalid JSON: {str(e)}",
                "output": result.stdout,
            }

    except subprocess.TimeoutExpired:
        return {
            "success": False,
            "error": f"Script timeout ({timeout} seconds)",
        }
    except Exception as e:
        return {
            "success": False,
            "error": f"Execution error: {str(e)}",
        }
    finally:
        temp_path.unlink(missing_ok=True)


def run_script_multiple(
    script_path: Path,
    count: int,
    scripts_dir: Path
) -> Dict[str, Any]:
    """Run a saved script multiple times"""
    # Julia scripts: use batch mode (single process, N problems)
    if _detect_language(script_path) == "julia":
        problems, errors = run_julia_batch(script_path, count, scripts_dir)
        return {
            "success": len(problems) > 0,
            "problems": problems,
            "count": len(problems),
            "errors": errors[:10] if errors else None
        }

    # Python scripts: run N times (one process per problem)
    problems = []
    errors = []

    for i in range(count):
        success, result = run_script_once(script_path, scripts_dir)

        if success:
            problem = result
            problem["id"] = str(uuid.uuid4())
            problem["generated_at"] = datetime.utcnow().isoformat()
            problems.append(problem)
        else:
            error_msg = result if isinstance(result, str) else str(result)
            errors.append(f"Run {i+1}: {error_msg[:100]}")

    return {
        "success": len(problems) > 0,
        "problems": problems,
        "count": len(problems),
        "errors": errors[:10] if errors else None
    }


def _run_script_n_times(args: Tuple) -> Tuple[str, List[Dict], List[str]]:
    """Worker function: run a single script N times. Returns (name, problems, errors)."""
    script_path, scripts_dir, count = args
    script_path = Path(script_path)
    scripts_dir = Path(scripts_dir)
    script_name = script_path.stem

    # Julia: single batch invocation
    if _detect_language(script_path) == "julia":
        problems, errors = run_julia_batch(script_path, count, scripts_dir)
        return script_name, problems, errors

    # Python: N separate invocations
    problems = []
    errors = []

    for i in range(count):
        success, result = run_script_once(script_path, scripts_dir)
        if success:
            problem = result
            problem["id"] = str(uuid.uuid4())
            problem["generated_at"] = datetime.utcnow().isoformat()
            problems.append(problem)
        else:
            error_msg = result if isinstance(result, str) else str(result)
            errors.append(f"{script_name}[{i+1}]: {error_msg[:80]}")

    return script_name, problems, errors


def _discover_scripts(scripts_dir: Path) -> List[Path]:
    """Find all .py and .jl scripts, excluding temp files."""
    skip = {"temp_test.py", "temp_test.jl"}
    scripts = []
    for ext in ("*.py", "*.jl"):
        for s in scripts_dir.glob(ext):
            if s.name not in skip:
                scripts.append(s)
    return scripts


def mass_generate(
    scripts_dir: Path,
    count_per_script: int,
    staged_problems: List[Dict[str, Any]],
    max_workers: int = 8,
) -> Dict[str, Any]:
    """Run ALL saved scripts N times each in parallel and auto-stage all problems"""
    scripts = _discover_scripts(scripts_dir)

    total_generated = 0
    errors = []
    script_results = {}

    tasks = [(str(sp), str(scripts_dir), count_per_script) for sp in scripts]

    with ProcessPoolExecutor(max_workers=max_workers) as pool:
        futures = {pool.submit(_run_script_n_times, t): t[0] for t in tasks}

        for future in as_completed(futures):
            script_name, problems, errs = future.result()
            staged_problems.extend(problems)
            total_generated += len(problems)
            errors.extend(errs)
            script_results[script_name] = len(problems)
            print(f"Completed {script_name}: {len(problems)}/{count_per_script}")

    return {
        "success": total_generated > 0,
        "total_generated": total_generated,
        "staged": len(staged_problems),
        "scripts_run": len(script_results),
        "per_script": script_results,
        "errors": errors[:30] if errors else None,
        "message": f"Mass generated {total_generated} problems from {len(script_results)} scripts"
    }


_SQL_COLS = ("question_latex, answer_key, difficulty, main_topic, subtopic, "
             "grading_mode, answer_type, calculator_allowed, solution_latex, "
             "question_image, time_limit_seconds")


def _sql_esc(s) -> str:
    return s.replace("'", "''") if isinstance(s, str) else (s or '')


def _problem_to_sql_row(p: dict) -> str:
    tl = p.get('time_limit_seconds')
    return (
        f"('{_sql_esc(p['question_latex'])}', '{_sql_esc(p['answer_key'])}', "
        f"{p['difficulty']}, '{_sql_esc(p['main_topic'])}', '{_sql_esc(p['subtopic'])}', "
        f"'{_sql_esc(p['grading_mode'])}', '{_sql_esc(p.get('answer_type','expression'))}', "
        f"'{_sql_esc(p.get('calculator_allowed','none'))}', "
        f"'{_sql_esc(p.get('solution_latex',''))}', "
        f"'{_sql_esc(p.get('question_image',''))}', "
        f"{'NULL' if tl is None else int(tl)})"
    )


def mass_generate_to_file(
    scripts_dir: Path,
    count_per_script: int,
    output_path: Path,
    max_workers: int = 8,
) -> Dict[str, Any]:
    """Run ALL scripts N times each; write problems directly to SQL file."""
    scripts = _discover_scripts(scripts_dir)

    total_generated = 0
    errors: List[str] = []
    script_results: Dict[str, int] = {}
    tasks = [(str(sp), str(scripts_dir), count_per_script) for sp in scripts]
    first_row = True

    with open(output_path, 'w') as f:
        f.write(f"-- Generated by Locus Factory\n")
        f.write(f"-- Generated at: {datetime.utcnow().isoformat()}\n")
        f.write(f"-- Scripts: {len(scripts)}\n\n")
        f.write(f"INSERT INTO problems ({_SQL_COLS}) VALUES\n")

        with ProcessPoolExecutor(max_workers=max_workers) as pool:
            futures = {pool.submit(_run_script_n_times, t): t[0] for t in tasks}
            for future in as_completed(futures):
                script_name, problems, errs = future.result()
                for p in problems:
                    if not first_row:
                        f.write(",\n")
                    f.write(_problem_to_sql_row(p))
                    first_row = False
                total_generated += len(problems)
                errors.extend(errs)
                script_results[script_name] = len(problems)
                print(f"Completed {script_name}: {len(problems)}/{count_per_script}")
            f.flush()

        f.write(";\n" if total_generated > 0 else "-- No problems generated\n")

    return {
        "success": total_generated > 0,
        "total_generated": total_generated,
        "scripts_run": len(script_results),
        "per_script": script_results,
        "errors": errors[:30] if errors else None,
        "output_file": output_path.name,
        "message": f"Wrote {total_generated} problems to {output_path.name}",
    }
