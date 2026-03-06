"""Configuration management for Locus Factory"""

import os
from pathlib import Path
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

# Paths
BASE_DIR = Path(__file__).parent
SCRIPTS_REPO = Path(os.getenv("SCRIPTS_REPO_PATH", str(BASE_DIR / "scripts")))
SCRIPTS_DIR = SCRIPTS_REPO / "src"
SCRIPTS_PYTHON_DIR = SCRIPTS_REPO / "python"
JULIA_PROJECT = SCRIPTS_REPO / "julia"
EXPORTS_DIR = BASE_DIR.parent / "exports"

# Create directories if they don't exist
SCRIPTS_DIR.mkdir(exist_ok=True)
EXPORTS_DIR.mkdir(exist_ok=True)

# LLM Configuration (from env vars, can be overridden via API)
llm_config = {
    "endpoint": os.getenv("LLM_ENDPOINT"),
    "api_key": os.getenv("LLM_API_KEY"),
    "model": os.getenv("LLM_MODEL", "gpt-4"),
}

# Database Configuration
DATABASE_URL = os.getenv("DATABASE_URL", "postgres://locus:locus_dev_password@localhost:5433/locus")

# In-memory problem staging
staged_problems = []
