# MCP Setup

This example is for the Rust rewrite.

## Start The Server

From the repo root:

```bash
cargo run -p multipass-cli -- --ship /absolute/path/to/ship mcp-server
```

Or after building:

```bash
cargo build -p multipass-cli
./target/debug/multipass-rs --ship /absolute/path/to/ship mcp-server
```

## Add It To A Client

Example shape for an MCP client that launches a command:

- executable: `/absolute/path/to/multipass/target/debug/multipass-rs`
- args: `--ship /absolute/path/to/ship mcp-server`

For clients that support a shell wrapper, the equivalent is:

```bash
cargo run -p multipass-cli -- --ship /absolute/path/to/ship mcp-server
```

## Current Tool Surface

- `multipass_status`
- `multipass_list_wings`
- `multipass_list_rooms`
- `multipass_search`
- `multipass_add_record`
- `multipass_delete_record`

## What The Tools Are Good For Right Now

- `multipass_status`
  - quick ship health and record counts
- `multipass_search`
  - text retrieval over the current SQLite-backed store
- `multipass_add_record`
  - manual persistence from an MCP-capable client
- `multipass_delete_record`
  - cleanup of bad manual records

## What This Example Does Not Promise

The rewrite does not yet provide:

- legacy Python/Chroma parity
- transcript import over MCP
- hook-driven automatic save flows
- richer write tools beyond manual record add/delete
