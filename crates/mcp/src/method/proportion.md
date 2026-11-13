# Proportion

**Speed says.** Canons of proportion are scaffolding to depart from (SP-XVI-1). Simple, equal
proportions give repose; varied proportions give life; avoid equal divisions unless stillness is
wanted (SP-XVI-2, SP-XVI-3). A few dominant quantities control the variety (SP-XVI-9). When
measuring, use one unit and compare only vertical and horizontal distances, and treat the results
as approximate (SP-VIII-2, SP-VIII-3). His 5:8 and golden-section remarks are offered as one
observation, heavily hedged (SP-XVI-8, Appendix).

**With VansonCraft.**
- `study.proportions {ids, unit}` measures objects in units of a chosen object's height (a head, a
  vase's mouth) and lists edges and centres that line up within a tolerance (plumb and level
  lines), plus the angles between centres.
- Check for accidental equal spacing between parts: equal gaps in `objects` positions, or a
  `heightInUnits` of exactly 1 where variety was wanted.
- Block in first: `study.blockIn` and the `Construction` layer.
- Lock or change a proportion on purpose: scale with `transform {scaleX, scaleY, ids}`.

**More from the book.**

- Limits first (SP-VIII-4):
  1. On the unlocked `Construction` layer, draw a rectangle for the subject's limits and a line for its main axis, then lock the layer again.
  2. Run `study.proportions {ids: [limits, …], unit: limits}`.
  3. Note what falls at the half (the limits' `middle`).
- A long line on the centre halves the panel, and a lone vertical looks "stark and lonely" to Speed. Break up the first with parallels; give the second company (SP-XII-27). To check the centre, include the ground rectangle in `study.proportions` and look for a shared `center` or `middle`.
- He thinks the formality of the layout should suit the degree of naturalism, his "humble opinion": a strictly symmetric layout goes with stylised rendering (SP-XVI-6).

**Limits.** No ratio is "beautiful" in itself. Use a canon or the golden section only when the brief
asks for it; never force a subject — above all a face — into a standard.
