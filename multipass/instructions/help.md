# multipass

AI memory system. Store everything, find anything. Local, free, no API key.

---

## Slash Commands

| Command              | Description                    |
|----------------------|--------------------------------|
| /multipass:init      | Install and set up multipass   |
| /multipass:search    | Search your memories           |
| /multipass:mine      | Mine projects and conversations|
| /multipass:status    | Ship overview and stats      |
| /multipass:help      | This help message              |

---

## MCP Tools (19)

### Ship (read)
- multipass_status -- Ship status and stats
- multipass_list_wings -- List all wings
- multipass_list_rooms -- List rooms in a wing
- multipass_get_taxonomy -- Get the full taxonomy tree
- multipass_search -- Search memories by query
- multipass_check_duplicate -- Check if a memory already exists
- multipass_get_aaak_spec -- Get the AAAK specification

### Ship (write)
- multipass_add_crate -- Add a new memory (crate)
- multipass_delete_crate -- Delete a memory (crate)

### Knowledge Graph
- multipass_kg_query -- Query the knowledge graph
- multipass_kg_add -- Add a knowledge graph entry
- multipass_kg_invalidate -- Invalidate a knowledge graph entry
- multipass_kg_timeline -- View knowledge graph timeline
- multipass_kg_stats -- Knowledge graph statistics

### Navigation
- multipass_traverse -- Traverse the ship structure
- multipass_find_tunnels -- Find cross-wing connections
- multipass_graph_stats -- Graph connectivity statistics

### Agent Diary
- multipass_diary_write -- Write a diary entry
- multipass_diary_read -- Read diary entries

---

## CLI Commands

    multipass init <dir>                  Initialize a new ship
    multipass mine <dir>                  Mine a project (default mode)
    multipass mine <dir> --mode convos    Mine conversation exports
    multipass search "query"              Search your memories
    multipass split <dir>                 Split large transcript files
    multipass wake-up                     Load ship into context
    multipass compress                    Compress ship storage
    multipass status                      Show ship status
    multipass repair                      Rebuild vector index
    multipass hook run                    Run hook logic (for harness integration)
    multipass instructions <name>         Output skill instructions

---

## Auto-Save Hooks

- Stop hook -- Automatically saves memories every 15 messages. Counts human
  messages in the session transcript (skipping command-messages). When the
  threshold is reached, blocks the AI with a save instruction. Uses
  ~/.multipass/hook_state/ to track save points per session. If
  stop_hook_active is true, passes through to prevent infinite loops.

- PreCompact hook -- Emergency save before context compaction. Always blocks
  with a comprehensive save instruction because compaction means the AI is
  about to lose detailed context.

Hooks read JSON from stdin and output JSON to stdout. They can be invoked via:

    echo '{"session_id":"abc","stop_hook_active":false,"transcript_path":"..."}' | multipass hook run --hook stop --harness claude-code

---

## Architecture

    Wings (projects/people)
      +-- Rooms (topics)
            +-- Lockers (summaries)
                  +-- Crates (verbatim memories)

    Corridors connect rooms within a wing.
    Tunnels connect rooms across wings.

The ship is stored locally using ChromaDB for vector search and SQLite for
metadata. No cloud services or API keys required.

---

## Getting Started

1. /multipass:init -- Set up your ship
2. /multipass:mine -- Mine a project or conversation
3. /multipass:search -- Find what you stored
