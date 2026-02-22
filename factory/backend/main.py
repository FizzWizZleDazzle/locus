"""
Locus Factory - Script-Based Problem Generation Service

Workflow:
1. Generate Python script (via LLM or template)
2. Save script to scripts/ directory
3. Test script (run once)
4. Run script in batch to generate problems
5. Review and approve problems
6. Export to SQL for PostgreSQL

Features:
- LLM-powered script generation
- Script library management
- SymPy validation
- Problem staging and approval
- SQL/JSON export
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from routes.config_routes import router as config_router
from routes.script_routes import router as script_router
from routes.problem_routes import router as problem_router

# Create FastAPI app
app = FastAPI(title="Locus Factory")

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
