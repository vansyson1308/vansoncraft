# Critic review: the study commands and the preview modes

*Status: 2026-11-12, a review of the code and guidance as they stood during the work; line numbers are as read then. Most findings have since been fixed: see [qc-report.md](qc-report.md) for each decision. Reviewer: the product and art critic; the code was read, not run.*

## What was reviewed

- **The eight engine commands:** `crates/engine/src/cmd/study.rs` and `study/{raster,lines,construct,geometry,tests}.rs`.
- **The `render_preview` changes:** `crates/mcp/src/tools/dispatch.rs` (`study_preview`, `render_preview`) and `definitions.rs`.
- **The method resources:** `crates/mcp/src/method/*.md`.
- **The four new prompts:** `speed-drawing`, `composition-variants`, `revise-locally` and `memory-exercise` in `crates/mcp/src/prompts.rs`.

Each finding was checked against the four reading ledgers.

## Verdict in brief

The commands work at the right level: they describe, they never change the art they read, and the two that draw add separate objects as one undo step. Their `method` strings are honest.

At first reading, six things undercut that:

1. **The agent could not see three of the "look" steps.** The flipped view and the difference maps reached it only as base64 text.
2. **`study.lines` called polyline curves straight.**
3. **The guidance turned one metric into a goal.** "Link the darks" meant "lower the mass count".
4. **The hatching guidance reversed Speed's stated default.**
5. **`study.balance` was dominated by the half-tone ground** that our own `mass` topic tells agents to lay first.
6. **Group members lost their fill rules.**

All six were fixed in the working tree while this review was written. The four code fixes (items 1, 2, 5 and 6) each have a test; the two guidance fixes (items 3 and 4) are text changes. The table below gives the evidence. What remains open is small:

- In `study.lines`, polyline joints ignore `straightTolerance`, so a block-in polygon can read as curved.
- The top-level `curvatureVariation` is still pooled across paths.
- `study.blockIn` reports no side angles or lengths.
- `study.values` shares and mass counts include antialiased edge pixels.
- `study.hatch` has no tone ramp.

## Status of the findings, re-checked against the working tree on 2026-11-12

Test names: engine tests are in `crates/engine/src/cmd/study/tests.rs`, MCP tests in `crates/mcp/src/tests/speed.rs`.

| Finding | First severity | Status now | Evidence |
|---|---|---|---|
| P1-1 Pictures returned as base64 text; `render_preview` without `flip` | P1 | Fixed | `run_command` lifts `pngBase64` into an image (`dispatch.rs:115–124`); `render_preview` takes `flip` (`definitions.rs:146`, `dispatch.rs:495`). Test: MCP `pictures_a_command_returns_come_back_as_images` |
| P1-2 Polyline curves read as straight | P1 | Fixed; one gap remains (P2-10) | `lines.rs:119–173`. Test: `a_curve_drawn_with_points_reads_as_a_curve` |
| P1-3 "Link the darks" made a target | P1 | Fixed | `mass.md:8–9` and `:18–19` ("describe the plan; they are not targets"); the phrase is gone from `prompts.rs` |
| P1-4 Hatch angle guidance against SP-VIII-15 | P1 | Fixed | `line.md:21–26` (one angle by default; straight lines only) |
| P2-1 `study.balance` swamped by a toned ground | P2 | Fixed in code | `weigh: "contrast"` against the median tone (`raster.rs:396–416`). Test: `balance_can_weigh_contrast_on_a_toned_ground`. `balance.md` still doesn't mention it (only `limits.md` does) |
| P2-2 `spread` mislabelled | P2 | Fixed in the text | Spec (`study.rs`) and `rhythm.md:12–13`. `frameShare` was not added |
| P2-3 Pooled `curvatureVariation` | P2 | Partly fixed | Per path in `perPath` (`lines.rs:243`), caveat in `rhythm.md`. The top level is still pooled (`lines.rs:264`) |
| P2-4 Block-in exceeds its tolerance; no figures | P2 | Partly fixed | `maxDeviation` (`construct.rs:189`) and the `limits.md` wording. Test: `block_in_reports_how_far_the_outline_strays`. Still no side angles or lengths |
| P2-5 Groups lose fill rules and clips | P2 | Fixed | `geometry.rs:62–126`. Tests: `a_groups_members_keep_their_own_fill_rules_and_clip_groups_are_refused`, `hatch_fills_both_lobes_of_a_figure_eight` |
| P2-6 Feather and blur export wording | P2 (overstated: see the correction there) | Fixed | `mass.md:27–30` now states SVG filter, SVG Tiny left out, PDF as an image, PNG baked; the lost-edge recipe is at `mass.md:32–36` |
| P2-7 Memory exercise as theatre for an agent | P2 | Fixed | `memory.md:10–12`; the `memory-exercise` prompt now says the agent has seen the reference |
| P2-8 Portrait silhouette optimisation | P2 | Fixed | `portrait.md:18–20` (compare on the block-in only) |
| P2-9 Extremes cited but not measured | P2 | Fixed | `study.values` `extremes` (`raster.rs:336`, `373`). Test: `balance_can_weigh_contrast_on_a_toned_ground` |
| P2-10 Polyline joints ignore `straightTolerance` | P2 | **Open** | `lines.rs:140` |
| P3-1 Squint wording | P3 | Fixed | `study.rs:10`, `mass.md:16` ("value-group map"), with no blur added |
| P3-2 `flip: "vertical"` for upside down | P3 | Fixed | `procedure.md:23–24` and the prompt (`prompts.rs:518`) use `both`, and `horizontal` for the mirror |
| P3-3 Antialiasing in `study.values` | P3 | **Open** | `raster.rs` `masses` |
| P3-4 `study.proportions` | P3 | Partly fixed | `method` added (`lines.rs:359`); no artboard reference lines |
| P3-5 Hatch tone and recipe | P3 | Partly fixed | Guidance and `weight`/`color` in the recipe. No tone ramp; `crossAngle` still defaults to +90° |
| P3-6 Locked `Construction` layer | P3 | Fixed | `rhythm.md:20–22` gives the unlock and relock steps |
| P3-7 Links to `docs/speed/` | P3 | Fixed | `docs/speed/` now exists in the working tree |
| P3-8 Wrong citations | P3 | Fixed | `procedure.md:5–6`; `mass.md:8–9` |
| P3-9 Memory caveat | P3 | Fixed | `memory.md:3–4` |
| P3-10 Accent stated as a rule | P3 | Fixed | `mass.md:22–26` |
| P3-11 No branch on a reference | P3 | Fixed | `procedure.md:12–14`, `prompts.rs:510` |
| P3-12 Preview sizes differ | P3 | Not re-checked | none |
| P3-13 `server.rs` doc comment | P3 | Fixed | `server.rs:249–259` |

The rest of this document records the findings as first found, with locations as first read. One correction to the first draft: PDF export does *not* drop feathered objects. The engine bakes them into images before the PDF writer runs (`crates/engine/src/cmd/rasterfx.rs:1–11`). The warning in `crates/pdf/src/export/paint.rs:212–214` applies only to callers that skip that step.

---

## Command by command

### `study.render` and the `render_preview` modes (`region`, `mode: gray|values`, `levels`, `thresholds`, `showTemplates`, and `flip` on the engine only)

- **Problem it solves.** It gives an agent the reduced views Speed uses for judging:
  - values without colour (SP-II-1, SP-XIII-4);
  - masses in three tones (SP-IX-2, SP-V-4) or two (SP-XIV-7);
  - the picture turned over (SP-III-8, SP-XII-22);
  - a detail zoomed in (SP-P-3, SP-III-1);
  - a template reference, shown beside the work (SP-V-1, SP-XIV-4).
- **What the app already had.** `ui.render` drew an artboard at one scale, `document.export` wrote PNG files, and `edit.colors.toGrayscale` changes the document. Turning the view (`view.rotate`) exists only in the desktop UI. Before this change nothing could zoom into a region or show value groups (MCP audit F7).
- **Could existing commands have done it?** Only by misusing them: grayscale-convert, render, then undo. That is a document edit used as a view, and it is risky with concurrent agents. New code was justified.
- **Output.** A raster view (PNG), not geometry.
- **Harm.**
  - The values view is a posterisation. The module doc (`study.rs:10`) and `mass.md:16` call it "seen as with half-closed eyes". Speed's squint lowers the tone and *widens the focus*, that is, it blurs (SP-IX-7, p. 121). Posterising keeps every small sharp detail as its own tiny patch, so a fussy picture looks less fussy to Speed's squint than to this view. The two views disagree in the cases that matter.
  - Equal-step thresholds (33.3 and 66.7) can split a designed value in two when it sits near a boundary.
- **Control, undo, verify.** It is a query and changes nothing ([queries]). `thresholds` lets the agent put the boundaries between its planned values. `scale` and `maxSide` are bounded.
- **The gap.** `render_preview` forwards `region`, `mode`, `levels`, `thresholds` and `showTemplates`, but not `flip` (`dispatch.rs:457`; schema at `definitions.rs:133–145`). Unknown arguments are refused (`dispatch.rs:330–337`), so `render_preview {flip}` is an error. The agent's only route to a flipped view is `run_command study.render`, which returns the PNG as base64 text (finding P1-1). *Fixed since:* `render_preview` takes `flip`, and `run_command` returns pictures as images.

### `study.values`

- **Problem it solves.** The share of each value group, its mean L*, how many separate masses it breaks into, and the contrast between groups. These serve Speed's three-tone foundation (SP-IX-2), "a few simple masses" (SP-V-2), large simple masses (SP-XIII-6), and contrast against complexity (SP-XIV-9).
- **What the app already had.** Nothing comparable.
- **Could existing commands have done it?** No.
- **Output.** A hint: numbers, plus an optional map.
- **Harm.**
  - The mass count is the number most likely to become a target (see P1-3). Speed values *variety* of mass (SP-XIII-1, SP-XIII-2) and praises Watteau's "clatter of spots" (SP-XIV-13). Fewer masses is not better in itself.
  - Antialiased edges make thin bands of middle value along every dark–light boundary (`raster.rs:147–186` counts every pixel). A two-value design therefore shows a small "middle" share, and a long straight edge at a fractional pixel position can even count as a middle mass. The existing test tolerates exactly this: [render] allows three greys for a one-colour ellipse.
- **Control, undo, verify.** It changes nothing. `levels`, `thresholds`, `minMass` and `region` are all explicit, and the `method` string is honest ("not of what the artist meant by them").

### `study.balance`

- **Problem it solves.** Where the darks centre, their spread over the four quarters, and how much is near-empty. It serves SP-XV-1 as a spectrum and SP-XII-5 as a rough proxy.
- **What the app already had.** Nothing.
- **Could existing commands have done it?** No.
- **Output.** A hint.
- **Harm.**
  - The weight is darkness against white: `(100 − L*) / 100` (`raster.rs:348`). Our `mass` topic tells agents to make the half-tone ground first, "a rectangle the size of the artboard" (`mass.md:13–15`, after SP-IX-4). That ground then carries most of the weight, evenly. A worked case: a ground at L* 55 with a dark band at L* 20 down the left tenth of the artboard gives an x offset of about −0.07, where the same band on white gives −0.9. Compositions built on that toned ground, which is what `composition-variants` asks agents to compare, would all read as nearly centred.
  - `balance.md:5` cites SP-XV-5, "balance the extremes of the tone scale, not the amounts", and then offers only a tool that weighs amounts.
  - The `method` string rightly disclaims attention and subject (SP-XV-7).
  - *Fixed since:* `weigh: "contrast"` weighs each pixel's difference from the median tone. `balance.md` should tell agents to use it on a toned ground; today only `limits.md` mentions it.
- **Control, undo, verify.** It changes nothing, and `region` is available. There is no way to say what the ground is.

### `study.lines`

- **Problem it solves.**
  - A length-weighted direction histogram (SP-IV-2, SP-XI-3, SP-XII-7).
  - Straight against curved share (SP-XII-10, SP-XV-2).
  - Corners and inflections (SP-VIII-6).
  - Curvature variation (SP-XI-2, SP-VII-5).
  - A `spread` figure.
- **What the app already had.** Nothing. `path.curvature` is a drawing tool, not a readout.
- **Could existing commands have done it?** Only by the agent parsing `document.node` path data itself, which is slow and error-prone.
- **Output.** A hint.
- **Harm.**
  1. **Polylines.** As first read, a line segment was never sampled, so a curve drawn as many short straight segments reported `straightShare` 1, no inflections and `curvatureVariation` 0. That covers `draw_path` points without handles and flattened SVG imports, the input agents produce most often.
     - **Now:** runs of line segments whose joints turn by less than `cornerAngle` read as a sampled curve, with inflections (`lines.rs:119–173`).
     - **Still wrong:** a joint counts as a bend at any turn above 1e-4 rad (`lines.rs:140`), whatever its length and whatever `straightTolerance` says (P2-10).
  2. **Pooled curvature.** The top-level `curvatureVariation` pools every curved sample of every path and is not normalised by size (`lines.rs:195–207`, `264`). Two circles of different radii read as "varied", while Speed's variety is *along one line* (SP-XI-2, SP-XI-5). A per-path value is now in `perPath` (`lines.rs:243`).
  3. **Noise looks like variety.** Any wiggle raises the coefficient of variation. An agent told that variety gives life would be rewarded for fussiness, which Speed warns against (SP-XI-6, SP-XII-18).
  4. **`spread` is mislabelled.** It is 1 for an even mix of verticals and horizontals, and the test asserts it as "perpendicular lines spread every way" (`tests.rs:152`). The spec and `rhythm.md:12–13` call 1 "every direction". Speed's calmest arrangement (verticals plus horizontals, Diagram XXVIII; SP-XVI-4, p. 236) therefore reads as maximally scattered.
- **Control, undo, verify.** It changes nothing; `straightTolerance` and `cornerAngle` are exposed, and the method string is honest about what it counts.

### `study.proportions`

- **Problem it solves.** Sizes in units of one object's height, plumb and level alignments, and angles between centres. It serves SP-VIII-2 and SP-VIII-3, and SP-XVII-5 when the marks are placed by the user.
- **What the app already had.** `doc_inspect` already gives bounds. An agent could do this arithmetic itself.
- **Value.** The value is consistency and fewer tokens, plus the clustering of alignments. This is the least necessary of the eight.
- **Output.** A hint.
- **Harm.**
  - Snapping to round units ("exactly 7 heads") would be canon worship (SP-XVI-1). `proportion.md` warns against it.
  - Its reply has no `method` string, unlike the other queries (`lines.rs:326–332`).
  - It ignores the artboard, so it cannot say "this line sits on the centre" (SP-XII-27) or "find the half" (SP-VIII-4).
- **Control, undo, verify.** It changes nothing; `unit` and `tolerance` are explicit.

### `study.compare`

- **Problem it solves.** Silhouette overlap (`iou`, `extra`, `missing`), aspects, centroid offset and a diff map. It serves accuracy drills (SP-II-5, SP-VI-1), silhouette observation (SP-V-10) and the memory exercise (SP-XVIII-5).
- **What the app already had.** `document.compare` gives a pixel RMSE between the whole artboard and an image *file*. It compares no shapes and does no normalising.
- **Could existing commands have done it?** Only destructively: pathfinder intersect and unite, then a measurement, then undo. New code was justified.
- **Output.** A hint, plus an optional map.
- **Harm.**
  - This is the one command whose number invites "keep adjusting until it is high".
  - `portrait.md:18` recommends it for portraits against "the reference silhouette". That is a direct invitation to optimise toward a mechanical record of a photograph. Speed rejects that record for portraits (SP-XVII-11), for students using the camera (SP-XXI-1), and for any expressive study (SP-VI-1).
  - As first read, groups were filled as one non-zero shape: even-odd holes filled solid, siblings wound opposite ways cancelled, and clipping groups counted what they hide. The working tree now keeps each member's rule, merges the members' spans, and refuses clip groups (`geometry.rs:62–126`, `328–342`). See P2-5.
  - Normalising by height and bounding-box centre is a reasonable choice and is stated.
- **Control, undo, verify.** It changes nothing. The method string says "silhouettes, not likeness", which is good.

### `study.blockIn`

- **Problem it solves.** A straight-line envelope and long axis as construction lines on a locked, non-printing template layer that exports leave out. It serves the surveyor's block-in (SP-VIII-5), scaffolding that is easy to kick away (SP-VIII-7), the setting out (SP-IX-3), and "square lines" (SP-XVII-5). Its `axes` also serve implied verticality (SP-XII-12), and `hull` gives the envelope of a cluster (SP-XIII-12).
- **What the app already had.** `object.path.simplify` (it returns curves) and template layers.
- **Is automating it right?** Speed's block-in is an act of *observing*: the student finds the flats and compares angles with the model. Run on one's own drawing it is a summary. It earns its place when a drawing's envelope is compared with a traced reference's envelope.
- **The method differs from Speed's.** The ring is Ramer–Douglas–Peucker vertices on the outline, then Visvalingam reduction to `maxSides` (`construct.rs:95–112`). That is an inscribed chord polygon, not Speed's lines laid *along the flattest stretches* and extended beyond their crossings (Diagram IV Fig. X). Ledger B warned of this exact difference.
- **Two smaller gaps.**
  - The Visvalingam step has no distance bound, so when `maxSides` binds, the polygon can stray further than `tolerance`. `limits.md:12–13` says "within a tolerance".
  - The reply gives side counts, not side angles and lengths (`construct.rs:170–179`), and those are what Speed compares.
  - *Since first read:* the reply has `maxDeviation`, and `limits.md` says `maxSides` can exceed `tolerance`. Side angles and lengths are still missing.
- **Output.** Geometry: editable paths, with a recipe in `data-*`.
- **Harm.**
  - With `as: "art"` the polygon can become the final contour, cutting curves into flats. Speed criticises students who do this (SP-VIII-7, p. 92).
  - It needs paths: a placed raster reference cannot be blocked in without `imageTrace.*` first, and the method texts don't say so.
- **Control, undo, verify.** One undo step ([blockin]), the source is untouched, the result survives a round trip ([round]), and it is hidden from renders unless `showTemplates` is set.
- **One friction.** The `Construction` layer is locked (`construct.rs:82`), yet `rhythm.md:18–19` tells agents to draw tying lines on it.

### `study.hatch`

- **Problem it solves.** Parallel or crossed hatching, clipped exactly to the fill area as separate editable lines, with an optional seeded `jitter`. It serves parallel shading (SP-VIII-15), separate strokes rather than zig-zag (SP-VIII-16), and the pen and etching analogues (SP-XX-13, SP-XX-14).
- **What the app already had.**
  - `stylize.scribble`, a live effect that draws back-and-forth lines. Speed says never scribble back and forth, because the line is not under control and thickens at each turn (SP-VIII-16, p. 102).
  - `brushStrokes.crosshatch`, a raster filter.
  - A blend between two lines under a clipping mask: the lines run full length under the clip.
- **Could existing commands have done it?** Not as well. The new command fills a real gap, and this is the most clearly justified of the drawing commands.
- **Output.** Geometry, with the recipe in the group's data.
- **Harm.**
  - Spacing and weight are constant, so each call gives one flat, even tint. Speed varies both for tone: "darker and closer together when depth is wanted … varying in thickness when gradation is needed" (SP-VIII-15). Uniform machine-ruled tint is close to what he calls "so mechanical a method" (SP-IX-9, p. 118).
  - `crossAngle` defaults to the first angle plus 90°, a right-angled grid.
  - The hatching is not live: edit the shape and the lines don't follow, though the recipe allows a re-run.
  - The recipe omits `weight` and `color` (`construct.rs:288`). Both can be read back from the lines.
- **Control, undo, verify.** One undo step; it refuses rather than truncates past `maxLines` and changes nothing when it refuses ([hlimit]); fill rules are respected ([holes]); seeds reproduce ([jitter]).

---

## Findings

### P1: misleading or harmful

**P1-1. The agent is told to look at images it cannot see.**

- *Where:*
  - Through MCP, the flipped view and the difference maps come back as base64 text. `run_command` returns the reply as JSON text (`dispatch.rs:105–113`; `ToolResult::json` at `tools.rs:30–32`), and `render_preview` refuses `flip` (`dispatch.rs:457`, `definitions.rs:133–145`, `dispatch.rs:330–337`).
  - `procedure.md:22` and the `speed-drawing` prompt step 5 (`prompts.rs:510–511`) send the agent to the flip.
  - `memory.md:14–15` and the `memory-exercise` prompt send it to the red/blue map.
- *Why it matters:* the agent receives a long base64 string it cannot view. It then either skips the look or reports that it looked. That is a false verification, and it also floods the context.
- *Change:*
  1. Add `flip` to the `render_preview` schema and to the forwarded keys in `study_preview`.
  2. In `run_command`, when a reply has a top-level `pngBase64`, return it as an image content block and remove it from the text, as `study_preview` already does.
  3. Add an MCP test that `run_command study.compare {image: true}` returns an image block.

**P1-2. (Was P1 at first reading; now largely fixed, and the rest is P2-10.)**

- *As first read:* `study.lines` called polyline curves 100% straight.
- *Now:* the working tree reads smooth polyline runs as curves.

**P1-3. "Link the darks" turns a count into a goal.**

- *Where:* `mass.md:18` ("Link the darks" = fewer dark masses with a larger `largestShare`), `mass.md:8–9` ("link the masses … (SP-XIV)", with no principle id), and `prompts.rs:507` ("link the darks rather than scattering them").
- *Why it matters:* no ledger principle says this. Speed asks for variety of mass (SP-XIII-1, SP-XIII-2) and praises the "pretty clatter of spots" (SP-XIV-13). An agent following this text will merge darks to lower `masses`. That is metric optimisation of the kind `limits.md` forbids, and it pushes toward one blob.
- *Change:* replace it with what Speed says: "Plan a few large, simple masses first (SP-V-2, SP-XIII-6); small spots may cross them for variety (SP-XIV-13). `masses` and `largestShare` describe the plan; they are not targets." Cite real ids.

**P1-4. The hatching guidance reverses Speed's default and asks for a mode the tool lacks.**

- *Where:* `line.md:21–22`: "choose the angle from the form (across, down, around) rather than one angle for all".
- *Why it matters:* SP-VIII-15 says the opposite. Keep the lines parallel when in doubt, because that "preserves a unity", and break the rule only for strongly marked forms such as hair, bone and strained muscle (p. 103). The same file's "Speed says" paragraph has it right ("parallel lines in an easy direction"). Agents act on the "With VansonCraft" bullet. "Around" (curved, cross-contour hatching) is also something `study.hatch` cannot draw: it makes straight lines only.
- *Change:* "Use one hatch angle across the drawing by default (SP-VIII-15); change it only where a form calls for it: down a hard form, crossed for the dark next to the light (SP-VIII-16). `study.hatch` draws straight lines only; curved hatching round a form is not available yet."

### P2: significant gaps

**P2-1. `study.balance` is swamped by a toned ground.**

- *Where:* `raster.rs:348`, read against `mass.md:13–15` and `balance.md:9–15`.
- *Change:*
  1. Add `weigh: "darkness" | "contrast"`, where contrast weighs |L* − the region's median L*|, or a given `ground` L*.
  2. Report the ground's share.
  3. Say in `balance.md` to use contrast weighting on toned grounds.

**P2-2. `spread` is wrong for horizontals plus verticals.**

- *Where:* the `study.lines` spec in `study.rs` (around line 73), `rhythm.md:12–13`, and the assertion at `tests.rs:152`.
- *Change:*
  1. Describe it as "0: one orientation; 1: no single dominant orientation (uniform, or evenly split between two)".
  2. Add `frameShare`: the length within ±3° of 0° and 90°, and of the artboard's actual diagonals. That serves SP-XI-3 and SP-XII-17 directly.

**P2-3. `curvatureVariation` is pooled across paths and rewards noise.**

- *Where:* `lines.rs:195–207`, `264`. A per-path value now exists at `lines.rs:243`.
- *Change:*
  1. Make the top-level value the length-weighted mean of the per-path values.
  2. Document that any wiggle raises it.
  3. Have `rhythm.md` read it together with `inflections`: an egg-like curve has varying curvature and *no* inflections (SP-XI-5).

**P2-4. The block-in is a chord polygon and can exceed its tolerance.**

- *Where:* `construct.rs:95–112` and `170–179`; `limits.md:12–13`.
- *Change:*
  1. Return `maxDeviation` per ring and `edges: [{angle, length}]`.
  2. Say in `limits.md` that the vertices lie on the outline and that `maxSides` can override `tolerance`.
  3. Later, add an option that fits lines to low-curvature runs and intersects them, which is Speed's Fig. X.

**P2-5. Groups lost their members' fill rules and clips. Fixed in the working tree, still untested.**

- *As first read:* `shape_of` filled a whole group as one non-zero shape and ignored `clip`. This affected `study.compare` and `study.hatch`.
- *Now:* each path or compound path keeps its rule, the members' spans are merged, and clip groups are refused with an error (`geometry.rs:62–126`, `328–342`).
- *Still to do:*
  1. Add a test: a group of two overlapping circles wound opposite ways hatches and compares as their union; a clip group is refused.
  2. Update `limits.md:13`: "exact clipping by each path's fill rule; clipping groups are refused".
  3. Note that `study.lines` still reads the clip path and the hidden parts of a clip group (`paths_in`, `geometry.rs`), which is acceptable for a line count but worth one sentence in its `method` string.

**P2-6. The export behaviour of feather and blur is stated loosely.** (Corrected: the first draft wrongly said PDF leaves these effects out. Strictly this is P3; it is now fixed.)

- *Where:* `mass.md:22–24` ("rasterises when exported") and `line.md:27–28`.
- *What actually happens:*
  - SVG export writes these effects as filters (`crates/svg/src/export/paint.rs:590`) and leaves them out of SVG Tiny (`:629`).
  - PDF export, through the engine, replaces a feathered or blurred object with an image rendered at the document's raster-effects settings (`crates/engine/src/cmd/rasterfx.rs:1–11`). In the PDF the object stops being vector.
  - PNG export bakes the effect into pixels.
  - The feather radius is one value for the whole outline.
- *Change:* state this exactly. The text now does so, at `mass.md:27–30`.

**P2-7. The memory exercise is theatre for an agent.**

- *Where:* the `memory-exercise` prompt (`prompts.rs:586–610`) and `memory.md`.
- *Why it matters:* hiding a layer does not remove the reference from the agent's context, so the agent is not drawing from memory.
- *Change:* frame it as a workflow the agent runs *for the user*. The agent places the reference and hides it, the user draws, and the agent compares only when asked. If an agent does it for itself, say plainly that it has already seen the reference. Speed's exercise trains a person (SP-XVIII-5), and ledger D's rule against claiming to measure memory still holds.

**P2-8. The portrait topic invites optimising silhouette overlap.**

- *Where:* `portrait.md:18`.
- *Change:* "Use `study.compare` only on the block-in, to catch large errors of disposition. Never iterate a finished portrait toward a higher `iou`: likeness is not the silhouette, and Speed's Holbein departs from measured fact on purpose (SP-XVII-11, SP-XVII-12)."

**P2-9. The extremes are cited but not measured.**

- *Where:* `balance.md:5` (SP-XV-5); SP-XIV-5 in `mass.md:13`.
- *Change:* add `extremes: {darkL: 1st percentile, lightL: 99th percentile, midOffset: (darkL + lightL)/2 − 50, pureShare: share at L* < 2 or > 98}` to `study.values`. The per-pixel L* is already in memory, so this costs a few lines.

**P2-10. In `study.lines`, polyline joints ignore `straightTolerance`, so the answer still depends on how a line is drawn.**

- *Where:* `lines.rs:140`. A joint between two line segments makes both segments "curved" whenever it turns more than 1e-4 rad (0.006°).
- *Why it matters:*
  - Two 100 pt segments meeting at 1° read as curved, while a cubic that turns 1° over the same 200 pt reads as straight (2° per 10 pt is the default tolerance).
  - A `study.blockIn` polygon whose corners are all under `cornerAngle` (default 30°: for example a 12-sided block-in of a round shape) reads as curved. Speed's block-in is straight lines by definition (SP-VIII-5).
  - The `method` string (`lines.rs:270`) still describes only the cubic rule.
- *Change:*
  1. Count a joint as a bend only when its turn per 10 pt, taken over the mean length of the two segments, reaches `straightTolerance`: the same rule cubics get.
  2. Report `polylineShare`.
  3. Mention polylines in `method`.
- *Tests to add:*
  - A 64-gon reads curved with low `curvatureVariation`.
  - A zig-zag polyline counts its inflections.
  - Two long segments meeting at 1° read straight.
  - A rectangle still has 4 corners.

### P3: polish

- **P3-1. Squint wording.** `study.rs:10` and `mass.md:16` call the values view the half-closed-eyes view. Say "value-group map", or add `blur` (pt) to `study.render` and use the term only with blur on.
- **P3-2. The upside-down view is a half turn.** `procedure.md:22` and `prompts.rs:510` use `flip: "vertical"` for SP-III-8. Speed's look between the legs is a half turn, which is `flip: "both"`. A vertical mirror is a different, also useful, check. `flip: "horizontal"` is the black-glass mirror (SP-IX-7, ledger B's note).
- **P3-3. Antialiasing in `study.values`.** Report `edgeShare`, the pixels within one pixel of a change of band, and leave edge pixels out of the mass count. Alternatively render the study unantialiased.
- **P3-4. `study.proportions`.** Add a `method` string. Add the artboard's edges, centre lines and halves as reference lines in `alignments`.
- **P3-5. Tone in `study.hatch`.**
  - Now: say in `line.md` to hatch each value shape separately, closer and heavier in the shadow, with `cross` only on the core shadow next to the light (SP-VIII-15, SP-VIII-16).
  - Later: a `spacingTo` / `weightTo` ramp across the shape.
  - Default `crossAngle` to a smaller offset, or say why 90°.
  - Add `weight` and `color` to the recipe.
- **P3-6. The locked `Construction` layer.** `rhythm.md:18–19` should give the steps: unlock it with `layer.setProps`, draw, lock it again. Alternatively add a `study.guideLine` command that draws onto it.
- **P3-7. References to files that don't exist.** `index.md:5`, `study.rs:2`, `resources.rs:24` and `:30`, and the comments at `dispatch.rs:121` and `:495` and `server.rs:110` point to `docs/speed/`, which does not exist in the repository. House rule 13 forbids links a reader cannot open. Land `docs/speed/` in the same PR, or drop the references.
- **P3-8. Wrong citations.**
  - `procedure.md:6` cites SP-XIX-4 for guide lines in academic studies; that is SP-XIX-1.
  - `mass.md:9` cites "SP-XIV" for a claim no XIV principle makes (see P1-3).
- **P3-9. A missing caveat.** `memory.md:3` states "Memory keeps the essential (SP-XVIII-2)" without ledger D's strong caveat: memory reconstructs and normalises, close to Speed's own "dull uniformity" (SP-XVII-4).
- **P3-10. An option stated as a rule.** `mass.md:21` gives "the strongest dark against the strongest light, in one place" as an instruction. Speed reports it as Turner's and Corot's device for unity (SP-XIV-15), and elsewhere puts the sharpest accents on non-essentials (SP-XIII-10). Present it as one option.
- **P3-11. No branch on a reference.** The `speed-drawing` prompt never asks whether there is a reference. SP-V-8 suggests line construction without one and mass study with one.
- **P3-12. Preview sizes differ.** `render_preview` sizes the same artboard differently on its two paths: a 400 pt artboard previews at 512 px by default and at 1024 px with `mode`. Comparisons of the colour and value views then change scale.
- **P3-13. Outside the study scope.** In `server.rs`, the doc comment of `with_busy_after` ("How long a call runs…") now sits above `fell_back_from`, so one function lost its doc and the other has the wrong first paragraph.

---

## Overclaiming check

As first read. The status table near the top shows that every row needing a fix has since been fixed.

| Statement | Where | Verdict | Fix |
|---|---|---|---|
| "seen as with half-closed eyes" / "is the half-closed-eyes view" | `study.rs:10`, `mass.md:16` | Overclaim: no blur or widened focus | P3-1 |
| "Link the darks" = fewer masses | `mass.md:18`, `prompts.rs:507` | Invented principle, and a metric made a target | P1-3 |
| "choose the angle from the form … rather than one angle for all" | `line.md:22` | Contradicts SP-VIII-15 | P1-4 |
| "spread … 1 = every direction" | `rhythm.md:12–13`, spec | Factually wrong for H+V | P2-2 |
| "Accent: the strongest dark against the strongest light, in one place" | `mass.md:21` | A device presented as a rule | P3-10 |
| "Memory keeps the essential" | `memory.md:3` | Missing its science caveat | P3-9 |
| "within a tolerance" (block-in) | `limits.md:12` | Not guaranteed when `maxSides` binds | P2-4 |
| "exact line clipping by the fill rule" | `limits.md:13` | Now true per member; clip groups are refused, and the text should say so | P2-5 |
| "rasterises when exported" | `mass.md:23` | Loose: SVG keeps a filter; PDF and PNG do become pixels | P2-6 (fixed) |
| `study.balance` method: "read it as one measure of how the darks are spread, not as the picture's balance" | `raster.rs:380` | Honest. Add "on a toned ground the ground dominates" | P2-1 |
| `study.values` method: "not of what the artist meant by them" | `raster.rs:327` | Honest | none |
| `study.lines` method: "whether a rhythm works is for the eye to judge" | `lines.rs:270` | Honest; it doesn't describe the new polyline rule | P2-10 |
| `study.compare` method: "silhouettes, not likeness" | `construct.rs:393` | Honest | none |
| Golden section and canons | `proportion.md:19–20`, `limits.md:21–23` | Honest: "No ratio is 'beautiful' in itself" | none |
| Symmetry | `portrait.md:20` | Honest: "Never average, beautify or symmetrise" | none |
| "They never judge a picture" | `index.md:23` | Honest, provided P1-3 is fixed | none |
| `composition-variants`: "do not pick a winner by the numbers" | `prompts.rs` | Honest | none |

No text presents a heuristic as reading the artist's intent, and none presents viewer attention as measured. The balance and limits texts disclaim both. The golden section and symmetry are not treated as better in themselves.

---

## Five principles not yet served well, and the smallest honest addition for each

### 1. Lost-and-found edges

- **Principles:** SP-IV-1, SP-V-12, SP-IX-6, SP-XIII-9, SP-III-2. This is the biggest gap between Speed's media and vector, and every ledger names it.
- **Can `stylize.feather` serve it honestly?** Only partly.
  - It is a raster effect (`RasterFx::Feather { radius }`) with *one* radius around the whole outline. It can soften a whole mass, such as a cast shadow or a far hill, but it cannot keep one stretch of an edge and lose another, which is the principle.
  - Its export behaviour differs by format: pixels in PNG; an SVG filter in SVG, left out of SVG Tiny; in PDF the whole object becomes an image rendered at the document's raster-effects settings (`crates/engine/src/cmd/rasterfx.rs:1–11`). So the PDF has no vector path for that shape at all.
- **How it must be declared.** Whenever an agent uses feather or blur, the report must say: "This edge uses Feather, a raster effect. It is drawn in pixels at the document's raster-effects resolution (`document.rasterEffectsSettings`). SVG keeps it as a filter (SVG Tiny drops it), PNG bakes it in, and in PDF the object becomes an image. In the document the path stays editable. Its softness is the same all round the shape."
- **The smallest honest addition (no new engine code).** A "lost edge" recipe in `mass.md` and `line.md`, in order of preference:
  1. **For a mass edge, a vector bridge.** Where two masses should merge, put a thin intermediate-value shape between them, or let one shape's fill be a gradient running across the edge into its neighbour's value. This is Speed's own half tone as a buffer (SP-IX-14). It is vector and exports everywhere.
  2. **For a line, a width profile.** Taper the width to zero, and leave real gaps by splitting the path (`stroke.widthProfile.set`).
  3. **Only for whole-shape softness, a declared feather,** worded as above.
  4. **A partial lost edge.** Layer a feathered copy under the crisp shape and fade the crisp one with a linear-gradient opacity mask (`transparency.makeOpacityMask`) along the stretch to be lost. It is declared as above. In PDF the feathered copy becomes an image, so check the export before offering it.

  `mass.md` now carries items 1–3 as "Lost and found edges, in order of preference", together with the per-format statement.
- **Later.** A per-path "edge softness profile" in the style of width points, with an export story for SVG and PDF, is the real feature. Until it exists, an agent must never call a vector edge "soft".

### 2. Reading a curve by its chord, inflection and farthest point

- **Principles:** SP-VIII-6, SP-VII-5, SP-XI-5. This is Speed's most Bézier-ready technique.
- **What exists.** `study.lines` counts inflections and gives a curvature variation.
- **What is missing.** It gives no chords and no apexes. An agent therefore cannot say "this span bows 0.18 of its chord, at 0.4 along", which is how Speed compares a drawn curve with the model.
- **Smallest honest addition.** In `perPath`, report `spans`: the stretches between ends, corners and inflections. For each span give `chordAngle`, `chordLength`, `apexAt` (0–1 along the chord) and `apexHeight` (as a share of the chord). The same figures would let a later "simplify with character" keep the apexes (SP-VI-3).

### 3. Tone through hatching

- **Principles:** SP-VIII-15, SP-VIII-16, SP-IX-13.
- **What has improved.** `line.md` now says to keep one angle by default and to hatch each value shape separately.
- **What is missing.** Each hatch is still one even tint, and there is no curved, cross-contour hatching.
- **Smallest honest addition.**
  - A linear `spacingTo` / `weightTo` ramp across the shape.
  - Cross-contour hatching through an untested route: `object.blend.make` between two guide curves, expanded with `object.blend.expand`. Try it and verify it before offering it in guidance.

### 4. The first impression governing the long work

- **Principles:** SP-XVIII-3, SP-XVIII-7, SP-XIX-4, SP-XIX-6, SP-II-3.
- **What exists.** Only guidance and undo. `procedure` still has no "keep the first stage" step.
- **Smallest honest addition (guidance plus existing commands).**
  1. After the block-in, `artboard.duplicate` it as "Stage 1 – first impression", and keep it.
  2. After each detail pass, run `render_preview {mode: "values", artboard}` on both artboards at the same `max_side`, and put the two images side by side in the report.
  3. Revert with undo when the big pattern has broken.
- **Later.** A document intent note and named history checkpoints.

### 5. Measuring the drawing against the reference

- **Principles:** SP-XVII-5, SP-VIII-2, SP-VIII-3, SP-XIV-4.
- **What has improved.** `mass.md` now gives the same-filter recipe: run the value studies on two regions with the same thresholds.
- **What is missing.**
  - Proportions are still measured one set at a time.
  - `study.blockIn` cannot read a placed raster reference, and no text says to trace it first (`imageTrace.makeAndExpand`, then `study.blockIn`).
- **Smallest honest addition.**
  - Let `study.proportions` take `pairs` of reference and drawing landmarks, and report each ratio twice with the difference.
  - Add one line to `line.md` about tracing a raster reference before blocking it in.
  - Landmarks stay user-placed: no face detection (SP-XVII-5, ledger D).

**Served since first read.** Balance of the extremes (SP-XV-5, SP-XIV-5) is now served by `study.values` `extremes`. A small gap remains, and it stays DEFERRED: the share of flat against gradated fill (SP-XV-3) is not reported. It could be read from the document model and labelled approximate under transparency.

## What I would not change

- Queries that never write.
- Separate, exactly clipped hatch lines.
- Construction on a non-printing template layer.
- Recipes stored in `data-*`.
- Seeded `jitter` that defaults to 0.
- Honest `method` strings.

Each of these matches Speed's own split between mechanical scaffolding and expressive work. Keep them.
