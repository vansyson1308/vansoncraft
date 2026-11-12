# Ledger C: Speed, *The Practice and Science of Drawing* (1913), chapters X–XIV (Rhythm)

Source: Project Gutenberg #14264, extracted to `book.txt`. Page numbers come only from `[#Page_N]` anchors. A quote whose text crosses an anchor is cited "pp. N–M". Plate captions have no page anchor of their own and are cited by plate number.

Scope note: the brief gave lines 4448–6820 but also listed chapter **X Rhythm**. `## X` is actually at line 4131; line 4448 is `## XI`. To cover all five named chapters I read **lines 4126–6819** (from the `[#CHAPTER_X]` anchor up to `## XV`).

Conventions: quotes are in “curly quotes”, copied exactly, with straight quotes inside as in the source. *Ours* marks our interpretation. Existing command ids are named only where they appear in the command registry (`crates/engine/src/cmd/`); what they do was not checked in the ledger.

## Reading log

| Section (chapter + heading) | book.txt lines | XHTML file | Page anchors covered | Read? | Figures viewed (large `7373817595610384519_*` copy) |
|---|---|---|---|---|---|
| X Rhythm: opening, naturalism, primitivism | 4126–4297 | `8481025652923775349_14264-h-1.htm.xhtml` | 127–132 | yes | Plate XXX |
| X: sidenote "Unity and Variety" | 4298–4442 | h-1 | 132–136 | yes | none |
| XI Rhythm: Variety of Line: straight, circle, ellipse, egg, profile | 4443–4562 | h-1 | 137–141 | yes | Diagram VII |
| XI: "Variety in Symmetry", "Variety of Thickness and Accent" | 4563–4613 | h-1 | 141, 143 (no 142 anchor) | yes | Diagrams VIII, IX |
| XII Rhythm: Unity of Line: opening, contour links | 4614–4660 | h-1 | 144–145 | yes | none |
| XII: "Parallelism" (Blake), straight lines and sublimity, strength | 4661–4818 | `8481025652923775349_14264-h-2.htm.xhtml` (file starts at 4661) | 145–149 | yes | Plates XXXI, XXXII |
| XII: "The Horizontal and the Vertical", steadying straights, Titian | 4819–5019 | h-2 | 149–155 (no 152–153) | yes | Diagrams X, XI; Plates XXXIII, XXXIV |
| XII: "The Right Angle", panel shape, corners, diagonals | 5020–5201 | h-2 | 155–162 (no 158) | yes | Diagrams XII, XIII; Plates XXXV, XXXVI |
| XII: "Curved Lines", continuous flow, flame-like flow | 5202–5380 | h-2 | 162–169 (no 164–166, 168) | yes | Diagrams XIV(1), XIV(2), XV, XVI; Plates XXXVII, XXXVIII |
| XII: jarring lines, radiating lines, sympathetic radiation | 5381–5600 | h-2 | 171–176 (no 170, 174) | yes | Diagrams XVII, XVIII, XIX; Plates XXXIX, XL, XLI |
| XII: line sympathy (portrait, hair), abstract arrangements, scale vs detail | 5601–5778 | h-2 | 176–184 (no 178) | yes | Diagrams XX, XXI, XXII; Plate XLII |
| XIII Variety of Mass: shape, tone values, quality and texture | 5779–6029 | h-2 | 185–192 | yes | Plate XLIII |
| XIII: edges, focus, serrated edges, gradation | 6030–6254 | h-2 | 192–199 | yes | Plate XLIV; Diagram XXIII |
| XIV Unity of Mass: tone schemes, seeing values, extremes, scale methods, two-mass formula | 6255–6492 | h-2 | 200–207 | yes | Plates XLV, XLVI |
| XIV: flat tones, contrast vs complexity, colour vs tone, gradated tones, Watteau/Turner/Corot, conclusion | 6493–6819 | h-2 | 207–218 (no 210, 213, 215) | yes | Diagrams XXIV, XXV, XXVI; Plate XLVII |

Totals: 18 plates (XXX–XLVII) and 20 diagrams (VII–XXVI; XIV is split into two images), which is **39 image files, all viewed**. I cropped and enlarged Diagrams XVIII and XIX to read the line numbers. I measured line positions in Diagrams X-A and XI-D from pixels. Missing page anchors fall on full-page plates or diagrams (the transcriber's notes in the text place Diagrams X and XI on pp. 152–153, for example).

---

## X Rhythm

**Locator**: h-1. Anchors `[#CHAPTER_X]`, `[#Page_127]` to `[#Page_136]`. Sub-anchors: `#plate30`, sidenote "Unity and Variety" (p. 132). Book.txt 4126–4442.

### SP-X-1: An abstract music of line, tone and colour
- *Author says*: "rhythm" means the power that lines, tones and colours have, through their arrangement, to affect us the way musical sounds do, apart from what they represent. Geometric lines alone are enough to carry it. Associations with the objects shown make it stronger when the two agree. When they clash, the associations win (his example is a sublime line arrangement under a braying donkey). “There is, as it were, an abstract music of line, tone, and colour.” (p. 127). Also: “arrangements of mere geometrical lines are sufficient to suggest them.” (p. 131).
- *Figures*: Plate XXX (Speed's study for "Rosalind and Orlando": a head in a hat and five separate hand studies) is placed here but the text never refers to it.
- *Conditions / counter-examples*: subject associations can "obscure or entirely destroy" the line and tone music (pp. 131–132). He says such clashes are rare in nature.
- *Our interpretation*: this is the premise of the whole range. The abstract layout (lines, masses) is a separate layer of meaning that you can design and inspect on its own.
- *Implication for vector software*: show a document's "abstract layer", meaning strokes and masses with subject detail stripped (e.g. outline mode, flat grayscale masses). An agent should design and review that layer as well as the subject.
- *Do not algorithmize because*: which emotion an arrangement carries is the author's claim, not something we can measure. Tools may show the structure but must not label its mood.
- *Disposition candidates*: design-principle (premise for every overlay below); agent-prompt-or-resource (treat the abstract layout as a deliberate design layer).
- *Science caveat*: the claim that line arrangements carry fixed emotional effects is 1913 artistic opinion, not established perception science.

### SP-X-2: Relate natural appearance to the abstract structure; naturalism must not crowd it out
- *Author says*: the danger of 19th-century naturalism is attention to appearances at the expense of abstract structure. The artist's business is to tie appearance to that structure, even in the most finished detail. In his reading of art history, good schools add natural truth "as much … as the abstract significance at the base of the expression could stand without loss of power" (p. 129), and decline begins when they go further. “it is the business of the artist to relate the form, colour, and tone of natural appearances to this abstract musical quality” (p. 128).
- *Figures*: none.
- *Conditions / counter-examples*: he is sceptical of modern "primitivism" (pp. 129–130), but credits its insistence on design with weakening the "copying nature" theory.
- *Our interpretation*: order of work: abstract structure first, then add detail only as far as the structure can carry it.
- *Implication for vector software*: a staged agent workflow: (1) structural lines and masses, (2) detail, with a check after each detail pass that the stage-1 structure still reads (compare a blurred or simplified render to the stage-1 sketch).
- *Do not algorithmize because*: how much detail the structure can stand is a judgement. A tool can show both versions side by side but cannot decide.
- *Disposition candidates*: multi-step-workflow (structure, then detail, then re-check); agent-prompt-or-resource.
- *Science caveat*: the art-historical theory (Greece, Italy, decadence) is the author's speculation, as he says himself ("roughly the theory", p. 129).

### SP-X-3: Every picture has an underlying structure of lines and masses
- *Author says*: a structure of lines and masses always underlies the planning, even under broken technique. Lines and masses are two views of the same structure. Much of a picture's power to move comes from this original planning. “There is at the basis of every picture a structure of lines and masses.” (p. 131).
- *Figures*: none here (see SP-XII-15 for his "anatomy" diagrams).
- *Conditions / counter-examples*: the structure "may not be very obvious" (p. 131).
- *Our interpretation*: lines and masses are the two analysis channels for the rest of the range: chapters XI–XII cover lines, XIII–XIV cover masses.
- *Implication for vector software*: two analysis views of one document: a **line view** (centrelines and contours of shapes) and a **mass view** (flattened value regions). Vector data makes both cheap to produce: paths give contours, and rendered luminance gives masses.
- *Do not algorithmize because*: n/a for showing the structure. Saying whether it is good is judgement.
- *Disposition candidates*: analysis-overlay (line view / mass view); design-principle.
- *Science caveat*: none (a description of practice).

### SP-X-4: Unity and Variety are the two axes
- *Author says*: rhythm can be studied from two somewhat opposed qualities, as harmony and contrast are in colour. Unity is the relation of all parts to one controlling conception and carries the "deeper emotional notes". Variety gives charm, vitality, the picturesque, the "dither" (the play between the larger parts). “Without variety there can be no life.” (p. 132).
- *Figures*: none.
- *Conditions / counter-examples*: "dither" is a term he coined in an earlier chapter (book.txt ~2416–2474, outside this range, which I did not read). Here it means the play of life between parts.
- *Our interpretation*: every later principle pushes toward one axis or the other. A useful tag for any feature is whether it adds unity (parallels, echoes, alignment to the frame) or variety (width variation, curvature change, edge play).
- *Implication for vector software*: group guidance and analysis under two headings, "unity checks" and "variety checks", so an agent knows which way a suggestion pushes and does not stack only one kind.
- *Do not algorithmize because*: the right balance depends on the subject (SP-X-6).
- *Disposition candidates*: design-principle; agent-prompt-or-resource (vocabulary for prompts and reports).
- *Science caveat*: none for the framework. The claim about "deeper emotional notes" is opinion.

### SP-X-5: Unity without variety is lifeless
- *Author says*: attempts at absolute perfection end in deadness: the Buddhist ideal, fakirs, a saint on a column, Egyptian sculpture (long simple lines, smooth surfaces, no action), Roman copies of Greek work. Roman mouldings use the circle, which has the least variety; Greek mouldings use conic sections, which have the most. “all those who have aimed at an absolute perfection have usually ended in a deadness.” (p. 133).
- *Figures*: none (the mouldings come back in Diagram VII, SP-XI-5).
- *Conditions / counter-examples*: he still grants that Egyptian work achieves "complete" sublimity (p. 135). Deadness is the price of that aim, not a failure to reach it.
- *Our interpretation*: perfectly regular construction (exact circles, uniform spacing, identical repeats) looks mechanical. Vector tools produce exactly that by default.
- *Implication for vector software*: an analysis that **flags mechanical regularity**: exact circular arcs, constant stroke width, identical copies, exactly equal spacing. It should say "regular" without saying "bad". A related tool ("add controlled variation") must not become random jitter (see SP-XIV-17).
- *Do not algorithmize because*: whether deadness is intended (Egyptian sublimity, a logo) is the artist's choice.
- *Disposition candidates*: analysis-overlay (regularity flags); improve-existing (`addon.arrange.randomizeOpacity` exists in the command list; any "vary" tool should be seeded and bounded); design-principle.
- *Science caveat*: the Greek conic versus Roman circle claim about mouldings is a historical generalisation the author does not support with evidence.

### SP-X-6: Variety must stay governed by unity ("variety in unity")
- *Author says*: variety without unity is "a riotous exuberance" (p. 136). Good work is unity-dominant, with all variety done "in conformity to some large idea of the whole". The most perfect work has the least variety compatible with life. He cites "variety in unity", Hogarth's "varying well", and offers "contrasts in harmony" for colour. “variety must always be under the moral control of unity, or it will get out of hand and become extravagant.” (p. 134). “Good style in art has been defined as "variety in unity,"” (p. 136).
- *Figures*: none.
- *Conditions / counter-examples*: he says the fashion of 1913 errs toward "An anarchy of individualism" (p. 134).
- *Our interpretation*: add variety inside a fixed large structure and keep it small. Default: unity first, variety second.
- *Implication for vector software*: agent guidance: decide the large structure (dominant direction, main masses), then add variation inside it, and limit how far variation may move a shape from its structural guide. A workflow can keep the guide layer (SP-XII-15) visible while varying.
- *Do not algorithmize because*: the least variety compatible with life has no numeric threshold.
- *Disposition candidates*: design-principle; multi-step-workflow; agent-prompt-or-resource.
- *Science caveat*: "moral control" is a metaphor, not a claim we can test.

### SP-X-7: Beauty sits near the type; charm lies in individual departures from it; prettiness has neither
- *Author says*: excessive departure from a common type reads as ugliness, while beauty has both unity and variety and errs toward unity. "Mere prettiness" is charm without character or type, and "always feeble" (p. 135). A "perfect" face would leave us cold. What charms is the subtle individual variation, which life and expression always produce. “Beauty possesses both variety and unity, and is never extreme, erring rather on the side of unity.” (pp. 134–135). “it would seem to be the imperfections of perfection that charm and stir us” (p. 136).
- *Figures*: none.
- *Conditions / counter-examples*: he cites Burke but uses "beautiful" differently.
- *Our interpretation*: for drawing, idealised or averaged forms (such as a generic AI face) lose life. Keep the specific departures.
- *Implication for vector software*: guidance only. When an agent draws a figure or face, keep specific asymmetries and avoid mirrored perfect symmetry (links to SP-XI-8).
- *Do not algorithmize because*: **never** turn "departure from type" into a beauty or ugliness score for faces or bodies. It would amount to judging real people and is not supported science.
- *Disposition candidates*: agent-prompt-or-resource (avoid over-idealised forms); not-automatable (as a metric).
- *Science caveat*: the claims about beauty, ugliness and type are 1913 aesthetic opinion. Do not present them as fact.

### SP-X-8: Rules do not replace the impulse; principles save fumbling
- *Author says*: many principles of arranging lines, tones and colours are hard to break "without calamity" (p. 130), and knowing them saves "needless fumbling". But teaching only perfects the means of expression. “don't for one moment think that anything in the nature of rules is going to take the place of the initial artistic impulse” (p. 130).
- *Figures*: none.
- *Conditions / counter-examples*: he repeats this in XIV (SP-XIV-16, -17).
- *Our interpretation*: these principles are a **diagnostic aid**, not a generator.
- *Implication for vector software*: write analysis output as observations and questions ("three long verticals run parallel to the frame edge"), never as pass/fail scores. Agent prompts should say that the user's intent overrides any principle.
- *Do not algorithmize because*: the "initial artistic impulse" is by definition outside the tool.
- *Disposition candidates*: design-principle (wording of all analysis features); agent-prompt-or-resource.
- *Science caveat*: none.

---

## XI Rhythm: Variety of Line

**Locator**: h-1. Anchors `[#CHAPTER_XI]`, `[#Page_137]` to `[#Page_143]` (no 142 anchor). Sub-anchors: `#diagram07`, `#diagram08`, `#diagram09`. Sidenotes "Variety in Symmetry" (p. 141), "Variety of Thickness and Accent" (p. 143). Book.txt 4443–4613.

### SP-XI-1: Line rhythm has three determinants
- *Author says*: line rhythm depends on (1) the shape of the lines, (2) their relation to each other, (3) their relation to the panel's edges. It should agree with the subject. “Line rhythm or music depends on the shape of your lines, their relation to each other and their relation to the boundaries of your panel.” (p. 137).
- *Figures*: none.
- *Conditions / counter-examples*: "In all good work this music of line is in harmony with the subject" (p. 137). The aim depends on the subject.
- *Our interpretation*: three measurable families: per-path shape (curvature), path-to-path relations (parallel, radiating, continuing, crossing), and path-to-artboard relations (parallel to edges or diagonals).
- *Implication for vector software*: organise a "line rhythm" overlay in three tabs: **Shape** (curvature plot per path), **Relations** (parallel, radiating, continuation, crossings), **Frame** (orientation relative to artboard edges and diagonals). Each tab measures geometry only.
- *Do not algorithmize because*: "harmony with the subject" is about intent.
- *Disposition candidates*: analysis-overlay; design-principle (taxonomy for later features).
- *Science caveat*: none.

### SP-XI-2: The straight line and the circle have the least variety; the circle is perfect "static unity"
- *Author says*: a straight line has no variety. A circle curves at a constant rate, so it is the least varied curve. Both are "two of the dullest" lines (p. 137) and are rarely used except to set off others, and then with some play. Yet the circle has beauty through perfect unity, hence its use as a symbol of eternity. “a circle, by curving at exactly the same ratio all along, has no variation of curvature” (p. 137). “It is of all curves the most perfect example of static unity.” (p. 138).
- *Figures*: none.
- *Conditions / counter-examples*: in XII they return as carriers of sublimity and strength (SP-XII-6).
- *Our interpretation*: an exact geometric definition: variety of a line = variation of curvature along it. Line: κ = 0. Circle: κ constant.
- *Implication for vector software*: a **curvature-variation metric** per path: sample κ(s) by arc length and report the range, the coefficient of variation, and how many sign changes it has. A line and a circle both score 0 variation. Show a curvature comb overlay (the command list has `path.curvature`; whether it draws a comb was not checked). *Limits*: the metric describes geometry only. A low score is not a defect (a horizon, a coin). It is sensitive to sampling and to tiny node noise, so smooth before measuring.
- *Do not algorithmize because*: n/a for the metric. "dull" is judgement.
- *Disposition candidates*: analysis-overlay (curvature comb and variation number); engine-feature (curvature statistics on path info); practice-exercise (draw curves with rising curvature variation).
- *Science caveat*: "dullest" is aesthetic opinion. The curvature definitions are exact mathematics.

### SP-XI-3: Verticals and horizontals tie a design to a rectangular frame and set off curves
- *Author says*: vertical and horizontal lines are parallel to a rectangular picture's edges and "unite" the design to them. They also contrast with curves: mouldings behind a portrait head, a column beside a figure. Portrait painters look for such lines in the background. Even the grid ruled for copying a drawing ("squaring it out") improves it by contrast. “vertical and horizontal lines are of the utmost value in rectangular pictures, uniting the composition to its bounding lines” (p. 137).
- *Figures*: none.
- *Conditions / counter-examples*: this holds for rectangular panels. A non-rectangular frame would change which lines are "parallel to the boundaries" (*ours*, inferred from SP-XII-16).
- *Our interpretation*: alignment to the frame is a unity device. A few straight lines among curves increase how curved the curves look.
- *Implication for vector software*: in the frame-relation analysis, measure the length-weighted share of stroke and edge direction within ±3° of 0° and 90° relative to the artboard. *Limits*: it counts geometry, not visual weight. Text baselines and UI frames dominate it, so exclude them.
- *Do not algorithmize because*: n/a for the measurement.
- *Disposition candidates*: analysis-overlay (direction histogram relative to artboard); agent-prompt-or-resource ("if a design is all curves, consider one steadying straight"; see SP-XII-11).
- *Science caveat*: the claim that a grid would "improve the look" is the author's observation, not verified.

### SP-XI-4: Avoid the perfect circle and the modelled sphere in natural objects
- *Author says*: never draw natural objects with a perfect circle, "even a full moon", and look for some variety in any lively drawing. Never model a perfect sphere, "the dullest of all curved surfaces" (p. 138). “The perfect curve of the circle should always be avoided in the drawing of natural objects (even a full moon)” (p. 138).
- *Figures*: none.
- *Conditions / counter-examples*: he means natural objects and "vital drawings". In XII he uses a perfect sun disc on purpose for stillness (Diagram X-B).
- *Our interpretation*: vector ellipse tools produce exactly the curve he says to avoid. Organic subjects drawn with primitive shapes will look stiff.
- *Implication for vector software*: (a) an "organic ellipse" option: an ellipse whose curvature varies (e.g. a superellipse, or an egg-like curve with unequal ends; see SP-XI-5). (b) A lint that notes perfect circles or ellipses tagged as organic subjects, as information only. (c) Agent guidance: after placing primitives for natural forms, convert them to paths and vary the curvature. *Limits*: the tool cannot know what a shape is meant to depict unless the agent or user says so.
- *Do not algorithmize because*: the subject label (natural object or geometric emblem) has to come from the user or agent.
- *Disposition candidates*: engine-feature (egg/ovoid primitive); agent-prompt-or-resource; analysis-overlay (perfect-circle note).
- *Science caveat*: aesthetic opinion.

### SP-XI-5: Curvature variety ladder: circle, ellipse, egg
- *Author says*: a circle seen in perspective becomes an ellipse, which has more variety, but its four quarters are alike. The most beautiful symmetrical curve may be the egg of the Erechtheum's egg-and-dart moulding: its curvature changes everywhere yet flows almost as evenly as a circle. It is "roughly" the line of a face contour (Gainsborough, Van Dyck). “at no point is its curving at the same ratio as at any other point” (pp. 138–139).
- *Figures*: **Diagram VII** (Egg and dart from an Erechtheum caryatid). A straight fillet line runs across the top. Below it hang three ovoid "eggs". Each is symmetric about a vertical axis, broadest and flattest near the top, and narrows to a tighter rounded base, so curvature increases steadily from shoulder to base. Each egg sits inside a U-shaped shell drawn as 2–3 parallel offset contours that open at the top. Between the shells hang "darts": 3–4 nested parallel strokes forming a narrow V from the fillet down almost to the bottom band. At the bottom is a slightly curved band (astragal) with two oval beads and short reels. *To reproduce*: an egg can be built as an ellipse whose lower half has a smaller semi-axis, or as a Bézier with unequal top and bottom tangent lengths, mirrored about the vertical axis.
- *Conditions / counter-examples*: this applies to symmetrical figures. The egg's continuity "approaches the circle".
- *Our interpretation*: a concrete target: **symmetric, smooth, monotonically varying curvature** (no inflections on each half), unlike the constant curvature of a circle and the fourfold repeat of an ellipse.
- *Implication for vector software*: an "ovoid" primitive with a taper parameter (0 = ellipse). In the curvature overlay, report "quarters alike" for ellipses. As a practice exercise, recreate the egg-and-dart band as repeated symbols. *Limits*: "most beautiful" cannot be measured. The tool only describes how curvature is distributed.
- *Do not algorithmize because*: n/a for the primitive. The beauty ranking is the author's opinion.
- *Disposition candidates*: engine-feature (ovoid primitive); practice-exercise (egg-and-dart band); analysis-overlay (curvature symmetry readout).
- *Science caveat*: "most beautiful symmetrically curved figure" and the link to graceful portraits are opinion.

### SP-XI-6: In a profile, variety should serve the run of the contour
- *Author says*: profile lines tend to have too much variety for their unity. The finest profiles subordinate variety to the "run" of the contour. He guesses the Greeks removed the hollow above the nose to lengthen the run from forehead to nose tip, and doubts this was ever a common type. “The most beautiful profiles are usually those in which variety is subordinated to the unity of the contour.” (pp. 139–140).
- *Figures*: none.
- *Conditions / counter-examples*: the "Greek profile" is idealisation, not observation (p. 140).
- *Our interpretation*: on a complex contour, a strong overall sweep should carry the small bumps. The many details should read as one gesture.
- *Implication for vector software*: an overlay fits a smooth low-order "run" curve (heavily smoothed copy) to a selected contour and shows how far the detail departs from it. A large departure means the detail dominates the run. *Limits*: the smoothing scale is arbitrary, and the overlay cannot say whether the departures are intended.
- *Do not algorithmize because*: how much departure is right is judgement. Do not "correct" faces toward a Greek ideal.
- *Disposition candidates*: analysis-overlay (run curve with departure band); agent-prompt-or-resource ("feel the run before the bumps").
- *Science caveat*: claims about Greek types are the author's speculation.

### SP-XI-7: Draw the defining edge from its quieter side
- *Author says*: when painting a profile, paint all the variety on the face side first, blur that edge, then draw the profile line from the background side in one confident stroke, because the background varies little and does not interrupt the swing. Note where the edge is sharp and where it is lost. “this run or unity of the line is the thing to feel” (p. 141).
- *Figures*: none.
- *Conditions / counter-examples*: this is painting technique (brush, wet paint).
- *Our interpretation*: in vector terms, a figure's silhouette can be defined by the background shape's edge (cut or mask) drawn as one long path, rather than assembled from internal details.
- *Implication for vector software*: workflow: draw the internal detail slightly over the edge, then define the silhouette with one continuous path used as a clipping mask or background cut-out. That keeps one clean run plus deliberate lost and found edges (via feathered masks). *Limits*: this is a procedure, not a check.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: multi-step-workflow (silhouette-by-mask); agent-prompt-or-resource.
- *Science caveat*: none.

### SP-XI-8: Variety in Symmetry: opposed hollows and fullnesses
- *Author says*: limbs are roughly symmetrical, but each side varies, and the two sides vary in opposition: a convex curve on one side usually faces a concave one on the other. Finding more of this "will often improve a poorly drawn part" (p. 141). The body is rarely held symmetrically, and any action produces this variety. The principle is meant for when things "won't come". “Supposing there is a convex curve on the one side, you will often have a concave form on the other.” (p. 141). “The body is seldom, except in soldiering, held in a symmetrical position.” (p. 143).
- *Figures*: **Diagram VIII**: an arm drawn diagonally from the hand at lower left to the shoulder at upper right (about 40° from horizontal). A thin straight axis line runs through its length. Letters along both contours mark hollows (A) and fullnesses (B). They alternate along each contour, and each A faces a B on the opposite contour. **Diagram IX**: three figures, each with a straight vertical axis line: (1) a leg in front or three-quarter view, (2) a leg in side view, (3) a standing nude from behind, weight on one leg, one hand on hip, the other arm raised to the head. In each, A on one contour faces B on the other at the same height. *To reproduce*: two roughly parallel contours either side of an axis whose curvature sign is mostly opposite at matching positions along the axis (left concave where right is convex), with the bulges staggered.
- *Conditions / counter-examples*: "roughly speaking" symmetrical, and "often", not always.
- *Our interpretation*: for paired contours of a tubular form, the pattern is an anti-phase relation of curvature along the axis. Mirror-symmetric paired contours look dead.
- *Implication for vector software*: an analysis for a pair of contours, or one closed shape with a user-set axis: sample signed curvature on both sides at matching positions along the axis and show the in-phase versus anti-phase pattern (a two-band plot along the axis). Agent guidance: avoid `object.repeat.mirror` (present in the command list) for organic limbs, or break the mirror afterwards. *Limits*: needs a chosen axis. It describes, it does not judge anatomy, and it does not apply to everything with two sides.
- *Do not algorithmize because*: whether a particular pose calls for opposition is observation of the model.
- *Disposition candidates*: analysis-overlay (opposition plot); agent-prompt-or-resource (don't mirror organic forms); practice-exercise (redraw a mirrored limb with staggered hollows and fullnesses).
- *Science caveat*: an anatomical observation stated loosely ("often"). Not quantified.

### SP-XI-9: Vary line thickness and accent
- *Author says*: a line of even thickness is "dead and inexpressive" next to one that is varied and stressed. Where a natural boundary draws attention, darken and sharpen the line. Where the contour is nearly lost, make it soft and blurred. Much of a drawing's vitality depends on this. “A line of equal thickness is a very dead and inexpressive thing compared with one varied and stressed at certain points.” (p. 143).
- *Figures*: none.
- *Conditions / counter-examples*: "must be studied first hand" (p. 143).
- *Our interpretation*: line drawing has two expressive channels, width/darkness and sharpness, and their placement should follow how prominent the contour is.
- *Implication for vector software*: the command list already has `stroke.widthPoint.set` and `stroke.widthProfile.*` (improve-existing). Possible additions: per-segment stroke **softness** (feathered edge or opacity taper along the path), and an analysis flag for uniform-width strokes (coefficient of variation of width = 0). Agent workflow: after the line drawing, place accents (wider, darker, sharper) at chosen points and taper elsewhere. *Limits*: a uniform-width flag is not a fault in technical drawing, icons or type.
- *Do not algorithmize because*: **where** accents go depends on what the drawing should stress. Automatic random width variation is not what he describes.
- *Disposition candidates*: improve-existing (width profiles); engine-feature (softness/opacity along a stroke); analysis-overlay (uniform-width note); practice-exercise.
- *Science caveat*: "dead" is aesthetic opinion.

---

## XII Rhythm: Unity of Line

**Locator**: begins in h-1 (4614–4660) and continues in h-2 (from 4661). Anchors `[#CHAPTER_XII]`, `[#Page_144]` to `[#Page_184]` (missing 152–153, 158, 164–166, 168, 170, 174, 178: plate or diagram pages). Sub-anchors: `#plate31`–`#plate42`, `#diagram10`–`#diagram22`. Sidenotes: "Parallelism" (p. 145), "The Horizontal and the Vertical" (p. 149), "The Right Angle" (p. 155), "Curved Lines" (p. 162). The "XIII VARIETY OF MASS" text at line 4664 is the XHTML `<title>` of file h-2, not a heading. Chapter XII continues.

### SP-XII-1: Unity of line is the larger quality; lines flow through the panel, real or imagined
- *Author says*: unity of line needs a larger grasp and is rarer than variety. It is about the relation of parts to the whole, which is really the whole of Composition. Most compositions have a rhythmic flow of lines that may be actual lines, implied lines that link or mass parts, or only "a certain stress and flow in the forms", like the melodies of a symphony. “Unity of line is a bigger quality than variety, and as it requires a larger mental grasp, is more rarely met with.” (p. 144). “they may be only imaginary lines linking up or massing certain parts” (p. 144).
- *Figures*: Diagrams XV and XVI (SP-XII-20).
- *Conditions / counter-examples*: the flow is "more gracious and easy, or more vigorous and powerful" depending on the subject (p. 145).
- *Our interpretation*: the important lines are often not objects in the file. They are paths the eye infers through several objects.
- *Implication for vector software*: a non-printing **rhythm-line layer** (guides that can be curves). `guide.add` and `guide.setLayer` exist in the command list. Whether curved guides are supported was not checked. Agents could draw the intended flow lines first and then place objects along them.
- *Do not algorithmize because*: imagined lines are perceptual inferences. A tool can propose candidate links (SP-XII-2) but cannot know which ones a viewer sees.
- *Disposition candidates*: engine-feature (curved non-printing guides); multi-step-workflow (flow lines first); design-principle.
- *Science caveat*: the symphony analogy is a metaphor.

### SP-XII-2: Carry contours across gaps; remote parts linked by continuation
- *Author says*: a contour in one place is often "picked up" by another object's contour elsewhere. With no actual line between them, this sets up unity. Look for it in whole compositions and in a single figure, head or hand. “nothing serves to unite a picture like this relationship of remote parts.” (p. 145).
- *Figures*: Diagram XV (dashed lines link the wind gods' contours to Venus's head and on to the right-hand figure) and Diagram XVI (dashed arcs); see SP-XII-20.
- *Conditions / counter-examples*: none stated.
- *Our interpretation*: geometrically, path A's end tangent, extended, meets path B and joins it roughly tangentially (same direction and similar curvature).
- *Implication for vector software*: a **continuation finder**: for each open path end, or each contour segment that ends at an occlusion, extrapolate the end (tangent plus curvature, as a circular-arc extension) up to a length limit, and report other contours it meets within an angle tolerance (e.g. ±10°). Draw these as dashed overlay lines, like Speed's dashes. Also agent guidance: when placing an object, consider aligning one of its contours with the extension of another. *Limits*: extensions are geometric guesses, and false positives are common in dense art. It reports possible links, not perceived ones.
- *Do not algorithmize because*: which links matter is judgement. Never auto-move objects to create links.
- *Disposition candidates*: analysis-overlay (continuation finder); agent-prompt-or-resource; practice-exercise (link two figures through one implied contour).
- *Science caveat*: the claim that the eye completes contours across gaps resembles the later Gestalt "good continuation" idea, but the strength Speed claims ("nothing serves … like") is opinion.

### SP-XII-3: Parallelism sustains and amplifies a line's quality, for simple lines only
- *Author says*: groups of parallel lines produce a sustained effect, like a sustained organ chord compared with a staccato one. It steadies and unites. It only works with the simplest lines (straight or simple curves), never with complex shapes except in decorative pattern. Blake is "a mine of information" (p. 146). “This parallelism can only be used successfully with the simplest lines, such as a straight line or a simple curve” (p. 145).
- *Figures*: **Plate XXXI**: four Blake *Job* plates. A (Plate I, "Thus did Job continually"): a family kneeling under a tree, the figures' upright lines parallel, sheep backs a row of parallel horizontals in front. B (Plate IV): a messenger running in on a strong diagonal from the left at a seated couple, sheep backs horizontal. C (Plate XXI): standing figures in a row holding upright instruments, a forest of parallel verticals, sheep lying in front. D (Plate X, "laughed to scorn"): three friends whose outstretched arms and fingers point in parallel at Job. **Plate XXXII** E (Plate II): stacked curved figures either side of the throne, repeating arcs.
- *Conditions / counter-examples*: not with complicated shapes, except "in decorative patterns".
- *Our interpretation*: parallel repetition multiplies whatever feeling the single line carries. Its value drops as the line gets complex.
- *Implication for vector software*: a **parallel-family detector**: cluster straight segments (and simple arcs by curvature and centre) by orientation within ±3–5° and report families with total length and member count. Overlay them in one colour. Generation helpers: `object.path.offsetPath` and `object.blend.*` (both in the command list) already produce offset or parallel families. *Limits*: the detector counts geometric parallels, not implied ones (e.g. the direction of a figure), and does not weigh visibility.
- *Do not algorithmize because*: n/a for detection. The emotional amplification is not measurable.
- *Disposition candidates*: analysis-overlay (parallel families); improve-existing (offset/blend as "echo" helpers); agent-prompt-or-resource ("parallel only simple lines").
- *Science caveat*: the claimed emotional amplification is opinion.

### SP-XII-4: More complete parallelism, more emphasis; carry the direction into secondary forms
- *Author says*: in the last *Job* plate (XXI) the parallelism is more complete than in Plate I, giving "greater exaltation and peace". Blake carries the verticals of the kneeling figures into the sheep in front by adding small bands of vertical shading. Parallel pointing arms intensify scorn (Plate X). “the greater completeness with which the parallelism has been carried out has given a much greater emphasis to the effect” (p. 146).
- *Figures*: Plate XXXI A versus C. C has more and longer parallel verticals. In A, the sheep at front have short vertical shading strokes that continue the kneeling figures' verticals. D: four or five parallel arm lines converge on Job.
- *Conditions / counter-examples*: see SP-XII-3.
- *Our interpretation*: emphasis grows with how many elements join the family and how closely they agree. Hatching direction can join secondary forms to the family.
- *Implication for vector software*: a hatching or fill-pattern option that **inherits a chosen direction family**: set the hatch angle from a selected guide line. The parallel-family overlay could report "coverage" (share of total stroke length in the dominant family). *Limits*: coverage is a count and implies nothing about feeling.
- *Do not algorithmize because*: how much emphasis suits the subject is judgement.
- *Disposition candidates*: engine-feature (hatch angle from a reference path); analysis-overlay (family coverage); practice-exercise (same scene at 30% and 80% parallel coverage).
- *Science caveat*: "greater exaltation" is opinion.

### SP-XII-5: Balance unequal groups with line and tone weight, not by moving figures
- *Author says*: in *Job* Plate XVIII, three figures kneel on the right and one on the left. Blake balances them by losing the outline of the third right-hand figure, giving the single left figure a double line (its hair outline), shading it more strongly, and turning Job's head and stance slightly left. “By losing the outline of the third figure on the right and getting a double line out of the single figure on the left” (p. 147).
- *Figures*: **Plate XXXII G** (Plate XVIII): Job stands centre, arms spread wide (a strong horizontal T with the vertical body), before an altar of square stones. One stooped kneeling figure on the left, a stooped group on the right. Their backs repeat the same curve (the "repetition of the line of the back in stooping figures").
- *Conditions / counter-examples*: he notes it "does not show so well" in the reproduction (p. 147).
- *Our interpretation*: visual weight = number of contours × contrast × darkness, and can be tuned without moving anything.
- *Implication for vector software*: a **visual-weight readout** for selected groups: summed edge length × local contrast, plus dark area. It is a rough proxy for comparing left and right. Agent guidance: to balance, try adjusting contour emphasis (stroke width, outline loss, shading) before moving figures. *Limits*: perceived weight also depends on subject interest (chapter XV, outside this range). The proxy is crude.
- *Do not algorithmize because*: balance is perceived. The readout informs and must not dictate.
- *Disposition candidates*: analysis-overlay (weight proxy per region); agent-prompt-or-resource; cross-reference to ledger D (XV Balance).
- *Science caveat*: none specific. Treat the weight proxy as a heuristic.

### SP-XII-6: Maximum-unity lines (straight, circle, square) carry sublimity and strength, but still need "dither"
- *Author says*: mathematically straight lines and perfect circles never appear in good drawing, but their maximum unity gives them power where sublimity or deep feeling is wanted: rows of Greek columns, Gothic vertical clusters. Their "dither" comes from the entasis of columns and steps and the rough cutting of Gothic stone. Classic buildings are unity-dominant (one conception, high finish). Gothic is variety-dominant (each craftsman varies capitals and cusps). Norman strength comes from circular arches, straight lines and square ornament. Strength generally reads in straight, square contours (strong man, square brow and jaw, rocky hills). “where the expression of sublimity or any of the deeper and more profound sentiments are in evidence, they are often to be found.” (p. 147). “All objects with which one associates the look of strength will be found to have straight lines in their composition.” (p. 148).
- *Figures*: none.
- *Conditions / counter-examples*: "never found in good artistic drawing" as mathematically exact forms. Even sublime work needs subtle play.
- *Our interpretation*: (a) "Near-straight" (a very slight curve or irregularity) is the target, not exactly straight. (b) Squareness = strength, roundness = softness, as a style vocabulary.
- *Implication for vector software*: a "near-straight" option for the line tool: a straight line with a tiny bow or width irregularity, seeded and bounded, documented as "dither". Agent guidance: for strength or monumentality, use straighter contours and square corners. For softness, use full curves (see SP-XII-14). *Limits*: the style vocabulary is cultural convention, not a law.
- *Do not algorithmize because*: the amount of dither must stay small and deliberate (SP-XIV-17 warns against mechanical jogging).
- *Disposition candidates*: engine-feature (subtle bow/irregularity on lines); agent-prompt-or-resource (strength/softness vocabulary); design-principle.
- *Science caveat*: "look of strength", "mental power" of a square forehead, and sublimity are 1913 opinion. Do not repeat the forehead and character claims as fact.

### SP-XII-7: Horizontal = calm and contemplation; vertical = elevation; both tie to the frame
- *Author says*: the horizontal goes with calm and contemplation (sea horizon, desert plain, ripple bars, sunset bands). The vertical goes with elevation and the sublime (stone pine, cypress, spires, a lone column, even a factory chimney). Parallel to the frame, they give stability and a "well-knit look". “the horizontal being associated with calm and contemplation and the vertical with a feeling of elevation.” (p. 149).
- *Figures*: see SP-XII-8 and -9.
- *Conditions / counter-examples*: associations may modify the effect (the chimney is impressive "in spite of its unpleasant associations").
- *Our interpretation*: the share of horizontal versus vertical direction is a primary compositional choice tied to mood.
- *Implication for vector software*: the direction histogram (SP-XI-3) reported as "horizontal share, vertical share, oblique share". Agent guidance: pick a dominant direction for the subject's mood on purpose, and record the choice in the plan. *Limits*: the histogram counts geometry and cannot read mood.
- *Do not algorithmize because*: the line-to-emotion mapping is the author's claim, not a model to apply.
- *Disposition candidates*: analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: **strong caveat**. Fixed emotional meanings for horizontals and verticals are 1913 opinion and cultural convention, not verified science.

### SP-XII-8: Diagram X: horizontal calm disturbed step by step (sea and sky)
- *Author says*: A is six straight lines across a rectangle, and he thinks it suggests a calm sunset at sea. B adds a sun disc (the least varied curve, so still calm), a vertical figure (agrees with the horizontals) and curving cloud lines ("the dying energy of departing day"). C bends the figure, half covers the sun and curves the clouds more: "our calm evening has become a windy one". “A is nothing but six straight lines drawn across a rectangular shape” (p. 150).
- *Figures*: **Diagram X**, measured from the image. **A**: landscape rectangle about 1.6:1. Six horizontals, as fractions of frame height from the top and width: y≈0.22 (x 0.08–0.96), 0.53 (0.06–0.84), 0.61 (0.03–0.91), 0.665 (0.02–0.95), 0.69 (edge to edge: the horizon), 0.90 (0.02–0.96). Uneven spacing, lines bunched just above the horizon. **B**: same frame. Horizon at about 0.6 and one sea line about 0.85. In the sky, two long shallow arcs rising left to right, the upper one ending in a tight hook at the right. A small circle (sun, about 6% of width) at about 0.25 width just above the horizon. A narrow solid vertical blade (figure, about 3% of width, about 25% of height) at about 0.75 width, standing across the horizon. **C**: the cloud arcs curve more and form an elongated elliptical swirl curling round at the right. The sun circle is cut by a cloud line so only part shows. The figure is a leaning crescent.
- *Conditions / counter-examples*: B's curves are "not yet enough to destroy the impression".
- *Our interpretation*: a controlled experiment with three variables: line curvature, the verticality of an accent, and completeness of the static circle.
- *Implication for vector software*: a **practice exercise / agent calibration set**: generate A, B and C exactly from the numbers above and ask the user or agent to describe the differences. It is also a test case for the curvature and direction overlays (A: ~100% horizontal, zero curvature; C: higher curvature, an oblique accent). *Limits*: the numbers come from one 1913 diagram, not a standard.
- *Do not algorithmize because*: the "calm" or "windy" reading is the author's.
- *Disposition candidates*: practice-exercise; analysis-overlay (test fixture; must be authored from scratch, see cross-cutting note on assets).
- *Science caveat*: emotion claims are opinion.

### SP-XII-9: Diagram XI: verticals (pines) to swing to struggle
- *Author says*: D is a row of pines on a plain: "exaltation and infinite calm". E adds foliage giving a swinging line, carried on by a matching line in the sky: "some life and variety". F bends the trees and destroys the vertical: energy, "the stress and struggle of the elements". “If we entirely destroy the vertical feeling and bend our trees, as at F, the expression of much energy will be the result” (p. 151).
- *Figures*: **Diagram XI**, measured. **D**: landscape rectangle. Eight thin verticals at about x = 0.13, 0.18, 0.32, 0.55, 0.66, 0.83, 0.87, 0.91 of width, from the top edge to about 0.93 of height, unevenly spaced (a pair, singles, a close triplet). **E**: the same trunks topped by irregular dark blob masses (foliage) in a band along the top, with a larger cluster at upper right and a few lower blobs. One shallow arc rises left to right at about 0.65 height (the "swinging line"), and the upper edge of the foliage band forms a matching swing. **F**: the trunks are curves bowing and leaning right, more toward the top, like wind-bent trees. The foliage masses are heavier and swept right, with two arcs across the middle.
- *Conditions / counter-examples*: a little variety (E) adds life without losing calm.
- *Our interpretation*: the same three-step ladder for verticals. Uneven spacing keeps even D from looking mechanical.
- *Implication for vector software*: an exercise set like SP-XII-8. It is also a test that a "bend" or warp tool applied with increasing strength moves the direction histogram from vertical to oblique. *Limits*: as SP-XII-8.
- *Do not algorithmize because*: the emotional reading is the author's.
- *Disposition candidates*: practice-exercise; analysis-overlay (fixture).
- *Science caveat*: emotion claims are opinion.

### SP-XII-10: Straight lines steady rich curves
- *Author says*: the "aloofness of straight lines" from the fuss of variety lets them steady exuberant forms. The Venetians used this. Giorgione's *Fête Champêtre* has a seated woman's contour forced into a rigid straight line, stressed by her flute. Without it and the squarish drapery the richness would turn "sickly, if not positively sweet". Van Dyck put a hard straight line near the head, sometimes ending a dark background abruptly against a light distance to stress it. Titian's *Bacchus and Ariadne* is steadied by horizontal sky lines and vertical trunks. “went out of his way to get a straight line to steady his picture and contrast with the curves.” (p. 151).
- *Figures*: **Plate XXXIII** (Giorgione): the seated nude at right, seen from behind, has a back contour from shoulder to hip that runs as an almost straight slightly oblique line, and the flute is a straight short diagonal at her face. The well's edge at left gives more straight lines. **Plate XXXIV** (Titian): horizontal cloud bands in the left sky and vertical tree trunks at upper right frame a mass of swinging figures.
- *Conditions / counter-examples*: this matters most where forms are "rich" and full.
- *Our interpretation*: a ratio idea: a curve-heavy design needs some straight elements as a counterweight. It need not be a separate object. A contour can be straightened.
- *Implication for vector software*: in the line-rhythm summary, report the **straight versus curved length share**. When it is near 100% curves, an agent can consider straightening part of an existing contour (a "flatten segment" helper: make a selected curve segment nearly straight while keeping its endpoints). *Limits*: the share does not know which forms are "rich".
- *Do not algorithmize because*: where to put the steadying line is judgement.
- *Disposition candidates*: analysis-overlay (straight/curve share); engine-feature (flatten-segment helper, possibly near `path.smoothRegion`/`path.curvatureEdit`, not checked); agent-prompt-or-resource.
- *Science caveat*: "sickly, if not positively sweet" is opinion.

### SP-XII-11: Contrast compensates for constraints, and the occlusion test
- *Author says*: in Titian's *Bacchus and Ariadne*, Bacchus's forward leg had to stay vertical, since moving it right would throw the figure group onto Ariadne and upset the balance. So the heavy standing leg of the cymbal-playing Bacchante is there to make Bacchus's leg look light by contrast. He finds this by covering the leg with a finger. “the duty of the aggressive standing leg of the leading Bacchante, with its great look of weight, is to give a look of lightness” (p. 155). Method: “if you cover it over with your finger and look at the composition without it” (p. 154).
- *Figures*: Plate XXXIV. Bacchus leaps from the car at centre-left, his forward leg near vertical over the wheel. The cymbal player at centre-right stands on a firmly planted leg.
- *Conditions / counter-examples*: this is the author's reading, offered as "an idea that occurred to me" (p. 154).
- *Our interpretation*: (a) one element can exist to change how another reads. (b) **Hiding an element and looking again** is a valid analysis method.
- *Implication for vector software*: a **temporary hide / A-B compare** workflow: toggle an element or group and compare thumbnails side by side. `object.hide` and `layer.hideOthers` exist. A one-key "peek without" toggle that does not touch the undo history would help users and agents. Agents can render with and without an element via the control channel (`ui.render`) to reason about its role. *Limits*: comparing two renders shows a difference. It does not say which is better.
- *Do not algorithmize because*: what an element "does" for the whole is perceptual judgement.
- *Disposition candidates*: multi-step-workflow (with/without compare); improve-existing (non-destructive "peek without" toggle); agent-prompt-or-resource.
- *Science caveat*: none.

### SP-XII-12: Implied verticality ("straight-up-ness") counts as a vertical
- *Author says*: a "straight-up-ness" in a figure, or a sense of a horizontal plane, works like an actual vertical or horizontal line. Blake's "Morning Stars" figures have no upright line yet form a "vertical chord", with "a flame-like line running through them" giving joy. “A feeling of straight-up-ness in a figure or of the horizontal plane in anything will produce the same effect as a vertical or horizontal line” (p. 155).
- *Figures*: **Plate XXXII H** (*Job* Plate XIV): a row of four angels across the top with raised arms, each figure as a whole tall and upright, though no single contour is a straight vertical.
- *Conditions / counter-examples*: none stated.
- *Our interpretation*: direction analysis must also consider an object's **principal axis** (its overall orientation), not only its edges.
- *Implication for vector software*: the direction histogram should have a mode that counts each object's principal axis (from second moments of its area), weighted by elongation, alongside the edge-tangent mode. *Limits*: principal axes are not meaningful for round objects, and the result depends on how objects are grouped.
- *Do not algorithmize because*: n/a for the axis computation.
- *Disposition candidates*: analysis-overlay (axis mode); design-principle (coined term "straight-up-ness").
- *Science caveat*: "joyous energy" is opinion.

### SP-XII-13: The right angle is the strongest, most arresting "chord"
- *Author says*: vertical crossing horizontal is one of the strongest, most attention-holding combinations and appears where there is dramatic power: the cross, a post against the sea horizon, a telegraph pole against a hill at sunset. A contracted brow (vertical furrows continuing the nose, crossed by the brow) looks powerful. So does a profile where a horizontal brow meets a vertical forehead. A forehead bulging in a curve loses the look, "although there is obviously more room for brains". “The combination of the vertical with the horizontal produces one of the strongest and most arresting chords that you can make” (pp. 155–156).
- *Figures*: **Diagram XII** ("Illustrating influence of the right angle" per the List of Diagrams). **A**: a frontal face oval with a horizontal brow line crossed by a short vertical double line (furrows) running down from it like a cross, and a short mouth dash. **B**: a profile whose forehead is a straight vertical meeting a horizontal brow at a right angle. **C**: the same profile with the forehead bulging as a convex arc, so there is no right angle.
- *Conditions / counter-examples*: its attention-drawing power is a problem at the frame corners (SP-XII-16).
- *Our interpretation*: right-angle junctions are attention hotspots. Use them where you want attention and avoid stray ones.
- *Implication for vector software*: a **junction finder**: report T and + junctions between long straight-ish strokes and edges with crossing angle 80–100°, marked on an overlay, so an agent can check that they sit at intended focal points. *Limits*: the finder sees only geometry. Implied right angles (SP-XII-14) are missed unless object axes are used.
- *Do not algorithmize because*: whether the drama is wanted is intent.
- *Disposition candidates*: analysis-overlay (right-angle junctions); agent-prompt-or-resource.
- *Science caveat*: "rivets the attention" and "look of power" are opinion. Strong phrasing ("more powerful … than any other") is unverified.

### SP-XII-14: A structural angle can be made with tone and edge emphasis, and line character can contrast between figures
- *Author says*: in Watts's *Love and Death*, Death's power comes from a right angle felt through the pose. Since the contour did not supply it, Watts "introduced it" by shading the farther arm, insisting on the light upper edge of the outstretched arm and hand, and losing the outline of the head beyond. Square drapery forms, the hard square steps (especially the strong horizontal of the first step) and the upright doorway support it. Love is all curves and soft, melting modelling, hung on "a rich full curve starting at the standing foot" of Death, supported by rounded roses. “Whereas the figure of Death is all square lines and flat crisp planes, the whole hanging on a dramatic right angle” (p. 157). Also: “Not getting it in the contour, Watts has boldly introduced it by means of shading the farther arm” (p. 157).
- *Figures*: **Plate XXXV**: a tall narrow canvas. Death, a draped figure seen from behind, fills the left half, its arm raised across to the right at top. Love, a small nude, pushes back at right among roses. Steps across the bottom. **Diagram XIII** (tall rectangle about 1:2): long verticals at left (door jamb and Death's back) running nearly full height. A long straight edge from Death's shoulder out to the right near the top (upper edge of the arm), meeting the verticals to form the angle. Drapery drawn as angular straight strokes. Two or three horizontals at the bottom (steps) with a short riser. On the right, a long sweeping curve from the lower centre up to Love's head with a dashed parallel outside it. Small C-shaped curls along the right edge (roses).
- *Conditions / counter-examples*: he notes his diagrams are "ugly", like anatomy drawings.
- *Our interpretation*: (a) structural lines can be formed by **contrast edges**, not just strokes. (b) Pair figures with supporting textures of the same character (square with square, round with round).
- *Implication for vector software*: agent workflow: to strengthen an intended structural line, raise the value contrast along that edge, sharpen it, and soften competing edges nearby. The edge-contrast overlay (SP-XIII-9) shows where the strong edges actually are, and the structure guide shows where they are meant to be. *Limits*: the overlay shows the gap between guide and result, not whether the result works.
- *Do not algorithmize because*: the meaning of the contrast (death versus love) is the subject.
- *Disposition candidates*: multi-step-workflow (guide line, then edge emphasis, then compare); analysis-overlay; agent-prompt-or-resource (match texture character to figure character).
- *Science caveat*: emotional readings are opinion.

### SP-XII-15: "anatomy of compositions" diagrams: trace the hidden lines over the work
- *Author says*: his diagrams show some main lines of a composition's "anatomy". They are not always obvious in the original and are "justly much broken into by truths of natural appearance", but an abstract arrangement underlies every good picture, "carefully hidden" by great artists. Students should trace the diagrams and lay the tracing over the reproduction. He admits other important lines may have been missed. “If the student will trace them and put his tracing over the reproductions of the originals” (p. 159).
- *Figures*: all the line diagrams in this chapter (XIII, XV, XVI, XVII, XX).
- *Conditions / counter-examples*: his choice of lines is admittedly partial ("Other lines … may have been overlooked", p. 159).
- *Our interpretation*: this is an overlay feature described in 1913: a translucent structure layer over the artwork.
- *Implication for vector software*: a **composition-anatomy layer**: non-printing and translucent, drawn by the user or agent, toggleable over the art (`layer.outlineOthers` and guide layers in the command list may help). Agents could analyse a user's artwork by proposing an anatomy sketch as a suggestion layer, labelled as one reading among others. *Limits*: any auto-extracted anatomy is a proposal. Speed's own lines are selections, not unique facts.
- *Do not algorithmize because*: choosing which lines are the anatomy is interpretation.
- *Disposition candidates*: engine-feature (anatomy layer type); multi-step-workflow; practice-exercise (trace masters' structures, using public-domain images with attribution).
- *Science caveat*: none.

### SP-XII-16: The panel shape is the first given; corners attract the eye; lines lead the eye
- *Author says*: the panel shape is fixed before you start, and every line relates to it. Verticals and horizontals are "always right" in a rectangle. Each corner is a right angle and draws the eye where you don't want it, so fill corners with a dark mass or with lines that swing past and carry the eye to the centre. Squares are hardest because leaving the centre puts you in a corner. Students should practise filling a square. “There is one condition in a composition, that is laid down before you begin, and that is the shape of your panel or canvas.” (p. 159). “lines have a power of directing the attention, the eye instinctively running with them” (p. 160). “It is a good exercise for students to give themselves a square to fill” (p. 160).
- *Figures*: none.
- *Conditions / counter-examples*: in a normal rectangle the corner problem shows up only after the centre is arranged.
- *Our interpretation*: (a) set the artboard first, and fix its aspect before composing. (b) Check corners for line endings and right-angle junctions that pull attention outward.
- *Implication for vector software*: a **corner and exit check**: within the corner zones (e.g. 12% of the shorter side), report stroke ends, junctions and strong edges pointing into the corner, plus lines whose extension leaves the artboard near a corner. Square-fill practice template. Agent guidance: set the artboard size and aspect before drawing, and do not resize it casually afterwards. *Limits*: points at geometry only and does not track gaze.
- *Do not algorithmize because*: whether a corner accent is wanted is intent.
- *Disposition candidates*: analysis-overlay (corner zones); practice-exercise (fill a square); agent-prompt-or-resource (artboard first).
- *Science caveat*: "the eye instinctively running with them" and the corner's right angle "always attracts the eye" are claims about eye movement made without evidence in 1913. Treat them as heuristics.

### SP-XII-17: Diagonals relate to the frame; the "general swing" counts, not literal lines
- *Author says*: diagonals also relate to a rectangle, and many compositions hang on one. "Vertical" and "horizontal" rarely mean naked lines (no pure vertical in a cypress), only the general swing. Velázquez's *Surrender of Breda*: a vertical chord of spears at left continued in the horse's leg and the key-receiver's front leg. A horizontal of the distant city continued by a gun on a shoulder (oddly placed, so deliberate). The flag line at right exactly parallel to the top-right-to-bottom-left diagonal, the sword nearly parallel to it, and a horse's back leg on the other diagonal. “the general swing is across the panel in harmony with one or other diagonal” (p. 161).
- *Figures*: **Plate XXXVI**: dense upright spears at right and a few at left, a horse's rump at right, two commanders at centre exchanging the key, a city skyline low in the distance, a flag slanting at right.
- *Conditions / counter-examples*: the artifice should not be "too apparent" (p. 162).
- *Our interpretation*: test relations to the artboard diagonals as well as the edges. The relation works through overall direction across several objects.
- *Implication for vector software*: the frame tab of the direction histogram includes **artboard-diagonal bins** (angles of the two diagonals for the actual aspect ratio). Show the diagonals as optional guides and highlight segments within ±3° of parallel to either one. The ruler test he describes is exactly this check. *Limits*: as SP-XI-3.
- *Do not algorithmize because*: n/a for measurement.
- *Disposition candidates*: analysis-overlay (diagonal parallels); engine-feature (show artboard-diagonal guides).
- *Science caveat*: "dignified look" is opinion.

### SP-XII-18: Curves give charm but need restraint and steadying; look for straightness in curves
- *Author says*: curves lack the "moral integrity" of straight lines. They hold charm, grace and exuberance but get out of hand without straights and flats (rococo, late Rubens). The finest curves "err on the side of the square forms rather than the circle". Unrestrained circular and volute curves are "gross". “The finest curves are full of restraint, and excessive curvature is a thing to be avoided in good drawing.” (p. 162). “Always be on the look out for straightnesses in curved forms and for planes in your modelling.” (p. 163).
- *Figures*: none.
- *Conditions / counter-examples*: Rubens's "best work is full of squarer drawing and planes".
- *Our interpretation*: a good organic curve often reads as a chain of slightly curved, near-straight runs joined by tighter turns, rather than uniform arcs.
- *Implication for vector software*: in the curvature overlay, show the share of each path at low curvature (near-straight runs) against high curvature, and flag long stretches of near-constant high curvature (circle-like volutes). A drawing aid could convert a selected smooth curve into "planes" (a near-straight segment chain with rounded joins) as a style option. *Limits*: thresholds depend on scale. Normalise curvature by object size.
- *Do not algorithmize because*: "excessive" is relative to the subject.
- *Disposition candidates*: analysis-overlay; engine-feature (curve-to-planes stylisation); agent-prompt-or-resource ("look for straightnesses in curves").
- *Science caveat*: "moral integrity", "gross" are opinion.

### SP-XII-19: Diagram XIV: more curvature, more energy (sea and sky with curves)
- *Author says*: the same sea and sky with curved lines: A, slightly curved, already shows some energy. B and C increase it. D, one vigorous swirl, is a "hurricane", roughly the rhythmic basis of Turner's *Hannibal Crossing the Alps*. “the increasing curvature of the lines increases the energy expressed, until in D, where the lines sweep round in one vigorous swirl” (p. 163).
- *Figures*: **Diagram XIV(1)** A: landscape rectangle with about nine nearly horizontal lines that undulate slightly, one with a small bump (distant hill or cloud), the horizon line a little heavier at about 0.6 height. C: cumulus outlines, scalloped convex lobes in two clusters across the upper half. A band of near-horizontal lines in the middle (sea). Waves along the bottom with curling crests. **Diagram XIV(2)** B: the lower lines wavier. Two long upper curves rising to the right, the upper one turning back in a hook. D: one large vortex. About five nested arcs sweep from upper right, down round the bottom and up the left side, with two upward lobes (cresting forms) at upper left.
- *Conditions / counter-examples*: compare Diagram X (straight lines, repose).
- *Our interpretation*: the scale runs from straight through gentle to scalloped to a single spiral. Mean absolute curvature and turning angle both rise along it.
- *Implication for vector software*: a test fixture and exercise as in SP-XII-8. A summary metric for a set of strokes: **total absolute turning per unit length** (∫|κ|ds / length). *Limits*: the metric rises with any wiggle, including noise and texture strokes, and cannot tell a swirl from fussiness.
- *Do not algorithmize because*: the "energy" reading is the author's.
- *Disposition candidates*: practice-exercise; analysis-overlay (turning-per-length readout).
- *Science caveat*: emotion claims are opinion.

### SP-XII-20: Continuous flow leads the eye to the main interest; stop the exits with uprights
- *Author says*: a simple, graceful form for the "tying lines" is a continuous flow, each line growing out of another, leading the eye to the main interests. Botticelli's *Birth of Venus* and Veronese's *Rape of Europa* are examples. Europa's curves are nearer to circles and would look gross bare, but they are hidden under rich natural detail. Botticelli's restraint is "infinitely more satisfying" as pure line. Both use straight lines to steady. Veronese keeps a horizontal feeling in the sky, horizon and sea edge, and adds two pyramids whose straight lines lead "straight on to the principal head". Botticelli echoes the horizon in the ground at lower right and puts straight upright trees at right as "a determined stand" against lines leaving the picture. “a continuous flow, one line evolving out of another in graceful sequence, thus leading the eye on from one part to another” (p. 163). “he has made a determined stand against the flow of lines carrying you out of the picture on the right” (p. 167).
- *Figures*: **Plate XXXVII / Diagram XV** (Venus). Landscape rectangle. Long sweeping curves enter from upper left (the wind gods) and travel across the top to the right. A small upright ellipse at top centre (Venus's head and hair) with a long descending curve. A large reclined loop in the lower centre-left formed by the shell, body and ground contours. Dashed arcs are imaginary continuations, e.g. from the lower left up to the top centre and over to the right-hand figure. At the right edge, three near-full-height straight verticals (solid and dash-dot: the trees). A short horizontal at left at about 0.4 height (horizon) and two short horizontals at lower right (ground). **Plate XXXVIII / Diagram XVI** (Europa). Many large arcs nearer to circles, solid and dashed. A cluster of small spiral curls at centre (heads). Big dashed circles in the right half (foliage). A long slightly curved line from top right-of-centre to bottom right (tree trunk). Two short straight lines converging upward at upper left of centre (the pyramid) pointing toward the main head. A horizontal at lower right (sea edge).
- *Conditions / counter-examples*: how bare the line structure can be depends on the medium and finish (tempera simplicity versus rich oil).
- *Our interpretation*: (a) build the main flow as one chained curve system that ends at the focal point. (b) Place blockers (straight uprights) where the flow would otherwise leave the frame.
- *Implication for vector software*: an **exit check**: extend the main flow lines (tangent at the artboard edge, or from the continuation finder) and flag where they leave the frame without a stopping element. An agent workflow: mark the focal point, draw the flow to it on the anatomy layer, then place objects. *Limits*: flow-line extraction is ambiguous, so present it as a proposal.
- *Do not algorithmize because*: what the "principal interest" is must come from the user.
- *Disposition candidates*: analysis-overlay (flow exits); multi-step-workflow; agent-prompt-or-resource.
- *Science caveat*: "leading the eye" is an unverified eye-movement claim. Treat as a heuristic.

### SP-XII-21: Flame-like flow: exalting but restless
- *Author says*: a flame-like flow of curves that meet, part, meet again or cross in continual upward movement. El Greco is the striking case: thrilling like watching a fire, "something exalting", but in excess it "suffer[s] from lack of repose", since nowhere does the eye rest. Blake uses flame lines with more steadying straights (*Job* III and V, not reproduced). “curved lines meeting and parting and meeting again, or even crossing in one continual movement onwards” (p. 167).
- *Figures*: **Plate XL** (El Greco, *Ascension*): Christ rises at top centre holding a banner on a long staff, with writhing figures below whose limbs and light masses flicker upward. **Plate XLI** (El Greco, *Baptism*): a tall panel, a heavenly host above in swirling light masses, Christ kneeling and John standing below. The light shapes are elongated flickering tongues.
- *Conditions / counter-examples*: balance and repose are "necessary in the treatment of even the most violent subjects" (p. 169).
- *Our interpretation*: braided, elongated, upward-tapering curves. Repose comes from adding some straights or flats.
- *Implication for vector software*: an agent style vocabulary entry ("flame flow: elongated S-curves that taper upward and braid") with the caution to add steadying straights. A brush or path effect could generate tapering braided strands along a guide. *Limits*: a style, not a check.
- *Do not algorithmize because*: the right dose of restlessness is judgement.
- *Disposition candidates*: agent-prompt-or-resource; engine-feature (strand/braid along a path, optional).
- *Science caveat*: "exalting", "thrills one" are opinion.

### SP-XII-22: Jagged, interrupted, clashing lines plus harsh tone contrast read as terror; check by flipping
- *Author says*: constant interruption of the flow and an angular, jagged clash of lines give terror and horror (fork lightning). Blake's *Job* XI was reproduced turned round so the horror can be seen to belong to the line music, not just the subject. Harsh tonal contrasts (hard lights against hard darks) do the same. Good battle pictures are full of such clashes. Uccello's *Battle of St Egidio* is milder: clashes are confined to small parts and there is no chiaroscuro. “a harsh jarring of one against another in an angular, jagged fashion, produces a feeling of terror and horror” (p. 171). “Harsh contrasts of tone produce much the same look of terror as harsh contrasts of line.” (p. 171). “I have had it put sideways on so that you may see that the look of horror is not only in the subject” (p. 171).
- *Figures*: **Plate XXXII F** (*Job* XI): writhing forms, a serpent, flames, hard light against hard dark. Its caption says "Printed the wrong way up". I could not confirm the printed orientation by eye. **Plate XXXIX / Diagram XVII** (Uccello): bundles of parallel lances in the upper left lean steeply and are crossed by a few long lances at opposing angles. Horse and armour contours are broken into short angular stepped segments. Foreshortened lances on the ground are short straight bits at many angles, converging toward a vanishing point (which he finds unintentionally comic).
- *Conditions / counter-examples*: Uccello shows a "milder edition" because the clashes are small in scale.
- *Our interpretation*: (a) many crossings at large angles, broken contours and high adjacent contrast are the geometry of agitation. (b) **Rotating or flipping the view** lets you judge the abstract design separately from the subject.
- *Implication for vector software*: a **view rotate/flip preview** (non-destructive, no change to the document) for checking design. `object.rotate` changes the document, so a view-only rotate is different (whether one exists was not checked). An analysis of **crossing density and crossing-angle histogram** plus **contour interruption count** (segments per unit length with sharp corners). *Limits*: high crossing density is also normal in hatching and texture, so let users exclude texture layers. The analysis cannot detect "terror".
- *Do not algorithmize because*: the emotional reading is opinion. Do not auto-label art as "horror".
- *Disposition candidates*: engine-feature (view flip/rotate for review); analysis-overlay (crossings, jaggedness); multi-step-workflow (flip check).
- *Science caveat*: line-to-terror claims are opinion. The flip test is a long-standing studio practice, not a validated measure.

### SP-XII-23: Radiating lines give unity, even from a centre outside the picture
- *Author says*: lines radiating in smooth curves from a common centre give unity. The centre may be well outside the picture. The feeling that they would meet if extended unites them. “the feeling that they would meet if produced gives them a unity that brings them into harmonious relationship.” (p. 172).
- *Figures*: Diagram XVIII F (rays from beyond the left edge); Turner's "feeling of radiating lines from the rising sun" (SP-XIV-15).
- *Conditions / counter-examples*: "smooth curves". He extends this beyond straight rays.
- *Our interpretation*: a computable relation: a family of lines whose extensions pass near one point.
- *Implication for vector software*: a **radiation detector**: for straight segments (and tangents at the ends of gentle curves), find points where many extensions pass within tolerance (a Hough-like vote over the plane, including outside the artboard) and report the clusters and their centres. A generation helper: `object.repeat.radial` exists in the command list. An extra option could radiate guides from an off-artboard centre. *Limits*: curved "radiation" is loosely defined, and votes from short segments are noisy.
- *Do not algorithmize because*: n/a for detection.
- *Disposition candidates*: analysis-overlay (radiation centres); improve-existing (radial guides from any centre).
- *Science caveat*: "harmonious" is opinion.

### SP-XII-24: Sympathetic radiation: relate unrelated lines by adding lines in sympathy with them
- *Author says*: Panel A has three lines drawn at random to be as unrelated as possible. In B, 1-2 is made dominant and radiating lines are drawn around it. 7-8 radiates with 3-4, 5-6 and 1-2 at once and so relates all three, and 9-10 adds to the relation with 1-2. Taking 3-4 (D) or another line (F) as dominant gives entirely different compositions from the same scribble. He proposes it as a game: one person draws two or three random lines, the other harmonises them. “It is this echoing of lines through a composition that unites the different parts and gives unity to the whole.” (p. 172). “I would suggest this as a new game to students” (p. 173).
- *Figures*: **Diagram XVIII** (enlarged and read). **A**: landscape panel. 1-2 is a gently convex curve from the upper left (~0.19w, 0.31h) down to the lower right (~0.86w, 0.67h). 3-4 is an S-curve from the bottom (~0.32w, 0.86h) up to the top (~0.62w, 0.18h). 5-6 is a short slightly convex stroke from the lower left (~0.28w, 0.60h) rising to the right (~0.70w, 0.43h). All three cross near (~0.54w, 0.44h). **B** (1-2 dominant): the upper right fills with 5–6 long roughly parallel curves sweeping from upper left to lower right, echoing 1-2. At the lower left a fan of curves (3-4, 7-8, 9-10 and others) springs from a shared base near (~0.25w, 0.85h) and opens up and to the right like a sheaf. 7-8 sweeps along the bottom to the bottom-right corner, and 9-10 arcs to the right-centre. **D** (3-4 dominant): curves spring from a base near the bottom centre-left and fan up and out to both sides like a fountain. 8 reaches the upper-left corner and 11/12 are on the right. **F**: lines fan rightward from a point at or beyond the left edge near mid-height, with one long arc across the top. *Note*: the text says "In Panel F, 1-2 has been taken as the dominant line" (p. 173), but the caption says "F. TAKING LINE 5-6 AS DOMINANT LINE".
- *Conditions / counter-examples*: masses are then needed to cover crossings (SP-XII-25).
- *Our interpretation*: an explicit procedure: (1) pick a dominant line. (2) Add lines that share a radiating centre or direction with both the dominant line and an unrelated one, as **bridges**. (3) Add echoes to strengthen the dominant swing. (4) Cover the remaining clashes.
- *Implication for vector software*: (a) **Practice game** in-app: generate 2–3 random strokes in a panel and let the user or agent harmonise them. (b) **Assistive tool**: given a chosen dominant line and another line, propose candidate bridging curves (e.g. arcs through a shared radiation centre within the panel) as suggestions on the anatomy layer. (c) A relational graph overlay showing which lines are "in sympathy" (shared centre or parallel) and which are isolated. *Limits*: "in sympathy" must be defined by tolerances (shared centre within r, angle within θ). The suggestions are geometric options and cannot judge harmony.
- *Do not algorithmize because*: the author shows that several valid solutions exist (B, D, F). Choosing the dominant line is the creative act, so tools should offer, not pick.
- *Disposition candidates*: practice-exercise (the game); analysis-overlay (sympathy graph); engine-feature (bridging-curve suggestions, optional); multi-step-workflow.
- *Science caveat*: "not at all unpleasing" is opinion.

### SP-XII-25: Near-right-angle crossings are harsh; use them for drama or cover them with a mass
- *Author says*: lines crossing at nearly a right angle are harsh and discordant. They are useful to draw attention dramatically to one spot and otherwise should be avoided or covered. In C he puts a mass over the clash of the original scribble and over the angles where 3-4 cuts the radiating lines above 1-2, then adds a small mass at 11 for balance. “The crossing of lines at angles approaching the right angle is always harsh and somewhat discordant” (p. 172).
- *Figures*: **Diagram XVIII C**: B plus a large irregular dark (foliage-like) mass covering the crossing area at upper middle and a small dark mass "11" near the lower-left edge. **E**: D plus large dark masses across the top like tree crowns on the fan of trunks, and a small mass at lower left. **G**: F plus a dark central mass over the crossing and a small mass at the right edge.
- *Conditions / counter-examples*: crossings are allowed where drama is wanted (compare SP-XII-13).
- *Our interpretation*: the crossing-angle histogram (SP-XII-22) gives the evidence. The fix is occlusion by a mass, then rebalancing.
- *Implication for vector software*: in the crossing overlay, mark crossings at 70–110° that lie away from any user-marked focal point. Agent guidance: hide unwanted crossings under a mass, then check balance (SP-XII-5). *Limits*: in vector art many crossings are invisible (under fills), so only visible stroke and edge crossings should count, which requires rendering-aware intersection.
- *Do not algorithmize because*: whether a crossing is a wanted accent is intent.
- *Disposition candidates*: analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: "harsh", "discordant" are opinion.

### SP-XII-26: Chains of sympathy and transitions (colour and music analogy)
- *Author says*: discordant givens (a commission's fixed elements) can be brought into harmony by stages through intermediate related lines, as discordant colours are harmonised by colours related to both, and as a musician modulates to a remote key through a few linking chords. In Diagram XIX H/J a straight line is related to 5-6 through 7-8 and 9-10. Exact radiating points are not needed when lines are "obviously in sympathy". 3-4 is linked through 11-12, and 13-14 strengthens several relations. “discordant notes can be brought into harmony by the introduction of others related to both the original colours” (p. 173). “although 5-6 and 9-10 do not radiate from the same point, they are obviously in sympathy.” (p. 176).
- *Figures*: **Diagram XIX H**: 1-2 is a straight diagonal from the lower left (~0.1w, 0.75h) to the upper-right corner (~0.96w, 0.17h). 3-4 is an arc from the upper left (~0.14w, 0.21h) bending down to (~0.62w, 0.84h). 5-6 is a hook that starts near 1, rises in a large arc toward the upper right and curls back to end at (~0.66w, 0.61h). **J** (H harmonised, 1-2 dominant): new lines spring from the lower-left region. 7-8 runs to the right edge. 9-10 curls into a small hook at centre-right, echoing 5-6. 11-12 rises from the bottom to the upper right. 13-14 curves from the bottom centre to the left edge. A dashed line at 5's end shows the course on which 5-6 would share 9-10's centre. Two or three free arcs at upper left echo 5-6. **L**: J plus dark masses at the top-right corner, left-centre and lower left.
- *Conditions / counter-examples*: in H, the straight line "somewhat increases the difficulties" (p. 176).
- *Our interpretation*: relation can be **transitive**. A line can be tied to another through a chain of pairwise relations, and loose tolerance is acceptable.
- *Implication for vector software*: the sympathy graph (SP-XII-24) should show connected components: isolated lines are candidates for a bridge. A colour counterpart: for two clashing swatches, suggest intermediate swatches related to both (interpolation in a perceptual colour space). `edit.colors.blend*` exists in the command list for interpolation along objects. *Limits*: graph tolerances are arbitrary, and the existence of a chain says nothing about harmony.
- *Do not algorithmize because*: which chain to build is a design choice.
- *Disposition candidates*: analysis-overlay (connected components); improve-existing (colour bridging suggestions); agent-prompt-or-resource.
- *Science caveat*: the music analogy is a metaphor.

### SP-XII-27: A line at dead centre and a lone vertical: break them up with parallels and echoes
- *Author says*: in panel I, two random lines are straight and at right angles, and the longer one runs across the centre. First "trick the eye out of knowing that this line is in the centre" by drawing parallels below it that lead the eye down to 9-10, which becomes more important and better proportioned to the panel. The lone vertical 3-4 is "stark and lonely", so add two verticals (11-12, 13-14). Two lines in sympathy with 5-6 lead back to the top edge. Masses complete it (M). “The first thing to do is to trick the eye out of knowing that this line is in the centre” (p. 176). “The vertical line 3-4 is rather stark and lonely” (p. 176).
- *Figures*: **Diagram XIX I**: 1-2 is a straight horizontal at mid-height (~0.08–0.93w). 3-4 is a short straight vertical at ~0.25w (~0.25–0.80h) crossing it at a right angle. 5-6 is a curve from the lower left (~0.1w, 0.7h) rising gently, crossing 1-2 at ~0.6w and hooking up to a curl at the top right. **K**: I plus horizontals 7-8 and 9-10 below 1-2 (at ~0.62h and ~0.70h, so 9-10 now dominates below the centre), two tall verticals 11-12 and 13-14 near the right edge (~0.85w, ~0.90w), an arc at the upper left and a long shallow arc along the top right echoing 5-6. **M**: K plus tree-like dark masses on 3-4 and on the two right verticals. It reads as a flat landscape with trees.
- *Conditions / counter-examples*: none.
- *Our interpretation*: two specific warnings: (a) a dominant line exactly on the centre line halves the panel. (b) A single isolated vertical needs company or support. Both can be fixed by echoes.
- *Implication for vector software*: an analysis note when a long horizontal or vertical (more than 50% of the artboard span) lies within ±2% of the artboard centre line, and a "lonely line" note for a long straight line with no parallel family member (SP-XII-3). *Limits*: centred lines are often intentional (symmetry, UI, logos). These are notes, not warnings.
- *Do not algorithmize because*: n/a for detection. The fix is design.
- *Disposition candidates*: analysis-overlay (centre-line and lonely-line notes); practice-exercise; cross-reference to ledger D (XVI Proportion).
- *Science caveat*: none beyond opinion about "better proportion".

### SP-XII-28: Long lines "pick out" and stress forms they echo (portrait, hair, necklace)
- *Author says*: lines (and colours, as with blue eyes and a blue dress) accentuate the forms they are in sympathy with. A picture hat's curve stresses eyes and brows, and a necklace stresses the eyes or the face oval depending on how high it hangs. In Burne-Jones's portrait, the widening circles of a round mirror swing round with the eyes and concentrate attention there, most on the eye nearest the centre of the circles. Masking the head with a hole in paper equalises attention between the eyes. The necklace and dress folds echo the face oval. Hair dragged up sharpens the profile, and a low knot blunts it and stresses fullness under the chin. “The influence of long lines is thus to "pick out" from among the lines of a face those with which they are in sympathy” (p. 177). “the widening circles of which swing round in sympathy with the eyes and concentrate the attention on them” (p. 177). “The hair dragged up high at the back sharpens the lines of the profile as the low knot blunts them.” (p. 182).
- *Figures*: **Plate XLII** (Burne-Jones, portrait of his daughter): seated girl, frontal; a large circular mirror behind her head reflects a window and her own head; a double necklace; hands folded low. **Diagram XX**: portrait rectangle. Two nested pairs of arcs, roughly concentric with the face, enclose it like parentheses from the top to about 0.5 height. A dashed oval round the hair and the face oval. A double U-curve necklace under the chin, concentric with the face oval. Below, arm and hand lines form a broad boat or V shape pointed at the lower left. A horizontal waist line. **Diagram XXI** (A, hair up): profile facing left with a high knot. Short dashed strokes in front of the face run parallel to the forehead-to-nose and chin projections. A dashed upward arc behind the hair repeats its upward sweep. **Diagram XXII** (B, low knot): the same profile, which he says he drew identical. Dashed full convex curves wrap the top of the head, bulge round the low knot and curve under the chin.
- *Conditions / counter-examples*: "only a very sharply-cut face that can stand this low knot" (p. 181).
- *Our interpretation*: echoing a form (similar curvature and centre, or parallel offset) draws attention to it. Concentric framing aims attention at the centre.
- *Implication for vector software*: an **echo finder**: for a selected key form (e.g. a face oval), list other paths with matching curvature or centre (concentric arcs, parallel offsets). Agent guidance: to emphasise a feature, frame it with concentric or parallel echoes, and check that you are not echoing something you don't want stressed. *Limits*: echoes are geometric and cannot tell whether a stressed feature is "best".
- *Do not algorithmize because*: whether a feature is "best" or "worst" must never become an attractiveness judgement of real people.
- *Disposition candidates*: analysis-overlay (echo finder); agent-prompt-or-resource (framing by echo).
- *Science caveat*: the claims about attention and flattering hairstyles are opinion, with dated gender framing ("Ladies who have the instinct for choosing a hat", p. 177). Rewrite neutrally in any guidance.

### SP-XII-29: Arrangements carry their effect whatever objects produce them
- *Author says*: the diagrams are abstract to show that the music of line lives in the arrangements themselves. Long uninterrupted horizontals give repose whether they are land and clouds, a breeze on a pool or a timber yard. Rows of verticals do the same whether cathedral, pine forest or scaffold poles. Volutes convey energy whether wave, cloud, dust or tangled hoop iron. Associations can strengthen, change or destroy the effect: a painter thinking of the wood's commercial value misses it. “whenever you get the swinging lines of the volute, an impression of energy will be conveyed” (p. 182).
- *Figures*: all chapter diagrams.
- *Conditions / counter-examples*: associations "may be greatly increased, modified, or even destroyed" (p. 182).
- *Our interpretation*: the same structural vocabulary applies to any subject, including non-pictorial work such as patterns or layouts.
- *Implication for vector software*: the line-rhythm analyses can run on any document type. Agent guidance: name the intended structural vocabulary (horizontal repose, vertical rows, volute energy) separately from the subject matter.
- *Do not algorithmize because*: the mapping from arrangement to emotion is opinion. Use it as a shared vocabulary, never as an output label.
- *Disposition candidates*: design-principle; agent-prompt-or-resource.
- *Science caveat*: **strong caveat**: "always" is an overclaim. Responses to line arrangements are not shown to be universal.

### SP-XII-30: Scale sets detail: big or architectural works need a severe skeleton; small works can hold rich detail
- *Author says*: abstract structure is overlaid with natural truth according to need. Large decorative work within architecture needs a severe line skeleton and little naturalism. Small cabinet pictures can carry full detail, because the whole fits in the field of view. Large pictures are seen from a distance where detail weakens them, so every artifice must give "breadth of treatment". “the smaller the picture the more detail it should contain, and the larger the less” (p. 183).
- *Figures*: none.
- *Conditions / counter-examples*: "sounds rather paradoxical … but it is nevertheless true" (p. 183).
- *Our interpretation*: detail density should be planned against **viewing size and distance**, not canvas pixels.
- *Implication for vector software*: an agent should ask for or infer the output use (billboard, poster, icon, print) before setting detail density. A preview at the expected viewing size (e.g. reduced thumbnail, or a blurred view simulating distance) helps judge breadth. *Limits*: viewing conditions are user input. The tool cannot infer them.
- *Do not algorithmize because*: n/a for the preview. The detail level is judgement.
- *Disposition candidates*: multi-step-workflow (target viewing size first); analysis-overlay (viewing-distance preview); agent-prompt-or-resource.
- *Science caveat*: plausible on visual-acuity grounds, but stated without evidence. Treat as practical advice.

---

## XIII Rhythm: Variety of Mass (body heading "VARIETY OF MASS")

**Locator**: h-2. Anchors `[#CHAPTER_XIII]`, `[#Page_185]` to `[#Page_199]`. Sub-anchors: `#plate43`, `#plate44`, `#diagram23`. Sidenotes: "Variety of Shape" (p. 185), "Variety of Tone Values" (p. 187), "Variety in Quality and Texture" (p. 189), "Variety of Edges" (p. 192), "Variety of Gradiation" [sic] (p. 199). Book.txt 5779–6254.

### SP-XIII-1: Masses vary in five ways
- *Author says*: the masses of a picture vary in shape, tone value, edges, texture or quality, and gradation, and each has its own rhythm. “variety in their shape, their tone values, their edges, in texture or quality, and in gradation” (p. 185).
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: five analysis channels for fills and regions. In vector terms: path shape, rendered luminance, edge softness, fill texture or pattern, and gradient.
- *Implication for vector software*: a "mass rhythm" panel with five readouts per region or group: shape descriptor, mean L*, edge softness profile, texture or pattern presence, gradient span (ΔL* within the fill). *Limits*: regions in a vector file do not always match perceived masses (overlaps, transparency). Analyse a flattened render.
- *Do not algorithmize because*: n/a for the readouts.
- *Disposition candidates*: analysis-overlay; design-principle.
- *Science caveat*: none.

### SP-XIII-2: Variety of shape is hard to invent: watch for "pet forms" and use nature
- *Author says*: shape variety is hard to invent and common in nature. Try drawing a row of elms of similar height and spacing, or groups of cumulus, from imagination: you will repeat two or three pet forms, or only one. Nature is the storehouse (a lump of coal suggests rock forms, cigarette smoke suggests line arrangements). “Variety of shape is one of the most difficult things to invent, and one of the commonest things in nature.” (p. 185).
- *Figures*: none (Diagram XXIII trees come later).
- *Conditions / counter-examples*: excess variety will "overpower the unity" (p. 186). See SP-XIII-3.
- *Our interpretation*: this is the most common failure of generated or copy-pasted vector art: identical or near-identical repeated shapes.
- *Implication for vector software*: a **near-duplicate shape finder**: normalise each closed path for position, scale and rotation (e.g. turning-function or Fourier descriptors) and cluster near-identical shapes, highlighting "pet form" clusters. Symbol instances count as one pet form (`symbol.*` exists). Agent guidance: when drawing many clouds, trees or leaves, vary their construction, not just scale or rotation. *Limits*: it finds geometric repeats. Deliberate pattern repeats (wallpaper, icons) will be flagged too, so make it opt-in or per layer.
- *Do not algorithmize because*: n/a for detection. Inventing variety from nature is the artist's work. An automatic "vary shapes" tool would produce noise, not nature.
- *Disposition candidates*: analysis-overlay (pet-form clusters); agent-prompt-or-resource; practice-exercise (draw ten different clouds; the tool shows clusters).
- *Science caveat*: none (an observation about practice).

### SP-XIII-3: Variety comes from nature, unity from imagination; the frame does not exist in nature
- *Author says*: imagination is needed for the larger unity of the composition. Variety should serve it. Nature suggests arrangements but not unity within four bounding lines, "the first condition of your picture". Yet nature tends toward unity over time (flowers in a vase settle, trees fill a gap). “the first condition of your picture, the four bounding lines, does not exist in nature.” (p. 186). “nature's laws tend to pictorial unity of arrangement.” (p. 187).
- *Figures*: none.
- *Conditions / counter-examples*: he expects to be told the vase and tree effect is just light and air seeking. He insists the effect holds whatever the cause.
- *Our interpretation*: a division of labour: reference supplies variety of form, and the designer supplies fit to the frame.
- *Implication for vector software*: agent workflow: gather or observe reference forms for variety, but plan the arrangement against the artboard. When tracing references (`imageTrace.*` exists in the command list), re-compose to the frame rather than keeping the photo's framing.
- *Do not algorithmize because*: relating forms to the frame is the imaginative act.
- *Disposition candidates*: multi-step-workflow; agent-prompt-or-resource.
- *Science caveat*: "nature's laws tend to pictorial unity" is the author's observation and speculation, not established.

### SP-XIII-4: Tone values: one black-to-white scale; colour matters only through its value
- *Author says*: all masses, whether light and shade or colour, can be placed on one imagined scale from white to black, treating the whole impression as "one mosaic" of lighter and darker masses. A dark object in light may be lighter than a white one in shadow. Observing values correctly is important and difficult. “Colour only matters in so far as it affects the position of the mass in this imagined scale of black and white.” (p. 187).
- *Figures*: none.
- *Conditions / counter-examples*: the "only" applies to the value analysis. He discusses colour elsewhere.
- *Our interpretation*: perceived lightness (e.g. CIE L*) is the working variable for mass design.
- *Implication for vector software*: a **non-destructive value view** (render as L* grayscale). `edit.colors.toGrayscale` in the command list is destructive, and `view.proofColors` may or may not cover this (not checked). Also a **value-group overlay**: quantise L* into N bands (3–9, user-set) and show areas per band. *Limits*: L* models average perception, and simultaneous contrast is not captured. The banding thresholds are arbitrary.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: engine-feature (value view mode); analysis-overlay (value bands and areas); agent-prompt-or-resource (check values in grayscale).
- *Science caveat*: none. A lightness scale is standard. Use a perceptual lightness metric, not naive RGB averaging.

### SP-XIII-5: Two senses of "tone"; "out of value"
- *Author says*: "tone" means an individual mass's place on the scale, and also the musical relation of all values to one governing "oneness of tone idea", like a note versus the tone of an orchestra. "Values" always means the relations between masses. A picture is "out of value or out of tone" when some values are lighter or darker than our sense of harmony expects. "Tone" often includes colour ("warm tone"). “out of value or out of tone when some of the values are darker or lighter than our sense of harmony feels they should be” (p. 188).
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: terms to standardise in our guidance: *value* = lightness of a mass, *tone* = the overall value scheme. "Out of value" = a mass whose value breaks the scheme.
- *Implication for vector software*: an analysis could flag **value outliers** within a group the user marks as one plane or one material (e.g. a mass far from its group's median L*). *Limits*: what counts as one group must come from the user. It shows deviation, not error.
- *Do not algorithmize because*: "our sense of harmony" is judgement.
- *Disposition candidates*: agent-prompt-or-resource (glossary); analysis-overlay (value outliers within user groups).
- *Science caveat*: none.

### SP-XIII-6: Tone music is strongest with large, simple masses
- *Author says*: the music of tone dominates when masses are large and simple, with little variety of texture and gradation. A mist improves a landscape's tone by merging masses. Glazing over a picture can work similarly, and so can light face powder (removes shiny lights, merges half-tones into the lights). Chinese and Japanese art is "very beautiful in this respect". “This quality of tone music is most dominant when the masses are large and simple” (p. 188).
- *Figures*: **Plate XLIII** (Speed's *Monte Solaro, Capri*, charcoal and white chalk on brown paper) sits here without a text reference. It shows a rock massif in a few large light and dark planes on a middle-tone ground.
- *Conditions / counter-examples*: see SP-XIV-9 on contrast and complexity.
- *Our interpretation*: fewer, larger value masses read more strongly. Simplification ("mist") is a useful review filter.
- *Implication for vector software*: a **simplify-for-review preview** (blur plus value banding, a "mist" view), and a metric: number of value masses (connected regions per value band) and the area share of the largest few. *Limits*: the count depends on blur radius and band count. It indicates simplicity, not quality.
- *Do not algorithmize because*: n/a for the preview.
- *Disposition candidates*: analysis-overlay (mist view, mass count); agent-prompt-or-resource ("fewer, larger masses").
- *Science caveat*: aesthetic opinion.

### SP-XIII-7: Middle values read as gentle; wide, sudden contrasts read as dramatic
- *Author says*: tone relations are most "sympathetic" when only middle values are used (lights low, darks high), as in half-light. They are most dramatic with great contrasts and sudden jumps, as when sun breaks on land against a retreating storm, a scene that also has warm-cold colour contrast. “Tone relationships are most sympathetic when the middle values of your scale only are used” (p. 189). “They are most dramatic and intense when the contrasts are great and the jumps from dark to light sudden.” (p. 189).
- *Figures*: none.
- *Conditions / counter-examples*: tone sentiment should agree with the subject (p. 207, SP-XIV-9).
- *Our interpretation*: two measurable variables: **value range** (spread of L*) and **adjacent value jump** (contrast across shared edges).
- *Implication for vector software*: readouts: L* histogram with its 5th–95th percentile span, and the distribution of ΔL* across region boundaries. Agent guidance: choose a value key for the mood on purpose. *Limits*: numbers describe the scheme and cannot say whether it is "sympathetic".
- *Do not algorithmize because*: the mood labels are the author's.
- *Disposition candidates*: analysis-overlay; agent-prompt-or-resource.
- *Science caveat*: the mood mappings are opinion. A link between high contrast and arousal is commonly assumed, but this book does not establish it.

### SP-XIII-8: Texture and quality: oneness for tone music, broken surface for brilliance (mostly painting)
- *Author says*: variety of surface quality is subtle and best learnt at first hand. Tone music is best served by "oneness of quality" (Whistler relies on canvas grain). Force and brilliance need broken, crumbling paint (Monet), at the cost of fine modelling. Thick paint yellows. He recommends a chart of paints dragged thin to thick over pencil lines, and absorbent canvas. In drawing, texture matters mainly in charcoal. “A oneness of quality throughout the work is best suited to exhibit it.” (p. 190).
- *Figures*: none.
- *Conditions / counter-examples*: the materials advice (oil yellowing, absorbent canvas) is specific to oil painting.
- *Our interpretation*: for vector work, "quality" corresponds to fill textures, grain or noise effects and brush textures. Use one consistent surface for tonal unity, and mix surfaces only for sparkle.
- *Implication for vector software*: agent guidance: keep one texture or grain treatment across a piece unless brilliance or sparkle is the goal, and note where textures differ. A "texture inventory" could list the distinct patterns and effects used (`effect.list` exists in the command list). The oil materials material is **out of scope**.
- *Do not algorithmize because*: the play of textures "must be appreciated at first hand" (p. 189).
- *Disposition candidates*: agent-prompt-or-resource; analysis-overlay (texture inventory); not-automatable (materials chemistry, not applicable).
- *Science caveat*: the claim that thick paint yellows more is the author's 15-year observation of one chart, an anecdote rather than a study.

### SP-XIII-9: Vary edges between soft and sharp; monotonous edges are hard or weak
- *Author says*: there is rhythmic beauty in the play from soft to sharp along a mass's edge. Monotonous sharpness is "hard, stern, and unsympathetic", useful in decorative and architectural work. Monotonous softness is weak and depends on colour for any merit. Mixing the two is best. The artist "seldom lets a sharpness of edge run far without losing it". Leading edges may be firm but still varied. The Venetians fused their edges, always firm and never feeble. “A monotonous sharpness of edge is hard, stern, and unsympathetic.” (p. 192). “He seldom lets a sharpness of edge run far without losing it occasionally.” (p. 193).
- *Figures*: Plate XLIV (SP-XIII-10).
- *Conditions / counter-examples*: hard edges are suitable in decorative or architectural settings (p. 192).
- *Our interpretation*: vector art defaults to monotonous sharpness, so this is a direct note for vector style.
- *Implication for vector software*: **lost-and-found edges**: (a) engine feature for variable edge softness along a path (feather amount that varies with position, like width points). (b) An **edge-hardness overlay** for the rendered image: gradient magnitude across each region boundary, coloured sharp to soft, plus "longest unbroken sharp run" per contour. Agent guidance: in painterly or naturalistic styles, break long sharp edges and keep a few deliberate firm ones. *Limits*: in flat-design styles uniform sharp edges are correct (his own decorative exception). The overlay measures rendering, not intent.
- *Do not algorithmize because*: where to lose and where to find edges follows the subject and focus (SP-XIII-10, -11). Random softening is not the principle.
- *Disposition candidates*: engine-feature (variable feather along path); analysis-overlay (edge hardness); agent-prompt-or-resource.
- *Science caveat*: "unsympathetic", "feeble" are opinion.

### SP-XIII-10: Put the sharpest accents on non-essentials and keep key features soft
- *Author says*: in *Bacchus and Ariadne* and many Venetian works, the most accented edges are on unessential things (a white drapery on the cymbal player's arm, a small white flower on a boy's head), while flesh edges are fused. Velázquez's Philip IV has a sharp edge only in the highlights of a neck chain. In *Breda*, the sharpest accents are bows on a tunic and a horse's forelock. Soft edges give the main features "largeness and mystery", and sharp accessories restore balance. “the most accented edges are reserved for unessential parts” (p. 193).
- *Figures*: **Plate XLIV** (*Breda* detail): a youth in a white tunic at centre with a dark horse's head to the right and dark figures in front at left. The tunic's edges vary from crisp to merged. He warns that the half-tone reproduction sharpens edges "monotonously" (p. 194).
- *Conditions / counter-examples*: it is "rarely" in the head, "except it may be occasionally in the eyes" (p. 194).
- *Our interpretation*: a counter-intuitive allocation rule: the hardest edges need not sit on the focal subject. They can be a balancing accent elsewhere.
- *Implication for vector software*: agent guidance (with the edge-hardness overlay): check where the sharpest edges are and decide whether they belong there. Do not assume the focal object must be the sharpest. *Limits*: this is a Venetian and Velázquez convention, not a universal rule. The overlay just locates the sharpest edges.
- *Do not algorithmize because*: essential versus non-essential is semantic and the user decides.
- *Disposition candidates*: agent-prompt-or-resource; analysis-overlay (rank of sharpest edges).
- *Science caveat*: "largeness and mystery" is opinion.

### SP-XIII-11: One focus for the whole picture: an ensemble, not a catalogue of focused parts
- *Author says*: seen as an ensemble, mass boundaries play between lost and found everywhere, and even the sea horizon varies. Focus on each part separately and hard edges appear everywhere. A picture made that way "will not hang together". In naturalistic work, one focal impression is as necessary as one perspective centre: you cannot focus foreground and distance at once. Primitive conventions are exempt. “a picture that is a catalogue of many little parts separately focussed will not hang together as one visual impression.” (p. 195). “you cannot have the focus in a picture at the same time on the foreground and the distance.” (p. 195).
- *Figures*: none.
- *Conditions / counter-examples*: applies to work aiming at "impressions directly produced in us by natural appearances" (p. 196). Not to conventional styles.
- *Our interpretation*: an edge-sharpness gradient should follow one focal region (or depth band). Uniform sharpness everywhere is a "catalogue".
- *Implication for vector software*: a **focal-softness workflow**: the user or agent marks a focal region or depth band, and the tool proposes (does not force) edge softening that increases with distance from it. The edge overlay can show sharpness against distance from focus. *Limits*: depth is unknown in 2D vector art unless layers or annotations supply it. In flat styles this does not apply.
- *Do not algorithmize because*: where the focus lies is the artist's choice. Automatic depth-of-field is a photographic effect, not the principle.
- *Disposition candidates*: multi-step-workflow; analysis-overlay (sharpness against focus distance); engine-feature (focus-based softness proposal, optional).
- *Science caveat*: the comparison with optical focus is broadly consistent with vision (the fovea), but how he applies it to pictures is a convention.

### SP-XIII-12: Serrated edges (trees against sky) need a large governing order
- *Author says*: serrated edges like trees against the sky are hard to treat, and a "hard, fussy, cut-out, photographic appearance" misses their beauty. There are three types. Early Italian: a thin tree with a rhythmic pattern of leaves against the sky, dark on light, a good foil to flesh (Millais's poplars are similar). Giorgione: rich, solid masses (both types appear in the *Fête Champêtre*). Corot: a wide focus, leaves resolved into masses whose edges are lost and found, with a few sharp leaves or a bough. Whatever the treatment, consider the tree as a whole. “some large order must be found to which the fussiness of the edges must conform.” (p. 198).
- *Figures*: **Diagram XXIII**: three tall narrow panels. **A** (centre, after a Botticelli attribution): three spindly straight trunks with sparse leaves as small separate dark flecks along the height, a dark-on-light pattern. **B** (right, Botticelli *Annunciation*): one thin straight trunk with a compact rounded crown of separate small leaves. **C** (left, Bellini): a dense tall oval crown built from many short leaf strokes over a short twisted trunk. For Corot, see Diagram XXVI (SP-XIV-15).
- *Conditions / counter-examples*: there are "very numerous" variations.
- *Our interpretation*: a serrated silhouette should follow a simple envelope shape. Detail is ordered by the envelope, not scattered.
- *Implication for vector software*: an **envelope check**: compute a smoothed hull (e.g. alpha shape or heavy smoothing) of a cluster of small shapes (leaves) and show it, so the user sees whether a clear large order exists. A scatter or spray tool (`symbol.spray` exists in the command list) could distribute within a user-drawn envelope with density falling toward the edges. *Limits*: the envelope is a proposal, and its quality is judgement.
- *Do not algorithmize because*: which treatment (pattern, mass or lost-found) fits is a style choice.
- *Disposition candidates*: analysis-overlay (envelope); improve-existing (spray within envelope, falloff); agent-prompt-or-resource (three tree treatments as a style menu).
- *Science caveat*: "beauty and sublimity" of trees is opinion.

### SP-XIII-13: Learn hard before loose; sound structure under free handling
- *Author says*: masters begin with a hard manner and little edge variety, master the design problems hard contours force on them, then loosen. Under good free painting lies "a bed-rock structure of well-constructed masses and lines". “So err in your student work on the side of hardness rather than looseness” (p. 199).
- *Figures*: none.
- *Conditions / counter-examples*: "Occasionally only let yourself go at a looser handling" (p. 199).
- *Our interpretation*: for workflows: resolve the design with hard edges and flat shapes first, then add softness.
- *Implication for vector software*: an agent workflow: compose with hard-edged flat shapes, check structure (value view, line overlays), then add softness, gradients and texture as a later pass, preferably as non-destructive effects that can be toggled off to re-check structure. *Limits*: a procedure, not a check.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: multi-step-workflow; practice-exercise (hard-edge version first); agent-prompt-or-resource.
- *Science caveat*: none (teaching advice).

### SP-XIII-14: Economy of gradation: one octave of value, so keep power in reserve
- *Author says*: gradation follows form and lighting, but each part's modelling must stay in harmony with the whole. It is very hard to decide how much of the value scale each part may use. The best work is very economical and keeps power in reserve, since there is only one octave from black to white. “You have only the one scale from black to white to work with, only one octave” (p. 199).
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: a **value budget**: each object's internal value range should be a modest fraction of the picture's total range.
- *Implication for vector software*: a readout per object or group: its internal L* span (gradient and shading) as a fraction of the document's span, flagging objects that use most of the whole range on their own. *Limits*: some objects legitimately carry the full range (the focal object in a chiaroscuro scheme). The ratio informs and does not rule.
- *Do not algorithmize because*: the right allocation is judgement.
- *Disposition candidates*: analysis-overlay (value budget per object); agent-prompt-or-resource ("model each part with few values").
- *Science caveat*: the "one octave" analogy is a metaphor. The fixed range of paint or display is a real constraint.

---

## XIV Rhythm: Unity of Mass (body heading "UNITY OF MASS")

**Locator**: h-2. Anchors `[#CHAPTER_XIV]`, `[#Page_200]` to `[#Page_218]` (missing 210, 213, 215: diagram pages). Sub-anchors: `#plate45`, `#plate46`, `#diagram24`, `#plate47`, `#diagram25`, `#diagram26`. Book.txt 6255–6819.

### SP-XIV-1: Unity of mass comes from a well-ordered tone scheme; learn it from nature, not by copying
- *Author says*: what was said about unity of line applies to mass outlines. The special unity of masses comes from a rhythmic scheme of tone values. Nature's tone arrangements always have unity (even town smoke makes "tone symphonies"), so base your scheme on a good value study from nature where you can. Twilight, moonlight and sunlight offer new material. But tone music "is not to be found by unintelligent copying" (p. 214), and Emerson: you won't find beauty "unless you take it with you". “Nature is the great teacher here, her tone arrangements always possessing unity.” (p. 200). “it is not to be found by unintelligent copying except in rare accidents.” (p. 214).
- *Figures*: none.
- *Conditions / counter-examples*: old masters used conventional, untrue but harmonious schemes (SP-XIV-2).
- *Our interpretation*: use reference images for **value relationships**, but a traced photo does not give a designed tone scheme.
- *Implication for vector software*: agent workflow: from a reference, extract a simplified value map (3–5 bands) as a study, then design the scheme deliberately, without converting the photo's values directly. When `imageTrace.*` is used, offer a "value-study" preset with few bands as a step, not a finished output. *Limits*: band extraction is mechanical. The design decisions come after it.
- *Do not algorithmize because*: the selection the artist brings (Emerson's "unless you take it with you") is the artist's contribution.
- *Disposition candidates*: multi-step-workflow (reference, value study, designed scheme); improve-existing (trace preset for value studies); agent-prompt-or-resource.
- *Science caveat*: "always possessing unity" is opinion.

### SP-XIV-2: Conventional schemes and the pull of light masses
- *Author says*: older masters used a few tried arrangements, especially a low-toned sky with flesh painted lighter than the sky, which in nature happens only in strong sun (try holding your hand against the sky). A bright sky competes with the figures because the eye goes to the light masses. Lowering it helps the figures but risks a "heavy stuffy look" and is only admissible in frankly conventional treatment. “The eye instinctively goes to the light masses in a picture” (p. 201).
- *Figures*: none.
- *Conditions / counter-examples*: a dark storm cloud is the exception (p. 201).
- *Our interpretation*: the lightest large masses compete for attention. Check that the focal subject is not outshone by background light.
- *Implication for vector software*: an analysis listing the top-N lightest masses by area-weighted L* and whether the user-marked focal object is among them. *Limits*: attention also depends on contrast, faces and text. Lightness is one cue, not a predictor.
- *Do not algorithmize because*: whether a conventional low sky suits the style is a choice.
- *Disposition candidates*: analysis-overlay (lightest-mass ranking); agent-prompt-or-resource.
- *Science caveat*: "The eye instinctively goes to the light masses" is a 1913 claim about attention. Brightness and contrast do attract attention in modern studies, but the strong form here is unverified as stated.

### SP-XIV-3: Keep the level of naturalism consistent between figures and background
- *Author says*: if truth to nature is carried far in the figures, the same truth will be expected in the background. If only certain truths are selected in the figures, the background can take more liberty. “If truth to natural appearances is carried far in the figures, the same truth will be expected in the background” (p. 201).
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: style consistency: the degree of realism should match across the picture.
- *Implication for vector software*: agent guidance: when one part is rendered in detail (gradients, texture, lost-and-found edges), match the background's treatment or deliberately stylise both. A rough consistency readout could compare per-layer detail measures (node density, gradient use, edge softness) across foreground and background layers. *Limits*: the readout is crude, and deliberate contrast of styles exists.
- *Do not algorithmize because*: the level of naturalism is a style decision.
- *Disposition candidates*: agent-prompt-or-resource; analysis-overlay (per-layer detail comparison, optional).
- *Science caveat*: none.

### SP-XIV-4: Judging values: use the same filter on reference and work, and look over a wide field
- *Author says*: values are hard to perceive. A black glass (darkening mirror) helps, but compare nature in the glass with your work in the glass, never nature in the glass with your work seen directly, which gives low-toned work. Values show only when attention is spread over a wide field. Half-closing the eyes kills much colour. Widening them keeps colour but is less safe. Begin with quiet grey days, and indoors keep the subject out of direct light with light from behind you. “the comparison should be made between nature seen in the glass and your work seen also in the glass.” (p. 202). “The artist has to acquire the habit of generalising his visual attention over a wide field” (p. 205).
- *Figures*: none.
- *Conditions / counter-examples*: the black glass is to "correct one's eye" only (p. 202).
- *Our interpretation*: any review filter (grayscale, blur, darken) must be applied **equally** to reference and artwork in a side-by-side view.
- *Implication for vector software*: a **paired-filter compare view**: a placed reference image and the artwork shown side by side, both under the same filter (grayscale, blur, darken, "squint"). Agents should judge values only from such paired renders. *Limits*: filters are approximations of the studio aids.
- *Do not algorithmize because*: n/a for the view.
- *Disposition candidates*: engine-feature (paired filter preview); multi-step-workflow; agent-prompt-or-resource; practice-exercise (quiet-light value studies).
- *Science caveat*: none (practical advice).

### SP-XIV-5: Avoid pure white and pure black masses; keep power in reserve
- *Author says*: never use pure white or pure black masses, though you may go near them. Using the whole gamut gives a "screaminess" that looks unrestrained, like a singer's extreme notes. Nature's highest lights and deepest darks always have some colour. He then hedges: “there should never be pure white or pure black masses in a picture.” (pp. 202–203). “But perhaps this is dogmatising” (p. 203).
- *Figures*: none.
- *Conditions / counter-examples*: strongly lit effects need sacrifices (SP-XIV-6). He admits good work might use the extremes, "though I think it very unlikely".
- *Our interpretation*: by default, keep large areas inside a slightly compressed value range. Tiny accents may touch the extremes (he says "masses").
- *Implication for vector software*: a gentle **extremes note**: area share of fills at L* > 98 or < 2 (excluding the artboard background if the user says so). An agent default palette with near-white and near-black instead of #FFFFFF and #000000 for large masses. *Limits*: flat or graphic design, accessibility contrast needs and print conventions often require true black and white. This is a style default for painterly illustration only.
- *Do not algorithmize because*: he himself calls it possibly "dogmatising".
- *Disposition candidates*: analysis-overlay (extremes share); agent-prompt-or-resource (default near-extremes in illustration).
- *Science caveat*: "screaminess" is opinion. The observation that highlights carry colour is broadly true.

### SP-XIV-6: Three ways to fit nature's value range into paint
- *Author says*: paint cannot match nature's range in bright light, so something is sacrificed. (1) Start from the light end and work down tone by tone: you reach paint's darkest dark too soon, so much of the picture goes dark (Rembrandt). (2) Fix the lightest light and darkest dark and work relatively between them: suits quiet light only (Bastien-Lepage, grey days). (3) Start from the dark end and work up: you reach paint's lightest light too soon, so the light end is compressed. It gives brilliance and lets lights dominate (Turner, modern sunlight). “The third way, and this is the more modern, is to begin from the dark end of the scale” (p. 204).
- *Figures*: none.
- *Conditions / counter-examples*: method 3 is best for sunlight, and method 2 only for quietly lit subjects.
- *Our interpretation*: these are three **tone-mapping strategies**: anchor highlights and clip shadows, linear normalisation between extremes, or anchor shadows and compress highlights.
- *Implication for vector software*: for value remapping of a selection or a traced reference, three presets matching these strategies, shown as curves on L*. Agent guidance: pick a strategy for the lighting mood (sunlit means preserve dark relations and compress the lights). *Limits*: a curve preset is mechanical. Which relations to preserve is judgement.
- *Do not algorithmize because*: n/a for the presets. The choice of strategy is artistic.
- *Disposition candidates*: engine-feature (value-remap presets); agent-prompt-or-resource.
- *Science caveat*: none. The problem of fitting nature's range into a smaller one is well recognised (it is what HDR tone mapping addresses). The attribution of methods to specific painters is the author's reading.

### SP-XIV-7: Strong light gives a two-mass formula: light against shadow
- *Author says*: very strong light (sunlight, or a dark room lit by one window) simplifies values into two harsh masses, brilliant light against dark shadow. This has been a favourite formula since Leonardo (*Virgin of the Rocks*), seen in Correggio and Reynolds. It gives breadth and directness but is "well-worn". Manet renewed it with glaring direct light that cuts out half-tones. Light, shade and half-tone are "enemies of colour" and of flatness. Even Manet's aggressive pattern follows composition principles: horizontal couch, vertical screen, a diagonal from top left to bottom right with the bedclothes edge parallel to it. “often resolving themselves into two masses, a brilliant light contrasted with a dark shadow.” (pp. 205–206). “the whole swing hangs on the diagonal from top left-hand corner to right; lower corner” (p. 207).
- *Figures*: **Plate XLV** (Correggio, *Venus, Mercury and Cupid*): a tall panel with three pale, strongly modelled nudes forming one light mass against a near-black foliage background. **Plate XLVI** (Manet, *Olympia*): a reclining pale nude on white bedding, a broad horizontal light band across a dark background, a vertical dark edge (screen) at centre, a dark-skinned attendant in a light dress at right. The bedding's lower edge runs parallel to the TL–BR diagonal.
- *Conditions / counter-examples*: "For certain impressions it probably will never be bettered" (p. 206).
- *Our interpretation*: a scheme template: few half-tones, two value groups, simple silhouettes. It suits vector art well (flat fills).
- *Implication for vector software*: a two-value **posterise preview** (threshold slider) to check whether the design reads as two clear masses. An agent "two-mass" scheme template. *Limits*: the preview is diagnostic and does not judge whether two masses suit the work.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: analysis-overlay (two-value threshold view); agent-prompt-or-resource (two-mass scheme).
- *Science caveat*: "enemies of colour" is opinion.

### SP-XIV-8: Plan on flat tones first
- *Author says*: large flat tones give power, simplicity and breadth, and they show up every small variation used in modelling, so you model with the fewest tones. Whatever richness you add later, plan on a basic structure of simple, flat tones. “see to it that in planning your picture you get a good basic structure of simply designed, and as far as possible flat, tones.” (p. 207).
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: the most directly applicable workflow rule for vector illustration.
- *Implication for vector software*: **blocking-in stage**: an agent first draws the composition as a handful of flat-filled shapes (3–6 values), checks it in the value view, and only then adds gradients, texture and detail. A "flatten preview" (replace each gradient by its mean colour) lets users and agents re-check the base structure at any time. *Limits*: none for the procedure. The preview is approximate for transparent objects.
- *Do not algorithmize because*: n/a.
- *Disposition candidates*: multi-step-workflow (core); engine-feature (flatten-gradients preview); agent-prompt-or-resource.
- *Science caveat*: none.

### SP-XIV-9: Strong contrast requires simple masses; low contrast allows more variety
- *Author says*: near values give a quiet, reserved impression and wide gaps give a dramatic one, and tone sentiment should agree with the subject. More variety of tone and shape is allowed with a small range of values than with strong contrast. For "black and white effects" the masses must be very simply designed, or breadth and unity are lost, so strong-contrast effects are the hardest. “When strong contrasts of tone or what are called black and white effects are desired, the masses must be very simply designed.” (p. 208).
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: a trade-off: **contrast × number of masses** should stay bounded.
- *Implication for vector software*: an analysis plotting value range (L* span) against mass count (connected regions after banding), with a note when both are high (many small, high-contrast masses). *Limits*: no threshold is given. The plot positions a design and does not judge it.
- *Do not algorithmize because*: where the bound lies is judgement.
- *Disposition candidates*: analysis-overlay (contrast versus complexity); agent-prompt-or-resource ("high contrast, then fewer, larger masses").
- *Science caveat*: opinion, though consistent with practice.

### SP-XIV-10: Strong colour contrast and strong tone contrast don't mix
- *Author says*: you "cannot with much hope of success" have strong contrasts of colour and of tone in the same picture. If colours contrast strongly, keep their values close: Chinese and Japanese art uses violent colour contrasts at equal value. If tone contrasts are strong (Rembrandt), restrain colour, as Reynolds did by making all shadows the same brown. “you cannot with much hope of success have strong contrasts of colour and strong contrasts of tone in the same picture” (p. 208).
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: for adjacent masses, high ΔL* and high hue/chroma difference together is the risky combination.
- *Implication for vector software*: an **adjacent-pair check**: for neighbouring regions, compute ΔL* and chromatic difference (e.g. ΔC*/Δh in CIELCh) and flag pairs high on both. A palette helper could equalise L* across a contrasting hue set (the "same tone value" approach). *Limits*: the accessibility needs of text and UI often require high lightness contrast, so this applies to pictorial work. Thresholds are arbitrary.
- *Do not algorithmize because*: "too violent" is judgement. Some styles want it.
- *Disposition candidates*: analysis-overlay (adjacent-pair check); engine-feature (equal-lightness palette tool); agent-prompt-or-resource.
- *Science caveat*: aesthetic opinion. The descriptions of Chinese and Japanese practice and Reynolds are the author's generalisations.

### SP-XIV-11: Flat tones are like straight lines and gradated tones like curves
- *Author says*: flat tones relate to repose and serious feeling as straight lines do (a cloudless sky over a bare plain, a calm evening reducing the west to one dark tone). Gradated tones relate to charm as curves do. Watteau and Gainsborough base their charm on gradations "swinging and interlacing", without large flat tones. The principle also applies in small details wherever grace is the motive. “There is some analogy between straight lines and flat tones, and curved lines and gradated tones.” (p. 209).
- *Figures*: Diagram XXIV, Plate XLVII (SP-XIV-13).
- *Conditions / counter-examples*: gradated compositions need steadying elements (SP-XIV-14) and character (SP-XIV-12).
- *Our interpretation*: one choice of mood for line and mass alike: straight with flat, or curved with gradated.
- *Implication for vector software*: agent style guidance: match fill treatment to line treatment (flat fills with straighter geometry, gradients with curves) unless deliberately contrasting. The mass panel could report the share of gradient fills next to the line panel's curve share. *Limits*: a correlation report, not a rule.
- *Do not algorithmize because*: mood choice is intent.
- *Disposition candidates*: design-principle; agent-prompt-or-resource; analysis-overlay (gradient share).
- *Science caveat*: emotional mappings are opinion.

### SP-XIV-12: Gradated, charming schemes need character to avoid prettiness
- *Author says*: Watteau achieved charm "without degenerating into feeble prettiness" through character in the figures (especially the men) and well-drawn, varied draperies. His landscapes lack character and would be intolerable without the figures. His followers (Boucher) kept the prettiness and lost the character. “doing this without degenerating into feeble prettiness, and this he did by an insistence on character in his figures” (p. 211).
- *Figures*: Plate XLVII.
- *Conditions / counter-examples*: none.
- *Our interpretation*: soft gradients plus soft drawing slide into blandness. Keep some firm, specific drawing somewhere.
- *Implication for vector software*: agent guidance: in a gradient-heavy, soft-edged piece, keep crisp, specific drawing in the key forms (faces, hands, drapery folds).
- *Do not algorithmize because*: "character" cannot be measured.
- *Disposition candidates*: agent-prompt-or-resource; not-automatable (judging character).
- *Science caveat*: opinion.

### SP-XIV-13: Watteau's tone path: an accent, a dark frame that gradates round, and a line of figures as spots
- *Author says*: in *Embarquement pour Cythère*, a strongly accented contrast at the top centre (trees against sky) starts a dark tone that gradates down into the ground, past the lower-right corner and across the front. Near the lower-left corner it reverses (light to dark) and ends sharply against the sky in the rock at left. The rich detail hides this larger motive, which holds it together. The dark frame "gradate[s] away into the distance", cut by a wedge-shaped middle tone of hills leading to the horizon. A line of figures crosses it: a "pretty clatter of spots", light spots in the darks ending in the light figure at right, dark spots in the lights ending in the cupids against the sky. Steadying comes from the tree stem and statue (vertical), the distant horizontal at left, the ground line and the staffs. His charcoal scribble avoids drawing to show the tone music is abstract. “Note the pretty clatter of spots this line of figures brings across the picture” (p. 212).
- *Figures*: **Diagram XXIV** (charcoal): a dark mass fills the upper right (trees), with its sharpest edge against the light sky near the top centre. The dark runs down the right side into a dark foreground band across the bottom, then rises up the left edge as a dark rock form sharp against the sky. The sky is light at upper left and centre. A middle-tone wedge (hills) narrows from left-centre to the horizon. A band of small light and dark spots (figures) crosses the middle from lower left to right. **Plate XLVII**: the painting, matching this structure.
- *Conditions / counter-examples*: "Flat tones and hard edges are avoided" (p. 211).
- *Our interpretation*: a tone composition can be a **path of value**: a continuous dark band whose value and edge sharpness change along its length, punctuated by alternating spots that invert the local value.
- *Implication for vector software*: an agent exercise and workflow: draw the tone "path" as a broad gradient band (stroke with a gradient along the path, if supported), then place light spots in darks and dark spots in lights along a figure line. The value view checks it. *Limits*: as with all of these, this is a method and not a test.
- *Do not algorithmize because*: the specific path is a composition decision.
- *Disposition candidates*: practice-exercise (abstract tone-path study); multi-step-workflow; engine-feature (gradient along stroke, if absent; not checked).
- *Science caveat*: "pretty" is opinion.

### SP-XIV-14: Gradated compositions need steadying straight lines
- *Author says*: Watteau steadies with verticals and horizontals. Turner's *Ulysses* uses the horizon, wave lines, masts, oars and, in the original, radiating lines from the sun. Corot uses the vertical central trees and a horizontal distance behind the figure. “Without these steadying influences these compositions of gradated masses would be sickly and weak.” (p. 214).
- *Figures*: Diagrams XXIV, XXV, XXVI (SP-XIV-13, -15).
- *Conditions / counter-examples*: none.
- *Our interpretation*: the mass-level counterpart of SP-XII-10. Gradient-heavy work needs some straight elements.
- *Implication for vector software*: combine readouts: when the gradient share is high and the straight-line share is low, the overlay can point it out. Agent guidance as in SP-XII-10. *Limits*: as SP-XII-10.
- *Do not algorithmize because*: placement is judgement.
- *Disposition candidates*: analysis-overlay (combined note); agent-prompt-or-resource.
- *Science caveat*: "sickly and weak" is opinion.

### SP-XIV-15: Put the greatest dark against the greatest light at one place, and let masses gradate away from it
- *Author says*: Turner's *Ulysses deriding Polyphemus* gains great unity by putting the darkest dark (rocks and ships' prows) against the lightest light (the rising sun). From there dark and light masses gradate in different directions and merge above the sails. The sails cut sharply into the dark and the far-right rocks and ship into the light, with edges accented in places and lost in others. Corot does the same: a dark silhouette against a light sky, balanced shapes, lost-and-found edges, and the greatest light-dark contrast where the right-hand rock cuts the sky. “A great unity of expression is given by bringing the greatest dark and light together in sharp contrast” (p. 214).
- *Figures*: **Diagram XXV** (Turner, charcoal): the light is centred right of centre near the horizon, with the sky hatched in arcs roughly concentric around it. A dark mass at upper left. Dark rocks and ships form a band across the lower middle against the light. Tall light sail shapes (narrow wedges) cut up into the dark at left. A small dark ship at far right against the light. Horizontal horizon and wave strokes at the bottom. **Diagram XXVI** (Corot, charcoal, portrait format): a dark frame of masses (trees at left, foreground, a rock at right) round a light central opening of sky. A central group of three or four tall thin vertical trunks with soft feathery crowns rising to the top. A tiny figure at their base, left. The right-hand rock meets the sky with the sharpest edge.
- *Conditions / counter-examples*: grace and charm motives (gradated masses, lost-and-found edges) also apply in the smallest details (p. 216).
- *Our interpretation*: there is **one maximum-contrast location**, and the value field falls away from it. It acts as the tonal focus.
- *Implication for vector software*: a **max-contrast locator**: compute local contrast (ΔL* across edges, weighted by edge length) on the render and mark the top few locations, so the user or agent can confirm they coincide with the intended focus. Competing maxima are worth a note. *Limits*: it finds tonal contrast and cannot know the intended focus. Faces and text draw attention regardless.
- *Do not algorithmize because*: n/a for the locator. Where the focus should be is intent.
- *Disposition candidates*: analysis-overlay (max-contrast locator); agent-prompt-or-resource ("one place of greatest contrast").
- *Science caveat*: "great unity of expression" is opinion. Contrast as an attention cue is plausible but not established here.

### SP-XIV-16: Knowledge of principles does not make a picture; keep a record of ideas
- *Author says*: a composition can obey every rule and be "of no account". The life-giving quality defies analysis and comes from the artist. Record pictorial ideas, however fumbled. They germinate over years. Don't wait until you can draw well. Still, principles help: most painters are saved "worry" over things that "won't come". The old masters were more systematic, which freed their instinct for higher things. “A composition may be perfect as far as any rules or principles of composition go, and yet be of no account whatever.” (p. 216). “Knowledge in art is like a railway built behind the pioneers who have gone before” (p. 218).
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: analysis features support diagnosis. They are not a substitute for an idea, and an idea notebook is worth supporting.
- *Implication for vector software*: product stance: never present compliance with these analyses as quality. An "idea sketch" quick-capture (rough artboards kept with the document) suits his advice. Agents should present suggestions as options for the user's idea, not a recipe.
- *Do not algorithmize because*: the "life-giving quality" is by his account unanalysable.
- *Disposition candidates*: design-principle (wording, no scores); agent-prompt-or-resource; not-automatable.
- *Science caveat*: none.

### SP-XIV-17: Conceal the artifice; mechanical jitter is not "dither"
- *Author says*: all artifice must be hidden, since "a picture obviously composed is badly composed". It is as if parts were placed in rhythmic relation and the picture then "jarred a little", shifting everything slightly and introducing "dither". But no mechanical jogging produces the vital quality (some photographers jog the camera to get "play"). The mechanical principles are subordinate to a vital one. “a picture obviously composed is badly composed.” (p. 218). “no mechanical jogging will introduce the vital quality referred to” (p. 218).
- *Figures*: none.
- *Conditions / counter-examples*: none.
- *Our interpretation*: this is a **direct warning against implementing "dither" as random jitter**. Variation should be intentional and related to form (SP-XI-8, -9, SP-XIII-9), and structure should not look like a template.
- *Implication for vector software*: (a) any "jitter", "randomise" or "hand-drawn" feature should be documented as a stylistic effect, not as the life Speed describes. (b) Agents should not snap everything exactly to the structural guides. Treat guides as targets with slight, considered departures, and hide guides in output. (c) Analyses that find too-exact alignment (all elements exactly on a grid or diagonal) can note "artifice may be apparent". *Limits*: exactness may be intended (diagrams, UI).
- *Do not algorithmize because*: he says explicitly that the vital quality cannot come mechanically.
- *Disposition candidates*: design-principle (no fake-dither claims); agent-prompt-or-resource; analysis-overlay (exact-alignment note).
- *Science caveat*: none.

---

## Cross-cutting notes

**Recurring themes**
1. **Unity vs Variety** is the frame for chapters X–XIV: line variety (XI), line unity (XII), mass variety (XIII), mass unity (XIV). Every feature idea can be tagged by which axis it serves.
2. **Lines and masses are two views of one structure** (SP-X-3), and the line principles have mass analogues: straight ~ flat tone, curve ~ gradated tone (SP-XIV-11). Steadying straights appear in both (SP-XII-10, SP-XIV-14).
3. **The frame is the first given**: alignment to edges, diagonals and corners recurs (SP-XI-3, SP-XII-16, -17, SP-XIV-7 on Manet).
4. **Inspection methods he actually uses**, which map directly to tool features: tracing an overlay over the original (SP-XII-15); covering an element with a finger (SP-XII-11); turning the picture round (SP-XII-22); masking with a hole in paper (SP-XII-28); the black glass, used on both reference and work (SP-XIV-4); half-closing or widening the eyes (SP-XIV-4); laying a ruler on a line to test parallelism to the diagonal (SP-XII-17); abstract charcoal "scribbles" of tone (SP-XIV-13).
5. **Geometry he defines precisely enough to compute**: constant versus varying curvature (SP-XI-2, -5); parallel families (SP-XII-3); lines meeting if extended (radiation, SP-XII-23); right-angle crossings (SP-XII-13, -25); contour continuation (SP-XII-2); opposition of curvature on paired contours (SP-XI-8); a lightness scale (SP-XIII-4); edge sharpness variation (SP-XIII-9); one maximum-contrast point (SP-XIV-15).
6. **Vector defaults conflict with several principles**: perfect circles and ellipses, uniform stroke width, hard edges everywhere, identical copies and symbols, exact snapping, pure #000/#FFF. Each is a candidate for gentle analysis notes and agent guidance (never errors).

**Defined or coined terminology** (for a shared glossary)
- *Rhythm*: the power of lines, tones and colours, by their arrangement, to affect us like music (p. 127).
- *Unity*: relation of all parts to one controlling conception (p. 132). *Variety*: charm, vitality, picturesque (p. 132).
- *Dither*: the play between parts that gives life. Coined in an earlier chapter (book.txt ~2416, not in this range) and used here pp. 132, 134, 147–148, 218.
- *Variety in Symmetry*: opposed hollows and fullnesses on roughly symmetric forms (p. 141, "what may be called").
- *Run* (of a line or contour): its overall unity or sweep (pp. 139–141).
- *Squaring it out*: ruling a grid over a study to copy it (p. 138).
- *Sustained effect / sustained chord*: what parallelism gives (p. 145). *Vertical chord* (p. 155). *Chord* is also used for the right angle (p. 156).
- *Straight-up-ness*: implied verticality of a whole figure (p. 155).
- *Anatomy* of a composition: its underlying abstract lines (p. 159).
- *Naked line* versus *general swing* (pp. 160–161).
- *Tying lines*: the lines that bind a composition (p. 163). *Continuous flow* (p. 163). *Flame-like flow / flame line* (pp. 167–169).
- *Radiating lines*, *dominant line*, lines *in sympathy*, *sympathetic radiation* (pp. 172–176). *Sympathetic rhythm* (Plate XLII caption).
- *Pick out* (lines accentuating features they echo, p. 177).
- *Breadth of treatment* (p. 184).
- *Pet forms* (repeated invented shapes, p. 185 "pet forms"; "pet cloud or tree", p. 186).
- *Tone values / values* (relations on the black-white scale); *tone* (also the governing whole); *out of value / out of tone* (pp. 187–188). *Tone music* (p. 188). *Oneness of quality* (p. 190).
- *Music of edges* (p. 193). *Lost-and-foundness* (pp. 198, 216). *One focal impression / one all-embracing focus* (p. 195).
- *Bed-rock structure* (p. 198). *Reserve power* (p. 203). *Screaminess* (p. 203).
- *Gradated tone composition* (p. 211). *Clatter of spots* (p. 212).
- Borrowed definitions: "variety in unity" (good style), Hogarth's "varying well" (composition), and his own suggestion "contrasts in harmony" (colour), p. 136.

**Where he warns against mechanical rules** (these must shape how any feature is worded)
- Rules never replace the "initial artistic impulse" (p. 130).
- The Greeks allowed "No formula or canon of proportions or other mechanical device" to subdue life (p. 133).
- Books like this are for when things "won't come", not for inspired work (p. 143).
- His diagrams pick out only some lines, and the real ones are "carefully hidden" (p. 159).
- The pure white and black rule "perhaps … is dogmatising" (p. 203).
- A composition "perfect as far as any rules … go" may be "of no account" (p. 216).
- "a picture obviously composed is badly composed" and "no mechanical jogging" (p. 218).
- Tone music is "not to be found by unintelligent copying" (p. 214).
- **Implication for us**: analyses must describe, not score. Generators must not claim to create "life". The user's intent overrides every heuristic.

**Science and ethics caveats that recur**
- Fixed emotional meanings of lines and tones (horizontal = calm, vertical = elevation, right angle = drama, volute = energy, jagged = terror, middle values = sympathy) are 1913 artistic opinion and convention. Never present them as fact or use them as output labels.
- Eye-movement and attention claims ("the eye instinctively running with them", the eye goes to light masses, corners attract the eye) are unverified as stated. Use them only as heuristics.
- Claims about beauty, type, ugliness, faces, foreheads and hairstyles (SP-X-7, SP-XII-6, SP-XII-28) must **never** become attractiveness scoring of people. Some framing is dated and gendered (p. 177). Rewrite neutrally.
- Historical generalisations (Greek conic versus Roman circular mouldings, the theory of art's decline, Chinese and Japanese practice) are the author's.

**Assets caution** (project rule): the plates are public-domain reproductions via Project Gutenberg. If any are used in-app (tutorial overlays, exercises), each file needs an `ASSETS.md` row and licence notes. Prefer re-drawing the diagrams from scratch as original geometry generated in code (the measurements above allow that). No Adobe material is involved.

## Open questions

1. **Range mismatch**: the brief's line numbers (4448–6820) start at chapter XI, but its chapter list includes X. I covered X as well (4126–4447). Does ledger B also cover X? If so, reconcile the SP-X IDs.
2. **Diagram XVIII panel F**: the text says 1-2 is the dominant line (p. 173), but the caption says "F. TAKING LINE 5-6 AS DOMINANT LINE". Which should exercises follow? The caption matches the image labels better, in my reading.
3. **Blake *Job* XI orientation**: the caption says "Printed the wrong way up", the text says "put sideways on" (p. 171). I could not settle the printed orientation by eye. It matters only if we reproduce the flip-test example.
4. **Unreferenced plates**: Plate XXX (Rosalind and Orlando study) and Plate XLIII (Monte Solaro) appear in this range with no textual reference. Should they be tagged to principles (I tentatively tied XLIII to SP-XIII-6)?
5. **Defining "in sympathy"** for the sympathy graph (SP-XII-24, -26): shared radiation centre within what radius? Parallel within what angle? Similar curvature within what ratio? This needs user testing. The author accepts "obviously in sympathy" without a shared point.
6. **Perceptual metrics**: which lightness model (CIE L*, OKLab L) and what render resolution for value and edge analyses? How should transparency and blend modes be handled?
7. **Existing capabilities**: I named command ids from `cmd_ids.txt` (e.g. `stroke.widthProfile.*`, `guide.*`, `object.repeat.radial`, `edit.colors.toGrayscale`, `path.curvature`) without checking their behaviour. Before deciding between "improve-existing" and "engine-feature", someone needs to check whether curved guides, a non-destructive grayscale or view-rotate mode, per-segment feathering, and gradients along a stroke exist.
8. **Cross-ledger links**: SP-XII-5 (balance by line weight) and SP-XII-27 (centred line, "better proportion") overlap with chapters XV Balance and XVI Proportion (ledger D). The definition of "dither" is in chapter VI (ledger A or B).
9. **Wording of analysis output**: should overlays ever show a single number (e.g. curvature variation), or only visual marks plus descriptive text? Given the author's own warnings, I recommend descriptive marks, with numbers only behind a detail toggle.
10. **Viewing-size input** (SP-XII-30): where should a document's intended viewing size or distance live (document metadata?) so agents and previews can use it?
