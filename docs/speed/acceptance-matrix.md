# Acceptance matrix

> **Status:** final results of 2026-11-12, branch `claude/clever-dijkstra-2o2vgb`, on a Linux
> container (4 cores, 15 GB RAM, Rust 1.97.0, debug builds). "Checked by" names the role that ran
> or reviewed it; the roles were separate agents unless it says Lead.

Coverage is reported three ways, never merged:

- **Research:** 21/21 chapters, the Preface and the Appendix read in EPUB order; 253 principles
  ledgered, every quote checked against the text by script.
- **Implementation:** every principle has a disposition (`traceability.md`): 53 TESTED, 6
  IMPLEMENTED, 137 GUIDANCE (in the method notes), 3 EXERCISE, 7 PRINCIPLE, 11 NOT-AUTOMATED with
  reasons, 36 DEFERRED with reasons.
- **Verification:** the automated runs and scenarios below; visual QC in three rounds; one real
  client (Claude Code) verified. Windows, Claude Desktop and Codex are NOT VERIFIED.

| # | Criterion | Data / scenario | How measured | Threshold | Result | Evidence | Checked by |
|---|---|---|---|---|---|---|---|
| 1 | Book coverage | the whole EPUB | four reading logs; quotes checked by script | 21/21 chapters, Preface, Appendix, figures; every principle dispositioned | **PASS** — 21/21 + Preface + Appendix; 80+ figures viewed; 253/253 principles dispositioned | `ledger/`, `book-notebook.md`, `traceability.md` | Book Researchers ×4, Critic |
| 2 | Committed features | 8 study commands, preview modes, method resources, 4 prompts, the audit fixes | tests + scenarios | 100 % implemented, tested, MCP-reachable, evidenced | **PASS** — 24 engine tests (20 study, 4 id) and 10 MCP tests (`tests::speed`), all commands reached through MCP in scenarios A–H | test runs below | Lead, Tester (scripted), Reviewer |
| 3 | Serious defects | audit findings, review findings | runtime reproduction, review | no confirmed P0/P1 open in scope | **PASS** with one scoped exception — F1 and F2 fixed (F2 for MCP headless sessions; desktop deferred, documented); the reviewer's three P1s fixed; F2b mitigated by `expect_document` | `audit.md`, `qc-report.md` | Auditor, Reviewer, Lead |
| 4 | Required workflows | scenarios A–J | scripted MCP runs on the final binary | 100 % pass | **PASS** (table below) | `examples/speed/` | Lead, Visual QC |
| 5 | Vector editability | A–D | `doc_inspect`; SVG text; save/reopen | no raster stand-ins; every flower its own named group; reopen keeps them | **PASS** — named groups per flower, vase, stems and leaves, lip and setting; no `<image>`; MCP test `a_vase_of_flowers_stays_editable_part_by_part` | `vase-compositions.vectorcraft`, SVGs | Lead, Tester, Visual QC |
| 6 | Undo/redo | F | `document.json` before/after | content, geometry, appearance and hierarchy restored | **PASS** — 29 steps undone → identical; redone → identical; a revision undone in one step | `vase-revision-report.json` (`undo_restores: true`) | Tester |
| 7 | Round trip | E, I | native JSON equality; SVG re-import; Chromium render of the SVG vs the app's PNG | native equal but for the save time; SVG ≤ 1 % pixels off | **PASS** — native equal except `metadata.modified`; SVG re-import 0.002 %; Chromium vs app ≤ 0.015 % on the vases, 0.17 % on the line study, 0.39 % on the hard-path sheet (edges) | `qa` diffs | Tester, Lead |
| 8 | Bad input | G, fuzz | unknown arguments, unknown/deleted ids, wrong types, huge numbers, corrupt/truncated files, unwritable paths, `command_sweep` | clear refusal, no crash, document unchanged | **PASS** — all refused with reasons; server alive; document unchanged; `command_sweep` 24/24 including the new commands | logs | Tester |
| 9 | MCP live mode | H | desktop app (Xvfb, software GPU) + `--connect` | right app, right document, change visible in the UI | **PASS** — `backend.kind: remote`; the drawing appeared in the window (`ui_screenshot`); a person's click between calls seen; closing the app fails clearly; wrong port exits | `mcp-clients.md` | Lead |
| 10 | Client compatibility | Claude Code CLI, a real model | `claude -p` with this server as its only MCP server | completes a drawing brief through the tools | **PASS for Claude Code 2.1.322 (Linux)**; Claude Desktop, Codex, Windows NOT VERIFIED | `mcp-clients.md` | Lead |
| 11 | Build and repo checks | final tree | `cargo fmt --check`; `cargo clippy -p vectorcraft-engine -p vectorcraft-mcp -p vectorcraft-cli --all-targets -D warnings`; tests below; `xtask layers`; `xtask assets` | all pass | **PASS** for these; the full `cargo xtask ci` (workspace clippy and tests, wasm) was not run here (disk); CI is relied on | logs | Lead |
| 12 | Visual QC | the exported files of A–D | thumbnail, 100 %, zoom crops, Chromium renders; rubric 1–5 | every applicable criterion ≥ 4 | **PASS in round 3** (round 1 failed the curve rhythm and three compositions; two revision rounds) | `qc-report.md` | Visual QC |
| 13 | Handoff | docs | set-up, connection, rollback, limits, examples | complete | **PASS** | `handoff.md`, `mcp-clients.md`, `README.md` | Lead |

## Scenarios (final binary)

| Scenario | What ran | Result |
|---|---|---|
| A — from a blank page | three artboards; one atomic `command_batch` per composition (75–80 steps); value and balance studies; native, SVG per artboard and PNG saved; reopened | **PASS**, 33 tool calls for all three |
| B — line and mass | the same subject as a line study (strokes with a weight hierarchy, hatching down the form, block-in on the Construction layer) and a mass study (three tones, fills only) | **PASS** |
| C — three compositions | diagonal rise, low spread, tall S-curve, each with its recipe in `vase-recipes.json` | **PASS**; v1 → v5 driven by `study.values` and three QC rounds |
| D — local revision | stronger accent line, background texture reduced, a leaf over the shoulder to join the darks; `doc_inspect` before/after | **PASS** — outside the targets only the container of the new leaf changed; one undo restores all. Honest note: the dark-mass count stays 3 → 3 (dark share 0.101 → 0.110); by eye the darks read as linked (QC round 3), while a 1–2 px seam at the rim still separates the leaves from the vase in the count. The accent outlines split the focal flower's light mass, a side effect reported to the user |
| E — hard paths | open S with width profile, cusp, near-duplicate anchors, figure eight, compound hole, nested rotated/scaled groups, clipped cross-hatching | **PASS**; found and fixed N1 and N2 |
| F — history and persistence | 29 undo/redo steps; unwritable save; corrupt and truncated files | **PASS** |
| G — MCP errors | schema, ids, types, huge numbers, failed atomic batch, two documents (`uid`, `expect_document`) | **PASS** |
| H — desktop | see row 9 | **PASS** |
| I — independent render | every example SVG rendered by Chromium and compared with the app's PNG | **PASS** |
| J — the book's workflow | the `speed-drawing` procedure: intent, block-in, three values, lines, look with `render_preview` modes, revise, structure check, save/export; and a real client following the method notes | **PASS** (artistic quality: row 12) |

## Test runs (final tree)

| Command | Result |
|---|---|
| `cargo test -p vectorcraft-mcp` | 178 + 5 + 15 passed (baseline was 168 + 5 + 15) |
| `cargo test -p vectorcraft-engine --lib -- study id_reuse` | 24 passed |
| `cargo test -p vectorcraft-engine --lib` | 5 901 passed, 1 failed, 2 ignored; the failure, `tests::textedit::the_fallback_glyph_of_a_consonant_after_its_vowel_sign_is_reported`, passes when run alone (a shared font database under parallel tests; it does not touch this change) |
| `cargo test -p vectorcraft-engine --test command_sweep` | 24 passed |
| `cargo clippy -p vectorcraft-engine -p vectorcraft-mcp -p vectorcraft-cli --all-targets -- -D warnings` | clean |
| `cargo fmt --all -- --check`, `cargo xtask layers`, `cargo xtask assets` | clean; 361 assets attributed |

## Performance

`vectorcraft-cli perf` (the 50 000-path synthetic document), six alternating runs each of the
baseline (`2304b38f`) and the final **debug** builds on the same machine; medians (and minima) in ms.
The budgets are for release builds: the debug build misses the render budgets in both versions, so
read the absolute numbers as relative only. No release build was measured here (time and disk).

| Measure | Budget | Baseline | Final | Δ median |
|---|---|---|---|---|
| render: fit page, 2880×1800 | 100 | 244.3 (226.6) | 229.3 (222.9) | −6 % |
| render: refresh after a pan | 100 | 239.9 (216.9) | 240.7 (229.7) | 0 % |
| render: 400 % zoom | 16 | 89.4 (85.0) | 91.6 (86.9) | +3 % |
| hit test per click | 2 | 0.57 (0.45) | 0.60 (0.51) | +5 % |
| save `.vectorcraft` | 300 | 108.3 (94.3) | 115.1 (106.3) | +6 % |
| open `.vectorcraft` | 300 | 331.7 (298.0) | 348.4 (299.0) | +5 % |
| export SVG | 500 | 233.3 (195.7) | 218.6 (186.5) | −6 % |
| Pathfinder Unite, 1 000 paths | 150 | 93.4 (87.0) | 103.7 (82.9) | +11 % |
| handle drag, 5 000 anchors | 8 | 5.13 (4.17) | 5.17 (4.11) | +1 % |
| segment drag, 5 000 anchors | 8 | 4.97 (4.14) | 5.29 (3.96) | +6 % |

The differences go both ways within the run-to-run spread (the minima are as close or closer), the
load average during the runs was 1.8–2.3 (the renders use every core), and the change touches none
of these paths except an O(1) branch in `Session::edit`. No regression is claimed or excluded
beyond that noise.

**MCP workflow costs** (final debug build, headless, the three-composition project; medians of 5):

| Call | Time | Reply |
|---|---|---|
| `render_preview` (400×520 artboard) | 98 ms | 201 KiB |
| `render_preview {mode: "values"}` | 76 ms | 23 KiB |
| `render_preview {region}` (4× zoom, 600 px) | 133 ms | 153 KiB |
| `command_run study.values` / `study.balance` | 68 / 71 ms | 1.2 / 0.9 KiB |
| `doc_inspect {depth: 0}` / whole tree | 0.9 / 5.6 ms | 2.9 / 307 KiB |

Three compositions took 33 tool calls; the revision 23 (including both `doc_inspect` snapshots for
the diff); the line and mass studies 60. A real Claude Code session drew, checked and revised a
vase in 47 tool calls. Ask for `doc_inspect {depth}` rather than the whole tree, and for the value
view when only tones matter.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-12 | major | First version, with the final runs. |
