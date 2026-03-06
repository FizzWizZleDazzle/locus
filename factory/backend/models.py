"""Pydantic models for Locus Factory API"""

from pydantic import BaseModel
from typing import Optional, Dict, Any
from enum import Enum


class CalculatorLevel(str, Enum):
    """Calculator levels form a hierarchy: none < scientific < graphing < cas"""
    NONE = "none"
    SCIENTIFIC = "scientific"
    GRAPHING = "graphing"
    CAS = "cas"


class LLMConfig(BaseModel):
    endpoint: str
    api_key: str
    model: str


class GenerateScriptRequest(BaseModel):
    main_topic: str
    subtopic: str
    difficulty_level: str = "medium"  # "very_easy", "easy", "medium", "hard", "very_hard", "competition"
    prompt_template: Optional[str] = None
    language: str = "julia"  # "julia" or "python"


class SaveScriptRequest(BaseModel):
    name: str
    script: str
    description: Optional[str] = None
    overwrite: bool = False
    language: str = "julia"  # "julia" or "python"


class TestScriptRequest(BaseModel):
    script: str
    language: str = "julia"  # "julia" or "python"


class RunScriptRequest(BaseModel):
    script_name: str
    count: int = 1


class MassGenerateRequest(BaseModel):
    """Run ALL scripts N times each"""
    count_per_script: int = 100


class ConfirmProblemRequest(BaseModel):
    problem: Dict[str, Any]
    approved: bool = True


class ValidateScriptRequest(BaseModel):
    script: str
    language: str = "julia"
    runs: int = 30


class ExportRequest(BaseModel):
    format: str = "sql"
