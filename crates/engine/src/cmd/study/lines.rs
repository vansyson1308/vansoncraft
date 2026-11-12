//! The studies that read geometry: `study.lines` (Speed's rhythm of line: direction, straight and
//! curved, corners and inflections) and `study.proportions` (sizes measured against a unit, and
//! the plumb and level lines that pass through several objects).
//!
//! Angles are as seen on screen: 0 is level and they turn counter-clockwise, so a line rising to
//! the right is 45° although the document's y axis points down.

use std::f64::consts::PI;

use serde_json::{Value, json};
use vectorcraft_doc::NodeId;
use vectorcraft_geom::{ParamCurve, Point, SubPath};

use super::geometry::{PathItem, paths_in};
use super::{num, r3, targets, whole};
use crate::cmd::{bad, checked_id};
use crate::{Result, Session};

/// Samples per curved segment.
const SAMPLES: usize = 16;
/// Most segments one line study reads.
const MAX_SEGMENTS: usize = 200_000;

/// What one path's lines add up to.
#[derive(Default, Clone)]
struct Tally {
    length: f64,
    straight: f64,
    corners: usize,
    inflections: usize,
    /// Length-weighted sums of cos 2θ and sin 2θ (orientation, ignoring which way a line runs).
    c2: f64,
    s2: f64,
    bins: [f64; 8],
    /// (curvature, length) of the curved samples.
    curvature: Vec<(f64, f64)>,
    segments: usize,
}

impl Tally {
    fn add(&mut self, o: &Tally) {
        self.length += o.length;
        self.straight += o.straight;
        self.corners += o.corners;
        self.inflections += o.inflections;
        self.c2 += o.c2;
        self.s2 += o.s2;
        for (a, b) in self.bins.iter_mut().zip(o.bins) {
            *a += b;
        }
        self.curvature.extend_from_slice(&o.curvature);
        self.segments += o.segments;
    }
    fn chord(&mut self, a: Point, b: Point) {
        let len = a.distance(b);
        if len <= 1e-9 {
            return;
        }
        // As seen: y up.
        let theta = (-(b.y - a.y)).atan2(b.x - a.x);
        self.c2 += len * (2.0 * theta).cos();
        self.s2 += len * (2.0 * theta).sin();
        let deg = theta.to_degrees().rem_euclid(180.0);
        let bin = (((deg + 11.25) / 22.5).floor() as usize) % 8;
        if let Some(b) = self.bins.get_mut(bin) {
            *b += len;
        }
    }
    fn orientation(&self) -> f64 {
        r3((0.5 * self.s2.atan2(self.c2)).to_degrees().rem_euclid(180.0))
    }
}

/// The turn (radians, signed) from direction `u` to direction `v`.
fn turn(u: vectorcraft_geom::Vec2, v: vectorcraft_geom::Vec2) -> f64 {
    u.cross(v).atan2(u.dot(v))
}

/// One segment, sampled: its chords' lengths and directions, and the turns between them.
struct Seg {
    length: f64,
    /// Turning inside the segment: (signed turn, length of the chord after it).
    turns: Vec<(f64, f64)>,
    first: vectorcraft_geom::Vec2,
    last: vectorcraft_geom::Vec2,
    line: bool,
}

/// The segments of `sp`, sampled `SAMPLES` times a curve; zero-length ones left out.
fn segments(sp: &SubPath, t: &mut Tally) -> Vec<Seg> {
    let mut out = Vec::new();
    for i in 0..sp.segment_count() {
        let c = sp.segment(i);
        let line = sp.segment_is_line(i);
        let pts: Vec<Point> = if line { vec![c.p0, c.p3] } else { (0..=SAMPLES).map(|k| c.eval(k as f64 / SAMPLES as f64)).collect() };
        let (mut length, mut turns, mut first, mut last) = (0.0, Vec::new(), None, None::<vectorcraft_geom::Vec2>);
        for w in pts.windows(2) {
            let (Some(&a), Some(&b)) = (w.first(), w.get(1)) else { continue };
            let len = a.distance(b);
            if len <= 1e-9 {
                continue;
            }
            t.chord(a, b);
            let dir = (b - a) / len;
            if let Some(u) = last {
                turns.push((turn(u, dir), len));
            }
            first.get_or_insert(dir);
            last = Some(dir);
            length += len;
        }
        if let (Some(first), Some(last)) = (first, last) {
            out.push(Seg { length, turns, first, last, line });
        }
    }
    out
}

/// One subpath's tally, with `straight_tol` (radians per 10 pt) and `corner` (radians).
///
/// A run of straight segments whose joints turn by less than `corner` is read as one sampled curve
/// (a polygon standing for a circle, a curve drawn with plain points): its joints are its bends,
/// so it counts as curved, with inflections where the bends change side. A turn beyond `corner`
/// at a joint is a corner.
fn subpath_tally(sp: &SubPath, straight_tol: f64, bend: f64, corner: f64) -> Tally {
    let mut t = Tally::default();
    let segs = segments(sp, &mut t);
    let n = segs.len();
    // The signed bends in path order (inside segments and at smooth joints), for inflections.
    let mut bends: Vec<f64> = Vec::new();
    // Whether each straight segment is part of a polyline curve (a smooth joint on either side).
    let mut bent = vec![false; n];
    let joints = if sp.closed && n > 1 { n } else { n.saturating_sub(1) };
    let mut joint_turn = vec![None; n];
    for j in 0..joints {
        let (Some(a), Some(b)) = (segs.get(j), segs.get((j + 1) % n)) else { continue };
        let a_turn = turn(a.last, b.first);
        if a_turn.abs() > corner {
            t.corners += 1;
        } else if a.line && b.line && a_turn.abs() > 1e-4 && a_turn.abs() <= bend {
            if let Some(x) = bent.get_mut(j) {
                *x = true;
            }
            if let Some(x) = bent.get_mut((j + 1) % n) {
                *x = true;
            }
            t.curvature.push((a_turn.abs() / ((a.length + b.length) / 2.0).max(1e-9), (a.length + b.length) / 2.0));
        }
        if let Some(x) = joint_turn.get_mut(j) {
            *x = Some(a_turn);
        }
    }
    for (i, s) in segs.iter().enumerate() {
        t.segments += 1;
        t.length += s.length;
        let inside: f64 = s.turns.iter().map(|(a, _)| a.abs()).sum();
        let curved = if s.line { bent.get(i).copied().unwrap_or(false) } else { inside * 10.0 / s.length >= straight_tol };
        if curved {
            t.curvature.extend(s.turns.iter().map(|(a, len)| (a.abs() / len, *len)));
        } else {
            t.straight += s.length;
        }
        bends.extend(s.turns.iter().map(|(a, _)| *a));
        let next_line = segs.get((i + 1) % n.max(1)).is_some_and(|b| b.line);
        match joint_turn.get(i).copied().flatten() {
            // A corner, or an angle where two straight lines meet, starts the next curve afresh.
            Some(a) if a.abs() > corner || (s.line && next_line && a.abs() > bend) => bends.push(f64::NAN),
            Some(a) => bends.push(a),
            None => {}
        }
    }
    t.inflections = inflections(&bends);
    t
}

/// Changes of the side the bends turn to; a NaN (a corner) starts afresh.
fn inflections(bends: &[f64]) -> usize {
    let (mut count, mut last_sign) = (0, 0i8);
    for b in bends {
        if b.is_nan() {
            last_sign = 0;
            continue;
        }
        // Turning this small is noise from sampling, not a change of direction.
        if b.abs() > 1e-4 {
            let sign = if *b > 0.0 { 1 } else { -1 };
            if last_sign != 0 && sign != last_sign {
                count += 1;
            }
            last_sign = sign;
        }
    }
    count
}

/// The coefficient of variation of the curved samples' curvature, length-weighted.
fn curvature_variation(samples: &[(f64, f64)]) -> f64 {
    let total: f64 = samples.iter().map(|(_, l)| l).sum();
    if total <= 1e-9 {
        return 0.0;
    }
    let mean = samples.iter().map(|(k, l)| k * l).sum::<f64>() / total;
    if mean <= 1e-12 {
        return 0.0;
    }
    let var = samples.iter().map(|(k, l)| (k - mean).powi(2) * l).sum::<f64>() / total;
    r3(var.sqrt() / mean)
}

/// `study.lines`.
pub(super) fn lines(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "study.lines";
    let ids = targets(s, p, C)?;
    let straight_tol = num(p, "straightTolerance", 2.0, 0.0, 90.0, C)?.to_radians();
    let corner = num(p, "cornerAngle", 30.0, 1.0, 179.0, C)?.to_radians();
    let bend = num(p, "bendAngle", 15.0, 0.1, 179.0, C)?.to_radians().min(corner);
    let limit = whole(p, "limit", 50, 0, 10_000, C)? as usize;
    let doc = &s.doc()?.doc;
    let mut items: Vec<PathItem> = Vec::new();
    for id in &ids {
        let n = doc.node(*id).ok_or_else(|| bad(C, format!("no object {}", id.0)))?;
        paths_in(n, &mut items, 0);
    }
    if items.is_empty() {
        return Err(bad(C, "no paths among the objects"));
    }
    let mut all = Tally::default();
    let mut per = Vec::new();
    for it in &items {
        let mut t = Tally::default();
        for sp in &it.path.subpaths {
            t.add(&subpath_tally(sp, straight_tol, bend, corner));
        }
        all.add(&t);
        if all.segments > MAX_SEGMENTS {
            return Err(bad(C, format!("the paths have more than {MAX_SEGMENTS} segments")));
        }
        if per.len() < limit {
            let mut e = json!({
                "id": it.id.0,
                "length": r3(t.length),
                "straightShare": if t.length > 0.0 { r3(t.straight / t.length) } else { 0.0 },
                "corners": t.corners,
                "inflections": t.inflections,
                "curvatureVariation": curvature_variation(&t.curvature),
                "closed": !it.path.subpaths.is_empty() && it.path.subpaths.iter().all(|s| s.closed),
                "meanOrientation": t.orientation(),
            });
            if let Some(name) = it.name {
                e["name"] = json!(name);
            }
            per.push(e);
        }
    }
    let len = all.length.max(1e-9);
    let straight = all.straight / len;
    let r = (all.c2.hypot(all.s2) / len).clamp(0.0, 1.0);
    let directions: Vec<Value> = all.bins.iter().enumerate().map(|(i, b)| json!({"angle": i as f64 * 22.5, "share": r3(b / len)})).collect();
    Ok(json!({
        "paths": items.len(),
        "totalLength": r3(all.length),
        "straightShare": r3(straight),
        "curvedShare": r3(1.0 - straight),
        "corners": all.corners,
        "inflections": all.inflections,
        "curvatureVariation": curvature_variation(&all.curvature),
        "directions": directions,
        "meanOrientation": all.orientation(),
        "spread": r3(1.0 - r),
        "perPath": per,
        "truncated": items.len() > limit,
        "method": "each path's segments as drawn (not its stroke outline), curves sampled 16 times a segment. A curved segment is straight when it turns less than straightTolerance per 10 pt; straight segments whose joints turn by bendAngle or less form one curve drawn with points and count as curved (straight lines meeting at a larger angle stay straight); an inflection is a change in the side of turning; a corner is a turn of more than cornerAngle at an anchor; spread is 1 − the length-weighted mean resultant of the doubled line angles. These count shapes; whether a rhythm works is for the eye to judge.",
    }))
}

/// One object's measured box.
struct Measured {
    id: NodeId,
    name: Option<String>,
    b: vectorcraft_geom::Rect,
}

/// `study.proportions`.
pub(super) fn proportions(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "study.proportions";
    let ids = targets(s, p, C)?;
    let tol = num(p, "tolerance", 1.0, 0.0, 1000.0, C)?;
    let limit = whole(p, "limit", 100, 0, 10_000, C)? as usize;
    let unit_id = match p.get("unit").filter(|v| !v.is_null()) {
        Some(v) => Some(checked_id(s, v, C)?),
        None => None,
    };
    let doc = &s.doc()?.doc;
    let mut objs = Vec::new();
    for id in &ids {
        let n = doc.node(*id).ok_or_else(|| bad(C, format!("no object {}", id.0)))?;
        let b = doc.bounds_of(&[*id], false).ok_or_else(|| bad(C, format!("object {} has no bounds", id.0)))?;
        objs.push(Measured { id: *id, name: n.name.clone(), b });
    }
    let unit_id = unit_id.or_else(|| objs.first().map(|o| o.id)).ok_or_else(|| bad(C, "no objects"))?;
    let unit_h = doc.bounds_of(&[unit_id], false).map(|b| b.height()).ok_or_else(|| bad(C, format!("the unit {} has no bounds", unit_id.0)))?;
    if unit_h <= 1e-9 {
        return Err(bad(C, "the unit object has no height"));
    }
    let objects: Vec<Value> = objs
        .iter()
        .map(|o| {
            let b = o.b;
            let mut e = json!({
                "id": o.id.0, "x": r3(b.x0), "y": r3(b.y0), "width": r3(b.width()), "height": r3(b.height()),
                "aspect": if b.height() > 1e-9 { json!(r3(b.width() / b.height())) } else { Value::Null },
                "heightInUnits": r3(b.height() / unit_h), "widthInUnits": r3(b.width() / unit_h),
                "center": {"x": r3(b.center().x), "y": r3(b.center().y)},
            });
            if let Some(n) = &o.name {
                e["name"] = json!(n);
            }
            e
        })
        .collect();
    let mut alignments = Vec::new();
    let edges: [(&str, &str, fn(&vectorcraft_geom::Rect) -> f64); 6] = [
        ("vertical", "left", |b| b.x0),
        ("vertical", "center", |b| b.center().x),
        ("vertical", "right", |b| b.x1),
        ("horizontal", "top", |b| b.y0),
        ("horizontal", "middle", |b| b.center().y),
        ("horizontal", "bottom", |b| b.y1),
    ];
    for (axis, edge, at) in edges {
        let mut v: Vec<(f64, u64)> = objs.iter().map(|o| (at(&o.b), o.id.0)).collect();
        v.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut run: Vec<(f64, u64)> = Vec::new();
        let mut flush = |run: &mut Vec<(f64, u64)>| {
            if run.len() >= 2 {
                let at = run.iter().map(|r| r.0).sum::<f64>() / run.len() as f64;
                alignments.push(json!({"axis": axis, "edge": edge, "at": r3(at), "ids": run.iter().map(|r| r.1).collect::<Vec<_>>()}));
            }
            run.clear();
        };
        for e in v {
            if run.first().is_some_and(|f| e.0 - f.0 > tol) {
                flush(&mut run);
            }
            run.push(e);
        }
        flush(&mut run);
    }
    let truncated = alignments.len() > limit;
    alignments.truncate(limit);
    let angles: Vec<Value> = objs
        .windows(2)
        .filter_map(|w| {
            let (a, b) = (w.first()?, w.get(1)?);
            let (ca, cb) = (a.b.center(), b.b.center());
            let deg = (-(cb.y - ca.y)).atan2(cb.x - ca.x) * 180.0 / PI;
            Some(json!({"from": a.id.0, "to": b.id.0, "degrees": r3(deg), "distance": r3(ca.distance(cb))}))
        })
        .collect();
    Ok(json!({
        "method": "geometric bounds (not stroke outlines) measured against the unit's height; alignments are runs of edges or centres within tolerance of the run's first; angles join the centres of objects listed one after another, 0 level and counter-clockwise as seen. Measurements are approximate guides: a drawing may depart from them on purpose.",
        "unit": {"id": unit_id.0, "height": r3(unit_h)},
        "objects": objects,
        "alignments": alignments,
        "angles": angles,
        "truncated": truncated,
    }))
}
