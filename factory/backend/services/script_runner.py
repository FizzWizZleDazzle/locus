"""Script execution service"""

import json
import subprocess
import uuid
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Any, Tuple


def run_script_once(script_path: Path, cwd: Path) -> Tuple[bool, Any]:
    """
    Run a script once and return (success, result_or_error)

    Returns:
        (True, problem_dict) on success
        (False, error_message) on failure
    """
    try:
        result = subprocess.run(
            ["python3", script_path.name],
            capture_output=True,
            text=True,
            timeout=10,
            cwd=str(cwd),
        )

        if result.returncode != 0:
            return False, result.stderr

        try:
            problem = json.loads(result.stdout.strip())
            return True, problem
        except json.JSONDecodeError as e:
            return False, f"Invalid JSON: {str(e)}"

    except subprocess.TimeoutExpired:
        return False, "Script timeout (10 seconds)"
    except Exception as e:
        return False, f"Execution error: {str(e)}"


def test_script_code(script_code: str, scripts_dir: Path) -> Dict[str, Any]:
    """Test a script by running it once (used for inline testing)"""
    temp_path = scripts_dir / "temp_test.py"

    try:
        temp_path.write_text(script_code)

        result = subprocess.run(
            ["python3", "temp_test.py"],
            capture_output=True,
            text=True,
            timeout=10,
            cwd=str(scripts_dir),
        )

        if result.returncode != 0:
            return {
                "success": False,
                "error": result.stderr,
                "stdout": result.stdout,
            }

        try:
            problem = json.loads(result.stdout.strip())

            required_fields = ["question_latex", "answer_key", "difficulty",
                             "main_topic", "subtopic", "grading_mode"]
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
            "error": "Script timeout (10 seconds)",
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


def mass_generate(
    scripts_dir: Path,
    count_per_script: int,
    staged_problems: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Run ALL saved scripts N times each and auto-stage all problems"""
    scripts = [s for s in scripts_dir.glob("*.py") if s.name != "temp_test.py"]

    total_generated = 0
    errors = []
    script_results = {}

    for script_path in scripts:
        script_name = script_path.stem
        generated_from_script = 0

        for i in range(count_per_script):
            success, result = run_script_once(script_path, scripts_dir)

            if success:
                problem = result
                problem["id"] = str(uuid.uuid4())
                problem["generated_at"] = datetime.utcnow().isoformat()
                staged_problems.append(problem)
                generated_from_script += 1
                total_generated += 1
            else:
                error_msg = result if isinstance(result, str) else str(result)
                errors.append(f"{script_name}[{i+1}]: {error_msg[:80]}")

        script_results[script_name] = generated_from_script
        print(f"Completed {script_name}: {generated_from_script}/{count_per_script}")

    return {
        "success": total_generated > 0,
        "total_generated": total_generated,
        "staged": len(staged_problems),
        "scripts_run": len(script_results),
        "per_script": script_results,
        "errors": errors[:30] if errors else None,
        "message": f"Mass generated {total_generated} problems from {len(script_results)} scripts"
    }
