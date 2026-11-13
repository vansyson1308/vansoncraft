# Handoff: status, limits, backlog, checkpoint and rollback

> **Status:** 2026-11-13. Updated with the final results of `acceptance-matrix.md` and the first
> release, [0.25.0](https://github.com/vansyson1308/vansoncraft/releases/tag/v0.25.0) (a pre-release).

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
| MCP `expect_document` guard | IMPLEMENTED, TESTED: checked by the app (or the headless session) as each command runs, 2026-11-13 |
| MCP app identity (`app.identity`, restart detection), `setup`, `doctor` | IMPLEMENTED, TESTED; Claude Code 2.1.322 CLIENT VERIFIED in desktop mode on Linux (`mcp-clients.md`) |
| Example artworks (`examples/speed/`) | VISUALLY VERIFIED (three QC rounds, `qc-report.md`); every SVG rendered by Chromium matches the app's PNG |
| Atomic `command_batch` rollback | IMPLEMENTED, TESTED |
| Engine: no id reuse after undo (`unique_ids`, on in MCP headless and in the desktop app with `--control`) | IMPLEMENTED, TESTED |
| Method resources and four prompts | IMPLEMENTED, TESTED (protocol), CLIENT VERIFIED (Claude Code 2.1.322 read and used them, `mcp-clients.md`) |
| Desktop live mode (H) | TESTED on Linux under Xvfb with a software GPU (`mcp-clients.md`) |
| Release 0.25.0 packages (Windows x64 MSI and portable ZIP, Linux x86_64 tar.gz and AppImage) | TESTED before publishing on fresh Windows Server 2025 and Ubuntu 24.04 runners:<br>• the scripted MCP client;<br>• `setup`/`doctor`;<br>• the desktop bridge.<br>After publishing, from the public URLs:<br>• checksums and the headless MCP client again on both runners (ZIP, tar.gz);<br>• the full Linux check in a Linux container (`mcp-clients.md`). |
| Claude Desktop, Codex, clients on Windows | BLOCKED here (no such client, credentials or Windows desktop); configuration written from the official docs |
| Lost-and-found (soft) edges as a vector primitive | DEFERRED: no honest vector primitive; guidance offers an intermediate value shape, a gradient across the edge, or a declared feather effect |
| F4 (undo group left open on disconnect), F5, F8, F9, F12, F14, F16–F18 | DEFERRED (see `audit.md`) |
| Menu placement of the study commands in the desktop app | DEFERRED: they run from the command palette, the control channel, the CLI and MCP; menus need translations for every language |

## Known limitations

- The study numbers describe the picture (tone, darkness distribution, geometry, silhouettes). They
  do not read intent, attention or quality, and the method notes say so.
- `study.values` and `study.balance` measure the region as exported on white; transparent areas
  count as white.
- `study.blockIn` may cut corners of the outline by up to its tolerance; with `hull` the convex
  envelope can cut inside concave parts.
- `study.hatch` makes straight lines only; it refuses more than `maxLines` instead of thinning them.
- A command that runs other commands itself (`command.batch`, an action, a script) is checked
  against `expect_document` once, when it starts.
- `unique_ids` is on only where an agent drives the session (MCP headless, the app with
  `--control`): an action recorded there and played in a window without `--control` after an undo
  can bind other ids.
- Headless sessions have no autosave: save with `save_file`.
- An SVG export of a document with several artboards uses the first artboard's viewBox; export each
  artboard (`export {artboard}`) for separate files.

## Backlog (ranked)

1. Closing a client's undo group when its connection drops (F4).
2. Stop a cancelled `command_batch` between steps (F5).
3. Default bounds for `doc_inspect` and `screenshot` (F8, F9); compact JSON replies.
4. Menu entries and translations for the study commands.
5. A width-profile helper that tapers a stroke where a contour is "lost" (SP-IV-1, SP-VIII-11).
6. A "pet forms" check: pairwise `study.compare` over invented shapes to flag near-duplicates
   (SP-XIII-2).
7. An SVG export of a whole multi-artboard document with a viewBox spanning every artboard (today:
   the first artboard's; export each with `export {artboard}`).
8. The example art's cosmetic defects from QC round 3 (`qc-report.md`): a stem loop in C, an arch
   along a leaf, petal tips, the accent outline's heavy side, stem ends at 7–8×.

## Checkpoint

- Branch `claude/clever-dijkstra-2o2vgb`, based on `main` at `2304b38f`.
- Reproduce the build and tests:

```bash
cargo build -p vansoncraft-cli
```

```bash
cargo test -p vansoncraft-mcp
```

```bash
cargo test -p vansoncraft-engine --lib -- study id_reuse
```

## Rollback

Everything is additive except four behaviour changes; each is one place to revert:

| Change | Where | Revert by |
|---|---|---|
| Failed atomic batches roll back | `crates/mcp/src/tools/dispatch.rs` (`command_batch`) | pass `keep_on_error: true` per call, or revert the `roll_back` lines |
| Small artboards preview larger | `crates/mcp/src/tools/dispatch.rs` (`render_preview`, `PREVIEW_FLOOR`) | set the clamp back to 1.0 |
| New ids stay above undone ones (MCP headless only) | `crates/mcp/src/headless.rs` sets `session.unique_ids`; the floor lives in `crates/engine/src/doc_state.rs` (`id_floor`), `session/edit.rs`, `cmd/edit.rs`, `cmd/symboledit.rs` | drop the `unique_ids = true` line (the engine then behaves as on `main`), or remove the `id_floor` lines |
| Fallback notice | `crates/mcp/src/server.rs`, `apps/vansoncraft-cli/src/main.rs` | drop `fell_back_from` |

Or revert the whole branch: the pull request is a set of commits on top of `main`.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-13 | minor | The first release, 0.25.0, and what its packages were tested with. |
| 2026-11-13 | minor | Desktop mode: `expect_document` checked by the app, `unique_ids` with `--control`, app identity and restart detection; `setup` and `doctor`; a desktop Claude Code run. |
| 2026-11-12 | major | First version. |
