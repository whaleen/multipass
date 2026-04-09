---
name: multipass
description: multipass — mine projects and conversations into a searchable memory ship. Use when asked about multipass, memory ship, mining memories, searching memories, or ship setup.
allowed-tools: Bash, Read, Write, Edit, Glob, Grep
---

# multipass

A searchable memory ship for AI — mine projects and conversations, then search them semantically.

## Prerequisites

Ensure `multipass` is installed:

```bash
multipass --version
```

If not installed:

```bash
pip install multipass
```

## Usage

multipass provides dynamic instructions via the CLI. To get instructions for any operation:

```bash
multipass instructions <command>
```

Where `<command>` is one of: `help`, `init`, `mine`, `search`, `status`.

Run the appropriate instructions command, then follow the returned instructions step by step.
