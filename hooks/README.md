# Hooks

These hooks are part of the active multipass workflow.

They are not a side feature and they are not legacy wrappers around the old Python/Chroma ingest
path. Their job is narrower and more important: decide when an agent must persist memory before
stopping or compacting.

## What They Do

| Hook | When It Fires | What Happens |
|------|--------------|-------------|
| `multipass_save_hook.sh` | every `MULTIPASS_SAVE_INTERVAL` human messages | blocks the agent and tells it to persist important context |
| `multipass_precompact_hook.sh` | immediately before compaction | always blocks so the agent saves before context is lost |

The hooks themselves do **not** write records into the ship. They force the agent to do that
through the active multipass memory surface, which is currently the MCP write path.

## Current Persistence Model

When a hook blocks, the agent should save via multipass itself:

- preferably through MCP `multipass_add_record`
- storing concrete facts, decisions, constraints, code context, and useful verbatim quotes

This keeps the hook system aligned with the rewrite instead of reviving the old transcript-ingest
path.

## Install — Claude Code

Add to `.claude/settings.local.json`:

```json
{
  "hooks": {
    "Stop": [{
      "matcher": "*",
      "hooks": [{
        "type": "command",
        "command": "/absolute/path/to/hooks/multipass_save_hook.sh",
        "timeout": 30
      }]
    }],
    "PreCompact": [{
      "hooks": [{
        "type": "command",
        "command": "/absolute/path/to/hooks/multipass_precompact_hook.sh",
        "timeout": 30
      }]
    }]
  }
}
```

Make them executable:

```bash
chmod +x hooks/multipass_save_hook.sh hooks/multipass_precompact_hook.sh
```

## Install — Codex CLI

If your Codex setup supports command hooks, use the same scripts:

```json
{
  "Stop": [{
    "type": "command",
    "command": "/absolute/path/to/hooks/multipass_save_hook.sh",
    "timeout": 30
  }],
  "PreCompact": [{
    "type": "command",
    "command": "/absolute/path/to/hooks/multipass_precompact_hook.sh",
    "timeout": 30
  }]
}
```

## Configuration

Environment variables:

- `MULTIPASS_SAVE_INTERVAL`
  - default: `15`
  - how many human messages between auto-save checkpoints
- `MULTIPASS_HOOK_STATE_DIR`
  - default: `~/.multipass/hook_state`
  - where hook state and logs are stored

## Architecture

The hook scripts are self-contained shell entrypoints. They use small local `python3` snippets for
JSON parsing and transcript counting, but they do not depend on any in-repo Python package.

## Debugging

Check the hook log:

```bash
cat ~/.multipass/hook_state/hook.log
```

Typical output:

```text
[14:30:15] Session abc123: 12 exchanges, 12 since last save
[14:35:22] Session abc123: 15 exchanges, 15 since last save
[14:35:22] TRIGGERING SAVE at exchange 15
[14:40:01] PRE-COMPACT triggered for session abc123
```

## Important Boundaries

These hooks currently do not:

- import transcripts automatically
- mine conversations directly
- write to SQLite themselves

That is intentional. They are control points, not a second persistence stack.
