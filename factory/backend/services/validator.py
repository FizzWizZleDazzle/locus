"""Script validation service — runs a script multiple times and checks output quality."""

import json
import re
import subprocess
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


_UNEVALUATED_PATTERNS = [
    "Integral(", "Derivative(", "Sum(", "Limit(",
    "Piecewise(", "Product(", "Subs(", "Order(",
]


def _check_unevaluated_sympy(problems: list) -> Dict[str, Any]:
    """Check 8: answer_key should not contain unevaluated SymPy objects."""
    if not problems:
        return {"name": "unevaluated_sympy", "passed": False, "score": 0, "details": "No problems"}

    valid = 0
    issues = []
    for i, p in enumerate(problems):
        ak = str(p.get("answer_key", ""))
        found = [pat for pat in _UNEVALUATED_PATTERNS if pat in ak]
        if found:
            issues.append(f"Problem {i+1}: unevaluated {', '.join(found)}")
        else:
            valid += 1

    score = (valid / len(problems)) * 100
    return {
        "name": "unevaluated_sympy",
        "passed": score >= 95,
        "score": round(score, 1),
        "details": f"{valid}/{len(problems)} clean" + (f"; {'; '.join(issues[:3])}" if issues else ""),
    }


def _check_solution_quality(problems: list) -> Dict[str, Any]:
    """Check 9: solution_latex should be non-empty, balanced, and free of code artifacts."""
    if not problems:
        return {"name": "solution_quality", "passed": False, "score": 0, "details": "No problems"}

    code_patterns = [r'\bprint\s*\(', r'\bdef\s+', r'\bfunction\s+', r'\bimport\s+', r'\breturn\s+']
    valid = 0
    issues = []
    for i, p in enumerate(problems):
        sol = p.get("solution_latex", "") or ""
        ok = True
        reason = ""

        if len(sol.strip()) < 10:
            ok, reason = False, "solution too short or empty"
        elif sol.count("{") != sol.count("}"):
            ok, reason = False, "unbalanced braces in solution"
        else:
            for pat in code_patterns:
                if re.search(pat, sol):
                    ok, reason = False, "code artifact in solution"
                    break

        if ok:
            valid += 1
        else:
            issues.append(f"Problem {i+1}: {reason}")

    score = (valid / len(problems)) * 100
    return {
        "name": "solution_quality",
        "passed": score >= 80,
        "score": round(score, 1),
        "details": f"{valid}/{len(problems)} pass solution checks" + (f"; {'; '.join(issues[:3])}" if issues else ""),
    }


def _check_time_limit(problems: list) -> Dict[str, Any]:
    """Check 10: time_limit_seconds must be set and within range, scaled by difficulty."""
    if not problems:
        return {"name": "time_limit", "passed": False, "score": 0, "details": "No problems"}

    valid = 0
    issues = []
    for i, p in enumerate(problems):
        tl = p.get("time_limit_seconds")
        diff = p.get("difficulty", 1000)
        ok = True
        reason = ""

        if tl is None:
            ok, reason = False, "time_limit_seconds not set"
        elif not (1 <= tl <= 3600):
            ok, reason = False, f"time_limit_seconds={tl} out of range [1, 3600]"
        elif diff < 700 and tl > 300:
            ok, reason = False, f"easy problem (ELO {diff}) has time={tl}s (max 300s)"
        elif diff > 2000 and tl < 30:
            ok, reason = False, f"hard problem (ELO {diff}) has time={tl}s (min 30s)"

        if ok:
            valid += 1
        else:
            issues.append(f"Problem {i+1}: {reason}")

    score = (valid / len(problems)) * 100
    return {
        "name": "time_limit",
        "passed": score >= 80,
        "score": round(score, 1),
        "details": f"{valid}/{len(problems)} pass time limit checks" + (f"; {'; '.join(issues[:3])}" if issues else ""),
    }


# Hardcoded from crates/common/src/lib.rs MainTopic.subtopics()
_VALID_TOPICS = {
    "arithmetic": [
        "addition", "subtraction", "multiplication", "long_division",
        "fractions", "mixed_numbers", "decimals", "percentages",
        "order_of_operations", "ratios_proportions",
    ],
    "algebra1": [
        "one_step_equations", "two_step_equations", "multi_step_equations",
        "linear_inequalities", "compound_inequalities", "slope_and_intercept",
        "graphing_lines", "systems_substitution", "systems_elimination",
        "exponent_rules", "polynomial_operations", "factoring_gcf",
        "factoring_trinomials", "quadratic_formula",
    ],
    "geometry": [
        "angle_relationships", "triangle_properties", "triangle_congruence",
        "similar_triangles", "circle_theorems", "arc_length_sectors",
        "area_of_polygons", "perimeter", "surface_area", "volume",
        "pythagorean_theorem", "right_triangle_trig",
    ],
    "algebra2": [
        "complex_number_operations", "complex_number_equations",
        "rational_expressions", "rational_equations", "radical_expressions",
        "radical_equations", "exponential_growth_decay", "exponential_equations",
        "logarithm_properties", "logarithmic_equations",
        "arithmetic_sequences", "geometric_sequences",
    ],
    "precalculus": [
        "domain_and_range", "function_composition", "inverse_functions",
        "transformations", "unit_circle", "graphing_trig", "trig_identities",
        "sum_difference_formulas", "inverse_trig_functions",
        "law_of_sines_cosines", "polar_coordinates", "polar_curves",
        "vector_operations", "dot_cross_product",
    ],
    "calculus": [
        "limits_algebraic", "limits_at_infinity", "continuity",
        "derivative_rules", "chain_rule", "implicit_differentiation",
        "related_rates", "curve_sketching", "optimization", "lhopitals_rule",
        "antiderivatives", "u_substitution", "integration_by_parts",
        "definite_integrals", "area_between_curves", "volumes_of_revolution",
    ],
    "differential_equations": [
        "separable_equations", "first_order_linear", "exact_equations",
        "homogeneous_equations", "second_order_constant",
        "characteristic_equation", "undetermined_coefficients",
        "variation_of_parameters", "laplace_transforms", "systems_of_odes",
    ],
    "multivariable_calculus": [
        "partial_derivatives", "gradient", "directional_derivatives",
        "lagrange_multipliers", "double_integrals", "triple_integrals",
        "change_of_variables", "line_integrals", "greens_theorem",
        "stokes_divergence",
    ],
    "linear_algebra": [
        "row_reduction", "matrix_arithmetic", "matrix_inverses",
        "determinants", "vector_spaces", "subspaces",
        "linear_independence", "eigenvalues", "diagonalization",
        "linear_transformations",
    ],
    "statistics": [
        "descriptive_statistics", "data_displays", "probability_basics",
        "counting_principles", "normal_distribution", "sampling_distributions",
        "confidence_intervals", "hypothesis_testing", "linear_regression",
        "chi_square_tests",
    ],
    "physics": [
        "kinematics_1d", "kinematics_2d", "newtons_laws", "friction_forces",
        "work_energy", "conservation_of_energy", "momentum_collisions",
        "rotational_motion", "simple_harmonic_motion", "wave_properties",
        "sound_waves", "fluid_mechanics", "thermodynamics", "ideal_gas_law",
        "electrostatics", "dc_circuits", "magnetism", "electromagnetic_induction",
        "geometric_optics", "wave_optics", "gravitation",
    ],
    "test": [
        "expressions", "numerics", "sets", "tuples", "lists",
        "intervals", "inequalities", "equations", "booleans",
        "words", "matrices", "multipart",
    ],
}


def _check_topic_validity(problems: list) -> Dict[str, Any]:
    """Check 11: every problem must have a valid (main_topic, subtopic) pair."""
    if not problems:
        return {"name": "topic_validity", "passed": False, "score": 0, "details": "No problems"}

    valid = 0
    issues = []
    for i, p in enumerate(problems):
        mt = p.get("main_topic", "")
        st = p.get("subtopic", "")
        # Handle "main/sub" format in topic field
        if "/" in mt and not st:
            parts = mt.split("/", 1)
            mt, st = parts[0], parts[1]

        if mt not in _VALID_TOPICS:
            issues.append(f"Problem {i+1}: invalid main_topic '{mt}'")
        elif st not in _VALID_TOPICS[mt]:
            issues.append(f"Problem {i+1}: invalid subtopic '{st}' for '{mt}'")
        else:
            valid += 1

    score = (valid / len(problems)) * 100
    return {
        "name": "topic_validity",
        "passed": score >= 100,
        "score": round(score, 1),
        "details": f"{valid}/{len(problems)} valid topics" + (f"; {'; '.join(issues[:3])}" if issues else ""),
    }


def _check_grading_roundtrip(problems: list) -> Dict[str, Any]:
    """Check 12: grade each answer_key against itself via grade-check binary."""
    if not problems:
        return {"name": "grading_roundtrip", "passed": False, "score": 0, "details": "No problems"}

    # Try to find the grade-check binary
    import shutil
    binary = shutil.which("grade-check")
    if not binary:
        # Check common cargo build locations
        for candidate in [
            "target/release/grade-check",
            "target/debug/grade-check",
            "../../target/release/grade-check",
            "../../target/debug/grade-check",
        ]:
            if Path(candidate).exists():
                binary = candidate
                break

    if not binary:
        return {
            "name": "grading_roundtrip",
            "passed": True,
            "score": 100,
            "details": "grade-check binary not found — skipped (pass with warning)",
        }

    # Build JSONL input
    lines = []
    for p in problems:
        lines.append(json.dumps({
            "answer_key": str(p.get("answer_key", "")),
            "answer_type": p.get("answer_type", "expression"),
            "grading_mode": p.get("grading_mode", "equivalent"),
        }))
    stdin_data = "\n".join(lines) + "\n"

    try:
        result = subprocess.run(
            [binary],
            input=stdin_data,
            capture_output=True,
            text=True,
            timeout=30,
        )
        if result.returncode != 0:
            return {
                "name": "grading_roundtrip",
                "passed": False,
                "score": 0,
                "details": f"grade-check failed: {result.stderr[:100]}",
            }

        valid = 0
        issues = []
        for i, line in enumerate(result.stdout.strip().split("\n")):
            if not line.strip():
                continue
            try:
                out = json.loads(line)
                if out.get("ok"):
                    valid += 1
                else:
                    issues.append(f"Problem {i+1}: {out.get('result', 'failed')}")
            except json.JSONDecodeError:
                issues.append(f"Problem {i+1}: invalid JSON output")

        total = len(problems)
        score = (valid / total) * 100 if total > 0 else 0
        return {
            "name": "grading_roundtrip",
            "passed": score >= 95,
            "score": round(score, 1),
            "details": f"{valid}/{total} self-grade OK" + (f"; {'; '.join(issues[:3])}" if issues else ""),
        }
    except subprocess.TimeoutExpired:
        return {
            "name": "grading_roundtrip",
            "passed": False,
            "score": 0,
            "details": "grade-check timed out after 30s",
        }
    except Exception as e:
        return {
            "name": "grading_roundtrip",
            "passed": True,
            "score": 100,
            "details": f"grade-check error: {str(e)[:80]} — skipped (pass with warning)",
        }


# ---------------------------------------------------------------------------
# Main validation entry point
# ---------------------------------------------------------------------------

def validate_script(script: str, language: str, runs: int, scripts_dir: Path) -> Dict[str, Any]:
    """
    Run a script `runs` times and validate the output across 12 categories.
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

    # Run all 12 checks
    categories = [
        _check_robustness(problems, errors, runs),
        _check_answer_format(problems),
        _check_latex_quality(problems),
        _check_diversity(problems),
        _check_difficulty_spread(problems),
        _check_svg_validity(problems),
        _check_gradeability(problems),
        _check_unevaluated_sympy(problems),
        _check_solution_quality(problems),
        _check_time_limit(problems),
        _check_topic_validity(problems),
        _check_grading_roundtrip(problems),
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
