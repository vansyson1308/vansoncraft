# Acceptance matrix

> **Status:** results of 2026-11-12 on branch `claude/clever-dijkstra-2o2vgb`, Linux container
> (4 cores, 15 GB RAM). "Checked by" names the role that ran or reviewed it; roles were separate
> agents unless it says Lead.

Coverage is reported three ways, never merged: **research** (what was read and given a
disposition), **implementation** (what exists as code or guidance) and **verification** (what was
run and looked at).

| # | Criterion | Data / scenario | How measured | Threshold | Result | Evidence | Checked by |
|---|---|---|---|---|---|---|---|
| 1 | Book coverage | the whole EPUB in spine order | four readers' reading logs; every quote checked against the text by script | 21/21 chapters, Preface, Appendix, figures; every principle has a disposition | **PASS** — 21/21 chapters, Preface and Appendix read, 80+ figures viewed; 258 principle entries (X read twice, reconciled); dispositions in `traceability.md` | `ledger/`, `book-notebook.md`, `traceability.md` | Book Researchers ×4, Product/Art Critic |
| 2 | Committed features | the 8 study commands, MCP preview modes, method resources and prompts | unit and MCP tests; scenarios A–J | 100 % implemented + tested + MCP-reachable + evidence | **PASS** — 18 engine tests (`cmd::study::tests`, `tests::id_reuse`) and 8 MCP tests (`tests::speed`) pass; every command reached through MCP in scenarios A–E | test logs below; `examples/speed/` | Lead, Tester (scripted) |
| 3 | Serious defects | audit findings | runtime reproduction | no confirmed P0/P1 open in scope | **PASS** — P1 F1 and F2 fixed and tested; F2b (ids repeat across documents) mitigated by `uid` and guidance, the per-call document guard deferred (see `audit.md`) | `audit.md` | MCP auditor, Lead |
| 4 | Required workflows | scenarios A–J on the final commit | scripted MCP runs | 100 % pass | see rows A–J | `examples/speed/`, reports | Lead, Visual QC |
| 5 | Vector editability | A, B, D | `doc_inspect` tree; SVG contains named groups and paths, no `<image>` | no raster stand-ins; each flower a named group; save/reopen keeps them | **PASS** — 8 named groups per composition, every flower its own group; SVG has `data-name` per part and no `<image>`; MCP test `a_vase_of_flowers_stays_editable_part_by_part` | `vase-compositions.svg`, `vase-log.json` | Lead, Tester |
| 6 | Undo/redo | F | document JSON before/after | content, geometry, appearance, hierarchy restored | **PASS** — 29 steps undone → identical to the start; redone → identical to the end | `hard-report.json` | Tester (scripted) |
| 7 | Round trip | E, I | native: `document.json` equality; SVG: re-import and pixel diff; independent: Chromium render of the SVG vs the app's PNG | native equal except the save time stamp; SVG ≤ 1 % pixels off (edge antialiasing) | **PASS** — native equal except `metadata.modified`; SVG re-import 0.002 % of pixels off; Chromium vs app 0.007 % (vase A) and 0.39 % (hard paths, edges only) | `qa` diff images, `hard-report.json` | Tester, Lead |
| 8 | Bad input | G and fuzz | unknown args, unknown/deleted ids, wrong types, huge numbers, corrupt and truncated files, unwritable paths | refused with a clear error; no crash; document unchanged | **PASS** — all refused with named reasons; server alive; document unchanged. One wording issue (N3, P3) | `hard-report.json` | Tester (scripted) |
| 9 | MCP live mode | H | desktop app under Xvfb + `--connect`; `doc_inspect.backend`, `uid`, `ui_screenshot` vs state; close the app; wrong port | right app, right document, change seen in the UI | PENDING | `desktop-report.json` | Lead |
| 10 | Client compatibility | Claude Code CLI (real model) | `claude -p` with the server as its only MCP server | the client completes a drawing brief through the tools | PENDING (first run passed on a stale binary) | `client/` transcript summary | Lead |
| 11 | Build and repo checks | final commit | `cargo fmt --check`, `cargo clippy -D warnings` (engine, mcp, cli), tests of the touched crates, `cargo xtask layers`, `cargo xtask assets`, wasm check of the engine | all pass | PENDING | logs | Lead |
| 12 | Visual QC | the exported files of A–D | thumbnail, 100 %, zoomed regions; rubric 1–5 per criterion | every applicable criterion ≥ 4, no major visual defect | PENDING | `qc-report.md` | Visual QC (independent) |
| 13 | Handoff | docs | a reader can install, connect, roll back, see limits | complete | PENDING | `handoff.md`, `mcp-clients.md` | Lead |

## Scenarios

| Scenario | What ran | Result |
|---|---|---|
| A — from a blank page through MCP | three artboards, layers, 75–77 steps per composition in one atomic `command_batch` each, previews, value and balance studies, native + SVG + PNG saved and reopened | **PASS**, 28 tool calls for all three compositions |
| B — line and mass | one subject as a line study (strokes only, width profiles, hatching down the form, block-in on the Construction layer) and as a mass study (fills only, three values) | **PASS**; the first line study showed stems through the vase and petal bases through the centres (no overlapping) — fixed by drawing on paper-white fills; the hatching was pulled inside the contour |
| C — three compositions | A diagonal rise, B low spread, C tall S-curve, each with its recipe in `vase-log.json` | **PASS**; v1 → v2 revision guided by `study.values` (light share 2–3 % in 3–4 scattered masses → 4–5 %) |
| D — local revision | stronger line at the accent, background texture removed, darks linked; before/after `doc_inspect` diff | **PASS** after one caught error: the first run edited composition C's group (names repeat across compositions); scoped search fixed it. Final: only the targets and their containers changed; dark masses 4 → 2; one undo restores all. Side effect reported: the accent contours split the light mass 3 → 9 |
| E — hard paths | open S with width profile, cusp, near-duplicate anchors, self-intersecting figure eight, compound hole, nested rotated and scaled groups, clipped cross-hatching | **PASS**; found and fixed N1 (hatch refused the figure eight) and N2 (1e-14 overshoot) |
| F — history and persistence | 29 undo/redo steps; save to an unwritable path; corrupt and truncated files | **PASS** |
| G — MCP errors and concurrency | schema errors, ids, types, failed atomic batch, two documents | **PASS**; concurrency with a person: see H |
| H — desktop | see row 9 | PENDING |
| I — real exports | SVG rendered by Chromium (independent of the app) and compared | **PASS** |
| J — the book's workflow | the `speed-drawing` procedure on the vase brief (A–D) | **PASS** for the procedure steps; artistic quality: see row 12 |

## Test runs

| Command | Result |
|---|---|
| baseline `cargo test -p vectorcraft-mcp` (commit `2304b38f`) | 168 + 5 + 15 passed |
| `cargo test -p vectorcraft-mcp` (this branch, first run) | 174 passed, 1 failed (a fixed-resource count of 4 → 5, updated) |
| `cargo test -p vectorcraft-engine --lib -- study id_reuse` | 18 passed |

## Performance

PENDING (see `handoff.md` for the protocol used).

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-12 | major | First version. |
