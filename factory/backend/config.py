"""Configuration management for Locus Factory"""

import os
from pathlib import Path
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

# Paths
BASE_DIR = Path(__file__).parent
SCRIPTS_DIR = BASE_DIR / "scripts" / "src"
EXPORTS_DIR = BASE_DIR / "exports"

# Create directories if they don't exist
SCRIPTS_DIR.mkdir(exist_ok=True)
EXPORTS_DIR.mkdir(exist_ok=True)

# LLM Configuration (from env vars, can be overridden via API)
llm_config = {
    "endpoint": os.getenv("LLM_ENDPOINT"),
    "api_key": os.getenv("LLM_API_KEY"),
    "model": os.getenv("LLM_MODEL", "gpt-4"),
}

# Locus Backend Configuration
locus_config = {
    "backend_url": os.getenv("LOCUS_BACKEND_URL", "http://localhost:3000"),
    "api_key": os.getenv("LOCUS_API_KEY", "development-factory-key-change-in-production"),
}

# In-memory problem staging
staged_problems = []
