# Mass

**Speed says.** Mass drawing draws the flat appearance: shapes of tone, with what you know of the
form set aside (SP-V-1). Reduce a complicated appearance to a few simple masses — the painter's
first necessity (SP-V-2, SP-IX-2). Set the ground to the half tone, then put down the lights and
the darks, noting where edges are sharp and where they merge (SP-IX-4..6). Build form from planes,
large ones first, avoiding "gross roundness" (SP-IX-12). Keep one focal impression (SP-V-5,
SP-XIII-11); vary edges between sharp and soft (SP-XIII-9). Plan a few large, simple masses first
(SP-V-2, SP-XIII-6); small spots may cross them for variety (SP-XIV-13). When a picture is muddled,
reduce it to flat tones (SP-XV-4). Review with half-closed eyes or a black glass (SP-IX-7).

**With VectorCraft.**
1. Three values first: a light, a half tone and a dark (e.g. L* 85, 55, 20). Make the half tone
   the ground (a rectangle the size of the artboard on a bottom layer), then the light shapes and
   the dark shapes as separate filled paths on named layers.
2. `render_preview {mode: "values", levels: 3}` gives a value-group map. `study.values` reports
   each group's share of the picture, how many separate masses it breaks into, the contrast, and
   the ends of the scale in use (`extremes`). `masses` and `largestShare` describe the plan; they
   are not targets. When the brief asks to link the darks, they show whether it happened.
3. Add gradation and smaller planes inside each mass only after the three-value plan reads. Prefer
   flat planes and stepped values to default radial gradients.
4. Accent: one option is the strongest dark against the strongest light in one place, Turner's
   and Corot's device for unity (SP-XIV-15); Speed also notes the sharpest accents can sit on
   non-essentials (SP-XIII-10). Measure an accent with `study.values {region: [x, y, width,
   height]}` on its area: compare its `contrast` (lightest group's mean L* minus the darkest's)
   with the whole picture's, before and after a change.
5. Edges: soft edges are not native to vector fills. Options: an intermediate value shape between
   two masses, a gradient across the edge, or a feather or blur live effect. Say which you used:
   the effect's radius is one value for the whole object; SVG export writes it as an SVG filter
   (left out of SVG Tiny), PDF export renders the object into an image, and PNG bakes it in.

**Lost and found edges, in order of preference.** (1) Between two masses that should merge, a thin
shape of the value between them, or a gradient fill running across the edge into the neighbour's
value (Speed's half tone as a buffer, SP-IX-14): vector, exports everywhere. (2) For a line, a width
profile tapering to nothing, or a real gap. (3) Only for softness all round a shape, a declared
feather. Never call a vector edge "soft" when it is not.

**Reference and drawing through the same filter.** Place the reference on a template layer beside
the artboard and run `study.values` (or `render_preview {mode: "values"}`) on both regions with the
same `levels` and `thresholds` (SP-XIV-4, SP-IX-7).

**More from the book.**

- Treat complex light, such as sun through leaves or reflections, as flat light and shadow shapes laid across the forms, not as shading per object (SP-V-7).
- On a round form the highlight sits inside the contour, not on the lit edge (SP-VIII-13). His case assumes one window and a pale wall; other lighting moves it.
- In colour, he paints separate middle tones for the lights and the shadows, usually one warm and one cool (SP-IX-15). The direction depends on the light; it is not a law.
- To make a structural line felt where no contour gives it, raise the contrast and sharpen the edge along it, and soften the edges near it. Check with `render_preview {mode: "values"}` (SP-XII-14).
- A soft, gradated piece needs crisp drawing in its key forms (hands, features, folds), or it turns pretty (SP-XIV-12).

**Limits.** Value groups are bands of CIE L* lightness of the rendered picture. They measure tone,
not the subject, and do not know which mass is the "important" one.
