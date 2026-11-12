# Portrait

**Speed says.** Likeness lies in how a face differs from the common type; smoothing away its
characteristics loses vitality and gains no beauty (SP-XVII-1, SP-XVII-2). Likeness is a chord of
the whole, not a sum of features (SP-XVII-3). Look for the differences first, while the eye is
fresh; block in the disposition early with no hard and fast rules (SP-XVII-4, SP-XVII-5). Paired
features are seldom seen as the same: don't force symmetry (SP-XVII-12). Order of work: the
enclosing shape in square lines, the skull, a centre line with brows, nose, mouth and chin, then the
masses, from the impression "seen a long way off" inwards (Procedure XVII).

**With VectorCraft.**
- Work from the user's reference on a template layer (`file.place {template: true}` or a template
  `layer.new`), shown with `render_preview {showTemplates: true}`.
- Landmarks: small marks or short guide paths on the `Construction` layer at brows, nose base,
  mouth, chin; `study.proportions` reads their heights against the head's height.
- Masses: a few dark shapes (eye sockets, under the nose, under the lip, the shadow side) read as a
  face before any outline does (SP-III-6, Diagram I).
- Use `study.compare {a: block-in, b: reference shape}` only on the block-in, to catch large errors
  of disposition. Never iterate a finished portrait toward a higher `iou`: likeness is not the
  silhouette, and Speed's Holbein departs from measured fact on purpose (SP-XVII-11, SP-XVII-12).

**More from the book.**

- Shade the eye's white as part of a sphere, with the lids following it. Mark the change of plane under the lower lid. Draw the brow as a tone shape with soft and harder edges (SP-VIII-19).
- State the portrait's dominant aim: likeness, objective map, decorative pattern, or ceremony. Let it govern palette, pose and ornament (SP-XVII-13).
- Borrow a master's quality only where you see it in this sitter (SP-XVII-9).
- With an energetic gesture, restrain the colour, for example with `recolor.reduce {colors: 4}`, as he reads Hals (SP-XVII-10).
- Prefer a resting expression unless the brief asks otherwise (SP-XVII-14).
- Clothes: fold structure and the large garment shapes come before ornament. Try arrangements on copies made with `artboard.duplicate {art: true}` (SP-XVII-15).

**Limits.** Never average, beautify or symmetrise a face, and never score attractiveness. Speed's
remarks on physiognomy and "types" are period opinion. Draw real people only with the user's own
material and consent.
