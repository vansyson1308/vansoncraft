# Ledger D — Speed, *The Practice and Science of Drawing* (1913), ch. XV–XXI and Appendix

Book Researcher part D of 4. Source: `book.txt` lines 6815–8989 (Gutenberg #14264 EPUB extraction) plus the XHTML and the large JPEGs (`7373817595610384519_*.jpg`). Page numbers come only from `[#Page_N]` anchors. Each entry separates **Author says**, **Our interpretation** and the **Implication** we infer. Quotes are verbatim and 25 words or fewer.

Conventions: "p. N" is the printed page whose anchor comes before the quoted words. "pp. N–M" means the quote crosses an anchor. Pages 232–234 have no anchors in this edition (they are presumably the leaves for Plate XLVIII and Diagram XXVIII), so the text runs from p. 231 straight to p. 235.

## Reading log

| Section (chapter + heading) | book.txt lines | XHTML file | Page anchors covered | Read? | Figures viewed (large JPEG) |
|---|---|---|---|---|---|
| XV Rhythm: Balance (h2 reads "XV BALANCE"; contents reads "RHYTHM: BALANCE"). Sidenotes: Straight Lines and Curves; Flat and Gradated Tones | 6815–6913 | `…14264-h-2.htm.xhtml` | 219, 220, 221, 222 (part) | yes | — |
| XV cont. Sidenotes: Light and Dark Tones; Warm and Cold Colours; Interest and Mass; Variety and Unity. Line 6918 "XVI RHYTHM: PROPORTION" is the `<title>` of file h-3 (a file-split artifact), not a heading | 6915–7045 | `…14264-h-3.htm.xhtml` | 222 (cont.)–226 | yes | Diagram XXVII |
| XVI Rhythm: Proportion | 7046–7354 | h-3 | 227, 228, 229, 230, 231, 235, 236, 237, 238 | yes | Plate XLVIII; Diagram XXVIII (1) A/D/G, (2) B/E/H, (3) C/F/I; Plate XLIX |
| XVII Portrait Drawing (incl. sidenotes "Expression" at 7787 and "Treatment of Clothes" at 7825) | 7355–7885 | h-3 | 239–255 | yes | Plates L, LI, LII, LIII |
| XVIII The Visual Memory | 7886–8144 | h-3 | 256–264 | yes | Plate LIV (printed here, discussed in XX) |
| XIX Procedure | 8145–8313 | h-3 | 265–270 | yes | none in chapter |
| XX Materials (sidenotes: Lead Pencil 8412; Silver and Gold Point 8443; Charcoal 8464; Red Chalk 8542; Black Conté and Carbon Pencil 8608; White chalk 8622; Lithography 8642; Pen and Ink 8697; Etching 8738; Paper 8757) | 8314–8780 | h-3 | 271–285 | yes | Plates LV, LVI; LIV (cross-ref. p. 281); Plate IV (outside range, viewed only because p. 281 cites it) |
| XXI Conclusion | 8781–8864 | h-3 | 286–288 | yes | none |
| Appendix | 8865–8983 | h-3 (`#APPENDIX`) | 289, 290, 291 | yes | Diagram XXIX "The Phi Proportion" (not in the List of Diagrams). Tick positions measured in pixels |
| (stop) Index | 8984–8989 | h-3 | 292 | heading only | — |

All 14 images in the range were opened at full size (12 figures): Diagram XXVII; Diagram XXVIII (3 images); Diagram XXIX; Plates XLVIII–LVI (9 plates). Plate IV was also viewed as a cross-reference.

**Principle count:** XV 8, XVI 9, XVII 15, XVIII 9, XIX 11, XX 15, XXI 5, Appendix 4. **Total 76.**

---

## XV Rhythm: Balance

**Locator:** starts in `…14264-h-2.htm.xhtml` (`#CHAPTER_XV`, `#Page_219`, h2 `#pgepubid00218`). It continues in `…14264-h-3.htm.xhtml` from the "Between Light and Dark Tones" sidenote, with `#diagram27` / `#id-3199986787303765109`. Page anchors run from 219 to 226.

### SP-XV-1 Balance must be approximate: never perfect, never far out
- *Author says:* Art sets opposing factors against each other: straight and curved, light and dark, warm and cold. Perfect balance between them is dull, and balance that is far out disturbs the eye. Pictures that aim at repose come closest to perfect balance. Exciting subjects need less, but every picture needs some, "as in good tragedy". "Were the balance between them perfect, the result would be dull and dead." (p. 219)
- *Figures:* none.
- *Conditions / counter-examples:* the motive (repose or turbulence) sets how close to equilibrium the balance should be, but it is never zero.
- *Our interpretation:* balance is a band that moves with the mood of the piece, not an optimum to maximise.
- *Implication for vector software:* every XV readout (straight/curved share, tonal extremes, warm/cold totals) should show where the art sits on a spectrum, read against a mood the user declares (calm or energetic). It should never be a pass/fail score or a "perfectly balanced" badge. It measures geometry and colour only, and cannot know the intended mood unless told.
- *Do not algorithmize because:* the right degree of imbalance is a judgement about subject and feeling. An optimiser that pushed toward 50/50 would produce exactly the "dull and dead" result he warns about.
- *Disposition candidates:* design-principle (frames every XV metric as a band); agent-prompt-or-resource (the agent states the intended mood before balancing); analysis-overlay (descriptive only).
- *Science caveat:* the analogies to worlds and States are rhetoric, not physics or politics.

### SP-XV-2 Straight lines against curves, with a bias toward the straight
- *Author says:* The richer the curves, the more severe the straights needed for repose. Subjects of movement need fewer straights, and contemplative subjects lean toward straights. A picture made only of rich curves is irritating, and of the two extremes, all-straight is preferable. The finest art keeps the balance on the side of square forms. "Even the finest curves are those that approach the straight line rather than the circle" (p. 221). He links straights to permanence, restraint and infinity, and rich curves to "uncontrolled energy". He calls rococo "vicious" and judges Egyptian severity nobler.
- *Figures:* none here. He refers back to the Greek temples discussed earlier.
- *Conditions / counter-examples:* the subject's need for movement or repose decides the bias.
- *Our interpretation:* two properties are measurable: the share of near-straight versus tightly curved contour, and how flat each curve is (low curvature for its length).
- *Implication for vector software:* an analysis overlay could colour path segments by curvature (turning per unit length, or deviation from the chord) and report the length-weighted share of each kind. A "flatten curve" assist could pull handles toward the chord. Limits: geometric curvature is not perceived "richness", which also depends on scale, stroke width and context. The overlay reports and does not judge.
- *Do not algorithmize because:* the moral weighting is the author's taste, and the target share depends on the subject.
- *Disposition candidates:* analysis-overlay (curvature map and share); improve-existing (smoothing/simplify option that prefers flatter curves); design-principle.
- *Science caveat:* the moral and emotional meanings of line types (straight as permanence, curve as vice) are 1913 taste, not verified perception science.

### SP-XV-3 Flat tones against gradated tones, with a bias toward flat
- *Author says:* The same balance applies to tone, with flat tones taking the place of straights and gradated tones the place of curves. "Deeper, more permanent things" find expression in wide flat tones, while excess gradation gives prettiness or "gross roundnesses of vicious modelling". He contrasts Greuze with Giotto, the Italian primitives and Puvis de Chavannes. "The balance in the finest work is usually on the side of flat tones rather than on the side of gradated tones." (p. 221)
- *Figures:* none.
- *Conditions / counter-examples:* he says "usually". This is a bias, not a ban.
- *Our interpretation:* in vector art, flat fill versus gradient, mesh or blur is literally an object property, so this balance can be inspected directly.
- *Implication for vector software:* an overlay could report the share of filled area that uses gradients, meshes or blurs versus flat fills. Agent default: build in flat fills and add gradation only where it carries form. Limits: this counts the mechanism, not the visual effect. A subtle two-stop gradient can read as flat, and faceted flat shapes can read as modelled.
- *Do not algorithmize because:* "prettiness" is a judgement.
- *Disposition candidates:* agent-prompt-or-resource; analysis-overlay; design-principle.
- *Science caveat:* the rankings of named painters are taste.

### SP-XV-4 Rescue a muddled picture by reducing it to flat tones
- *Author says:* A picture that is "mucked up" can be put right by painting out the gradations and getting back to "a simpler equation". Over-concern with the gradations of small modelling is a common reason pictures go wrong. "The less expenditure of tone values you can express your modelling with, the better, as a general rule." "It can be got on the right road again by reducing it to a basis of flat tones" (p. 221).
- *Figures:* none.
- *Conditions / counter-examples:* he says "often" and "as a general rule". It is a repair method, not a style mandate.
- *Our interpretation:* a reversible "simplify to a few flat values" step works as a debugger for pictures.
- *Implication for vector software:* two engine features. First, a non-destructive tone-simplify view: each object rendered at its mean luminance, quantised to N levels, with gradients collapsed to their average. Second, a "flatten gradients to flat fills" command that works on a duplicate and goes through history. Agent workflow: duplicate the artboard, collapse it to 3–5 values, fix the big pattern, then bring gradation back. Limits: the quantisation thresholds are arbitrary, and the user chooses which gradations return.
- *Do not algorithmize because:* deciding that a picture is "out of gear" is a judgement. Offer the tool; never trigger it automatically.
- *Disposition candidates:* engine-feature (tone-simplify view; flatten-gradients command); multi-step-workflow; agent-prompt-or-resource.
- *Science caveat:* none (studio practice, presented as such).

### SP-XV-5 Balance the extremes of the tone scale, not the amounts
- *Author says:* If the lights stay well short of white, the darks should stay well short of black (Whistler). Painters who go near white also go near black (Rembrandt, Holl). It is the extremes that balance, not the amounts. He reports the "old rule" of two-thirds light and one-third dark, and rejects it: "too many exceptions, and no mention is made of half tones". High-key work ("the white picture in which all the tones are high") usually contains spots of intense dark, darker the lighter the rest is (Turner's black gondolas). One-tone pictures in middle tones are consistent with the principle. "When the highest lights are low in tone, the darkest darks should be high in tone." (p. 222)
- *Figures:* none.
- *Conditions / counter-examples:* "Like all so-called laws in art, this rule is capable of many apparent exceptions" (p. 222). Small accent spots behave differently from areas.
- *Our interpretation:* measure how far the lightest and darkest significant areas sit from middle grey, and report small accents separately.
- *Implication for vector software:* an overlay could compute luminance (for example CIE L*) of the rendered art and report the extremes by area percentile (say 1st and 99th), list small extreme accents separately, and show how symmetric the range is about mid-grey. Limits: rendered values depend on colour management and display. A percentile cut can hide an intended accent. There should be no "fix" button that stretches tones.
- *Do not algorithmize because:* the author himself declines to make it a rule. The readout informs; the artist decides.
- *Disposition candidates:* analysis-overlay; agent-prompt-or-resource (check the tonal range before finishing).
- *Science caveat:* "a rightness about it that is instinctively felt" is aesthetic opinion. The claim about Rembrandt's whites "when fresh" is conjecture.

### SP-XV-6 Warm against cold: support a hot colour with cool and neutral tones
- *Author says:* Split the spectrum roughly in half: reds, oranges and yellows are warm; purples, blues and greens are cold. Playing them against each other gives colouring vitality. "The further your colouring goes in the direction of warmth, the further it will be necessary to go in the opposite direction" (p. 223). Examples: Titian's masses of cold blue; the "Blue Boy" in blue against warm brown. Red huntsmen painted on hot backgrounds are "screaming". Support a strong red with as many cool and neutral tones as possible and avoid hot shadows. Blue may be too strong a contrast unless the canvas is big enough to place it at a distance. Most painters keep to middle courses: more freedom, less power. The middle range ("colours partly neutralised by admixture with their opposites") is safer for beauty and refinement (p. 224).
- *Figures:* none.
- *Conditions / counter-examples:* opposites placed side by side can be "too much", and distance mitigates. The middle range is safer but less forceful.
- *Our interpretation:* two measurable quantities: the warm/cool balance weighted by area and chroma, and the adjacency of strongly opposed hues.
- *Implication for vector software:* an overlay could classify fills as warm, cool or neutral by hue angle above a chroma threshold, report area-times-chroma totals, flag the most saturated extreme, and flag strongly opposed hues that touch. A colour command could "neutralise toward complement" by mixing in N% of the opposite. Limits: the warm/cool boundary is a convention (greens and purples are ambiguous), perceived temperature depends on context, and the overlay cannot tell whether an imbalance is intended.
- *Do not algorithmize because:* colour harmony is a judgement, and the rule is a tendency.
- *Disposition candidates:* analysis-overlay; improve-existing (colour harmony/guide variants that mix toward the complement); agent-prompt-or-resource.
- *Science caveat:* halving the spectrum into warm and cold is a painters' convention, not a physical property. "Admixture with their opposites" means pigment (subtractive) mixing, which does not map exactly onto RGB or Lab interpolation. The aesthetic verdicts are opinion.

### SP-XV-7 Interest can balance mass
- *Author says:* This balance concerns "the mental significance of objects rather than rhythmic qualities". The human figure is so interesting that a minute figure can balance a great mass. "Without the little figure the composition would be out of balance." (p. 225) Landscape painters use figures this way.
- *Figures:* Diagram XXVII shows two small landscape frames. In each, a heavy, scribbled black tree mass fills the left half and the top, and a low ground line runs across. The right frame adds one tiny upright figure standing on the empty right side, about two-thirds across. That figure is the only difference between the two.
- *Conditions / counter-examples:* the effect depends on how interesting the object is. The human figure is the strongest case.
- *Our interpretation:* visual weight is not just area times darkness. Meaning matters, and geometry cannot read it.
- *Implication for vector software:* any "visual weight" overlay (area × darkness × contrast, reported as a weighted centroid against the artboard centre) must say plainly that it ignores meaning. It should accept manual "interest weight" tags per object, so a small figure can be marked as heavy. Agent guidance: when a composition is lopsided, try a small meaningful focal element before adding more mass. Limits: the centroid is not felt balance, and the tool cannot know which object viewers care about or where any viewer will look.
- *Do not algorithmize because:* interest is mental significance and varies with culture and viewer.
- *Disposition candidates:* analysis-overlay (with stated limits and manual weights); agent-prompt-or-resource; design-principle.
- *Science caveat:* that people and faces attract attention is broadly consistent with later research. The claim that this "balances" a picture is the author's judgement, not a measurement.

### SP-XV-8 Balance variety against unity
- *Author says:* Variety brings expression, the picturesque and "the joyous energy of life". Unity relates that variety to underlying "bed-rock principles". How far unity dominates depends on the artist and the theme. "If the unity of his conception is allowed to exclude variety entirely, it will result in a dead abstraction" (p. 226). Variety without unity becomes "a riotous extravagance".
- *Figures:* none.
- *Conditions / counter-examples:* the mix is personal and depends on the theme.
- *Our interpretation:* a recapitulation of the book's central opposition from earlier chapters.
- *Implication for vector software:* framing for agent prompts. First set a unifying structure (a few masses, repeated directions), then vary within it, then check the variety is still subordinate. No metric is proposed.
- *Do not algorithmize because:* both terms are judged as a whole.
- *Disposition candidates:* design-principle; agent-prompt-or-resource.
- *Science caveat:* none (aesthetic theory stated as such).

---

## XVI Rhythm: Proportion

**Locator:** `…14264-h-3.htm.xhtml`. Anchors: `#CHAPTER_XVI`, `#Page_227` … `#Page_231`, then `#Page_235` … `#Page_238`. Figures: `#plate48` (`#id-835834584764099985`), `#diagram28` (`#id-2487283462621767763`, `#id-109785992922800858`, `#id-6601199228601475671`), `#plate49` (`#id-514941788366787293`). The text cross-references "Appendix, page 289".

### SP-XVI-1 Canons of proportion are scaffolding to depart from
- *Author says:* Rules and canons meant to reduce beauty to formula "have not been a great success". A canon such as one for the human body can still serve "offering some standard from which he can depart at the dictates of his artistic instinct." (p. 227) Ancient sculptors and Leonardo used such canons. Science lifts work to "a high point of mechanical perfection", but the life, the "dither", still has to be accounted for. Proportion is "the quantitative relation of the parts to each other and to the whole" and deserves the greatest consideration in planning. Yet "there is something chilling to the artist in an array of dead figures" (p. 228).
- *Figures:* none.
- *Conditions / counter-examples:* a canon helps only as initial scaffolding.
- *Our interpretation:* proportion reference data must ship as optional, departable guides, never as correction targets.
- *Implication for vector software:* ratio rulers and canon overlays should not snap by default. They should state deviation neutrally (for example "head = 1/7.3 of height") and never auto-correct a drawing toward a canon. Agent rule: use a canon to block in, then depart from it deliberately to fit the subject.
- *Do not algorithmize because:* the author says the "dither" is beyond science.
- *Disposition candidates:* design-principle; improve-existing (guides and grids optional, non-snapping by default); agent-prompt-or-resource.
- *Science caveat:* the idea that mathematics underlies beauty is presented as "very ancient" and unresolved. The hope pinned on wavelength science is speculation.

### SP-XVI-2 Simple proportion gives repose and sublimity; varied proportion gives life
- *Author says:* He offers this hedged: "I can only give my own feeling on the matter; but I think in some degree they have" (p. 228). Proportions that resolve into a simple, easily grasped unity give repose and sublimity. Variety that stops the eye grasping a simple whole gives "lively restlessness". "Unity makes for sublimity, while variety makes for the expression of life." (p. 229) Scale matters for real objects (a sublimely proportioned dog-kennel versus a temple), but a picture can give an impression of great size at small scale.
- *Figures:* Diagram XXVIII (see SP-XVI-4).
- *Conditions / counter-examples:* explicitly his own feeling. Scale matters for objects, less for pictures.
- *Our interpretation:* "easily grasped" is roughly a small-integer ratio among the main divisions.
- *Implication for vector software:* an overlay could list the main divisions of the artboard (major horizontals and verticals, edges of the largest shapes) as fractions and flag small-integer ratios (1/2, 1/3, 2/3, 1/4…) within a tolerance. Limits: the tolerance is arbitrary, which lines count as "main" is a guess unless the user picks them, and a flag is not a fault.
- *Do not algorithmize because:* the mapping to emotion is the author's admitted feeling.
- *Disposition candidates:* analysis-overlay; design-principle.
- *Science caveat:* the link from proportion to sentiment is self-declared opinion.

### SP-XVI-3 Avoid equal proportions except for deliberate stillness
- *Author says:* The half is the easiest proportion to grasp and the most devoid of variety and life. It is used only for "great repose and aloofness from life", and even then with variety in the minor parts. Thirds, quarters and other equal divisions behave the same way, to a lesser degree. "Nature seems to abhor equalities, never making two things alike or the same proportion if she can help it." (p. 229) Architecture repeats equal forms (columns, windows, mouldings) because repose matters more there. Even so, the best work varies them: the outer Greek columns stand closer together and lean slightly inward, and repeated forms are "infinitely varied in themselves". Equality is "seldom" found in the main distribution of large masses (p. 230). The passage includes a digression against "systems founded on equalities" in social reform.
- *Figures:* Diagram XXVIII A, B, C, G.
- *Conditions / counter-examples:* equality is legitimate for repose and for effects "remote from nature and life". Architecture tolerates it more than painting.
- *Our interpretation:* equal intervals are a tool for stillness. The main divisions should usually be unequal.
- *Implication for vector software:* vector tools make equality the default: distribute evenly, align to centre, equal blend steps, repeat grids, mirror. Each could gain a "vary" counterpart: progressive or irregular spacing, bounded seeded variation, size variation in repeats. An overlay could flag exact halves and equal spacing among top-level shapes. Agent guidance: don't put the horizon or main subject at an exact half unless stillness is wanted. Limits: the flags detect geometric equality, not whether the result reads as dead.
- *Do not algorithmize because:* whether stillness is wanted is the artist's intent.
- *Disposition candidates:* improve-existing (distribute/repeat/blend with varied spacing); analysis-overlay (equality flags); agent-prompt-or-resource.
- *Science caveat:* "Nature seems to abhor equalities" and the social-reform remark are rhetoric. The Greek column refinements are real architectural practice, but we have not verified his exact description.

### SP-XVI-4 Diagram XXVIII: the same elements placed equally and varied (with a viewing method)
- *Author says:* The diagram uses a horizon, posts and bird spots. A puts the horizon across the centre, a post in the middle and two matching spots: "the deadest and most static of results". B (thirds) and C (quarters) are static, though less completely. D, E and F use the same elements varied in size and position, with "no obvious mechanical relationship", and have much more life. G adds more elements at equal distances and is dead. H and I vary them so the eye grasps no relationship, and look lively. "As the amount of variety increases so does the life and liveliness of the impression." (p. 236) Because every line is vertical or horizontal (the calmest directions), a static effect persists throughout. To view them, cover all but one, because "they affect each other when seen together" (p. 231).
- *Figures:* all three images viewed; positions are my estimates. **A:** horizon at mid-height, a single post at the exact centre, two dots mirror-placed upper left and upper right. **D:** horizon lower (about 60% down), a shorter post right of centre (about 63% across), two dots at different heights. **G:** six equal posts at equal spacing, six dots in an even row. **B:** horizon low, two posts at the thirds with a dot above each. **E:** two posts of different lengths at about 20% and 70%, dots at different heights. **H:** about eight posts of very different lengths in uneven clusters, scattered dots. **C:** three equal posts at the quarters, three dots in a row. **F:** three posts of varied height at irregular positions, three scattered dots. **I:** many posts of varied height at irregular positions, scattered dots.
- *Conditions / counter-examples:* only horizontals and verticals are used.
- *Our interpretation:* a clean, reproducible exercise.
- *Implication for vector software:* a practice template: a frame with a horizon, N posts and N dots, done once "equal" and once "varied". View one variant at a time, as he instructs (isolation or solo-artboard view). An equality detector could count equal intervals and lengths among selected objects. Limits: the detector can say "intervals equal within 1%". It cannot say "lively".
- *Do not algorithmize because:* "life" is an impression. Random jitter is not designed variety (see SP-XXI-2).
- *Disposition candidates:* practice-exercise; analysis-overlay; improve-existing (isolation/solo view for comparing variants).
- *Science caveat:* the diagrams are a demonstration, not an experiment. The "life" response is the author's and the reader's impression.

### SP-XVI-5 Centralised composition: take repose from symmetry, then hide the equality
- *Author says:* Madonna pictures often place the figure centrally with equal spaces either side. "Having got the repose this centralisation gives, everything is done to conceal this equality" (p. 231). In the Ansidei Madonna the throne is exactly central, and its width equals the space on each side, making three equal divisions. Circular arches ("the least possible amount of variety"), the horizontal steps, the vertical throne and the rows of beads add calm. The figures are varied. All the heads swing to the right, balanced by St Nicholas's strongly insisted staff on the right. St John's staff is made transparent, "as if made of glass", so it does not add to the swing. Fruit in the lower right is "dragged in" to offset the Christ on the left.
- *Figures:* Plate XLVIII, a tall arch-topped panel. The throne stands on a stepped plinth exactly central under a semicircular arch, with the Madonna and Child on it. St John stands at left, bare-legged, with a thin staff. St Nicholas stands at right, mitred and reading, holding a crozier. Strings of beads hang either side of the canopy. Small round fruits sit at lower right. The Madonna looks down toward the book on the right of the group. I did not check every head's direction against his "all the heads swing towards the right".
- *Conditions / counter-examples:* for pictures where "hush and reverence" are wanted.
- *Our interpretation:* symmetry for the big structure, asymmetry for the parts, with counterweights to compensate.
- *Implication for vector software:* symmetry and reflect tools could offer a follow-up step: detach the mirrored copy and vary contours and details on one side. A "mirror-difference" readout would show how far the two halves differ. Agent workflow: after a symmetric layout, vary the secondary elements and check that the line directions don't all swing one way. Limits: the readout measures geometric difference, not whether the variation balances.
- *Do not algorithmize because:* choosing counterweights (a staff, some fruit) is compositional judgement.
- *Disposition candidates:* multi-step-workflow; improve-existing (reflect/symmetry with an optional "vary the copy"); analysis-overlay (mirror-difference).
- *Science caveat:* what Raphael intended ("introduced at the last moment") is the author's conjecture.

### SP-XVI-6 The formality of the design should suit the degree of naturalism
- *Author says:* In his "humble opinion", the Ansidei's obvious artifice and severe convention clash with its naturalistic detail and solidity. Early Italian remoteness from natural appearance suited formal compositions and religious subjects. "The small amount of truth to visual nature in the work of earlier men went better with the formality of such compositions." (p. 235)
- *Figures:* Plate XLVIII.
- *Conditions / counter-examples:* stated as his opinion.
- *Our interpretation:* match how stylised the rendering is to how formal the arrangement is.
- *Implication for vector software:* agent guidance. A strongly symmetric, emblematic layout pairs with flat, stylised rendering. Naturalistic rendering wants a less formal arrangement. Not measurable.
- *Do not algorithmize because:* stylistic coherence is a judgement.
- *Disposition candidates:* design-principle; agent-prompt-or-resource.
- *Science caveat:* none beyond its being opinion.

### SP-XVI-7 Dynamic composition: varied proportions and lines that converge on the focus
- *Author says:* Tintoretto's "Finding of the Body of St. Mark" is all life and movement. "Nowhere does the eye grasp any obvious mathematical relationship" (p. 236). Its arches match the Raphael's but are placed off-centre, with varied lines and their calm broken by flickering lights. The lines of the architecture and the kneeling figure's arm converge on the Apostle's outstretched hand. He traces the eye down St Mark, past the recumbent figure, along the band of light on the floor to the frightened group, and back. Quantities are "thrown off apparently haphazard from lines leading the eye round the picture". Light and shade are strongly contrasted.
- *Figures:* Plate XLIX. A deep vaulted gallery recedes from the upper centre-left. A tall figure stands at far left with his arm raised. A nude body lies in the left-centre foreground and a robed figure kneels at centre. At right, a cluster of figures lowers a body from a wall tomb. A pale band of light crosses the floor toward the right-hand group. Dark and light are strongly opposed.
- *Conditions / counter-examples:* for energy and drama, the opposite of SP-XVI-5.
- *Our interpretation:* convergence is geometric and can be checked. The eye path is one expert's reading.
- *Implication for vector software:* a line-extension overlay could extend selected straight edges and mark where they meet, showing whether leading lines converge on the intended focal object. Limits: it shows geometric convergence only. It must never claim to predict a viewer's eye path or order of fixation.
- *Do not algorithmize because:* eye-path claims are interpretive. Do not produce "gaze path" predictions.
- *Disposition candidates:* analysis-overlay (line extension and convergence); agent-prompt-or-resource; design-principle.
- *Science caveat:* the eye-path narration is introspective. Viewing paths vary between viewers and tasks, so do not present it as fact.

### SP-XVI-8 5:8, a proportion with no obvious mathematical relationship
- *Author says:* A pleasing proportion found often in nature and art is "roughly" 5 to 8. "In such a proportion the eye sees no mathematical relationship." (p. 237) Below 5 it nears 4:8, which makes the smaller part one-third of the total, "a dull proportion". Above 5 it nears equality. He mentions a German proportional compass "said to contain the secret of good proportion": "there is certainly something remarkable about it" (pp. 237–238). He points to the Appendix, p. 289.
- *Figures:* Diagram XXIX (Appendix).
- *Conditions / counter-examples:* "roughly stated". The band lies between the dull third and near-equality.
- *Our interpretation:* the working principle is "avoid obvious ratios" more than "use 0.618 exactly". 5:8 sits in a band between 1:2 and 1:1.
- *Implication for vector software:* an optional ratio-guide preset offering φ divisions (0.382 and 0.618) alongside thirds, non-snapping by default. Limits: the author gives no tolerance, and there is no reliable evidence that viewers prefer it (see SP-A-4).
- *Do not algorithmize because:* compositions must not be scored by their closeness to φ.
- *Disposition candidates:* improve-existing (guide/ratio presets); design-principle.
- *Science caveat:* **strong.** Modern evidence for a special aesthetic preference for the golden ratio is weak and inconsistent, and "often occurs in nature" is widely overstated. The compass anecdote is hearsay.

### SP-XVI-9 A few dominant quantities control the variety
- *Author says:* Variety should be "under the control of a few simple, dominant quantities" that give a readily grasped unity, except where violence is wanted. The simpler the proportion, the more sublime; the more complicated, the livelier. From a few large proportions the eye can go on to enjoy the smaller varieties. "In good proportion the lesser parts are not allowed to obtrude, but are kept in subordination to the main dispositions" (p. 238).
- *Figures:* none.
- *Conditions / counter-examples:* except where "violence and lack of repose are wanted".
- *Our interpretation:* a hierarchy. Fix two to four main divisions or masses first, then nest subordinate detail inside them.
- *Implication for vector software:* agent workflow: block in the few largest shapes and their proportions on their own layer, check them, then subdivide. An overlay could list the top N shapes by area share, showing whether a few dominate or many compete. Limits: area is a proxy. Prominence also depends on contrast and interest (SP-XV-7).
- *Do not algorithmize because:* choosing the dominant quantities is the composition itself.
- *Disposition candidates:* multi-step-workflow; analysis-overlay; agent-prompt-or-resource.
- *Science caveat:* none beyond aesthetic opinion.

---

## XVII Portrait Drawing

**Locator:** `…14264-h-3.htm.xhtml`. Anchors: `#CHAPTER_XVII`, `#Page_239` … `#Page_255`. Sidenotes: "Expression." (line 7787, p. 252) and "Treatment of Clothes." (line 7825, p. 253). Figures: `#plate50` (`#id-5892653204816325220`), `#plate51` (`#id-5938484319636555513`), `#plate52` (`#id-8664878836940027326`), `#plate53` (`#id-7627363724287135717`).

**Warnings in this chapter against forcing faces into standard proportions** (every one found; each is also covered by a principle below):
1. Individuality "is likely for a long time to defy the analysis of science" (p. 239) → SP-XVII-1.
2. The retouching photographer "stipples out" characteristics; interest and vitality are lost and no beauty is gained (p. 239) → SP-XVII-2.
3. What makes one object more beautiful than another is "something that is characteristic of the appearance of the one and not of the other" (pp. 239–240) → SP-XVII-2.
4. Likeness lies at "the opposite pole" from the common type (p. 240) → SP-XVII-1.
5. Approach the portrait through how the subject "differs from a common type". The tired eye drifts to "a dull uniformity" (p. 241) → SP-XVII-4.
6. Proportions down the centre line must be got right, yet "it is impossible to lay down hard and fast rules for these things" (p. 242) → SP-XVII-5.
7. Holbein is honest not to "a cold, mechanically accurate record" but to the vital impression (p. 248) → SP-XVII-11.
8. The deliberately unequal pupils in "Lady Audley". Plate L's caption: "every bit of variety is sought for, the difference in the eyes and on either side of the mouth" → SP-XVII-12.
9. Momentary expression fixed for ever is "terrible, and most unlifelike" (p. 252) → SP-XVII-14.

### SP-XVII-1 Likeness lies in what differs from the common type
- *Author says:* Something in each individual defies scientific analysis. The portraitist's concern is the effect of personality on outward form. Catching the likeness means seizing the things that belong only to this individual and expressing them forcefully. Things common to the species make a likeness to a type. "The individual likeness is not in this direction but at the opposite pole to it." (p. 240) It is remarkable that no two heads look exactly alike, given how narrow the range of difference is.
- *Figures:* Plate L, caption: "Note how every bit of variety is sought for, the difference in the eyes and on either side of the mouth, etc."
- *Conditions / counter-examples:* none. This is the chapter's foundation.
- *Our interpretation:* the strongest warning in the range against standard face proportions. Averaging moves a portrait toward the type, which is away from the likeness.
- *Implication for vector software:* no face templates that snap features to average positions, and no "beautify" or "normalise face" command. Any portrait proportion aid compares the user's own reference (a photo they own) with the user's own drawing, never with a population average. An agent drawing a specific person must not generalise toward a stock face. We can measure positions the user marks. We cannot judge likeness.
- *Do not algorithmize because:* likeness is a holistic perceptual judgement. Inferring anything about a real person from their face is also an ethical line.
- *Disposition candidates:* design-principle; agent-prompt-or-resource; not-automatable (likeness itself).
- *Science caveat:* the recognition anecdotes are broadly consistent with modern findings that people are very good at recognising familiar faces. "Defy the analysis of science" is a 1913 position.

### SP-XVII-2 Smoothing away characteristics loses vitality and gains no beauty
- *Author says:* People think character is opposed to beauty, and sitters are shy of their features. The fashionable photographer "stipples out" any striking characteristic: "it is doubtful whether any beauty has been gained, and certain that interest and vitality have been lost" (p. 239). What makes one thing more beautiful than another is something characteristic of it, so even an artist who wants beauty must study individual characteristics.
- *Figures:* none.
- *Conditions / counter-examples:* none given.
- *Our interpretation:* this is the 1913 version of beauty retouching, and the warning applies directly to automatic smoothing.
- *Implication for vector software:* no automatic beautify or smooth pass on portraits. Path smoothing or simplification on facial contours should be explicit, scoped to a selection, previewable and reversible. Agents should not "clean up" a traced face by default, because smoothing every contour removes identity. Limits: a smoothing tolerance cannot know which irregularities are character.
- *Do not algorithmize because:* telling character from noise is a judgement.
- *Disposition candidates:* design-principle; agent-prompt-or-resource; improve-existing (smooth/simplify with preview and scope).
- *Science caveat:* the idea that beauty lies in the characteristic is philosophical opinion.

### SP-XVII-3 Likeness is a chord of the whole, not a sum of features
- *Author says:* With only the eye visible, it is almost impossible to recognise even a friend, or to tell laughing from crying. The lower face alone is hard too. A remembered head is an impression of everything combined, "a sort of chord of which the features will be but the component elements." (p. 241) Draw each part in relation to this chord. The moment the eye gets interested solely in one part, the likeness suffers.
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* detail work has to be checked continually against the whole.
- *Implication for vector software:* keep a whole-picture "distance view" (navigator thumbnail or small preview) visible during close work. Agents, after editing one feature, re-render the whole and compare it with the whole reference rather than the feature alone. The tool only shows; it does not judge.
- *Do not algorithmize because:* the impression is holistic.
- *Disposition candidates:* improve-existing (navigator / second small view kept live during zoomed work); multi-step-workflow; agent-prompt-or-resource.
- *Science caveat:* consistent with later research on holistic (configural) face processing, but stated here from experience.

### SP-XVII-4 Look for the differences first, while the eye is fresh
- *Author says:* Approach the portrait through the differences. First seek where the subject "differs from a common type" in general disposition and proportion, and leave what all heads share for later, because the fresh eye sees differences more readily. "The tendency of a tired eye is to see less differentiation, and to hark back to a dull uniformity" (p. 241).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* in a generative agent the analogue of the "tired eye" is drift toward a default or average face. Countering it means checking explicitly against the reference.
- *Implication for vector software:* agent workflow: before drawing, write down 3–5 distinctive proportions or dispositions of this subject, taken from the reference, and verify those first. The product must never try to detect or claim to measure a user's fatigue.
- *Do not algorithmize because:* deciding what is distinctive needs perception of this subject.
- *Disposition candidates:* agent-prompt-or-resource; multi-step-workflow; practice-exercise.
- *Science caveat:* the tired-eye claim is studio experience, not verified science.

### SP-XVII-5 Block in the disposition early, with no hard and fast rules
- *Author says:* Look first at how the features are disposed. Note the proportions "down an imagined centre line" of the brows, the base of the nose, the mouth and the chin. Block out the enclosing line of the face "in square lines". Get these right early, or a carefully drawn feature may have to be moved later. A head is recognised far off from the general disposition of its masses. Get an early idea of the skull shape and its relation to the face. "But it is impossible to lay down hard and fast rules for these things." (p. 242)
- *Figures:* none specific.
- *Conditions / counter-examples:* he names which landmarks to compare but never gives standard ratios for them.
- *Our interpretation:* measure the subject's own proportions, not a canon's.
- *Implication for vector software:* a "landmark ruler". The user draws a centre line and places ticks (brows, nose base, mouth, chin) on their reference and on their drawing, and the tool shows the two sets of ratios side by side, with no default "ideal" values. Blocking "in square lines" is a straight-segment block-in layer (pen or polygon). Landmarks stay user-placed, because automatic face-landmark detection would be biometric processing of real people.
- *Do not algorithmize because:* he refuses fixed rules. Automatic face analysis is also out of scope.
- *Disposition candidates:* engine-feature (reference-versus-drawing ratio ruler); multi-step-workflow; practice-exercise.
- *Science caveat:* none (method).

### SP-XVII-6 Order of work: flexible in point drawing, masses first in painting, far to near
- *Author says:* In point drawing, some begin with the eyes and some leave them to the end. Drawing the eye has "a mesmeric effect" that blinds one to the cold technical view of the head, so there is "a good deal to be said" for postponing it (p. 242). In point drawing you can begin anywhere, but in setting out a painting the general disposition of the masses comes first. Done well, the blocking already shows the sitter's character, and you can judge it by whether it suggests the original; if it doesn't, correct it before going further. Work by "working, as it were, from the general impression of the masses of the head as seen a long way off" (p. 243), adding detail and bringing the impression nearer.
- *Figures:* Plates LI and LII are the author's own red-chalk portraits. The heads are fully modelled in soft rubbed tone with crisp accents at the eyes, nostrils and moustache or beard. The coats are left as a few strong contour lines and areas of parallel diagonal hatching. Finish is concentrated in the head.
- *Conditions / counter-examples:* "point drawing" (drawing with a point: pencil, chalk) is flexible; painting is not.
- *Our interpretation:* masses, then check, then detail, with the likeness tested at every stage.
- *Implication for vector software:* a workflow of a masses layer, a likeness check at thumbnail size, then features. An agent may defer the eyes and other high-salience details until the masses are verified.
- *Do not algorithmize because:* judging the blocking against the sitter is perception.
- *Disposition candidates:* multi-step-workflow; agent-prompt-or-resource.
- *Science caveat:* the "mesmeric effect" is studio lore.

### SP-XVII-7 Insight: select the essential and subordinate the petty
- *Author says:* He compares portraiture to biography: a valet's honest biography of his master is not a great work, and Darwin's gardener thought his master merely "moons about all day". The value lies in insight: sorting the trivial from the essential and the common from the particular. "The fine portrait will express the larger and subordinate the petty individualities" (p. 244). The power to find these things "it is, of course, impossible to teach" (p. 245). The student can only study the best portraiture.
- *Figures:* none.
- *Conditions / counter-examples:* any painter's honest expression "will be of some interest".
- *Our interpretation:* what to emphasise in a portrait is the author's choice, not something to derive.
- *Implication for vector software:* when drawing a real person, an agent asks the user what to emphasise and what to simplify. It never guesses at the person's character.
- *Do not algorithmize because:* the selection is interpretive and concerns a real person.
- *Disposition candidates:* not-automatable; agent-prompt-or-resource (ask the user).
- *Science caveat:* none.

### SP-XVII-8 Inner character and carriage show in the form (period physiognomy)
- *Author says:* The artist need not grasp the sitter's mind, but must catch "the effect of this inner man on his outward appearance". "The habitual cast of thought in any individual affects the shape and moulds the form of the features" (p. 244). Head carriage is characteristic: "the strongly emotional man" carries his head high, while "the man of deep thought" bends it forward (pp. 244–245). Each sitter's characteristic action is usually absent at the first sitting. "A little diplomacy and conversational humouring" brings out the unconscious pose (p. 245).
- *Figures:* none.
- *Conditions / counter-examples:* the selection of form "is by no means apparent to all".
- *Our interpretation:* the usable part is observational: draw the person's actual, habitual carriage rather than a stiff pose. The character typology is not usable.
- *Implication for vector software:* agents should take pose and carriage from the reference, not from a stock pose. They must never infer or state character, emotion type, intellect or personality from facial form or posture, and the product must never offer such analysis.
- *Do not algorithmize because:* physiognomic inference is unsupported and harmful.
- *Disposition candidates:* design-principle (a prohibition); agent-prompt-or-resource (safety guidance); not-automatable.
- *Science caveat:* **strong.** That habits of thought mould facial form, and that head carriage separates "emotional" from "thoughtful" types, is physiognomy, which modern science does not support. Automated inference of character from faces is discredited and ethically fraught.

### SP-XVII-9 Sincerity: draw what you see; adopt a master's qualities only once you see them in nature
- *Author says:* "But he must never be insincere in his work." (p. 245) If the student does not appreciate the fine things in recognised masters, he should stick to honest portrayal of what he sees. Only when he "awakens to the sight in nature" of qualities he admired in others' work can he honestly bring them into his own.
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* a style borrowed without grounding in what is observed is insincere. The theme returns in SP-XXI-4 and SP-XXI-5.
- *Implication for vector software:* agents should not stamp a style ("in the style of X") onto a subject without grounding it in the subject's observed form. Study references; don't copy them (this project's clean-room rule).
- *Do not algorithmize because:* sincerity concerns the maker.
- *Disposition candidates:* design-principle; agent-prompt-or-resource.
- *Science caveat:* none.

### SP-XVII-10 Vitality must be balanced by repose (Frans Hals: sober colour, restless edges)
- *Author says:* The popular "striking presentment of the live person" sacrifices "repose, harmony of colour and form, and the wise ordering and selecting" for "a short-lived excitement". Such portraits depend on fresh paint and lose their value as it mellows, unless their design is good. Frans Hals balanced vigorous form with great sobriety of colour; in later work it is "little more than black and white, with a little yellow ochre and Venetian red". "It is this extreme reposefulness of colour that opposes the unrest in the form" (p. 246). Hals "never, if he can help it, lets an edge run smoothly", often leaving it jagged, which ties vitality to variety (pp. 246–247).
- *Figures:* none of Hals.
- *Conditions / counter-examples:* "some very wonderful modern portraits" were painted from the vitality point of view.
- *Our interpretation:* balance across channels: energy in one (gesture, edge) calls for restraint in another (colour). Edge variety is a concrete, controllable property.
- *Implication for vector software:* (a) a limited-palette constraint, for example four colours on the Hals model, to apply when gesture is energetic (see SP-XX-4). (b) Vector edges are uniformly smooth by default. Stroke width profiles, low-amplitude roughening and broken contours are honest ways to keep an edge "on the move", and an overlay could report the variance of stroke width per path. Limits: the number cannot tell expressive variety from noise.
- *Do not algorithmize because:* random roughening is not expressive variety (SP-XXI-2).
- *Disposition candidates:* improve-existing (palette-constraint presets; width profiles); agent-prompt-or-resource; design-principle.
- *Science caveat:* how paintings will age and whether colour "opposes the unrest in the form" are aesthetic judgements.

### SP-XVII-11 Be honest to the vital impression, not to a mechanical record (Holbein)
- *Author says:* Another point of view is the calm "expressive map of the individuality of those forms" (p. 247): a simple formal attitude drawn with searching honesty, as in the Holbein school. Without design it is dull literal transcription. Holbein's Windsor drawings are less literal than people think. They are "honest and accurate to the vital impression of the live sitter made on the mind of the live artist." (p. 248) Such a drawing differs from one made with "the cold, mechanical accuracy of a machine". Among the faces passed in a crowd, one suddenly rivets attention and its memory lingers, and Holbein's best drawings seem seen in such flashes.
- *Figures:* Plate L, Holbein in red chalk. A three-quarter head in a cap, facing viewer-left. The face is modelled with fine hatching and soft tone, and the garments are a few long contour lines. The two eyes visibly differ in opening and shape, and the two corners of the mouth differ.
- *Conditions / counter-examples:* the "map" point of view needs design, or it turns dull.
- *Our interpretation:* tracing a photo yields the mechanical record. The vital impression is the artist's.
- *Implication for vector software:* agents treat a reference as evidence, then propose deliberate, subject-based adjustments for the user to approve. They never claim the output captures anyone's "impression".
- *Do not algorithmize because:* the vital impression exists in the artist's mind.
- *Disposition candidates:* not-automatable; design-principle.
- *Science caveat:* the "flash" account of face memory is introspective.

### SP-XVII-12 Don't force symmetry: paired features are seldom seen as the same
- *Author says:* In Holbein's "Lady Audley", the pupil of the left eye is larger. "I do not suppose that as a matter of mechanical accuracy this was so" (p. 248), but "the impression of the eyes seen as part of a vivid impression of the head is seldom that they are the same size." (p. 249) Holbein first drew them equal, then while "pulling it together" deliberately drew a line outside the original, enlarging the pupil. The author reads this as done "at the dictates of the vivid mental impression", "to my thinking". He adds that it is "not at all clearly seen in the reproduction".
- *Figures:* Plate LIII, "The Lady Audley". The head is turned three-quarters to viewer-right, with a dark mass of hair under a pale hood and a delicately modelled face. Hood, body and jewels are sparse outline. The title is lettered across the top. **I could not see a clear difference in pupil size in this reproduction**, and the author says the same. Plate L shows asymmetry more visibly (see SP-XVII-11).
- *Conditions / counter-examples:* explicitly his interpretation.
- *Our interpretation:* mirroring paired features is a shortcut that kills the impression.
- *Implication for vector software:* reflect and symmetry modes should not be the default for faces or paired features. If an agent mirrors a feature to save effort, it must then vary the copy to match the reference. A mirror-difference readout can show how symmetric a pair is, for information only. Limits: it cannot say what degree of asymmetry is right.
- *Do not algorithmize because:* the right variation follows the particular subject. Random asymmetry is not the same thing.
- *Disposition candidates:* agent-prompt-or-resource; design-principle; improve-existing (symmetry tool: one-step "detach and vary").
- *Science caveat:* real faces are measurably asymmetric, but the claim here is about impression and intent, and it is the author's reading of Holbein.

### SP-XVII-13 Points of view (symbol, symphony, official): let one dominate
- *Author says:* **Symbol of the person within** (Watts): Watts found out all he could about the sitter's mind. Of the two Browning portraits, Lehmann's is more "like" the man casual acquaintances saw, but "Watts's portrait is like the man who wrote the poetry, and Lehmann's is not." (p. 250) **Symphony of form and colour** (Whistler): a pose chosen for its pattern, with character "not allowed to obtrude". **Official portrait:** the dignity of the office, with more decorative pageantry and stateliness. The list is not complete, and the views are not incompatible. "One or other quality will dominate in a successful work" (p. 251). Combining too many loses directness, yet "no good portrait is without some of the qualities of all".
- *Figures:* none of these works are reproduced.
- *Conditions / counter-examples:* "the chances are against such a composite affair being a success" (p. 251).
- *Our interpretation:* each portrait needs one declared, dominant aim.
- *Implication for vector software:* an agent prompt asks for, or states, the dominant aim (vivid likeness, objective map, symbol, decorative pattern, ceremonial) and lets it govern palette, pose and ornament. Not measurable.
- *Do not algorithmize because:* it is a matter of intent.
- *Disposition candidates:* agent-prompt-or-resource; design-principle.
- *Science caveat:* none. The period condescension ("many ladies … their principal characteristic was the charm of their appearance", pp. 250–251) must not be reproduced in any guidance.

### SP-XVII-14 Expression: enduring, not momentary
- *Author says:* The instantaneous camera has taught people to expect a momentary expression in a portrait, the faint smile above all. Fixed for ever, a passing expression "is terrible, and most unlifelike". "A portrait is a panel painted to remain for centuries without movement." (p. 252) So a large amount of repose must enter it, and portraits without it "pall on one if constantly seen". The real expression is enduring: it belongs to the forms and to the marks left by life. Passing expressions are muscle contractions, "the effect of which is very similar in most people" (p. 253).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* artwork meant to be lived with wants a resting expression.
- *Implication for vector software:* agent guidance: for characters, mascots, icons and portraits that will be seen repeatedly, prefer a resting expression unless the brief asks for a momentary one. Not measurable.
- *Do not algorithmize because:* none needed; this is guidance.
- *Disposition candidates:* agent-prompt-or-resource; design-principle.
- *Science caveat:* "very similar in most people" simplifies a contested question about how universal facial expressions are. Do not cite it as fact.

### SP-XVII-15 Clothes: draw the timeless laws of drapery and design, not fashion trivia
- *Author says:* Sitters fear modern clothes will look dated, and they do if fixed without selection. Drapery folds "follow laws … that are the same in all times" (pp. 253–254), so express the figure through the drapery. Work to a thought-out design selected from the sitter's varied appearance, "trying one thing after another". Extreme fashions can give "very piquant" designs. "The principles governing good designs are the same in all times" (p. 254). Only fashion detail pursued for its own sake dates. "Meaningless, so-called classical draperies" are "a feeble one" (p. 255).
- *Figures:* Plate LIV (drapery study, see SP-XX-11) is relevant, though it is printed in XVIII.
- *Conditions / counter-examples:* none.
- *Our interpretation:* fold structure and the large pattern of the garment come before ornament.
- *Implication for vector software:* agents drawing clothing work out the fold structure over the body and the overall pattern of garment shapes before decorative detail, and try several arrangements as variant artboards before committing.
- *Do not algorithmize because:* choosing the arrangement is design judgement.
- *Disposition candidates:* agent-prompt-or-resource; multi-step-workflow (variants).
- *Science caveat:* none.

### Procedure XVII: setting out a portrait (pp. 241–243, 245, 249, 262)
1. While the eye is fresh, look first for where this head differs from the common type, in disposition and proportion [SP-XVII-4, SP-XVII-1].
2. Note the proportions down an imagined centre line: brows, base of the nose, mouth, chin [SP-XVII-5].
3. Block out the enclosing line of the face in square lines [SP-XVII-5].
4. Get an early idea of the skull's shape and its relation to the face [SP-XVII-5].
5. In point drawing, start anywhere that interests you, either anchoring on an eye or leaving the eyes until last to avoid their "mesmeric effect". In painting, construct the general disposition of the masses first [SP-XVII-6].
6. Judge the blocking by whether it already suggests the sitter. If it doesn't, correct it before going on [SP-XVII-6; see SP-XIX-4].
7. Work from the impression of the head "seen a long way off", adding detail and bringing it nearer, with every part drawn in relation to the whole chord [SP-XVII-6, SP-XVII-3].
8. Watch for the sitter's characteristic carriage, which appears once they are at ease [SP-XVII-8 (observational part only)].
9. In the early sittings, choose the aspect you mean to paint and afterwards work largely from the memory of it [SP-XVIII-7].
10. For clothes, try arrangements "one thing after another" until a suggestive design appears [SP-XVII-15].
11. At the end, "pulling it together" (his phrase, p. 249), move toward the vivid impression, which may mean departing from measured fact, as Holbein did with the pupil [SP-XVII-11, SP-XVII-12].

---

## XVIII The Visual Memory

**Locator:** `…14264-h-3.htm.xhtml`. Anchors: `#CHAPTER_XVIII`, `#Page_256` … `#Page_264`. Figure printed here: `#plate54` (`#id-3388341625681065354`), Plate LIV. It is referenced from XX (p. 281: "page 260") and described under SP-XX-11.

**Standing rule for this chapter (from the brief, and ours):** the product must never claim to measure, score, diagnose or improve a user's memory, attention or cognition. Any comparison it offers is the user's own check of geometric difference, started by the user, and worded as "difference in position or size", never as a "memory score".

### SP-XVIII-1 Memory is the storehouse, and pictures colour what we see
- *Author says:* "The memory is the great storehouse of artistic material" (p. 256). From early years the artist stores impressions from nature, from works of art, from anything seen, and imagination draws on that store. The memory of pictures colours how we see nature: after the National Gallery he saw Trafalgar Square as Veronese or Turner would have painted it, "the memory of their work colouring the impression the scene produced" (p. 256). He then sets pictures aside ("indirect or second-hand impressions", p. 257) to discuss direct visual memory of nature.
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* references shape output, for people and for models.
- *Implication for vector software:* a reference board belonging to the user (images they own or that are licensed) fits the way artists work. Agents should know that references bias their output, and should study them for qualities, not copy them (clean-room).
- *Do not algorithmize because:* it concerns the artist's personal store.
- *Disposition candidates:* design-principle; not-automatable.
- *Science caveat:* the Trafalgar Square episode is introspective anecdote. Priming by recently seen images is plausible but is not established by this account.

### SP-XVIII-2 Memory keeps the essential; repeated on-the-spot study dulls the impression
- *Author says:* The Pre-Raphaelites sought out every detail; the Impressionists sought the whole effect and invented a light technique "founded on scientific principles". Painters have since realised that for "anything but very ordinary effects … this painting on the spot must give place to more reliance on the memory" (p. 257). "Memory has this great advantage over direct vision: it retains more vividly the essential things" (p. 257) and "has a habit of losing what is unessential" (p. 258). Facing the same subject day after day brings ever more commonplace impressions until the original emotion is lost, and the result is "a catalogue of tired objects more or less ingeniously arranged" (p. 259).
- *Figures:* none.
- *Conditions / counter-examples:* "very ordinary effects" can still be painted on the spot.
- *Our interpretation:* an early summary of the essentials should keep governing long work, and endless sampling of the subject should not replace it.
- *Implication for vector software:* agent workflow: capture an "intent sketch" first (the main masses and lines), check later stages against it, and stop accumulating reference detail once it no longer serves the intent.
- *Do not algorithmize because:* "essential" is defined by the artist's impression.
- *Disposition candidates:* multi-step-workflow; design-principle.
- *Science caveat:* **strong.** Modern memory research treats memory as reconstructive and schema-driven: it drops detail but also distorts and normalises (compare his own "dull uniformity", p. 241). "Retains more vividly the essential things" is not reliable as stated. The contrast of English "intense individualism" with the French "synthetic" and "logical" mind is national stereotype.

### SP-XVIII-3 Snapshot the first flash and let it guide the long work
- *Author says:* What makes one want to paint is "debatable and shadowy ground". The artist conveys rare moments of apprehending truth in things seen. These "flashes of inspiration" at the start of every vital picture "occur but seldom". "What the painter has to do is to fix them vividly in his memory, to snapshot them, as it were" (p. 258), so they guide the "toilsome procedure". The flash may come from nature or purely from imagination (a composition that flashes across the mind). However commonplace the subject, the flash clothes it in newness, "be it only an orange on a plate" (p. 259).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* a document should carry its own statement of intent.
- *Implication for vector software:* a pinned, user-written "intent note" with a first-sketch thumbnail per document, visible during later work (document info or a small panel). An agent records the brief's intended impression in one sentence and rereads it at each stage. We must never claim to capture the user's inspiration.
- *Do not algorithmize because:* the flash itself is inspiration.
- *Disposition candidates:* engine-feature (document intent note / pinned first-sketch view); agent-prompt-or-resource; not-automatable (the flash).
- *Science caveat:* none (subjective account).

### SP-XVIII-4 Memorise in the terms of the medium: lines and tones
- *Author says:* A drawing is "certain marks upon a paper". How easily a painter remembers an impression in a workable form depends on his power to analyse vision technically. The more he knows of "the anatomy of picture-making" (p. 259), the easier it is. Remembering the emotional side is not enough: "the scene must be committed to memory in terms of whatever medium you intend to employ for reproducing it" (p. 260), "as if you were actually drawing the scene on some imagined piece of paper in your mind".
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* translate the subject into the medium's vocabulary before making marks.
- *Implication for vector software:* agents translate a brief or reference into vector terms before drawing: the main lines, the masses, 3–5 values, colour notes and stroke strategy. This is a natural MCP prompt template ("plan in medium terms").
- *Do not algorithmize because:* the translation is a perceptual act. The template only structures it.
- *Disposition candidates:* agent-prompt-or-resource; multi-step-workflow; practice-exercise.
- *Science caveat:* none.

### SP-XVIII-5 The faculty can be trained: graded, daily memory drawing
- *Author says:* Like memorising poems or plays, visual memory "is capable" of much development (p. 260). Lecoq de Boisbaudran trained pupils, "beginning with very simple things like the outline of a nose" and moving to complex subjects by easy stages, "with the most surprising results". Students should "form a habit of making every day in their sketch-book a drawing of something they have seen" (pp. 260–261). "Don't be discouraged if the results are poor and disappointing at first". Remember the spirit of the subject. "Some scribbling and fumbling will necessarily have to be done", but aim to "draw as frankly and clearly as you can" (p. 261).
- *Figures:* none.
- *Conditions / counter-examples:* memory drawing will not be definite at first.
- *Our interpretation:* a graded exercise ladder.
- *Implication for vector software:* an optional practice mode with a graded series (simple outline → compound shape → small scene). The user studies, hides, draws, and may then reveal the source. If they ask, an overlay shows the difference in position and size. The app never scores or claims to measure memory.
- *Do not algorithmize because:* the faculty is the user's own and the app must not assess it.
- *Disposition candidates:* practice-exercise; not-automatable (memory itself).
- *Science caveat:* the trainability claims and Lecoq's "surprising results" are historical reports, not verified here. Make no claims about cognitive improvement.

### SP-XVIII-6 The memory exercise: enjoy it, analyse it flat, draw it in your mind
- *Author says:* With a subject too fleeting to draw on the spot: "Drink a full enjoyment of it, let it soak in" (p. 261), because the recollection will guide the drawing later. "This mental impression is not difficult to recall; it is the visual impression in terms of line and tone that is difficult to remember." Next consider it as a flat visual impression. Note the proportions, shapes and disposition of the main lines. "Do the whole drawing in your mind, memorising the forms and proportions of the different parts", and "fix it in your memory to the smallest detail". If only the emotional side is remembered, "you will be hopelessly at sea", since memory retains remarkably little of things constantly seen unless one tries to memorise their appearance (pp. 261–262).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* two passes, first emotional, then formal. The formal pass is the hard one.
- *Implication for vector software:* the steps of the practice mode (see Procedure XVIII). For an agent, the analogue is writing an explicit formal description of the reference (proportions, shapes, disposition) before drawing, instead of relying on a vague semantic label.
- *Do not algorithmize because:* the exercise is for the person.
- *Disposition candidates:* practice-exercise; agent-prompt-or-resource.
- *Science caveat:* poor recall of familiar objects is consistent with everyday experience, but here it is observation, not data.

### SP-XVIII-7 Work to a scheme even in front of nature; make a quick first sketch
- *Author says:* "The true artist, even when working from nature, works from memory very largely" (p. 262). He works to a scheme in tune with the first emotion: nature changes, but his intention does not, and he selects from nature only what supports it. "He is a feeble artist, who copies individually the parts of a scene" and expects the sum to make a picture. If possible, first make a rapid sketch of the main masses and lines under the first enthusiasm, to refresh the memory later. "It is seldom that the vitality of this first sketch is surpassed by the completed work" (p. 262). In portraits the sitter varies from day to day, so in the early sittings "select the aspect he means to paint and afterwards work largely to the memory of this" (p. 262).
- *Figures:* none.
- *Conditions / counter-examples:* "if circumstances permit".
- *Our interpretation:* keep the first sketch as the governing reference.
- *Implication for vector software:* keep a locked "first sketch" artboard or layer visible (side by side or as a thumbnail) throughout the job. An agent should not keep re-sampling parts of a reference piecemeal. When working from one of several photos, it chooses the aspect once and keeps to it.
- *Do not algorithmize because:* choosing the scheme is intent.
- *Disposition candidates:* multi-step-workflow; improve-existing (pin a locked sketch layer or artboard as a reference view); agent-prompt-or-resource.
- *Science caveat:* none.

### SP-XVIII-8 Decide the scheme at first sight, and don't switch for novelty
- *Author says:* "Always work to a scheme on which you have decided" (p. 262). The faculties are most active when the subject is first presented: decide the scheme then, take your fill of the impression, and learn the subject. Then work straight on, using nature to support the original impression. New schemes will occur to you, and "every new one has a knack of looking better than your original one" (p. 263), but mostly only because it is new. So memory serves even when working directly in front of nature.
- *Figures:* none.
- *Conditions / counter-examples:* "it is not often" that the new scheme is really better.
- *Our interpretation:* commit, and make changes as deliberate branches.
- *Implication for vector software:* agent guidance: fix composition and palette in a planning step. A mid-course change of scheme needs explicit user approval, and goes into a separate version or variant rather than drifting into the main work.
- *Do not algorithmize because:* whether a new scheme is better is a judgement.
- *Disposition candidates:* agent-prompt-or-resource; multi-step-workflow.
- *Science caveat:* the novelty claim is introspective.

### SP-XVIII-9 Matter and manner must both be memorised, then fused
- *Author says:* A subject has two aspects: the sensuous pleasure and spiritual significance, and "the lines, tones, shapes, &c., and their rhythmic ordering", "the matter and manner, as they may be called" (p. 263). "Both these aspects must be memorised, and of the two the second will need the most attention." They are two aspects of one thing and must unite in the act. "The craftsman mind, so laboriously built up, should by now have become an instinct, a second nature" (p. 264). When it works, the right strokes come and the artist feels only "a fierce joy".
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* a plan needs both the intended effect and the formal means.
- *Implication for vector software:* an agent's plan states the intended effect in words (matter) and the formal means (lines, tones, shapes, rhythm: manner). The fusion is not a software function.
- *Do not algorithmize because:* the fusion is the artist's act.
- *Disposition candidates:* agent-prompt-or-resource; not-automatable.
- *Science caveat:* none.

### Procedure XVIII: the memory exercise (pp. 260–263)
1. Make a daily habit of one sketch-book drawing, from memory, of something seen that interested you [SP-XVIII-5].
2. Grade the difficulty: begin with very simple things (the outline of a nose) and move to complex subjects by easy stages, after Lecoq de Boisbaudran [SP-XVIII-5].
3. With a fleeting subject that moves you, first drink a full enjoyment of it and let it soak in [SP-XVIII-6].
4. Then consider it from the material side, as a flat visual impression [SP-XVIII-6, SP-XVIII-4].
5. Note the proportions of the main lines, their shapes and their disposition, as if you were drawing it [SP-XVIII-6].
6. Do the whole drawing in your mind, in the terms of your medium (lines and tones for a drawing), memorising the forms and proportions of the parts "to the smallest detail" [SP-XVIII-6, SP-XVIII-4].
7. Draw it. Try especially to remember the spirit. Accept some scribbling and fumbling, but aim to draw frankly and clearly [SP-XVIII-5].
8. Don't be discouraged by poor early results; persevere [SP-XVIII-5].
9. When working from nature, make a rapid first sketch of the main masses and lines under the first enthusiasm, if circumstances permit, and keep it to refresh the memory [SP-XVIII-7].
10. Decide the scheme when the subject is first presented, then work straight on, using nature to support it and not switching to new schemes [SP-XVIII-8].
11. Memorise both the matter and the manner, giving the manner more attention [SP-XVIII-9].

*Product translation (ours):* a practice mode runs steps 2 and 4–8: show the subject, hide it, the user draws, and the user may optionally reveal the subject and toggle an overlay. There is no scoring, timing pressure or memory claim.

---

## XIX Procedure

**Locator:** `…14264-h-3.htm.xhtml`. Anchors: `#CHAPTER_XIX`, `#Page_265` … `#Page_270`. No figures.

### SP-XIX-1 Know what you want before touching the paper
- *Author says:* Don't flounder "in the hope that something will turn up". "It is seldom if ever that an artist puts on paper anything better than he has in his mind before he starts" (p. 265). "Don't spoil the beauty of a clean sheet of paper by a lot of scribble"; aim at "making the paper more beautiful by every touch you give". "To know what you want to do and then to do it is the secret of good style and technique." He criticises students who pin up paper, draw a line down the middle, take measurements and block in without thought, "as a clerk would copy a letter already drafted for him" (p. 265). Guide lines, measurement and blocking in are "very necessary in academic work, if rather fettering to expressive drawing" (p. 266), and even there intelligence must be used.
- *Figures:* none.
- *Conditions / counter-examples:* mechanical setting-out is acceptable, indeed needed, in academic study.
- *Our interpretation:* plan first, then execute.
- *Implication for vector software:* agents write a short plan (composition, masses, line strategy) before any drawing call. Avoid generating many random strokes and pruning afterwards. For people, an optional planning thumbnail.
- *Do not algorithmize because:* the mind's-eye image belongs to the maker.
- *Disposition candidates:* agent-prompt-or-resource; multi-step-workflow.
- *Science caveat:* none.

### SP-XIX-2 Be moved first: truth perceived intelligently, not camera truth
- *Author says:* Look well at the model first and be moved by something fine or interesting in the form. See in your mind's eye what drawing you mean to make. In school studies be "unflinchingly honest to the impression", but "dismiss the camera idea of truth from your mind". Instead of becoming a mechanical copying instrument, "let your drawing be an expression of truth perceived intelligently." (p. 266)
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* identify what is interesting before drawing it.
- *Implication for vector software:* an agent names what the drawing is about (the interesting quality) before drawing. Image-trace output is presented as a scaffold, not as the drawing (see SP-XXI-1).
- *Do not algorithmize because:* being moved is not a function.
- *Disposition candidates:* agent-prompt-or-resource; design-principle.
- *Science caveat:* none.

### SP-XIX-3 The first strokes decide; restart rather than build on a bad start
- *Author says:* "Be extremely careful about the first few strokes": "the quality of your drawing is often decided in these early stages." (p. 266) Vital, expressive strokes can be developed; feeble ones rarely can. If the start is bad, "turn your paper over and start afresh, trying to seize upon the big, significant lines and swings in your subject at once".
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* the first strokes should be the big directional lines ("swings").
- *Implication for vector software:* agents lay down the big directional lines first. If the first pass is weak, they start a fresh layer or artboard rather than patching. The app could offer a quick "new attempt": a blank duplicate artboard beside the old one, which is kept.
- *Do not algorithmize because:* "vital" is a judgement.
- *Disposition candidates:* multi-step-workflow; agent-prompt-or-resource.
- *Science caveat:* none.

### SP-XIX-4 Correct early: go back to the last stage that was right
- *Author says:* "It is much easier to put down a statement correctly than to correct a wrong one" (p. 266), so take out the whole part if convinced it is wrong. Make direct, accurate statements, and "don't waste time trying to manoeuvre a bad drawing into a good one". Stop as soon as you go wrong. On a walk you don't follow a wrong road hoping it leads right. "Stop and rub out your work until an earlier stage that was right is reached" (p. 267) and start again from there. A trained eye notices sooner.
- *Figures:* none.
- *Conditions / counter-examples:* see SP-XIX-7, which says not to rub out in expressive drawing. Our reading is that XIX-4 concerns wrong foundations, early stages and study work.
- *Our interpretation:* this maps directly onto checkpoints and reverting.
- *Implication for vector software:* named "known good" checkpoints in history, with "return to checkpoint" that keeps later work on a branch rather than discarding it. Agents verify each stage (render, then look) and revert to the last verified checkpoint instead of piling corrections onto a wrong base.
- *Do not algorithmize because:* the tool records states; deciding that a state is wrong is perception.
- *Disposition candidates:* engine-feature (history checkpoints / branches; check `crates/engine/src/history.rs` for what exists); multi-step-workflow; agent-prompt-or-resource.
- *Science caveat:* none.

### SP-XIX-5 Rest the eye; don't labour when the mind isn't working
- *Author says:* Don't work too long without a brief rest: "the eye often gets fatigued very quickly and refuses to see truly, but soon revives if rested a minute or two." (p. 267) Don't labour when the mind isn't working. "Pull yourself together, and ask what it is you are trying to express". "And if the mind is not working, the work of the hand will be of no account." (p. 267)
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* fresh looks are a working tool. The usual studio tricks (mirror view, small view, greyscale view) are ours, not his.
- *Implication for vector software:* optional fresh-look views (flip horizontally, thumbnail, greyscale), started by the user. The app must never detect, infer or prompt about fatigue.
- *Do not algorithmize because:* the user's state is their own.
- *Disposition candidates:* improve-existing (view flip / greyscale / navigator); not-automatable (fatigue).
- *Science caveat:* visual adaptation is real, but this is studio experience. Do not measure fatigue or claim to.

### SP-XIX-6 Detail must stay subordinate to the initial impulse
- *Author says:* One "has constantly to be making fresh effort" (p. 268). Especially in the final stages, "in adding details and small refinements, it is doubly necessary for the mind to be on fire with the initial impulse", or the main qualities are obscured and the result enfeebled (p. 268).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* finishing is where the big pattern gets lost.
- *Implication for vector software:* an overlay compares the current state with an earlier checkpoint at thumbnail size or in tone-simplified form (SP-XV-4) and shows whether late detail changed the big pattern. Agents, after adding detail, compare the blurred or reduced render with the earlier stage. Limits: a pixel difference is not "main qualities", so this is a prompt for human review.
- *Do not algorithmize because:* the impulse is the maker's.
- *Disposition candidates:* analysis-overlay (low-resolution checkpoint comparison); multi-step-workflow.
- *Science caveat:* none.

### SP-XIX-7 Don't rub out in expressive drawing; leave the wrong line
- *Author says:* "Do not rub out, if you can possibly help it, in drawings that aim at artistic expression." In academic work you may, but as little as possible. Rubbing out weakens a drawing, like stopping a witty remark to correct a word. "If a wrong line is made, it is left in by the side of the right one in the drawing of many of the masters." (p. 268) The aim is to "draw cleanly and fearlessly, hand and eye going together", which takes time.
- *Figures:* Plate LV (silverpoint, no erasing possible) shows clean, committed lines with no visible erasure.
- *Conditions / counter-examples:* academic work may rub out a little. Compare SP-XIX-4.
- *Our interpretation:* in vector work rubbing out is invisible (undo or delete), so the analogue is whether corrections stay visible and whether a stroke is laid down confidently or nudged endlessly.
- *Implication for vector software:* a "keep pentimenti" option that moves superseded strokes to a faint locked layer instead of deleting them; a practice mode with no undo (append-only). Agents prefer committing one confident replacement path over many micro-edits of one path.
- *Do not algorithmize because:* none needed; these are tool options.
- *Disposition candidates:* engine-feature (pentimenti layer option); practice-exercise (no-undo mode); agent-prompt-or-resource.
- *Science caveat:* none.

### SP-XIX-8 Accuracy first; facility follows; expression needs speed
- *Author says:* "Let painstaking accuracy be your aim for a long time." (p. 268) Facility and quickness then "come of their own accord". In expressive drawing quickness is essential, because the waves of emotion under which the eye really sees do not last. "There must be no hitch in the machinery of expression" (p. 269).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* two kinds of practice: slow and accurate, and quick.
- *Implication for vector software:* practice modes. An accuracy exercise: draw, then (user-initiated) compare with the reference in an overlay. A quick-sketch exercise with an optional timer. Never claim to measure skill beyond reporting geometric deviation.
- *Do not algorithmize because:* skill is not what the tool measures.
- *Disposition candidates:* practice-exercise.
- *Science caveat:* the duration of the "waves of emotion" is introspection.

### SP-XIX-9 Simplicity: every touch must mean something
- *Author says:* "Try and express yourself in as simple, not as complicated a manner as possible. Let every touch mean something" (p. 269). If you don't see what to do next, "don't fill in the time by meaningless shading and scribbling until you do". Wait, rest your eye by looking away, then find something right that needs doing.
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* economy of means. The theme recurs in SP-XXI-5.
- *Implication for vector software:* agent guidance: economy of strokes; no decorative hatching added to fill space or look "finished". A density readout (objects per area) could point to filler regions, but density is not meaninglessness, so it only prompts review.
- *Do not algorithmize because:* meaning is a judgement.
- *Disposition candidates:* agent-prompt-or-resource; design-principle.
- *Science caveat:* none.

### SP-XIX-10 Prime the eye with a master's drawing before you start
- *Author says:* Before beginning, study a master draughtsman whose work the subject suggests. Take it in with full enjoyment, and "your eye will unconsciously be led to see in nature some of the qualities of the master's work." (p. 269) You will see the subject as a finer thing. Reproductions are cheap, and "every student should begin collecting reproductions of the things that interest him" (p. 269).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* study is not copying.
- *Implication for vector software:* an MCP resource could hold a curated, licence-checked list of public-domain study references with notes on their qualities (line economy, massing). Users and agents study them and never trace or copy them into deliverables. Under this project's clean-room rules, never Adobe-authored material.
- *Do not algorithmize because:* "unconscious" learning of qualities is the person's.
- *Disposition candidates:* agent-prompt-or-resource; practice-exercise.
- *Science caveat:* the priming claim is introspective (compare SP-XVIII-1).

### SP-XIX-11 Nervous vitality and health
- *Author says:* He stresses "the extreme importance of nervous vitality in a fine draughtsman" (p. 269) and a healthy life. Given two men of equal ability, the healthy one wins. He rejects the idea that an artist must be dissipated: a conventional life is "deadening" to the artist, but an unconventional one need not be dissipated. Take care of "that most valuable of studio properties, vigorous health." (p. 270)
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* life advice, outside the product's scope.
- *Implication for vector software:* none. Do not build wellness monitoring or nudges.
- *Do not algorithmize because:* it is personal and outside the domain.
- *Disposition candidates:* not-automatable.
- *Science caveat:* "nervous vitality" is a period concept. This is not medical advice.

### Procedure XIX: making a drawing (pp. 265–269)
1. Optionally, first study a master's drawing that the subject suggests, with full enjoyment [SP-XIX-10].
2. Look well at the model and be moved by something fine or interesting in the form [SP-XIX-2].
3. See in your mind's eye the drawing you mean to do before touching the paper. Know what you want [SP-XIX-1].
4. In academic studies only, use guide lines, measurements and blocking in, with intelligence [SP-XIX-1].
5. Put down the first few strokes with extreme care, seizing the big, significant lines and swings [SP-XIX-3].
6. If the start is bad, turn the paper over and begin afresh [SP-XIX-3].
7. As soon as you feel you have gone wrong, stop, go back to the last stage that was right, and continue from there [SP-XIX-4].
8. When things won't come, rest the eye briefly by looking away [SP-XIX-5, SP-XIX-9].
9. If the mind isn't working, stop, ask what you are trying to express, fix it in your mind, then go on [SP-XIX-5].
10. Let every touch mean something. No filler shading [SP-XIX-9].
11. In expressive drawings don't rub out. Leave a wrong line beside the right one [SP-XIX-7].
12. In the final stages, re-kindle the initial impulse so that details do not obscure the main qualities [SP-XIX-6].
13. Over the long term, aim for painstaking accuracy first; facility and speed will follow [SP-XIX-8]. Look after your health [SP-XIX-11].

*Agent translation (ours):* plan (3) → big lines (5) → render and look → checkpoint or revert (7) → detail under the plan (10, 12) → compare with the checkpoint at small size (SP-XIX-6).

---

## XX Materials

**Locator:** `…14264-h-3.htm.xhtml`. Anchors: `#CHAPTER_XX`, `#Page_271` … `#Page_285`. Sidenotes: Lead Pencil (p. 274), Silver and Gold Point (p. 275), Charcoal (p. 275), Red Chalk (Sanguine) (p. 278), Black Conté and Carbon Pencil (p. 280), White chalk (p. 280), Lithography (p. 281), Pen and Ink (p. 282), Etching (p. 283), Paper (p. 284). Figures: `#plate55` (`#id-6206880884822723072`), `#plate56` (`#id-1292169841138301470`). Cross-references: "page 46 [Transcribers Note: Plate IV]" and "page 260 [Transcribers Note: Plate LIV]" (pp. 280–281).

For each medium, the *Implication* field gives what the medium contributes expressively (from the author), a **vector analogue** (ours), and what has **no honest analogue** (ours). "Simulation" means a raster or texture effect that imitates the look without the behaviour.

### SP-XX-1 The medium decides what you select; never ask more of it than it can do
- *Author says:* Materials are "of the greatest importance in determining what qualities in the infinite complexity of nature he selects for expression". "Every material he works with possesses certain vital qualities peculiar to itself", and the draughtsman must "be careful never to attempt more than it is capable of doing." (p. 271) Because oil paint can come so close to illusion, much effort has gone into "a meretricious attempt to deceive the eye" (p. 271).
- *Figures:* none.
- *Conditions / counter-examples:* with pen and ink the need to select is obvious. With oil it is "more often lost sight of".
- *Our interpretation:* vector has its own vital qualities: crisp edges, flat colour, exact repetition, scalable line, clean layering and transparency.
- *Implication for vector software:* agent guidance: design to vector's strengths (shape, edge, flat colour, line) rather than imitating photographic or painterly rendering with meshes and blurs, unless the brief asks. Presets that emulate other media should say plainly what they only simulate.
- *Do not algorithmize because:* none needed; this is a guiding principle.
- *Disposition candidates:* design-principle; agent-prompt-or-resource.
- *Science caveat:* none.

### SP-XX-2 Not illusion: painted symbols
- *Author says:* "Art is not a substitute for nature" (p. 272), but an expression of feeling in the artist's consciousness, closely tied to the material. It is expressed in symbols as true to nature as possible while staying in tune with the idea, but "never regarded by the fine artist as anything but painted symbols nevertheless" (p. 272). The artist never means you to forget it is a painted picture.
- *Figures:* none.
- *Conditions / counter-examples:* "however naturalistic the treatment his theme may demand".
- *Our interpretation:* stylisation is legitimate, and illusion is not the goal by default.
- *Implication for vector software:* an agent does not chase photorealistic vector by default. It states its stylisation choice.
- *Do not algorithmize because:* none needed.
- *Disposition candidates:* design-principle.
- *Science caveat:* aesthetic philosophy, stated as such.

### SP-XX-3 A point should only do what a point can
- *Author says:* With "more limited knowledge of the phenomena of vision", early masters had less chance to go astray. Now that every effect can be represented, one must decide how far complete accuracy serves the impression. "They did not attempt anything with a point that a point was not capable of expressing." (p. 273) Modern drawings attempt tone and colour effects, "things entirely outside the true province of drawing". Line work "went out of fashion in our schools".
- *Figures:* Plates LV and LVI are drawings that stay drawings, built from line plus hatching or wash.
- *Conditions / counter-examples:* none.
- *Our interpretation:* a "drawing" works by line, and should not fake painted tone.
- *Implication for vector software:* a line-only drawing mode or style constraint (strokes only, no fills, hatching allowed). An agent asked for a "drawing" expresses form through line and hatching, not gradient meshes.
- *Do not algorithmize because:* none needed; this is a mode choice.
- *Disposition candidates:* design-principle; agent-prompt-or-resource; practice-exercise (line-only).
- *Science caveat:* "more limited knowledge of the phenomena of vision" is the author's historical generalisation.

### SP-XX-4 Limitations give power; add means gradually
- *Author says:* "There is something that makes for power in the limitations your materials impose." (p. 273) Some artists who are fine in limited media are feeble in oil. Students should restrain themselves. Begin with monochrome in three tones ("as explained in a former chapter"), then for figure work use ivory black and Venetian red, which teaches much about warm and cold. "Keep the darks rather lighter and the lights darker than nature" at first. Take on the full tonal scale only with experience, and add colours gradually. A palette "full of the most amazing colours" gives poor results. "It is a wise man who can discover his limitations and select a medium the capacities of which just tally with his own." (p. 274) Try many media: descriptions convey little, and "they must be handled by the student".
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* a constraint ladder.
- *Implication for vector software:* a constrained-palette document mode. Restrict swatches to N colours, for example three values of one hue, or a black-and-red-earth ramp, plus a compressed value range. Out-of-palette colours get a soft warning and an option to recolour to the nearest swatch. Agents can be told to work inside such a constraint. A practice ladder follows the same order: 3 values → 2 pigments → wider range.
- *Do not algorithmize because:* none needed; these are constraints the user chooses.
- *Disposition candidates:* improve-existing (swatch groups / recolour artwork limited to N colours); engine-feature (palette constraint with warnings); practice-exercise; agent-prompt-or-resource.
- *Science caveat:* none (pedagogy).

### SP-XX-5 Lead pencil
- *Author says:* "One of the most beautiful for pure line work" and a training in precision of observation, "unrivalled for sketch-book use" (p. 274). It comes in degrees from hard and grey to soft and black, and "does not need fixing". "For pure line drawing nothing equals it, except silver point" (p. 274), and Ingres loved it. It is not suited to mass drawing, because of "the offensive shine that occurs if dark masses are introduced". "Its charm is the extreme delicacy of its grey-black lines." (p. 275)
- *Figures:* none of pencil.
- *Conditions / counter-examples:* only for very lightly shaded work.
- *Our interpretation:* delicate, precise line with a value range set by the grade.
- *Implication for vector software:* **contributes:** precise, delicate grey line, grade as a value and width range, unsuited to masses. **Vector analogue:** a thin stroke with slight pressure-driven width and opacity variation; a "grade" preset maps to stroke colour (grey to near-black) and base width; light hatching for tone. **No honest analogue:** graphite's angle-dependent shine, and its tooth interacting with paper grain (texture overlays only simulate it).
- *Do not algorithmize because:* not applicable.
- *Disposition candidates:* improve-existing (pencil preset built from width profiles and pressure); design-principle.
- *Science caveat:* none.

### SP-XX-6 Silver and gold point
- *Author says:* Drawn with a silver point on paper given "a faint wash of Chinese white"; "without this wash the point will not make a mark" (p. 275). "For extreme delicacy and purity of line no medium can surpass this method", and it is best for a beautiful line such as a profile. As training, "no rubbing out of any sort is possible, and eye and hand must work together with great exactness." (p. 275) It is a "corrective to the picturesque vagaries of charcoal work". Gold point gives "a warmer line".
- *Figures:* Plate LV, from a silverpoint drawing. A woman's head tilted up, three-quarter to viewer-left. Thin, even, grey lines. The hair is many long parallel sweeping strokes, and the face is modelled by very light, close hatching. Diagonal parallel hatching behind the shoulder gives a background tone, and the shoulders and body are a few loose sweeping lines. No erasure or smudged tone is visible.
- *Conditions / counter-examples:* needs a prepared ground.
- *Our interpretation:* fine line plus committed, unerasable marks.
- *Implication for vector software:* **contributes:** the purest, most delicate line; discipline through no erasure; a warm or cool line colour (gold or silver). **Vector analogue:** a hairline, nearly constant-width stroke in warm or cool grey on a tinted "prepared" background; tone by groups of parallel lines; a practice mode with undo disabled (append-only, see SP-XIX-7). **No honest analogue:** the physical metal-on-ground mark. (Silverpoint tarnishes over time; that is our knowledge, not the author's.)
- *Do not algorithmize because:* not applicable.
- *Disposition candidates:* improve-existing (fine-line preset); practice-exercise (no-undo).
- *Science caveat:* none.

### SP-XX-7 Charcoal and the rub-down / lift-out method
- *Author says:* Of the media that combine the two approaches to form (line and mass), charcoal is "the first and most popular". It comes in degrees. "A chisel-shaped point is the most convenient" (p. 275): the broad side gives dark masses while the edge stays sharp for fine lines. It works freely and forcefully and is "much more like painting than any other form of drawing", with "a wide piece of charcoal making a wide mark similar to a brush." (p. 276) Rubbed with a finger it gives a soft grey tone. A bread pellet (best, because it lifts without smudging) or rubber takes out lights. Method: draw the proportions roughly, rub down to a middle tone, then draw the lights with bread and the shadows with charcoal, as in the monochrome exercises. It is used for setting out on canvas and fixed with a spray diffuser. Sticks nearly an inch thick allow "a long swinging line". As an end in itself it is "too much like painting to have the particular beauties of a drawing, and too much like drawing to have the qualities of a painting" (p. 277). Fix lightly part-way so you can work over it. Rubbing out fixed parts or scraping with a pen-knife is only for academic or working drawings. It can be finished with Conté. Artificial charcoal is more even; the French "Fusain Rouget" comes in three degrees; "a bad piece of charcoal is worse than useless". The fixative is white shellac in spirits of wine. It is "not on the whole the medium an artist with a pure love of form selects, but rather that of the painter" (p. 278).
- *Figures:* none of charcoal here.
- *Conditions / counter-examples:* less refined for finished drawings. Scraping and heavy rubbing spoil freshness.
- *Our interpretation:* a brush-like, two-way (line and mass) tool with a subtractive light method.
- *Implication for vector software:* **contributes:** fast massing; brush-like width from the broad side; subtractive lights on a rubbed middle tone; long swinging lines. **Vector analogue:** a calligraphic brush whose width depends on stroke direction (chisel: broad dragged sideways, thin on edge; a calligraphic brush exists in `crates/brush/src/calli.rs`). "Rubbing down" is lowering the opacity of the block-in layer or laying a mid-tone shape beneath it. "Lifting lights" is light-coloured or knockout shapes, or opacity masks, over the mid-tone ground. "Fixing" between stages is locking the layer. **No honest analogue:** smudge physics, charcoal dust and the grain of the stick. Blur and grain effects are simulation.
- *Do not algorithmize because:* not applicable.
- *Disposition candidates:* improve-existing (calligraphic/chisel brush presets; "mid-tone ground" document setup); multi-step-workflow (rub-down / lift-out, see the procedures below); design-principle.
- *Science caveat:* none.

### SP-XX-8 Red chalk (sanguine)
- *Author says:* "A delightful medium" for pure line or a mixed method, a natural red earth in Venetian red. Natural pieces vary, from hard and gritty to soft and smooth. Conté's artificial sticks are never gritty but are softer, wear quickly and make fine lines less well. Rubbed with a finger or rag it spreads evenly into a middle tone. Lights are drawn with hard pointed rubber cut to a chisel, or with bread for softer lights. Because you can keep correcting by rubbing, it is recommended for picture studies "where much fumbling may be necessary". "Unlike charcoal, it hardly needs fixing, and much more intimate study of the forms can be got into it." (p. 278) "Most of the drawings by the author reproduced in this book are done in this medium." The danger: "your drawing looks so much at its best that you are apt to be satisfied too soon." (p. 279) "But for portrait drawings there is no medium to equal it." Powdered chalk with water and a little gum arabic, applied by sable brush, "makes a rich velvety dark". Choose paper with little size: "the smoothest paper without a faced and hot-pressed surface" (O.W. etching paper). Black chalk is sometimes added for strength, or for "almost a full colour effect" (p. 279).
- *Figures:* Plates LI and LII, the author's own red chalk, "rubbed, the high lights being picked out with rubber". The heads are modelled in soft rubbed tone with lights lifted (forehead and crown in LI, cheek and forehead in LII), and crisp darker accents at the eyes, nostrils and moustache or beard. The coats are a few strong contours and areas of parallel diagonal hatching.
- *Conditions / counter-examples:* the "satisfied too soon" danger for students.
- *Our interpretation:* a single warm hue, a rubbed middle tone, lifted lights and corrections that are easy to make.
- *Implication for vector software:* **contributes:** a warm single-hue range; intimate modelling; easy correction; a velvety dark accent. **Vector analogue:** a one-hue ramp (pale tint to deep red-brown) on warm paper; rubbed tone as feathered fills or low-opacity soft-edged shapes; lifted lights as paper-coloured feathered shapes; the velvety dark as a deeper, more saturated accent; black as a second ink. **No honest analogue:** grit, tooth, and the physical spread of rubbed pigment. **Agent caution:** an attractive rendering is not proof that the drawing is right, so check proportions first.
- *Do not algorithmize because:* not applicable.
- *Disposition candidates:* improve-existing (two-ink sanguine palette preset); agent-prompt-or-resource ("pretty is not correct").
- *Science caveat:* none.

### SP-XX-9 Holbein's layered method: rub down, then redraw with finality
- *Author says:* Holbein tinted the paper in most of his portrait drawings, sometimes with "zinc white as a wash", which let him add silverpoint lines and avoid trouble with the paper's size. "His aim seems to have been to select the few essential things in a head and draw them with great finality and exactness." (pp. 279–280) Earlier work in red or black chalk was often rubbed down and redrawn, with a brush and chalk mixed with water and gum, or with a pure silverpoint line. In other drawings the paper was tinted with watercolour and rubbed back to white for the lights, or Chinese white was used.
- *Figures:* Plates L and LIII (Holbein): tinted grounds, heads finished, the rest sparse outline.
- *Conditions / counter-examples:* none.
- *Our interpretation:* explore loosely, then state the essentials decisively on top. Layers make this honest in vector.
- *Implication for vector software:* **vector analogue (direct):** an exploration layer → dim it (lower opacity) and lock it → a final layer of a few decisive, clean paths on top; a tinted background; lights as paper-white shapes. This is a workflow an agent can follow exactly.
- *Do not algorithmize because:* choosing "the few essential things" is the judgement.
- *Disposition candidates:* multi-step-workflow; agent-prompt-or-resource.
- *Science caveat:* the author attributes "zinc white" to Holbein (16th century). Zinc white is generally dated to the late 18th and 19th centuries, so this may be anachronistic or loose naming. Verify before reusing.

### SP-XX-10 Black Conté and carbon pencil: supplements that don't read as another medium
- *Author says:* Black Conté is a hard black chalk in sticks of different degrees and in cedar pencils, rather more gritty than red chalk or charcoal. It supplements charcoal "when more precision and definition are wanted". "It has very much the same quality of line and so does not show as a different medium." (p. 280) It rubs and spreads tone like charcoal. "Carbon pencils are similar to Conté, but smoother in working and do not rub."
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* a precision tool should share the visual voice of the main tool.
- *Implication for vector software:* precision passes (for example a fine pen for accents) should share colour, cap, end and texture with the main brush so they don't read as a different medium. A shared graphic style or brush family achieves this.
- *Do not algorithmize because:* not applicable.
- *Disposition candidates:* design-principle; improve-existing (graphic styles / brush families shared across tools).
- *Science caveat:* none.

### SP-XX-11 White chalk on toned paper: keep the paper's half tone between lights and darks
- *Author says:* White chalk on toned paper draws the lights, with "the paper serving as a half tone while the shadows and outlines are drawn in black or red". The white "should never be allowed to come in contact with the black or red chalk of the shadows"; "the half tone of the paper should always be between them." (p. 280) For rubbed work, "white pastel is better than the ordinary white chalk". Examples: Plate IV (white pastel with red chalk) and Plate LIV (hard white chalk). This is the usual method for drapery studies, because of "the extreme rapidity with which the position of the lights and shadows can be expressed" (p. 281) in so unstable a subject.
- *Figures:* Plate LIV, black and white Conté on brown paper. A figure in windblown drapery. The folds' lights are white chalk hatching and the darks and contours are black lines. Much bare paper is left as the middle tone between them. Faint straight construction lines show at left, and secondary figures are only outlined in black at lower right. Plate IV (outside the range, viewed only for this cross-reference) is captioned "Study on tissue-paper in red chalk for figure of Boreas": a red-chalk figure over a squared grid with diagonal construction lines. **I could not see distinct white pastel in that reproduction** (see Open questions).
- *Conditions / counter-examples:* none.
- *Our interpretation:* a three-value system with a measurable spatial rule.
- *Implication for vector software:* **contributes:** speed; three clear values (paper, light, dark); legibility through the untouched middle. **Vector analogue (honest):** a "toned ground" setup with a mid-tone background, a light-stroke group and a dark-stroke group. A geometric check can flag where shapes in the light group touch or overlap shapes in the dark group (minimum distance below a threshold), and that is honestly measurable. Limits: it knows only group membership (user tags or colour thresholds), not intent, and an artist may choose to break the rule. **No honest analogue:** the paper's tooth catching the chalk.
- *Do not algorithmize because:* the check is mechanical; whether to obey it is the artist's choice.
- *Disposition candidates:* analysis-overlay (light/dark contact check); engine-feature (toned-ground document preset); practice-exercise (drapery study on a toned ground); multi-step-workflow.
- *Science caveat:* none.

### SP-XX-12 Lithography
- *Author says:* Lithography has suffered from trade uses, but it is one of the most wonderful ways of reproducing an artist's work. "It is the prints that are really the originals" (p. 281). One draws on a smooth limestone with greasy chalk in several degrees. "No rubbing out is possible, but lines can be scratched out with a knife", or white lines drawn over tone with a knife. The chalk can be rubbed up with water and brushed on, and the side of the chalk gives every variety of tone. Don't let "the warm finger touch the stone", or it may print. The stone is treated with gum arabic and a little nitric acid, washed, inked with a roller (boiled linseed varnish and lithographic colours) and printed. Drawings on transfer paper reproduce line nearly identically, but tone is better drawn on the stone. "No more perfect means of reproduction could be devised" (p. 282).
- *Figures:* none.
- *Conditions / counter-examples:* line transfers well from paper; tone does not.
- *Our interpretation:* the print, the output, is the work.
- *Implication for vector software:* **contributes:** faithful multiples; scratched-out whites; grainy crayon tone. **Vector analogue:** a crayon-grain texture brush (simulation); scratched whites as light strokes over dark masses. The general lesson: design for the final output (print, screen, export), since the export is the original. **No honest analogue:** the stone chemistry, the grease accidents, the press. (The chemical steps are historical description and must not be relayed as safety instructions.)
- *Do not algorithmize because:* not applicable.
- *Disposition candidates:* design-principle (design for the output); improve-existing (optional crayon-grain brush); not-automatable (the process).
- *Science caveat:* none.

### SP-XX-13 Pen and ink, and the character of each pen
- *Author says:* A favourite of old masters, Rembrandt above all, who, "often heightening the effect with a wash", "conveyed marvellous suggestions with the simplest scribbles" (p. 282). It is difficult for students but trains "quick definite statement". "No hugging of half tones is possible, things must be reduced to a statement of clear darks" (p. 283), which corrects the habit of seeing mostly half tones. Steel pens range from fine crow-quills to thick "J" nibs. "The natural crow-quill is a much more sympathetic tool than a steel pen, although not quite so certain in its line", with more play and variety, and is better for free drawing. Reed pens give thick lines. The glass pen gives "a line of only one thickness … but this can be drawn in any direction" (p. 283).
- *Figures:* Plate LVI, Rubens's pen-and-wash study for the tree in "The Boar Hunt". Gnarled trunks, one leaning diagonally from lower centre to the right, and a root or stump mass at lower left-centre. Thin, nervous pen lines for twigs and leaves, heavier lines on the trunk contours, wash for dark foliage masses and the shadowed trunk, and open sky left blank.
- *Conditions / counter-examples:* difficult for beginners.
- *Our interpretation:* pens differ mainly in how their width responds to pressure and direction, which maps directly onto width profiles.
- *Implication for vector software:* **contributes:** clear darks without half tones; quick, definite statement; line character by tool. **Vector analogue (honest):** glass pen = constant-width stroke; crow-quill or steel nib = pressure- or speed-driven variable-width profile; reed = broad stroke, possibly with a calligraphic angle; wash = semi-transparent flat fills behind the line work; "no half tones" = a two-value (ink and paper) mode. **No honest analogue:** ink pooling, nib catches and blots (roughening or texture only simulates them).
- *Do not algorithmize because:* not applicable.
- *Disposition candidates:* improve-existing (pen presets from stroke width profiles); practice-exercise (two-value ink mode); agent-prompt-or-resource.
- *Science caveat:* none.

### SP-XX-14 Etching
- *Author says:* One draws with a steel point on a waxed copper or zinc plate and bites the lines in dilute nitric acid. The longer the bite, the deeper and darker the line, and "variety in thickness is got by stopping out with a varnish the light lines" (p. 283) while the darker lines are bitten longer. In printing, the plate is inked and wiped so only the lines hold ink. Ink left in places gives a tint, and ink smudged out of the lines softens them. "There are no end of tricks a clever etching printer will adopt" (p. 284).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* line weight assigned in a few discrete tiers ("bites"), plus a plate tone.
- *Implication for vector software:* **contributes:** a disciplined hierarchy of line weights; a plate-tone tint; a softened line. **Vector analogue (honest):** assign lines to 2–4 weight tiers (graphic styles or named stroke weights) and set the width per tier; plate tone as a light tint fill; softening as a slight blur or glow (simulation). **No honest analogue:** the variability of wiping and printing.
- *Do not algorithmize because:* not applicable.
- *Disposition candidates:* improve-existing (stroke-weight tiers via styles); design-principle (line-weight hierarchy); not-automatable (printing variability).
- *Science caveat:* none.

### SP-XX-15 Paper and surface
- *Author says:* "The texture of your paper will have a considerable influence on your drawing." (p. 284) Try every sort: "a new paper is often a stimulant to some new quality in drawing". "Avoid the wood-pulp papers, as they turn dark after a time"; linen rag (O.W. papers) is safe. Where sympathetic rather than hard qualities are wanted, pin a wad of several sheets under the working sheet for "a more sympathetic surface". To redraw an unsatisfactory study, pin a thin paper over it so it shows through, and so "start as it were from the point where one left off" (p. 285): "I fancy they are called 'bank-note' papers."
- *Figures:* Plate IV's caption ("on tissue-paper") illustrates the thin-paper practice.
- *Conditions / counter-examples:* the soft wad is for when a clear, hard line is not wanted.
- *Our interpretation:* tracing over your own previous study is a sound way to iterate.
- *Implication for vector software:* **vector analogue (direct):** an "onion-skin redraw": duplicate the layer or artboard, dim and lock the old one, and draw fresh on top. This is the honest form of the bank-note-paper method and works well for agents iterating on their own work. A paper colour or texture background layer (printing or not). **No honest analogue:** tooth affecting each mark; the soft wad (closest are tablet pressure-curve settings); archival darkening.
- *Do not algorithmize because:* not applicable.
- *Disposition candidates:* improve-existing (onion-skin redraw command: duplicate, dim, lock, new layer); multi-step-workflow; design-principle.
- *Science caveat:* the archival advice (groundwood paper darkens, rag paper lasts) is broadly sound but outside our domain.

### Procedures from XX (the chapter gives several methods)
**Charcoal rub-down / lift-out (p. 276):** 1. Draw the general proportions roughly. 2. Rub down with the finger to an even middle tone, with the rough drawing showing through. 3. Draw the lights with bread or rubber. 4. Draw the shadows with charcoal, "in much the same manner as … the monochrome exercises". 5. For more finish, fix lightly once carried some way, then work over it (p. 277). 6. Optionally finish with Conté [SP-XX-7, SP-XX-10]. *Vector form (ours):* block-in layer → mid-tone shape or opacity → light shapes or masks → dark strokes → lock → refine.

**Toned paper, white and black/red (p. 280):** draw lights in white and shadows and outlines in black or red, with the paper's half tone always between them, never touching [SP-XX-11].

**Holbein's rub-and-redraw (pp. 279–280):** early chalk work → rubbed down → redrawn with brush and gum-chalk or a silverpoint line, selecting a few essentials with finality [SP-XX-9].

**Restrained colour ladder (p. 273):** monochrome in three tones → ivory black + Venetian red → keep darks lighter and lights darker than nature → full tonal scale only with experience → add colours one at a time [SP-XX-4].

**Thin-paper redraw (pp. 284–285):** pin thin paper over the first study and continue from where you left off [SP-XX-15].

### Materials summary: contribution, vector analogue, what has no honest analogue
| Medium | Expressive contribution (author) | Vector analogue (ours) | No honest analogue (ours) |
|---|---|---|---|
| Lead pencil | precise, delicate grey-black line; precision training; poor for masses (shine) | thin stroke with slight pressure-driven width and opacity; grade → colour and width preset | graphite shine; tooth/grain |
| Silver / gold point | purest line; no erasing → exactness; gold warmer | hairline constant-width stroke; tinted ground; no-undo practice mode | metal-on-ground mark (tarnish: ours) |
| Charcoal | line and mass; brush-like broad side; rub to middle tone; lift lights | direction-dependent calligraphic brush; mid-tone layer; light shapes or masks over it; lock to "fix" | smudge physics, dust, stick grain |
| Red chalk | warm single hue; intimate modelling; easy correction; velvety dark | one-hue ramp on warm paper; feathered fills; paper-coloured lights; deep accent | grit, rubbed pigment spread |
| Black Conté / carbon | precision supplement that doesn't read as a new medium | shared style/brush family for precision passes | grit |
| White chalk on toned paper | speed; three values; untouched middle tone | toned-ground preset; light/dark contact check | paper tooth |
| Lithography | faithful multiples; scratched whites; crayon tone | crayon-grain brush (simulation); light strokes over dark; design for output | stone chemistry, grease, press |
| Pen and ink | clear darks, no half tones; quick statement; line character per pen | width profiles: constant (glass), pressure-variable (quill/steel), broad (reed); wash = translucent fill; two-value mode | ink pooling, nib catches, blots |
| Etching | line-weight hierarchy by bite; plate tone; softened lines | stroke-weight tiers; tint fill; slight blur (simulation) | wiping and printing variability |
| Paper | texture shapes the drawing; a new paper stimulates; thin paper for redrawing | background paper colour/texture; onion-skin redraw | tooth per mark; soft wad (closest: pressure curve) |

---

## XXI Conclusion

**Locator:** `…14264-h-3.htm.xhtml`. Anchors: `#CHAPTER_XXI`, `#Page_286`, `#Page_287`, `#Page_288`. No figures.

### SP-XXI-1 There is no mechanical standard of accuracy; the camera is only for those who don't need it
- *Author says:* The book has spoken out strongly against "a mechanical standard of accuracy in artistic drawing" (p. 286). If such a standard existed, the camera would serve. The camera may help only those "thoroughly competent to do without it", "to those who can look, as it were, through the photograph" and draw from it with the freedom they would bring to nature, which is very hard. "The camera is a convenience to be avoided by the student." (p. 286)
- *Figures:* none.
- *Conditions / counter-examples:* allowed for the competent, with that freedom.
- *Our interpretation:* auto-tracing is the modern camera.
- *Implication for vector software:* image trace and photo-to-vector features are tools for the competent. Their output is presented as a starting point. Agents never present auto-traced photos as "drawings" and say when output was traced. Practice and learning modes discourage tracing.
- *Do not algorithmize because:* none needed; this is policy and guidance.
- *Disposition candidates:* agent-prompt-or-resource; design-principle; improve-existing (image-trace documentation and notes).
- *Science caveat:* none.

### SP-XXI-2 Precision first; a master's variations are seldom conscious
- *Author says:* He would be "very sorry" if students took the book to advocate "a loose and careless manner of study" (pp. 286–287). The student must aim for years at "the most painstaking accuracy of observation and record". "The variations on mechanical accuracy in the work of a fine draughtsman need not be, and seldom are, conscious variations." (p. 287) "Mechanical accuracy is a much easier thing to accomplish than accuracy to the subtle perceptions of the artist", and one who "cannot draw with great precision the ordinary cold aspect of things cannot hope to catch the fleeting aspect of his finer vision". Training should develop "to the uttermost the observation of form and all that it signifies", and the power to portray it accurately (p. 287).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* vitality is not noise. Deliberate or random distortion is not what the book means by the life in drawing.
- *Implication for vector software:* **critical for agents:** do not add random jitter, wobble or "hand-drawn" roughening to fake vitality. Vitality comes from accurate perception of the particular subject, and variation should follow the subject (SP-XVII-12, SP-XVI-3). Roughen and jitter effects are style choices, labelled as such.
- *Do not algorithmize because:* the variations he praises arise from perception, not from rules.
- *Disposition candidates:* agent-prompt-or-resource; design-principle.
- *Science caveat:* none.

### SP-XXI-3 A mannerism remote from nature is a trick of the hand
- *Author says:* Artists who can only draw "in some weird fashion remote from nature" may make work of some interest, but they are "too much at the mercy of a natural trick of hand" to be more than "interesting curiosities in art" (p. 287).
- *Figures:* none.
- *Conditions / counter-examples:* he allows "some interest".
- *Our interpretation:* a fixed house style applied to everything is the machine version of the trick of hand.
- *Implication for vector software:* agents should not default to one style regardless of subject. Style follows the subject and the brief.
- *Do not algorithmize because:* none needed; this is guidance.
- *Disposition candidates:* design-principle; agent-prompt-or-resource.
- *Science caveat:* a value judgement on non-naturalistic art. Taste has changed since; treat it as one view.

### SP-XXI-4 Sincerity over originality
- *Author says:* "Unflinching honesty must be observed in all your studies." Only then will "the 'you' in you" find expression, and that personal quality is "the very essence of distinction in art" (p. 287). "The 'seeking after originality' so much advocated would be better put 'seeking for sincerity.'" (p. 287) Seeking originality usually becomes running after any peculiarity of manner thrown up by fashion. One of the most original men, Shakespeare (unnamed), took hackneyed plots: "What custom wills in all things do you do it." (p. 288)
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* novelty-seeking without content is insincere.
- *Implication for vector software:* agents should not chase novelty effects for their own sake. The product should not present "originality" as something it generates, because sincerity is the user's.
- *Do not algorithmize because:* sincerity is personal.
- *Disposition candidates:* design-principle; not-automatable.
- *Science caveat:* none.

### SP-XXI-5 Style is a clear idea of what you want, reached by the shortest means
- *Author says:* Individual style comes naturally as you become more conscious of what you want to express. There are two kinds of insincerity in style: a ready-made conventional manner "that is not understood and that does not fit the matter", and laboriously seeking an original manner "when no original matter exists". "Good style depends on a clear idea of what it is you wish to do" (p. 288), "the shortest means to the end aimed at". "'The style is the man,' as Flaubert says." Its value depends on the quality of the mental vision, and "this is not a matter where direct teaching can help you" (p. 288).
- *Figures:* none.
- *Conditions / counter-examples:* none.
- *Our interpretation:* it echoes SP-XIX-1 (know what you want) and SP-XIX-9 (economy).
- *Implication for vector software:* agents state the intent, then choose the most economical means. Style presets are chosen to fit the matter, never applied by default.
- *Do not algorithmize because:* the author says it cannot be taught.
- *Disposition candidates:* agent-prompt-or-resource; design-principle; not-automatable.
- *Science caveat:* not science, but an attribution flag: "The style is the man" is usually attributed to Buffon (1753), not Flaubert. Verify before quoting it in product text.

---

## A Appendix (the Phi proportion)

**Locator:** `…14264-h-3.htm.xhtml`. Anchors: `#APPENDIX`, `#Page_289`, `#Page_290`, `#Page_291`. Figure: `#diagram29` (`#id-2951252688186211276`), "The Phi Proportion", not listed in the book's List of Diagrams. Cross-referenced from SP-XVI-8 ("Appendix, page 289"). The pictures it tests are Plates IX and XXXIII–XXXVIII, which are **outside this range** and were not viewed or measured here.

### SP-A-1 The additive series converges on one ratio, Phi
- *Author says:* Add a 5-inch line to an 8-inch line and you get 13; keep adding the last two to get 5, 8, 13, 21, 34, 55 inches, &c. Mr William Schooling told him that adjacent lines are "practically in the same proportion": 8 is 1.600 times 5, 13 is 1.625 times 8, 21 is 1.615 times 13. Schooling worked out the exact proportion for a series in which each pair of neighbours has the same ratio and any two add to the next. "There is only one proportion that will do this" (p. 289). "For practical purposes" it is the ratio between 5 and "a fraction over 8", or exactly (1 + √5)/2 = 1.61803398875 "(nearly)". Schooling called it "the Phi proportion".
- *Figures:* Diagram XXIX (SP-A-2).
- *Conditions / counter-examples:* "practically" and "for practical purposes".
- *Our interpretation:* the mathematics is correct. The ratios of consecutive terms of any additive (Fibonacci-type) series tend to φ, and φ is the only ratio for which a geometric series is also additive. The inches are arbitrary, since the ratio has no units.
- *Implication for vector software:* φ can be offered as one optional ratio constant (divisions at 0.382 and 0.618 of a span) in guide presets. The series itself is not needed.
- *Do not algorithmize because:* the maths is safe to encode; the aesthetic claim is not (SP-A-4).
- *Disposition candidates:* improve-existing (guide/ratio presets); design-principle.
- *Science caveat:* the mathematics, none. The aesthetics: see SP-A-4.

### SP-A-2 Diagram XXIX: the construction of the Phi segments
- *Author says:* the caption table, transcribed cell by cell from the XHTML: row 1 "EC is 1.618033, &c., times size of" | "AB,"; row 2 "CD" | "BC,"; row 3 "DE" | "CD, &c.,"; row 4 "AC=CD"; row 5 "BD=DE, &c." The image's alt text shows ditto marks: `CD " " " " BC, DE " " " " CD`. Page 289.
- *Figures:* Diagram XXIX is a horizontal line with five ticks, A to E, spaced ever wider from left to right. **Measured** on the large JPEG (482×70 px): A≈18, B≈66, C≈141, D≈264, E≈463.5 px. So AB≈48, BC≈75, CD≈123, DE≈199.5. Ratios: BC/AB≈1.56, CD/BC≈1.64, DE/CD≈1.62. AC≈123≈CD, and BD≈198≈DE.
- *Conditions / counter-examples:* the drawn figure is approximate (BC/AB is 1.56, not 1.618).
- *Our interpretation:* "EC" must be a misprint or transcription error for **"BC"**, because EC = CD + DE ≈ φ⁴·AB, not φ·AB. The intended statement is BC = φ·AB, CD = φ·BC, DE = φ·CD, &c., together with the additive identities AC = CD and BD = DE.
- *Implication for vector software:* a "φ ruler" guide that, from one chosen segment, generates successive ticks at φ multiples, or divides a span at φ.
- *Do not algorithmize because:* nothing here needs judgement; it is geometry.
- *Disposition candidates:* improve-existing (guides); practice-exercise (constructing a φ division by hand).
- *Science caveat:* none (geometry).

### SP-A-3 Phi applied to placing the main points in seven pictures
- *Author says:* "Testing this proportion on the reproductions of pictures in this book in the order of their appearing" (p. 290), he lists points on seven pictures that lie "exactly Phi proportion" from the edges (see **Appendix data**). These points are usually the "most important point" or "most important horizontal line" of the composition. He chose "the first seven pictures reproduced in this book that were not selected with any idea of illustrating this point". "In each some very important quantity has been placed in this proportion." (p. 291) He could go on, including the minor relationships, "were it not for the fear of becoming wearisome".
- *Figures:* none in this range (Plates IX, XXXIII–XXXVIII are elsewhere).
- *Conditions / counter-examples:* the Europa head is "slightly to left of Phi … unless in the reproduction a part of the picture on the left has been trimmed away, as is likely". For Venus's shell, "the smaller quantity being below this time".
- *Our interpretation:* his use of φ is for placing the principal element, not for scoring a whole composition.
- *Implication for vector software:* optional reference data: "φ lines" at 0.382 and 0.618 of the width and height, offered beside thirds and other divisions as one placement option for the principal element. Never an automatic score, and never "your composition is 73% golden". Limits: see SP-A-4.
- *Do not algorithmize because:* the test is post hoc, and which point "counts" is chosen after looking.
- *Disposition candidates:* improve-existing (guide preset); agent-prompt-or-resource (one option among several, carrying its caveat).
- *Science caveat:* strong; see SP-A-4.

### SP-A-4 "The eye evidently takes some especial pleasure in it": his caveats, and ours
- *Author says:* Having listed the seven pictures, he concludes that "the eye evidently takes some especial pleasure in it, whatever may eventually be found to be the physiological reason underlying it." (p. 291)
- *Figures:* none.
- *Conditions / counter-examples (the author's own hedges, gathered):* 5:8 is "roughly stated" (p. 237). "For practical purposes" (p. 289). The cropped-reproduction hedge for Europa (p. 291). The physiological reason is unknown (p. 291). Measurements were taken on the book's halftone reproductions, not the paintings (p. 290). No tolerance is given, and in most cases he does not say which segment is the smaller. In XVI he says canons "have not been a great success" (p. 227) and that "there is something chilling to the artist in an array of dead figures" (p. 228). The proportional compass is "said to contain the secret" (p. 238). The Appendix's confident "exactly" sits uneasily with these.
- *Our interpretation:* this is a period enthusiasm, presented with honest hedges, not a demonstration.
- *Implication for vector software:* every φ feature ships with a neutral label ("a historical proportion guide; see Speed 1913, Appendix") and no claim of pleasingness. Agents never tell users that φ makes art more pleasing.
- *Do not algorithmize because:* the underlying claim is unproven.
- *Disposition candidates:* design-principle (never present as science); agent-prompt-or-resource (caveat text).
- *Science caveat:* **strong.** (1) Post-hoc search with many candidate features and an unstated tolerance will find φ by chance. Our illustrative arithmetic, with an assumed tolerance of ±2.5% of the dimension: the two φ bands (around 0.382 and 0.618) cover 10% of each axis, so a random feature lands "on φ" on a given axis about 10% of the time. With ten salient features, at least one will hit about 65% of the time (1 − 0.9¹⁰). (2) Experimental work since Fechner (1876) has found a preference for golden rectangles weak and inconsistent. Later reviews (for example C. D. Green, 1995, *Perception*) judge the effect fragile and dependent on method. Verify these references before citing them. (3) That "the eye evidently takes some especial pleasure in it" is not measured anywhere in the book.

---

## Appendix data

Encode this as **optional, historical reference data**, never as a scoring target.

**A. The series and the ratio (p. 289).**
| Quantity | Value (author) | Note |
|---|---|---|
| Series (inches) | 5, 8, 13, 21, 34, 55, &c. | each term = sum of the previous two; units are arbitrary |
| 8 / 5 | 1.600 | author |
| 13 / 8 | 1.625 | author |
| 21 / 13 | 1.615 | author |
| 34 / 21, 55 / 34 | (1.619, 1.618) | **ours**, not in the book |
| Phi (exact) | (1 + √5)/2 = 1.61803398875 "(nearly)" | the author's 11 decimals |
| "For practical purposes" | 5 : "a fraction over 8" | the author's wording |
| Division points of a span (derived, **ours**) | 1/φ ≈ 0.618034 and 1/φ² ≈ 0.381966 | the "Phi proportion with the two sides" means a point at one of these fractions |
| Diagram XXIX relations | BC = φ·AB, CD = φ·BC, DE = φ·CD; AC = CD; BD = DE | caption prints "EC" for "BC" (our correction) |
| Ch. XVI band (p. 237) | smaller : larger ≈ 5 : 8. Below that → toward 4 : 8 (smaller = 1/3 of total, "dull"); above → toward equality | qualitative band, no tolerance |

**B. The seven pictures (pp. 290–291).** "Sides" = a horizontal position measured between the left and right edges. "Top and bottom" = a vertical position measured between the top and bottom edges. Printed page numbers are the author's; plate numbers come from the transcriber's notes in the EPUB.
| # | Picture (author's page; transcriber's plate) | Element measured | Axis | Author's wording | Smaller segment | Author's notes and caveats |
|---|---|---|---|---|---|---|
| 1a | "Las Meninas", Velázquez (p. 60; Plate IX) | right-hand side of the light opening of the door at the end of the room | horizontal (sides) | "exactly Phi proportion" | not stated | "a very important point in the 'placing' of the composition" |
| 1b | same | bottom of that opening | vertical (top and bottom of canvas) | "exactly" | not stated | — |
| 2a | "Fête Champêtre", Giorgione (p. 151; Plate XXXIII) | lower end of the flute held by the seated female figure | horizontal | "exactly" | not stated | — |
| 2b | same | lower side of the hand holding the flute ("a point slightly above the end of flute") | vertical | "exactly" | not stated | "an important centre in the construction" |
| 3 | "Bacchus and Ariadne", Titian (p. 154; Plate XXXIV) | shadow under the chin of Bacchus | both axes | "the proportion … comes in" (no "exactly") | not stated | "the most important point in the composition being the placing of this head" |
| 4a | "Love and Death", Watts (p. 158; Plate XXXV) | point from which the drapery radiates on the figure of Death | vertical | "exactly" | not stated | — |
| 4b | same | point where the right-hand side of Love's right leg cuts the dark edge of the steps | horizontal | "exactly" | not stated | — |
| 5a | "Surrender of Breda", Velázquez (p. 161; Plate XXXVI) | first spear in the upright row, top right | horizontal | "exactly" | not stated | — |
| 5b | same | height of the gun carried horizontally by a man in the middle distance above the central group | vertical | "exactly" | not stated | "gives height of group of figures on left, and is the most important horizontal line in the picture" |
| 6a | "Birth of Venus", Botticelli (p. 166; Plate XXXVII) | horizon line | vertical | "Phi proportion" | implied above (**our inference** from 6b's "this time") | — |
| 6b | same | height of the shell Venus stands on | vertical | "Phi proportion" | **below** ("the smaller quantity being below this time") | — |
| 6c | same | extreme edge of the dark drapery held by the figure on the right blowing toward Venus | horizontal | "Phi proportion" | not stated | — |
| 7a | "The Rape of Europa", Veronese (p. 168; Plate XXXVIII) | top of Europa's head | vertical | "exactly" | not stated | — |
| 7b | same | right-hand side of the same head | horizontal | "slightly to left of Phi" | not stated | "unless in the reproduction a part of the picture on the left has been trimmed away, as is likely"; then "it would be exactly Phi proportion" |

**Caveats to ship with the data (the author's first, then ours):** measured on reproductions, possibly cropped (author). No tolerance and no measuring method given, and in most cases no statement of which segment is smaller (author's omission). Selection: "the first seven … not selected with any idea of illustrating this point" (author), but the feature in each picture was chosen after looking (ours). No mechanism: "whatever may eventually be found to be the physiological reason" (author). The modern evidence is weak (ours; SP-A-4).

**Suggested encoding (ours):**
```json
{ "id": "speed1913.phi", "ratio": 1.61803398875,
  "divisions": [0.381966, 0.618034], "axes": ["x", "y"],
  "source": "Speed, The Practice and Science of Drawing (1913), Appendix pp. 289-291; ch. XVI p. 237",
  "status": "historical artistic opinion; not verified perception science",
  "default": { "visible": false, "snap": false } }
```

---

## Cross-cutting notes

**Recurring themes across XV–XXI.**
- *Variety against unity, and life against repose.* This runs through every chapter: balance (XV-1, XV-8), proportion (XVI-2, XVI-3, XVI-9), portrait (XVII-10, XVII-14), Diagram XXVIII. Equality and perfect balance mean deadness; unchecked variety means riot.
- *Hierarchy and subordination.* A few dominant quantities (XVI-9), larger over petty individualities (XVII-7), detail under the initial impulse (XIX-6), a few essentials drawn with finality (XX-9).
- *The first impression governs.* The "flash" (XVIII-3), the first sketch (XVIII-7), the first strokes (XIX-3), the scheme fixed early (XVIII-8), the fresh eye (XVII-4).
- *Selection over transcription.* Medium-specific selection (XX-1), the camera rejected (XXI-1), the "catalogue of tired objects" (XVIII-2), the clerk copying a letter (XIX-1).
- *Precision as a precondition, not the goal.* XIX-8, XXI-2.
- *Sincerity.* XVII-9, XXI-4, XXI-5.
- *Simplify to recover.* Flat tones (XV-4), masses first (XVII-6), the last right stage (XIX-4).

**Terminology (the author's words, as used in the range).**
- "dither": the life or vitality that no science accounts for (p. 227).
- "vital" versus "academic" drawing (p. 248; cross-ref. to earlier chapters).
- "point drawing": drawing with a point (pencil, chalk), contrasted with painting (p. 242).
- "mass drawing" and "line drawing": the "two extreme points of view" (p. 275, cross-ref.).
- "half tone" and "middle tone"; "flat" versus "gradated" tones; "rich curves" (curves far from straight).
- "square lines" (straight-segment blocking, p. 242).
- "static balance" (Plate XLVIII caption).
- "pulling it together" (final vitalising pass, p. 249); "mucked up" and "out of gear" (p. 221).
- "matter and manner" (p. 263); "the camera idea of truth" (p. 266); "chord" (likeness, p. 241).
- "Phi proportion" (Schooling's name, p. 289).

**Every warning in the range against mechanical formula or mechanical accuracy.**
1. Perfect balance is "dull and dead" (p. 219).
2. "Like all so-called laws in art, this rule is capable of many apparent exceptions"; the two-thirds light rule is rejected (p. 222).
3. Canons "have not been a great success; the beautiful will always defy such clumsy analysis" (p. 227).
4. "No science will help you here" (the "dither", p. 227).
5. "Something chilling … in an array of dead figures"; "I can only give my own feeling" (p. 228).
6. Equality is the machine's, variety is nature's (pp. 229–230).
7. Raphael's "extremely obvious artifice" (p. 235).
8. Individuality defies "the analysis of science"; retouching kills vitality (p. 239).
9. Likeness at the "opposite pole" to the type (p. 240).
10. "Impossible to lay down hard and fast rules" (p. 242).
11. "Impossible to teach" (p. 245).
12. Holbein is honest to the vital impression, not "a cold, mechanically accurate record" (p. 248).
13. A fixed momentary expression is "terrible, and most unlifelike" (p. 252).
14. "A catalogue of tired objects" (p. 259); "a feeble artist, who copies individually the parts of a scene" (p. 262).
15. The clerk copying a letter (p. 265); guide lines "fettering to expressive drawing" and "the camera idea of truth" (p. 266).
16. "If the mind is not working, the work of the hand will be of no account" (p. 267); "meaningless shading" (p. 269).
17. The "meretricious attempt to deceive the eye" (p. 271); "Art is not a substitute for nature" (p. 272).
18. "A mechanical standard of accuracy" rejected; the camera to be avoided by students (p. 286); variations "seldom are, conscious" (p. 287).
19. Style "is not a matter where direct teaching can help you" (p. 288).

The Conclusion (XXI) does summarise this: he spoke "strongly against the idea of a mechanical standard of accuracy" (p. 286), but insists this is not licence for carelessness (p. 287).

**Internal tensions worth knowing.**
- *XVI against the Appendix.* XVI is sceptical of canons. The Appendix leans toward φ as something the eye "evidently" enjoys. Ship φ only as an optional guide with caveats.
- *XIX-4 against XIX-7.* "Rub out your work until an earlier stage that was right is reached" versus "do not rub out". Our reading: the first is about wrong foundations and study work, the second about the surface of expressive drawing.
- *XVIII-2 against XVII-4.* Memory "retains more vividly the essential things" versus the tired eye drifting to "dull uniformity". Modern memory science is closer to the second.
- *Accuracy against the vital impression.* Resolved in XXI-2: precision first; the master's variations are seldom conscious.

**Cross-references to earlier chapters (for the other ledgers).** Greek temples balancing straight architecture with rich sculpture (p. 221). Vertical and horizontal lines "the calmest we have" (p. 236). Academic versus vital drawing (p. 248). "As we have already said", a few scribbled lines giving more life than fixed actuality (p. 252). Monochrome in three tones "as explained in a former chapter" (p. 273). The "two extreme points of view" (line and mass) (p. 275). Hals's restless edges tie to ch. XIII's sidenote "Variety of Edges" (file h-2).

**Agent and product safety rules distilled from this range (ours).**
- Never infer character, intellect or emotion type from a face or posture (SP-XVII-8). Keep face landmarks user-placed (SP-XVII-5).
- Never normalise, beautify or average a likeness (SP-XVII-1, SP-XVII-2). Don't mirror paired features without varying them (SP-XVII-12).
- Never claim to measure a user's memory, fatigue, skill or cognition (XVIII, SP-XIX-5, SP-XIX-8).
- Never predict a viewer's eye path. Report geometry only (SP-XVI-7, SP-XV-7).
- Never present φ or any canon as scientifically pleasing (SP-XVI-8, SP-A-4).
- Never fake vitality with random jitter (SP-XXI-2).
- Study references; never copy them (SP-XIX-10, SP-XVII-9; the repository's clean-room rules).
- Say when output was traced from a photo (SP-XXI-1).

**Roll-up of candidate features (ours; for triage).**
- *Engine:* tone-simplify view and flatten-gradients command (XV-4); history checkpoints and branches (XIX-4); pentimenti layer (XIX-7); document intent note and pinned first sketch (XVIII-3, XVIII-7); toned-ground preset (XX-11); palette constraint (XX-4); reference-versus-drawing landmark ruler (XVII-5).
- *Analysis overlays (descriptive, with stated limits):* curvature share (XV-2); flat versus gradient share (XV-3); tonal extremes (XV-5); warm/cool totals and adjacency (XV-6); visual-weight centroid with manual interest weights (XV-7); small-integer and equal-interval flags (XVI-2, XVI-3, XVI-4); mirror-difference (XVI-5, XVII-12); line-extension convergence (XVI-7); top-N area share (XVI-9); checkpoint comparison at thumbnail size (XIX-6); light/dark contact check (XX-11).
- *Improve existing:* vary options for distribute, repeat and blend (XVI-3); non-snapping canon and φ guides (XVI-1, XVI-8, A-1–A-3); pen, pencil and silverpoint presets from width profiles (XX-5, XX-6, XX-13); calligraphic chisel presets (XX-7); stroke-weight tiers (XX-14); onion-skin redraw (XX-15); navigator and fresh-look views (XVII-3, XIX-5).
- *Practice exercises:* Diagram XXVIII equal versus varied (XVI-4); graded memory drawing (XVIII-5, XVIII-6); no-undo mode (XIX-7, XX-6); two-value ink (XX-13); constraint ladder (XX-4); drapery on a toned ground (XX-11).
- *Agent prompts and resources:* plan in medium terms (XVIII-4, XIX-1); masses → check → detail (XVI-9, XVII-6); declare the mood and the dominant aim (XV-1, XVII-13); commit to the scheme (XVIII-8); economy of strokes (XIX-9, XXI-5); "pretty is not correct" (XX-8).

---

## Open questions

1. **Plate IV mismatch.** p. 281 says a drawing "with white pastel and red chalk is reproduced on page 46", which the transcriber maps to Plate IV. But Plate IV's caption reads "Study on tissue-paper in red chalk for figure of Boreas", and I could not see white pastel in the reproduction. Either the transcriber's mapping is wrong or the white did not reproduce. Check against a scan of the printed edition before using it as an example.
2. **Diagram XXIX caption "EC".** The pixel measurements support reading "BC". Confirm against a print scan of p. 289 before encoding the caption verbatim.
3. **Appendix measurements.** Should anyone measure Plates IX and XXXIII–XXXVIII (outside this range) to test the claims? That is analysis rather than reading, the plates are cropped reproductions, and the author gives no tolerance or side. Recommendation (ours): don't build anything on the seven claims; ship φ as a plain optional guide.
4. **Missing page anchors 232–234.** XVI cites "(p. 230)" for the Ansidei plate, but Page_230 is a text page in this edition. Plate page numbering is unclear, so avoid citing page numbers for plates.
5. **Chapter XV's title.** The h2 reads "XV BALANCE", the contents "RHYTHM: BALANCE", and file h-3's `<title>` "XVI RHYTHM: PROPORTION" is an artifact. Which label should public docs use? (Suggest "XV Rhythm: Balance", as in the contents.)
6. **Warm/cool classification (SP-XV-6).** The author only halves the spectrum roughly. The hue-angle boundaries and the chroma threshold for "neutral" are a product decision and should be documented as a convention.
7. **Manual "interest weight" (SP-XV-7).** Is a per-object tag acceptable UI, or should the balance overlay be left out entirely, given how little geometry can say?
8. **History checkpoints (SP-XIX-4).** What does `crates/engine/src/history.rs` already support (named states, branches)? I did not inspect it beyond a filename grep.
9. **Holbein's "zinc white" (SP-XX-9)** and **"The style is the man … Flaubert" (SP-XXI-5).** Both look historically doubtful. Verify before reusing in user-facing text.
10. **Plate LIII.** The unequal pupils the author describes are not visible in this reproduction (he says so himself). Don't use this plate as a visual demonstration; Plate L shows eye and mouth asymmetry more clearly.
11. **Memory practice mode (XVIII).** Is any comparison overlay acceptable under the brief's rule never to claim to measure memory? My suggestion is user-initiated only, worded as geometric difference, with no scores, streaks or progress claims. Needs a product decision.
12. **"Point drawing" scope (SP-XVII-6).** Speed's "point drawing" covers pencil, chalk and pen. Should agent guidance treat vector line work as "point drawing" (flexible order), or default to "painting" order (masses first) when fills are used? We suggest: masses first whenever fills dominate.
