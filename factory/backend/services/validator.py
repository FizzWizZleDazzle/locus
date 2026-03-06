"""Script validation service — runs a script multiple times and checks output quality."""

import re
import xml.etree.ElementTree as ET
from typing import Dict, Any, List
from pathlib import Path

from services.script_runner import run_julia_batch, run_script_once, test_script_code

_VALID_ANSWER_TYPES = {
    "expression", "numeric", "set", "tuple", "list",
    "interval", "inequality", "equation", "boolean",
    "word", "matrix", "multi_part",
}
_VALID_GRADING_MODES = {"equivalent", "factor", "expand"}


# ---------------------------------------------------------------------------
# Individual check functions
# ---------------------------------------------------------------------------

def _check_robustness(problems: list, errors: list, total_runs: int) -> Dict[str, Any]:
    """Check 1: error rate must be <= 10%."""
    error_rate = len(errors) / total_runs if total_runs > 0 else 1.0
    passed = error_rate <= 0.10
    return {
        "name": "robustness",
        "passed": passed,
        "score": round((1 - error_rate) * 100, 1),
        "details": f"{len(problems)}/{total_runs} succeeded ({error_rate*100:.0f}% error rate)",
    }


def _check_answer_format(problems: list) -> Dict[str, Any]:
    """Check 2: answer_key matches declared answer_type format rules."""
    if not problems:
        return {"name": "answer_format", "passed": False, "score": 0, "details": "No problems to check"}

    valid = 0
    issues = []
    for i, p in enumerate(problems):
        at = p.get("answer_type", "expression")
        ak = str(p.get("answer_key", ""))
        ok = True
        reason = ""

        if at == "boolean":
            if ak.lower() not in {"true", "false", "yes", "no", "t", "f", "1", "0"}:
                ok, reason = False, f"boolean answer '{ak}' not in valid set"
        elif at == "numeric":
            # No free variables allowed (letters except e for sci notation, pi, E, I)
            cleaned = re.sub(r'\b(pi|E|I|e)\b', '', ak)
            if re.search(r'[a-df-hj-zA-DF-HJ-Z]', cleaned):
                ok, reason = False, f"numeric answer '{ak[:40]}' contains variables"
        elif at == "set":
            if not ak.strip():
                ok, reason = False, "set answer is empty"
        elif at == "tuple":
            # At least 2 comma-separated elements
            parts = [x.strip() for x in ak.strip("() ").split(",") if x.strip()]
            if len(parts) < 2:
                ok, reason = False, f"tuple needs >= 2 elements, got {len(parts)}"
        elif at == "interval":
            if not (re.search(r'(open|closed):', ak) or '{' in ak):
                ok, reason = False, f"interval format invalid: '{ak[:40]}'"
        elif at == "inequality":
            if not re.search(r'[<>≤≥]', ak):
                ok, reason = False, f"inequality missing relational operator: '{ak[:40]}'"
        elif at == "equation":
            # Exactly one standalone = (not <=, >=, ==)
            standalone_eq = re.findall(r'(?<![<>=!])=(?!=)', ak)
            if len(standalone_eq) != 1:
                ok, reason = False, f"equation needs exactly 1 '=', found {len(standalone_eq)}"
        elif at == "matrix":
            if not (ak.startswith("[") and "[[" in ak.replace(" ", "")):
                ok, reason = False, f"matrix needs [[row], ...] format"
        elif at == "multi_part":
            if "|||" not in ak:
                ok, reason = False, f"multi_part needs ||| separator"
        elif at == "expression":
            # Balanced parentheses, non-empty
            if not ak.strip():
                ok, reason = False, "expression answer is empty"
            elif ak.count("(") != ak.count(")"):
                ok, reason = False, "unbalanced parentheses"

        if ok:
            valid += 1
        else:
            issues.append(f"Problem {i+1}: {reason}")

    score = (valid / len(problems)) * 100 if problems else 0
    return {
        "name": "answer_format",
        "passed": score >= 90,
        "score": round(score, 1),
        "details": f"{valid}/{len(problems)} answers match format" + (f"; issues: {'; '.join(issues[:3])}" if issues else ""),
    }


def _check_latex_quality(problems: list) -> Dict[str, Any]:
    """Check 3: LaTeX quality — balanced delimiters, no code artifacts."""
    if not problems:
        return {"name": "latex_quality", "passed": False, "score": 0, "details": "No problems"}

    valid = 0
    issues = []
    code_patterns = [r'\bprint\s*\(', r'\bdef\s+', r'\bfunction\s+', r'\bimport\s+', r'\breturn\s+']

    for i, p in enumerate(problems):
        q = p.get("question_latex", "")
        ok = True
        reason = ""

        if not q.strip():
            ok, reason = False, "empty question"
        elif q.count("(") != q.count(")") or q.count("{") != q.count("}") or q.count("[") != q.count("]"):
            ok, reason = False, "unbalanced delimiters"
        else:
            for pat in code_patterns:
                if re.search(pat, q):
                    ok, reason = False, f"code artifact detected"
                    break

        if ok:
            valid += 1
        else:
            issues.append(f"Problem {i+1}: {reason}")

    score = (valid / len(problems)) * 100
    return {
        "name": "latex_quality",
        "passed": score >= 90,
        "score": round(score, 1),
        "details": f"{valid}/{len(problems)} pass LaTeX checks" + (f"; issues: {'; '.join(issues[:3])}" if issues else ""),
    }


def _check_diversity(problems: list) -> Dict[str, Any]:
    """Check 4: unique answers and unique questions ratio >= 50%."""
    if len(problems) < 2:
        return {"name": "diversity", "passed": len(problems) > 0, "score": 100 if problems else 0, "details": "Too few problems to check diversity"}

    answers = [str(p.get("answer_key", "")) for p in problems]
    questions = [p.get("question_latex", "") for p in problems]
    unique_a = len(set(answers)) / len(answers) * 100
    unique_q = len(set(questions)) / len(questions) * 100
    passed = unique_a >= 50 and unique_q >= 50
    return {
        "name": "diversity",
        "passed": passed,
        "score": round(min(unique_a, unique_q), 1),
        "details": f"Unique answers: {unique_a:.0f}%, unique questions: {unique_q:.0f}%",
    }


def _check_difficulty_spread(problems: list) -> Dict[str, Any]:
    """Check 5: difficulty values should have non-trivial spread."""
    if not problems:
        return {"name": "difficulty_spread", "passed": False, "score": 0, "details": "No problems"}

    diffs = [p.get("difficulty", 0) for p in problems]
    lo, hi = min(diffs), max(diffs)
    mean = sum(diffs) / len(diffs)
    spread = hi - lo
    passed = spread >= 20 or len(set(diffs)) > 1
    return {
        "name": "difficulty_spread",
        "passed": passed,
        "score": min(100, spread),
        "details": f"min={lo}, max={hi}, mean={mean:.0f}, spread={spread}",
    }


def _check_svg_validity(problems: list) -> Dict[str, Any]:
    """Check 6: SVG images are valid XML with viewBox (or valid compressed s1: format)."""
    svg_problems = [p for p in problems if p.get("question_image")]
    if not svg_problems:
        return {"name": "svg_validity", "passed": True, "score": 100, "details": "No SVGs to check (OK)"}

    valid = 0
    issues = []
    for i, p in enumerate(svg_problems):
        img = p["question_image"]
        if img.startswith("s1:"):
            # Compressed format — just check for ~v token (viewBox)
            if "~v" in img:
                valid += 1
            else:
                issues.append(f"SVG {i+1}: compressed but missing ~v (viewBox)")
        else:
            try:
                root = ET.fromstring(img)
                if root.get("viewBox") or root.get("viewbox"):
                    valid += 1
                else:
                    issues.append(f"SVG {i+1}: missing viewBox")
            except ET.ParseError as e:
                issues.append(f"SVG {i+1}: invalid XML: {str(e)[:50]}")

    score = (valid / len(svg_problems)) * 100 if svg_problems else 100
    return {
        "name": "svg_validity",
        "passed": score >= 90,
        "score": round(score, 1),
        "details": f"{valid}/{len(svg_problems)} SVGs valid" + (f"; {'; '.join(issues[:3])}" if issues else ""),
    }


def _check_gradeability(problems: list) -> Dict[str, Any]:
    """Check 7: valid grading_mode, answer_type, non-empty answer_key for expression/numeric."""
    if not problems:
        return {"name": "gradeability", "passed": False, "score": 0, "details": "No problems"}

    valid = 0
    issues = []
    for i, p in enumerate(problems):
        gm = p.get("grading_mode", "")
        at = p.get("answer_type", "")
        ak = str(p.get("answer_key", ""))
        ok = True
        reason = ""

        if gm not in _VALID_GRADING_MODES:
            ok, reason = False, f"invalid grading_mode '{gm}'"
        elif at not in _VALID_ANSWER_TYPES:
            ok, reason = False, f"invalid answer_type '{at}'"
        elif at in ("expression", "numeric") and not ak.strip():
            ok, reason = False, f"empty answer_key for {at}"

        if ok:
            valid += 1
        else:
            issues.append(f"Problem {i+1}: {reason}")

    score = (valid / len(problems)) * 100
    return {
        "name": "gradeability",
        "passed": score >= 90,
        "score": round(score, 1),
        "details": f"{valid}/{len(problems)} gradeable" + (f"; {'; '.join(issues[:3])}" if issues else ""),
    }


# ---------------------------------------------------------------------------
# Main validation entry point
# ---------------------------------------------------------------------------

def validate_script(script: str, language: str, runs: int, scripts_dir: Path) -> Dict[str, Any]:
    """
    Run a script `runs` times and validate the output across 7 categories.
    Returns a structured validation report.
    """
    ext = ".jl" if language == "julia" else ".py"
    temp_name = f"temp_validate{ext}"
    temp_path = scripts_dir / temp_name

    problems: List[Dict] = []
    errors: List[str] = []

    try:
        temp_path.write_text(script)

        if language == "julia":
            from services.script_runner import run_julia_batch
            problems, errors = run_julia_batch(temp_path, runs, scripts_dir)
        else:
            from services.script_runner import run_script_once
            for i in range(runs):
                success, result = run_script_once(temp_path, scripts_dir)
                if success:
                    problems.append(result)
                else:
                    errors.append(f"Run {i+1}: {str(result)[:100]}")
    except Exception as e:
        errors.append(f"Execution error: {str(e)}")
    finally:
        temp_path.unlink(missing_ok=True)

    # Run all 7 checks
    categories = [
        _check_robustness(problems, errors, runs),
        _check_answer_format(problems),
        _check_latex_quality(problems),
        _check_diversity(problems),
        _check_difficulty_spread(problems),
        _check_svg_validity(problems),
        _check_gradeability(problems),
    ]

    overall_pass = all(c["passed"] for c in categories)

    return {
        "overall_pass": overall_pass,
        "categories": categories,
        "total_runs": runs,
        "successful_runs": len(problems),
        "error_rate": round(len(errors) / runs, 3) if runs > 0 else 1.0,
        "sample_problems": problems[:3],
        "errors": errors[:10],
    }
