# Limits: what the studies measure, and the author's own warnings

**What the commands measure.**
- `study.values`, `study.render {mode}`: CIE L* lightness of the picture as exported on white,
  banded by thresholds; masses are connected patches of one band. Tone only — not subject, intent or
  importance.
- `study.balance`: the centre of darkness, or of contrast with the median tone (`weigh:
  "contrast"`, for toned grounds). Not attention, not "visual weight" of colour or subject.
- `study.lines`: geometry of paths as drawn (not stroke outlines): directions, straightness,
  corners, inflections, curvature variation. Not rhythm itself.
- `study.proportions`: bounding boxes against a unit object; alignments within a tolerance.
- `study.compare`: silhouette overlap on a grid. Not likeness; a good drawing may differ on purpose.
- `study.blockIn`: a few straight lines whose corners lie on the outline; `maxDeviation` says how
  far the outline strays from them (`maxSides` can make that more than `tolerance`).
  `study.hatch`: straight lines clipped exactly by each path's own fill rule (clipping groups are
  refused); jitter is seeded and reproducible.

**Speed's own warnings against formula.** Measurements are never absolutely accurate (p. 88).
Blocking in is scaffolding to be kicked away (p. 92). His method is one way among many (p. 100,
p. 112). Rules never replace the initial artistic impulse (p. 130); canons and composite ideal types
produce deadness (pp. 133–134); a composition perfect by the rules may be of no account (p. 216); a
picture obviously composed is badly composed (p. 218).

**1913 opinion, not verified science.** Fixed emotional meanings of lines and tones; claims about
where the eye goes; beauty, types and physiognomy; the golden section as a law. Use them as
heuristics an artist may choose, never as facts or scores.

**So:** describe, don't score. Never optimise a number at the expense of the brief. Never fake
"life" with random jitter. Never average or beautify a face.

**More from the book.**

- Say when output was traced from a photograph (`imageTrace.*`). A trace is a starting point, not the drawing (SP-I-7, SP-XXI-1).
- `study.compare` belongs to accuracy studies and block-ins. Never iterate expressive work toward a higher `iou` (SP-VI-1).
- Polish is not life, and roughness isn't either: a drawing is academic "only because it is dead" (SP-VI-5).
- Asked for "the style of X", say what its devices are for and apply them to this subject. Don't apply one house style to every brief (SP-VI-7, SP-XXI-3).
