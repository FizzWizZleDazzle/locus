"""Configuration routes"""

from fastapi import APIRouter
from models import LLMConfig
from config import llm_config

router = APIRouter()


@router.get("/")
async def root():
    return {
        "message": "Locus Factory API",
        "status": "running",
        "version": "2.0",
        "workflow": "script-based"
    }


@router.post("/config/llm")
async def configure_llm(config: LLMConfig):
    llm_config["endpoint"] = config.endpoint
    llm_config["api_key"] = config.api_key
    llm_config["model"] = config.model
    return {"message": "LLM configuration updated"}


@router.get("/config")
async def get_config():
    return {
        "llm": {
            "endpoint": llm_config["endpoint"],
            "api_key": "***" + (llm_config["api_key"][-4:] if llm_config["api_key"] else ""),
            "model": llm_config["model"],
            "configured": bool(llm_config["endpoint"] and llm_config["api_key"]),
        },
    }
