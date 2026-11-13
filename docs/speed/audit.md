# Audit and baseline (November 2026)

> **Status:** audit of commit `2304b38f` (the fork's `main`) with runtime reproductions; fixes on branch
> `claude/clever-dijkstra-2o2vgb`. Findings carry the status they have in this branch.

Scope: what an AI agent needs to create and edit real vector art through MCP — Bézier editing,
paint, transforms, booleans and clipping, selection and hierarchy, undo, persistence, SVG, the MCP
server, errors and limits. Method: a code audit of `crates/mcp`, `apps/vansoncraft-cli` and the
control server by an independent agent (read-only), then reproduction of each claim against a
debug build through the real MCP stdio protocol. Historical parity numbers and test counts in other
docs were not taken as evidence.

## Baseline

| Item | Value |
|---|---|
| Repository | `vansyson1308/vansoncraft`, a fork of VansonCraft (product name, crates and file format still `vansoncraft`) |
| Commit audited | `2304b38f19378954c8e874a82fa8735f9bce48c4` (`main` = `origin/main` at the start) |
| Working tree | clean; no open pull requests or issues on the fork |
| Machine | Linux container, 4 cores, 15 GB RAM, about 30 GB free disk; Rust 1.97.0 |
| Build | `cargo build -p vansoncraft-cli` (dev profile): 9 min 24 s from cold, target dir 4.9 GB |
| Tests run at baseline | `cargo test -p vansoncraft-mcp`: 168 + 5 + 15 passed, 0 failed (4 min 13 s) |
| Not run | the full `cargo xtask ci` (a workspace test build needs about 30 GB of target dir, more than the disk); wasm build; macOS and Windows builds |

Branding: the app, crates, CLI, file format and MCP server are still named VansonCraft
(`vansoncraft-cli`, `.vansoncraft`, `vansoncraft://` URIs), with `drawcraft` aliases for old files.
These are technical names that file formats, configs and clients depend on; nothing was renamed.
`LICENSE-*`, `NOTICE`, `ASSETS.md` and `ATTRIBUTION.md` are kept as they are.

## Capability table

| Capability | Where | Verified by (this audit) | Gap |
|---|---|---|---|
| Bézier paths from points or SVG `d`, handles | `path.create`, MCP `draw_path` | scenarios A, B, E through MCP; SVG rendered by Chromium matches the app's PNG (0.39 % of pixels differ, edges only) | handles are absolute points: now stated in the schema (F11) |
| Fill, stroke, width profiles | `paint.*`, `stroke.set`, `stroke.widthProfile.*` | scenarios B and D; Chromium comparison of width profiles | — |
| Transforms, nested groups | `object.rotate`, `object.scale`, `transform` | scenario E (rotated group inside a scaled group), SVG re-import 0.002 % pixel difference | `draw_shape.rotation` is clockwise, `transform.rotate` counter-clockwise (F13) |
| Booleans, compound paths, clipping | `object.pathfinder.*`, `object.compoundPath.make`, `object.clippingMask.make` | scenario E (donut, clipped cross-hatching) | — |
| Selection, groups, layers, stacking, artboards | `select.*`, `object.group`, `layer.*`, `layer.move`, `artboard.*` | scenarios A, C, D | names may repeat across artboards: scope `document.find` with `within`/`intersects` |
| Undo and redo | `edit.undo`, `edit.redo`, undo groups | scenario F: 29 steps undone and redone, document JSON identical | an undone object's id was handed to the next new object (F2, fixed) |
| Native save and reopen | `document.save`, `document.open` | scenario E: reopened document equal except `metadata.modified` (stamped on save) | — |
| SVG export and import | `document.export`, `document.open` | scenario I: Chromium render of the exported SVG vs the app's PNG export | an SVG export of a several-artboard document holds the first artboard's viewBox; export each artboard for separate files |
| Autosave / recovery | desktop Data Recovery (`cmd/recovery.rs`) | not exercised | headless MCP sessions have none: unsaved headless work is lost when the server stops (stated in the fallback notice, F1) |
| MCP server, 33 tools, resources, prompts | `crates/mcp` | protocol tests; scenarios A–G | see findings |
| Errors and limits | engine params checks, MCP caps | scenario G: unknown argument → −32602, unknown or deleted id → error naming it, non-number → error, huge numbers → refused, corrupt and truncated files → error, server alive | the message for a non-number `x` says "missing number" (N3) |
| Performance budgets | `vansoncraft-cli perf` | see `acceptance-matrix.md` | — |

## Findings

Severity: **P0** data loss, corruption or a serious permission bypass; **P1** a main workflow wrong or
unusable, the wrong object or document edited, a reproducible crash; **P2** a functional bug with a
workaround, or a significant performance or UX obstacle; **P3** polish.

No P0 was found: the automation read/write roots, request and reply caps and the panic guard held
up in the code and in the runs.

| # | Sev | Finding | Reproduction (MCP, headless unless noted) | Expected / actual (before) | Status |
|---|---|---|---|---|---|
| F1 | P1 | With no flag, `vansoncraft-cli mcp` falls back to headless silently when no app listens on 127.0.0.1:7979; only the `initialize` instructions end with "Backend: headless." | start the server with no app; `draw_shape` → success; nothing in any window; ending the client loses the art | the agent and user can tell / they could not | **Fixed.** The fallback is stated in `initialize` and after the first tool result; `doc_inspect` reports `backend {kind, describe, desktopWindow}`. Tests `a_server_that_fell_back_says_so_once`, `doc_inspect_says_which_backend_the_tools_act_in`. |
| F2 | P1 | After undo, the next new object took the undone object's id, so a stale id silently edited the new object | `draw_shape` rect (id 2) → `undo` → `draw_shape` ellipse → **id 2** → `set_paint {ids:[2]}` painted the ellipse (`objects: 1`) | "no such object" / the wrong object changed | **Fixed for MCP headless sessions**: the engine session option `unique_ids` keeps new ids above the highest id the document had (`DocState::id_floor`, raised by undo and redo, not by a cancelled group whose journal entries are dropped); the MCP headless session turns it on, and so does the desktop app started with `--control` (2026-11-13), the window an agent drives. Off by default otherwise, because 14 engine tests and recorded journals (the Actions panel's) rely on today's ids. Tests `engine::tests::id_reuse::*`, MCP `a_stale_id_after_undo_fails_instead_of_painting_a_new_object`. |
| F2b | P1 | Ids repeat across documents (each starts at 1) and every tool acts on the active document; in the desktop app a person can switch tabs between calls | `draw_shape` → `file.new` → `draw_shape`: both ids 2 | a guard / none | **Fixed (2026-11-13):** `doc_inspect` gives the document `uid` (unique per process) and `backend`; `command_run`, `run_command`, `invoke_menu` and `command_batch` take `expect_document: uid`, and every command such a call runs carries it as `expectDocument`, which the app checks in the frame that runs the command (the headless session in the same call), so neither a tab switched between two calls nor one switched during a batch, nor a document an earlier step opened, is ever changed: the command is refused with nothing run, and an atomic batch rolls back. Tests: `ui-egui` `control::tests::a_command_meant_for_another_document_runs_nowhere`, MCP `a_call_meant_for_another_document_runs_nothing`, `every_step_of_a_pinned_batch_checks_its_document`, `verified::pinned_commands_carry_the_document_for_the_app_to_check`. |
| F3 | P2 | `command_batch {atomic:true}` only grouped undo: a failing step left the earlier steps applied | batch `[shape.rectangle, no.such.command]` → objects 2 → 3, history "Job" | nothing applied / partial | **Fixed:** a failed atomic batch cancels its undo group and replies `rolledBack: true`; `keep_on_error: true` keeps the old behaviour. Files written and documents opened are not taken back (documented). Tests `a_failed_atomic_batch_leaves_the_document_as_it_was`, updated `an_atomic_batch_is_one_undo_step_in_a_document_it_creates`, `atomic_batch_still_closes_its_undo_group_when_results_grow_too_big`. |
| F4 | P2 | An undo group opened by an atomic batch stays open in the desktop app if the client disconnects mid-batch | (desktop) begin a group over the control channel, disconnect, edit by hand, undo once | only the hand edit undone / hypothesis: all of them | **Deferred** (needs the control server to close a connection's group; not reproduced here). |
| F5 | P2 | Cancelling a `command_batch` does not stop it between steps | `notifications/cancelled` during a long batch | stops / runs to the end (code) | **Deferred.** |
| F6 | P2 | `render_preview` never scaled up: a 32 pt artboard previewed at 32 px | `file.new {32×32}` → `render_preview` → 32×32 | judgeable / tiny | **Fixed:** small artboards are drawn up to 512 px by default and to `max_side` when given (at most 16 px/pt); page-size previews keep their size (image tokens grow with pixels). Test `a_small_artboard_previews_large_enough_to_judge`; the export-parity test now compares with an export at the same scale. |
| F7 | P2 | No image view of a region, and no grey or value view | — | — | **Fixed:** `render_preview {region, mode: gray\|values, levels, thresholds, showTemplates}` through the new `study.render` command. Test `render_preview_zooms_into_a_region_and_shows_value_groups`. |
| F8 | P2 | `screenshot {scale}` has no size cap at the MCP layer | `screenshot {scale:16}` on a page → ~124 Mpx | bounded / unbounded | **Deferred** (use `render_preview`). |
| F9 | P2 | `doc_inspect {}` returns the whole tree, pretty-printed | large documents | bounded by default / unbounded | **Deferred;** agents are told to pass `depth`, `ids` or `kind`. |
| F10 | P2 | The remote backend reconnects to whatever listens on the port, with no app identity check | restart the app between calls | warned / silent | **Fixed (2026-11-13):** `--connect` asks `app.identity` on every connection and refuses a program that isn't VansonCraft; a new launch of the app (its `instance` changed) refuses the next call once with "was restarted … call doc_inspect", since uids and ids start again from 1. Tests `verified::*`. |
| F11 | P2 | Bézier `in`/`out` handles are absolute points, but the schema didn't say | — | — | **Fixed** in the `draw_path` description and schema. |
| F12 | P3 | One invalid UTF-8 line ends the MCP session | `printf '\xff\n…'` | −32700 and continue / exit | Deferred. |
| F13 | P3 | `draw_shape.rotation` is clockwise, `transform.rotate` counter-clockwise | — | — | Documented in `docs/speed/mcp-clients.md`. |
| F14 | P3 | `set_paint`, `transform`, `pathfinder` run several requests and change the selection | — | — | Deferred. |
| F15 | P3 | `docs/mcp.md` gave stale resource and prompt counts | — | — | Fixed in `docs/mcp.md`. |
| F16 | P3 | MCP accepts 16 MiB lines, the app's control channel 4 MiB | — | — | Deferred. |
| F17 | P3 | Six alias tool pairs with different argument names enlarge `tools/list` | — | — | Deferred. |
| F18 | P3 | New art defaults to a white fill and previews render on white | — | — | Deferred; prompts pass explicit fills. |
| N1 | P2 | (new code, found in QA) `study.hatch` refused a self-intersecting figure eight: its lobes' signed areas cancel | `study.hatch` on `M0 50 C0 0 100 100 100 50 C100 0 0 100 0 50 Z` | lines in both lobes / "no area" | **Fixed:** the area test uses the scanline fill. Test `hatch_fills_both_lobes_of_a_figure_eight`. |
| N2 | P3 | (new code) hatch lines turned back from an angle overshot their shape by 1e-14 pt, so `document.find {within}` missed them | hatch at 62° on an artboard-sized rect, then `document.find {within: artboard}` | found / missed | **Fixed:** endpoints settled to 1e-6 pt inside the shape's bounds. Test in `hatch_lays_clipped_lines_inside_the_fill_area`. |
| N3 | P3 | `shape.rectangle {x: "ten"}` answers "missing number `x`" | — | "not a number" / "missing" | Deferred (engine-wide wording). |
| N4 | P3 | Agent hazard: part names repeat across compositions; a by-name search edited the wrong composition in our first revision run | scenario D, first run | caught by the before/after diff | **Guidance:** the `revise-locally` prompt scopes searches with `within`/`intersects` and diffs `doc_inspect` before and after. |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-13 | minor | F2 for the desktop app (`unique_ids` with `--control`), F2b (`expect_document` checked by the app per command) and F10 (app identity, restart) fixed. |
| 2026-11-12 | major | First audit of the fork: baseline, capability table, 18 audit findings and 4 QA findings with runtime reproductions; F1, F2 (headless), F3, F6, F7, F11, F15, N1, N2 fixed with tests; independent code review findings resolved (see qc-report.md). |
