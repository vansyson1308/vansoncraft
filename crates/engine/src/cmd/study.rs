//! Drawing studies: looking aids and construction commands taken from Harold Speed's *The
//! Practice and Science of Drawing* (1913). `docs/speed/` holds the reading notes, the trace from
//! each principle to these commands, and their limits.
//!
//! They measure what is on the page and draw aids for looking; none of them judges the art or
//! reads the artist's intent. Every number is a plain heuristic whose method the params text and
//! the reply's `method` state, so an agent or a person can decide what it is worth.
//!
//! - `study.render`: an artboard or a region at a chosen zoom, in colour, as grey or as value
//!   groups (a value-group map of Speed's masses).
//! - `study.values`: how much of the region each value group covers, how many separate masses it
//!   breaks into, and the contrast between the darkest and the lightest group.
//! - `study.balance`: where the region's dark weight centres and how it spreads over its quarters.
//! - `study.lines`: the directions, the straight and curved share, the inflections and corners of
//!   paths' lines.
//! - `study.proportions`: sizes against a unit object, and edges or centres that line up.
//! - `study.compare`: how closely a drawn shape matches a reference shape (silhouette overlap).
//! - `study.blockIn`: objects' straight-line envelopes and long axes, as construction lines.
//! - `study.hatch`: parallel hatching inside closed shapes, as editable paths.
//!
//! The queries change nothing. `study.blockIn` and `study.hatch` add new objects as one undo
//! step each and never touch the objects they start from.

mod construct;
mod geometry;
mod lines;
mod raster;
#[cfg(test)]
mod tests;

use serde_json::Value;
use vansoncraft_doc::NodeId;

use super::{CommandSpec, Result, Session, bad, cmd, given_objects, has_doc};

/// Most objects one study takes (each with all the paths inside it).
pub(crate) const MAX_OBJECTS: usize = 2000;

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(
            query "study.render",
            "Study View",
            [],
            None,
            "{artboard?: index=0, region?: [x, y, width, height] (document points; default the artboard), scale?: pixels per point (0.01–64; default: the region fitted to maxSide), maxSide=1024 (longest side in pixels, 16–4096), mode=\"color\" (color|gray|values), levels=3 (values mode: 2–5 value groups), thresholds?: [L*…] (values mode: levels − 1 rising lightness boundaries, 0–100; default equal steps of CIE L*), showTemplates?: bool (also draw template layers such as a reference or construction lines; exports leave them out), flip=\"none\" (none|horizontal|vertical|both: mirrored left to right, mirrored top to bottom, or both: turned upside down, a half turn; to see the picture afresh)} the region as exported to PNG on white, at any zoom; gray: each pixel's CIE L* lightness as grey; values: each pixel painted with the mean grey of its value group → {width, height, scale, region: [x, y, width, height], mode, thresholds?, pngBase64}",
            has_doc,
            raster::render
        ),
        cmd!(
            query "study.values",
            "Value Study",
            [],
            None,
            "{artboard?: index=0, region?: [x, y, width, height], scale?: pixels per point (0.01–64; default: the region fitted to maxSide), maxSide=512 (16–2048), levels=3 (2–5), thresholds?: [L*…] (levels − 1 rising boundaries, 0–100; default equal steps of CIE L*), minMass=0.002 (smallest patch counted as a mass, as a share of the region's pixels), image?: bool (also return the value map as pngBase64), showTemplates?: bool} the region rendered as a PNG export on white, each pixel put in a value group by its CIE L* lightness (antialiased edges fall in whichever group their blend reaches) → {width, height, thresholds, extremes: {darkL, lightL (1st and 99th percentile L*), midOffset ((darkL + lightL) / 2 − 50), pureShare (L* below 2 or above 98)}, groups: [{level (0: darkest), name, share (of the pixels), meanL, masses (8-connected patches of at least minMass), largestShare (share of the group's pixels in its largest patch)}], contrast (lightest group's meanL − darkest group's, L* 0–100), method, pngBase64?}",
            has_doc,
            raster::values
        ),
        cmd!(
            query "study.balance",
            "Balance Study",
            [],
            None,
            "{artboard?: index=0, region?: [x, y, width, height], scale?: pixels per point (0.01–64; default: the region fitted to maxSide), maxSide=512 (16–2048), showTemplates?: bool, weigh=\"darkness\" (darkness: each pixel weighs (100 − L*) / 100 | contrast: |L* − the median tone| / 100, for a toned ground)} where the region's tonal weight centres → {centroid: {x, y} (document points; null when nothing weighs), offset: {x, y} (−1…1: from the region's centre to its edges), quadrants: {topLeft, topRight, bottomLeft, bottomRight} (shares of the weight), emptyShare (pixels lighter than L* 90), weigh, ground: {L (median L*), share (pixels within 5 of it)}, method}",
            has_doc,
            raster::balance
        ),
        cmd!(
            query "study.lines",
            "Line Study",
            [],
            None,
            "{ids?: [id…] (default the selection; groups, compound paths and layers are searched for their paths), straightTolerance=2 (degrees a segment may turn per 10 pt and still count as straight), cornerAngle=30 (degrees: a turn at an anchor beyond this is a corner), bendAngle=15 (degrees: straight segments whose joints turn this much or less are read as one curve drawn with points), limit=50 (paths listed in perPath)} the paths' geometry as drawn (not their strokes) → {paths, totalLength (pt), straightShare, curvedShare, corners, inflections, curvatureVariation (coefficient of variation of the curved parts' curvature: 0 for arcs of one radius; any wiggle raises it), directions: [{angle, share}] (8 bins of 22.5°, length-weighted; 0: horizontal, 90: vertical, counter-clockwise as seen), meanOrientation, spread (0: one orientation … 1: no single dominant orientation, uniform or evenly split between two), perPath: [{id, name?, length, straightShare, corners, inflections, curvatureVariation, closed, meanOrientation}], truncated, method}",
            has_doc,
            lines::lines
        ),
        cmd!(
            query "study.proportions",
            "Proportion Study",
            [],
            None,
            "{ids?: [id…] (default the selection), unit?: id (the object whose height is the unit; default the first), tolerance=1 (pt within which edges or centres line up), limit=100 (alignments listed)} objects' geometric bounds measured against the unit → {unit: {id, height}, objects: [{id, name?, x, y, width, height, aspect (width / height), heightInUnits, widthInUnits, center: {x, y}}], alignments: [{axis: vertical|horizontal, edge: left|center|right|top|middle|bottom, at, ids}], angles: [{from, to, degrees (0: level, counter-clockwise as seen), distance}] (between the centres of objects listed one after another), truncated}",
            has_doc,
            lines::proportions
        ),
        cmd!(
            query "study.compare",
            "Compare Shapes",
            [],
            None,
            "{a: id (the study), b: id (the reference), normalize=true (scale a by height and centre it on b first: the shapes are compared, not their places), resolution=256 (grid cells along the longer side, 32–1024), image?: bool (a PNG: grey where both cover, red where only a does, blue where only b does)} the filled areas of two objects' paths (a group's paths together) compared on a grid → {iou (overlap over union, 0–1), extra (share of a outside b), missing (share of b outside a), aspectA, aspectB, centroidOffset: {x, y} (pt, a's minus b's, after normalising), scale (applied to a), method, pngBase64?}",
            has_doc,
            construct::compare
        ),
        cmd!(
            "study.blockIn",
            "Block In",
            [],
            None,
            "{ids?: [id…] (default the selection), tolerance?: pt (how far the straight lines may stray from the outline; default 4 % of each object's larger side), maxSides=12 (3–64), hull?: bool (the convex envelope instead of the outline's main bends), axis?: bool (also draw each object's long axis), as=\"construction\" (construction: on the locked template layer `Construction`, which exports leave out | art: above each object, on its layer, to refine), stroke=\"#3a7bd5\", weight=0.75 (pt)} each object's outline as a few straight lines, as one undo step; the objects are left as they were; the corners of the lines lie on the outline → {ids: [id…], layer?, sides: [n…], maxDeviation: [pt…] (how far each outline strays from its lines; maxSides can make it more than tolerance), axes?: [degrees…]}",
            has_doc,
            construct::block_in
        ),
        cmd!(
            "study.hatch",
            "Hatch",
            [],
            None,
            "{ids?: [id…] (default the selection: closed paths, compound paths or groups of them), spacing=4 (pt between lines, 0.25–1000), angle=45 (degrees, 0: level, counter-clockwise as seen), cross?: bool (a second set across the first), crossAngle?: degrees (the second set's angle, counter-clockwise as seen; default angle + 90), weight=0.5 (pt), color=\"#000000\", inset=0 (pt each line stops short of the outline), minLength=0 (pt: shorter lines are left out, such as specks where a line grazes the shape), jitter=0 (0–0.5 of the spacing each line is moved by, from seed), seed=1, maxLines=2000 (at most 20000; more lines than this is an error, not a cut)} lines inside each object's fill area, clipped to it exactly as editable paths in a group above the object (its recipe in the group's data), as one undo step → {groups: [id…], lines}",
            has_doc,
            construct::hatch
        ),
    ]
}

/// The objects a study works on: `ids` (or `id`) when given, else the selection.
pub(crate) fn targets(s: &Session, p: &Value, c: &str) -> Result<Vec<NodeId>> {
    let ids = match given_objects(s, p, c)? {
        Some(ids) => ids,
        None => s.doc()?.selection.objects.clone(),
    };
    if ids.is_empty() {
        return Err(bad(c, "select objects or give `ids`"));
    }
    if ids.len() > MAX_OBJECTS {
        return Err(bad(c, format!("at most {MAX_OBJECTS} objects")));
    }
    Ok(ids)
}

/// An optional number in `lo..=hi`, else `default`.
pub(crate) fn num(p: &Value, key: &str, default: f64, lo: f64, hi: f64, c: &str) -> Result<f64> {
    match p.get(key) {
        None | Some(Value::Null) => Ok(default),
        Some(v) => {
            v.as_f64().filter(|x| x.is_finite() && (lo..=hi).contains(x)).ok_or_else(|| bad(c, format!("`{key}` must be a number from {lo} to {hi}")))
        }
    }
}

/// An optional whole number in `lo..=hi`, else `default`.
pub(crate) fn whole(p: &Value, key: &str, default: u64, lo: u64, hi: u64, c: &str) -> Result<u64> {
    match p.get(key) {
        None | Some(Value::Null) => Ok(default),
        Some(v) => v.as_u64().filter(|x| (lo..=hi).contains(x)).ok_or_else(|| bad(c, format!("`{key}` must be a whole number from {lo} to {hi}"))),
    }
}

/// An optional boolean, else `false`.
pub(crate) fn flag(p: &Value, key: &str, c: &str) -> Result<bool> {
    match p.get(key) {
        None | Some(Value::Null) => Ok(false),
        Some(Value::Bool(b)) => Ok(*b),
        Some(_) => Err(bad(c, format!("`{key}` must be true or false"))),
    }
}

/// A value rounded for a reply: numbers an agent reads, not more digits than they carry.
pub(crate) fn r3(x: f64) -> f64 {
    if x.is_finite() { (x * 1000.0).round() / 1000.0 } else { 0.0 }
}
