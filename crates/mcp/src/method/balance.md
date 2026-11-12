# Balance

**Speed says.** Balance must be approximate: never perfect, never far out (SP-XV-1). Straight
against curved, flat tones against gradated ones, warm against cold (SP-XV-2, -3, -6). Balance the
extremes of the tone scale, not the amounts (SP-XV-5). Interest can balance mass: a small,
interesting accent can hold a large plain area (SP-XV-7). Balance variety against unity (SP-XV-8).

**With VectorCraft.**
- `study.balance` gives the centre of the dark weight (each pixel weighs its darkness on white),
  its offset from the artboard's centre (−1…1), the share of weight in each quarter, and the share
  of near-empty space. On a toned ground use `weigh: "contrast"`, which weighs each pixel's
  difference from the median tone; darkness alone would weigh the ground. Compare layouts with it;
  do not drive the offset to zero (SP-XV-1).
- Three compositions: make three artboards (`artboard.new`, named "A — …", "B — …", "C — …"),
  vary the placement of the main mass, the direction of the main line, and the space left empty;
  run `study.balance` and `study.values` on each and report the differences in plain words with the
  recipe used.
- An accent that is small but strong can offset a large quiet mass: check it with
  `render_preview {mode: "values"}` and a zoom on the accent.

**More from the book.**

- To see what one element does, cover it and look again (SP-XII-11). Use `object.setProps {ids, visible: false}`, then `render_preview`, then `edit.undo`.
- A central, symmetric layout gives repose (SP-XVI-5). Then vary the secondary parts so that the equality doesn't show. If `render_preview {flip: "horizontal"}` looks unchanged, nothing has been varied yet.

**Limits.** The measure weighs darkness only: not colour, subject or meaning. Where a viewer looks
first depends on much more; read the numbers as how the darks are spread, not as the picture's
balance.
