# Ledger A: Speed, *The Practice and Science of Drawing* (1913), front matter, Preface and Chapters I–V

- **Source:** Project Gutenberg #14264, the plain-text extraction `book.txt`, lines 1–2299. Chapter VI starts at line 2300 and is out of range.
- **Reader:** Book Researcher A. Every line in the range was read in order, in chunks of about 250 lines.
- **Citations:** "p. N" is used only where a `[#Page_N]` anchor supports it. A passage belongs to page N when it falls after `[#Page_N]` and before the next anchor. A passage that crosses an anchor is cited as "pp. N–M". Quotes are 25 words or fewer and are verbatim. "…" marks an omission.
- **Three kinds of statement are kept apart:**
  - *Author says*: Speed's own claim.
  - *Our interpretation*: ours.
  - *Implication*: features we infer.
- **Repo context (checked only by grepping ids, not by testing behaviour):**
  - Command ids: `view.outline`, `view.grid`, `view.rotate`, `view.guides*`, `edit.colors.toGrayscale`, `object.path.simplify`, `object.path.smooth`, `object.pathfinder.outline`.
  - Crates: `trace`, `three-d`, `cad`, `brush`, `effects`. File: `crates/engine/src/cmd/perspgrid.rs`.
  - MCP prompts: `poster`, `icon-set`, `recolor`, `trace-and-style`, `export-set`, …
  - MCP resources: document, commands, command-reference.
  - Running the app: `ui.render` and `ui.screenshot`, per AGENTS.md.
  - An implication that says "exists" means only that the id or crate was seen.

## Reading log

| Section (chapter + heading) | book.txt lines | XHTML file | Page anchors covered | Read? | Figures viewed |
|---|---|---|---|---|---|
| Front matter: PG header, title page, frontispiece | 1–82 | `8481025652923775349_14264-h-0.htm.xhtml` | none (comes before `[#Page_v]`) | yes | **Plate I**: whole plate, plus each of the 4 stages cropped and enlarged 2×, with grey-level means |
| PREFACE | 83–241 | same | v, vi, vii, viii, ix, x (`[#Page_xi]` at 239 closes it) | yes | none |
| CONTENTS | 242–322 | same | xi | yes | none |
| LIST OF PLATES | 323–543 | same | xii, xiii, xiv | yes | titles only |
| LIST OF DIAGRAMS | 544–657 | same | xv, xvi | yes | Diagram III, viewed only as a cross-reference (its text is outside this range) |
| I Introduction | 658–1125 | same | 17–30 | yes | Plates II, III, IV |
| II Drawing | 1126–1348 | same | 31–37 | yes | Plates V, VI |
| III Vision | 1349–1698 | same | 38–49 | yes | Plate VII, **Diagram I**, Plate VIII |
| IV Line Drawing | 1699–1952 | same | 50–57 | yes | Plate IX |
| V Mass Drawing, with sub-heading "The Impressionist Point of View." at line 2070 | 1953–2294 | same | 58–67 | yes | Plates X, XI, XII, XIII |
| Boundary: `## VI` heading | 2295–2299 | same | the `[#Page_68]` anchor only | read the heading, then stopped | n/a |

### Where the figures sit, and whether the text discusses them

| Figure | Line | Nearest page anchor | Discussed in text? |
|---|---|---|---|
| Plate I (frontispiece) | 75–81 | before p. v | **No.** Only the caption is in this range. The method appears to be explained in Ch. IX (outside range; book.txt about line 3874 mentions "monochrome studies painted into a middle tone"). |
| Plate II, Leonardo | 855 | between pp. 22 and 23 | No. Leonardo is discussed in Ch. IV. |
| Plate III, "April" | 1007 | p. 27 | No |
| Plate IV, Boreas | 1065 | p. 29 | No |
| Plate V, Botticelli | 1255 | p. 35 | Indirectly: Botticelli's "lightness" is discussed on p. 34 |
| Plate VI, Alfred Stephens | 1327 | p. 37 | No |
| Plate VII, Apollo | 1508 | p. 43 | No |
| **Diagram I** | 1565 | p. 45 | **Yes**, pp. 43–45 |
| Plate VIII, study on toned paper | 1625 | p. 46 | No |
| Plate IX, Watteau | 1802 | p. 53 | No |
| Plate X, Lui Liang | 1989 | p. 59 | Yes (Eastern mass drawing, p. 59) |
| Plate XI, Las Meninas | 2058 | p. 61 | Yes, p. 60 |
| Plate XII, Michelangelo (attrib.) | 2252 | p. 66 | Yes, pp. 66–67 ("On pages 66 and 67…") |
| Plate XIII, Degas | 2264 | p. 66 | Yes, pp. 66–67 |

Plates I, III, IV, VII and VIII carry no artist's name in their captions. That suggests they are by Speed himself, but this is our inference: the text in this range never says so.

### Figure notes (what we saw)

- **Plate I.** "Four photographs of same monochrome painting in different stages illustrating a method of studying mass drawing with the brush". The List of Plates calls it a "study from the life".
  - **Stage 1 (top left).** Dark lines on a white ground. Single contours mark the bald head, ear, eyes, nose, collar and coat edges. Shadow zones (side of the face, hair, coat) are hatched loosely with parallel diagonal strokes. There is no tone in the background. Mean grey level is about 175 of 255, mostly paper.
  - **Stage 2 (top right).** The whole field is covered with a middle grey (mean about 125). The head is laid in as a few broad, faceted planes. The lit forehead, nose ridge, far cheek and chin are lighter. The shadow side of the face, the eye sockets, under the nose and under the chin are darker flat patches. The shirt front is a light vertical band, and the coat is a dark mass of broad strokes. Edges are blocky and soft, and the first drawing's lines are absorbed. Light seems to come from the upper right of the picture.
  - **Stage 3 (bottom left).** The background is slightly darker (mean about 119). There are more, smaller planes. Eyes, brows, nostrils and mouth are articulated. The hair is a defined dark mass, and the collar and shirt planes are separated.
  - **Stage 4 (bottom right).** The planes are fused into continuous modelling. The highest lights (forehead, nose, cheekbone) and the darkest accents (eye sockets, nostrils, hair) are stated. Edges are crisp where the lit face meets the background and lost where the hair and coat merge into it. The background is lighter and more even (mean about 139).
  - A small light rectangle at the lower right of stages 2–4 is probably a label in the photograph.
- **Diagram I.**
  - **A** ("first drawing made by children, showing how vision has not been consulted"): a closed, roughly circular outline; about 14 short radial strokes round the top for hair; two small circles with centre dots for eyes; a small outlined triangle for the nose; a rectangle ruled into a grid of teeth (about 2 × 6) for the mouth. All of it is line, with no tone.
  - **B** ("what might have been expected if crudest expression of visual appearance had been attempted"): no outline at all. A heavy hatched dark mass of hair at the top, two short dark horizontal dashes under the brows for the eye shadows, a small dark mark under the nose, a dark mouth mark, and a hatched dark wedge under the jaw and chin. The head reads as a face from about six dark shapes on white.
- **Plate II, Leonardo (Windsor).** A woman's head in profile. Fine contour lines, with closely spaced diagonal parallel hatching concentrated in the hair and the shadow of the neck. Below, the bust and shoulders are only faint, searching outlines, with several alternative contour lines. A numeral "14" is written near the jaw.
- **Plate III, study for "April"** (red chalk on toned paper). A draped seated woman with her arm raised to her head. Shadow masses are built from close parallel hatching, the drapery folds from line plus hatched tone, and the lights are left as paper.
- **Plate IV, Boreas** (red chalk on tissue paper). A winged male nude in a running or leaping pose over a squaring grid of roughly 7–8 by 9–10 lines, with diagonal construction lines. The wings are long sweeping parallel strokes. Contours are restated in places, and tone is hatched on the shadow side.
- **Plate V, Botticelli.** A standing woman in a long, high-waisted gown with puffed sleeves and streaming hair, a cornucopia shape behind her and infants beside her. Thin searching lines and very little shadow. The hem is blown out, and the figure barely seems to weigh on the ground.
- **Plate VI, Alfred Stephens** (red chalk). A crouching nude enclosed by sweeping circular and elliptical guide lines. Bold restated contours, and hatching on the shadow planes of the limbs. The figure is compact and weighty.
- **Plate VII, Apollo** (red chalk rubbed with the finger, highlights lifted with rubber). A leaning male nude with one arm outstretched, over a squaring grid. The modelling is smooth and rubbed. The few dark accents lie along the shadow-side contours.
- **Plate VIII, study for a picture** (red conté and white pastel rubbed on toned paper). A standing nude with arms raised over her head. The mid-grey paper serves as the middle value: lights are built up in white (torso, face, drapery) and darks in chalk (hair, shadow edges, background next to the figure). There is little line, and the edges are mostly soft, with a few crisp accents such as the upper-arm contours.
- **Plate IX, Watteau.** A standing woman seen from behind in a long checked gown. The checks are drawn as crossing lines that follow the folds. There are dark accents in the fold hollows, and the contour is broken and restated, sometimes heavy, sometimes absent.
- **Plate X, Lui Liang** (15th-century Chinese). Geese among reeds and water. The birds are tonal brush masses (dark wings, lighter breasts) with very little contour. The reeds are long, tapering, calligraphic strokes. There are no cast shadows or directional light, and the ground is flat and atmospheric. The scroll has brocade mount borders.
- **Plate XI, Las Meninas.** The upper half is a large dark mass (ceiling, back wall, dark pictures). The figures run in a band across the lower half. The Infanta, in the centre in a light dress, is the lightest and most contrasted figure. The open doorway at the back is a small light rectangle with a figure in it. There is a dog at lower right and a tall canvas edge at the left. The halftone flattens it, and Speed himself says the reproduction does not show the differences of focus (p. 60).
- **Plate XII, Michelangelo (attributed).** A twisting male torso and thigh in pen. The contours are built from many overlapping convex bulges of muscle. Where a front form crosses a rear one, the front contour continues and the rear one stops, forming T-junctions. Hatching follows the roundness of the forms, the heaviest darks sit in the deepest creases (under the thigh), and the curves are long and swinging.
- **Plate XIII, Degas** (dancer bending to adjust her shoe). The tutu is one large light mass set against a darker floor and background, with diagonal strokes in the background. The thin arms and legs are read mostly as silhouette. Edges range from crisp (arm against tutu) to lost (tutu into light areas).
- **Diagram III** (cross-reference only; its text is in a later chapter). A frame of about 7 × 5 inches, divided by threads into roughly inch squares, with points A and B marked on its edges.

---

## Preface

**Locator:**
- XHTML file: `8481025652923775349_14264-h-0.htm.xhtml`, book.txt lines 83–241.
- Anchors: `[#PREFACE]`, `[#Page_v]` … `[#Page_x]`, closed by `[#Page_xi]`; `pgepubid00001`–`00008`.

### SP-P-1: No short cuts. Principles, not tricks
- *Author says:* The book offers no "wrinkles" for drawing faces, trees, clouds and so on, of the kind Victorian drawing masters loved. Instead it gives practical help that asks more of the student's intelligence. Quote: "No good can come of such methods, for there are no short cuts to excellence." (p. v)
- *Figures:* none.
- *Conditions / counter-examples:* He promises "help of a very practical kind" (p. v). His objection is to recipes for particular subjects, not to practical method.
- *Our interpretation:* A library of subject recipes ("how to draw a tree") is exactly what he rejects. Guidance should work at the level of method: how to see, simplify and order.
- *Implication for vector software:* Agent resources should teach procedures and checks (line study, mass study, value plan). Canned subject templates should not be the core of the drawing guidance.
- *Do not algorithmize because:* this is a stance about how to teach. Subject recipes are the very short cut it warns against.
- *Disposition candidates:*
  - [design-principle]: build agent guidance around method.
  - [agent-prompt-or-resource]: state this stance at the top of the drawing guide.
- *Science caveat:* none.

### SP-P-2: Mechanical accuracy is necessary but not enough
- *Author says:* His art-school training was copying, from simple solids (cubes, cones, cylinders) up to casts and posed models. He calls it excellent mechanical training of eye and hand. But the old masters' drawings were seldom "like the model" (p. vi), so a long road separates mechanically accurate drawing from artistically accurate drawing. Quote: "all this mechanical training of eye and hand was excellent; but it was not enough." (p. v)
- *Figures:* none.
- *Conditions / counter-examples:* He endorses beginning with simple solids, which he calls "an excellent system to begin with" (p. v). He is not against accuracy; see SP-II-5 and SP-II-7.
- *Our interpretation:* There are two quality bars: geometric correctness and expressive rightness. Software can help with the first; the second belongs to the artist.
- *Implication for vector software:*
  - Keep correctness checks (perspective, ellipses, proportion) separate from expressive review in any agent critique loop.
  - Offer a practice set of primitive solids for calibration. The `three-d` crate might render ground-truth projections (not verified).
- *Do not algorithmize because:* "artistically accurate" is defined by the artist's intent (SP-II-6), so there is nothing to measure it against.
- *Disposition candidates:*
  - [practice-exercise]: cube, cone and cylinder drills.
  - [analysis-overlay]: correctness checks only.
  - [design-principle]: keep the two quality bars separate.
- *Science caveat:* none.

### SP-P-3: Words can only vaguely suggest what art deals with
- *Author says:* Writing about art is like writing about the taste of sugar: only those who have tasted it will understand. Written language belongs to the understanding. Quote: "art deals with ideas of a different mental texture, which words can only vaguely suggest." (p. vi)
- *Figures:* none.
- *Conditions / counter-examples:* Books still help people who have "the impelling desire" (p. vii).
- *Our interpretation:* This bears directly on text-driven agents. A text prompt leaves visual qualities underdetermined.
- *Implication for vector software:* Agent workflows should require a render-and-look loop (`ui.render` / `ui.screenshot`) and not trust textual reasoning about geometry. Guidance should pair words with visual examples.
- *Do not algorithmize because:* the gap he describes is precisely what cannot be put into words.
- *Disposition candidates:*
  - [agent-prompt-or-resource]: make the visual check mandatory.
  - [design-principle]: resources show as well as tell.
- *Science caveat:* none.

### SP-P-4: Know what you wish to do. Your point of view decides what you see
- *Author says:* Seen from Raphael's standpoint, a Monet is a "meaningless jargon of wild paint-strokes". Seen from Monet's, a Raphael is "hard, tinny" (p. vii). Points of view produce variety, so theory is "the first essential of any good practice". Exhibitions are full of ingenious work with "no artistic brains" (pp. vii–viii). Quote: "To have a clear idea of what it is you wish to do, is the first necessity of any successful performance." (p. vii)
- *Figures:* none.
- *Conditions / counter-examples:* The point-of-view argument is about how viewers judge. The clear-idea argument is about the maker.
- *Our interpretation:* Before drawing, choose and state a point of view (line or mass, and the quality to express), then judge the result by that point of view and not by another style's criteria.
- *Implication for vector software:*
  - An MCP prompt step, "state intent": subject, approach (line or mass), intended quality (for example bulk or lightness), and focal point. Store it as a document note.
  - Critique later compares the result with that intent.
  - Analysis tools should not apply one mode's criteria to another, for example flagging soft edges in a mass study.
- *Do not algorithmize because:* the intent is the artist's or agent's decision. Software can only record it and remind.
- *Disposition candidates:*
  - [agent-prompt-or-resource]
  - [multi-step-workflow]: intent, then draw, then check against intent.
- *Science caveat:* none.

### SP-P-5: Sound principles to select from abundance, along tradition
- *Author says:* There are "no set traditions to guide us" (p. viii). Reproductions bring the art of the whole world, including China and Japan, to the student, causing "artistic indigestion" (p. viii). Principles are needed to select what answers to the student's "own inner need" (p. ix). He rejects the "anarchists in art" who would start afresh (p. ix). Quote: "it must be along the main stream, along lines in harmony with tradition that the chief advance must be looked for." (p. x)
- *Figures:* none.
- *Conditions / counter-examples:* Sincere fresh starts "are interesting and may be productive of some new vitality" (pp. ix–x).
- *Our interpretation:* An agent trained on everything faces the same overload. Principles act as the filter for which references and styles to draw on.
- *Implication for vector software:* Frame the guidance resource as cited principles, not a style catalogue. Curate reference boards for each intent.
- *Do not algorithmize because:* selection by "inner need" is judgement.
- *Disposition candidates:*
  - [design-principle]
  - [agent-prompt-or-resource]
- *Science caveat:* none. This is cultural-historical opinion.

### SP-P-6: Study line and mass as two opposite points of view, separately
- *Author says:* He advocates "the logical method for the study of drawing from the two opposite points of view of line and mass" (p. x). Quote: "help students to avoid some of the confusion that results from attempting simultaneously the study of these different qualities of form expression" (p. x)
- *Figures:* Plate I, the frontispiece, is a mass study with the brush. Plates XII and XIII later contrast line and mass.
- *Conditions / counter-examples:* SP-III-9 adds that most modern drawing mixes the two. Separation is a discipline for study, not a ban in finished work.
- *Our interpretation:* This is the structural spine of the book: two modes with different primitives, the contour and the tone-shape.
- *Implication for vector software:*
  - Offer two explicit study modes. Line study: strokes only, no fills, one colour. Mass study: filled shapes only, no strokes, a limited set of values.
  - An agent can be constrained to one mode per pass.
  - A layer or document "mode" lint could warn when strokes appear in a mass layer.
- *Do not algorithmize because:* nothing in this is judgement-bound; it is a constraint on workflow.
- *Disposition candidates:*
  - [multi-step-workflow]
  - [analysis-overlay]: the mode lint.
  - [practice-exercise]
  - [design-principle]
- *Science caveat:* none.

---

## I Introduction

**Locator:**
- Same XHTML file, book.txt lines 658–1125.
- Anchors: `[#THE_PRACTICE_AND_SCIENCE_OF_DRAWING]`, `[#CHAPTER_I]` (line 666), `[#Page_17]` (658) … `[#Page_30]` (1103); plates `[#plate02]`, `[#plate03]`, `[#plate04]`.

### SP-I-1: Training perfects the means of expression. Feeling is beyond teaching
- *Author says:* The best things in art are intuitive, and "Intuitions are shy things" (p. 17). Training can deal only with perfecting the means of expression, but the artist should develop that means fully so that he is a "fit instrument" (pp. 17–18). The emotional side is "beyond the scope of teaching". Teachers can only surround students with the best works of art and nature (p. 30). Quote: "Great things are only done in art when the creative instinct of the artist has a well-organised executive faculty at its disposal." (p. 18)
- *Figures:* none discussed. Plates II–IV are placed in this chapter without comment (see Figure notes).
- *Conditions / counter-examples:* How far to pursue intellectual analysis is left "to his individual temperament" (p. 18). The book covers Form and not Colour (p. 18).
- *Our interpretation:* A clean division of labour. Tools and guidance improve the executive faculty. Intent and feeling stay with the author, whether human or agent.
- *Implication for vector software:* Make execution precise and fluent (construction aids, quick revision). Ship exemplars, such as public-domain master drawings as study references, rather than claiming to supply taste.
- *Do not algorithmize because:* the author explicitly puts feeling outside what can be taught. An "emotion score" would be the mechanical substitute he rejects (SP-I-2).
- *Disposition candidates:*
  - [design-principle]
  - [agent-prompt-or-resource]: study of exemplars.
  - [not-automatable]: feeling.
- *Science caveat:* none. This is a philosophical stance.

### SP-I-2: Artistic records are made by a "feeling instrument" and should differ between artists
- *Author says:* Science builds a measured world with instruments, the same for every observer. Art records "the effect of observation on the living consciousness", and only the "feeling instrument—man" can record it (p. 20). Quote: "The same fact accurately portrayed by a number of artistic intelligences should be different in each case" (p. 20)
- *Figures:* none.
- *Conditions / counter-examples:* This is no licence for incompetence (SP-II-7).
- *Our interpretation:* Different renderings are expected and desirable. There is no single correct output for a prompt.
- *Implication for vector software:* Agent workflows may offer several distinct interpretations. Analysis tools check constraints and never measure convergence on a reference rendering. Do not use "similarity to a reference" to score artistic tasks in agent evals.
- *Do not algorithmize because:* individuality is the point.
- *Disposition candidates:*
  - [design-principle]
  - [agent-prompt-or-resource]
- *Science caveat:* none.

### SP-I-3: Abstract form and colour carry feeling, as music does
- *Author says:* Form and colour unconnected with natural appearances have an emotional power like music. They appeal to an inner sense that he finds in all civilised races, Chinese and Japanese art included, which "conform to very much the same sense of harmony" (p. 21). He cites Pater: "music is the type of art" (p. 29). Quote: "the sense of Harmony, Beauty, or Rhythm (all three but different aspects of the same thing)" (p. 21)
- *Figures:* none.
- *Conditions / counter-examples:* Each art also has emotions peculiar to its own sense (p. 20). The deeper motives (awe, joy of life, death) lie outside the technical side of art (pp. 21–22).
- *Our interpretation:* Non-representational arrangement (shape, line, interval) matters apart from the subject. That is the lever the Rhythm chapters (X–XVI) pull, in other ledgers.
- *Implication for vector software:* Analysis of abstract structure (shape sizes, line directions, intervals) is meaningful even in representational art. An "abstract design" view could flatten every object to its value and hide what it depicts.
- *Do not algorithmize because:* harmony is felt. The view reveals structure; judging it stays with the human or agent.
- *Disposition candidates:*
  - [analysis-overlay]: abstract-design view.
  - [design-principle]
- *Science caveat:* An innate sense of harmony shared across cultures is a 1913 aesthetic belief. Empirical aesthetics finds some shared preferences but large cultural and individual variation.

### SP-I-4: Any subject will do. Select and emphasise what expresses the deeper feeling
- *Author says:* "To him nothing comes amiss": beautiful or squalid people, palaces or hovels, the Alps or the Great Western Railway (p. 22). A beautiful picture of a cabbage is not "idealised" by formula but shows a truer, deeper vision (p. 22). The painter finds and emphasises the elements of appearance that express deeper associations, such as a mountain's size, age and permanence, and must paint under their influence, "no amount of technical knowledge will take the place of feeling" (pp. 23–24). Quote: "The commonplace is not the true, but only the shallow, view of things." (p. 22)
- *Figures:* none.
- *Conditions / counter-examples:* Selection is "instinctive" when the painter feels the associations (p. 24).
- *Our interpretation:* The operations are selection and emphasis. What to emphasise follows from the association chosen; for example, permanence suggests big, stable masses. That example is ours.
- *Implication for vector software:*
  - Agent prompt: name the association, then list what will be emphasised and what suppressed.
  - Emphasis tools act on a selection (contrast, scale, line weight). Suppression tools simplify, lower contrast, or merge into the surrounding mass.
- *Do not algorithmize because:* the choice of associations, and of which elements carry them, is judgement.
- *Disposition candidates:*
  - [agent-prompt-or-resource]
  - [multi-step-workflow]: select, emphasise, suppress.
- *Science caveat:* none.

### SP-I-5: Matter and manner are one. Avoid both extremes
- *Author says:* Neither "Art for art's sake" nor "Art for subject's sake" is enough: "neither position can neglect the other without fatal loss" (p. 24). Painting for the paint alone tends to say "See what a clever painter I am!" (p. 25). High ideals with neglected form fail to convince (p. 25). Quote: "In all good art the matter expressed and the manner of its expression are so intimate as to have become one." (p. 25)
- *Figures:* none.
- *Conditions / counter-examples:* A "simple-minded painter" may be moved without knowing it and still select well (pp. 24–25).
- *Our interpretation:* Showing off technique is one failure mode. Good intentions with weak execution is the other.
- *Implication for vector software:* An agent self-critique item: "Does every effect serve the stated intent?" A lint can count stacked effects (drop shadows, gradients, textures). Whether they serve the intent stays a judgement.
- *Do not algorithmize because:* the unity of matter and manner is a holistic judgement.
- *Disposition candidates:*
  - [agent-prompt-or-resource]
  - [design-principle]
- *Science caveat:* none.

### SP-I-6: Art is the "rhythmic expression of feeling". Expression needs ordering, and a catalogue of details is not art
- *Author says:*
  - Raw expression is not art: the boy shouting, the savage flinging his limbs (pp. 26–27).
  - Ordered to a rhythm, the same impulse becomes song and dance. Told in arranged words, an incident becomes poetry. A hut whose walls and roof are proportioned becomes architecture (pp. 27–28).
  - A drawing that is "a sort of catalogue of its appearance in its details" is not necessarily art (p. 28).
  - Working definition: "the Rhythmic expression of Feeling" (p. 27).
  - Quote: "by Rhythm is meant that ordering of the materials of art (form and colour, in the case of painting)" (p. 27). The sentence continues: "so as to bring them into relationship with our innate sense of harmony".
- *Figures:* none.
- *Conditions / counter-examples:* He calls every such definition "inadequate" (p. 26). How good an artist is depends on "the quality of their feeling and the fitness of its expression" (pp. 28–29).
- *Our interpretation:* There are two ingredients. Feeling cannot be taught. Ordering can be, in part, and the later chapters on rhythm, proportion and balance treat it.
- *Implication for vector software:*
  - Ordering is where software can help: proportion, intervals, alignment, repetition with variation.
  - Agent workflow: after placing elements, an "ordering pass" adjusts sizes and intervals, so the result is not an inventory of objects.
  - An analysis can flag a flat hierarchy, where every object has similar size and contrast.
- *Do not algorithmize because:* an "innate sense of harmony" is not a fixed ratio. This is a warning for whoever ledgers the proportion chapters.
- *Disposition candidates:*
  - [design-principle]
  - [multi-step-workflow]: the ordering pass.
  - [analysis-overlay]: a hierarchy check on the size and contrast distribution.
- *Science caveat:* the "innate sense of harmony", as in SP-I-3.

### SP-I-7: Representation must convince. Train representation and expression, not copying
- *Author says:*
  - Without the skill to convince reasonable people, sublime intention ends in the ridiculous. Skilful but feelingless pictures also fail (p. 29).
  - The object of training is "the representation of visible nature and of the powers of expression possessed by form and colour" (p. 29).
  - Students need a better idea of art than "setting down your canvas before nature and copying it", and the chapter hopes to "disturb the 'copying theory'" (p. 30).
  - Quote: "If he fails from lack of skill to make his representation convincing to reasonable people … he will probably have landed in the ridiculous." (p. 29)
- *Figures:* none.
- *Conditions / counter-examples:* Where the motive is genuine feeling, the work "seldom … fails to convey something" (p. 29).
- *Our interpretation:* There are two axes: representational conviction, which can partly be tested, and expressive power, which cannot.
- *Implication for vector software:* Correctness tooling matters (perspective, proportion, anatomical reference). But "trace the reference exactly" is the copying theory. Present Image Trace (the `trace` crate) as a starting point or study aid, not as the drawing.
- *Do not algorithmize because:* expressive power has no test.
- *Disposition candidates:*
  - [design-principle]
  - [agent-prompt-or-resource]
  - [improve-existing]: present the trace presets as study aids.
- *Science caveat:* none.

---

## II Drawing

**Locator:**
- Same XHTML file, book.txt lines 1126–1348.
- Anchors: `[#CHAPTER_II]` (1126), `[#Page_31]` (1127) … `[#Page_37]` (1317); `[#plate05]`, `[#plate06]`.

### SP-II-1: Drawing is the expression of form on a plane surface. Form comes before colour
- *Author says:*
  - Art owes more to form than to colour for its range of expression. Great artists restricted their colour: Apelles reportedly used three colours (black, red, yellow) and Rembrandt "little else".
  - Drawing can be taught more than colour, which "would seem to depend much more on a natural sense" (p. 31).
  - Students should train their eye for form "with all the might" they have.
  - Quote: "By drawing is here meant the expression of form upon a plane surface." (p. 31)
- *Figures:* none.
- *Conditions / counter-examples:* Drawing is "the first" and "also the last" thing the painter studies (p. 31).
- *Our interpretation:* Work form and value first, and add colour as a later layer.
- *Implication for vector software:*
  - Add a non-destructive value view that renders the document in luminance, so form can be judged apart from colour.
  - `edit.colors.toGrayscale` exists but appears to be an edit, not a view (behaviour not verified).
  - Add limited-palette document presets.
- *Do not algorithmize because:* nothing in this needs judgement; it is a definition and a priority.
- *Disposition candidates:*
  - [analysis-overlay]: luminance view.
  - [improve-existing]: greyscale as a view mode.
  - [design-principle]: form before colour.
- *Science caveat:* Pliny's account of the Greek painters' palette is usually read as four colours, white included, so "three" is Speed's report. "Rembrandt used little else" is roughly fair, since his palette was dominated by earth colours, but it also included white and a few other pigments. Low stakes.

### SP-II-2: To express form, one must first be moved by it
- *Author says:* Portraying an object accurately "in cold blood" is not enough. Every object has "an emotional significance, a hidden rhythm" that the "accurate, painstaking, but cold artist" misses. A photograph never has it: "You are never moved to say when looking at one, 'What fine form'" (pp. 31–32). Quote: "To express form one must first be moved by it." (p. 31)
- *Figures:* none.
- *Conditions / counter-examples:* He admits "It is difficult to say in what this quality consists" (p. 32).
- *Our interpretation:* For an agent, "being moved" can at most mean committing to one specific perceived quality of the form before drawing.
- *Implication for vector software:* Prompt step: say what is striking about this form, for example "the weight of the shoulders". Input from a photograph is redrawn selectively, not auto-converted.
- *Do not algorithmize because:* he says outright that the emphasis and selection it produces are too subtle to tabulate: "they escape analysis" (p. 32).
- *Disposition candidates:*
  - [agent-prompt-or-resource]
  - [not-automatable]
- *Science caveat:* "never found in a … photograph" is a 1913 aesthetic opinion that predates the wide acceptance of art photography.

### SP-II-3: Select the significant and suppress the non-essential. Laboured finish can lose the impression
- *Author says:*
  - The emphasis and selection in a drawing made under strong feeling "are too subtle to be tabulated" (p. 32).
  - A few lines drawn quickly can have more vitality and truth than a highly wrought drawing, where the obvious non-essentials creep in and obscure the original impression.
  - "Fine things seem only to be seen in flashes" (p. 32).
  - Quote: "this selection of the significant and suppression of the non-essential that often gives to a few lines drawn quickly … more vitality and truth" (p. 32)
- *Figures:* none.
- *Conditions / counter-examples:* A finished drawing could be better if every touch were added "in tune" with the aim. That is rare, and belongs to "the few great ones" (p. 32).
- *Our interpretation:* This is the risk in long iterative refinement, which is how agent edit loops work: drifting away from the first impression.
- *Implication for vector software:*
  - Capture the first gesture early on its own locked layer. Compare each refinement pass against it.
  - Keep version snapshots so a livelier earlier state can be recovered.
  - Use simplify (`object.path.simplify`) for suppression.
  - A detail-density overlay shows where effort went.
- *Do not algorithmize because:* what is significant is judgement.
- *Disposition candidates:*
  - [multi-step-workflow]: keep the first impression as a reference.
  - [analysis-overlay]: detail density.
  - [agent-prompt-or-resource]
- *Science caveat:* none.

### SP-II-4: Form acts on the viewer's body. Bulk versus lightness as expressive qualities
- *Author says:*
  - Before a fine drawing of a strong man we "feel a thrill of its strength in our own bodies". Before a beautiful woman, its sweetness (pp. 32–33).
  - Art lets us "experience life at second hand" (p. 33).
  - We are "always profoundly impressed" by bulk, vastness or mass: Michelangelo's Sistine ceiling, the weight of the ground, the bulk of a mountain (pp. 33–34).
  - We are also charmed by lightness. Botticelli's figures "seldom have any weight". The Madonna's hands "might be holding flowers". A light wind always passes through his draperies (p. 34).
  - Quote: "The measure of the feeling in either case will be the extent to which the artist has identified himself with the subject" (p. 33)
- *Figures:*
  - Plate V (Botticelli) shows the lightness: thin lines, little shadow, a hem blown out, a figure that barely weighs on the ground.
  - Plate VI (Stephens) is compact and weighty within circular guides. Speed does not say it illustrates bulk; that pairing is ours.
- *Conditions / counter-examples:* Pictures educate by enlarging experience, not by telling a moral story (p. 33).
- *Our interpretation:* Weight and lightness are deliberate choices. Plausible geometric means, all ours:
  - Weight: large unbroken masses, firm contours, forms clearly resting on a support, overlaps.
  - Lightness: thin lines, minimal shadow, few contact cues, drapery in motion.
- *Implication for vector software:* Give agents a vocabulary of named qualities ("bulk", "lightness") with suggested, non-binding means, and public-domain exemplars for each.
- *Do not algorithmize because:* identification with the subject is the source. Our mapping from qualities to geometry is only a heuristic.
- *Disposition candidates:*
  - [agent-prompt-or-resource]
  - [design-principle]
- *Science caveat:* The bodily response echoes the aesthetics of *Einfühlung* (empathy) current around 1900. Modern "embodied simulation" research is suggestive but contested. Treat it as artistic opinion.

### SP-II-5: Academic studies need cold accuracy and high finish, like a singer's scales
- *Author says:*
  - The student must train the eye by "the most painstaking of drawings", as a singer sings scales with equal weight and mechanical time (p. 34).
  - These drawings should be "as highly finished as hard application can make them", so that minute visual expression becomes habitual and later almost instinctive, leaving the mind free "to dwell on the bigger qualities" (p. 35).
  - Quote: "In these school studies feeling need not be considered, but only a cold accuracy." (p. 34)
- *Figures:* none.
- *Conditions / counter-examples:* This is a training mode, separate from artistic work ("during your academic training", p. 37). There is a forward reference to the chapter on academic drawing (p. 34).
- *Our interpretation:* The same activity has two modes, and each has its own success criteria.
- *Implication for vector software:* Accuracy drills for agents, with metrics. Draw a described primitive or cast-like form, then compare it with a ground-truth projection by silhouette overlap and by error in the ellipse axes. This is the one place where metrics belong.
- *Do not algorithmize because:* this is the one mode he wants mechanical, so it may be measured.
- *Disposition candidates:*
  - [practice-exercise]: accuracy drills with metrics.
  - [analysis-overlay]: comparison with a reference silhouette.
- *Science caveat:* none.

### SP-II-6: Drawing must be more vivid than ordinary sight. Accuracy is judged against the intended significance
- *Author says:*
  - Drawing worthy of the name presents form "in a more vivid manner than we ordinarily see them in nature".
  - Each new draughtsman discovers a new significance in common things and renders it "hot and underlined" (p. 35).
  - Calling a drawing untrue because it lacks the commonplace appearance "may be foolish" (p. 35).
  - Science observes like a weighing machine; artistic accuracy is "a sentient individual recording the sensations produced in him" (p. 36).
  - Quote: "Its accuracy depends on the completeness with which it conveys the particular emotional significance that is the object of the drawing." (p. 35)
- *Figures:* none.
- *Conditions / counter-examples:* This is no excuse for faulty drawing (SP-II-7).
- *Our interpretation:* Exaggerating ("underlining") what is significant is legitimate and intended.
- *Implication for vector software:* Critique tools report deviations and never auto-correct them. An agent can mark a deviation as intentional emphasis in an object note, so later checks skip it.
- *Do not algorithmize because:* correctness here is relative to intent.
- *Disposition candidates:*
  - [design-principle]: checks report, they never correct.
  - [agent-prompt-or-resource]
- *Science caveat:* none.

### SP-II-7: A rough physical standard of rightness, scientific aids, subtle departures
- *Author says:*
  - There is no absolute artistic standard, but students who say they "saw it so" are not excused (p. 36).
  - Use "Perspective, Anatomy, and, in the case of Landscape, even Geology and Botany" (p. 36).
  - Expression clothed in physically offensive representation is forgiven only by "the few" (p. 37).
  - In the best drawing, departures from mechanical accuracy are so subtle that many deny they exist (p. 37).
  - "during your academic training let your aim be a searching accuracy" (p. 37).
  - Quote: "there undoubtedly exists a rough physical standard of rightness in drawing, any violent deviations from which … is productive of the grotesque" (p. 36)
- *Figures:* none.
- *Conditions / counter-examples:* How far expression may override physical structure "will always be a much debated point" (p. 37).
- *Our interpretation:* There is a tolerance band. Small, deliberate departures enliven; large ones read as errors.
- *Implication for vector software:*
  - Plausibility checks that report how large a deviation is:
    - Perspective consistency: do nominally parallel lines share a vanishing point?
    - Ellipse orientation on cylinders: the minor axis lies along the cylinder's axis (our geometry, not Speed's).
    - Symmetry.
    - Proportion against a chosen canon.
  - Reporting magnitude lets the agent decide between "subtle" and "violent".
- *Do not algorithmize because:* the line between subtle and violent is judgement. Software can report but not decide.
- *Disposition candidates:*
  - [analysis-overlay]
  - [engine-feature]: check commands that return reports.
  - [agent-prompt-or-resource]: "departures should be subtle".
- *Science caveat:* none. The sciences he names are genuine aids.

---

## III Vision

**Locator:**
- Same XHTML file, book.txt lines 1349–1698.
- Anchors: `[#CHAPTER_III]` (1349), `[#Page_38]` (1350) … `[#Page_49]` (1692); `[#plate07]`, `[#diagram01]` (1565), `[#plate08]`.

### SP-III-1: The first difficulty is learning to see
- *Author says:* A student asked her master whether she should "paint nature as she saw nature". He answered: "Yes, madam, provided you don't see nature as you paint nature" (p. 38). Quote: "the first difficulty the student of painting has to face: the difficulty of learning to see." (p. 38)
- *Figures:* none.
- *Conditions / counter-examples:* An act of vision "is not so simple a matter" as people think (p. 38).
- *Our interpretation:* Seeing is a trained skill separate from mark-making. Habitual schemas contaminate perception.
- *Implication for vector software:* For agents, perceiving the reference and their own output is the bottleneck. Inspect renders (`ui.render`) at several scales and in reduced views (value, silhouette, inverted), not only the scene graph.
- *Do not algorithmize because:* nothing in this needs judgement; it is a premise for the checks that follow.
- *Disposition candidates:*
  - [multi-step-workflow]: look-and-check loops.
  - [agent-prompt-or-resource]
- *Science caveat:* none.

### SP-III-2: The retinal picture is flat: masses of colour with shapes, gradations and edges
- *Author says:*
  - Light travels in straight lines. A red object absorbs all the rays except red. Atmosphere modifies colour with distance, as with distant mountains (p. 39).
  - The pictures on our retinas "are flat, of two dimensions, the same as the canvas on which we paint" (p. 39).
  - Examined "without any prejudice", as in a camera obscura, they consist of masses of colour that give the illusion of depth (p. 39).
  - Quote: "masses of colour in infinite variety and complexity, of different shapes and gradations, and with many varieties of edges" (p. 39)
- *Figures:* none in this chapter. Diagram III (in a later chapter) is the device he offers for seeing appearance as a flat subject.
- *Conditions / counter-examples:* He sets colour aside: "we must not stray into the fascinating province of colour" (p. 39).
- *Our interpretation:* This gives a complete vocabulary for mass drawing: shape, value or colour, gradation, and edge quality. It maps neatly onto vector: closed shape, fill, gradient, and edge treatment.
- *Implication for vector software:*
  - Edge quality is vector's weak point, because every edge is hard by default.
  - Engine feature: per-segment edge softness (feather) along a boundary.
  - Analysis: an inventory of each region's shape, mean value, gradation and boundary edge types.
- *Do not algorithmize because:* nothing in the model needs judgement, but the choices made with it stay the artist's.
- *Disposition candidates:*
  - [design-principle]: shape, value, gradation and edge as the four attributes of a mass.
  - [engine-feature]: variable edge softness.
  - [analysis-overlay]
- *Science caveat:*
  - The retina is curved, and the eye does not form one evenly sharp "picture": the fovea is sharp, the periphery coarse, and the eyes move constantly. Perception is not the viewing of a retinal image.
  - "Flat projection" is optically true and useful for artists. "Mosaic on the retina" is a simplification.
  - "Absorbs all but red" simplifies spectral reflectance.

### SP-III-3: Depth cues, and the picture's single focal plane (the background problem)
- *Author says:*
  - We judge distance "principally" from having two eyes and from focusing at different distances (p. 40).
  - A picture can be focused only at one distance. In nature the background is out of focus while we look at an object, but in a painting it is on the same focal plane. This is "one of the chief causes of the perennial difficulty in painting backgrounds" (p. 40).
  - Painters' devices for this "do not concern us here" (p. 40).
  - Quote: "in a painting the background is necessarily on the same focal plane as the object" (p. 40)
- *Figures:* none.
- *Conditions / counter-examples:* He defers the remedies.
- *Our interpretation:* A picture must suggest that unattended regions are out of focus by other means, such as softer edges, lower contrast and less detail.
- *Implication for vector software:* A "focus falloff" tool. Mark a focal object, then progressively soften edges and reduce contrast and detail on groups by depth or by distance from focus, non-destructively. Ties to SP-V-5.
- *Do not algorithmize because:* how much to soften is judgement. A parametric tool is fine.
- *Disposition candidates:*
  - [multi-step-workflow]
  - [engine-feature]: non-destructive focus falloff on groups.
- *Science caveat:* Modern vision science counts many monocular cues as major: occlusion, perspective, shading, texture gradient, relative size, motion parallax. Accommodation is a weak cue except close up. Speed's ranking of binocular vision and focusing as the "principal" cues is a 1913 simplification.

### SP-III-4: Vision is bound up with touch. We "see the feel" of things, and associations vary by person
- *Author says:*
  - Two retinal pictures and focusing would not explain our knowledge of solidity without the sense of touch (p. 40).
  - The infant learns to link the feel of things with their look: the hard bed-post, the soft pillow (pp. 40–41).
  - Everybody "sees" whether a thing looks hard or soft: "the 'feel' of it" (p. 42).
  - Associations differ: an ordinary person and a great poet describe one scene very differently. A great painter's form stirs "deep associations" that he passes on to the viewer (pp. 42–43).
  - Quote: "Sight is therefore not a matter of the eye alone." (p. 42)
- *Figures:* Plate VII sits here without comment: rubbed red chalk, highlights lifted, soft modelling with accents along the shadow-side contours.
- *Conditions / counter-examples:* Touch is the "universal" association he singles out from many others (p. 42).
- *Our interpretation:* Material qualities such as hard, soft, rough and smooth are read from visual cues: edge sharpness, highlight shape, texture. Speed says only that the look of hardness is learned; the cue list is ours.
- *Implication for vector software:* A material-cue vocabulary for agents. Hard: crisp edges and small sharp highlights. Soft: soft edges and broad gradations. This would be guidance, not a rule.
- *Do not algorithmize because:* the deeper associations (the poet against the ordinary viewer) are personal.
- *Disposition candidates:*
  - [agent-prompt-or-resource]
  - [design-principle]
- *Science caveat:* "Touch calibrates vision" is the classic empiricist account (Berkeley; Molyneux's problem). Modern developmental research shows substantial early visual competence alongside learning across the senses. It is an open, partly empirical question. Treat it as Speed's hypothesis.

### SP-III-5: The mental idea of a shape differs from how it looks: the cone and the ellipse, and elevations
- *Author says:*
  - Asked to think of a cone, people picture a circular base with a side sloping up to a point above its centre, "as one would feel it" (p. 42).
  - Architectural elevations and geometrical projections appeal to the mental idea. They are views no one could ever see, with the eye in front of every part at once, yet they "convey a very accurate idea" and "can be scaled" (p. 45).
  - Quote: "in almost every visual aspect the base line is that of an ellipse, not a circle" (p. 42)
- *Figures:* none.
- *Conditions / counter-examples:* Elevations have "great advantage as working drawings" (p. 45). Both kinds of drawing are legitimate.
- *Our interpretation:* There are two legitimate kinds of representation: projection for knowledge (orthographic, measurable) and projection for appearance (perspective, foreshortened).
- *Implication for vector software:*
  - **Geometry.** A circle drawn on a receding plane must be an ellipse. Provide a perspective-ellipse helper (`perspgrid.rs` exists; behaviour not verified). Add a lint for circular ends on objects that are otherwise drawn in perspective.
  - **Modes.** Keep the CAD and dimension work (the `cad` crate), which uses the mental or elevation view, distinct from visual illustration.
  - **Practice.** Cones and cylinders at many tilts.
- *Do not algorithmize because:* not judgement-bound. The ellipse is a geometric fact.
- *Disposition candidates:*
  - [improve-existing]: a perspective ellipse helper (or [engine-feature] if none exists).
  - [analysis-overlay]: the circle-in-perspective lint.
  - [agent-prompt-or-resource]
  - [practice-exercise]
- *Science caveat:* none. Projective geometry confirms it: a circle projects to an ellipse, which becomes a circle face-on and a line edge-on. Our own addition: in perspective, the centre of the ellipse is not the projection of the circle's centre.

### SP-III-6: First drawings consult touch, not vision (Diagram I)
- *Author says:*
  - The child draws a head as "an object having a continuous boundary in space", which the mind conceives "as a line".
  - Hair is a row of little lines around the top. Eyes are points or circles. The nose is a triangle or an L, which is how it feels.
  - The mouth gets a row of teeth "so seldom seen, but always apparent if the mouth is felt" (p. 43).
  - He finds the type universal, from a scribble at Pompeii to "savage drawing generally" (pp. 43–44).
  - The simplest reduction of what is actually seen would be Diagram B: "the shadows under eyes, nose, mouth, and chin, with the darker mass of the hair". It is just as easy to draw, but it does not satisfy the touch-based idea of the object (pp. 44–45).
  - Quote: "A blind man would not draw differently, could he but see to draw." (p. 44)
- *Figures:* Diagram I (see Figure notes). A is an outline with symbol features. B is about six dark masses and no outline, and it still reads as a face.
- *Conditions / counter-examples:* Type B "does not appeal to the ordinary child" (pp. 44–45). Elevations show that a non-visual idea of form can still be useful (p. 45).
- *Our interpretation:*
  - Symbolic drawing (A) is the touch schema; shadow-mass reduction (B) is the visual alternative.
  - Agent-made vector drawings, and SVG generated by language models, default to type A.
  - For icons and pictograms, type A is often the right choice. The intent decides.
- *Implication for vector software:*
  - **Prompt.** When the task is representation rather than iconography, do not reach for the outline-plus-symbols schema. Place the few darkest masses as filled shapes with no outline.
  - **Practice ("Diagram B").** Make a recognisable head from six or fewer filled dark shapes and no strokes.
  - **Lint (heuristic).** Detect a "symbol schema": a closed outline with small, symmetric glyphs inside it, all strokes and no fills.
- *Do not algorithmize because:* whether the job wants a schema or appearance is the author's choice. The lint is only a hint.
- *Disposition candidates:*
  - [agent-prompt-or-resource]
  - [practice-exercise]
  - [analysis-overlay]: the heuristic lint.
  - [design-principle]
- *Science caveat:*
  - "Not yet been pointed out" overstates it. To our knowledge, child-art research of the same period, such as Luquet's account of children drawing what they *know* ("intellectual realism"), made related points.
  - The explanation by touch in particular is Speed's hypothesis. Children's schemas are also explained by conceptual knowledge and graphic convention.
  - "Universal" is an overgeneralisation.
  - Our note, not Speed's: studies of drawings by blind people (for example J. M. Kennedy's) report outline used for edges, which is broadly consistent with outline as a non-visual idea of a boundary.

### SP-III-7: Vision is used to serve the other senses. Finish means completeness of the idea, not amount of detail
- *Author says:*
  - Most people consult vision only to serve other ends: the weather, the hay crop, how deep a stream is, whether that is a balloon among the clouds (pp. 45–46).
  - So they understand only pictures carried far enough to suggest those other associations. Other pictures, "they say, are not finished enough" (p. 46).
  - The Pre-Raphaelites' intense detail is an exception, because it was part of their poetic intention (p. 46).
  - Finish refers only to "the completeness with which the emotional idea the painter set out to express has been realised" (p. 46).
  - Quote: "Finish in art has no connection with the amount of detail in a picture" (p. 46)
- *Figures:* Plate VIII sits here without comment. The toned paper carries the middle value, with white for lights and chalk for darks. The study is complete with very little line and almost no detail.
- *Conditions / counter-examples:* Detail is finish when it is part of the intent, as with the Pre-Raphaelites.
- *Our interpretation:* This gives a stopping rule: stop when the idea is realised, however much detail there is.
- *Implication for vector software:*
  - Define "done" from the intent statement (SP-P-4), not from counts of objects or anchors.
  - Critique tools must not reward detail.
  - A detail-density heat map (anchors or objects per area) makes the distribution of detail visible, so it can be concentrated where intended.
- *Do not algorithmize because:* the completeness of an emotional idea cannot be measured.
- *Disposition candidates:*
  - [agent-prompt-or-resource]: a definition of done.
  - [analysis-overlay]: detail density.
  - [design-principle]
- *Science caveat:* none. Claims about "the great majority of people" are opinion.

### SP-III-8: Defeat touch associations by looking upside down
- *Author says:*
  - The best argument for art in general education is that "it should teach people to see" (p. 47).
  - The Japanese habit of looking at a landscape upside down between the legs surprises one into seeing colour and form for once, "with the association of touch for the moment forgotten" (p. 47).
  - He notes that the retinal picture, normally inverted, is then "the right way up" (p. 47).
  - Quote: "a way of seeing without the deadening influence of touch associations" (p. 47)
- *Figures:* none.
- *Conditions / counter-examples:* none given.
- *Our interpretation:* Defamiliarising the image, by inverting or mirroring it, reveals shapes as shapes.
- *Implication for vector software:*
  - Quick view-only toggles for a 180° rotation and a horizontal mirror. `view.rotate` exists; the mirror must not change the document.
  - An agent check step: render inverted and compare shapes and values.
  - An exercise: copy a reference upside down.
- *Do not algorithmize because:* not judgement-bound; it is a viewing aid.
- *Disposition candidates:*
  - [improve-existing]: rotate and mirror view toggles.
  - [analysis-overlay]
  - [multi-step-workflow]
  - [practice-exercise]
- *Science caveat:* The explanation, suppressing "touch associations", is Speed's. That inversion disrupts familiar-object and face recognition and makes shapes look unfamiliar is well supported (the face inversion effect), so the practice is sound even though its explanation is dated. The aside about the retinal image being "right way up" is optically correct but has no bearing on perception.

### SP-III-9: Two approaches, Line and Mass. They converge and interconnect, and should be studied separately
- *Author says:*
  - The first approach is mental perception, built from touch together with vision, "whose primitive instinct is to put an outline round objects as representing their boundaries in space" (p. 48).
  - The second is visual perception: colour shapes on the retina, "a sort of mosaic of colour" (p. 48).
  - Carried far enough, both give very similar results.
  - "You cannot put an accurate outline round an object without observing the shape it occupies in the field of vision", and it is hard to consider colour masses without being conscious of the objects (p. 48).
  - He names the first Line Drawing and the second Mass Drawing (p. 48).
  - Neglect line and you lose the "expressive significance of form" that only a feeling for lines conveys. Neglect mass and you are poorly equipped "with a brush full of paint" (p. 49).
  - Quote: "Most modern drawing is a mixture of both these points of view, but they should be studied separately if confusion is to be avoided." (p. 48)
- *Figures:* Plate I (mass), and Plates XII and XIII (Ch. V).
- *Conditions / counter-examples:* Mixing the two is normal in finished modern work. Separation is for study.
- *Our interpretation:*
  - This is the book's backbone.
  - In vector, strokes roughly correspond to line and fills to mass. That mapping is ours and imperfect: a filled shape also has a boundary, and in mass drawing that boundary is "the boundaries of visual masses" (SP-V-11).
- *Implication for vector software:*
  - **Workflow.** A two-pass agent workflow, line then mass, on separate layers.
  - **Line-only view.** Strokes and contours at their true weight. This differs from the wireframe of `view.outline`.
  - **Mass-only view.** Fills flattened, strokes hidden.
  - **Lint.** Flag the "colouring-book" look, where a mass study has a uniform outline round every fill.
- *Do not algorithmize because:* which approach suits a subject is judgement.
- *Disposition candidates:*
  - [multi-step-workflow]
  - [analysis-overlay]: line-only and mass-only views.
  - [design-principle]
  - [practice-exercise]
- *Science caveat:* none. The two-way split is a teaching model, not neuroscience.

---

## IV Line Drawing

**Locator:**
- Same XHTML file, book.txt lines 1699–1952.
- Anchors: `[#CHAPTER_IV]` (1699), `[#Page_50]` (1700) … `[#Page_57]` (1928).
- Footnote `[#FNanchor_1_1]` / `[#Footnote_1_1]`: Leonardo, *Treatise on Painting*, para. 178. Para. 176 is quoted in the text. `[#plate09]`.

### SP-IV-1: Outlines exist in appearance only as boundaries of masses, and those boundaries are lost and found
- *Author says:*
  - The earliest drawing in history, like the child's, is outline, though lines have a "somewhat remote relation" to vision (p. 50).
  - Boundaries in appearance "are not always clearly defined, but are continually merging into the surrounding mass and losing themselves to be caught up again later on and defined once more" (p. 50).
  - The instinct for line comes from touch: "When an object is felt there is no merging in the surrounding mass, but a firm definition of its boundary" (p. 50).
  - Quote: "Outlines can only be said to exist in appearances as the boundaries of masses." (p. 50)
- *Figures:* Plate IX (Watteau) sits in this chapter: contours broken and restated, from heavy to absent. Not discussed.
- *Conditions / counter-examples:* Line's visual justification is weak, and he still values it highly (SP-IV-2).
- *Our interpretation:* A visible edge is a difference of value or colour between two masses, and its strength varies along its length.
- *Implication for vector software:*
  - **Edge-contrast overlay.** Compute the luminance difference along each shared boundary between adjacent fills, showing where an edge is visually "found" (strong) or "lost" (weak). This is computable.
  - **Strokes.** Allow strokes with variable width and opacity and real gaps along their length (`stroke.rs` exists; its width profiles were not checked).
  - **Agent guidance.** Avoid closed outlines of uniform weight round everything.
- *Do not algorithmize because:* where to lose an edge for expression is judgement. The contrast map itself is objective.
- *Disposition candidates:*
  - [analysis-overlay]: the edge-contrast map.
  - [engine-feature]: lost-and-found strokes and soft edges.
  - [agent-prompt-or-resource]
- *Science caveat:* The touch origin of line is Speed's hypothesis (SP-III-4). Treating visible boundaries as luminance or colour discontinuities agrees with modern accounts of edge detection.

### SP-IV-2: Line appeals to the imagination, directs the eye, and has a music of its own
- *Author says:*
  - The emotional stimulus of fine design comes largely from line work.
  - A line instinctively directs the eye along its course, "enabling the artist to concentrate the attention of the beholder where he wishes" (pp. 50–51).
  - There is "a music of line that is found at the basis of all good art" (p. 51), treated later under line rhythm.
  - Blake, whose "visual knowledge was such a negligible quantity" but whose "mental perceptions were so magnificent", insisted on line (p. 51).
  - Quote: "The power a line possesses of instinctively directing the eye along its course is of the utmost value" (p. 50)
- *Figures:* none.
- *Conditions / counter-examples:* Deferred to the rhythm chapters.
- *Our interpretation:* Lines are compositional vectors as well as boundaries.
- *Implication for vector software:*
  - An overlay that extends the main tangents and axes to show where they lead: into the focal area, or off the canvas.
  - A "line flow" view of the dominant stroke directions.
  - An agent heuristic (ours): the main lines lead toward the focal point.
- *Do not algorithmize because:* the "music of line" is felt. The overlays inform; they do not judge.
- *Disposition candidates:*
  - [analysis-overlay]
  - [agent-prompt-or-resource]
  - Cross-reference: Ch. X–XII (other ledgers).
- *Science caveat:* Eye-tracking shows gaze jumping in saccades between fixations on salient or informative points, not gliding along lines. Any effect of "leading lines" on attention is modest. Treat "directs the eye along its course" as an artistic heuristic.

### SP-IV-3: The historical layering: outline, then tint, then shading put inside the outline
- *Author says:*
  - Early Egyptian wall paintings were "outlines tinted". Sculpture went from incised outline to cutting away between the outlines into low relief, which may have suggested shading between outlines (p. 51).
  - "Light and shade were not seriously perceived until Leonardo da Vinci" (p. 51).
  - Light and shade were then put inside the outline "as something as it were put on the object" (p. 52). Leonardo valued the relievo, the "standing out" quality (p. 52).
  - Quote: "Botticelli used nothing else than an outline lightly shaded to indicate form." (p. 51)
- *Figures:*
  - Plate II (Leonardo): contour with hatching inside it.
  - Plate V (Botticelli): outline, lightly shaded.
- *Conditions / counter-examples:* The "solids in space" idea governed this tradition (p. 52).
- *Our interpretation:* This is the usual vector illustration pipeline: outline, then flat fill, then shading shapes clipped inside. It is one convention, the touch-based one, not the visual one.
- *Implication for vector software:* A named workflow template for the outline-based convention: outline, then tint, then shading drawn inside with a clip, mask or Draw Inside. Name it, so agents know it is a choice.
- *Do not algorithmize because:* not judgement-bound; it is a workflow.
- *Disposition candidates:*
  - [multi-step-workflow]
  - [design-principle]
- *Science caveat:* This is art history, not science, and it is overstated. Shading (Greek *skiagraphia*) existed in ancient painting, and in Giotto and Masaccio before Leonardo.

### SP-IV-4: The eye sees only what it is on the look-out for
- *Author says:* That light and shade were so long unseen is proof of this, and there may be things "just as wonderful yet to be discovered in vision" (p. 52). Quote: "the eye only sees what it is on the look-out for" (p. 52)
- *Figures:* none.
- *Conditions / counter-examples:* none given.
- *Our interpretation:* Observation is directed. Looking without a target finds little.
- *Implication for vector software:*
  - Design each analysis view to answer one question (value, silhouette, edges, line direction, detail).
  - Agent checklists become targeted passes, "look for X", instead of "does it look good?"
- *Do not algorithmize because:* not judgement-bound; it is a design rule for the tools.
- *Disposition candidates:*
  - [design-principle]: one overlay, one question.
  - [multi-step-workflow]
- *Science caveat:* none. It agrees with modern findings on attention, such as inattentional blindness.

### SP-IV-5: Leonardo's "invariable" outlines refer to the mental idea of form, not to appearance
- *Author says:*
  - Leonardo (para. 176) says knowledge of outline is "of most consequence" and can be learnt, because the outlines of the figure, "particularly those which do not bend, are invariably the same", while shadows are infinite (p. 52).
  - Speed answers that visually the space figures occupy "is by no means 'invariably the same'" (p. 52).
  - The phrase "those that do not bend" confirms that Leonardo means the felt form (p. 53).
  - Quote: "It can only refer to the mental idea of the shape of the members of the human figure." (p. 53)
- *Figures:* none.
- *Conditions / counter-examples:* none given.
- *Our interpretation:* "Outline" has two senses: the canonical shape of a part, which is knowledge independent of view, and its projected silhouette, which depends on the view.
- *Implication for vector software:*
  - Agents building figures from reusable parts (symbols, components) must re-project and foreshorten them for each view, not reuse one silhouette at every angle. This is a common failure of component-based drawing.
  - Possible: symbols with view variants, or parts driven by 3D (the `three-d` crate; speculative).
- *Do not algorithmize because:* not judgement-bound.
- *Disposition candidates:*
  - [agent-prompt-or-resource]
  - [design-principle]
  - [engine-feature]: view-dependent symbol variants (speculative).
- *Science caveat:* none.

### SP-IV-6: The outline basis held pictures together, and fusing edges approached the visual method
- *Author says:* Through Leonardo, Raphael, Michelangelo, Titian and the Venetians, the outline basis remained "as the means of holding their pictures together" (p. 53). Quote: "the Venetians, by fusing the edges of their outline masses, got very near the visual method" (p. 53)
- *Figures:* none.
- *Conditions / counter-examples:* Velázquez completed the change (SP-V-4).
- *Our interpretation:* Hard-outline construction and fused-edge mass are two ends of one continuum. The degree of edge fusion is a style control.
- *Implication for vector software:* A global "edge fusion" preview slider that renders the same shapes from hard-edged to softly fused (feather or blur), while the outline skeleton stays editable underneath.
- *Do not algorithmize because:* the degree is a choice.
- *Disposition candidates:*
  - [engine-feature]: global edge-softness preview.
  - [analysis-overlay]
- *Science caveat:* none.

### SP-IV-7: Accumulated visual detail can obscure the design. Study early work, where the skeleton shows
- *Author says:*
  - Each discovered visual fact adds "another instrument to the orchestra", which widens the range of expression but makes composition harder (p. 53).
  - Botticelli's simple means give "an innocence and imaginative appeal" (p. 53).
  - When the realisation of appearance is most complete, the mind is "led away by side issues", looking "not at it but through it" (pp. 53–54).
  - The stump and charcoal displaced the point (p. 54).
  - Revivals of earlier convention insisted on outline: Greek archaism, Rossetti, Burne-Jones, Puvis de Chavannes (pp. 54–55).
  - Study "the early rather than the late work", because "The skeleton of the picture is more apparent in the earlier" (p. 55).
  - Quote: "The accumulation of the details of visual observation in art is liable eventually to obscure the main idea and disturb the large sense of design" (p. 54)
- *Figures:* none.
- *Conditions / counter-examples:* Primitiveness alone is no virtue: "one need not make a hero of the pavement artist" (p. 55).
- *Our interpretation:* Establish the design (the skeleton) first, and keep it legible under the detail.
- *Implication for vector software:*
  - **Skeleton view.** Show only the N largest shapes and the main lines, by an area filter or a "design" layer.
  - **Agent workflow.** Design the skeleton first, add detail, then re-check that the skeleton still reads.
  - **Reference study.** When learning a style, prefer early or simple exemplars.
- *Do not algorithmize because:* how much detail is too much is judgement.
- *Disposition candidates:*
  - [analysis-overlay]: the skeleton view.
  - [multi-step-workflow]
  - [agent-prompt-or-resource]
- *Science caveat:* none. That realism distracts is opinion.

### SP-IV-8: The medium suits the method: the point for line, charcoal and stump for mass
- *Author says:* At the height of naturalism, line went out of fashion and "charcoal, and an awful thing called a stump, took the place of the point". Line is reviving, and nothing better restores "a fine sense of design" (p. 54). Quote: "Charcoal is a beautiful medium in a dexterous hand, but is more adaptable to mass than to line drawing." (p. 54)
- *Figures:* none.
- *Conditions / counter-examples:* none given.
- *Our interpretation:* Tools have affinities with methods. The stump is the equivalent of a smudge or blur tool, and he distrusts it.
- *Implication for vector software:*
  - Workspace presets: a line-study set of pen, pencil and stroke tools, and a mass-study set of blob brush, shape builder and fill tools. This is our idea; the existence of those tools was not checked.
  - Do not make smudging or blurring the default way to get tone.
- *Do not algorithmize because:* not judgement-bound.
- *Disposition candidates:*
  - [design-principle]
  - [improve-existing]: workspace presets (speculative).
- *Science caveat:* none.

### SP-IV-9: Refinement by very slight departures: the Parthenon's curved lines and inclined columns
- *Author says:*
  - The Parthenon unites the primitive with the most refined.
  - Its architects sloped the columns inwards "to add to the strength of its appearance".
  - "The amount of these variations is of the very slightest".
  - It stays simple, with "the primitive strength of Stonehenge" in its row of columns rising from the steps "without any base" (p. 56).
  - Quote: "What architect now thinks of correcting the poorness of hard, straight lines by very slightly curving them?" (p. 56)
- *Figures:* none.
- *Conditions / counter-examples:* The variations are "of the very slightest".
- *Our interpretation:* Mechanical straightness and regularity can look poor. A barely perceptible curve or lean adds life, so the amounts must stay tiny.
- *Implication for vector software:*
  - A "bow / entasis" command that adds a controlled tiny curvature to straight segments, as a sagitta that is a percentage of the length, and an optional slight inward lean for repeated verticals.
  - Agent guidance: where life is wanted, avoid perfectly straight and perfectly regular repetition, and keep any departure barely perceptible.
- *Do not algorithmize because:* whether a line wants curvature is judgement. A parametric tool is fine.
- *Disposition candidates:*
  - [engine-feature]
  - [agent-prompt-or-resource]
- *Science caveat:* The curvature and the inward-leaning columns are documented in the archaeology. Their purpose (optical correction, aesthetics, construction) is debated. "Poorness of hard, straight lines" is aesthetic opinion.

### SP-IV-10: Art's highest mark is primitive strength plus refinement, built on a simple line scheme
- *Author says:*
  - Michelangelo joined primitive grandeur to exquisite refinement. His followers copied the muscular type but lost the strength, and "a decadence set in" (p. 56).
  - Art reaches "its highest mark" when "the infinite refinements and graces of culture" are added to "the primitive strength and simplicity of early art" without weakening it (p. 56).
  - The modern artist must select from a huge visual wealth, including the arts of the East. "The steadying influence and discipline of line work were never more necessary" (p. 57).
  - Quote: "no work that aims at a sublime impression can dispense with the basis of a carefully wrought and simple line scheme" (p. 57)
- *Figures:* none.
- *Conditions / counter-examples:* Primitive conditions alone make "a very uncouth affair" (p. 55).
- *Our interpretation:* The order of work is a simple, strong scheme first, with refinements added later and kept subordinate. Copying a master's surface without his structure repeats the followers' decadence.
- *Implication for vector software:*
  - A workflow gate: lock the simple line scheme on its own layer before refinement, and re-check it after each refinement pass (the skeleton view from SP-IV-7).
  - Agent guidance warns that imitating a style's surface without its structure fails.
- *Do not algorithmize because:* "sublimity" cannot be measured.
- *Disposition candidates:*
  - [multi-step-workflow]
  - [design-principle]
  - [agent-prompt-or-resource]
- *Science caveat:* none.

---

## V Mass Drawing

**Locator:**
- Same XHTML file, book.txt lines 1953–2294.
- Anchors: `[#CHAPTER_V]` (1953), `[#Page_58]` (1954) … `[#Page_67]` (2279).
- Sub-heading "The Impressionist Point of View." at line 2070 (no anchor).
- Footnote `[#FNanchor_2_2]` / `[#Footnote_2_2]`: Goethe, quoted in Carlyle's *French Revolution*.
- Plates: `[#plate10]`–`[#plate13]`.

### SP-V-1: Mass drawing draws the flat appearance, with knowledge of the felt shapes forgotten
- *Author says:*
  - Outline is a Western instinct, because "the first mental idea of an object is the sense of its form as a felt thing, not a thing seen" (p. 58).
  - Mass drawing is the other basis.
  - Its "scientific truth … is obvious". But if accurate copying were the sole object of art, the photographic camera would soon solve painting (p. 58).
  - Quote: "based on the consideration of the flat appearances on the retina, with the knowledge of the felt shapes of objects for the time being forgotten" (p. 58)
- *Figures:* none.
- *Conditions / counter-examples:* Copying alone is not the object.
- *Our interpretation:* Mass drawing suspends knowledge of the object for a while.
- *Implication for vector software:*
  - A mass-study mode in which the agent works from a reference image's value regions, or from its own render, as flat shapes, not from the meaning of the objects.
  - Analysis can posterise a reference into N value regions with the `trace` crate, to study the shapes.
  - The final drawing remains selective (SP-V-9).
- *Do not algorithmize because:* pure copying is exactly what he says is not art.
- *Disposition candidates:*
  - [multi-step-workflow]
  - [improve-existing]: a "mass study" trace preset.
- *Science caveat:* as for SP-III-2.

### SP-V-2: Reduce a complicated appearance to a few simple masses. This is the painter's first necessity
- *Author says:* Mass drawing "is the natural means of expression when a brush full of paint is in your hands". The full explanation is deferred to the practical chapter (Ch. IX) (pp. 58–59). Quote: "The reducing of a complicated appearance to a few simple masses is the first necessity of the painter." (pp. 58–59)
- *Figures:* **Plate I** (see Figure notes):
  1. Line placement, with hatched shadow zones.
  2. The whole field brought to a middle grey, and the head laid in as a few flat, faceted light and dark planes with soft, blocky edges.
  3. Subdivision into smaller planes, features articulated, darks strengthened.
  4. Planes fused into continuous modelling. The highest lights and darkest accents come last. Edges are crisp at the lit face and lost at the hair and coat.
- *Conditions / counter-examples:* The text of this range does not describe Plate I's procedure. These stages are read from the image alone. Ch. IX (outside range) should confirm the order: for example, whether the middle tone is the ground or the first lay-in.
- *Our interpretation:* Work coarse to fine by masses: a few big masses on a middle-value ground, then subdivide, then fuse and add accents.
- *Implication for vector software:*
  - **Signature agent workflow** (the "Plate I workflow" or MCP prompt `mass-study`):
    0. Optionally, place the main lines lightly.
    1. Set a middle-value ground with a background rectangle.
    2. Block in two to four big masses (light and shadow) as flat fills with no strokes.
    3. Subdivide into secondary planes and half-tones.
    4. Refine edges, losing some and sharpening others, and place the highest lights and darkest darks last.
  - **Analysis.** A posterised value view, and a count of distinct value regions at coarse scale, as a "few masses?" check.
- *Do not algorithmize because:* which masses matter is judgement. The steps can be scaffolded.
- *Disposition candidates:*
  - [multi-step-workflow]
  - [agent-prompt-or-resource]
  - [analysis-overlay]: posterised value view.
  - [practice-exercise]
- *Science caveat:* none.

### SP-V-3: Eastern flat vision: restrained line supports mass boundaries, with no light and shade and a wide diffused light
- *Author says:*
  - Chinese and Japanese art rests on "nature regarded as a flat vision, instead of a collection of solids in space" (p. 59).
  - "Light and shade, which suggest solidity, are never used, a wide light where there is no shadow pervades everything". The drawing is done "with the brush in masses" (p. 59).
  - He speculates that the Eastern mind is less "obsessed by the objectivity of things" (p. 59).
  - Quote: line is "seldom used to accentuate the solidity of things, but chiefly to support the boundaries of masses and suggest detail" (p. 59)
- *Figures:* **Plate X** (Lui Liang). The geese are tonal brush masses with little contour. The reeds are tapering calligraphic strokes. There are no cast shadows or directional light, and the ground is flat.
- *Conditions / counter-examples:* He presents this as an impression "judging from their art" (p. 59).
- *Our interpretation:* Flat-fill vector illustration is close to this mode: flat masses, line used sparingly on selected mass boundaries, ambient light.
- *Implication for vector software:* A "flat mass" style guide or preset: flat fills, no directional shading, sparse tapered strokes on selected boundaries. Tapered calligraphic width profiles serve the reed and grass strokes.
- *Do not algorithmize because:* not judgement-bound; it describes a style.
- *Disposition candidates:*
  - [agent-prompt-or-resource]: the style guide.
  - [improve-existing]: tapered brush profiles.
  - [design-principle]
- *Science caveat:* The "Eastern mind" generalisation is a 1913 cultural stereotype, not science. "Never used" overgeneralises East Asian painting. Use only the formal observation.

### SP-V-4: A structure of tone masses can replace an outline basis
- *Author says:* By Titian's time, with light and shade, linear and aerial perspective, and fused edges, painters had begun "to suspect the necessity of painting to a widely diffused focus" (p. 59). Velázquez was the first to found a picture "entirely on visual appearances" (p. 60). Quote: "a basis of objective outlines was discarded and replaced by a structure of tone masses" (p. 60)
- *Figures:* Plate XI.
- *Conditions / counter-examples:* See SP-V-6 on what this costs.
- *Our interpretation:* Tone masses alone can be the skeleton.
- *Implication for vector software:* The mass counterpart of the skeleton view: a value-structure view, posterised to three to five values and blurred, to test whether the tone masses hold the composition on their own. Agent check: does it read at three values, blurred?
- *Do not algorithmize because:* not judgement-bound.
- *Disposition candidates:*
  - [analysis-overlay]
  - [multi-step-workflow]
- *Science caveat:* "First picture … painted entirely from the visual … standpoint" is art-historical opinion.

### SP-V-5: One visual impression with a focal centre, and focus nowhere sharp (Las Meninas)
- *Author says:* Velázquez treated his painting room "entirely as one flat visual impression". "The focal attention is centred on the Infanta", with the figures on either side more or less out of focus and those far right "quite blurred" (p. 60). Quote: "The focus is nowhere sharp, as this would disturb the contemplation of the large visual impression." (p. 60)
- *Figures:* **Plate XI.** The Infanta is the lightest, most contrasted figure, set against a dark upper half. Speed warns that the reproduction "does not show these subtleties" (p. 60), and the plate shows no visible differences of focus.
- *Conditions / counter-examples:* His own caveat about the reproduction.
- *Our interpretation:* A hierarchy of focus: one centre, with sharpness and contrast falling off from it, and even the centre not hard.
- *Implication for vector software:*
  - The focus-falloff feature from SP-III-3, with an explicit focal point.
  - A verification overlay that plots edge sharpness and contrast against distance from the focal point.
  - An agent prompt to name a single focal centre.
- *Do not algorithmize because:* where focus sits, and how much falloff, is judgement. The parametric tool is fine.
- *Disposition candidates:*
  - [engine-feature]
  - [analysis-overlay]
  - [agent-prompt-or-resource]
- *Science caveat:* Peripheral vision does have lower acuity, but the periphery is not experienced as "blurred" in the way a painted blur is. Speed's account is artistic.

### SP-V-6: The cost of pure visual realism: no surface, no design, a cold impression
- *Author says:*
  - As a "tour de force", Las Meninas is "a painter's picture" that makes "but a cold impression on those not interested in the technique of painting" (p. 60).
  - Without the support of fine outline design and the accents that convey "a fine form stimulus to the mind", art "has lost much of its emotional significance" (pp. 60–61).
  - Quote: "All sense of design is lost. The picture has no surface; it is all atmosphere between the four edges of the frame" (p. 60)
- *Figures:* Plate XI.
- *Conditions / counter-examples:* He calls it "wonderful" all the same (p. 60).
- *Our interpretation:* The mass method still needs design (a pattern on the picture surface) and accents to keep its emotional force. Speed values both modes.
- *Implication for vector software:* Even mass workflows end with a design check, the big flat pattern of shapes, and an accents pass that places a few crisp, form-giving edges deliberately.
- *Do not algorithmize because:* emotional significance cannot be measured.
- *Disposition candidates:*
  - [design-principle]
  - [multi-step-workflow]: the accents pass.
- *Science caveat:* none.

### SP-V-7: The impressionist view: form as shapes of colour masses, at the price of dependence on the model
- *Author says:*
  - Seen as colour sensations, subjects "too ugly" for the old formula yield new harmonies.
  - Effects "too hopelessly complicated" for light-and-shade principles, "for instance, sunlight through trees in a wood", become "quite paintable, considered as an impression of various colour masses" (p. 61).
  - The method "necessitated an increased dependence on the model" (p. 61).
  - Monet's group studied "the anatomy of the light" and laid pure colours side by side "to be mixed as they came to the eye" (p. 63).
  - The old school said there was no drawing in these pictures. Pushed to a definite focus, they would have satisfied touch and solidity too (p. 63).
  - Quote: "Nothing is ugly when seen in a beautiful aspect of light, and aspect is with them everything." (p. 61)
- *Figures:* none.
- *Conditions / counter-examples:* He sets colour aside (p. 63).
- *Our interpretation:* Treating a complex effect as a set of colour masses makes it manageable, and that suits vector very well: a dapple is a flat shape.
- *Implication for vector software:*
  - Agent technique: render dappled light, reflections and the like as flat light and shadow shapes, not as shading on each object.
  - A tool to scatter light patches clipped to surfaces (speculative).
  - Broken-colour stipple belongs to the colour chapters, out of scope here.
- *Do not algorithmize because:* not judgement-bound.
- *Disposition candidates:*
  - [agent-prompt-or-resource]
  - [multi-step-workflow]
  - [engine-feature]: light-patch scatter (speculative).
- *Science caveat:* Optical mixing of adjacent dabs is partitive, that is averaging, which is not the same as additive mixing of lights. The story of the impressionist palette is simplified.

### SP-V-8: The direction of perception: visual to mental for the impressionists, mental to visual for the older school
- *Author says:*
  - The older view drew "a pictorial description … based on the mental idea", so the model "was not so necessary" (pp. 61–62).
  - "In the older point of view the visual impression is the result of the mental perception" (p. 62).
  - Impressionism therefore produced pictures of the actual visual world. The older view produced the works of imagination.
  - Imaginative impressionist attempts suffer from "the destruction of contours by scintillation, atmosphere, &c., and the loss of line rhythm" (p. 62).
  - Quote: "In the case of the Impressionist the mental perception is arrived at from the visual impression" (p. 62)
- *Figures:* none.
- *Conditions / counter-examples:* "there is much new material brought to light by this movement waiting to be used imaginatively" (p. 62).
- *Our interpretation:* An agent drawing from a text prompt with no reference is in the mental-to-visual position, where line construction suits. Drawing from a photograph is visual-to-mental, where mass study is available.
- *Implication for vector software:* A default rule, ours, for branching the MCP prompt. With no reference image, start from line construction (the skeleton of SP-IV-10). With a reference image, offer the mass-study workflow.
- *Do not algorithmize because:* it is a default, not a law.
- *Disposition candidates:*
  - [multi-step-workflow]
  - [agent-prompt-or-resource]
- *Science caveat:* none.

### SP-V-9: One sees with the mind. Keep only the significant, and keep one conception
- *Author says:*
  - Impressionism's implied assumption "that one sees with the eye alone is wrong". He quotes Goethe: "the eye sees in it what the eye brings means of seeing". "One sees with the mind" (p. 64).
  - The significant facts "must be sorted out from the mass of irrelevant matter and presented in a lively manner" (p. 64).
  - Painting the same spot again and again, the painter either repaints everything at each visit, small and hurried, or paints "a bit of today's impression alongside of yesterday's", which is "dull and lacking in oneness of conception" (pp. 64–65).
  - Quote: "The impression carried away from a scene that has moved us is not its complete visual aspect." (p. 64)
- *Figures:* none.
- *Conditions / counter-examples:* Working from nature is "well enough for studies" (p. 64).
- *Our interpretation:* Two rules: select, and hold one consistent conception across sessions.
- *Implication for vector software:*
  - For agents working over many turns: derive every edit from the one stated intent and global plan (light direction, value plan, palette), stored as document metadata. Do not patch regions locally with inconsistent "impressions".
  - Consistency analyses (heuristics): shading gradients share one light direction, and values match the plan.
- *Do not algorithmize because:* what is significant is judgement.
- *Disposition candidates:*
  - [multi-step-workflow]
  - [analysis-overlay]: consistency checks.
  - [agent-prompt-or-resource]
- *Science caveat:* none. It agrees with modern constructive, top-down accounts of perception.

### SP-V-10: Silhouette observation is the accurate way to observe visual shapes. It gains character and loses plastic form
- *Author says:*
  - Impressionism's effect on draughtsmanship is "drawing from the observation of the silhouette occupied by objects in the field of vision", the flat appearance on the retina, excluding the touch sense (p. 65).
  - Its result is "An increased truth to the character of appearances … with a corresponding loss of plastic form expression" (pp. 65–66).
  - Quote: "This is, of course, the only accurate way in which to observe visual shapes." (p. 65)
- *Figures:* Plate XIII (Degas).
- *Conditions / counter-examples:* The gain and the loss come together.
- *Our interpretation:* The silhouette is the 2D region an object occupies, and the accuracy of a visual shape is judged on it.
- *Implication for vector software:*
  - A silhouette view: every object, or every top-level group, filled solid dark on light, to check that shapes read and have character.
  - Group silhouettes computed as the union of the group's shapes (pathfinder unite).
  - In practice exercises, overlap scores against a reference segmentation.
- *Do not algorithmize because:* the trade between character and plastic form is a choice.
- *Disposition candidates:*
  - [analysis-overlay]
  - [practice-exercise]
  - [improve-existing]: a pathfinder-based silhouette.
- *Science caveat:* none.

### SP-V-11: Contours of felt forms (Michelangelo) against boundaries of visual masses (Degas)
- *Author says:*
  - **The Michelangelo.** "The silhouette is only the result of the overlapping of rich forms considered in the round". Every muscle and bone is "mentally realised as a concrete thing". Note "the sense of energy and movement conveyed by the swinging curves", with a forward reference to p. 162 (p. 66).
  - **The Degas.** "The silhouette is everywhere considered first", and the plastic sense is "arrived at from the accurate consideration of the mass shapes" (p. 66). It shows individual character: "those underfed little arms, and the hand holding the tired ankle". The contours "have been searched for those accents expressive of all this" (p. 66).
  - **The contrast.** Michelangelo gives "a type of physical strength and power", not an individual (p. 67).
  - Quote: "in the one case the lines are the contours of felt forms and in the other the boundaries of visual masses" (p. 66)
- *Figures:*
  - **Plate XII:** overlapping convex muscle forms, front contours running on at T-junctions while rear ones stop, hatching that follows the form, long swinging curves.
  - **Plate XIII:** one light tutu mass against a dark ground, thin limbs read as silhouette, and edges ranging from crisp to lost.
- *Conditions / counter-examples:* "They both contain lines" (p. 66). Both modes use line, with different meanings.
- *Our interpretation:* These are two strategies for building a figure in vector:
  - Additive: overlapping closed forms whose union is the silhouette (Michelangelo).
  - Silhouette-first: one shape, then interior masses (Degas).
- *Implication for vector software:*
  - **"Form-assembly" workflow.**
    - Build from overlapping convex shapes (ellipses, bulges) stacked in depth.
    - An overlap-aware "contour from stacked forms" command: the front form's line continues across a junction and the rear form's line stops there.
    - A curvature plot that separates long swinging arcs from choppy segments.
  - **"Silhouette-first" workflow.**
    - One silhouette shape.
    - Refine the accents of character along the contour.
    - Cut the interior value shapes.
- *Do not algorithmize because:* the choice of strategy, and where the character accents go, are judgement.
- *Disposition candidates:*
  - [multi-step-workflow]: both workflows.
  - [engine-feature]: contour from stacked forms.
  - [analysis-overlay]: curvature continuity.
  - [practice-exercise]
- *Science caveat:* none.

### SP-V-12: A flat pattern with lost-and-found edges. Searching for character prevents mechanical results
- *Author says:*
  - The rhythm differs too: the Michelangelo has a line rhythm, the Degas a flat pattern (p. 67), with a forward reference to "pages 192 et seq., variety of edges" (p. 67).
  - "This feeling for rhythm and the sympathetic searching for and emphasis of those points expressive of character" keeps the Degas from being "the mechanical performance" that scientific visual accuracy might have made it (p. 67).
  - That fate befell "many of the drawings of Degas's followers who unintelligently copy his method" (p. 67).
  - Quote: "a consideration of the flat pattern of shapes or masses with a play of lost-and-foundness on the edges" (p. 67)
- *Figures:* Plate XIII.
- *Conditions / counter-examples:* A method copied without its motive becomes mechanical.
- *Our interpretation:* This is a direct warning against imitating a style's surface. Lost-and-found edges are a means, not a look to be applied.
- *Implication for vector software:*
  - The edge tooling from SP-IV-1, plus a flat-pattern view (silhouette and value).
  - Agent guidance: "in the style of X" must carry that style's reasons (rhythm, accents of character), not only its marks.
  - Analysis can report edge-variety statistics, such as the share of boundary length that is lost against found, but not whether the variety falls in the right places.
- *Do not algorithmize because:* the "sympathetic searching" for character is explicitly the non-mechanical part.
- *Disposition candidates:*
  - [analysis-overlay]
  - [engine-feature]: lost-and-found edges.
  - [agent-prompt-or-resource]
  - [not-automatable]: placing the character accents.
- *Science caveat:* none.

---

## Cross-cutting notes

### Recurring themes (chapters I–V)
1. **Two points of view, line and mass.** Announced on p. x, grounded in vision on pp. 47–49, and developed in Ch. IV and V. In this range they are always compared, never ranked absolutely. Each mode's losses are stated: line loses visual truth; mass loses design and accents (pp. 60–61, 65–66).
2. **Selection and suppression.** Pp. 23–24, 32, 64. It is what separates art from a "catalogue" (p. 28) and from copying (p. 30). It is never reduced to a rule.
3. **Two kinds of accuracy.** Mechanical or academic accuracy is to be trained coldly (pp. 34–35, 37). Artistic accuracy is judged against intent (p. 35). A "rough physical standard" bounds the second (p. 36).
4. **Design, or skeleton, before detail.** Detail obscures design (p. 54). Finish is not detail (p. 46). Study early work (p. 55). The "simple line scheme" (p. 57). The "structure of tone masses" (p. 60).
5. **Primitive strength plus refinement** (pp. 55–57). Refinements are "of the very slightest" (p. 56).
6. **Edges and focus.** "Many varieties of edges" (p. 39). Boundaries that merge and are found again (p. 50). Fused edges (p. 53). Diffused focus (pp. 59–60). Lost-and-foundness (p. 67). This is the most vector-relevant gap: vector edges are hard by default.
7. **Touch against vision.** The explanation he uses throughout pp. 40–58. Its science is dated (SP-III-4), but the practical distinction between schema and appearance holds.
8. **Feeling is outside teaching and mechanism.** Pp. 17, 20, 24, 30, 31–32.

### Terminology as defined or used in this range
- **Drawing:** "the expression of form upon a plane surface" (p. 31).
- **Form / Colour:** the two divisions of technical study; this book covers Form (p. 18).
- **Art:** working definition "the Rhythmic expression of Feeling" (p. 27).
- **Rhythm:** "that ordering of the materials of art … so as to bring them into relationship with our innate sense of harmony" (p. 27). Harmony, Beauty and Rhythm are "different aspects of the same thing" (p. 21). Line rhythm is deferred (p. 51); swinging curves are forward-referenced to p. 162 (book.txt about line 5196); variety of edges to pp. 192 et seq. (about line 6006).
- **Line Drawing:** the method from mental perception (touch together with vision) whose instinct is "to put an outline round objects" (p. 48).
- **Mass Drawing:** the method from visual perception: flat retinal appearance, felt shapes forgotten (pp. 48, 58).
- **Mass:** used in two senses. (a) Regions of colour or tone in the visual field: "masses of colour" (p. 39), "a few simple masses" (pp. 58–59), "tone masses" (p. 60). (b) Bulk or weight as an expressive quality (p. 33). See Open questions.
- **Tone:** used without definition: "structure of tone masses" (p. 60), "tone, colour, form, light and shade" (p. 60), "the beauty of tone and colour relations" (p. 64). The List of Diagrams titles "Mass or tone rhythm" (Diagrams XXIV–XXV) suggest that mass and tone are near-synonyms later.
- **Values:** *not used* in this range in the tonal sense. Its body-text uses all mean "worth". Tonal "values" appear later (for example "scale of values", book.txt about line 6420, outside range).
- **Outline / contour:** in appearance, outlines are only "boundaries of masses" (p. 50). In line drawing, contours are "of felt forms" (p. 66).
- **Silhouette:** "the silhouette occupied by objects in the field of vision" (p. 65). Plate XIII's caption: "the spaces occupied by the different masses in the field of vision".
- **Lost-and-foundness:** the play along edges in a mass drawing (p. 67).
- **Finish:** "the completeness with which the emotional idea … has been realised", not detail (p. 46).
- **Focus / diffused focus:** pp. 40, 59–60.
- **Plastic form:** the felt solidity of form (pp. 65–66).
- **Chiaroscuro:** = light and shade (p. 53).
- **Relievo:** Leonardo's "standing out" (p. 52).
- **Accents:** the points that carry form or character (pp. 61, 66, 67).
- **Impressionist vision:** "this subjective way of considering appearances" (p. 61).
- **Academic:** used for exact, cold school studies (pp. 34–37). Defined in Ch. VI, out of range.

### Where he warns against mechanical rules or mechanical work
- P. v: no short cuts or "wrinkles".
- P. 17: intuitions are shy.
- P. 20: mechanical instruments miss the facts of feeling.
- P. 26: painting is not "the production … of more or less perfect representations".
- P. 30: the "copying theory".
- P. 32: selection is "too subtle to be tabulated" and escapes analysis.
- P. 56: Michelangelo's followers copied his type and lost the strength.
- P. 67: Degas's followers "unintelligently copy his method" and become mechanical.

The one place he *asks for* mechanical work is academic training (pp. 34–35, 37), and he keeps it explicitly apart from artistic work. For tooling, this means metrics and automatic checks are appropriate for practice drills and physical plausibility, while intent, selection, emphasis and character stay with the artist or agent.

### Speed's concepts mapped to vector primitives (our interpretation)
| Speed | Vector analogue | Gap |
|---|---|---|
| Line (contour of felt form) | stroke, open or closed path | uniform width; lost-and-found needs width and opacity profiles with gaps |
| Mass (shape in the visual field) | filled closed shape with no stroke | none |
| Gradation | gradient, gradient mesh | none |
| Edge quality (hard, soft, lost) | none by default (all edges hard) | per-segment feather or soft edge |
| Silhouette | union of a group's shapes | needs a view |
| Middle-tone ground (Plates I and VIII) | artboard background rectangle | workflow convention only |
| Squaring grid (Plates IV and VII, not discussed in this range) | `view.grid` | already exists |

### Cross-references for the other ledgers
- The method of Plate I is presumably in Ch. IX (book.txt about line 3874, "monochrome studies painted into a middle tone"). Plates XXIII–XXIV ("stages of the painting from a cast") and XXVI–XXIX ("stages of the same study") look like related step series.
- Diagrams II–XXVIII are listed by title only in this range. Diagram III is the frame-and-threads device for seeing appearances as flat, the practical form of SP-III-2 and SP-V-1.
- "Half closing the eyes" (about line 6420) is a later technique. It must not be attributed to chapters I–V.

---

## Open questions

1. **What is Plate I's procedure?** It is the frontispiece and is central, but no text in this range describes it. The stage descriptions in SP-V-2 come from the image alone. Ch. IX should confirm, for example, whether stage 2's middle grey is a prepared ground or the first lay-in. The List of Plates ("study from the life") and the caption ("monochrome painting … mass drawing with the brush") are consistent.
2. **Who made the unattributed plates?** Plates I, III, IV, VII and VIII have no artist's name, which suggests Speed. The text in this range does not say so.
3. **Which sense of "mass"?** Bulk (p. 33) or a tone region (pp. 39, 58–60)? Agent guidance should name both senses to avoid confusion.
4. **How well do line and mass map to strokes and fills?** Mass drawing still contains lines, as "boundaries of visual masses" (p. 66). A strict "no strokes in mass mode" lint may be too strict. Perhaps strokes should be allowed in mass mode only on mass boundaries.
5. **What are the grids on Plates IV and VII?** They look like squaring for transfer or enlargement, and Plate VI's circular guides look compositional. Neither is discussed in this range, so both are deferred to the practical chapters.
6. **Should eye-direction claims be rules or heuristics?** For example "a line directs the eye along its course" (pp. 50–51). Given the science caveat, we recommend heuristics with no automatic scoring.
7. **How should the proportion chapters treat the "innate sense of harmony"?** Chapter I's "innate sense of harmony" (pp. 21, 27) may be made operational in the proportion and rhythm chapters. Coordinate so it does not harden into fixed ratios, which would conflict with SP-I-6 and the warnings above.
8. **How should a detail-density overlay treat intended detail?** Speed allows that detail can be the intent (the Pre-Raphaelites, p. 46), so the overlay should report and never penalise.
9. **Text artefacts.**
   - Line 2000 reads "more [influenced": a stray "[" that is not an anchor, probably left by extraction.
   - The List of Plates repeats numbers ("XXIII … (1)", "XXIII … (2)", "XXIV … (3)", "XXIV … (4)").
   - The title page says "With 93 Illustrations & Diagrams".
10. **How should Plates XII and XIII be cited?** The text says the drawings are reproduced "On pages 66 and 67". In the ebook both plates sit between the `[#Page_66]` and `[#Page_67]` anchors, so we cite the text pages, not the plates' printed pages.
