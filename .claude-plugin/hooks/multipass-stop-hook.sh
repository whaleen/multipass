#!/bin/bash
# multipass Stop Hook — thin wrapper calling Python CLI
# All logic lives in multipass.hooks_cli for cross-harness extensibility
INPUT=$(cat)
echo "$INPUT" | python3 -m multipass hook run --hook stop --harness claude-code
