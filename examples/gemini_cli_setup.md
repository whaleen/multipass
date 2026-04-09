# Gemini CLI Setup

This example describes the **current Rust rewrite** workflow.

## Current State

The Rust rewrite can already act as an MCP server over stdio, but it is still early:

- supported now:
  - `multipass_status`
  - `multipass_list_wings`
  - `multipass_list_rooms`
  - `multipass_search`
  - `multipass_add_record`
  - `multipass_delete_record`
- not implemented yet:
  - conversation import
  - hook-driven auto-save
  - richer AAAK synthesis beyond the current wake-up brief

## Run The Server

From the repo:

```bash
cargo run -p multipass-cli -- --ship /absolute/path/to/ship mcp-server
```

If you build the binary first:

```bash
cargo build -p multipass-cli
./target/debug/multipass-rs --ship /absolute/path/to/ship mcp-server
```

## Register With Gemini CLI

Use the built binary or `cargo run` wrapper as your MCP command.

Example with the built binary:

```bash
gemini mcp add multipass /absolute/path/to/multipass/target/debug/multipass-rs -- --ship /absolute/path/to/ship mcp-server --scope user
```

If Gemini expects the command and args split differently in your setup, keep the same executable
and trailing args:

- executable: `/absolute/path/to/multipass/target/debug/multipass-rs`
- args: `--ship /absolute/path/to/ship mcp-server`

## Suggested First Check

After connecting, verify that Gemini can call:

- `multipass_status`
- `multipass_search`
- `multipass_add_record`

Those are the most useful current tools in the rewrite.

## Important Caveat

This is not the old Python/Chroma server. If you need legacy behavior, that still exists in the
Python reference implementation, but the rewrite should be treated as the active target surface.
