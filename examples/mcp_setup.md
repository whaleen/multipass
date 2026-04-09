# MCP Integration — Claude Code

## Setup

Run the MCP server:

```bash
python -m multipass.mcp_server
```

Or add it to Claude Code:

```bash
claude mcp add multipass -- python -m multipass.mcp_server
```

## Available Tools

The server exposes the full multipass MCP toolset. Common entry points include:

- **multipass_status** — ship stats (wings, rooms, crate counts)
- **multipass_search** — semantic search across all memories
- **multipass_list_wings** — list all projects in the ship

## Usage in Claude Code

Once configured, Claude Code can search your memories directly during conversations.
