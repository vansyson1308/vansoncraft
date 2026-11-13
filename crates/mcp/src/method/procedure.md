# Procedure: the order of work

**Speed says.** Know what you want before you touch the paper (SP-XIX-1, SP-P-4). Be moved by
something in the subject first (SP-XIX-2). The first strokes decide: seize the big lines and swings
(SP-XIX-3). Use guide lines, measuring and blocking in as scaffolding, then put it away (SP-VIII-7;
in academic studies, SP-XIX-1). Reduce the subject to a few simple masses, then subdivide, accents last
(SP-V-2, SP-XVI-9). As soon as you feel you have gone wrong, go back to the last stage that was right
(SP-XIX-4). Every touch must mean something: no filler (SP-XIX-9). In the final stage re-kindle the
first impression so detail does not bury it (SP-XIX-6).

**With VansonCraft.**
1. *Intent.* Write one sentence: the subject, the one impression it must give, and line study or
   mass study. With a reference, a mass study from it suits; without one, construct with lines
   (SP-V-8). Choose the artboard (`file.new`, or `artboard.new` for variants).
2. *Plan small.* For layouts, make small variants first on separate artboards (`artboard.new`,
   named), each a few large shapes only.
3. *Block in.* Draw the main masses as few large paths, or draw the outline and run
   `study.blockIn {ids, maxSides: 8, axis: true}` to see its straight-line envelope and long axis on
   the locked `Construction` template layer (left out of exports).
4. *Masses.* Put each mass on its own named layer or group (`layer.new {name}`), in three values
   first (see `mass`). Check with `render_preview {mode: "values"}` and `study.values`.
5. *Look.* `render_preview` at page size, then zoom into details with `render_preview {region}`.
   Turn it upside down to see it fresh: `render_preview {flip: "both"}` (SP-III-8); mirror it with
   `flip: "horizontal"` (a mirror or black glass, SP-IX-7).
6. *Checkpoint or revert.* When a step made it worse, `undo` back to the last right state rather
   than piling corrections on top (SP-XIX-4). A failed atomic `command_batch` already rolls back.
7. *Detail under the plan.* Add detail inside the masses, never across them; check the values map
   again — detail must not break the big groups (SP-XIX-6).
8. *Structure check.* `doc_inspect {depth: 2}`: named layers and groups for each part the user may
   want to edit; no raster images standing in for vector art.
9. *Save and export.* `save_file` (native) first, then `export` SVG/PNG; reopen to verify.

**More from the book.**

- Every effect must serve the stated impression. Drop any that only shows technique (SP-I-5).
- When it matters, name the felt quality, such as bulk or lightness (SP-II-4). Suggested means (ours): for bulk, large unbroken masses and firm contours; for lightness, thin lines and little shadow.
- Stop when the stated impression is realised. Detail is finish only when it is the intent (SP-III-7).
- Place each shape as one planned path. If it is wrong, redraw it rather than nudging anchors (SP-IX-8).
- Ask the output size first: "the smaller the picture the more detail it should contain, and the larger the less" (SP-XII-30). Check at viewing size with `render_preview {max_side}`.
- A traced reference gives variety of form, not a composition. Recompose its parts to the artboard (SP-XIII-3).

**Limits.** The order is Speed's for academic studies; he says many artists find their own way
(p. 100, p. 112). Follow the user's brief over this list.
