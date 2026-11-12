# Rhythm: variety and unity

**Speed says.** Rhythm is the power of lines, tones and colours, by their arrangement, to move us as
music does (SP-X). Variety gives life; unity relates every part to one conception; good work holds
both (chapters XI–XIV). Straight lines steady curves; constant curvature (the circle) has the least
character, varying curvature more (SP-XI-2, SP-XI-5). Parallel families give a sustained effect
(SP-XII-3); lines that would meet if extended tie a design together ("sympathetic radiation",
SP-XII-23, SP-XII-24); contours continue across gaps (SP-XII-2). Invented shapes tend to repeat as
"pet forms" (SP-XIII-2). Hide the construction; mechanical jitter is not "dither" (SP-XIV-17).

**With VectorCraft.**
- `study.lines {ids}`: `directions` (length-weighted orientation histogram), `spread` (0 = one
  orientation, 1 = no dominant one, which includes horizontals and verticals evenly split),
  `straightShare`, `curvatureVariation` (any wiggle raises it; read it with `inflections`: an
  egg-like curve varies in curvature with no inflection, SP-XI-5). Use it to *describe* a drawing
  ("almost all curves, no steadying straights"), then decide.
- Repetition with variation: duplicate a shape, then change its size, angle or curvature by hand;
  `study.compare {a, b}` shows how alike two shapes are — an `iou` near 1 between invented shapes
  may be a pet form.
- Tying lines: draw the main swing first as a construction path on the `Construction` layer
  (it is locked: `layer.setProps {id, locked: false}`, draw, then lock it again), and let contours
  echo it.
- `study.hatch {jitter, seed}` varies hatching spacing reproducibly; it is not "life" and should
  stay small.

**More from the book.**

- Speed links straight lines and square forms with strength, and full curves with softness (SP-XII-6). This is 1913 convention. Even "straight" wants slight, deliberate play, never added jitter.
- A flame-like upward flow of curves is restless. He steadies it with a few straights (SP-XII-21).
- Lines crossing near a right angle are harsh. Keep such crossings for drama, or cover them with a mass (SP-XII-25).
- Tie an isolated line in through intermediate lines related both to it and to the dominant line (SP-XII-26).
- He pairs flat tones with straight lines, and gradated tones with curves (SP-XIV-11). A gradient-heavy piece needs steadying straights; read `study.lines` `straightShare` first (SP-XIV-14).

**Limits.** No metric reads rhythm. Speed's fixed meanings of lines (horizontal = calm, vertical =
elevation, etc.) are 1913 artistic convention, not science; never present them as fact.
