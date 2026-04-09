# multipass Status

Display the current state of the user's memory ship.

## Step 1: Gather Ship Status

Check if MCP tools are available (look for multipass_status in available tools).

- If MCP is available: Call the multipass_status tool to retrieve ship state.
- If MCP is not available: Run the CLI command: multipass status

## Step 2: Display Wing/Room/Crate Counts

Present the ship structure counts clearly:
- Number of wings
- Number of rooms
- Number of crates
- Total memories stored

Keep the output concise -- use a brief summary format, not verbose tables.

## Step 3: Knowledge Graph Stats (MCP only)

If MCP tools are available, also call:
- multipass_kg_stats -- for a knowledge graph overview (triple count, entity
  count, relationship types)
- multipass_graph_stats -- for connectivity information (connected components,
  average connections per entity)

Present these alongside the ship counts in a unified summary.

## Step 4: Suggest Next Actions

Based on the current state, suggest one relevant action:

- Empty ship (zero memories): Suggest "Try /multipass:mine to add data from
  files, URLs, or text."
- Has data but no knowledge graph (memories exist but KG stats show zero
  triples): Suggest "Consider adding knowledge graph triples for richer
  queries."
- Healthy ship (has memories and KG data): Suggest "Use /multipass:search to
  query your memories."

## Output Style

- Be concise and informative -- aim for a quick glance, not a report.
- Use short labels and numbers, not prose paragraphs.
- If any step fails or a tool is unavailable, note it briefly and continue
  with what is available.
