# multipass Claude Code Plugin

A Claude Code plugin that gives your AI a persistent memory system. Mine projects and conversations into a searchable ship backed by ChromaDB, with 19 MCP tools, auto-save hooks, and 5 guided skills.

## Prerequisites

- Python 3.9+

## Installation

### Claude Code Marketplace

```bash
claude plugin marketplace add whaleen/multipass
claude plugin install --scope user multipass
```

### Local Clone

```bash
claude plugin add /path/to/multipass
```

## Post-Install Setup

After installing the plugin, run the init command to complete setup (pip install, MCP configuration, etc.):

```
/multipass:init
```

## Available Slash Commands

| Command | Description |
|---------|-------------|
| `/multipass:help` | Show available tools, skills, and architecture |
| `/multipass:init` | Set up multipass -- install, configure MCP, onboard |
| `/multipass:search` | Search your memories across the ship |
| `/multipass:mine` | Mine projects and conversations into the ship |
| `/multipass:status` | Show ship overview -- wings, rooms, crate counts |

## Hooks

multipass registers two hooks that run automatically:

- **Stop** -- Saves conversation context every 15 messages.
- **PreCompact** -- Preserves important memories before context compaction.

Set the `MULTIPASS_DIR` environment variable to a directory path to automatically run `multipass mine` on that directory during each save trigger.

## MCP Server

The plugin automatically configures a local MCP server with 19 tools for storing, searching, and managing memories. No manual MCP setup is required -- `/multipass:init` handles everything.

## Full Documentation

See the main [README](../README.md) for complete documentation, architecture details, and advanced usage.
