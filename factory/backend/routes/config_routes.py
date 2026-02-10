"""Configuration routes"""

from fastapi import APIRouter
from models import LLMConfig, LocusConfig
from config import llm_config, locus_config

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


@router.post("/config/locus")
async def configure_locus(config: LocusConfig):
    locus_config["backend_url"] = config.backend_url
    locus_config["api_key"] = config.api_key
    return {"message": "Locus configuration updated"}


@router.get("/config")
async def get_config():
    return {
        "llm": {
            "endpoint": llm_config["endpoint"],
            "api_key": "***" + (llm_config["api_key"][-4:] if llm_config["api_key"] else ""),
            "model": llm_config["model"],
            "configured": bool(llm_config["endpoint"] and llm_config["api_key"]),
        },
        "locus": {
            "backend_url": locus_config["backend_url"],
            "api_key": "***" if locus_config["api_key"] else None,
        },
    }
