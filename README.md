<div align="center">

<img src="assets/multipass_logo.png" alt="multipass" width="280">

# multipass

Local memory infrastructure for projects, conversations, and agent context.

</div>

> This fork is an active rewrite of MemPalace, doing the needful by giving this project the _prefered_ name, and rewriting it in Rust.
>
> For the current upstream Python implementation and full feature set, see:
> https://github.com/milla-jovovich/mempalace

## Overview

`multipass` is a local-first memory system for AI-assisted work.

This repo is the active Rust rewrite. It is not claiming full parity with upstream yet. It is the
new implementation surface for:

- project-local memory ingestion
- local ship storage
- search and recall
- MCP access
- hook-driven persistence checkpoints
- AAAK-oriented wake-up output

## Current Capabilities

Today, the rewrite can:

- initialize a project with `multipass.yaml`
- infer a default wing/room layout from a project directory
- mine project files into a local SQLite-backed ship
- search stored records with wing/room filters
- expose an MCP server over stdio
- add and delete manual records through MCP
- render an AAAK-style wake-up brief
- run save/precompact hook scripts that force persistence checkpoints

## Current Gaps

Not built out yet:

- conversation import
- transcript normalization
- richer graph/knowledge systems
- broader parity with upstream’s older feature surface

If you need those today, reference upstream.

## Core Concepts

- `ship`
  The top-level local memory store.
- `wing`
  A major partition, usually a project, topic, or workstream.
- `room`
  A focused area within a wing.
- `corridor`
  A connective or classification dimension shared across rooms.
- `locker`
  A higher-level organizational layer.
- `record`
  A stored memory unit.

## Quick Start

Build the Rust CLI:

```bash
cargo build -p multipass-cli
```

Initialize a project:

```bash
cargo run -p multipass-cli -- init /path/to/project
```

Mine a project into a ship:

```bash
cargo run -p multipass-cli -- --ship /path/to/ship mine /path/to/project
```

Search it:

```bash
cargo run -p multipass-cli -- --ship /path/to/ship search "auth decisions"
```

Render wake-up output:

```bash
cargo run -p multipass-cli -- --ship /path/to/ship wake-up
```

Start the MCP server:

```bash
cargo run -p multipass-cli -- --ship /path/to/ship mcp-server
```

## Commands

```bash
multipass-rs init <dir>
multipass-rs --ship <path> mine <dir>
multipass-rs --ship <path> search "<query>"
multipass-rs --ship <path> status
multipass-rs --ship <path> wake-up
multipass-rs --ship <path> mcp-server
```

## Files And State

Important local artifacts:

- `multipass.yaml`
  Project-local config contract
- `ship.sqlite3`
  SQLite store inside the selected ship directory
- `~/.multipass/hook_state`
  Hook counters and logs

## Repo Layout

- [`crates/`](/Users/josh/Projects/_whaleen/multipass/crates)
  Rust workspace
- [`examples/`](/Users/josh/Projects/_whaleen/multipass/examples)
  Current usage examples
- [`hooks/`](/Users/josh/Projects/_whaleen/multipass/hooks)
  Save and precompact hook scripts
- [`.claude-plugin/`](/Users/josh/Projects/_whaleen/multipass/.claude-plugin)
  Claude plugin assets
- [`.codex-plugin/`](/Users/josh/Projects/_whaleen/multipass/.codex-plugin)
  Codex plugin assets

## Testing

The rewrite uses the Rust test suite:

```bash
cargo test
```

The test surface is explicit under the crate test directories in [`crates/`](/Users/josh/Projects/_whaleen/multipass/crates).

## License

MIT
