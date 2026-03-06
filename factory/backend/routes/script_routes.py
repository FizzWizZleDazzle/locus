"""Script management and execution routes"""

import re
from datetime import datetime
from fastapi import APIRouter, HTTPException
from fastapi.concurrency import run_in_threadpool

from models import (
    GenerateScriptRequest,
    SaveScriptRequest,
    TestScriptRequest,
    RunScriptRequest,
    MassGenerateRequest,
    ValidateScriptRequest,
)
from config import SCRIPTS_DIR, EXPORTS_DIR, llm_config, staged_problems
from services.llm import generate_script_with_llm
from services.script_runner import (
    test_script_code,
    run_script_multiple,
    mass_generate,
    mass_generate_to_file,
)
from services.validator import validate_script

router = APIRouter()


def _find_script(name: str):
    """Find a script by stem name, trying .jl first then .py. Returns (path, language)."""
    jl = SCRIPTS_DIR / f"{name}.jl"
    if jl.exists():
        return jl, "julia"
    py = SCRIPTS_DIR / f"{name}.py"
    if py.exists():
        return py, "python"
    return None, None


def _list_all_scripts():
    """Discover all .py and .jl scripts in the scripts directory."""
    skip = {"temp_test.py", "temp_test.jl", "problem_utils.py", "svg_utils.py",
            "__pycache__"}
    scripts = []
    for ext in ("*.jl", "*.py"):
        for f in SCRIPTS_DIR.glob(ext):
            if f.name not in skip and f.stem not in skip:
                scripts.append(f)
    return scripts


@router.post("/generate-script")
async def generate_script(request: GenerateScriptRequest):
    """Generate a script using LLM"""
    script = await generate_script_with_llm(
        llm_config,
        request.main_topic,
        request.subtopic,
        request.difficulty_level,
        request.prompt_template,
        request.language,
    )

    return {
        "script": script,
        "language": request.language,
        "message": "Script generated successfully"
    }


@router.get("/scripts")
async def list_scripts():
    """List all saved scripts (both Python and Julia)"""
    scripts = []
    for script_file in _list_all_scripts():
        content = script_file.read_text()
        lines = content.split('\n')
        description = ""
        language = "julia" if script_file.suffix == ".jl" else "python"

        # Extract description from docstring (Python) or module doc (Julia)
        if language == "python" and len(lines) > 1 and lines[1].startswith('"""'):
            for i in range(2, min(10, len(lines))):
                if '"""' in lines[i]:
                    break
                description += lines[i].strip() + " "
        elif language == "julia" and len(lines) > 0:
            # Check for Julia doc comment: #= ... =# or leading # comments
            for i in range(min(5, len(lines))):
                line = lines[i].strip()
                if line.startswith('#') and not line.startswith('#='):
                    description += line.lstrip('# ').strip() + " "
                elif line.startswith('\"\"\"'):
                    for j in range(i+1, min(i+5, len(lines))):
                        if '\"\"\"' in lines[j]:
                            break
                        description += lines[j].strip() + " "
                    break
                elif line and not line.startswith('#'):
                    break

        scripts.append({
            "name": script_file.stem,
            "filename": script_file.name,
            "language": language,
            "description": description.strip() or "No description",
            "created": datetime.fromtimestamp(script_file.stat().st_mtime).isoformat(),
        })

    return {
        "scripts": sorted(scripts, key=lambda x: x["created"], reverse=True),
        "count": len(scripts)
    }


@router.post("/scripts/save")
async def save_script(request: SaveScriptRequest):
    """Save a script to the scripts directory"""
    safe_name = re.sub(r'[^a-zA-Z0-9_-]', '_', request.name)
    ext = ".jl" if request.language == "julia" else ".py"
    base_path = SCRIPTS_DIR / f"{safe_name}{ext}"

    script_path = base_path
    if script_path.exists() and not request.overwrite:
        timestamp = datetime.utcnow().strftime('%Y%m%d_%H%M%S')
        script_path = SCRIPTS_DIR / f"{safe_name}_{timestamp}{ext}"

    # Add header comment with description and metadata
    if request.language == "julia":
        header = f'# {request.description or request.name}\n# Generated: {datetime.utcnow().isoformat()}\n\n'
    else:
        header = f'"""\n{request.description or request.name}\nGenerated: {datetime.utcnow().isoformat()}\n"""\n\n'
    full_script = header + request.script

    script_path.write_text(full_script)

    return {
        "message": "Script saved successfully",
        "filename": script_path.name,
        "language": request.language,
        "path": str(script_path)
    }


@router.get("/scripts/{script_name}")
async def get_script(script_name: str):
    """Load a script by name (tries .jl first, then .py)"""
    path, language = _find_script(script_name)
    if path is None:
        raise HTTPException(status_code=404, detail="Script not found")

    content = path.read_text()
    return {
        "name": script_name,
        "script": content,
        "language": language,
        "path": str(path)
    }


@router.delete("/scripts/{script_name}")
async def delete_script(script_name: str):
    """Delete a script (tries both extensions)"""
    path, _ = _find_script(script_name)
    if path is None:
        raise HTTPException(status_code=404, detail="Script not found")

    path.unlink()
    return {"message": "Script deleted successfully"}


@router.post("/test-script")
async def test_script(request: TestScriptRequest):
    """Test a script by running it once"""
    return test_script_code(request.script, SCRIPTS_DIR, request.language)


@router.post("/validate-script")
async def validate_script_route(request: ValidateScriptRequest):
    """Run script N times and validate output across 7 quality categories"""
    return await run_in_threadpool(validate_script, request.script, request.language, request.runs, SCRIPTS_DIR)


@router.post("/run-script")
async def run_script(request: RunScriptRequest):
    """Run a saved script multiple times to generate problems"""
    path, _ = _find_script(request.script_name)
    if path is None:
        raise HTTPException(status_code=404, detail="Script not found")

    return run_script_multiple(path, request.count, SCRIPTS_DIR)


@router.post("/mass-generate")
async def mass_generate_route(request: MassGenerateRequest):
    """Run ALL saved scripts N times each; write problems directly to SQL file."""
    has_scripts = any(_list_all_scripts())
    if not has_scripts:
        raise HTTPException(status_code=400, detail="No scripts available")

    timestamp = datetime.utcnow().strftime('%Y%m%d_%H%M%S')
    output_path = EXPORTS_DIR / f"problems_{timestamp}.sql"

    return await run_in_threadpool(mass_generate_to_file, SCRIPTS_DIR, request.count_per_script, output_path)
