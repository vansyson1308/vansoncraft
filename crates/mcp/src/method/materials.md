# Materials

**Speed says.** The medium decides what you select; never ask more of it than it can do (SP-XX-1).
A drawing is made of painted symbols, not illusion (SP-XX-2). Limitations give power: add means
gradually — monochrome in three tones first, then a few colours (SP-XX-4). Each medium has its
character: the point for line, charcoal for line and mass, chalk on toned paper with the paper as
the half tone, pen for clear darks without half tones (SP-XX-5..15).

**Vector equivalents (ours).**
| Medium (Speed) | VansonCraft means |
|---|---|
| Pencil, silverpoint | thin strokes, slight width variation (width profiles), no fill |
| Pen and ink | strokes with width profiles; two-value design; hatching (`study.hatch`) |
| Charcoal rub-down | a half-tone ground shape, light shapes over it, dark strokes and shapes |
| Chalk on toned paper | a mid-tone artboard ground; light and dark shapes that never touch |
| Etching | stroke-weight tiers; parallel and crossed hatching |

**With VansonCraft.** Keep to the means the brief asks for. A "line study" uses strokes only; a
"mass study" uses filled shapes in a few values. Texture: hatching and repeated vector marks, with
counts kept bounded (`study.hatch` refuses more than `maxLines`).

**More from the book.**

- Material cues (ours, after SP-III-4): crisp edges and small highlights read hard; soft edges and broad gradation read soft.
- Outline, then tint, then shading clipped inside is one convention, the outline-based one (SP-IV-3).
- A flat-mass convention: flat fills, no directional shading, sparse tapered lines on chosen boundaries (SP-V-3). His "Eastern mind" remark is a period stereotype; use only the formal description.
- Keep the level of naturalism even between the figures and the background (SP-XIV-3).
- A pleasing red-chalk-like rendering is not proof the drawing is right: check proportions first (SP-XX-8).
- Explore, then redraw with finality (SP-XX-9, SP-XX-15): group the exploration, set `object.setProps {ids, opacity: 30}`, lock its layer, and draw a few decisive paths on a new layer above.

**Limits.** Grain, smudge, bleeding ink and paper tooth have no honest vector analogue. A raster
effect (feather, blur, texture) may be used only when declared to the user; never rasterise the
whole picture to pass for a drawing.
