# How to Use multipass Hooks (Auto-Save)

multipass hooks act as an "Auto-Save" feature. They help your AI keep a permanent memory without you needing to run manual commands.

### 1. What are these hooks?
* **Save Hook** (`multipass_save_hook.sh`): Saves new facts and decisions every 15 messages.
* **PreCompact Hook** (`multipass_precompact_hook.sh`): Saves your context right before the AI's memory window fills up.

### 2. Setup for Claude Code
Add this to your configuration file to enable automatic background saving:

```json
{
  "hooks": {
    "Stop": [
      {
        "matcher": "", 
        "hooks": [{"type": "command", "command": "./hooks/multipass_save_hook.sh"}]
      }
    ],
    "PreCompact": [
      {
        "matcher": "", 
        "hooks": [{"type": "command", "command": "./hooks/multipass_precompact_hook.sh"}]
      }
    ]
  }
}