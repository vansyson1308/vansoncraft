# Review and visual QC

> **Status:** 2026-11-12. Three independent reviewers (separate agents that did not write the code
> or the art) and the Lead's decisions on each finding. The art critic's full report is
> [critic-review.md](critic-review.md).

| Review | Reviewer | Read | Findings |
|---|---|---|---|
| Code review of the diff | Independent Reviewer | all changed and new code, run read-only, with Python checks of the numerics | 3 P1, 5 P2, P3 polish |
| Product and art critique of the studies and guidance | Product/Art Critic | code, method notes, prompts, the four reading ledgers | 4 P1, 10 P2, 13 P3 |
| Visual QC of the artworks | Visual QC | exported PNGs at thumbnail, 100 % and zoomed crops; SVG and project structure; logs | scores below, 6 major and several minor defects |

## Code review: findings and decisions

| Finding | Severity | Decision | Evidence |
|---|---|---|---|
| `turn_direction` test fails: `cornerAngle`, `bendAngle`, `crossAngle` don't say which way they turn | P1 | **Fixed**: `crossAngle` says counter-clockwise; `cornerAngle`/`bendAngle` are sizes of a turn either way, listed in `NOT_A_TURN` with the reason | full engine `--lib` run (acceptance matrix) |
| A fitted scale can round to `side + 1 ulp` and be refused (A4, 841.89 pt) | P1 | **Fixed**: a hair over the side is not over | MCP test in `a_small_artboard_previews_large_enough_to_judge` (A4 values preview, `maxSide: 1000`) |
| Raising the id floor on a cancelled undo group makes journal replay diverge | P1 | **Fixed differently**: no floor on cancel; and the floor became the session option `unique_ids`, off by default (journals recorded before keep their ids) and on in the MCP headless session, where agents work and no recovery journal replays | `id_reuse::a_cancelled_undo_group_leaves_ids_as_a_replay_makes_them`, `…without_unique_ids_the_counter_follows_the_document_as_before`; the 14 engine tests that rely on today's id reuse pass unchanged |
| `study.hatch` collects every line before the `maxLines` check | P2 | **Fixed**: refused as soon as the cap is passed | `hatch_refuses_too_many_lines_and_changes_nothing` |
| Quadratic Visvalingam and span search | P2 | **Fixed**: RDP within tolerance first (coarser until ≤ 2000 points) before the reduction; `compare` walks sorted spans with a cursor | `block_in_reports_how_far_the_outline_strays`, `compare_measures_silhouette_overlap` |
| An explicit `max_side` enlarged pages up to 16× | P2 | **Fixed**: `max_side` is an upper bound; only small artboards are drawn larger | `a_small_artboard_previews_large_enough_to_judge` |
| The rollback note overstated what was undone | P2 | **Fixed**: the note names what stays (edits in a document a step switched to, files, opened documents) and says the ids are void | `a_failed_atomic_batch_leaves_the_document_as_it_was` |
| Docs claimed checks not yet run, links to files not yet there | P2 | **Fixed** in the final docs (this file, `examples/speed/`, the matrix's final runs) | — |
| `scale` read but not described for `study.values`/`study.balance` | P3 | **Fixed** | — |
| Revert and `text.discardEmpty` still hand out old ids | P3 | Deferred (`unique_ids` covers undo/redo; a revert reopens the document) | `handoff.md` |
| Greedy alignment clustering can miss a pair | P3 | Documented in the reply's `method` | — |
| "Public domain" holds in the US; Speed died in 1957 | P3 | **Fixed** wording: public domain in the United States (Project Gutenberg); quotes stay short | `README.md`, `ledger/README.md` |

Checked and found correct by the reviewer: the CIE L* formulas and inverse, thresholds and groups,
8-connected masses, percentiles, balance, line-study counts, hull, RDP, scanline fill rules, hatch
rotation and its inverse, IoU and normalisation, no-panic rules, wasm safety, prompt placeholders.

## Art critique: findings and decisions

All four P1s were accepted and fixed: pictures returned by `command_run` now come back as images
and `render_preview` takes `flip`; polylines read as curves from their joints (`bendAngle`), with
angles above it kept straight; "link the darks" is no longer a target in the guidance; the hatching
advice follows Speed's default of one parallel direction (SP-VIII-15). Of the P2s, the toned-ground
balance (`weigh: "contrast"`), the `spread` wording, per-path curvature variation, block-in
`maxDeviation`, each member's own fill rule with clip groups refused, the exact export behaviour of
feather and blur, the memory exercise framed as a workflow for a person, the portrait warning
against iterating on `iou`, and the tone-scale `extremes` were done. The 48 guidance lines the
traceability table still owed were written into the method topics (47) or deferred (1). See
[critic-review.md](critic-review.md) for the full list and [traceability.md](traceability.md) for
the resulting statuses.

## Visual QC

Rubric 1–5 per criterion; the reviewer notes it is partly subjective. A criterion below 4 needs an
actionable note.

### Round 1 (the first exported set)

| Criterion | A | B | C | A revised | Line study | Mass study |
|---|---|---|---|---|---|---|
| Answers the brief | 4 | 4 | 4 | 4 | 4 | 4 |
| Composition | 4 | 3 | 3 | 3 | 3 | 3 |
| Line quality, rhythm of curves | 3 | 3 | 3 | 3 | 3 | 3 |
| Three value groups | 4 | 4 | 4 | 4 | n/a | 4 |
| Focal point | 4 | 4 | 4 | 4 | 3 | 4 |
| Editability | 4 | 4 | 4 | 4 | 4 | 4 |

Technically clean (no gaps, slivers, holes, raster stand-ins or node bloat; hidden joins hidden).
Below the bar: the stems were nearly straight (sag 1–6 % of their length), so the brief's
priority, the rhythm of curves, came only from the vases; B touched the right edge and left the
top 43 % empty; C sat 10 pt from the bottom with its shadow cut; the revision's linking leaf read
as a collar; the studies were static and the line study's vase outweighed its focal flower; the
stems stopped behind the whole rim instead of entering the mouth; the multi-artboard SVGs showed
only the first artboard.

Changes made for round 2 (all through MCP, as before): S-bowed stems (8–22 pt) going into the vase
mouth under a front lip; petals with unequal sides; B moved left with the arch raised and the
shadow inside the frame; C raised 24 pt; the revision links the darks with two leaves over the
shoulders along the diagonal, reduces the background texture (sparser, quieter) instead of removing
it, and outlines the accent thin at the tips; the studies off centre, with a weight hierarchy
(focal flower 1.4 pt, others 0.7 pt, vase about 2 pt), hatching without specks (`minLength`), the
hatching's source kept hidden, a strict three-tone mass study; one SVG per artboard; the empty
default layer removed.

### Round 2

ROUND2
