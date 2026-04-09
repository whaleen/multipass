# Hooks Tutorial

The hook scripts are part of the active rewrite flow.

Their purpose is simple:

- stop the agent at controlled checkpoints
- force it to persist important context before stopping or compacting

They do not write directly to the ship themselves. They block so the agent can save through the
current multipass surface, which is the MCP write path.

## Available Hooks

- `hooks/multipass_save_hook.sh`
  - blocks every `MULTIPASS_SAVE_INTERVAL` human messages
- `hooks/multipass_precompact_hook.sh`
  - always blocks before compaction

## Claude Code Setup

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

## What The Agent Should Do When Blocked

When the hook blocks, the agent should persist memory into multipass now.

Current preferred path:

1. use MCP
2. call `multipass_add_record`
3. save concrete decisions, constraints, code context, and useful quotes

## Configuration

- `MULTIPASS_SAVE_INTERVAL`
  - default `15`
- `MULTIPASS_HOOK_STATE_DIR`
  - default `~/.multipass/hook_state`

## Debugging

```bash
cat ~/.multipass/hook_state/hook.log
```

For the authoritative details, see:

- [hooks/README.md](/Users/josh/Projects/_whaleen/multipass/hooks/README.md)
