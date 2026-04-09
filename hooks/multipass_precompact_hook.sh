#!/bin/bash
set -euo pipefail

STATE_DIR="${MULTIPASS_HOOK_STATE_DIR:-$HOME/.multipass/hook_state}"
mkdir -p "$STATE_DIR"

INPUT="$(cat)"
SESSION_ID="$(
  printf '%s' "$INPUT" | python3 -c 'import json,sys,re; data=json.load(sys.stdin); raw=str(data.get("session_id","unknown")); clean=re.sub(r"[^a-zA-Z0-9_-]","",raw); print(clean or "unknown")' 2>/dev/null
)"
SESSION_ID="${SESSION_ID:-unknown}"

printf '[%s] PRE-COMPACT triggered for session %s\n' "$(date '+%H:%M:%S')" "$SESSION_ID" >> "$STATE_DIR/hook.log"

cat <<'HOOKJSON'
{
  "decision": "block",
  "reason": "COMPACTION IMMINENT. Persist the important context from this session into multipass now. Use the active multipass memory surface, preferably MCP record writes, and be thorough because detailed context will be lost after compaction. Save key decisions, current state, code context, constraints, and important quotes before allowing compaction."
}
HOOKJSON
