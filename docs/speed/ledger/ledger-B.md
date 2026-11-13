# Ledger B: Speed, *The Practice and Science of Drawing* (1913), chapters VI–IX (+ X)

Researcher: Book Researcher, part B of 4. Source: `book.txt` (EPUB plain-text extraction of Project Gutenberg #14264) and the XHTML/images in `epub/OEBPS/`.

Scope notes:
- The assigned chapters are VI–IX. The assigned **line range 2300–4448 also contains chapter X "Rhythm" (lines 4131–4447)**: `## X` is at line 4131, and line 4448 is `## XI`. To avoid leaving X uncovered, I read it fully and ledgered it as a supplementary section (`SP-X-*`). If part C also covers X, deduplicate against theirs.
- Line 2395 ("VII / THE STUDY OF DRAWING") is the `<title>` of XHTML file `14264-h-1`, **not** a chapter heading. Lines 2398–2655 (the "dither" passage, Plates XIV–XV) are still chapter VI (pp. 71–79). I checked this against the XHTML.
- Page numbers ("p. N") come only from `[#Page_N]` anchors. I checked each quote against the text with a script and took its page from the nearest preceding anchor. The printed pages 82, 95 and 107 have no anchor (they are plate or diagram pages), so nothing is cited to them. Plate captions are cited as "Plate N caption".
- In the source, `div.sidenote` marks the author's subheadings and `<b>` marks his own emphasis. The plain text drops the bold, so I extracted it from the XHTML and note it below as "(bold in source)".
- Field separation: *Author says* is paraphrase plus a quote. *Our interpretation* and *Implication* are ours. Where I say a repo feature "exists", I only saw its name in `crates/` (for example width profiles, feather, roughen, scribble, guides, perspective grid, `object.path.simplify`, the trace crate, posterize, grayscale, `object.blend.make`, pathfinder, the three-d crate). Verify each one before planning.

## Reading log

| Section | Lines in book.txt | XHTML file | Page anchors covered | Read? | Figures viewed (larger `7373817595610384519_*` copies) |
|---|---|---|---|---|---|
| VI The Academic and Conventional (opening: schools, prizes) | 2295–2391 | `8481025652923775349_14264-h-0.htm.xhtml` | 68, 69, 70, 71 | yes | none |
| VI (cont.): dither, Ingres, conventions, originality, materials | 2392–2655 | `8481025652923775349_14264-h-1.htm.xhtml` | 71 (cont.)–79 | yes | Plate XIV, Plate XV |
| VII The Study of Drawing | 2656–2767 | `…14264-h-1` | 80, 81, 83 (p. 82 has no anchor) | yes | Diagram II, Plate XVI |
| VIII Line Drawing: Practical, from "Observing Solids as a Flat copy" to "Blocking in your Drawing" | 2768–2954 | `…14264-h-1` | 84–90 | yes | Diagram III, Diagram IV, Plate XVII |
| VIII: "How to observe the Shape of Curves", "The Drawing proper", "…Background…", "Boundaries a series of Overlappings" | 2955–3070 | `…14264-h-1` | 90–93 | yes | Plate XVIII |
| VIII: "Shading" (cone, light and shade, line methods, direction, hair) | 3071–3362 | `…14264-h-1` | 93–103 (p. 95 has no anchor) | yes | Diagram V, Plate XIX, Plate XX, Plate XXI |
| VIII: feeding appreciation, Puvis, eyes, head guide lines | 3363–3545 | `…14264-h-1` | 103–109 (p. 107 has no anchor) | yes | Plate XXII, Diagram VI; out-of-range cross-reference Diagram XII |
| IX Mass Drawing: Practical: three tones, exercise, setting out, scumble, lights and darks, one stroke | 3541–3730 | `…14264-h-1` | 110–115 | yes | Plate XXIII, Plate XXIV |
| IX: brushes and touches, hatching, corrections, black glass, setting out, values, planes, anatomy | 3731–3996 | `…14264-h-1` | 115–123 | yes | Plate XXV |
| IX: life-study plates, brush direction, toned paper | 3997–4125 | `…14264-h-1` | 123–126 | yes | Plates XXVI, XXVII, XXVIII, XXIX; out-of-range cross-references Plate XLIX, Plate LIV |
| X Rhythm (supplementary: inside the line range, outside the chapter list) | 4126–4447 | `…14264-h-1` | 127–136 | yes | Plate XXX |

All lines from 2290 to 4452 were read in order, in chunks of about 250 lines. I viewed 25 figures: Diagrams II–VI; Plates XIV–XXX; and the cross-referenced Diagram XII and Plates XLIX and LIV.

---

## VI The Academic and Conventional

**Locator**: Starts in `14264-h-0` at `#CHAPTER_VI` (lines 2295–2391) and continues in `14264-h-1` (lines 2392–2655). First anchor `[#Page_68]`, last `[#Page_79]`. Sub-anchors: `#CHAPTER_VI`, `#pgepubid00074`–`#pgepubid00086`, `#plate14`, `#plate15`. The chapter has no sidenotes.

### Principles

**SP-VI-1: Accuracy studies are a means, not the end; keep two kinds of study**
- *Author says*: Art schools train students to copy mechanically what is set before them (antique casts, still life, a model sitting "as still and lifeless as he can"). Accurate studies are "very necessary" to train eye and hand, because natural appearances and knowledge of form are the student's later means of expression. When prizes reward only them, they become the goal. He proposes prizes for two kinds of work. Academic drawings should be as accurate "in a mechanical way" as industry can make them. In artistic drawings "the student should be encouraged to follow his bent, striving for the expression of any qualities that delight him" (p. 70). Quote: "they do tend to become the end instead of the means." (p. 70)
- *Figures*: none in this passage. Plate XIV (next file) is offered as an "unacademic" class drawing.
- *Conditions / counter-examples*: Feeling "will creep in" even to accuracy studies if the student has the instinct (p. 70). Schools that encourage only artistic qualities "neglect the academic side" and leave students "poorly equipped" (p. 70). Both kinds of study are needed. Expression is too often left "until after the school training is done" (p. 70).
- *Our interpretation*: There are two modes with different success criteria. Fidelity can be measured. Expression is not judged by likeness.
- *Implication for vector software*: An agent or exercise should say which mode it is working in. In study/fidelity mode, compare against a reference with overlays and error figures. In expressive mode, do not optimise for likeness to the reference. Exercises can come in pairs: an accuracy study and a free study of the same subject.
- *Do not algorithmize because*: the expressive mode's success ("a fine thing, probably only remotely like the model to the average eye", p. 70) has no metric. Scoring it by likeness recreates the prize problem he describes.
- *Disposition candidates*: design-principle (separate the modes); practice-exercise (paired accuracy and free studies); agent-prompt-or-resource (declare the mode before drawing).
- *Science caveat*: none. This is pedagogical opinion.

**SP-VI-2: "Dither", the least play on accuracy that still allows life**
- *Author says*: He uses an engine as a simile. Perfectly fitted parts cannot move; parts fitted too loosely give "a poor rickety thing". Vital drawing needs a small play on mechanical accuracy, which he names "dither". The better the work, the smaller the play. Too great a variation "under the influence of feeling" gives caricature. In a beautiful drawing "the variations… are so subtle as often to defy detection" (p. 73). A photograph has none of this play, except "blurred, vague" artistic photography. Machine-made goods are lifeless for the same reason. "Where the recording instrument is a live individual, there is no mechanical standard of accuracy possible" (p. 72). He calls dither "that play on mechanical accuracy, existing in all vital art" (p. 72); he says this vital quality "has not yet received much attention in art training" (bold in source). Quote: "There must be the smallest amount of play that will allow of its working." (p. 71)
- *Figures*: Plate XIV (Ernest Cole, red chalk) shows an old bearded man's head. Tone is concentrated around the eye socket, nose and cheek. Hair and beard are long, loose, unequal strokes that overlap. The edges are left open and the drawing is unfinished. Plate XV (Ingres, pencil) shows a young man with a gun and a dog. The head is closely modelled; the coat is in sparse, sure contour lines with almost no shading. The text says Ingres's lines are "instinct with life" yet easily taken for "merely accurate".
- *Conditions / counter-examples*: The play must be the minimum compatible with life; excess is caricature. It comes from feeling, not from adding variation for its own sake. Even Ingres sometimes lost it (SP-VI-3).
- *Our interpretation*: Dither is not noise. It is a small, motivated departure from measured truth: an emphasis, a slightly straighter or fuller line where the artist feels the form.
- *Implication for vector software*: Do not ship a random jitter or roughen effect and call it this. Things that help:
  - Freehand, pressure and width tools that keep the user's own variation.
  - Simplify and trace that do not erase small intended deviations (SP-VI-3).
  - Agent guidance that exact primitives (perfect circles, uniform strokes, equal spacing) read as mechanical, and that any departure should be deliberate and small.
- *Do not algorithmize because*: he ties the variation to feeling and to the particular subject. Random perturbation is the opposite: variety without motive. How much is right is judged, not computed.
- *Disposition candidates*: design-principle; agent-prompt-or-resource; not-automatable (the deviation itself); improve-existing (pressure and width input should keep the user's variation).
- *Science caveat*: That photography "fails" and that machine-made goods have "impoverished the world" are 1913 aesthetic opinions, not findings.

**SP-VI-3: Copying a study "in cold blood" loses vitality**
- *Author says*: Without nature in front of him it is hard to keep the dither when transferring a drawing to canvas. Some of Ingres's pictures "have in some cases become academic and lifeless" this way. "This is the great difficulty of working from studies." Quote: "it is so easy to lose those little points in your drawing that make for vitality of expression" (p. 73)
- *Figures*: Plate XV (Ingres).
- *Conditions / counter-examples*: It concerns working from studies rather than from life. He gives no exceptions.
- *Our interpretation*: Every re-drawing step (tracing, cleanup, enlargement, redrawing) tends to regularise the small expressive points.
- *Implication for vector software*: This bears directly on vector cleanup: image trace, simplify, smooth, and agents redrawing a user's sketch. Options:
  - A "keep character" option for simplify and trace that keeps inflections, corners and apex points (SP-VIII-6) instead of only minimising node count.
  - An overlay showing where a cleaned path departs from its source stroke.
  - Agent guidance to keep the source visible and compare after cleanup.
- *Do not algorithmize because*: which small points matter is a judgement. The tool can keep them and show them; the artist chooses.
- *Disposition candidates*: improve-existing (simplify and trace options); analysis-overlay (source versus cleaned deviation); agent-prompt-or-resource.
- *Science caveat*: none.

**SP-VI-4: Only the academic can be taught; it is practice, like a musician's exercises**
- *Author says*: A school can teach accuracy. Vitality must be appreciated by the student himself, and may appear "without his being aware that he is doing aught but honestly copying". "All the master can do is to stand by and encourage." In summary, academic drawing is "as necessary to the painter as the practising of exercises is to the musician" (p. 78), but "the vital matter of art is not in all this necessary training". Quote: "it is only the academic that can be taught." (p. 73)
- *Figures*: none.
- *Conditions / counter-examples*: The danger is that absorbing academic study crowds out the instinctive qualities (pp. 78–79).
- *Our interpretation*: The teachable part is observation and execution, and that is where software help is legitimate.
- *Implication for vector software*: Measurement, overlays, exercises and critique bots should aim at the teachable part (proportion, placement, values, edges) and leave the rest to the person. An agent critic can say "this proportion is 12% short"; it should not grade "life".
- *Do not algorithmize because*: the author puts vitality outside what can be taught.
- *Disposition candidates*: design-principle (scope of automated critique); practice-exercise.
- *Science caveat*: none.

**SP-VI-5: "Academic" means dead, not thorough or conventional**
- *Author says*: A drawing is not academic because it is thorough or in a conventional style, and it is not good because it is unconventional. "The test is whether it has life and conveys genuine feeling." (p. 74) Quote: "A drawing is not necessarily academic because it is thorough, but only because it is dead." (p. 73)
- *Figures*: Plate XIV (unfinished student work) and Plate XV (highly finished Ingres) are both presented as living.
- *Conditions / counter-examples*: none.
- *Our interpretation*: Finish and style are independent of the quality he cares about.
- *Implication for vector software*: Critique rubrics and agent self-review must not equate detail or polish with "academic", or roughness with "alive". This is guidance text.
- *Do not algorithmize because*: "life" is the criterion and has no measure.
- *Disposition candidates*: design-principle; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-VI-6: All art is convention; past a point, illusion kills life; select what translates**
- *Author says*: Art cannot escape conventions: "naturalism is just as much a convention as any of the other isms" (p. 74). Waxworks (Madame Tussaud's) and tableaux vivants are as real as can be and lifeless. "After a certain point," "the nearer your picture approaches the actual illusion of natural appearance, the further you are from the expression of life." (p. 74) The artist must seek the things in an appearance that convey vital expression and "are capable of being translated into the medium he is working with", and make his "painted symbols" to suit (p. 75). In a good picture the figures are often felt to move. Quote: "naturalism is just as much a convention as any of the other isms" (p. 74)
- *Figures*: none.
- *Conditions / counter-examples*: "After a certain point" means this is not an argument against observation.
- *Our interpretation*: Choose a convention, that is a degree of abstraction, that suits the subject and the medium. Select rather than copy everything.
- *Implication for vector software*: An agent should choose its convention up front: contour only; flat shapes in three values; line with light hatching. It should then select the features that convention can express, and not pile up gradients or mesh detail chasing photographic illusion. A guidance resource could list vector conventions and what each can express.
- *Do not algorithmize because*: which features "convey vital expression" is the artist's choice.
- *Disposition candidates*: agent-prompt-or-resource; design-principle.
- *Science caveat*: The claim that illusion and life are inversely related is aesthetic opinion. Its likeness to the later "uncanny valley" idea is our observation, not his claim.

**SP-VI-7: A borrowed manner is dead; originality comes from sincerity**
- *Author says*: A convention is dead only when a painter "consciously chooses a manner not his own, which he does not comprehend and is incapable of firing with his own personality" (p. 75). Each age needs its own conventions. The artist finds his own by following his bent and choosing what appeals to him, neither flouting nor accepting tradition "on principle". Repeated successes fail. "Every beautiful work of art is a new creation" (p. 77). Painting like Titian today is unlike Titian's spirit, and "self-conscious seeking after peculiarity only stops the natural evolution and produces abortions" (p. 77). An artist without a strong character may do better in "some well-tried manner" (p. 76). Quote (bold in source): "Originality is more concerned with sincerity than with peculiarity." (p. 76)
- *Figures*: none.
- *Conditions / counter-examples*: Genuine new points of view are not discouraged (p. 76).
- *Our interpretation*: Style is a consequence, not a target.
- *Implication for vector software*: Asked for "the style of X", an agent should understand and state what that style's devices are for, not just apply surface traits. Asked to be "original", it should follow the user's stated preferences rather than novelty for its own sake. It should not clone its own earlier successful layout for a new brief.
- *Do not algorithmize because*: sincerity and personal liking cannot be computed, and style presets cannot supply them.
- *Disposition candidates*: agent-prompt-or-resource; not-automatable.
- *Science caveat*: none.

**SP-VI-8: The material sets the convention; express only what the tool can**
- *Author says*: Each material imposes conventions. Study its expressive qualities and select the facts of nature that suit it. Sculptors render hair in stone only by its mass forms, never as an illusion. Bronze must look like bronze: he criticises the Trafalgar Square lions as "modelled in dough" and praises Alfred Stevens's little British Museum lion. Great painters never push illusion so far that you forget it is paint. In drawing, the great artists "always created a convention within which unlimited expression has been possible" (p. 78). Quote: "the great artists have always confined themselves to the qualities in nature that the tool they were drawing with was capable of expressing" (p. 78)
- *Figures*: none.
- *Conditions / counter-examples*: none; it is stated generally.
- *Our interpretation*: Vector is a medium. Its native qualities are exact edges, flat and gradient fills, Bézier curves, stroke weight and width profiles, layering and transparency.
- *Implication for vector software*: Defaults and agent guidance should express form with what vector does well: silhouette, clear shape edges, a few flat values, line weight. Imitating raster textures is not the goal. When softness is needed, use vector means (gradient, blend, opacity) deliberately. This also sets how we adapt Speed's chalk and paint advice in VIII and IX: translate the principle, not the material's physics.
- *Do not algorithmize because*: choosing which facts suit the medium is the artist's selection. Guidance can list the medium's means.
- *Disposition candidates*: design-principle; agent-prompt-or-resource.
- *Science caveat*: none. The remark on the lions is taste.

---

## VII The Study of Drawing

**Locator**: `14264-h-1`, lines 2656–2767. Anchors `[#Page_80]`, `[#Page_81]`, `[#Page_83]`; there is no `[#Page_82]` (that printed page holds Diagram II and Plate XVI). Sub-anchors: `#CHAPTER_VII`, `#pgepubid00087`–`#pgepubid00090`, `#diagram02`, `#plate16`. No sidenotes.

### Principles

**SP-VII-1: Two extreme approaches, studied separately: outline and mass**
- *Author says*: Form can be approached from outline, "directly related to the mental idea of form with its touch association", or from mass, "connected directly with the visual picture on the retina". Infinitely many styles lie between. Outline trains accurate observation, the expressive value of a line and definite statement, leading by degrees to full light and shade. Mass drawing with paint teaches tone values and form "by means of planes". The student will thus "learn accurately to observe and portray the tone masses (their shapes and values) to which all visual appearances can be reduced" (p. 81). Quote: "it is advisable for the student to study both separately, for there are different things to be learnt" (p. 80)
- *Figures*: none. The two paths are shown in VIII and IX.
- *Conditions / counter-examples*: The two paths meet at "the full realisation of form" (p. 81).
- *Our interpretation*: Line and mass are different ways of seeing that train different skills. Mixing them early confuses.
- *Implication for vector software*: Vector has both natively: strokes are outline, fills are mass. Provide and use two pure modes: contour-only (no fills) and mass-only (filled value shapes, no strokes). An agent can do a line pass and a mass pass on separate layers.
- *Do not algorithmize because*: n/a; this is structural.
- *Disposition candidates*: design-principle; practice-exercise (pure line study, pure mass study); multi-step-workflow.
- *Science caveat*: The outline/touch versus mass/retina split is the book's 1913 theory from its earlier chapters (outline as a tactile idea). It is not established vision science, and "tone masses" are an artist's simplification of the retinal image.

**SP-VII-2: Either approach alone is incomplete**
- *Author says*: Outline-only "sculptor's drawings" lack "the tone and atmosphere that always envelop form in nature" and leave the student lost with a brush. Mass-only study neglects the subtleties of contour and construction. Quote: "he will not understand the mental form stimulus that the direction and swing of a brush stroke can give." (p. 81) These things are best learnt in line work.
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: Each mode trains the other's blind spots.
- *Implication for vector software*: An agent self-check can catch both failures. An all-outline drawing has no value structure; an all-mass drawing may have sloppy contours. A critique pass can check value grouping and contour accuracy separately.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: agent-prompt-or-resource; analysis-overlay (value-structure check plus contour check).
- *Science caveat*: "atmosphere that always envelop[s] form" is an observational generalisation; acceptable as such.

**SP-VII-3: Order of study: start at the extremes, at the same time, and let them converge**
- *Author says*: Begin with outline studies of simple casts or models, gradually add light and shade, and later draw from life. At the same time begin simple mass drawing in paint, "criticised solely from the point of view of tone values" (p. 83). He says no school does this, as far as he knows. Each study helps the other, and the two paths meet. Otherwise the result is "confusion and the 'muddling through' method" (p. 83). Quote: "beginning with their most extreme positions, that is, bare outline on the one side and on the other side tone masses" (p. 83)
- *Figures*: none.
- *Conditions / counter-examples*: In the first instance, tone masses are judged "for their accuracy of values only".
- *Our interpretation*: A two-track curriculum, each track with a single criterion.
- *Implication for vector software*: Practice exercises in two tracks with separate criteria. The line track is judged on placement and contour. The mass track is judged only on value shapes and values, not edges or detail. When calibrating, an agent should evaluate one criterion at a time.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: practice-exercise; multi-step-workflow.
- *Science caveat*: none.

**SP-VII-4: The failure mode: paint used like chalk, fear of putting down a mass**
- *Author says*: Without elementary tone study, the beginner uses paint "as much like chalk on paper as possible". Having "no knowledge of reducing appearances to a structure of tone masses or planes", he is afraid to commit masses. Quote: "timid of losing his outlines, he fears to put down a mass" (p. 83)
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: Reliance on outlines, with interiors as an afterthought.
- *Implication for vector software*: This is a common LLM-agent failure in SVG: every object gets an outline stroke and one flat fill. Guidance: build value masses first (light, half-tone and shadow shapes), not outlined objects; a mass-only mode makes this explicit. An analysis can report the ratio of outlined single-fill objects to tone shapes.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: agent-prompt-or-resource; analysis-overlay.
- *Science caveat*: none.

**SP-VII-5: Flatnesses give strength to full forms (Plate XVI and Diagram II)**
- *Author says*: This appears in captions only; the body of VII does not discuss it. Plate XVI caption: "A splendid example of Rubens' love of rich, full forms. Compare with the diagram opposite, and note the flatnesses that give strength to the forms." Diagram II caption: "Showing where squarenesses may be looked for in the drawing on the opposite page." The idea is developed in VIII (curves judged against straight lines) and IX (planes, SP-IX-12). Quote: "note the flatnesses that give strength to the forms." (Plate XVI caption; the script places it on p. 83, because p. 82 has no anchor)
- *Figures*: Plate XVI is a Rubens chalk study of a powerful male back, seen from behind and turning, with drapery at the hips. Diagram II reduces the same back to an outline of short, mostly straight strokes meeting at slight corners (shoulder, back of the arm, flank, hip), so the round contours read as a series of flats.
- *Conditions / counter-examples*: none given here.
- *Our interpretation*: Even rich, curved forms are made of flatter runs meeting at subtle turns, and that gives them strength. Even curvature looks weak.
- *Implication for vector software*: Bézier contours should vary their curvature, with flat runs and tighter turns, instead of uniform arcs. An analysis overlay could plot curvature along a path (a curvature comb) and flag runs of constant (circular) curvature. Agent guidance: build contours as near-straight runs joined by turns.
- *Do not algorithmize because*: where the flats fall depends on the form's structure. The tool can show them, not decide them.
- *Disposition candidates*: analysis-overlay (curvature plot); agent-prompt-or-resource; design-principle.
- *Science caveat*: none (aesthetic).

---

## VIII Line Drawing: Practical

**Locator**: `14264-h-1`, lines 2768–3545. Anchors `[#Page_84]`–`[#Page_109]`; pp. 95 (Diagram V) and 107 (Diagram VI) have none. Sub-anchors: `#CHAPTER_VIII`, `#pgepubid00091`–`#pgepubid00115`, `#diagram03`, `#diagram04`, `#plate17`, `#plate18`, `#diagram05`, `#plate19`, `#plate20`, `#plate21`, `#plate22`, `#diagram06`. Sidenotes, with line numbers:
- "Observing Solids as a Flat copy" (2787)
- "Fixing Positions of Salient Points" (2828)
- "Blocking in your Drawing" (2933)
- "How to observe the Shape of Curves" (2955)
- "The Drawing proper" (2988)
- "In Blocking-in observe Shape of the Background as much as the Object" (3043)
- "Boundaries a series of Overlappings" (3063)
- "Shading" (3071)

### Principles

**SP-VIII-1: See the subject as flat: close one eye, use a gridded frame**
- *Author says*: A drawing is made on a flat surface, so appearances must first be reduced to one. The habit of perceiving solidity makes this hard. We have two eyes, so two viewpoints; close one. The frame:
  - Stiff card about 12 × 9 in, with a 7 × 5 in opening in the centre.
  - Inches marked on all sides of the opening.
  - Black thread taken through each mark and across the opening, in both directions, fixed with sealing-wax and kept taut. The result is a trellis of 1-inch squares.
  - Held between the eye and the object, "in a perfectly vertical position", sides vertical and horizontal, one eye closed.

  It helps the student see in two dimensions ("the first technical difficulty") and judge proportions, "the squares of equal size giving one a unit of measurement by which all parts can be scaled" (p. 86). Quote: "appearances must be reduced to terms of a flat surface before they can be expressed on paper." (p. 84)
- *Figures*: Diagram III is a rectangle labelled "7 inches" by "5 inches", divided by 6 vertical and 4 horizontal threads into 1-inch squares. Dotted lines outside the opening show the thread looping from hole to hole, from a start point A to an end point B on the bottom edge, with a B also on the left side.
- *Conditions / counter-examples*: He calls it "the simplest and most mechanical way". It is reused for mass drawing (p. 120).
- *Our interpretation*: A square grid in the picture plane, seen from a single fixed viewpoint.
- *Implication for vector software*: A reference-grid overlay on placed images, with a matching grid on the artboard (same cell count, its own scale), lockable and non-printing. Through MCP an agent can then say "landmark X is in cell (3, 2), 40% across". Perspective grids and guides exist in the repo; check whether a plain proportional grid pair exists.
- *Do not algorithmize because*: n/a; this is a mechanical aid.
- *Disposition candidates*: engine-feature or improve-existing (proportional grid pair); analysis-overlay; practice-exercise (grid-transfer drawing).
- *Science caveat*: Closing one eye removes binocular disparity, which is geometrically sound. The "habit of perceiving solidity" resembles what is now called shape constancy, but here it is an observation, not a measured result.

**SP-VIII-2: Fix salient points first, against a vertical and horizontals**
- *Author says*: The first step in setting out a drawing is "the fixing of salient points, and getting their relative Positions" (p. 86). Assume a vertical through an important point A. Place B, C, D and E by the heights and lengths of horizontal lines from them to that vertical. Get the vertical from a plumb line held at arm's length (one eye closed) so that it covers A, or from a knitting needle held vertically. "The advantage of the needle is that comparative measurements can be taken with it." (p. 86) Quote: "the fixing of salient points, and getting their relative Positions." (p. 86)
- *Figures*: Diagram IV, Fig. Z: a vertical from A (top) to X (bottom). B is near the top on the right, C on the left a little above the middle, D on the right below the middle, E on the right near the bottom. Each has a dotted horizontal to the vertical.
- *Conditions / counter-examples*: Until the habit of carrying a vertical and horizontal in mind is formed, draw them in "with as much accuracy as measuring can give" (p. 90).
- *Our interpretation*: Landmarks come before contours, against a single plumb reference.
- *Implication for vector software*: A plumb-line overlay through a chosen point, showing the horizontal offsets and heights of other chosen points with leader lines as in Fig. Z. An MCP command can report landmark positions relative to a reference axis. Agent workflow: create named landmark points first (crown, pit of the neck, and so on), check how they relate, then draw through them.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: engine-feature (plumb/landmark overlay); multi-step-workflow; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-VIII-3: Measure comparatively: arm's length, eye fixed, verticals and horizontals only, against a unit**
- *Author says*: Hold the needle at arm's length with the eye in one position. Whether vertical or horizontal, keep it in a plane at right angles to the line of sight. Run the thumbnail up the needle to match a distance, then compare it with others without moving the eye. Nature seldom gives tidy ratios. In Fig. Z, C is "a little above the half" and D "a little less than a third" of the way up; B's offset is a quarter of the height from X to C; D's offset is "a little less than one-fifth of the whole height". How much "a little" is must be judged by eye. "These measurements are never to be taken as absolutely accurate" (p. 88), but they help beginners train the eye and are "at times useful in every artist's work". Establish "a unit of measurement, some conspicuous distance that does not vary in the object" (p. 89); on a living model many distances keep changing. Quote (bold in source): "It is never advisable to compare other than vertical and horizontal measurements." (p. 88)
- *Figures*: Diagram IV, Fig. Z (as above).
- *Conditions / counter-examples*: The measurements are approximate and a training aid.
- *Our interpretation*: Use ratio language, a stable unit, and horizontal and vertical components only. We read the vertical/horizontal rule as a protection against the needle foreshortening when it tilts out of the picture plane (hence "kept in a vertical plane").
- *Implication for vector software*: A measure overlay that reports horizontal and vertical distances as ratios of a chosen unit (set from a selection), rounded to simple fractions with a qualifier such as "a little above 1/2", which is how artists read proportion. An agent can compute ratios between landmarks in both the drawing and the reference and report differences in the same terms. Software measurement is exact, so his "never absolutely accurate" applies to reading the reference, not to the arithmetic.
- *Do not algorithmize because*: n/a, though the final placement is still "observed by eye".
- *Disposition candidates*: analysis-overlay; engine-feature (set unit from selection); agent-prompt-or-resource.
- *Science caveat*: none; the geometry of sighting is sound.

**SP-VIII-4: Draw between limits decided at the start; find the half first**
- *Author says*: For Plate XVIII he chose a vertical through the pit of the arm as the most useful line for measuring. He drew it first and decided its length, then marked the relative heights of other points on it. "It is generally advisable to note where the half comes first" (p. 89), and it is very useful if it falls at an obvious place (here, exactly at the fold at the pit of the stomach). Then horizontals are drawn across and the transverse distances measured against the heights. "These guide lines are done mentally later on, when the student is more advanced" (p. 89). Being able to draw between set limits is valuable later for placing a figure exactly in a picture. Quote: "Train yourself to draw between limits decided upon at the start." (p. 89)
- *Figures*: Plate XVIII shows a crouching male nude seen from behind, head turned, one arm resting on the far thigh. A long vertical runs from above the head to below the base, with tick marks at the transverse heights, and there are horizontal lines at the top and base. Straight blocking lines surround the far leg and the block. The near leg is fully drawn and shaded; the back and arms show lifted highlights. He left the guide lines and unfinished parts visible "so as to show the different stages" (p. 89).
- *Conditions / counter-examples*: Guides are drawn early and become mental later.
- *Our interpretation*: Fix the frame (top, bottom, main axis, midpoint) before anything else.
- *Implication for vector software*: Agent workflow, step 1: set the subject's limits on the artboard and its main axis, place a midpoint landmark, then the other landmarks, all on a non-printing construction layer. Supporting features: a construction layer that toggles, and snapping to the midpoint of a guide.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: multi-step-workflow; agent-prompt-or-resource; practice-exercise.
- *Science caveat*: none.

**SP-VIII-5: Block in with straight lines, as a surveyor does**
- *Author says*: Next, block out the spaces the model occupies, as a surveyor plans an irregular field. He invests it with straight lines, notes their lengths and the angles at which they cut each other, reproduces them to scale, and then draws the irregularities against this "scaffolding". Straight lines can be judged reliably: "The inclination and length of straight lines can be observed with certainty." (p. 92) But "a curve has not this definiteness, and is a very unstable thing to set about copying unaided" (p. 92). Even highly skilled draughtsmen draw such lines mentally. Quote: "he would proceed to invest it with straight lines, taking advantage of any straightness in the boundary" (p. 90)
- *Figures*: Diagram IV, Fig. X is an irregular, bean-like blob with a small bump on top and a concave waist. About ten dashed straight lines lie along its flattest stretches, each extended beyond where it crosses its neighbours. The resulting polygon follows the concavities as well as the convexities, so it is not a convex hull. In Plate XVIII the far leg and the block show the same.
- *Conditions / counter-examples*: In the diagram the lines are "produced beyond what was necessary" for clarity (p. 90).
- *Our interpretation*: Fit lines to the flattest stretches, intersect neighbours, and compare angles and lengths.
- *Implication for vector software*: A block-in envelope generated from a path or traced silhouette: detect low-curvature runs, fit lines, intersect them into a polygon, and extend them as construction lines. Overlay it on the reference and on the drawing to compare angles. Note that Douglas–Peucker gives chords through points, not lines laid along flats, so a different fit is needed. Agent workflow: draw the block-in polygon first with few segments, check its angles against the reference, then draw the curves inside it.
- *Do not algorithmize because*: n/a for the envelope; the later drawing is not mechanical (SP-VIII-7).
- *Disposition candidates*: engine-feature; analysis-overlay; multi-step-workflow; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-VIII-6: Read a curve by its chord, its inflection and its farthest point**
- *Author says*: For a curved line, first fix its extremities with the vertical and horizontal. For a double curve also fix the point where the curvature changes direction (C). Draw lines CA and CB and note the distances the curve travels from them, "and particularly the relative position of the farthest points reached" (p. 91). If needed, hold a pencil or knitting needle up against the model. This construction "should always be in your mind". Quote: "First note the points at which the curvature begins and ends, and then the distances it travels from a line joining these two points" (p. 91)
- *Figures*: Diagram IV, Fig. Y is an S-curve from B (bottom left) to A (top right) with its inflection at C. Dash-dot chords join B to C and C to A. Short dotted perpendiculars run from each chord to the curve's farthest point. Dotted horizontal and vertical lines from C meet a frame through A and B.
- *Conditions / counter-examples*: none.
- *Our interpretation*: A curve is specified by its endpoints, its inflections, and for each span the position along the chord and the height of its apex.
- *Implication for vector software*: This fits Bézier geometry closely. An overlay for a selected path could show:
  - its inflection points;
  - the chords between consecutive endpoints and inflections;
  - each span's apex, with its position as a fraction along the chord and its height as a fraction of the chord length.

  That gives agents a compact way to describe and compare curves ("apex at 0.4 along, height 0.18 of the chord"), as MCP inspection output. It also tells simplify-with-character (SP-VI-3) which points to keep.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: engine-feature; analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-VIII-7: Scaffolding is only scaffolding: kick it away**
- *Author says*: Some quarters discourage blocking in, because some students carry it on mechanically, "cutting up almost all their curves into flatnesses", and never break free into "the enjoyment of free line expression". That is bad. Yet the character of a curve can hardly be studied any other way. The initial work "should be done as accurately as possible, but don't let it interfere with your freedom in expressing the forms afterwards" (p. 91). It will be "some years" before a beginner can do without it (p. 93). Quote: "this is only a scaffolding, and should always be regarded as such and kicked away" (p. 92)
- *Figures*: In Plate XVIII the scaffolding is left on the far leg and absent from the near leg.
- *Conditions / counter-examples*: Beginners need it drawn; experienced draughtsmen do it in their heads.
- *Our interpretation*: Construction and the final line are different objects.
- *Implication for vector software*: Keep construction geometry on a separate non-printing guide or construction layer that is easy to dim, hide or delete. Block-in polygons must never become the final contour. An analysis could flag final contours that are polygonal (many straight segments at shallow angles) where the reference is curved.
- *Do not algorithmize because*: when to let go of the scaffolding is a judgement.
- *Disposition candidates*: improve-existing (guides and construction layer); analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-VIII-8: When blocking in, observe the background shapes as carefully as the object**
- *Author says*: "In the case of foreshortenings, the eye, unaided by this blocking out, is always apt to be led astray." (p. 93) A foreshortened object looks so unlike what you know it to be that it is better to concentrate on the background than on the form. Quote: "in blocking out any object, whether foreshortened or not, the shape of the background should be observed as carefully as any other shape" (p. 93)
- *Figures*: none specific.
- *Conditions / counter-examples*: It is stressed for foreshortening and applies to all blocking in, but not to the drawing proper, where the form's inner relations govern (SP-VIII-10).
- *Our interpretation*: Negative-space shapes avoid the bias of what we know about the object.
- *Implication for vector software*: An analysis overlay that computes and tints the negative shapes (the frame minus the subject, and the gaps between limbs) in both the reference and the drawing, side by side. Boolean path operations (pathfinder) already exist. Agent guidance: check the negative shapes whenever a subject is foreshortened.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: analysis-overlay; engine-feature; agent-prompt-or-resource.
- *Science caveat*: That knowledge of the solid misleads perception of foreshortened shapes agrees with later research on knowledge bias in drawing, but here it is his practical observation.

**SP-VIII-9: The drawing proper: feeling for form, structure and action; tentative at first, then free and swift**
- *Author says*: Once the ground is pegged out, the work turns from mechanical to expressive. Knowledge of the bones and muscles under the skin helps seize on what is significant; he recommends Fripp's *Human Anatomy for Art Students*. Notice "the swing of the action", the pull from the arm resting on the far thigh, the skin straining at the shoulder, the firm lines of the bent back against the crumpled forms of the front, and the overlapping contours, where they are accented and where lost. Draw "with as much feeling and conviction as you are capable of". At first you will work tentatively, but "remember it should be your aim to express yourself freely and swiftly." (p. 92) Quote: "The work up to this point has been mechanical, but it is time to consider the subject with some feeling for form." (p. 91)
- *Figures*: Plate XVIII, near leg and torso. Plate XVII is a demonstration head of an old bearded man, captioned "how different directions of lines can help expression of form". Hair strokes sweep back, beard strokes fall, and the forehead and cheeks are hatched in different directions.
- *Conditions / counter-examples*: none.
- *Our interpretation*: A deliberate switch from measuring to expressing.
- *Implication for vector software*: After the measured stage, an agent should first identify the action (the main swing lines) and the tensions and compressions, then the detail. It should then commit contours as confident single paths rather than many small corrections (SP-IX-8).
- *Do not algorithmize because*: it rests on "feeling and conviction" and on choosing what is significant.
- *Disposition candidates*: multi-step-workflow; agent-prompt-or-resource; not-automatable (the feeling).
- *Science caveat*: none.

**SP-VIII-10: Inner relations: draw both sides of a form together**
- *Author says*: In the drawing proper, unlike the blocking in, "the forms must be observed in their inner relations" (p. 93). The lines bounding one side of a form are judged against those bounding the other side, because the true expression of form depends on that relationship. Quote: "The drawing of the two sides should be carried on simultaneously, so that one may constantly compare them." (p. 93)
- *Figures*: none specific.
- *Conditions / counter-examples*: It applies to the drawing proper.
- *Our interpretation*: Form is carried by width and by how the opposite contours of one form couple (for example a bulge on one side against a flat on the other).
- *Implication for vector software*: An agent should draw the two contours of a limb alternately, not one after the other. An analysis could show the width profile between paired contours along a medial axis, compared with the reference.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: multi-step-workflow; analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-VIII-11: Boundaries are a series of overlappings, accented in some places and lost in others**
- *Author says*: The boundaries of complex forms such as the human figure "are not continuous lines"; the outline is made up of a series of overlappings, which should be sought and expressed. In the drawing proper also notice "where they are accentuated and where more lost" (p. 92). Quote: "One form overlaps another, like the lines of a range of hills." (p. 93)
- *Figures*: In Plate XVIII the contours of the back and leg end and restart where one form passes behind another. Plate XXII (Puvis) shows contours searched in the same way.
- *Conditions / counter-examples*: "Forms with any complexity".
- *Our interpretation*: Contours are open segments meeting at T-junctions, with varying weight, not one closed outline.
- *Implication for vector software*: Draw a figure's contour as several open paths, each tucking under the next, with width profiles tapering at the ends and thickening at accents. Avoid one uniform closed stroke around a silhouette. A candidate command could split a closed outline at its concavities into overlapping open strokes. Also agent guidance text.
- *Do not algorithmize because*: where to accent and where to lose a contour is the artist's emphasis.
- *Disposition candidates*: agent-prompt-or-resource; design-principle; improve-existing (width profiles, tapered ends); engine-feature (split outline into overlaps, candidate).
- *Science caveat*: none. The T-junction is a recognised occlusion cue in later vision science, but the author does not claim this.

**SP-VIII-12: In line drawing, tone serves form only: keep it light, and set up to suit**
- *Author says*: Do not aim at true tone values in a line drawing. "Don't burden a line drawing with heavy half tones and shadows; keep them light." (p. 100) "The beauty that is the particular province of line drawing is the beauty of contours" (p. 100). Great draughtsmen use "only just enough to express the form". "Think of the half tones as part of the lights and not as part of the shadows." (p. 100) Set-up:
  - Avoid a strong half-light, half-shade arrangement: "A point of view with a fairly wide light at your back is the best." (p. 99) Little shadow then shows, forms read by light and half tone, and the contours, turning away from the light, are darker. This is easy to state in line.
  - "Strong light and shade effects should be left for mass drawing." (p. 99) Holbein's drawings seldom show shadows.
  - "Select also a background as near the tone of the highest light on the object to be drawn as possible." (p. 99) A newspaper behind a sitter's head works; vary its tone by its distance and angle.
  - Study light and shade on objects of one local colour, such as white casts. In line drawing take "as little notice as possible" of local colour (p. 99).

  Quote: "In Line Drawing shading should only be used to aid the expression of form." (p. 93)
- *Figures*: Plate XV (Ingres) and Plate XVII both have sparse shading beneath strong contours.
- *Conditions / counter-examples*: In his own method (SP-VIII-15), local-colour darks such as lips, eyebrows and eyes are treated together with light and shade as tones (p. 100), so local colour is not wholly ignored.
- *Our interpretation*: Line mode means contours first and tone light and secondary, with a frontal diffuse light and a light background.
- *Implication for vector software*: For "line drawing" requests an agent should assume frontal diffuse light, keep hatching sparse and pale, avoid heavy shadow masses, and group half tones with the lights. Checks: an ink-coverage or darkness histogram for the hatching, and the contrast between the subject's edge and the background.
- *Do not algorithmize because*: n/a for the checks; how much tone is "just enough" is judgement.
- *Disposition candidates*: agent-prompt-or-resource; analysis-overlay; design-principle.
- *Science caveat*: That contours turning away from a frontal light "will naturally be darker" agrees broadly with diffuse shading near occluding edges, but it depends on the lighting and is not a law.

**SP-VIII-13: Light and shade on a round form: highlights fall inside the contour**
- *Author says*: In direct light, surfaces facing the light are lit and those turned away are in shadow. Cast shadows fall on the ground and nearby objects. High lights are where most direct light is reflected: clear on shiny surfaces, soft on dull ones; a glazed pot may even mirror the window.

  His Diagram V reasoning: the angle of incidence equals the angle of reflection. A dull surface is rough under a microscope, made of many facets at different angles, so it reflects light to the eye from more places, fewer as the surface turns away, until the shadow begins where the surface turns from the light. So the high light comes "much more within the edge of the object than you would have expected". Students put it at the edge facing the light, and their work loses roundness.

  General law: "The surfaces turned more to the source of light receive the greatest amount, and are the lightest." (p. 98) Light falls off through the half tones to the start of the shadow. A wide diffused light, as outdoors on a grey day, gives little or no shadow, and the modelling depends on light and half tone only. Quote: "remember always to look out for high lights within the contours of forms, not on the edges." (p. 97)
- *Figures*: Diagram V, a plan view:
  - Circle A is the cone. Window B–C is on the left wall (hatched), the eye D at the bottom, and wall E–F faces the window, with G–F along the bottom.
  - Lines B→1→D and C→2→D bound the highlight 1–2, on the side facing both the window and the eye.
  - Tangents B–4 and C–3 mark where the shadow begins.
  - Light reflected from the wall between E and H reaches the eye between 5 and 6.
  - 7 and 8 are the visible edges. In the text, 7–3 is light including the half tones, 1–2 the high light, 3–8 shadow with the most reflected light at 5–6.
- *Conditions / counter-examples*: Other direct light on the shadow side complicates the effect (p. 98). You will rarely draw a cone, but the principles hold "throughout the whole of nature" (p. 97). That is why drawing whitened blocks and pots is useful.
- *Our interpretation*: A practical shading model. Orientation to the light sets the value, and the highlight's position depends on both the light and the eye. It never sits on the silhouette edge.
- *Implication for vector software*: An agent checklist: light, half tone, terminator, shadow, reflected light, cast shadow, highlight inside the contour. A candidate light guide for primitives (circle, ellipse, cylinder, cone) that takes light and view directions and draws the zone boundaries as vector bands to work over. An analysis could flag highlights placed on or touching the silhouette.
- *Do not algorithmize because*: n/a for primitives. Complex forms need the artist to read the planes.
- *Disposition candidates*: agent-prompt-or-resource; analysis-overlay; engine-feature (primitive light guide); practice-exercise (shade a cone or sphere).
- *Science caveat*: His optics (incidence equals reflection; dull surfaces as many facets) agree qualitatively with later microfacet reflectance models, and "lightest where turned to the light" is roughly Lambertian. But his zone positions are worked out for one plan arrangement with a window and a pale wall. They are not universal, and the highlight's position is view-dependent.

**SP-VIII-14: Shadows: darkest next to the light, lighter in the middle from reflected light**
- *Author says*: Reflected light comes mostly from the direction opposite the main light, because objects there are strongly lit. So the part of the shadow turned most away from the reflected light, beside the light, is darkest. The lightest part of the shadow is "in the middle, rather towards the side away from the light, generally speaking". The cast shadow on the ground is as dark as the darkest part of the shadow on the object, because it too faces away from the reflected light. The "heavy, smoky look" of students' early work comes from ignoring this. "Nothing is more awful than shadows darker in the middle and gradually lighter towards their edges." (p. 98) Deep hollows, such as the armpit and the fold at the navel in Plate XVIII, are darker without contradicting the rule. Quote (bold in source): "the darkest part of the shadow will come nearest the lights between points 3 and 5." (p. 97)
- *Figures*: Diagram V (zone 3–5 darkest, 5–6 lit by reflection). Plate XVIII: he points to the "luminous quality" of the shadow on the body.
- *Conditions / counter-examples*: "Generally speaking". Other light sources change it, and diffuse light gives little shadow (pp. 98–99).
- *Our interpretation*: What is now usually called the core shadow sits beside the terminator, and reflected light lifts the inside of the shadow.
- *Implication for vector software*: An item on the agent checklist. An analysis heuristic: within a shadow shape, check that the darkest band borders the light side rather than the centre. In vector shading, build a shadow as a dark band along the terminator plus a lighter reflected-light shape inside it, not a radial gradient darkening toward the centre.
- *Do not algorithmize because*: n/a for the check; the exact values are observed.
- *Disposition candidates*: agent-prompt-or-resource; analysis-overlay; practice-exercise.
- *Science caveat*: This holds for the common arrangement of one main light with bright surroundings opposite it. It is not a physical law: dark surroundings give little reflected light. "Nothing is more awful" is taste.

**SP-VIII-15: The parallel-shading method, and parallel by default**
- *Author says*: There are many ways to draw in line. He describes one "logical" method, used for most of his own drawings in the book:
  - The object is first seen as a series of contours: outer ones against the background and inner ones bounding the subordinate forms.
  - Light and shade and differences of local colour are treated together as tones, drawn as parallel lines "darker and closer together when depth is wanted, and fainter and further apart where delicacy is demanded, and varying in thickness when gradation is needed".
  - The rule is broken only for strongly marked forms: the swing lines of hair, a prominent bone, straining muscles.
  - It gives "great beauty of surface and fleshiness", and a unity of great charm, because the lines follow "the direction of the light across the object rather than the form". It suits delicate work and is usual in silverpoint.

  When in doubt, keep them parallel: this "preserves a unity", and lines drawn in other directions for special reasons then "tell expressively" (p. 103). Quote: "suggested by means of lines drawn parallel across the drawing from left to right, and from below upwards, or vice versa" (p. 100)
- *Figures*: Plate XX, a study for the figure of Love in "Love Leaving Psyche": a nude with arms raised, with separate studies of an arm and legs. Almost all the shading runs in one diagonal, from lower left to upper right, closer and darker in shadow and sparse in the lights. The hair at the top is in heavier strokes.
- *Conditions / counter-examples*: "There are many different methods of drawing in line, and a student of any originality will find one that suits his temperament." (p. 100) This method conveys "a minimum amount of that 'form stimulus'" (p. 101).
- *Our interpretation*: One global hatch angle, with tone carried by spacing and weight, and deviations kept for meaning.
- *Implication for vector software*: A hatch fill for a region, with angle, spacing and line weight mapped from a tone (or a tone field), and local direction overrides. A document-wide default hatch angle so separate objects share a direction. Agents should use one angle across a drawing unless a form calls for another.
- *Do not algorithmize because*: n/a for the fill; when to break the rule is the artist's call.
- *Disposition candidates*: engine-feature; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-VIII-16: The direction of shading lines carries meaning**
- *Author says*:
  - Curving the shading lines adds to the force of relief and suggests stronger modelling. In foreshortening, where forms arch one over another, it adds to the foreshortened look (p. 101).
  - "Lines drawn down the forms give an appearance of great strength and toughness, a tense look." (p. 101) Use for joints, sinews, rocks, hard ground and gnarled trunks. In figures, use sparingly with across-the-form shading, for a change of texture or a strained form.
  - Lines crossing in every direction "suggest atmosphere and the absence of surface form" (p. 101). This belongs to pen-and-ink backgrounds and is seldom needed in pencil or chalk.
  - Never scribble back and forth, "except possibly where a mystery of shadow is wanted": such lines "are not sufficiently under control, and also the little extra thickness that occurs at the turn is a nuisance." (p. 102)
  - "The crossing of lines in shading gives a more opaque look." (p. 102) Use it for the darker passage of shadow nearest the lights, and sometimes in the half tones.

  Used "judiciously", these four qualities add expressive power, and the same principle applies to brushwork (SP-IX-13). Quote (bold in source): "lines of shading drawn across the forms suggest softness, lines drawn in curves fulness of form, lines drawn down the forms hardness" (p. 102), and crossing lines, atmosphere.
- *Figures*: Plate XIX is a reclining male nude with one arm over his head. The hatching curves around the thighs, belly and chest, following their cross-sections, with dark crossed hatching beneath the body and a separate study of an arm holding a staff. In Plate XVII the lines of hair, beard and face run in different directions.
- *Conditions / counter-examples*: "Used judiciously" (p. 102).
- *Our interpretation*: A four-way vocabulary: across/parallel means soft, cross-contour curves mean full, along the axis means hard, and crosshatch means atmosphere or opacity.
- *Implication for vector software*:
  - Hatch modes: parallel; cross-contour, following a flow defined by guide curves (`object.blend.make` between two paths might generate these; verify); along the axis; crosshatch (several angles, used to deepen the core shadow).
  - Generate separate strokes, never zig-zag polylines, except in an explicit scribble mode.
  - An agent resource table mapping quality to mode.
- *Do not algorithmize because*: which quality to express is the artist's intent; the engine only supplies the modes.
- *Disposition candidates*: engine-feature; agent-prompt-or-resource.
- *Science caveat*: The softness and hardness associations are artistic convention, not measured. That cross-contour curves strengthen the impression of 3D shape agrees with later perception studies of line-direction cues, but he gives it as practice, not evidence.

**SP-VIII-17: Hair: swinging lines in the lights, shadow masses in the darks**
- *Author says*: The beauty of hair in point drawing is "the swing and flow of its lines", especially in the lights. In the shadows the flow often stops, "to be replaced by a mystery of shadow". So use "a play of swinging lines alternating with shadow passages" (p. 102), with the shadow passages drawn like all other shadows, in parallel lines that do not follow the form. Quote: "The swinging lines should vary in thickness along their course" (p. 103), darker in some parts and gradating lighter in others.
- *Figures*: Plate XXI is a red-chalk profile of a woman. Her hair is a dark mass at the back of the head, with a few long, swinging, thick-to-thin lines lifting from the forehead and crown and lighter strands in the light. The background on the right is swept with curved parallel strokes, and the shoulder drapery is in loose open lines.
- *Conditions / counter-examples*: "Draughtsmen vary very much in their treatment of hair, and different qualities of hair require different treatment" (p. 102).
- *Our interpretation*: Alternate flowing thick-and-thin strokes with flat hatched masses.
- *Implication for vector software*: Width profiles that swell and thin along a path. A recipe for agents: hair is a few long tapered strokes in the light and hatched masses in the shadow, parallel to the global hatch angle.
- *Do not algorithmize because*: where the flow is shown and where it stops is the artist's choice.
- *Disposition candidates*: agent-prompt-or-resource; improve-existing (width profiles); practice-exercise.
- *Science caveat*: none.

**SP-VIII-18: Appreciation comes from "feeding" on the best drawings; contours are searched**
- *Author says*: Accurate copying is not enough: "Form to be expressed must first be appreciated." Teaching can bring the student to the right point of view, but "you cannot make him see". So study the best drawings you can find and look for the same qualities in nature.

  Puvis de Chavannes's study for "Repose" repays study: the contours searched for expressive qualities, the "felt" line of the seated figure's back, the right angle of the raised arm, and the contrast of the two standing figures, "the practical vigour of the one and the soft grace of the other".

  Cheap reproductions mean "the danger is not from starvation, but indigestion." (p. 104) When something strikes you as particularly fine, "Feed deeply on it." (p. 104) Hang it where you see it constantly. Be honest and feed on "the best that you genuinely like" until you tire of it and want something better. Quote: "How, then, is this appreciation of form to be developed? Simply by feeding." (p. 103)
- *Figures*: Plate XXII (Puvis, study for "Repose" at Amiens). At left a bearded man sits on a block; his raised forearm and hand make a right angle with the upper arm. To the right stand two nude men. The middle one is relaxed, with soft curving contours; the right-hand one has folded arms and stands upright and squarer. Caption: "Note how the contours are searched for expressive forms". The text cites Diagram XII (chapter XII, outside this range) for the right angle: Diagram XII shows a face with a cross and two profiles, one with a right-angled brow-and-nose line and one with softer curves.
- *Conditions / counter-examples*: Taste develops: "You will probably like very indifferent drawings at first" (p. 104).
- *Our interpretation*: Repeated, deep study of a few examples. The character of a contour expresses the type: straight and angular for vigour, soft curves for grace.
- *Implication for vector software*: An agent resource: a small, annotated set of public-domain master drawings, licence-checked and recorded in `ASSETS.md`, with notes on what to look for, so agents and users can "feed" on them. Guidance that a contour's character should match the character of the subject.
- *Do not algorithmize because*: genuine liking and appreciation are personal. A curated set and notes can help but cannot stand in for them.
- *Disposition candidates*: agent-prompt-or-resource; practice-exercise; not-automatable.
- *Science caveat*: none.

**SP-VIII-19: Eye structure: socket bone, spherical eyeball, lids and brow as tones**
- *Author says*:
  - Structure (Fig. 2). Under the skin and muscle are two structures: the bony ring of the socket and the eyeball. From A to B the opening runs smoothly into the bony prominence at the top of the nose. The rest of the edge is sharp, and free from C to E. The sharp edge begins at A, by the inner corner of the eye. From A to F the bony edge lies near the surface.
  - Brow and bone. The eyebrow follows the upper edge of the socket from B to C, then crosses the free arch C–D and soon ends. So under the brow there is a hollow from C toward B and a prominence from C toward D. "Look out for the bone, therefore, between the points CD and AF." (p. 106)
  - Eyeball and lids. The eyelids are bands of flesh on the eyeball's spherical surface and share its modelling. "Note particularly the sudden change of plane usually marked by a fold" (p. 106) where the lower lid meets the surface coming from the cheekbone; neglecting it is a common fault. Note also the strongly marked fold where the upper lid meets the flesh under the brow.
  - Eyebrow (Fig. 4). A few hairs radiate from A, thickest between B and E. A second growth runs downward from B to C. The darkest part is where the two growths meet near B and E, with another dark toward D. The edge C–B is soft (a bony prominence there carries a highlight that crosses the brow); C–D is sharper; D–B and A–B are softer. "The chief accent will generally be found at B" (p. 108).
  - Eyelashes (Fig. 5). They matter only for tone. They shade the white when the light is above, are thicker on the outer side and grow outward. With light from the left, A1 is barely shaded, B4 (the light side) is shaded by the lashes, and B3 and A2 are in shade because the sphere turns away.

  Quotes: "what we call the white of the eye is part of a sphere and will therefore have the light and shade of a sphere." (p. 106) "An eyebrow is considered by the draughtsman as a tone of a certain shape and qualities of edge." (pp. 106 and 108; the anchor falls mid-sentence and p. 107 is the diagram)
- *Figures*: Diagram VI, Figs. 1–5. Fig. 1 is an eye and brow with corner A. Fig. 2 shows the skull opening with the eyeball and points A–F. Fig. 3 is an eye with dotted lines for the socket and eyeball beneath, points A–G. Fig. 4 is an eyebrow showing hair direction, points A–E. Fig. 5 shows two eyes with arrows for light from the upper left, numbered 1–4 and lettered A and B.
- *Conditions / counter-examples*: Eyes vary a great deal, and fleshy fullness can modify the effect, but "some indication of a change is almost always to be observed" near C (p. 105).
- *Our interpretation*: Model features as shapes lying on underlying spheres and bones, and treat brows as tone shapes with specific edges.
- *Implication for vector software*: A portrait resource for agents: never a flat white eye; shade it like a sphere and let the lids follow; mark the plane change under the lower lid; draw the brow as a tone shape with soft and hard edge segments and an accent at the inner top. In vector this needs per-segment edge softness (SP-IX-6).
- *Do not algorithmize because*: n/a; this is structural knowledge applied by observation.
- *Disposition candidates*: agent-prompt-or-resource; practice-exercise.
- *Science caveat*: The anatomy broadly agrees with standard anatomy (orbital rim, eyeball). The hair-growth pattern is an artist's generalisation.

**SP-VIII-20: Head guide lines arch with the action of the head**
- *Author says*: Fig. 6 gives "a series of blocks all exactly alike in outline, with lines showing how the different actions of the head affect the guide lines on which the features hang". "These archings over should be carefully looked out for" whenever the head is not simply full face (p. 109). Quote: "how these actions can be suggested even when the contours are not varied." (p. 109)
- *Figures*: Diagram VI, Fig. 6: five identical egg-shaped heads with ears. The first has straight cross guide lines (full face). The others have the eye line arched up or down and the centre line curved or moved off-centre, showing the head tilted back, tilted forward, turned, and combinations.
- *Conditions / counter-examples*: Any position other than simple full face.
- *Our interpretation*: The features lie on curves wrapped round an ovoid, and the curvature of the guide lines encodes the head's rotation.
- *Implication for vector software*: A head construction guide: an ellipsoid with yaw, pitch and roll, drawing the projected centre-line and eye-line curves. The repo has a three-d crate and perspective tools that might be reused; verify. Agent guidance: place features along arched guides; a straight eye line implies an untilted head.
- *Do not algorithmize because*: n/a; this is projective geometry.
- *Disposition candidates*: engine-feature; agent-prompt-or-resource; practice-exercise.
- *Science caveat*: none.

### Procedure: line drawing from a model (chapter VIII, with Plate XVIII)

1. **Set up the light and background.** Use a fairly wide light behind you, so there is little shadow and the forms read by light and half tone. Hang a background close to the tone of the highest light, such as a newspaper adjusted by distance and angle. For early studies use an object of one local colour, such as a white cast. [SP-VIII-12]
2. **Flatten the view.** Close one eye. If helpful, look through the gridded frame, held vertical with its sides vertical and horizontal. [SP-VIII-1]
3. **Set the limits and the main axis.** Decide the drawing's limits on the paper. Choose the most useful vertical (in Plate XVIII, through the pit of the arm), draw it, and fix its length. [SP-VIII-4]
4. **Find the half.** Find where the half of that vertical falls, ideally at an obvious landmark (here the fold at the pit of the stomach). [SP-VIII-4, SP-VIII-3]
5. **Mark the heights of salient points** on the vertical, using a plumb line or knitting needle at arm's length, eye fixed, needle in a plane at right angles to the line of sight. Compare only vertical and horizontal distances, against a fixed unit, and treat the results as approximate. [SP-VIII-2, SP-VIII-3]
6. **Draw horizontals across** and measure the transverse distances against the heights. Leave the guide lines in until you can hold them in mind. [SP-VIII-2, SP-VIII-4]
7. **Block in with straight lines,** like a surveyor. Lay lines along the straightest stretches of the boundary, note their lengths and the angles where they cross, and extend them. Watch the background (negative) shapes as carefully as the object, especially when it is foreshortened. [SP-VIII-5, SP-VIII-8]
8. **Fix the curves.** Mark each curve's ends and inflection points against the vertical and horizontal, draw the chords, and note how far the curve travels from each chord and where its farthest point lies. [SP-VIII-6]
9. **Begin the drawing proper,** treating everything so far as scaffolding.
   - Draw with feeling for form, using anatomy: the swing of the action, pulls and strains, firm against crumpled forms.
   - Draw both sides of each form at the same time.
   - Build contours as overlappings, accented in places and lost in others.
   - Work tentatively at first, then freely and swiftly.

   [SP-VIII-7, SP-VIII-9, SP-VIII-10, SP-VIII-11]
10. **Shade lightly, and only to explain the form.**
    - Use parallel lines in an easy direction: darker and closer for depth, fainter and wider for delicacy, varying thickness for gradation.
    - Break the parallel only for hair, prominent bone or straining muscle. Curve the lines for fullness or foreshortening, run them down the form for hardness, and cross them for the opaque dark of the shadow next to the light.
    - Never scribble.
    - Treat the half tones as part of the lights. Keep highlights inside the contours. Shadows are darkest next to the light and lighter in the middle, except in deep hollows.

    [SP-VIII-12, SP-VIII-13, SP-VIII-14, SP-VIII-15, SP-VIII-16]
11. **Hair:** swinging lines of varying thickness in the lights, alternating with shadow passages in parallel shading. [SP-VIII-17]
12. **Head-specific checks:** the bone at C–D and A–F; the eyeball modelled as a sphere and the lids on it; the plane change under the lower lid; the brow as a tone shape with soft and hard edges and its accent at B; guide lines arching with the head's action. [SP-VIII-19, SP-VIII-20]
13. **Optional fourth stage (Plate XVIII caption only, not described in the body text):** "Rubbing with fingers (giving a faint middle tone over the whole), and picking out high lights with bread." [SP-VIII-13; compare the middle-tone idea in SP-IX-4]

Throughout: feed on fine drawings, and compare your contours with theirs for expressive search. [SP-VIII-18]

---

## IX Mass Drawing: Practical

**Locator**: `14264-h-1`, lines 3541–4125. Anchors `[#Page_110]`–`[#Page_126]`. Sub-anchors: `#CHAPTER_IX`, `#pgepubid00116`–`#pgepubid00133`, `#plate23`–`#plate29`. Sidenote: "Exercise in Mass Drawing" (line 3567).

### Principles

**SP-IX-1: Painting is drawing with a brush; start mass work alongside line work**
- *Author says*: It is wrong to treat painting as distinct from drawing. A brush full of paint demands some form of mass drawing, so begin simple brush exercises while still progressing with line. Charcoal is "a sort of mixed method, half line and half mass" (p. 113). It is allied to painting but no substitute, and used to the neglect of line drawing it leads to "a sloppy manner of work". Its popularity is because "you can get much effect with little knowledge." (p. 113) Painting into a middle tone "is the best method for studying form expression with the brush." (p. 113) Quote: "Painting is drawing (i.e. the expression of form) with the added complication of colour and tone." (p. 110)
- *Figures*: none.
- *Conditions / counter-examples*: "I don't suggest that this is the right or only way of painting" (p. 112).
- *Our interpretation*: Mass work is drawing, and media that give easy effects can hide weak drawing.
- *Implication for vector software*: Treat fills and value shapes as drawing (the expression of form), not decoration. Agents should not use blur, texture or glow effects to fake form they have not drawn.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: design-principle; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-IX-2: Reduce things to three tone masses: lights, half tones, shadows**
- *Author says*: Early on, form "the habit of reducing things into a simple equation of three tones as a foundation on which to build complex appearances" (p. 110). Quote: "Most objects can be reduced broadly into three tone masses, the lights (including the high lights), the half tones, and the shadows." (p. 110)
- *Figures*: In Plates XXIII–XXIV (cast) and XXVI–XXIX (life), each stage adds one of the three: the ground is the half tone, then the lights, then the darks. Swatches of the paints used sit in a corner: one light swatch at stage 2, light and dark swatches at stages 3 and 4.
- *Conditions / counter-examples*: "Broadly". It is a foundation to build on.
- *Our interpretation*: A three-value structure comes before any refinement.
- *Implication for vector software*: A study template made by posterising a reference to three levels (with adjustable luminance thresholds) and tracing the result into vector shapes; posterize and trace exist in the repo, verify. A check that groups a drawing's fills into value clusters and warns when there is no clear light/half/shadow grouping. Agents should start with three values only.
- *Do not algorithmize because*: n/a for the template; where to set the thresholds is judged against the subject.
- *Disposition candidates*: analysis-overlay; engine-feature (three-value tone map); multi-step-workflow; agent-prompt-or-resource.
- *Science caveat*: Three tones is a practical simplification, not a perceptual law.

**SP-IX-3: The setting out is mapping, not drawing, but it must be accurate**
- *Author says*: Draw the outlines of the masses strongly in charcoal, noting the shapes of the shadows carefully, "blocked out in square lines in true proportion relative to each other, and troubling about little else" (p. 111). It is the same scaffolding as for a line drawing, but here the drawing proper is done with the brush. Fix the charcoal with a spray of white shellac in spirits of wine.

  He returns to this later. No expressive work is needed in the setting out, but "the utmost care" is needed for its accuracy. Proportions and main masses must be settled early, using "every device of blocking out with square lines and measuring with your knitting-needle" (p. 120). "This initial stage is not really a drawing at all, but a species of mapping out" (p. 120), and "The variations and emphases that feeling may dictate can be done in the painting stage." (p. 120) Elaborate preparatory drawings on the canvas are risky because students then fear to cover them. Quote: "Let this be a setting out of the ground upon which you will afterwards express the form, rather than a drawing" (p. 111)
- *Figures*: Plate XXIII No. 1 is a cast head of a laughing youth with curly hair, outlined in charcoal with the shadow shapes hatched in straight strokes. Plate XXVI No. 1 is a life head looking up and to the left: charcoal contours, with the shadows of the face, neck and coat hatched in bold parallel strokes.
- *Conditions / counter-examples*: The only excuse for detailed preparatory drawing is to learn the subject first (pp. 120–121).
- *Our interpretation*: A map of shapes, especially the shadow shapes, in correct proportion.
- *Implication for vector software*: An agent workflow phase: rough polygonal shapes for the masses and the light/shadow boundaries, in correct proportion, on a construction layer, checked with the SP-VIII-2, -3 and -5 tools. No styling yet.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: multi-step-workflow; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-IX-4: Paint into a carefully matched middle tone**
- *Author says*: The exercise:
  - Choose a simple object (one of the cast fruits common in art schools).
  - Give it strong light and shade, preferably from artificial light, which is "not so subtle, and therefore easier", with "the light coming from either the right or left hand, but not from in front." (pp. 110–111)
  - "Arrange it so that the tone of the ground of your cast comes about equal to the half tones in the relief." (p. 111)
  - Mix raw umber and white to match the cast's half tones.
  - Scumble the tone over the whole canvas with a big brush, rubbing it in from side to side with little medium (a little oil if it is too stiff, no turpentine), "laying just the thinnest solid tone that will cover the surface." (p. 111) The fixed drawing should just show through.

  Quote: "Extreme care should be taken in matching this tone." (p. 111)
- *Figures*: In Plate XXIII No. 2 and Plate XXVII a flat dark-grey ground covers everything and the charcoal lines show through. The Plate XXVII caption says "The darks are due to the charcoal lines of initial drawing showing through middle tone."
- *Conditions / counter-examples*: In colour, do not paint the middle tone over the shadows (SP-IX-15).
- *Our interpretation*: The ground is the half-tone mass, so only the lights and darks are painted.
- *Implication for vector software*: Workflow: set the background (or a region) fill to the sampled half-tone value, then draw only light shapes and dark shapes over it. An in-app practice exercise: a primitive lit from the side on a mid-tone background. Agents should sample and match the half-tone value from the reference with care before drawing lights and darks.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: multi-step-workflow; practice-exercise; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-IX-5: Lights, then darks, each in one tone, varied by how it mixes into the ground**
- *Author says*:
  - Mix a tone equal to the highest lights and "map out simply the shapes of the light masses", leaving the ground for the half tones (pp. 111–112). The wet ground mixes into the light and darkens it, so the light's variety comes from thickness: thick paint is lighter, thin paint is darker.
  - Then mix a tone equal to the darkest shadow and map the shadows the same way: "In the case of the shadows the thicker you paint the darker will be the tone; and the thinner, the lighter." (p. 112)
  - If this is done accurately the work is well advanced; then correct and refine.
  - The plates show "how much can be expressed with the one tone, when variety is got by using the middle tone to paint into" (p. 119).
  - Keep early work "quite solid, but as thin as you can" (p. 115).

  Quote: "The thicker you paint the lighter will be the tone" (p. 112), for the lights.
- *Figures*: The sequences Plate XXIII No. 2 → Plate XXIV No. 3 → No. 4, and Plates XXVII → XXVIII → XXIX. Only two paint swatches (light and dark) appear, yet the images hold many intermediate values. Captions: "Gradations are got by thinner paint, which is mixed with the wet middle tone of the ground, and is darkened" (Plate XXIII); "here the thinner part is lighter, whereas in the case of the lights it was darker" (Plate XXIV).
- *Conditions / counter-examples*: For a touch that must not be affected by the ground, such as the shiny light on a glazed pot, use a very full brush (p. 114).
- *Our interpretation*: Three source values, with the intermediate values made by partial coverage over the ground.
- *Implication for vector software*: A clean vector equivalent: a mid-tone ground, one light colour and one dark colour, with the intermediate values made by opacity ("thin" means lower opacity) or gradients to transparency rather than by many separate colours. A "two tones into a ground" mode, restricting swatches to the light and the dark and varying only opacity, works as a practice exercise and as an agent recipe for strong value unity.
- *Do not algorithmize because*: n/a for the mode; where to vary the thickness follows the form.
- *Disposition candidates*: multi-step-workflow; practice-exercise; agent-prompt-or-resource; engine-feature (optional constraint to two swatches, opacity only).
- *Science caveat*: none.

**SP-IX-6: Edges: where masses meet sharply and where they are lost**
- *Author says*: The same care applies to the shadows, "noting carefully where they come sharply against the half tone and where they are lost" (p. 112). The Plate XXIII caption speaks of "the play of lost-and-foundness on their edges being observed". In colour, the edge between the middle tone of the lights and that of the shadows is "carefully studied and finished" (p. 113). Quote: "Note carefully where the light masses come sharply against the half tones and where they merge softly into them." (p. 112)
- *Figures*: In Plate XXIII No. 2 and Plate XXIV the light shapes on the forehead and cheek end in hard edges against the ground in some places (nose bridge, brow) and dissolve in others (where the cheek turns). Plates XXVII–XXIX show the same on the life head.
- *Conditions / counter-examples*: none.
- *Our interpretation*: Edge quality is a separate decision for each stretch of each edge.
- *Implication for vector software*: Vector has no native per-segment edge softness. Options:
  - A lost-and-found edge control: softness that varies along an edge, for example a masked feather or a gradient mask driven by a ramp like a width profile. Feather effects and opacity masks exist in the repo; verify.
  - An overlay marking each edge of a shape as hard or soft.
  - Agent guidance to decide edges explicitly: hard where the form turns sharply or contrast is high, soft where forms turn gently.
- *Do not algorithmize because*: which edges to lose depends on form and emphasis. A tool can suggest from a reference, not decide.
- *Disposition candidates*: engine-feature; analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-IX-7: Judge from a distance, with half-closed eyes, and in a black glass**
- *Author says*: Place the work beside the cast and walk back to correct it.

  Mass seeing is harder at first, because it seems natural to see things as outlines. "The frame with cottons across it should be used to flatten the appearance" (p. 120). Also make a black glass: a small piece of glass (a photographic negative will do) backed with black paper turned over its raw edges, or painted black on the back. Stand with your back to the object and the painting and hold the glass close in front of one eye, the other closed, so you see both reflected. "Seeing the tones thus reduced and simplified, you will be enabled more easily to correct your work." (p. 120)

  Half-closing the eyes works too, "by lowering the tone and widening the focus" (p. 121). Quote: "Faults that are not apparent when close, are easily seen at a little distance." (p. 112)
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: Review in reduced, simplified views, side by side with the reference.
- *Implication for vector software*: View modes shown side by side with the reference:
  - distance: a small thumbnail;
  - squint: blurred and darkened;
  - black glass: darkened, with compressed contrast.

  Our note, not his: a reflection is also reversed left to right, which is a known check. Agents can render the artboard small or blurred through render commands and compare value masses with the reference at the same reduction.
- *Do not algorithmize because*: n/a for the views; the correction is the artist's.
- *Disposition candidates*: engine-feature (view modes); analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: Squinting cuts both light and acuity (it blurs), which fits "lowering the tone". "Widening the focus" is loose wording. The black glass's effect is a practical claim.

**SP-IX-8: One stroke, deft and thin: much handling kills**
- *Author says*: "Try always to do as much as possible with one stroke of the brush" (p. 114). Study the shape and variety of the tone and swing the brush to get it in one touch.

  The first contact gives the extreme. Painting lights into a middle tone, the lightest part of the touch is where the brush first lands, darkening as the paint runs out; in shadows the darkest part comes first and the stroke lightens. A very full brush is hardly influenced by the ground and is used for a distinct touch. Otherwise use as little paint as possible: "Thinner paint is easier to refine and manipulate." (p. 114)

  Do the studies in one sitting if you can (p. 119); if not, use slow-drying bone brown and zinc white. Quotes: "paint has a vitality when the touches are deft, that much handling and continual touching kills." (p. 114) "the lightest part of your touch will be where the brush first touches the canvas when you are painting lights into a middle tone" (p. 114)
- *Figures*: In Plate XXV, columns 1–2, the strokes are solid at the start and thin out or fade along their length.
- *Conditions / counter-examples*: Mixtures of solid and transparent paint are for later (pp. 114–115).
- *Our interpretation*: Economy and decisiveness of marks, and a mark's behaviour depends on what lies under it.
- *Implication for vector software*:
  - Prefer a few decisive paths with few anchors, and avoid repeated node-nudging.
  - An analysis of anchor density and edit count per path.
  - A candidate "loaded brush" whose colour blends from full strength into the underlying colour along its length.
  - Agent guidance: place a shape in one well-planned path, and if it is wrong, redraw rather than micro-edit.
- *Do not algorithmize because*: deftness is a skill. Tools can make single-stroke work pay off but cannot supply the skill.
- *Disposition candidates*: design-principle; agent-prompt-or-resource; analysis-overlay; engine-feature (blend into the ground along a stroke).
- *Science caveat*: none.

**SP-IX-9: Each brush shape and touch has its own character**
- *Author says*: Brushes fall roughly into flat and round.
  - **Class A, round with a sharp point.** The long central hairs press differently from the side hairs, so the touch is uneven and hard to manage. Prefer the blunt-ended round, which gives "a much more even touch, and one much more suited to painting in planes of tone" (pp. 115–116).
  - **Class B, extreme flat.** It gives a flat, even tone with hard sharp edges, "like little square bricks", but carries little paint, so strokes are short. It is useful when great refinement is wanted; a corner gives the smallest touch, growing as pressure increases. It is also used to paint across the form.
  - **Class C, full flat with rounded corners.** It carries plenty of paint, lays an even tone, and leaves softer edges.
  - **Class D, filbert.** Being flat it paints in planes, and its rounded top gets in and out of contours; the blunt shape is best.

  C or D suit these exercises best, but try all four.

  The touches (Plate XXV) assume paint thinned with equal parts turpentine and linseed oil:
  - **Column 1, firm touch** with equal pressure: a plane of tone with firm edges, the brush's width, changing as the brush empties.
  - **Column 2, drag touch:** placed firmly, then dragged lightly away, leaving a gradated tone. It risks "a dexterity more concerned with the clever manner in which a thing is painted than with the truth expressed." (p. 117)
  - **Column 3, grazing touch:** light and quick, brilliant and soft, feathery when the brush is moderately full. It keeps colour fresh and suits hair, but is "of very little service" in monochrome cast work.
  - **Hatching** with the brush: rows of parallel lines to lighten or darken, useful for correcting intricate modelling, united afterwards with a dry brush. He calls it "so mechanical a method" and says Monet seldom used it (p. 118).

  Quote: "A great deal of the modelling in round objects is to be expressed by this variety of handling." (p. 117), of the drag touch.
- *Figures*: Plate XXV is a grid of light strokes on a dark ground, rows Class A–D and columns 1–3. A strokes are pointed, with tapering tips. B strokes are rectangular with square ends; the second B touch in column 2 is made of short horizontal strokes across. C strokes are broad with rounded tops, and C column 2 includes a curved swing. D strokes are tapered filberts, also with a curved swing. Column 3 strokes are broken and feathery.
- *Conditions / counter-examples*: These are "only given as typical examples… every artist will, of course, develop ways of his own." (p. 115)
- *Our interpretation*: A small taxonomy of marks: tip shape × pressure profile × paint loading.
- *Implication for vector software*: A brush set generated in code (AGENTS.md: defaults generated in code, no copied presets):
  - shapes: round-blunt, round-pointed, flat-square ("brick"), flat with rounded corners, filbert;
  - touches: firm (constant), drag (tapering gradient), graze (broken, feathery edge).

  A practice exercise: reproduce the Plate XXV grid. An agent resource on which mark suits which job (planes, gradation, hair).
- *Do not algorithmize because*: n/a for the brushes; choosing the touch is the artist's.
- *Disposition candidates*: engine-feature (brush library generated in code); practice-exercise; agent-prompt-or-resource.
- *Science caveat*: none. This describes oil paint and applies to vector only by analogy.

**SP-IX-10: Correct by re-grounding; don't be afraid to lose the drawing**
- *Author says*:
  - If the whole thing is wrong, let it dry, then "scumble a middle tone right over the whole thing, as you did at first, which will show the old work through" (p. 119). Correct the drawing and repaint the lights and shadows.
  - If part is wrong, wait until it is dry, rub in a little poppy oil thinned with turpentine (breathe on the canvas if it won't take), and wipe it off. Paint middle tone over that part, joining it carefully to the rest, and draw the lights and shadows again.
  - The attitude: "When you take up a brush to express yourself, it must be with no fear of hurting a careful drawing." (p. 121) Drawing is done all the time, with the brush. "It is often necessary when a painting is nearly right to destroy the whole thing" (p. 121), as a motor car must take a long run at a hill it cannot quite climb.

  Quote: "he that would save his work must often lose it" (p. 121)
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: Recover by veiling earlier work under a translucent middle tone and restating it, or by starting over.
- *Implication for vector software*: A candidate "re-ground" command: put a semi-opaque mid-tone layer over the selected work, so the old work shows through, then restate the lights and darks on top. Agents should be willing to delete and redraw a region rather than pile up corrections; undo and snapshots make this safe.
- *Do not algorithmize because*: deciding to destroy nearly-right work is a judgement.
- *Disposition candidates*: multi-step-workflow; agent-prompt-or-resource; design-principle.
- *Science caveat*: none.

**SP-IX-11: Values: each mass has a place on a scale from dark to light**
- *Author says*: In tone drawing there are the shapes of the masses and their values. How these tones relate is "an extremely important matter in painting", but it belongs to Colour, which "needs a volume to itself"; more is promised under Rhythm. Quote: "their position in an imagined scale from dark to light." (p. 122)
- *Figures*: The swatches in Plates XXIV and XXVIII–XXIX mark the two ends used.
- *Conditions / counter-examples*: Deferred to colour and rhythm.
- *Our interpretation*: Values are relative positions on a scale.
- *Implication for vector software*: A value readout for fills and strokes, a value-scale strip, and a grayscale preview. Grayscale conversion exists in the repo; check whether there is a preview mode. Use a perceptual lightness such as CIE L\*. Agents should pick values from an explicit numbered scale and report them.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: engine-feature; analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: none in his words. Using perceptual lightness rather than RGB averages is modern practice, not his.

**SP-IX-12: Modelling's character is in its planes; avoid "gross roundnesses"; know the structure**
- *Author says*: A line's character is found by observing its flatnesses (VIII), and in the same way "the character of modelling is found by observing its planes" (p. 122, bold in source). Seek the planes, or flat tones, everywhere. A stone carver blocks out his work in square surfaces; set out modelling in planes of tone, "painting in the first instance the larger ones, and then, to these, adding the smaller" (p. 122), and with a little fusing of edges the roundnesses appear. "Good modelling is full of these planes subtly fused together." (p. 122) A sphere, like a circle, is the surface "with the least character".

  Knowledge of anatomy, particularly the bones, is "of the utmost importance" for structure and action. But "Never let anatomical knowledge tempt you into exaggerated statements of internal structure" (p. 123), unless that helps what you want to express: it may suit violent action, but not a figure at rest or a portrait. Quote: "Nothing is so characteristic of bad modelling as 'gross roundnesses.'" (p. 122)
- *Figures*: In Plate XXIV No. 4 and Plate XXIX the faces are built from distinct flat patches of tone, with some edges fused. Plate XVI and Diagram II show flatnesses in a contour (SP-VII-5).
- *Conditions / counter-examples*: Large planes first, then smaller ones; fuse some of the edges.
- *Our interpretation*: Facet the form, then soften selected facet edges.
- *Implication for vector software*: Vector is naturally planar. Model with flat facets, large then small, and fuse chosen edges with blends, gradients between neighbouring facets, or feathering. Avoid the default radial-gradient "ball". An analysis could flag circles and ellipses filled with a centred radial gradient as likely "gross roundness". Agent workflow: block the large planes, add the sub-planes, then fuse.
- *Do not algorithmize because*: where the planes break follows the subject's structure.
- *Disposition candidates*: design-principle; multi-step-workflow; analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: That a circle has constant curvature is a fact. That it has "least character" is his judgement.

**SP-IX-13: Brush direction follows the same vocabulary as line direction**
- *Author says*: The line rules apply to brushwork: across the forms means softness, down the forms toughness and hardness, crossing in every direction atmosphere. His examples:
  - Tintoretto's "Finding of the Body of St. Mark" (Plate XLIX): the foreshortened figure on the ground at the left, helped by brushwork swinging round it.
  - Henner: softness from painting across the form.
  - James Ward's "Harlech Castle", National Gallery no. 1158: toughness from the brush following the tree-trunks.
  - Crossing strokes: backgrounds, sky and mist.

  Painting across the form is often impractical, because the colour changes more across a form than down it. So paint down in overlapping bands of shadow, half tone and light. Then either keep the paint so thin or the tones so fused that no strokes show, or draw a dry flat brush lightly across afterwards, "great care being taken to drag only from light to dark" (p. 124), wiping the brush after each touch and never going over the same place twice. For a tough look (a muscle in violent action, the tendons above the wrist and heel, bone near the surface), the brush should follow down the form. "It is not necessary and is often inadvisable for the brush work to show at all" (p. 125). Quote (bold in source): "The brush swinging round the forms suggests fore-shortening, and fulness of form generally" (p. 123)
- *Figures*: Plate XLIX (outside this range, viewed): a body lies strongly foreshortened on the floor at lower left. At this reproduction size the brushwork cannot be read, so the image does not let us check his point.
- *Conditions / counter-examples*: It matters only when the brushwork shows.
- *Our interpretation*: The vocabulary of mark direction does not depend on the medium.
- *Implication for vector software*: For brush-stroke or hatch-based vector work, offer orientation fields (around, across, along) as parameters. The agent resource is the same table as SP-VIII-16.
- *Do not algorithmize because*: as SP-VIII-16.
- *Disposition candidates*: agent-prompt-or-resource; engine-feature (shared with SP-VIII-16).
- *Science caveat*: as SP-VIII-16.

**SP-IX-14: Toned paper: lights and darks kept apart by the paper's half tone**
- *Author says*: Drawing on toned paper in white chalk or Chinese white with black or red chalk is another form of mass drawing. It is quick and excellent for studies to paint from, and "above all others the method for drapery studies" (p. 125).
  - Draw the lights in white, letting the paper show where a darker tone is needed. White goes on "thickly when a bright light is wanted and thinly where a quieter light is needed" (p. 125).
  - The chalk goes on heavily in the darks and less heavily in the lighter shadows.
  - Burne-Jones shaded his lights in gold and silver paint, on the same principle.
  - Put as much information as possible into the lights and shadows: "don't be satisfied with a smudge effect." (p. 126) Use the side of the chalk for a mass, or hatch in parallel lines.

  Quote: "Keep the lights separate from the shadows, let the half tone paper always come as a buffer state between them." (pp. 125–126)
- *Figures*: Plate LIV (outside this range, viewed) is a draped female figure in motion on brown paper. The lights are white-chalk hatching and strokes on the folds; the darks are black-chalk lines; the brown paper gives the half tone. Faint vertical and horizontal construction lines show near the figure.
- *Conditions / counter-examples*: none.
- *Our interpretation*: A three-value system in which the middle value is the ground and must lie between the lights and the darks.
- *Implication for vector software*: A "toned ground" workflow: a mid-tone artboard, light shapes in one colour and dark shapes in another, never touching. An analysis that finds boundaries shared directly between light shapes and dark shapes, with no half tone between them, and flags them. A practice exercise for drapery.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: multi-step-workflow; analysis-overlay; practice-exercise; agent-prompt-or-resource.
- *Science caveat*: none.

**SP-IX-15: In colour, use separate middle tones for lights and shadows, with a temperature contrast**
- *Author says*: In colour, an opaque middle tone over everything spoils the clarity and transparency of the shadows and may dull the colour of the lights. There is no need to paint it over the shadows. Instead, "the half tone or middle tone of the lights can be made, and a middle tone of the shadows" (p. 113), painted separately, with the edge where they meet "carefully studied and finished". Then add the variety within each. This keeps the difference in colour quality between lights and shadows, contrasts that "greatly affect the vitality of colouring". Other colour expedients are left for later. Quote: "the shadows usually being warm if the lights are cool and vice versa" (p. 114)
- *Figures*: none.
- *Conditions / counter-examples*: "Usually". Colour is deferred.
- *Our interpretation*: Two colour families, one for the lights and one for the shadows, with a designed boundary between them.
- *Implication for vector software*: An agent resource: choose a light family and a shadow family with a temperature contrast, and set the light/shadow boundary first. A candidate engine feature: paired light/shadow swatch groups.
- *Do not algorithmize because*: n/a for the pairing; the actual colours are observed or chosen.
- *Disposition candidates*: agent-prompt-or-resource; engine-feature (paired swatch groups).
- *Science caveat*: Light and shadow reversing in temperature is a painter's generalisation. Which way it goes depends on the light sources; warm sunlight with blue skylight, for example, gives cool shadows. It is not a law.

### Procedure: mass drawing in paint (chapter IX, Plates XXIII–XXIX)

1. **Set up the subject.** Choose a simple cast, such as fruit. Light it strongly with artificial light from the right or left, not from in front, and arrange the cast's ground to equal the half tones of the relief. [SP-IX-4]
2. **Set out in charcoal.** Draw the outlines of the masses strongly, especially the shadow shapes, blocked out in square lines in true proportion. Use every blocking and knitting-needle measuring device for the large proportions. This is mapping, not drawing. Fix it with shellac spray. [SP-IX-3, SP-VIII-2, SP-VIII-3, SP-VIII-5]
3. **Match the half tone.** Mix raw umber and white to equal the half tones, matched with extreme care. [SP-IX-4]
4. **Lay the ground.** Scumble it over the whole canvas with a big brush: little medium, a little oil if needed, no turpentine, "the thinnest solid tone that will cover the surface". The drawing should just show through. [SP-IX-4]
5. **Map the lights.** Mix a tone equal to the highest lights and map the light masses, leaving the ground as the half tones. Note where edges are sharp and where they merge. Vary the tone with thickness: thick is lighter, thin is darker, because the wet ground mixes in. [SP-IX-2, SP-IX-5, SP-IX-6]
6. **Map the shadows.** Mix a tone equal to the darkest shadow and map the shadows the same way, noting sharp and lost edges. Here thick is darker and thin is lighter. [SP-IX-2, SP-IX-5, SP-IX-6]
7. **Correct and refine** as needed. Put the work beside the cast and walk back. Also half-close your eyes, use the black glass with your back to the subject, and use the cotton frame. [SP-IX-7]
8. **Throughout,** do as much as possible in one deft stroke. The first contact is the extreme, and the stroke fades as the brush empties. Use as little paint as possible, and a full brush only for distinct touches. Use blunt round, Class C or filbert brushes, with paint pre-thinned with equal parts turpentine and linseed oil. [SP-IX-8, SP-IX-9]
9. **Finish in one sitting** if possible; otherwise use slow-drying bone brown and zinc white. [SP-IX-8]
10. **If it is wrong:**
    - All of it: let it dry, scumble the middle tone over everything (the old work shows through), correct, and repaint the lights and shadows.
    - Part of it: oil out with poppy oil and turpentine (breathe on the canvas if it won't take), wipe, paint middle tone over that part joined to its surroundings, and repaint.

    Do not be afraid to lose the drawing. [SP-IX-10]
11. **Progress** to casts of hands and heads, then to life studies done the same way, in monochrome into a middle tone (Plates XXVI–XXIX). [SP-IX-1, SP-IX-3]
12. **While modelling,** look for planes: large ones first, then smaller ones, fusing some edges. Avoid gross roundness. Use anatomy without exaggerating it. Choose the direction of the brushwork for its meaning: round the form, across it, down it, or crossing. [SP-IX-12, SP-IX-13]
13. **Colour variant.** Make a middle tone for the lights and a separate one for the shadows. Paint them separately, finish the edge where they meet, then add variety. Keep the warm/cool contrast. [SP-IX-15]
14. **Toned-paper variant (drapery).**
    - The paper is the half tone.
    - Draw the lights in white chalk, thick for bright and thin for quiet.
    - Draw the darks in black or red chalk, heavy for the darks and lighter for the lighter shadows.
    - Keep the paper between the lights and the darks. Use the side of the chalk for masses, or hatch, and never smudge.

    [SP-IX-14]

---

## X Rhythm (supplementary: inside the assigned line range, not in the assigned chapter list)

**Locator**: `14264-h-1`, lines 4126–4447. Anchors `[#Page_127]`–`[#Page_136]`. Sub-anchors: `#CHAPTER_X`, `#pgepubid00134`–`#pgepubid00144`, `#plate30`. Sidenote: "Unity and Variety" (line 4298). The chapter ends by announcing that line drawing will be taken first and mass drawing afterwards (chapters XI onward).

### Principles

**SP-X-1: Rhythm is an abstract music of line, tone and colour, to which appearances must be related**
- *Author says*: He uses "rhythm" for the power of lines, tones and colours, through their ordering and arrangement, to affect us as notes and chords do in music, without direct reference to nature. The naturalistic nineteenth century turned attention away from this, toward "realisations of appearances". He offers his ideas tentatively: "I do not know of any proper authority for them" (p. 127).

  In primitive art the emotional significance of line and form shows most clearly. A school is at its height when as much naturalistic truth has been added as the abstract base can bear; after that comes decadence. Modern "primitives" cannot recover the old conditions, but their insistence on design should help kill "realism" and the "copying nature" theory (p. 130). Quotes: "There is, as it were, an abstract music of line, tone, and colour." (p. 127) "it is the business of the artist to relate the form, colour, and tone of natural appearances to this abstract musical quality" (p. 128, bold in source).
- *Figures*: Plate XXX, a study for "Rosalind and Orlando": a woman in a hat with her hand at her chin, and five separate studies of a hand. It is pencil, and the text does not discuss it.
- *Conditions / counter-examples*: He presents it as a speculative theory: "roughly the theory" (p. 129).
- *Our interpretation*: Plan the abstract arrangement and keep it through to the detail.
- *Implication for vector software*: An agent should plan the abstract arrangement of lines and masses before rendering, and check it again after detailing.
- *Do not algorithmize because*: the emotional effect of an arrangement is the artist's aim, not a metric.
- *Disposition candidates*: design-principle; agent-prompt-or-resource.
- *Science caveat*: His appeal to science ("sound, heat, light, and possibly electricity and even nerve force are but different rhythmic forms of energy", and the hint that rhythm may hold "the secret of life itself", p. 128) is 1913 speculation. The cycle of art history he describes is a theory, not evidence.

**SP-X-2: Every picture rests on a structure of lines and masses; the abstract arrangement and the subject's sentiment should agree**
- *Author says*: The structure may be hidden under broken technique, but it is always there, and much of a picture's power to move us comes from it, apart from any association. Mere geometric lines are enough to suggest these qualities. Associations from the subject add to the effect when they are in sympathy and override it when they are not: a sublime arrangement fails under a braying donkey, because "the associations aroused by so ridiculous an appearance will override those connected with the line and tone arrangement" (p. 132). In nature the two usually agree; a donkey at rest is more sublime than a braying one. He goes on to treat line and tone only, without subject matter. Quote: "There is at the basis of every picture a structure of lines and masses." (p. 131)
- *Figures*: none.
- *Conditions / counter-examples*: The donkey case, where subject association overrides the arrangement.
- *Our interpretation*: Check the underlying structure on its own, apart from what is depicted.
- *Implication for vector software*: An "abstract view" overlay that reduces the drawing to its main lines and value blocks (outline mode plus a three-value map) to check the structure. Agents should match the line and mass skeleton to the intended mood.
- *Do not algorithmize because*: what sentiment a structure carries is judged.
- *Disposition candidates*: analysis-overlay; agent-prompt-or-resource; design-principle.
- *Science caveat*: His claims about the emotional effects of abstract arrangements are artistic opinion; later empirical aesthetics is mixed.

**SP-X-3: Unity and variety; variety is the "dither"; unity governs**
- *Author says*:
  - Unity relates every part to one controlling conception and carries the deeper emotional notes.
  - Variety holds charm, vitality and the picturesque: "it is the 'dither,' the play between the larger parts, that makes for life and character." (p. 132)
  - Perfection without variety is lifeless; his examples are Nirvana, fakirs and a stylite saint.
  - "Variety must always be under the moral control of unity" (p. 134). The most perfect work has the least variety compatible with life, as the engine has the least dither.
  - Variety without unity is "a riotous exuberance of life… wasting itself in a madness of excess". "In good work unity is the dominating quality" (p. 136).
  - Definitions he endorses: good style is "variety in unity"; Hogarth called composition the art of "varying well"; good colour might be "contrasts in harmony".

  Quote (bold in source): "Without variety there can be no life." (p. 132)
- *Figures*: none.
- *Conditions / counter-examples*: The balance leans toward unity.
- *Our interpretation*: Every variation should serve one governing idea of the whole.
- *Implication for vector software*: For agents: every variation in size, value, angle or detail should be subordinate to a stated idea of the whole; check that detail serves it. It ties back to SP-VI-2.
- *Do not algorithmize because*: striking the balance is judgement.
- *Disposition candidates*: design-principle; agent-prompt-or-resource; not-automatable.
- *Science caveat*: none (aesthetic).

**SP-X-4: Canons, composite ideal types and the circle lead to deadness; individuality is life**
- *Author says*:
  - The Greeks never let "No formula or canon of proportions or other mechanical device for the attainment of perfection" (p. 133) subdue their love of variety. Much Roman sculpture, portrait busts apart, lacks subtle variety of modelling.
  - Greek mouldings used conic sections, curves with the greatest variety. "The Romans used the curve of the circle in the sections of their mouldings" (p. 134), the curve with the least.
  - Building an ideal figure from the best features of many models produced dead work.
  - "Excess of variations from a common type is what I think we recognise as ugliness" (p. 134), and "beauty is on the side of unity and conformity to type" (p. 134). Beauty has both unity and variety, is never extreme, and errs toward unity. Prettiness is charm without character or type.
  - Egyptian sculpture shows great unity with variety suppressed until life "scarcely exists".
  - A beautiful face charms through its individual variations from the type. Movement and habit mould features away from it.

  Quote: "the Romans used the curve of the circle in the sections of their mouldings" (p. 134)
- *Figures*: none.
- *Conditions / counter-examples*: The present danger, he says, is the opposite: "An anarchy of individualism is upon us" (p. 134).
- *Our interpretation*: Curves with varying curvature and individual departures from type keep work alive. Formulas and averages deaden it.
- *Implication for vector software*: Where liveliness is wanted, prefer curves of varying curvature (conic-like, Bézier) to circular arcs; the curvature analysis is shared with SP-VII-5. Agents should avoid averaged, generic faces and figures and keep individual variation, while not exaggerating (SP-VI-2).
- *Do not algorithmize because*: how far to vary from type is judgement.
- *Disposition candidates*: design-principle; analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: "Ugliness as deviation from type, beauty as conformity" is his opinion. Later research on facial averageness partly agrees, but also finds that distinctiveness counts. The Greek conic-section mouldings are an art-historical claim he says he will explain later; it is not verified here.

**SP-X-5: Principles help, but cannot replace the impulse**
- *Author says*: Many principles of arranging lines, tones and colours are "difficult to transgress without calamity", and knowing them may "save him some needless fumbling" (p. 130). But do not think "anything in the nature of rules is going to take the place of the initial artistic impulse which must come from within" (p. 130). Art training is concerned only with perfecting the means of expression. Quote: "anything in the nature of rules is going to take the place of the initial artistic impulse" (p. 130)
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: This frames the whole project: features and guidance are means, not formulas.
- *Implication for vector software*: Our features, checks and guidance should present Speed's principles as means and warnings, not pass/fail rules on artistic quality.
- *Do not algorithmize because*: the author says so explicitly.
- *Disposition candidates*: design-principle; not-automatable.
- *Science caveat*: none.

---

## Cross-cutting notes

**Recurring themes**
1. **Mechanical first, expressive after, and keep them apart.** VI separates academic accuracy from vital expression. VIII separates scaffolding (salient points, blocking in, curve construction) from the drawing proper. IX separates the setting out ("a species of mapping out") from the brush drawing. In every case the mechanical stage must be accurate and must then be discarded or covered without regret (SP-VIII-7, SP-IX-3, SP-IX-10). For agent workflows this suggests explicit phases with different success criteria.
2. **Straight and flat to understand curved and round.** Curves are read against straight lines (SP-VIII-5, -6), rich forms show "flatnesses that give strength" (SP-VII-5), and modelling is built from planes (SP-IX-12). The circle and sphere are "least character" (SP-IX-12, SP-X-4). This translates directly to Bézier geometry: chords, inflections, apexes, and curvature variation.
3. **Dither, variety and life.** Introduced in VI (SP-VI-2) and redefined in X as variety, "the play between the larger parts" (SP-X-3). It is always bounded: too much is caricature or "riotous exuberance". It must not be implemented as random noise.
4. **Line versus mass as two ways of seeing** (SP-VII-1), each developed in its own practical chapter, converging at full form. Vector strokes and fills map onto them naturally.
5. **The middle tone as an organising idea.** In line drawing the half tones belong to the lights (SP-VIII-12). In mass drawing the ground is the half tone, and only lights and darks are painted (SP-IX-4, -5). On toned paper the half tone is a buffer between them (SP-IX-14). Plate XVIII's fourth stage rubs a faint middle tone over a line drawing.
6. **Edges.** Contours accented and lost (SP-VIII-11), lost-and-found edges (SP-IX-6), the eyebrow's edges (SP-VIII-19). This is the biggest gap between Speed's media and vector, which has no native per-segment edge softness.
7. **A vocabulary of mark direction.** Across means soft, curved round the form means full or foreshortened, down the form means hard, crossing means atmosphere or opacity. It is stated for line (SP-VIII-16) and repeated for brushwork (SP-IX-13), so it is independent of medium.
8. **Review in a reduced view.** Step back, half-close the eyes, use the black glass or the gridded frame (SP-VIII-1, SP-IX-7). This maps to render-and-compare loops for agents.
9. **The medium sets the convention** (SP-VI-8). The framing principle for translating Speed's chalk and oil advice to vector: carry the principle across, not the physics of the material.

**Defined terminology** (author's terms, with where they are defined)
- *Academic*: dead, not merely thorough (p. 73); also "all that can be really taught" (p. 78).
- *Conventional*, in the "dead sense": a manner not one's own (p. 75).
- *Dither*: "that play on mechanical accuracy, existing in all vital art" (p. 72); in X, variety (p. 132).
- *Salient points* (p. 86).
- *Blocking in / blocking out*: straight-line envelope (p. 90).
- *Scaffolding* (pp. 90–92).
- *The drawing proper* (p. 91).
- *Inner relations* (p. 93).
- *Overlappings* (p. 93).
- *Unit of measurement* (p. 89).
- *High lights, half tones, shadows, cast shadows, reflected light* (pp. 94–98).
- *Local colour* (p. 99).
- *Form stimulus* (p. 81, p. 101).
- *Tone masses* (p. 81, p. 110).
- *Values*: "position in an imagined scale from dark to light" (p. 122).
- *Planes* (p. 122).
- *Gross roundnesses* (p. 122).
- *Scumbling*: "rubbing the colour into the canvas… the thinnest solid tone that will cover the surface" (p. 111).
- *Middle tone* (pp. 111–113).
- *Lost-and-foundness*: Plate XXIII caption.
- *Drag touch* and *hatching* with the brush (pp. 117–118).
- *Black glass* (p. 120).
- *Feeding*: developing appreciation (p. 103).
- *Rhythm*, *Unity*, *Variety* (pp. 127–136).

**Where he warns against mechanical rules or academic formula**
- Measurements "are never to be taken as absolutely accurate" (p. 88).
- Blocking in "is only a scaffolding… kicked away" (p. 92); students who keep cutting curves into flatnesses are criticised (p. 92).
- His line method is one "logical" way among many: "a student of any originality will find one that suits his temperament" (p. 100).
- "I don't suggest that this is the right or only way of painting" (p. 112).
- His brush strokes are "only given as typical examples… every artist will, of course, develop ways of his own" (p. 115).
- The drag touch risks "a dexterity more concerned with the clever manner in which a thing is painted than with the truth expressed" (p. 117).
- Brush hatching is "so mechanical a method" (p. 118).
- Don't exaggerate anatomy unless it serves the expression (p. 123).
- Don't treat style as a target (SP-VI-7).
- Canons of proportion and composite ideal types produce deadness (p. 133–134).
- Rules cannot replace "the initial artistic impulse" (p. 130).
- "it is only the academic that can be taught" (p. 73).

Implication for the whole programme: tools can measure and show; agents can follow the procedures; neither should score "life".

**Translation hazards (ours)**
- Several procedures depend on material behaviour: wet-into-wet darkening, a brush emptying, chalk thickness. Each has a vector stand-in (opacity, gradient along a stroke, layering), but SP-VI-8 argues for vector-native means, not simulation.
- Plates are reproductions. In Plate XLIX the brushwork cannot be checked at this size.

## Open questions

1. **Chapter X ownership.** The assigned line range (2300–4448) includes chapter X "Rhythm" (4131–4447), but the chapter list stops at IX. I ledgered X as `SP-X-1`…`SP-X-5`. Please confirm with part C whether they also cover X, and deduplicate.
2. **Probable text error, p. 88.** "The height of B is so near the top as to be best judged by eye, and its distance to the right is the same as B." Diagram IV Fig. Z has B near the top and E near the bottom, at about the same offset to the right. The sentence probably means E ("the height of E is so near the bottom…"). The source says B, and I have quoted it as given.
3. **Transcriber's cross-reference error.** On pp. 116–117 the text twice refers brush touches to "page 114", and the transcriber's note says "Plate XXVI". The brush-stroke illustration is Plate XXV; Plate XXVI is the charcoal life study. Cite Plate XXV for the brush strokes.
4. **Swatch positions.** The text says "The two tones used are noted in the right-hand lower corner" (p. 119). In Plates XXVII–XXIX the swatches are lower right, but in Plates XXIII–XXIV (the cast) they are lower left. This is minor, but matters if anyone builds an exercise that copies the layout.
5. **Plate XVIII's fourth stage** (rubbing a middle tone with the fingers, lifting highlights with bread) appears only in the caption, not the body text. Should it count as part of the line procedure, or as a bridge to mass drawing?
6. **Diagram II and Plate XVI** sit in chapter VII, but no text in VII discusses them. "Squareness" is developed later (around line 6861, chapter XV), so parts C and D should link SP-VII-5 to their principles.
7. **Per-segment edge softness in vector** (SP-IX-6, SP-VIII-11, SP-VIII-19) has no native SVG or vector primitive. Engineering needs to say whether a masked or variable feather, mesh, or blend is acceptable, and how it exports to SVG and PDF.
8. **Dither** (SP-VI-2, SP-X-3). Product should decide explicitly *not* to ship a feature named or marketed as "dither" or "life", to avoid turning it into random jitter. Confirm.
9. **Shading model** (SP-VIII-13, -14, IX-15). Any engine light guide should state its own model (light direction, view direction, environment) rather than present Speed's single window-and-wall arrangement as general. Who owns checking this against modern references?
10. **Reference set** (SP-VIII-18). The plates are 1913 reproductions of older works, with photo credits such as Bulloz, Neurdein and Anderson. Before any are shipped in the app as "feeding" material, each needs a licence check and an `ASSETS.md` row under the repo's asset rules. Shipping them is a publishing decision that needs the owner's approval.
11. **Colour** is explicitly deferred by the author ("needs a volume to itself", p. 122). Colour features beyond SP-IX-15 will need other public sources.
12. **Existing repo features named here** (width profiles, feather, roughen, scribble, guides, perspective grid, simplify, trace, posterize, grayscale, blend, pathfinder, three-d) were seen only by name in `crates/`. Their actual behaviour was not checked. Feature planners should verify before marking anything "improve-existing".
