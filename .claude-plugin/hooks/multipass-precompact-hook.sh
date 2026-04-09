#!/bin/bash
# multipass PreCompact Hook — thin wrapper calling Python CLI
# All logic lives in multipass.hooks_cli for cross-harness extensibility
INPUT=$(cat)
echo "$INPUT" | python3 -m multipass hook run --hook precompact --harness claude-code
