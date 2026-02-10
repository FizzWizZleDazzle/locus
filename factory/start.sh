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

    # Try npx tsc first (local install)
    if command -v npx &> /dev/null; then
        if npx -y typescript@latest --version &> /dev/null; then
            npx -y typescript@latest --project tsconfig.json
            echo "[OK] TypeScript compiled"
        else
            echo "[WARN] npx typescript not available, trying global tsc..."
            if command -v tsc &> /dev/null; then
                tsc --project tsconfig.json
                echo "[OK] TypeScript compiled"
            else
                echo "[ERROR] TypeScript not found. Install with: npm install -g typescript"
                exit 1
            fi
        fi
    # Try global tsc
    elif command -v tsc &> /dev/null; then
        tsc --project tsconfig.json
        echo "[OK] TypeScript compiled"
    else
        echo "[ERROR] TypeScript not found. Install with: npm install -g typescript"
        echo "[INFO] Or use npx: npx -y typescript@latest"
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

# Start backend
echo ""
echo "[*] Starting backend on http://localhost:$BACKEND_PORT ..."
python main.py > /tmp/locus_factory.log 2>&1 &
BACKEND_PID=$!

# Wait for backend
sleep 2
if ! curl -s http://localhost:$BACKEND_PORT/ > /dev/null 2>&1; then
    echo "[ERROR] Backend failed to start. Check /tmp/locus_factory.log"
    kill $BACKEND_PID 2>/dev/null
    exit 1
fi
echo "[OK] Backend ready"

# Start frontend HTTP server
cd "$FACTORY_DIR/frontend"
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
echo "Logs:     /tmp/locus_factory.log"
echo ""
echo "Press Ctrl+C to shutdown"
echo ""

# Cleanup on exit
trap "echo '';echo 'Shutting down...';kill $BACKEND_PID $FRONTEND_PID 2>/dev/null;echo 'Factory stopped.';exit 0" INT TERM

# Keep running
wait
