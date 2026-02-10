"""Pydantic models for Locus Factory API"""

from pydantic import BaseModel
from typing import Optional, Dict, Any


class LLMConfig(BaseModel):
    endpoint: str
    api_key: str
    model: str


class LocusConfig(BaseModel):
    backend_url: str
    api_key: str


class GenerateScriptRequest(BaseModel):
    main_topic: str
    subtopic: str
    difficulty_level: str = "medium"  # "easy", "medium", "hard"
    grading_mode: str = "equivalent"
    prompt_template: Optional[str] = None


class SaveScriptRequest(BaseModel):
    name: str
    script: str
    description: Optional[str] = None


class TestScriptRequest(BaseModel):
    script: str


class RunScriptRequest(BaseModel):
    script_name: str
    count: int = 1


class MassGenerateRequest(BaseModel):
    """Run ALL scripts N times each"""
    count_per_script: int = 100


class ConfirmProblemRequest(BaseModel):
    problem: Dict[str, Any]
    approved: bool = True


class ExportRequest(BaseModel):
    format: str = "sql"
