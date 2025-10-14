#!/usr/bin/env bash
# run.sh - Start backend, frontend, and open browser (cross-DE Linux safe)

# --- Step 1: Ensure .env exists with correct content ---
ENV_FILE="client/.env"
ENV_CONTENT="VITE_WS_URL=ws://127.0.0.1:3001/ws"

if [ -f "$ENV_FILE" ]; then
    if ! grep -Fxq "$ENV_CONTENT" "$ENV_FILE"; then
        echo "$ENV_CONTENT" > "$ENV_FILE"
        echo ".env updated with VITE_WS_URL"
    else
        echo ".env already correct"
    fi
else
    echo "$ENV_CONTENT" > "$ENV_FILE"
    echo ".env created with VITE_WS_URL"
fi

# --- Step 2: Detect terminal emulator ---
if command -v gnome-terminal &>/dev/null; then
    TERMINAL="gnome-terminal --"
elif command -v konsole &>/dev/null; then
    TERMINAL="konsole --hold -e"
elif command -v xfce4-terminal &>/dev/null; then
    TERMINAL="xfce4-terminal --hold -e"
elif command -v xterm &>/dev/null; then
    TERMINAL="xterm -hold -e"
else
    echo "No known terminal emulator found. Install gnome-terminal, konsole, xfce4-terminal, or xterm."
    exit 1
fi

# --- Step 3: Launch backend ---
$TERMINAL bash -c "cd server && cargo run; exec bash" &

# Small delay to avoid KDE terminal warnings
sleep 1

# --- Step 4: Launch frontend ---
$TERMINAL bash -c "cd client && npm run dev; exec bash" &

# --- Step 5: Open frontend in browser ---
open_url() {
    URL="http://localhost:5173"

    # 1. xdg-open (cross-DE)
    if command -v xdg-open &>/dev/null; then
        xdg-open "$URL" 2>/dev/null && return
    fi

    # 2. $BROWSER env variable
    if [ -n "$BROWSER" ] && command -v "$BROWSER" &>/dev/null; then
        "$BROWSER" "$URL" & return
    fi

    # 3. Common browsers
    for browser in firefox google-chrome brave chromium chromium-browser; do
        if command -v $browser &>/dev/null; then
            $browser "$URL" & return
        fi
    done

    # 4. Fallback
    echo "Could not automatically open browser. Open manually: $URL"
}

open_url

echo "Server and client starting..."
