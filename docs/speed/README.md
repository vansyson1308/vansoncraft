# Drawing with agents, after Harold Speed

> **Status:** first release of this work, 2026-11-12, branch `claude/clever-dijkstra-2o2vgb`.

This folder holds the work that makes this fork usable by Claude or Codex as a vector drawing
partner through MCP, grounded in a full reading of Harold Speed's *The Practice and Science of
Drawing* (1913; public domain in the United States, Project Gutenberg #14264).

The product experience it aims at: you connect Claude or Codex to the app, give a brief (or a
reference file), and the agent builds the drawing as editable vector objects, looks at previews,
measures what it made, corrects it, revises one part on request without disturbing the rest, and
hands back a structured project with SVG and PNG exports. Artistic judgement stays with the agent and
you; the app supplies precise tools, clear state and visual feedback.

| Document | What it holds |
|---|---|
| [audit.md](audit.md) | baseline, capability table, findings P0–P3 with reproductions and their status |
| [book-notebook.md](book-notebook.md) | the reading log and condensed notes on every chapter |
| [traceability.md](traceability.md) | every principle from the book → need → existing capability → gap → disposition → test → status |
| [ledger/](ledger/) | the full reading ledgers (principles with verified quotes, figures, caveats) |
| [acceptance-matrix.md](acceptance-matrix.md) | the quality gates, how each was measured, and the results |
| [mcp-clients.md](mcp-clients.md) | Windows set-up, Claude Code / Claude Desktop / Codex configuration, file paths, compatibility matrix, Vietnamese prompts |
| [qc-report.md](qc-report.md) | independent code review and visual QC of the artworks |
| [handoff.md](handoff.md) | what is IMPLEMENTED / TESTED / VISUALLY VERIFIED / CLIENT VERIFIED / BLOCKED / DEFERRED, known limitations, backlog, checkpoint and rollback |

## What was added

- **Study commands** (`crates/engine/src/cmd/study.rs`), shared by the desktop app (command
  palette, control channel), the CLI and MCP:
  `study.render` (any region at any zoom, in colour, grey or value groups, optionally turned upside
  down), `study.values` (value-group shares, connected masses, contrast), `study.balance`,
  `study.lines`, `study.proportions`, `study.compare`, and two that draw editable art as one undo
  step: `study.blockIn` (construction lines on a non-printing template layer) and `study.hatch`
  (exactly clipped hatching with a recorded, seeded recipe).
- **MCP:** `render_preview` zooms into regions and shows value groups; small artboards preview large
  enough to judge; `doc_inspect` reports the backend and document uid; a headless fallback is
  announced; failed atomic batches roll back; the method notes are resources
  (`vectorcraft://method`, `vectorcraft://method/{topic}`) and four workflows are prompts
  (`speed-drawing`, `composition-variants`, `revise-locally`, `memory-exercise`).
- **Engine fix:** an id an undo took away is never given to a new object.
- **Examples:** [`examples/speed/`](../../examples/speed/) — three compositions of a vase of flowers,
  a local revision, a line study and a mass study, each as a `.vectorcraft` project, SVG and PNG.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-12 | major | First version. |
