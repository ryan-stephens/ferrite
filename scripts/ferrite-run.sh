#!/bin/bash
# ferrite-run.sh — Wrapper script for Ferrite with self-update restart support.
#
# Usage:
#   screen -dmS ferrite ./ferrite-run.sh
#   # or simply:
#   ./ferrite-run.sh
#
# This script:
#   1. Restarts Ferrite when it exits with code 42 (update applied).
#   2. Automatically rolls back to ferrite.bak if the new binary fails to start
#      (detected via the pending-validation marker file).
#   3. Exits normally for any other exit code.

set -euo pipefail
cd "$(dirname "$0")"

MARKER="data/.update/pending-validation"

while true; do
    # If a pending-validation marker exists and we have a .bak, the previous
    # update failed to start — roll back automatically.
    if [ -f "$MARKER" ] && [ -f "ferrite.bak" ]; then
        echo "[ferrite-run] Update validation failed, rolling back to ferrite.bak..."
        mv ferrite ferrite.failed 2>/dev/null || true
        mv ferrite.bak ferrite
        rm -f "$MARKER"
        echo "[ferrite-run] Rollback complete, restarting..."
    fi

    # Remove the pending-validation marker on successful startup
    # (the binary itself should clear it after health check, but we also
    # clear it here if the process ran long enough to be considered healthy)
    ./ferrite "$@" &
    FERRITE_PID=$!

    # Wait a few seconds, then clear the marker if the process is still running
    (
        sleep 10
        if kill -0 "$FERRITE_PID" 2>/dev/null && [ -f "$MARKER" ]; then
            rm -f "$MARKER"
            echo "[ferrite-run] Startup validation passed, marker cleared."
        fi
    ) &
    VALIDATOR_PID=$!

    # Wait for the main process to exit
    wait "$FERRITE_PID" || true
    EXIT_CODE=$?

    # Clean up the validator subprocess
    kill "$VALIDATOR_PID" 2>/dev/null || true
    wait "$VALIDATOR_PID" 2>/dev/null || true

    if [ "$EXIT_CODE" -eq 42 ]; then
        echo "[ferrite-run] Ferrite requested restart (update applied), restarting in 1s..."
        sleep 1
        continue
    fi

    echo "[ferrite-run] Ferrite exited with code $EXIT_CODE, not restarting."
    break
done
