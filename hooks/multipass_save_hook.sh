#!/bin/bash
set -euo pipefail

SAVE_INTERVAL="${MULTIPASS_SAVE_INTERVAL:-15}"
STATE_DIR="${MULTIPASS_HOOK_STATE_DIR:-$HOME/.multipass/hook_state}"
mkdir -p "$STATE_DIR"

INPUT="$(cat)"

SESSION_ID="$(
  printf '%s' "$INPUT" | python3 -c 'import json,sys,re; data=json.load(sys.stdin); raw=str(data.get("session_id","unknown")); clean=re.sub(r"[^a-zA-Z0-9_-]","",raw); print(clean or "unknown")' 2>/dev/null
)"
SESSION_ID="${SESSION_ID:-unknown}"

STOP_HOOK_ACTIVE="$(
  printf '%s' "$INPUT" | python3 -c 'import json,sys; data=json.load(sys.stdin); print(data.get("stop_hook_active", False))' 2>/dev/null
)"

TRANSCRIPT_PATH="$(
  printf '%s' "$INPUT" | python3 -c 'import json,sys; data=json.load(sys.stdin); print(data.get("transcript_path",""))' 2>/dev/null
)"
TRANSCRIPT_PATH="${TRANSCRIPT_PATH/#\~/$HOME}"

STOP_HOOK_ACTIVE_LOWER="$(printf '%s' "$STOP_HOOK_ACTIVE" | tr '[:upper:]' '[:lower:]')"

if [ "$STOP_HOOK_ACTIVE_LOWER" = "true" ] || [ "$STOP_HOOK_ACTIVE" = "1" ]; then
  echo "{}"
  exit 0
fi

if [ -f "$TRANSCRIPT_PATH" ]; then
  EXCHANGE_COUNT="$(
    python3 - "$TRANSCRIPT_PATH" <<'PYEOF'
import json, sys
count = 0
with open(sys.argv[1], encoding="utf-8", errors="replace") as f:
    for line in f:
        try:
            entry = json.loads(line)
        except json.JSONDecodeError:
            continue
        msg = entry.get("message", {})
        if not isinstance(msg, dict) or msg.get("role") != "user":
            continue
        content = msg.get("content", "")
        if isinstance(content, str):
            if "<command-message>" in content:
                continue
        elif isinstance(content, list):
            text = " ".join(block.get("text", "") for block in content if isinstance(block, dict))
            if "<command-message>" in text:
                continue
        count += 1
print(count)
PYEOF
  )"
else
  EXCHANGE_COUNT=0
fi

LAST_SAVE_FILE="$STATE_DIR/${SESSION_ID}_last_save"
LAST_SAVE=0
if [ -f "$LAST_SAVE_FILE" ]; then
  LAST_SAVE="$(cat "$LAST_SAVE_FILE" 2>/dev/null || echo 0)"
fi

SINCE_LAST=$((EXCHANGE_COUNT - LAST_SAVE))
printf '[%s] Session %s: %s exchanges, %s since last save\n' "$(date '+%H:%M:%S')" "$SESSION_ID" "$EXCHANGE_COUNT" "$SINCE_LAST" >> "$STATE_DIR/hook.log"

if [ "$SINCE_LAST" -ge "$SAVE_INTERVAL" ] && [ "$EXCHANGE_COUNT" -gt 0 ]; then
  printf '%s' "$EXCHANGE_COUNT" > "$LAST_SAVE_FILE"
  printf '[%s] TRIGGERING SAVE at exchange %s\n' "$(date '+%H:%M:%S')" "$EXCHANGE_COUNT" >> "$STATE_DIR/hook.log"
  cat <<'HOOKJSON'
{
  "decision": "block",
  "reason": "AUTO-SAVE checkpoint. Persist the important context from this session into multipass now. Use the active multipass memory surface, preferably MCP record writes, and store concrete decisions, code insights, constraints, and verbatim quotes when useful. Continue the conversation after saving."
}
HOOKJSON
else
  echo "{}"
fi
