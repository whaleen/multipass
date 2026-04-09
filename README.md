<div align="center">

<img src="assets/multipass_logo.png" alt="multipass" width="280">

# multipass

Local memory infrastructure for projects, conversations, and agent context.

</div>

## Overview

`multipass` is a local-first memory system for AI-assisted work.

The goal is simple:

- ingest project files and conversation history
- preserve useful context locally
- make that context searchable and navigable
- expose it to CLIs, MCP clients, and future tools

This repository is currently in transition. It began as a Python system focused on Chroma-backed memory retrieval. It is now being repositioned as `multipass`, with a broader goal: a durable local memory engine that can eventually support a Rust rewrite and cleaner architecture.

So the right way to read this repo today is:

- there is a working Python implementation here
- some product language and internals are still evolving
- the core ideas matter more than the exact current implementation

## What Multipass Does

Today, `multipass` can:

- initialize a project with a local `multipass.yaml`
- infer a default wing and room layout from a project directory
- mine project files into a local ship
- mine conversation exports in a separate ingest mode
- search stored memory with wing/room filters
- expose an MCP server for tool-based access
- maintain a lightweight memory stack for wake-up/context loading
- experiment with AAAK as a compact memory dialect

At a high level, the current flow is:

1. initialize a project
2. generate a local project config
3. mine files or conversations into a local store
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
- `crate`
  A stored verbatim memory unit.

Not every part of the current implementation uses all of these consistently yet. That is part of the active cleanup.

## Current Status

This repo currently mixes:

- working Python CLI and MCP tooling
- older research-oriented concepts
- transitional naming from the MemPalace lineage
- newer `multipass` branding and ship-based terminology

Important practical notes:

- the first real mining run can be slow because the current Python implementation bootstraps Chroma and local embedding dependencies
- project mining depends on a generated `multipass.yaml`
- the CLI is functional, but some flows still reflect the original architecture rather than the desired long-term design

## Direction

The long-term direction is not “carry old research scaffolding forever.”

The long-term direction is:

- define a clean `multipass` product model
- understand the real behavior of the current Python implementation
- separate durable concepts from incidental implementation details
- design a Rust-native rewrite around the actual product contract

That likely means:

- better storage and indexing boundaries
- clearer ingest and retrieval flows
- a more stable MCP/tool contract
- less dependency on heavyweight first-run bootstrap
- a cleaner internal model for memory, graph relationships, and agent-facing retrieval

## Quick Start

Create a virtual environment and install the package:

```bash
python3 -m venv .venv
. .venv/bin/activate
pip install -e '.[dev]'
```

Initialize a project:

```bash
python -m multipass init --yes /path/to/project
```

Mine a project into a local ship:

```bash
python -m multipass --ship /path/to/ship mine /path/to/project
```

Search it:

```bash
python -m multipass --ship /path/to/ship search "auth decisions"
```

View ship status:

```bash
python -m multipass --ship /path/to/ship status
```

Start the MCP server:

```bash
python -m multipass.mcp_server --ship /path/to/ship
```

## Basic Commands

```bash
python -m multipass init <dir>
python -m multipass --ship <path> mine <dir>
python -m multipass --ship <path> mine <dir> --mode convos
python -m multipass --ship <path> search "<query>"
python -m multipass --ship <path> status
python -m multipass --ship <path> wake-up
python -m multipass.mcp_server --ship <path>
```

## Project Files

The current implementation uses a few important filesystem artifacts:

- project-local config:
  `multipass.yaml`
- global config:
  `~/.multipass/config.json`
- default local store:
  `~/.multipass/ship`
- identity file:
  `~/.multipass/identity.txt`

Depending on the operation, Chroma and local model assets may also be created under user cache directories.

## Repo Layout

- [`multipass/`](/Users/josh/Projects/_whaleen/multipass/multipass)
  Main Python package.
- [`.claude-plugin/`](/Users/josh/Projects/_whaleen/multipass/.claude-plugin)
  Claude Code plugin integration.
- [`.codex-plugin/`](/Users/josh/Projects/_whaleen/multipass/.codex-plugin)
  Codex plugin integration.
- [`tests/`](/Users/josh/Projects/_whaleen/multipass/tests)
  Test suite.

## Rewrite Notes

If you are here to work on the rewrite, the important thing is to understand behavior, not just terminology.

The Python implementation is useful for:

- discovering the actual user flow
- inspecting the current project config contract
- seeing what metadata gets stored
- understanding how search and wake-up currently behave
- identifying what is too coupled, too slow, or too awkward to preserve

It should be treated as a reference implementation, not necessarily the final architecture.

## License

MIT
