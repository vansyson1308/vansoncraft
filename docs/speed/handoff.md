# Handoff: status, limits, backlog, checkpoint and rollback

> **Status:** 2026-11-12, branch `claude/clever-dijkstra-2o2vgb`. Updated with the final results of
> `acceptance-matrix.md`.

## Status by item

Legend: **IMPLEMENTED** code or text exists; **TESTED** an automated test exercises it;
**VISUALLY VERIFIED** a reviewer looked at the exported result; **CLIENT VERIFIED** a real MCP client
(not a test harness) used it; **BLOCKED** cannot be done here, with the reason; **DEFERRED** worth
doing, not done in this release.

| Item | Status |
|---|---|
| `study.render` (region, zoom, gray, values, flip, templates) | IMPLEMENTED, TESTED, VISUALLY VERIFIED (previews in `examples/speed/`) |
| `study.values` (shares, masses, contrast) | IMPLEMENTED, TESTED, used in scenarios A–D |
| `study.balance` | IMPLEMENTED, TESTED |
| `study.lines` | IMPLEMENTED, TESTED |
| `study.proportions` | IMPLEMENTED, TESTED |
| `study.compare` | IMPLEMENTED, TESTED |
| `study.blockIn` (construction layer or art) | IMPLEMENTED, TESTED, VISUALLY VERIFIED (line study) |
| `study.hatch` (fill-rule clipping, seeded jitter, recipe in data) | IMPLEMENTED, TESTED, VISUALLY VERIFIED |
| MCP `render_preview` region / gray / values / scale-up | IMPLEMENTED, TESTED |
| MCP `doc_inspect.backend`, headless-fallback notice | IMPLEMENTED, TESTED |
| MCP `expect_document` guard | IMPLEMENTED, TESTED (headless); remote race documented |
| Example artworks (`examples/speed/`) | VISUALLY VERIFIED (three QC rounds, `qc-report.md`); every SVG rendered by Chromium matches the app's PNG |
| Atomic `command_batch` rollback | IMPLEMENTED, TESTED |
| Engine: no id reuse after undo (`unique_ids`, on in MCP headless) | IMPLEMENTED, TESTED; desktop app DEFERRED |
| Method resources and four prompts | IMPLEMENTED, TESTED (protocol), CLIENT VERIFIED (Claude Code 2.1.322 read and used them, `mcp-clients.md`) |
| Desktop live mode (H) | TESTED on Linux under Xvfb with a software GPU (`mcp-clients.md`) |
| Claude Desktop, Codex, Windows | BLOCKED here (no such client, credentials or machine); configuration written from the official docs |
| Lost-and-found (soft) edges as a vector primitive | DEFERRED: no honest vector primitive; guidance offers an intermediate value shape, a gradient across the edge, or a declared feather effect |
| F4 (undo group left open on disconnect), F5, F8–F10, F12, F14, F16–F18 | DEFERRED (see `audit.md`) |
| Menu placement of the study commands in the desktop app | DEFERRED: they run from the command palette, the control channel, the CLI and MCP; menus need translations for every language |

## Known limitations

- The study numbers describe the picture (tone, darkness distribution, geometry, silhouettes). They
  do not read intent, attention or quality, and the method notes say so.
- `study.values` and `study.balance` measure the region as exported on white; transparent areas
  count as white.
- `study.blockIn` may cut corners of the outline by up to its tolerance; with `hull` the convex
  envelope can cut inside concave parts.
- `study.hatch` makes straight lines only; it refuses more than `maxLines` instead of thinning them.
- `expect_document` is checked atomically only in headless mode.
- In the desktop app, an id an undo took away can still be given to a new object (`unique_ids` is
  off there until journals record it); pass `expect_document` and re-read ids after an undo.
- Headless sessions have no autosave: save with `save_file`.
- An SVG export of a document with several artboards uses the first artboard's viewBox; export each
  artboard (`export {artboard}`) for separate files.

## Backlog (ranked)

1. `unique_ids` in the desktop app with the flag recorded in the journal (F2), an atomic document
   guard inside `engine.execute` in the desktop control server (F2b), and closing a client's undo
   group when its connection drops (F4).
2. Stop a cancelled `command_batch` between steps (F5).
3. Default bounds for `doc_inspect` and `screenshot` (F8, F9); compact JSON replies.
4. App identity on reconnect (F10).
5. Menu entries and translations for the study commands.
6. A width-profile helper that tapers a stroke where a contour is "lost" (SP-IV-1, SP-VIII-11).
7. A "pet forms" check: pairwise `study.compare` over invented shapes to flag near-duplicates
   (SP-XIII-2).
8. An SVG export of a whole multi-artboard document with a viewBox spanning every artboard (today:
   the first artboard's; export each with `export {artboard}`).
9. The example art's cosmetic defects from QC round 3 (`qc-report.md`): a stem loop in C, an arch
   along a leaf, petal tips, the accent outline's heavy side, stem ends at 7–8×.

## Checkpoint

- Branch `claude/clever-dijkstra-2o2vgb`, based on `main` at `2304b38f`.
- Reproduce the build and tests:

```bash
cargo build -p vectorcraft-cli
```

```bash
cargo test -p vectorcraft-mcp
```

```bash
cargo test -p vectorcraft-engine --lib -- study id_reuse
```

## Rollback

Everything is additive except four behaviour changes; each is one place to revert:

| Change | Where | Revert by |
|---|---|---|
| Failed atomic batches roll back | `crates/mcp/src/tools/dispatch.rs` (`command_batch`) | pass `keep_on_error: true` per call, or revert the `roll_back` lines |
| Small artboards preview larger | `crates/mcp/src/tools/dispatch.rs` (`render_preview`, `PREVIEW_FLOOR`) | set the clamp back to 1.0 |
| New ids stay above undone ones | `crates/engine/src/doc_state.rs` (`id_floor`), `session/edit.rs`, `cmd/edit.rs`, `session/undo.rs`, `cmd/symboledit.rs` | remove the `id_floor` lines |
| Fallback notice | `crates/mcp/src/server.rs`, `apps/vectorcraft-cli/src/main.rs` | drop `fell_back_from` |

Or revert the whole branch: the pull request is a set of commits on top of `main`.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-12 | major | First version. |
