# Speed examples: a vase of flowers

Original artworks made for this repository through VansonCraft's MCP server only (an agent's tool
calls; no hand edits), as acceptance scenarios A–D of [docs/speed/acceptance-matrix.md](../../docs/speed/acceptance-matrix.md).
Brief: *a vase of flowers in vector, rhythm of curves first, three value groups (light, middle,
dark); three different compositions; every flower editable.*

| File | What it is |
|---|---|
| `vase-compositions.vansoncraft` | the project: three artboards, one layer per composition, one named group per flower, the vase, the stems and leaves, the setting |
| `vase-A.svg` … `vase-C.svg`, `vase-A.png` … `vase-C.png` | each composition exported (A — diagonal rise, B — low spread, C — tall S-curve) |
| `vase-A-values.png` … | the value-group map of each (`render_preview {mode: "values"}`) |
| `vase-recipes.json` | each composition's recipe (seed, intent), its groups' ids, value shares, masses and balance |
| `vase-compositions-revised.vansoncraft`, `vase-A-revised.svg`, `vase-A-revised.png`, `vase-A-revised-values.png`, `vase-A-revised-focus.png` | composition A revised locally: a stronger line at the accent, the background texture removed, the darks linked by a leaf mass |
| `vase-revision-report.json` | the before/after comparison: which objects changed (only the targets and one container), value groups before and after, undo restoring all |
| `line-and-mass-studies.vansoncraft`, `line-study.svg`, `mass-study.svg`, `line-study.png`, `mass-study.png` | the same subject as a line study (strokes, width profiles, hatching down the form) and a mass study (fills in three values) |
| `line-study-construction.png` | the line study with its block-in and axis on the `Construction` template layer, which exports leave out |

Open a project with `vansoncraft-cli mcp` (`open_file`) or the desktop app; every part is a path or
group you can select, recolour, reshape or delete on its own.
