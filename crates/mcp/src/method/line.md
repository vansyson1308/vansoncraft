# Line

**Speed says.** Outlines do not exist in nature; they are the boundaries of masses, lost in places
and found in others (SP-IV-1). Line appeals to the imagination and has a music of its own
(SP-IV-2). Measure with plumb and level lines against a unit, only vertical and horizontal distances
(SP-VIII-2, SP-VIII-3). Block in with straight lines like a surveyor, watching the background shapes
too (SP-VIII-5, SP-VIII-8). Fix each curve by its ends, its inflection points, its chord, and how far
and where it bows from the chord (SP-VIII-6). Then draw the form, both sides together, with
contours as overlappings, accented and lost (SP-VIII-9..11). Shade only to explain form: parallel
lines in an easy direction; curve them for fullness, run them down the form for hardness, cross them
for the dark next to the light; never scribble (SP-VIII-12..16).

**With VansonCraft.**
- Curves: draw with few anchors at the extremes and inflections, handles along the tangent
  (`draw_path` points with absolute `in`/`out`). `study.lines {ids}` reports inflections, corners,
  straight/curved share and curvature variation; a curve with many inflections it did not need
  usually has too many anchors.
- Accent and lost edges: vary stroke weight along a path with width profiles
  (`stroke.widthProfile.set`, `stroke.widthPoint.set`), thicker where the contour turns under
  and at the accent, thinner or broken where the form turns to the light.
- Shading in line: `study.hatch {ids, spacing, angle, cross}` lays exact, editable hatching inside a
  closed shape. Use one angle across the drawing by default — parallel lines preserve a unity
  (SP-VIII-15); change it only where a form calls for it: down a hard form, crossed for the dark
  next to the light (SP-VIII-16). Hatch each value shape separately, closer and heavier in the
  shadow. `study.hatch` draws straight lines only: curved hatching round a form is not available.
  Keep `maxLines` modest: hatching explains form, it is not texture filler.
- Scaffolding: `study.blockIn` and plumb/level checks with `study.proportions` (alignments).

**More from the book.**

- Redraw a reused part's silhouette for each angle (SP-IV-5).
- Build a figure from stacked, overlapping forms (the front contour runs on, the one behind stops), or silhouette first with the interior masses cut in. Pick one per figure (SP-V-11).
- Hair: a few long strokes that swell and thin (`stroke.widthProfile.set {points: [[0, 0.2, 0.2], [0.4, 1, 1], [1, 0.1, 0.1]]}`), with the shadows hatched by `study.hatch` (SP-VIII-17).
- For a profile that must run in one sweep, let the detail overrun the edge and clip it with one long path (`object.clippingMask.make`). This is our vector form of SP-XI-7; the study commands refuse clip groups, so pass them the members.
- Don't leave organic paired forms mirrored (`object.repeat.mirror`, `object.reflect`). A convex curve on one side usually faces a concave one on the other (SP-XI-8). After a mirror, edit one side.

**Limits.** The emotional meanings Speed gives to line directions are his artistic opinion, not
measured fact. Vector strokes are hard-edged; a "lost" edge needs a thinner stroke, a gap, or a
declared feather or blur effect (see `mass`), not a pretence of softness. `study.lines` reads a
curve drawn with plain points as a curve, from the turns at its joints.
