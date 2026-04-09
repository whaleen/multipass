<div align="center">

<img src="assets/multipass_logo.png" alt="multipass" width="280">

# multipass

Local memory infrastructure for projects, conversations, and agent context.

</div>

## Overview

`multipass` is a local-first memory system for AI-assisted work.

This branch is the active Rust rewrite.

If you need the full expected functionality and utility while the rewrite is still in progress,
use the `main` branch as the reference point for product scope and behavior. This branch should be
read as the forward-looking implementation surface, not as feature-complete parity yet.

The goal is simple:

- ingest project files and conversation history
- preserve useful context locally
- make that context searchable and navigable
- expose it to CLIs, MCP clients, and future tools

This repository is currently in transition toward a Rust-native local memory engine with a cleaner
architecture and a more stable product surface.

## What Multipass Does

Today, the Rust rewrite can:

- initialize a project with a local `multipass.yaml`
- infer a default wing and room layout from a project directory
- mine project files into a local ship
- search stored memory with wing/room filters
- expose an MCP server for tool-based access
- render an AAAK-style wake-up brief

At a high level, the current flow is:

1. initialize a project
2. generate a local project config
3. mine files into a local store
4. search or query that store later

## Core Concepts

The current model uses a small set of spatial concepts:

- `ship`
  The top-level local memory store.
- `wing`
  A major partition, usually a project, person, or topic.
- `room`
  A focused topic within a wing.
- `corridor`
  A memory type or connective category shared across wings.
- `locker`
  A summary/index layer pointing toward stored content.
- `record`
  A stored memory unit.

## Current Status

This repo currently mixes:

- active Rust rewrite work
- newer `multipass` branding and ship-based terminology
- product concepts that are still being finalized as implementation continues

Important practical notes:

- project mining depends on a generated `multipass.yaml`
- the Rust CLI and MCP surfaces are usable, but not yet at full intended parity

## Direction

The long-term direction is:

- define a clean `multipass` product model
- separate durable concepts from incidental implementation details
- design a Rust-native rewrite around the actual product contract

That likely means:

- better storage and indexing boundaries
- clearer ingest and retrieval flows
- a more stable MCP/tool contract
- less dependency on heavyweight first-run bootstrap
- a cleaner internal model for memory, graph relationships, and agent-facing retrieval

## Quick Start

Build or run the Rust CLI from the workspace:

```bash
cargo build -p multipass-cli
```

Initialize a project:

```bash
cargo run -p multipass-cli -- init /path/to/project
```

Mine a project into a local ship:

```bash
cargo run -p multipass-cli -- --ship /path/to/ship mine /path/to/project
```

Search it:

```bash
cargo run -p multipass-cli -- --ship /path/to/ship search "auth decisions"
```

View ship status:

```bash
cargo run -p multipass-cli -- --ship /path/to/ship status
```

Start the MCP server:

```bash
cargo run -p multipass-cli -- --ship /path/to/ship mcp-server
```

## Basic Commands

```bash
multipass-rs init <dir>
multipass-rs --ship <path> mine <dir>
multipass-rs --ship <path> search "<query>"
multipass-rs --ship <path> status
multipass-rs --ship <path> wake-up
multipass-rs --ship <path> mcp-server
```

## Project Files

The rewrite currently uses a few important filesystem artifacts:

- project-local config:
  `multipass.yaml`
- default local store:
  `ship.sqlite3` inside the chosen ship directory
- hook state:
  `~/.multipass/hook_state`

## Repo Layout

- [`.claude-plugin/`](/Users/josh/Projects/_whaleen/multipass/.claude-plugin)
  Claude Code plugin integration.
- [`.codex-plugin/`](/Users/josh/Projects/_whaleen/multipass/.codex-plugin)
  Codex plugin integration.
- [`crates/`](/Users/josh/Projects/_whaleen/multipass/crates)
  Rust rewrite workspace and test surface.

## Rewrite Notes

If you are here to work on the rewrite, use this branch for implementation work and use `main`
when you need the broader expected feature picture while parity is still being built out.

## License

MIT
