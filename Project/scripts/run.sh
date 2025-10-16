#!/usr/bin/env bash
# run.sh - Start backend + frontend in one script, works headless or GUI
set -e

SESSION_NAME="dev_env"
LOG_DIR="./logs"
BACKEND_DIR="./server"
FRONTEND_DIR="./client"
ENV_FILE="$FRONTEND_DIR/.env"
ENV_CONTENT="VITE_WS_URL=ws://127.0.0.1:3001/ws"

# --- Step 0: Check tmux ---
if ! command -v tmux &>/dev/null; then
    echo "Error: tmux is not installed. Install with:"
    echo "  sudo apt update && sudo apt install -y tmux"
    exit 1
fi

# --- Step 1: Ensure .env exists ---
mkdir -p "$FRONTEND_DIR"
if [ -f "$ENV_FILE" ]; then
    if ! grep -Fxq "$ENV_CONTENT" "$ENV_FILE"; then
        echo "$ENV_CONTENT" > "$ENV_FILE"
        echo ".env updated with VITE_WS_URL"
    fi
else
    echo "$ENV_CONTENT" > "$ENV_FILE"
    echo ".env created with VITE_WS_URL"
fi

# --- Step 2: Create logs directory ---
mkdir -p "$LOG_DIR"

# --- Step 3: Clean previous tmux session if exists ---
if tmux has-session -t $SESSION_NAME 2>/dev/null; then
    tmux kill-session -t $SESSION_NAME
fi

# --- Step 4: Start backend in tmux ---
tmux new-session -d -s $SESSION_NAME -n backend \
    "cd $BACKEND_DIR && cargo run 2>&1 | tee $LOG_DIR/backend.log"

# --- Step 5: Start frontend in tmux ---
tmux new-window -t $SESSION_NAME -n frontend \
    "cd $FRONTEND_DIR && npm install && npm install vite && npm run dev 2>&1 | tee $LOG_DIR/frontend.log"

echo "✅ Dev environment started!"
echo "📜 Logs: $LOG_DIR/backend.log and $LOG_DIR/frontend.log"
echo ""
echo "👉 Attach to session: tmux attach -t $SESSION_NAME"
echo "👉 Detach: Ctrl+B then D"
echo "👉 Stop everything: Ctrl+C here"
echo ""

# --- Step 6: Keep script alive so Ctrl+C works ---
trap "echo 'Stopping dev environment...'; tmux kill-session -t $SESSION_NAME 2>/dev/null; exit 0" SIGINT
while true; do sleep 2; done
