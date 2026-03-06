"""
Locus Factory - Script-Based Problem Generation Service

Workflow:
1. Generate Python script (via LLM or template)
2. Save script to scripts/ directory
3. Test script (run once)
4. Run script in batch to generate problems
5. Review and approve problems
6. Upload to Postgres or export to SQL/JSON

Features:
- LLM-powered script generation
- Script library management
- SymPy validation
- Problem staging and approval
- Direct Postgres insert + SQL/JSON export
"""

from contextlib import asynccontextmanager
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from config import DATABASE_URL
from services.db import init_pool, close_pool
from routes.config_routes import router as config_router
from routes.script_routes import router as script_router
from routes.problem_routes import router as problem_router


@asynccontextmanager
async def lifespan(app: FastAPI):
    await init_pool(DATABASE_URL)
    yield
    await close_pool()


# Create FastAPI app
app = FastAPI(title="Locus Factory", lifespan=lifespan)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=False,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
app.include_router(config_router, tags=["config"])
app.include_router(script_router, tags=["scripts"])
app.include_router(problem_router, tags=["problems"])


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=9090)
