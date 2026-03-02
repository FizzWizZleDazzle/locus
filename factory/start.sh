#!/bin/bash
# Locus Factory - Complete Startup Script

set -e

FACTORY_DIR="$(cd "$(dirname "$0")" && pwd)"
BACKEND_PORT=9090
FRONTEND_PORT=9091

echo "======================================"
echo "  Locus Factory Startup"
echo "======================================"
echo ""

# Check Python
if ! command -v python3 &> /dev/null; then
    echo "[ERROR] Python 3 not installed"
    exit 1
fi

# Check/compile TypeScript
cd "$FACTORY_DIR/frontend"
if [ -f "factory.ts" ]; then
    echo "[*] Compiling TypeScript..."

    if command -v tsc &> /dev/null; then
        tsc --project tsconfig.json
        echo "[OK] TypeScript compiled"
    elif command -v npx &> /dev/null; then
        echo "[*] Using npx to compile TypeScript..."
        npx -y tsc --project tsconfig.json
        echo "[OK] TypeScript compiled"
    else
        echo "[ERROR] TypeScript not found. Install with: npm install -g typescript"
        exit 1
    fi
fi

# Setup backend
cd "$FACTORY_DIR/backend"
if [ ! -d "venv" ]; then
    echo "[*] Creating virtual environment..."
    python3 -m venv venv
    source venv/bin/activate
    echo "[*] Installing dependencies..."
    pip install -q -r requirements.txt
    echo "[OK] Dependencies installed"
else
    source venv/bin/activate
fi

# Check for .env
if [ ! -f ".env" ]; then
    echo "[WARN] No .env file found. Creating from example..."
    cp .env.example .env
    echo "[WARN] Edit factory/backend/.env with your LLM API key"
fi

# Julia setup (optional — only Python scripts will work without it)
# Julia project now lives inside the scripts submodule
JULIA_DIR="$FACTORY_DIR/backend/scripts/julia"

if command -v julia &> /dev/null; then
    if [ ! -f "$JULIA_DIR/Manifest.toml" ]; then
        echo "[*] Installing Julia dependencies (~1 min, one-time)..."
        julia --project="$JULIA_DIR" -e 'using Pkg; Pkg.instantiate()'
        echo "[OK] Julia dependencies installed"
    fi
    SYSIMAGE="$JULIA_DIR/sysimage.so"
    if [ ! -f "$SYSIMAGE" ] || [ "$JULIA_DIR/Project.toml" -nt "$SYSIMAGE" ]; then
        # Try downloading pre-built sysimage from GitHub Releases
        if command -v gh &> /dev/null; then
            echo "[*] Trying to download pre-built Julia sysimage..."
            if gh release download sysimage-latest \
                --repo FizzWizZleDazzle/locus-scripts \
                -p 'sysimage.so' \
                -D "$JULIA_DIR" \
                --clobber 2>/dev/null; then
                echo "[OK] Downloaded pre-built sysimage"
            else
                echo "[INFO] No pre-built sysimage available, building locally..."
            fi
        fi

        # Build locally if download failed or unavailable
        if [ ! -f "$SYSIMAGE" ] || [ "$JULIA_DIR/Project.toml" -nt "$SYSIMAGE" ]; then
            echo "[*] Installing build dependencies..."
            julia --project="$JULIA_DIR/build" -e 'using Pkg; Pkg.instantiate()'
            echo "[*] Building Julia sysimage (~2 min, one-time)..."
            if julia --project="$JULIA_DIR/build" "$JULIA_DIR/build/build_sysimage.jl"; then
                echo "[OK] Julia sysimage built"
            else
                echo "[WARN] Sysimage build failed — Julia scripts will still work (slower startup)"
            fi
        fi
    fi
else
    echo "[WARN] Julia not installed — only Python scripts will work"
    echo "[INFO] Install Julia from https://julialang.org/downloads/"
fi

# Start backend
echo ""
echo "[*] Starting backend on http://localhost:$BACKEND_PORT (hot reload)..."
uvicorn main:app --host 0.0.0.0 --port $BACKEND_PORT --reload --reload-dir "$FACTORY_DIR/backend" > /tmp/locus_factory.log 2>&1 &
BACKEND_PID=$!

# Wait for backend
sleep 2
if ! curl -s http://localhost:$BACKEND_PORT/ > /dev/null 2>&1; then
    echo "[ERROR] Backend failed to start. Check /tmp/locus_factory.log"
    kill $BACKEND_PID 2>/dev/null
    exit 1
fi
echo "[OK] Backend ready"

# Start TypeScript watch (recompile on change)
cd "$FACTORY_DIR/frontend"
echo "[*] Starting TypeScript watcher..."
if command -v tsc &> /dev/null; then
    tsc --project tsconfig.json --watch --preserveWatchOutput > /tmp/locus_tsc.log 2>&1 &
else
    npx -y tsc --project tsconfig.json --watch --preserveWatchOutput > /tmp/locus_tsc.log 2>&1 &
fi
TSC_PID=$!

# Start frontend HTTP server
echo "[*] Starting frontend on http://localhost:$FRONTEND_PORT ..."
python3 -m http.server $FRONTEND_PORT > /dev/null 2>&1 &
FRONTEND_PID=$!
sleep 1
echo "[OK] Frontend ready"

# Open browser
echo ""
echo "[*] Opening browser..."
if command -v xdg-open &> /dev/null; then
    xdg-open "http://localhost:$FRONTEND_PORT" 2>/dev/null
elif command -v open &> /dev/null; then
    open "http://localhost:$FRONTEND_PORT"
else
    echo "[WARN] Could not auto-open browser. Visit http://localhost:$FRONTEND_PORT"
fi

echo ""
echo "======================================"
echo "  Factory Running"
echo "======================================"
echo "Frontend: http://localhost:$FRONTEND_PORT"
echo "Backend:  http://localhost:$BACKEND_PORT"
echo "Logs:     /tmp/locus_factory.log (backend)"
echo "          /tmp/locus_tsc.log (tsc watch)"
echo ""
echo "Press Ctrl+C to shutdown"
echo ""

# Cleanup on exit
trap "echo '';echo 'Shutting down...';kill $BACKEND_PID $FRONTEND_PID $TSC_PID 2>/dev/null;echo 'Factory stopped.';exit 0" INT TERM

# Keep running
wait
