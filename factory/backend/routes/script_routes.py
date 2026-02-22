"""Script management and execution routes"""

import re
from datetime import datetime
from fastapi import APIRouter, HTTPException

from models import (
    GenerateScriptRequest,
    SaveScriptRequest,
    TestScriptRequest,
    RunScriptRequest,
    MassGenerateRequest
)
from config import SCRIPTS_DIR, llm_config, staged_problems
from services.llm import generate_script_with_llm
from services.script_runner import (
    test_script_code,
    run_script_multiple,
    mass_generate
)

router = APIRouter()


@router.post("/generate-script")
async def generate_script(request: GenerateScriptRequest):
    """Generate a Python script using LLM"""
    script = await generate_script_with_llm(
        llm_config,
        request.main_topic,
        request.subtopic,
        request.difficulty_level,
        request.prompt_template
    )

    return {
        "script": script,
        "message": "Script generated successfully"
    }


@router.get("/scripts")
async def list_scripts():
    """List all saved scripts"""
    scripts = []
    for script_file in SCRIPTS_DIR.glob("*.py"):
        # Read metadata from docstring if present
        content = script_file.read_text()
        lines = content.split('\n')
        description = ""
        if len(lines) > 1 and lines[1].startswith('"""'):
            # Extract docstring
            for i in range(2, min(10, len(lines))):
                if '"""' in lines[i]:
                    break
                description += lines[i].strip() + " "

        scripts.append({
            "name": script_file.stem,
            "filename": script_file.name,
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
    # Sanitize filename
    safe_name = re.sub(r'[^a-zA-Z0-9_-]', '_', request.name)
    base_path = SCRIPTS_DIR / f"{safe_name}.py"

    script_path = base_path
    if script_path.exists() and not request.overwrite:
        timestamp = datetime.utcnow().strftime('%Y%m%d_%H%M%S')
        script_path = SCRIPTS_DIR / f"{safe_name}_{timestamp}.py"

    # Add header comment with description and metadata
    header = f'"""\n{request.description or request.name}\nGenerated: {datetime.utcnow().isoformat()}\n"""\n\n'
    full_script = header + request.script

    script_path.write_text(full_script)

    return {
        "message": "Script saved successfully",
        "filename": script_path.name,
        "path": str(script_path)
    }


@router.get("/scripts/{script_name}")
async def get_script(script_name: str):
    """Load a script by name"""
    script_path = SCRIPTS_DIR / f"{script_name}.py"
    if not script_path.exists():
        raise HTTPException(status_code=404, detail="Script not found")

    content = script_path.read_text()
    return {
        "name": script_name,
        "script": content,
        "path": str(script_path)
    }


@router.delete("/scripts/{script_name}")
async def delete_script(script_name: str):
    """Delete a script"""
    script_path = SCRIPTS_DIR / f"{script_name}.py"
    if not script_path.exists():
        raise HTTPException(status_code=404, detail="Script not found")

    script_path.unlink()
    return {"message": "Script deleted successfully"}


@router.post("/test-script")
async def test_script(request: TestScriptRequest):
    """Test a script by running it once"""
    return test_script_code(request.script, SCRIPTS_DIR)


@router.post("/run-script")
async def run_script(request: RunScriptRequest):
    """Run a saved script multiple times to generate problems"""
    script_path = SCRIPTS_DIR / f"{request.script_name}.py"
    if not script_path.exists():
        raise HTTPException(status_code=404, detail="Script not found")

    return run_script_multiple(script_path, request.count, SCRIPTS_DIR)


@router.post("/mass-generate")
async def mass_generate_route(request: MassGenerateRequest):
    """Run ALL saved scripts N times each and auto-stage all problems"""
    if not any(SCRIPTS_DIR.glob("*.py")):
        raise HTTPException(status_code=400, detail="No scripts available")

    return mass_generate(SCRIPTS_DIR, request.count_per_script, staged_problems)
