# Traceability: Speed's principles to VectorCraft

*Status: 2026-11-12, re-checked against the working tree after the fixes listed in `critic-review.md`. Covers every principle in the four reading ledgers (A: Preface–V, B: VI–IX, C: X–XIV, D: XV–Appendix) of Harold Speed, *The Practice and Science of Drawing* (1913, Project Gutenberg #14264).*

## How to read this

Each row is one principle id from a ledger. The columns say what the principle asks of a person or an agent, what VectorCraft already had, what was missing, how we chose to treat it, what we built or wrote, what proves it, and its status.

Code was read, not run, for this table. "Existing" names only command ids that were found in `crates/engine/src/cmd/` or `crates/effects/src/catalog.rs`. A test is cited only when its body exercises the behaviour named in the row.

**Disposition** is one of the ledgers' eight categories:

| Code | Category |
|---|---|
| DP | design-principle |
| AP | agent-prompt-or-resource |
| AO | analysis-overlay |
| EF | engine-feature |
| IE | improve-existing |
| WF | multi-step-workflow |
| PE | practice-exercise |
| NA | not-automatable |

**Status**:

| Status | Meaning |
|---|---|
| IMPLEMENTED | Code exists that serves the row, but no test exercises it for this use. |
| TESTED | Code exists and a named test exercises it (the test is cited). |
| GUIDANCE | Covered by agent guidance text, not code: the method topic named in "Our solution" already says it. |
| EXERCISE | Served by a practice exercise that exists (an MCP prompt). |
| PRINCIPLE | A design principle only: it shapes how features are built and worded. |
| NOT-AUTOMATED | Deliberately left to the person, with the reason. |
| DEFERRED | Worth building later; the row says why not now. |

**Guidance topics** are the MCP method resources `vectorcraft://method/{topic}`: `procedure`, `line`, `mass`, `rhythm`, `balance`, `proportion`, `portrait`, `memory`, `materials`, `limits`. Their text is in `crates/mcp/src/method/*.md`. The MCP prompts `speed-drawing`, `composition-variants`, `revise-locally` and `memory-exercise` are in `crates/mcp/src/prompts.rs`.

**Test tags.** Engine tests are in `crates/engine/src/cmd/study/tests.rs`; MCP tests are in `crates/mcp/src/tests/speed.rs`.

| Tag | Test |
|---|---|
| [Lstar] | `study::tests::lightness_is_cie_lstar` |
| [values] | `study::tests::values_measure_the_share_and_masses_of_each_group` |
| [vparams] | `study::tests::values_check_their_params` |
| [render] | `study::tests::render_zooms_into_a_region_and_paints_value_groups` (includes the `flip` checks) |
| [balance] | `study::tests::balance_finds_where_the_darks_weigh` |
| [lines] | `study::tests::lines_read_direction_straightness_corners_and_inflections` |
| [prop] | `study::tests::proportions_measure_in_units_and_find_plumb_lines` |
| [compare] | `study::tests::compare_measures_silhouette_overlap` |
| [blockin] | `study::tests::block_in_draws_construction_lines_on_a_template_layer_as_one_step` |
| [hatch] | `study::tests::hatch_lays_clipped_lines_inside_the_fill_area` |
| [holes] | `study::tests::hatch_follows_the_fill_rule_around_holes` |
| [hlimit] | `study::tests::hatch_refuses_too_many_lines_and_changes_nothing` |
| [jitter] | `study::tests::hatch_jitter_is_reproducible_from_its_seed` |
| [round] | `study::tests::studies_survive_a_native_round_trip` |
| [queries] | `study::tests::queries_change_nothing` |
| [mcp-region] | `tests::speed::render_preview_zooms_into_a_region_and_shows_value_groups` |
| [mcp-small] | `tests::speed::a_small_artboard_previews_large_enough_to_judge` |
| [mcp-run] | `tests::speed::the_studies_run_through_command_run` |
| [mcp-batch] | `tests::speed::a_failed_atomic_batch_leaves_the_document_as_it_was` |
| [polyline] | `study::tests::a_curve_drawn_with_points_reads_as_a_curve` |
| [groups] | `study::tests::a_groups_members_keep_their_own_fill_rules_and_clip_groups_are_refused` |
| [eight] | `study::tests::hatch_fills_both_lobes_of_a_figure_eight` |
| [toned] | `study::tests::balance_can_weigh_contrast_on_a_toned_ground` (also checks `study.values` `extremes`) |
| [deviation] | `study::tests::block_in_reports_how_far_the_outline_strays` |
| [mcp-images] | `tests::speed::pictures_a_command_returns_come_back_as_images` (`run_command` pictures, and `render_preview {flip}`) |
| [mcp-vase] | `tests::speed::a_vase_of_flowers_stays_editable_part_by_part` |

`crates/engine/tests/command_sweep.rs` also runs every registered command, the eight study commands included, with empty params and without a document. It probes no other hostile params.

**Chapter X reconciliation.** Ledger B ledgered chapter X as `SP-X-1`…`SP-X-5`, and ledger C as `SP-X-1`…`SP-X-8`. C's ids are primary. Each B id is listed once, as an alias, on the C row it matches best:

| B id | Listed on C row | Also overlaps |
|---|---|---|
| B SP-X-1 | C SP-X-1 | C SP-X-2 |
| B SP-X-2 | C SP-X-3 | none |
| B SP-X-3 | C SP-X-4 | C SP-X-5, C SP-X-6 |
| B SP-X-4 | C SP-X-7 | the moulding and canon part of C SP-X-5 |
| B SP-X-5 | C SP-X-8 | none |

---

## Preface

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-P-1 | No short cuts: teach principles and method, not subject recipes | Method guidance, not "how to draw a tree" | MCP prompts (poster, icon-set…) are task recipes | None needed | DP | Method topics are organised by method, never by subject (`limits`, index) | `method/index.md` topic table | PRINCIPLE |
| SP-P-2 | Mechanical accuracy is necessary, not sufficient; keep two quality bars apart | Check correctness separately from expression | `document.compare` (pixel RMSE against an image file) | No shape or proportion check | AO | `study.compare`, `study.proportions` for correctness only; their method strings say they don't judge expression | [compare], [prop] | TESTED |
| SP-P-3 | Words only vaguely suggest visual qualities | Render and look, not reason from text | `render_preview`, `ui.render` | Small artboards previewed tiny; no zoom | WF | `procedure` step 5 (look at page size, then `render_preview {region}`) | [mcp-small], [mcp-region] | TESTED |
| SP-P-4 | Know what you wish to do; judge by your own point of view | State intent before drawing | None | No intent record in the document | AP | `procedure` step 1; `speed-drawing` prompt step 1 | `method/procedure.md` cites it | GUIDANCE |
| SP-P-5 | Principles select from abundance, along tradition | Pick references by principle, not a style catalogue | None | None | DP | Topics cite principles; no style presets shipped | `method/index.md` | PRINCIPLE |
| SP-P-6 | Study line and mass separately, as opposite points of view | A line pass and a mass pass | Strokes, fills, layers | No mode lint | WF | `procedure` step 1 (choose line or mass study); `speed-drawing` `study` argument; `materials` (line study = strokes only) | `prompts.rs` speed-drawing values | GUIDANCE |

## I Introduction

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-I-1 | Training perfects the means; feeling is beyond teaching | Tools serve execution and never claim taste | None | None | DP | Study commands measure and draw aids only (`limits`) | `method/index.md`: "They never judge a picture" | PRINCIPLE |
| SP-I-2 | Each artist's record should differ; no single right output | Don't score artistic work by likeness to a reference | `document.compare` | Agents could optimise `iou` | DP | `limits`: never optimise a number against the brief; `study.compare` method string | `method/limits.md` | PRINCIPLE |
| SP-I-3 | Abstract form and colour carry feeling, as music does | See the structure apart from the subject | `edit.colors.toGrayscale` (an edit, not a view) | No non-destructive abstract view | AO | `study.render {mode: values}`, `render_preview {mode}` | [render], [mcp-region] | TESTED |
| SP-I-4 | Any subject: select and emphasise what carries the feeling | Name the impression; list what to stress and suppress | Simplify, paint | None in code | AP | `procedure` step 1 (one impression) | `method/procedure.md` step 1 (no emphasise/suppress list yet) | GUIDANCE |
| SP-I-5 | Matter and manner are one; no display of technique | Each effect must serve the intent | `effect.list` | None | AP | `procedure`: "every effect serves the stated impression" | Not in the text | GUIDANCE |
| SP-I-6 | Art is ordered feeling; a catalogue of details is not art | An ordering pass; see a flat hierarchy | Align, distribute | No size/contrast hierarchy readout | WF | `procedure` steps 4–7 (masses, then detail under the plan) | `method/procedure.md` | GUIDANCE |
| SP-I-7 | Representation must convince; train representation, not copying | Image Trace is a study aid, not the drawing | `imageTrace.*` | Traced output is not labelled | AP | `limits`: say when output was traced; a trace is a scaffold | Not in the text | GUIDANCE |

## II Drawing

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-II-1 | Drawing is form on a plane; form before colour | Judge values without colour, non-destructively | `edit.colors.toGrayscale` (destructive) | No luminance view | AO | `study.render {mode: gray}` (CIE L*), `render_preview {mode: gray}` | [Lstar], [queries] | TESTED |
| SP-II-2 | To express form you must first be moved by it | Commit to one perceived quality first | None | Feeling | NA | Prompted only: `procedure` (be moved by something, SP-XIX-2) | `method/procedure.md` | NOT-AUTOMATED |
| SP-II-3 | Select the significant; laboured finish loses the impression | Keep the first gesture; see where detail went | Undo, `artboard.duplicate`, `object.path.simplify` | No detail-density view; no snapshot compare | WF | `procedure` steps 2 and 7 (small variants; check the value map after detail) | `method/procedure.md` | GUIDANCE |
| SP-II-4 | Bulk and lightness are felt qualities of form | A vocabulary of named qualities, means non-binding | None | None | AP | `procedure` step 1, with bulk/lightness means and the Einfühlung caveat | Not in the text | GUIDANCE |
| SP-II-5 | Academic studies want cold, measured accuracy | Accuracy drills with figures | None | No shape-error figures | PE | `study.compare` gives `iou`, `extra`, `missing`, aspects | [compare] (the drill itself is not written) | TESTED |
| SP-II-6 | Accuracy is judged against the intended significance | Checks report, never correct | None | None | DP | Every study query changes nothing; there is no "fix" | [queries] | TESTED |
| SP-II-7 | A rough physical standard: report deviation size, not a verdict | Plausibility checks that report magnitude | Perspective grid (`perspgrid.rs`) | No perspective or ellipse consistency check | AO | Magnitudes from `study.compare` and `study.proportions`; perspective checks deferred | [compare], [prop] | TESTED |

## III Vision

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-III-1 | Learning to see is the first difficulty | Look at several scales and in reduced views | `render_preview` | No zoom, grey or value view | WF | `procedure` step 5; `render_preview {region, mode}` | [mcp-region] | TESTED |
| SP-III-2 | The flat picture is shapes of value, gradation and edge | Per-edge softness for masses | Fills, gradients, `stylize.feather` (one radius, raster) | No softness that varies along an edge | EF | `mass` step 5 (intermediate shape, gradient, or declared feather) | Deferred: no vector primitive softens part of an edge (feather is one radius, and it becomes an image in PDF and a filter in SVG). Recipe in `mass.md` (lost and found edges) | DEFERRED |
| SP-III-3 | A picture has one focal plane; suggest falloff by other means | Soften and lower contrast away from the focus | Feather, blur, opacity | No focus-falloff tool | EF | `mass`: keep one focal impression | Deferred: depth is unknown in 2D, and blur and feather turn the object into an image in PDF | DEFERRED |
| SP-III-4 | Vision is tied to touch: hardness read from edges | A hard/soft material cue vocabulary | None | None | AP | `materials`: hard = crisp edges and small highlights; soft = soft edges, broad gradation | Not in the text | GUIDANCE |
| SP-III-5 | The mental idea differs from appearance: circles become ellipses | Perspective ellipse helper; lint for round ends | `shape.ellipse`, perspective grid | No lint | IE | None in this change | Deferred: needs a declared perspective for the scene | DEFERRED |
| SP-III-6 | First drawings use touch symbols; a few darks read as a face | Don't default to outline plus symbols | None | None | AP | `portrait`: a few dark shapes read as a face before any outline (Diagram I) | `method/portrait.md` cites it | GUIDANCE |
| SP-III-7 | Finish is completeness of the idea, not detail | "Done" comes from the intent; no reward for detail | None | None | AP | `procedure`: done = the stated impression is realised | Not in the text | GUIDANCE |
| SP-III-8 | Look upside down to see shapes as shapes | A view-only inversion | `view.rotate` (desktop UI only) | No headless inverted render | IE | `study.render {flip: "both"}` is the half turn; `procedure` step 5 | [render] flip checks; [mcp-images] | TESTED |
| SP-III-9 | Line and mass converge; study them separately | Two passes; line-only and mass-only views | `view.outline` (wireframe, UI) | No mass-only view | WF | `procedure` step 1; `speed-drawing` `study` argument | `prompts.rs` | GUIDANCE |

## IV Line Drawing

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-IV-1 | Outlines are mass boundaries, lost and found | Vary edges along their length | `stroke.widthProfile.set`, `stroke.widthPoint.set` | No edge-contrast overlay | IE | `line`: accent and lost edges with width profiles | `method/line.md` cites it | GUIDANCE |
| SP-IV-2 | Line directs the eye and has a music of its own | See the dominant directions | None | No direction readout | AO | `study.lines` `directions` histogram; eye-leading kept a heuristic (`limits`) | [lines] | TESTED |
| SP-IV-3 | History: outline, then tint, then shading inside | Name outline–tint–shade as one convention | Clipping masks, Draw Inside | None | WF | `materials`: list it as one convention among others | Not in the text | GUIDANCE |
| SP-IV-4 | The eye sees only what it looks for | One view answers one question | None | None | DP | Each study command answers one question (module doc, index table) | `study.rs` header; `method/index.md` | PRINCIPLE |
| SP-IV-5 | Leonardo's invariable outline is a mental idea | Re-project reused parts for each view | Symbols | None | AP | `line`: don't reuse one silhouette at every angle | Not in the text | GUIDANCE |
| SP-IV-6 | Fusing edges moved from outline toward the visual method | Global edge fusion over an editable skeleton | Feather, blur (raster) | No vector fusion | EF | None | Deferred: raster-only today; in PDF a feathered object becomes an image | DEFERRED |
| SP-IV-7 | Accumulated detail obscures the design | Keep the skeleton legible; re-check it | `render_preview` | No skeleton view | WF | `procedure` step 7 (check the values map after detail) | `method/procedure.md` | GUIDANCE |
| SP-IV-8 | The medium suits the method: point for line, charcoal for mass | Tool choice per mode; blur isn't tone | None | None | DP | `materials` medium table | `method/materials.md` | GUIDANCE |
| SP-IV-9 | Refine by very slight departures (Parthenon curves) | A tiny, deliberate bow on straights | `path.curvatureEdit`, direct editing | No bow/entasis command | EF | None | Deferred: it risks becoming automatic jitter; it needs a per-line, deliberate control | DEFERRED |
| SP-IV-10 | A simple strong line scheme first; refinements subordinate | Lock the scheme and re-check it | Layers, lock | None | WF | `procedure` steps 3 and 7; `study.blockIn` on the `Construction` layer | `method/procedure.md` | GUIDANCE |

## V Mass Drawing

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-V-1 | Mass drawing draws the flat appearance, knowledge set aside | See a reference as value regions | `file.place {template}`, `imageTrace.*` | Templates are not measurable | WF | `study.values` and `study.render {mode: values}` with `showTemplates` read a template reference; `mass` | [blockin] (asserts `showTemplates` changes `study.values`) | TESTED |
| SP-V-2 | Reduce a complicated appearance to a few simple masses | Middle ground, a few masses, subdivide, accents last; a "few masses?" check | Shapes, layers | No mass count | WF | `mass` steps 1–4; `study.values` `masses` and `largestShare`; `speed-drawing` step 3 | [values] | TESTED |
| SP-V-3 | Eastern flat vision: flat masses, restrained line | A flat-mass style note; tapered strokes | Width profiles, brushes | None | AP | `materials`: a flat-mass convention, with the 1913 stereotype caveat | Not in the text | GUIDANCE |
| SP-V-4 | A structure of tone masses can replace an outline basis | Check it reads in three values, seen small | None | No value view | AO | `study.render {mode: values, levels: 3}`, small `maxSide` for distance | [render] | TESTED |
| SP-V-5 | One focal centre; focus nowhere sharp | Name the focus; see sharpness against it | None | No edge-sharpness overlay | AP | `mass`: keep one focal impression | `method/mass.md` cites it | GUIDANCE |
| SP-V-6 | Pure visual realism loses design | End with a design check and an accents pass | None | None | WF | `mass` step 4 (accent); `procedure` step 7 | `method/mass.md` | GUIDANCE |
| SP-V-7 | Impressionism: complex light as colour masses | Dapples as flat shapes, not shading per object | Shapes | None | AP | `mass`: render light patches as flat shapes | Not in the text | GUIDANCE |
| SP-V-8 | Direction of perception: mental to visual, or visual to mental | Branch on whether a reference exists | None | Prompt doesn't branch | WF | `procedure` step 1: with a reference, a mass study; without, line construction | `method/procedure.md` cites it; `speed-drawing` prompt asks | GUIDANCE |
| SP-V-9 | One sees with the mind; keep one conception | Derive every edit from one plan | None | None | WF | `procedure` step 1; `memory` (decide the scheme at first sight) | `method/memory.md` | GUIDANCE |
| SP-V-10 | Silhouette observation is the accurate way to see shapes | Silhouette view; overlap scores in exercises | Pathfinder unite | No overlap figure | AO | `study.compare` (`iou`, diff image); `study.render {mode: values, levels: 2}` | [compare] | TESTED |
| SP-V-11 | Contours of felt forms vs boundaries of visual masses | Two figure strategies: stacked forms, silhouette first | Shapes, pathfinder | No contour-from-stacked-forms | WF | `line`: describe both strategies | Not in the text | GUIDANCE |
| SP-V-12 | Flat pattern with lost-and-found edges; search for character | Variable edges; character accents placed by hand | Width profiles; feather (uniform) | No softness along an edge | EF | `mass` lost-and-found recipe; `line` width profiles; accents stay the artist's | Deferred: the engine feature; recipe in `mass.md` | DEFERRED |

## VI The Academic and Conventional

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-VI-1 | Accuracy studies are means; keep two kinds of study | Declare the mode; likeness metrics in study mode only | None | Nothing says when `study.compare` applies | DP | `limits`: `study.compare` is for accuracy studies and block-ins; never iterate expressive work toward a higher `iou` | Not in the text | GUIDANCE |
| SP-VI-2 | Dither: the least play on accuracy that allows life | Never ship jitter as "life" | `distort.roughen`, `stylize.scribble`, `addon.arrange.randomizeOpacity` | `study.hatch` has `jitter` | DP | `rhythm` and `limits`: jitter is not dither; `jitter` defaults to 0, is seeded and documented as style | [jitter]; `method/rhythm.md`, `method/limits.md` | GUIDANCE |
| SP-VI-3 | Copying a study in cold blood loses vitality | Simplify and trace that keep character points | `object.path.simplify`, `object.path.smooth` | No "keep character" option | IE | None | Deferred: needs apex and inflection detection fed into `simplify`; a change to an existing command | DEFERRED |
| SP-VI-4 | Only the academic can be taught | Automated critique aims at the teachable part | None | None | DP | Studies measure proportion, value, line and silhouette only | `method/index.md` | PRINCIPLE |
| SP-VI-5 | "Academic" means dead, not thorough | Don't equate polish with deadness or roughness with life | None | None | DP | `limits`: finish and style are not the measure | Not in the text | GUIDANCE |
| SP-VI-6 | All art is convention; past a point illusion kills life | Choose a convention up front | None | None | AP | `materials`: keep to the means the brief asks for (with SP-XX-2) | `method/materials.md` | GUIDANCE |
| SP-VI-7 | A borrowed manner is dead; originality is sincerity | "Style of X" must carry its reasons | None | None | AP | `limits`: state what a style's devices are for | Not in the text | GUIDANCE |
| SP-VI-8 | The material sets the convention | Use vector's own means | None | None | DP | `materials` (with SP-XX-1) | `method/materials.md` | GUIDANCE |

## VII The Study of Drawing

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-VII-1 | Two extreme approaches, studied separately | Pure line and pure mass passes | Strokes, fills | None | WF | `procedure` step 1; `speed-drawing` `study` argument | `prompts.rs` | GUIDANCE |
| SP-VII-2 | Either approach alone is incomplete | Check value structure and contour separately | None | No checks | AO | `study.values` (value structure), `study.lines` (contour) | [values], [lines] | TESTED |
| SP-VII-3 | Start at both extremes; one criterion per track | Paired line-track and mass-track exercises | None | No exercises | PE | None | Deferred: the exercise fixtures must be authored from scratch | DEFERRED |
| SP-VII-4 | Failure mode: paint used like chalk, fear of masses | Build value masses, not outlined objects | `doc_inspect` lists fill and stroke | No ratio readout | AP | `mass` step 1 (filled shapes); `materials` (mass study = filled shapes) | `method/mass.md` | GUIDANCE |
| SP-VII-5 | Flatnesses give strength to full forms | Curvature along a path; flag constant curvature | `path.curvature` (a drawing tool) | No curvature readout | AO | `study.lines` `straightShare`, `curvatureVariation` | [lines], [polyline]. Per path in `perPath`; the top level is pooled (review P2-3) | TESTED |

## VIII Line Drawing: Practical

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-VIII-1 | See the subject flat: one eye, gridded frame | A grid on the reference and one on the artboard | `view.grid` (UI), guides, `object.repeat.grid` | No proportional grid pair | IE | None | Deferred: a UI feature; `study.proportions` already measures | DEFERRED |
| SP-VIII-2 | Fix salient points against a vertical and horizontals | Plumb and level relations between landmarks | `document.inspect` bounds | No alignment finder | AO | `study.proportions` `alignments` (edges and centres within `tolerance`) | [prop] | TESTED |
| SP-VIII-3 | Measure against a unit, verticals and horizontals only | Sizes as ratios of a unit | Bounds | No unit ratios | AO | `study.proportions {unit}` `heightInUnits`, `widthInUnits` | [prop] | TESTED |
| SP-VIII-4 | Draw between limits set at the start; find the half | Limits, main axis and midpoint on a construction layer | Guides | No artboard halves in `study.proportions` | WF | `proportion`: set limits and find the half first; `study.blockIn {axis}` | Not in the text | GUIDANCE |
| SP-VIII-5 | Block in with straight lines, as a surveyor does | A straight-line envelope to compare angles with | `object.path.simplify` (curves) | No polygon envelope | EF | `study.blockIn` (chord polygon, not lines laid along the flats); `maxDeviation` | [blockin], [deviation] | TESTED |
| SP-VIII-6 | Read a curve by its chord, inflection and farthest point | Inflections, chords, apex positions | None | No apex readout | AO | `study.lines` `inflections`; `line`: anchors at extremes and inflections | [lines] (an S curve has 1 inflection) | TESTED |
| SP-VIII-7 | Scaffolding is scaffolding: kick it away | Construction kept apart, easy to hide or delete | Template layers, guides | None | IE | `study.blockIn` draws on a locked, non-printing template layer that exports leave out | [blockin] | TESTED |
| SP-VIII-8 | Observe the background shapes as carefully as the object | See the negative shapes | Pathfinder | No negative-shape overlay | AO | `line` cites it; `study.render {mode: values}` shows the negative shapes | `method/line.md` | GUIDANCE |
| SP-VIII-9 | The drawing proper: feeling, then free and swift | Find the action lines, then commit contours | None | Feeling | WF | `line` (SP-VIII-9..11) | `method/line.md` | GUIDANCE |
| SP-VIII-10 | Draw both sides of a form together | Pair contours | None | No width-between-contours view | WF | `line` | `method/line.md` | GUIDANCE |
| SP-VIII-11 | Boundaries are overlappings, accented and lost | Open overlapping strokes with tapered ends | Width profiles | None | AP | `line` (overlappings, width profiles) | `method/line.md` | GUIDANCE |
| SP-VIII-12 | In line drawing tone only serves form; keep it light | Sparse, light hatching, frontal diffuse light | None | None | AP | `line`: shade only to explain form | `method/line.md` | GUIDANCE |
| SP-VIII-13 | On a round form highlights fall inside the contour | A shading checklist | None | None | AP | `mass`: highlights inside the contour, never on the edge | Not in the text (the line.md range cite doesn't cover it) | GUIDANCE |
| SP-VIII-14 | Shadows are darkest next to the light | Core shadow beside the terminator; no radial "smoky" shadows | Gradients | None | AP | `line` (cross the lines for the dark next to the light); `mass` step 3 (no default radial gradients) | `method/line.md`, `method/mass.md` | GUIDANCE |
| SP-VIII-15 | Parallel shading, parallel by default | Hatching at one shared angle, tone by spacing and weight | `stylize.scribble` (one back-and-forth path) | No separate, exactly clipped lines | EF | `study.hatch {angle, spacing, weight}` | [hatch], [holes], [eight], [hlimit], [groups]; `line.md` keeps one angle by default | TESTED |
| SP-VIII-16 | Shading direction carries meaning: across, curved, down, crossed | Hatch modes; separate strokes, never zig-zag | Scribble (zig-zag) | No curved cross-contour hatching | EF | `study.hatch` (straight parallel lines, `cross`); curved hatching deferred | [hatch] (cross) | TESTED |
| SP-VIII-17 | Hair: swinging thick-to-thin lines, shadow masses | Width profiles that swell and thin | Width profiles | None | IE | `line`: hair as a few tapered strokes plus hatched masses | Not in the text | GUIDANCE |
| SP-VIII-18 | Appreciation comes from feeding on fine drawings | A curated study set | None | No licensed set | AP | None | Deferred: every image needs a licence check and an `ASSETS.md` row, and shipping is the owner's call | DEFERRED |
| SP-VIII-19 | Eye structure: socket, sphere, lids, brow as a tone shape | Portrait structure notes | None | None | AP | `portrait`: model the eye as a sphere; the brow as a tone shape with soft and hard edges | Not in the text | GUIDANCE |
| SP-VIII-20 | Head guide lines arch with the head's action | A head construction guide (ellipsoid) | `three-d` crate (not checked) | No guide | EF | None | Deferred: needs a 3D ellipsoid guide; the `three-d` crate is unverified | DEFERRED |

## IX Mass Drawing: Practical

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-IX-1 | Painting is drawing with a brush; effects can't fake form | No blur, glow or texture standing in for drawing | Effects | None | DP | `procedure` step 8 (no raster stand-ins); `materials` | `method/procedure.md`, `method/limits.md` (p. 112) | GUIDANCE |
| SP-IX-2 | Reduce things to three tone masses | A three-value study and check | None | No value grouping | AO | `study.values {levels: 3}`; `mass` step 1 | [values] | TESTED |
| SP-IX-3 | The setting out is mapping, but accurate | Rough polygonal masses in proportion | None | No block-in | WF | `study.blockIn`; `procedure` step 3 | [blockin] | TESTED |
| SP-IX-4 | Paint into a carefully matched middle tone | A half-tone ground | Rectangles, layers | None | WF | `mass` step 1 (half-tone ground) | `method/mass.md` cites it; `study.balance {weigh: "contrast"}` handles the ground ([toned]) | GUIDANCE |
| SP-IX-5 | Lights, then darks; intermediates from thickness | Light and dark shapes; opacity for "thin" | Opacity | None | WF | `mass` (SP-IX-4..6) | `method/mass.md` | GUIDANCE |
| SP-IX-6 | Edges: where masses meet sharply and where lost | A per-edge softness decision | Feather (uniform, raster) | No variable softness | EF | `mass`: lost-and-found recipe (vector bridge, width profile, declared feather) | Deferred: the engine feature; recipe in `mass.md` | DEFERRED |
| SP-IX-7 | Judge from a distance, half-closed eyes, a black glass | Reduced views beside the reference | None | No such views | AO | `study.render` (values, gray, small `maxSide`, `flip: "horizontal"` as the mirror) | [render]. There is no blur, so it is not a true squint; see review | TESTED |
| SP-IX-8 | One deft stroke; much handling kills | One planned path, redrawn rather than nudged | Undo | None | AP | `procedure`: redraw a path rather than micro-edit it | Not in the text | GUIDANCE |
| SP-IX-9 | Each brush shape and touch has its own character | A brush set generated in code | `crates/brush` (calligraphic, not checked) | No touch set | EF | None | Deferred: brush work belongs to the `brush` crate | DEFERRED |
| SP-IX-10 | Correct by re-grounding; don't fear losing the drawing | Delete and redraw a region safely | Undo, `artboard.duplicate` | No "re-ground" command | WF | `procedure` step 6 (undo to the last right state) | `method/procedure.md` | GUIDANCE |
| SP-IX-11 | Each mass has a place on a dark–light scale | Value readouts on a perceptual scale | None | No L* readout | AO | `study.values` `meanL` per group (CIE L*) | [Lstar], [values] | TESTED |
| SP-IX-12 | Modelling's character is in its planes; no gross roundness | Facets first, fused later; no radial "ball" | Gradients | No lint | AP | `mass` step 3 | `method/mass.md` cites it | GUIDANCE |
| SP-IX-13 | Brush direction uses the line-direction vocabulary | Direction of marks chosen for meaning | `study.hatch {angle}` | None | AP | `line`: one angle by default, changed where a form calls for it | `method/line.md` | GUIDANCE |
| SP-IX-14 | Toned paper: the half tone parts lights from darks | A toned-ground setup | Rectangles | No contact check | WF | `materials` table (shapes that never touch) | `method/materials.md` | GUIDANCE |
| SP-IX-15 | In colour, separate light and shadow middle tones, warm vs cool | A light family and a shadow family | Swatches | None | AP | `mass`: two colour families, with the "not a law" caveat | Not in the text | GUIDANCE |

## X Rhythm

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-X-1 (alias B SP-X-1) | An abstract music of line, tone and colour | Treat the abstract layout as a design layer | None | None | DP | `rhythm` | `method/rhythm.md` cites SP-X | GUIDANCE |
| SP-X-2 | Relate appearance to the structure; don't let naturalism crowd it | Structure, then detail, then re-check | None | None | WF | `procedure` step 7 | `method/procedure.md` | GUIDANCE |
| SP-X-3 (alias B SP-X-2) | Every picture has a structure of lines and masses | A line view and a mass view | None | None | AO | `study.lines` (lines), `study.values` / `study.render {mode: values}` (masses) | [lines], [values] | TESTED |
| SP-X-4 (alias B SP-X-3) | Unity and variety are the two axes | Tag suggestions as unity or variety | None | None | DP | `rhythm` | `method/rhythm.md` | GUIDANCE |
| SP-X-5 | Unity without variety is lifeless; regularity reads mechanical | Notes on exact circles, equal spacing, identical copies | None | No regularity notes | AO | `rhythm` (constant curvature); `study.compare` for repeats | Deferred: needs "regular is not bad" wording, and a logo may want it | DEFERRED |
| SP-X-6 | Variety must stay governed by unity | Fix the large structure, then vary inside it | None | None | DP | `rhythm` | `method/rhythm.md` | GUIDANCE |
| SP-X-7 (alias B SP-X-4) | Beauty sits near the type; charm in departures | Never a beauty or ugliness score for faces | None | None | NA | `portrait`: never score attractiveness; `limits` (pp. 133–134) | `method/portrait.md` | NOT-AUTOMATED |
| SP-X-8 (alias B SP-X-5) | Rules don't replace the impulse | Analysis as observation, never pass/fail | None | None | DP | The `method` strings of `study.values`, `study.balance`, `study.lines`, `study.proportions` and `study.compare` say what they don't measure; `limits` | `method/limits.md` (p. 130); `raster.rs`, `lines.rs`, `construct.rs` | PRINCIPLE |

## XI Rhythm: Variety of Line

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XI-1 | Line rhythm: line shape, mutual relations, relation to the frame | Readouts for shape, relations, frame | None | No relations (parallel families, radiation) | AO | `study.lines` (curvature, straightness, directions) | [lines] (shape and direction only) | TESTED |
| SP-XI-2 | Straight line and circle have the least variety | Curvature variation per line | None | None | AO | `study.lines` `curvatureVariation` (0 for one radius) | [lines], [polyline] | TESTED |
| SP-XI-3 | Verticals and horizontals tie the design to the frame | Share of direction near 0° and 90° | None | Bins are ±11.25°, not ±3° | AO | `study.lines` `directions` | [lines] | TESTED |
| SP-XI-4 | Avoid perfect circles in natural objects | A note on perfect circles in organic subjects | `shape.ellipse` | No ovoid primitive | AP | `rhythm` (constant curvature has the least character) | `method/rhythm.md` (via SP-XI-2) | GUIDANCE |
| SP-XI-5 | Curvature ladder: circle, ellipse, egg | An egg-like curve | None | No ovoid primitive | AP | `rhythm` | `method/rhythm.md` cites it | GUIDANCE |
| SP-XI-6 | In a profile, variety serves the run | Show departures from a smoothed run | None | None | AO | None | Deferred: the smoothing scale is arbitrary, and a tool must not "correct" faces | DEFERRED |
| SP-XI-7 | Draw the defining edge from its quieter side | Silhouette as one path (mask or cut-out) | `object.clippingMask.make` | None | WF | `line`: draw the silhouette as one long path used as a mask | Not in the text | GUIDANCE |
| SP-XI-8 | Variety in symmetry: opposed hollows and fullnesses | Don't mirror organic limbs | `object.repeat.mirror` | No opposition plot | AP | `line`: avoid mirroring organic forms, or break the mirror | Not in the text (`portrait` covers faces only) | GUIDANCE |
| SP-XI-9 | Vary line thickness and accent | Width and darkness follow the contour's prominence | Width profiles | `study.lines` ignores stroke width | IE | `line`: accents with width profiles | `method/line.md` | GUIDANCE |

## XII Rhythm: Unity of Line

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XII-1 | Unity of line: flows through the panel, real or imagined | Draw the intended flow lines first | `guide.add` | Curved guides not checked | WF | `rhythm`: draw the main swing as a construction path first | `method/rhythm.md` | GUIDANCE |
| SP-XII-2 | Carry contours across gaps | Find possible continuations | None | No continuation finder | AO | `rhythm` | `method/rhythm.md` cites it (the finder is deferred) | GUIDANCE |
| SP-XII-3 | Parallelism sustains, for simple lines only | See parallel families | `object.path.offsetPath`, `object.blend.make` | No family detection | AO | `study.lines` `directions` shares; `rhythm` | [lines] (histogram only) | TESTED |
| SP-XII-4 | Fuller parallelism, more emphasis; carry it into secondary forms | Hatch at a chosen family's angle | `study.hatch {angle}` | No "angle from a path" | EF | `study.hatch {angle}` set from a measured `meanOrientation` | No test of this use | IMPLEMENTED |
| SP-XII-5 | Balance unequal groups by line and tone weight | A rough weight proxy for comparing regions | None | Contour weight not counted | AO | `study.balance {region, weigh}` (darkness or contrast; no contour weight) | [balance], [toned] | TESTED |
| SP-XII-6 | Maximum-unity lines carry strength but need dither | A strength/softness vocabulary | None | None | AP | `rhythm`: straighter, squarer contours for strength, with the 1913 caveat | Not in the text | GUIDANCE |
| SP-XII-7 | Horizontal = calm, vertical = elevation (opinion) | Horizontal and vertical shares | None | None | AO | `study.lines` `directions`; the meanings stay opinion (`rhythm`, `limits`) | [lines] | TESTED |
| SP-XII-8 | Diagram X: horizontal calm disturbed step by step | A calibration set | None | No fixtures | PE | None | Deferred: fixtures must be authored from scratch (ledger C has the measurements) | DEFERRED |
| SP-XII-9 | Diagram XI: verticals, then swing, then struggle | A calibration set | None | No fixtures | PE | None | Deferred: as SP-XII-8 | DEFERRED |
| SP-XII-10 | Straight lines steady rich curves | Straight vs curved share | None | None | AO | `study.lines` `straightShare` | [lines], [polyline]. Polyline joints ignore `straightTolerance` (review P2-10) | TESTED |
| SP-XII-11 | Contrast compensates; cover a part and look again | Render with and without an element | `object.hide`, `layer.setProps {visible}` | No "peek without" that skips history | WF | `balance`: hide, render, show, compare | Not in the text | GUIDANCE |
| SP-XII-12 | Implied verticality counts as a vertical | Each object's principal axis | None | Not folded into the histogram | AO | `study.blockIn {axis: true}` returns each object's long axis in `axes` | [blockin] (a 2:1 ellipse's axis is level) | TESTED |
| SP-XII-13 | The right angle is the strongest chord | Find right-angle junctions | None | `study.lines` counts corners, not crossings | AO | None | Deferred: needs crossing detection that knows which lines are visible | DEFERRED |
| SP-XII-14 | A structural angle can come from tone and edge emphasis | Raise contrast along an intended line | None | None | WF | `mass`: guide line, then edge emphasis, then compare | Not in the text | GUIDANCE |
| SP-XII-15 | Trace a composition's hidden lines over it | A translucent structure layer | Template layers | The `Construction` layer is locked | EF | The `Construction` template layer from `study.blockIn`; `rhythm` gives the unlock-draw-lock steps | No test of drawing on it | IMPLEMENTED |
| SP-XII-16 | The panel shape is the first given; corners pull the eye | Fix the artboard first | `file.new`, `artboard.new` | No corner check | AP | `procedure` step 1 (choose the artboard) | `method/procedure.md` | GUIDANCE |
| SP-XII-17 | Diagonals relate to the frame; the general swing counts | Lines parallel to the artboard's diagonals | None | 45° bins, not the aspect's diagonals | AO | `study.lines` `directions` (45° and 135° bins) | No test of the diagonal bins | IMPLEMENTED |
| SP-XII-18 | Curves need restraint; look for straightness in curves | Straight runs within curves | None | None | AO | `study.lines` `straightShare`, `curvatureVariation` | [lines] | TESTED |
| SP-XII-19 | Diagram XIV: more curvature, more energy | A calibration set; turning per length | None | No turning-per-length figure | PE | None | Deferred: fixtures, as SP-XII-8 | DEFERRED |
| SP-XII-20 | Continuous flow leads to the main interest; stop the exits | An exit check | None | No exit check | AO | `rhythm` (tying lines) | Deferred: flow-line extraction is ambiguous | DEFERRED |
| SP-XII-21 | Flame-like flow: exalting but restless | A style note with its caution | None | None | AP | `rhythm`, with the opinion caveat | Not in the text | GUIDANCE |
| SP-XII-22 | Jagged clashing lines read as terror; check by flipping | Judge the design upside down | `view.rotate` (UI only) | No headless flip | WF | `study.render {flip}`; "terror" is never a label | [render] | TESTED |
| SP-XII-23 | Radiating lines give unity | Find shared radiation centres | `object.repeat.radial` | No detector | AO | `rhythm` | `method/rhythm.md` cites it | GUIDANCE |
| SP-XII-24 | Sympathetic radiation: add lines in sympathy | The harmonising game | None | No game | PE | `rhythm` | `method/rhythm.md` cites it (the game is not written) | GUIDANCE |
| SP-XII-25 | Near-right-angle crossings are harsh; cover them with a mass | Hide unwanted crossings | None | None | AP | `rhythm` | Not in the text | GUIDANCE |
| SP-XII-26 | Chains of sympathy and transitions | Relate an isolated line through bridges | None | None | AP | `rhythm` | Not in the text | GUIDANCE |
| SP-XII-27 | A line at dead centre, and a lone vertical: break them up | A note on a dominant line on the centre line | None | `study.proportions` ignores the artboard | AO | `proportion`: check for a dominant line on the centre | Not in the text | GUIDANCE |
| SP-XII-28 | Long lines pick out the forms they echo | An echo finder | None | None | AO | None | Deferred: never an attractiveness judgement; neutral wording first | DEFERRED |
| SP-XII-29 | Arrangements carry their effect whatever makes them | Name the structural vocabulary apart from the subject | None | None | DP | `limits` (fixed emotional meanings are 1913 opinion) | `method/limits.md` | GUIDANCE |
| SP-XII-30 | Scale sets detail: big works need a severe skeleton | Ask the output size; preview at viewing size | `render_preview {max_side}` | None | WF | `procedure`: ask the output use; preview at that size | Not in the text | GUIDANCE |

## XIII Rhythm: Variety of Mass

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XIII-1 | Masses vary in shape, value, edge, texture, gradation | Five readouts per region | None | Only value and patch counts exist | AO | `study.values` (value channel) | [values] (value only) | TESTED |
| SP-XIII-2 | Invented shapes repeat as pet forms | Find near-duplicate shapes | Symbols | No duplicate finder | AO | `rhythm`: `study.compare` `iou` between invented shapes | [compare] (pairwise only), [groups] | TESTED |
| SP-XIII-3 | Variety from nature, unity from imagination; the frame isn't in nature | Recompose a reference to the artboard | `imageTrace.*` | None | WF | `procedure`: recompose traced references to the frame | Not in the text | GUIDANCE |
| SP-XIII-4 | One black-to-white scale; colour counts by its value | A non-destructive value view | `edit.colors.toGrayscale` (destructive) | None | EF | `study.render {mode: gray}`; CIE L* | [Lstar], [render] | TESTED |
| SP-XIII-5 | Two senses of "tone"; "out of value" | Value outliers within a user group | None | No outlier readout | AO | None | Deferred: needs user-marked groups; `study.values {region}` is a stand-in | DEFERRED |
| SP-XIII-6 | Tone music is strongest with large simple masses | Mass count; area share of the largest | None | None | AO | `study.values` `masses`, `largestShare` | [values] | TESTED |
| SP-XIII-7 | Middle values gentle, wide contrasts dramatic (opinion) | Value range and jumps | None | No percentile span | AO | `study.values` `contrast` (lightest minus darkest group mean) | [values] | TESTED |
| SP-XIII-8 | Texture: one quality for tone music, broken for brilliance | Keep one surface treatment | `effect.list` | None | AP | `materials` (texture by hatching, bounded) | `method/materials.md` | GUIDANCE |
| SP-XIII-9 | Vary edges between soft and sharp | Edge softness that varies | Feather (uniform) | No variable softness | EF | `mass`: lost-and-found recipe | Deferred: the engine feature; recipe in `mass.md` | DEFERRED |
| SP-XIII-10 | Sharpest accents on non-essentials (Venetian practice) | Decide where the sharpest edges go | None | None | AP | `mass` step 4: one option, not a rule | `method/mass.md` cites it | GUIDANCE |
| SP-XIII-11 | One focus for the whole picture | One focal impression | None | No focal-softness tool | AP | `mass` | `method/mass.md` cites it | GUIDANCE |
| SP-XIII-12 | Serrated edges need a large governing order | An envelope over a cluster of small shapes | `symbol.spray` | No envelope | AO | `study.blockIn {hull: true}` on a group gives its convex envelope | [blockin] (the `hull` case) | TESTED |
| SP-XIII-13 | Learn hard before loose | Hard flat shapes first, softness later | None | None | WF | `mass` step 3 | `method/mass.md` | GUIDANCE |
| SP-XIII-14 | Economy of gradation: one octave of value | A value budget per object | None | No per-object figure | AO | `study.values {region}` over one object's bounds | No test of `region` in `study.values` | IMPLEMENTED |

## XIV Rhythm: Unity of Mass

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XIV-1 | Unity of mass from an ordered tone scheme, not copying | Reference → value study → designed scheme | `imageTrace.*` | None | WF | `mass` (cites the chapter) | `method/mass.md` ("SP-XIV") | GUIDANCE |
| SP-XIV-2 | Light masses pull the eye (opinion) | List the lightest masses | None | No ranking by place | AO | `study.values` light group share and masses | No test of this use | IMPLEMENTED |
| SP-XIV-3 | Keep naturalism consistent between figures and background | Match treatment across the picture | None | None | AP | `materials` | Not in the text | GUIDANCE |
| SP-XIV-4 | The same filter on reference and work | Paired filtered views | None | No paired view | EF | `study.render {region, showTemplates}` on the reference and on the drawing with the same `levels` and `thresholds` | Recipe in `method/mass.md` (same filter on both regions); no test of the paired use | IMPLEMENTED |
| SP-XIV-5 | Avoid pure white and pure black masses | A compressed default range | None | No extremes readout | AP | `mass` step 1 example (L* 85, 55, 20); `study.values` `extremes.pureShare` | [toned] (extremes); `method/mass.md` | TESTED |
| SP-XIV-6 | Three ways to fit nature's range into paint | Lightness remap presets | `recolor.*` (L* curves not checked) | None | EF | None | Deferred: needs a lightness-curve recolour; not this change | DEFERRED |
| SP-XIV-7 | Strong light gives a two-mass formula | A two-value view | None | None | AO | `render_preview {mode: values, levels: 2}`, `study.values {levels: 2}` | [mcp-region] (two greys) | TESTED |
| SP-XIV-8 | Plan on flat tones first | A block-in in flat values | None | No flatten-gradients preview | WF | `mass` steps 1–3 | `method/mass.md` | GUIDANCE |
| SP-XIV-9 | Strong contrast needs simple masses | Contrast and mass count together | None | No combined note | AO | `study.values` returns both `contrast` and `masses` | [values] | TESTED |
| SP-XIV-10 | Strong colour contrast and strong tone contrast don't mix | A check on neighbouring regions | None | No adjacency or chroma | AO | None | Deferred: needs region adjacency and chroma; colour is outside the book | DEFERRED |
| SP-XIV-11 | Flat tones ~ straight lines; gradated ~ curves | Match fill treatment to line treatment | None | None | AP | `rhythm` | Not in the text | GUIDANCE |
| SP-XIV-12 | Gradated schemes need character, or they turn pretty | Keep crisp drawing in key forms | None | Character | AP | `mass` | Not in the text | GUIDANCE |
| SP-XIV-13 | Watteau's tone path: accent, dark frame, spots | An abstract tone-path study | None | None | PE | None | Deferred: an exercise to author | DEFERRED |
| SP-XIV-14 | Gradated compositions need steadying straights | Gradient share next to straight share | None | No gradient share | AP | `rhythm` with `study.lines` `straightShare` | Not in the text | GUIDANCE |
| SP-XIV-15 | Greatest dark against greatest light at one place | Find the maximum-contrast place | None | No locator | AP | `mass` step 4, as one option (Turner's and Corot's device) | `method/mass.md` cites it | GUIDANCE |
| SP-XIV-16 | Knowing the principles doesn't make a picture | Never present compliance as quality | `artboard.new` (idea sketches) | None | DP | `limits` (p. 216) | `method/limits.md` | GUIDANCE |
| SP-XIV-17 | Conceal the artifice; mechanical jitter is not dither | No fake dither | Roughen, scribble | None | DP | `rhythm`, `limits`; `study.hatch` `jitter` defaults to 0 and is seeded | [jitter]; `method/rhythm.md` | GUIDANCE |

## XV Rhythm: Balance

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XV-1 | Balance approximate: never perfect, never far out | Readouts as a spectrum, never pass/fail | None | None | AO | `study.balance`; `balance`: "do not drive the offset to zero" | [balance], [toned]; `method/balance.md` | TESTED |
| SP-XV-2 | Straight against curved, biased to the straight | Straight vs curved share | None | None | AO | `study.lines` `straightShare` | [lines] | TESTED |
| SP-XV-3 | Flat against gradated tones, biased to the flat | Share of gradient-filled area | Fill types in the document model | No share | AO | `balance` cites it | Deferred: cheap from fills, but transparency makes it approximate | DEFERRED |
| SP-XV-4 | Rescue a muddled picture by reducing it to flat tones | A flat-tones view | None | No flatten-gradients command | AO | `study.render {mode: values}` | [render] | TESTED |
| SP-XV-5 | Balance the extremes of the tone scale, not the amounts | Lightest and darkest values against mid-grey | None | `study.balance` measures amounts | AO | `study.values` `extremes` (`darkL`, `lightL`, `midOffset`, `pureShare`) | [toned] | TESTED |
| SP-XV-6 | Warm against cold | Warm/cool totals | None | No colour readout | AO | `balance` cites it | Deferred: the warm/cool boundary is a convention the product must choose | DEFERRED |
| SP-XV-7 | Interest can balance mass | Meaning counts in balance | None | Geometry can't read interest | NA | `study.balance` `method` says subject isn't weighed; `balance` | `method/balance.md` | NOT-AUTOMATED |
| SP-XV-8 | Balance variety against unity | Structure first, then vary inside it | None | None | DP | `balance` | `method/balance.md` | GUIDANCE |

## XVI Rhythm: Proportion

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XVI-1 | Canons are scaffolding to depart from | No snapping to canons | Guides | None | DP | `proportion`; no canon overlays shipped | `method/proportion.md` | GUIDANCE |
| SP-XVI-2 | Simple proportion gives repose; varied gives life (his feeling) | See the main divisions | None | No ratio flags | AO | `proportion` | `method/proportion.md` | GUIDANCE |
| SP-XVI-3 | Avoid equal divisions except for stillness | Find accidental equal spacing | Distribute, repeat, blend (equal by default) | No equality detector | AO | `proportion`: read the gaps from `study.proportions` `objects` | `method/proportion.md` | GUIDANCE |
| SP-XVI-4 | Diagram XXVIII: equal vs varied placement | Lay out variants and compare them one at a time | `artboard.new` | None | PE | `composition-variants` prompt | `prompts.rs` | EXERCISE |
| SP-XVI-5 | Centralised composition: repose from symmetry, equality hidden | Symmetric layout, then varied parts | `object.repeat.mirror` | No mirror-difference readout | WF | `balance`; `composition-variants` `vary` option (symmetry against a diagonal) | The prompt has the option; the topic text doesn't say it | GUIDANCE |
| SP-XVI-6 | The formality of the design should suit its naturalism | Match stylisation to formality | None | None | AP | `proportion` | Not in the text | GUIDANCE |
| SP-XVI-7 | Dynamic composition: varied proportions, lines converging | Do leading lines converge on the focus? | None | No line extension | AO | None | Deferred: must never claim to predict an eye path | DEFERRED |
| SP-XVI-8 | 5:8, a proportion with no obvious relation | φ only as a hedged option | None | None | DP | `proportion` (heavily hedged; no ratio is beautiful in itself) | `method/proportion.md` | GUIDANCE |
| SP-XVI-9 | A few dominant quantities control the variety | Block in the largest shapes first | None | No area-share readout | WF | `proportion`, `procedure` | `method/proportion.md`, `method/procedure.md` | GUIDANCE |

## XVII Portrait Drawing

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XVII-1 | Likeness lies in what differs from the common type | No face templates, no averaging | None | None | DP | `portrait` | `method/portrait.md` cites it | GUIDANCE |
| SP-XVII-2 | Smoothing away characteristics loses vitality | No automatic beautify or smooth on faces | `object.path.smooth` (explicit) | None | DP | `portrait` | `method/portrait.md` | GUIDANCE |
| SP-XVII-3 | Likeness is a chord of the whole | Re-check the whole after each feature | `render_preview` | None | WF | `portrait` | `method/portrait.md` | GUIDANCE |
| SP-XVII-4 | Look for the differences first, while the eye is fresh | List the subject's distinctive proportions first | None | None | AP | `portrait` | `method/portrait.md` | GUIDANCE |
| SP-XVII-5 | Block in the disposition early, no hard rules | User-placed landmarks measured against the head | Bounds | No reference-vs-drawing pairing | EF | `study.proportions` on user-placed marks; `study.blockIn` (square lines) | [prop] | TESTED |
| SP-XVII-6 | Order of work: masses first in painting, far to near | Masses, a check at thumbnail size, then features | None | None | WF | `portrait` (Procedure XVII) | `method/portrait.md` | GUIDANCE |
| SP-XVII-7 | Insight: keep the essential, subordinate the petty | Ask the user what to stress | None | Insight | NA | `portrait`: ask the user what to emphasise | Not in the text yet | NOT-AUTOMATED |
| SP-XVII-8 | Inner character shows in form (period physiognomy) | Never infer character from a face | None | None | NA | Prohibited; `portrait`: physiognomy is period opinion | `method/portrait.md` | NOT-AUTOMATED |
| SP-XVII-9 | Sincerity: take a master's qualities only once seen in nature | Ground any borrowed style in the subject | None | None | AP | `portrait` | Not in the text | GUIDANCE |
| SP-XVII-10 | Vitality balanced by repose (Hals) | Restraint in one channel when another is energetic | `recolor.reduce` | None | AP | `portrait`: a limited palette when the gesture is energetic | Not in the text | GUIDANCE |
| SP-XVII-11 | Honest to the vital impression, not a mechanical record | The reference is evidence, not the target | None | The impression is the artist's | NA | Left to the person; `portrait`: `study.compare` on the block-in only, never iterate toward `iou` | `method/portrait.md` | NOT-AUTOMATED |
| SP-XVII-12 | Don't force symmetry on paired features | No mirrored features | `object.repeat.mirror` | No mirror-difference readout | AP | `portrait` | `method/portrait.md` cites it | GUIDANCE |
| SP-XVII-13 | Points of view (symbol, symphony, official): one dominates | State the dominant aim | None | None | AP | `portrait` | Not in the text | GUIDANCE |
| SP-XVII-14 | Expression: enduring, not momentary | Prefer a resting expression | None | None | AP | `portrait` | Not in the text | GUIDANCE |
| SP-XVII-15 | Clothes: timeless laws of drapery, not fashion | Fold structure before ornament; variants | `artboard.new` | None | AP | `portrait` | Not in the text | GUIDANCE |

## XVIII The Visual Memory

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XVIII-1 | Memory is the storehouse; pictures colour what we see | Know that references bias output; study, don't copy | `file.place {template}` | None | DP | `memory` | Not in the text | GUIDANCE |
| SP-XVIII-2 | Memory keeps the essential (strong science caveat) | An early summary governs long work | None | None | WF | `memory` cites it, with the caveat that memory reconstructs and normalises | `method/memory.md` | GUIDANCE |
| SP-XVIII-3 | Snapshot the first flash; let it guide the work | A pinned intent note and first sketch | `artboard.duplicate` (manual) | No document intent field | EF | `procedure` step 1 (intent sentence) | Deferred: a document metadata field and a panel; a separate change | DEFERRED |
| SP-XVIII-4 | Memorise in the terms of the medium | Plan in vector terms before drawing | None | None | AP | `memory` | `method/memory.md` cites it | GUIDANCE |
| SP-XVIII-5 | The faculty can be trained: graded daily memory drawing | A practice workflow that never scores the person | None | None | PE | `memory-exercise` prompt; `memory` | `prompts.rs`; `method/memory.md` | EXERCISE |
| SP-XVIII-6 | Enjoy it, analyse it flat, draw it in your mind | The exercise steps | None | None | PE | `memory-exercise` prompt | `prompts.rs`; `method/memory.md` cites it | EXERCISE |
| SP-XVIII-7 | Work to a scheme; make a quick first sketch | Keep the first sketch as the governing reference | `artboard.new` | None | WF | `procedure` step 2 (small variants first) | `method/procedure.md` | GUIDANCE |
| SP-XVIII-8 | Decide the scheme at first sight; don't switch for novelty | Changes of scheme are deliberate branches | Artboards | None | AP | `memory` | `method/memory.md` cites it | GUIDANCE |
| SP-XVIII-9 | Matter and manner memorised, then fused | A plan states the effect and the means | None | The fusion is the artist's | NA | `memory`: plan = effect + means; the fusion is not a function | Not in the text yet | NOT-AUTOMATED |

## XIX Procedure

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XIX-1 | Know what you want before touching the paper | A plan before any drawing call | None | None | AP | `procedure` step 1 | `method/procedure.md` cites it | GUIDANCE |
| SP-XIX-2 | Be moved first; dismiss camera truth | Name what the drawing is about | None | None | AP | `procedure` | `method/procedure.md` cites it | GUIDANCE |
| SP-XIX-3 | The first strokes decide; restart rather than patch | Big lines first; a fresh attempt beside the old | `artboard.new` | None | WF | `procedure` | `method/procedure.md` cites it | GUIDANCE |
| SP-XIX-4 | Correct early: go back to the last right stage | Return to a known good state | Undo; atomic `command_batch` rollback | No named checkpoints | WF | `procedure` step 6 | `method/procedure.md`; [mcp-batch] covers a failed batch only | GUIDANCE |
| SP-XIX-5 | Rest the eye; never measure fatigue | User-started fresh-look views | None | None | NA | `study.render {flip}` on request; nothing detects fatigue | [render] | NOT-AUTOMATED |
| SP-XIX-6 | Detail stays subordinate to the first impulse | Compare late stages with early ones, small | `artboard.duplicate` | No low-resolution comparison | WF | `procedure` step 7 | `method/procedure.md` cites it | GUIDANCE |
| SP-XIX-7 | Don't rub out in expressive drawing | Keep superseded strokes faintly | Layers, opacity | No pentimenti option | EF | None | Deferred: a layer option; undo already makes erasure invisible | DEFERRED |
| SP-XIX-8 | Accuracy first; facility follows | Accuracy and quick-sketch exercises | `study.compare` | No exercise prompt | PE | None | Deferred: `study.compare` is ready, the exercise is not written | DEFERRED |
| SP-XIX-9 | Every touch must mean something | No filler hatching | `study.hatch {maxLines}` | None | AP | `procedure`; `line` (keep `maxLines` modest) | `method/procedure.md` cites it | GUIDANCE |
| SP-XIX-10 | Prime the eye with a master's drawing | Curated study references | None | No licensed set | AP | None | Deferred: licence checks, `ASSETS.md` rows, and the owner's publishing decision | DEFERRED |
| SP-XIX-11 | Nervous vitality and health | Nothing: outside the product | None | None | NA | Build nothing (no wellness nudges) | Ledger D | NOT-AUTOMATED |

## XX Materials

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XX-1 | The medium decides what you select | Design to vector's strengths | None | None | DP | `materials` | `method/materials.md` cites it | GUIDANCE |
| SP-XX-2 | Not illusion: painted symbols | No photorealism by default | None | None | DP | `materials` | `method/materials.md` cites it | GUIDANCE |
| SP-XX-3 | A point should only do what a point can | A drawing works by line and hatching | None | No line-only lint | DP | `materials` (a line study uses strokes only) | `method/materials.md` | GUIDANCE |
| SP-XX-4 | Limitations give power; add means gradually | A constrained palette | `recolor.reduce` | No palette constraint | AP | `materials` | `method/materials.md` cites it | GUIDANCE |
| SP-XX-5 | Lead pencil: delicate precise line | Pencil analogue | Width profiles | None | IE | `materials` table | `method/materials.md` | GUIDANCE |
| SP-XX-6 | Silver and gold point: purest line, no erasing | Fine-line analogue | Width profiles | No no-undo practice mode | IE | `materials` table | `method/materials.md` | GUIDANCE |
| SP-XX-7 | Charcoal rub-down and lift-out | A mid-tone layer, lights over it | Shapes, opacity | None | WF | `materials` table (charcoal rub-down) | `method/materials.md` | GUIDANCE |
| SP-XX-8 | Red chalk: warm single hue; pretty is not correct | A one-hue ramp; check proportions first | Swatches | None | AP | `materials` | Not in the table | GUIDANCE |
| SP-XX-9 | Holbein: rub down, then redraw with finality | Dim and lock the exploration, redraw on top | Layers, opacity, lock | None | WF | `materials` / `procedure` | Not in the text | GUIDANCE |
| SP-XX-10 | Black Conté: a supplement that shares the voice | Precision passes share the main brush's look | Graphic styles | None | DP | `materials` | Not in the text | DEFERRED |
| SP-XX-11 | White chalk on toned paper: the half tone between | A toned-ground setup | Shapes | No light/dark contact check | WF | `materials` table (shapes that never touch) | `method/materials.md` | GUIDANCE |
| SP-XX-12 | Lithography: the print is the original | Design for the output; verify it | Export | None | DP | `procedure` step 9 (save, export, reopen) | `method/procedure.md` | GUIDANCE |
| SP-XX-13 | Pen and ink: clear darks, character per pen | Width profiles; hatching | Width profiles | No exact hatching | IE | `materials` table; `study.hatch` | [hatch], [holes], [eight] | TESTED |
| SP-XX-14 | Etching: weight tiers, plate tone | Stroke-weight tiers; hatching | Graphic styles | None | IE | `materials` table | `method/materials.md` | GUIDANCE |
| SP-XX-15 | Paper and surface; redraw over thin paper | Onion-skin redraw | Layers, opacity, lock | No one-step command | IE | `materials`: duplicate, dim, lock, draw on top | Not in the text | GUIDANCE |

## XXI Conclusion

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-XXI-1 | No mechanical standard; avoid the camera as a student | Say when output was traced | `imageTrace.*` | None | AP | `limits` | Not in the text | GUIDANCE |
| SP-XXI-2 | Precision first; a master's variations are seldom conscious | No random jitter to fake vitality | Roughen | None | AP | `limits`: never fake "life" with random jitter | `method/limits.md` | GUIDANCE |
| SP-XXI-3 | A mannerism remote from nature is a trick of the hand | No default house style | None | None | DP | `limits` | Not in the text | GUIDANCE |
| SP-XXI-4 | Sincerity over originality | Never claim to generate originality | None | Sincerity | NA | The product makes no originality claim | Ledger D | NOT-AUTOMATED |
| SP-XXI-5 | Style is a clear idea reached by the shortest means | Intent, then the most economical means | None | None | AP | `procedure` step 1 (intent) and its economy rule (SP-XIX-9) | `method/procedure.md` | GUIDANCE |

## Appendix (the Phi proportion)

| ID | Principle | User/agent need | Existing capability | Gap | Disp. | Our solution | Test / evidence | Status |
|---|---|---|---|---|---|---|---|---|
| SP-A-1 | The additive series converges on φ | φ as an optional, non-snapping guide | Guides | None | IE | None | Deferred: no demand; it would ship hidden, with the caveat text | DEFERRED |
| SP-A-2 | Diagram XXIX: constructing φ segments | A φ ruler | Guides | None | PE | None | Deferred: as SP-A-1 | DEFERRED |
| SP-A-3 | φ found post hoc in seven pictures | Never a score | None | None | NA | No φ scoring anywhere | `method/limits.md` (golden section is not a law) | NOT-AUTOMATED |
| SP-A-4 | "The eye takes some especial pleasure in it" (unproven) | Never present φ as pleasing | None | None | DP | `proportion`, `limits` | `method/proportion.md`: "No ratio is beautiful in itself" | GUIDANCE |

---

## Counts

Principles per chapter, from the ledgers' headings, with the table rows above:

| Chapter | Ledger | Principles | Rows here |
|---|---|---|---|
| Preface | A | 6 | 6 |
| I | A | 7 | 7 |
| II | A | 7 | 7 |
| III | A | 9 | 9 |
| IV | A | 10 | 10 |
| V | A | 12 | 12 |
| VI | B | 8 | 8 |
| VII | B | 5 | 5 |
| VIII | B | 20 | 20 |
| IX | B | 15 | 15 |
| X | C (primary); B's 5 as aliases | 8 | 8 |
| XI | C | 9 | 9 |
| XII | C | 30 | 30 |
| XIII | C | 14 | 14 |
| XIV | C | 17 | 17 |
| XV | D | 8 | 8 |
| XVI | D | 9 | 9 |
| XVII | D | 15 | 15 |
| XVIII | D | 9 | 9 |
| XIX | D | 11 | 11 |
| XX | D | 15 | 15 |
| XXI | D | 5 | 5 |
| Appendix | D | 4 | 4 |
| **Total** | | **253** | **253** |

The count matches the ledgers: ledger A 51, ledger B 48 (plus its 5 chapter-X ids, listed as aliases), ledger C 78, ledger D 76. Ledger D's own stated total is 76.

Status totals:

| Status | Rows |
|---|---|
| TESTED | 53 |
| IMPLEMENTED | 6 |
| GUIDANCE | 137 |
| EXERCISE | 3 |
| PRINCIPLE | 7 |
| NOT-AUTOMATED | 11 |
| DEFERRED | 36 |
| **Total** | **253** |

The counts were taken from the table with a script, which also checked that the ids are exactly the ledgers' 253 ids, each once.

Read TESTED with care. A TESTED row means a test exercises the command that serves the principle. It does not mean the principle is fully served: many TESTED rows name a partial gap (for example SP-VIII-5, SP-XI-1, SP-XIII-1). The critic review lists where a tested command still measures the wrong thing for the principle, for example polyline joints in `study.lines` or `study.balance` over a toned ground. Several fixes landed while this table was drafted, and each has a test: smooth polyline runs read as curves, group members keep their fill rules, `study.balance {weigh: "contrast"}`, `study.values` `extremes`, block-in `maxDeviation`, and pictures from `run_command` arriving as images.

The guidance once owed for 48 rows was written into the method topics ("More from the book" in each `crates/mcp/src/method/*.md`); SP-XX-10 stays DEFERRED: a precision pass sharing the main tool's look needs shared stroke styles to be actionable.
