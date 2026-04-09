# Conversation Import

## Current State

The Rust rewrite does not implement conversation import yet.

## What Exists Now

- project init
- project mining from local files
- FTS-backed search
- MCP status/search/manual record tools
- AAAK wake-up summaries

## Planned Later

- transcript normalization
- conversation mining
- agent/session import

## Reference Only

If you need to inspect the legacy behavior, look at:

- `multipass/convo_miner.py`
- `multipass/normalize.py`

Those Python files are reference material only, not the target architecture for the rewrite.
