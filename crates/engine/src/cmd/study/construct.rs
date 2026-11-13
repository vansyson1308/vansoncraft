//! The studies that draw: `study.blockIn` (Speed's blocking in of a shape with a few straight
//! lines, and its long axis) and `study.hatch` (lines laid inside a shape, clipped to it exactly);
//! and `study.compare`, which measures how closely one filled shape matches another.
//!
//! New objects are ordinary editable paths with the recipe that made them in their data (`data-*`
//! in SVG), added as one undo step. The objects they start from are never changed.

use serde_json::{Value, json};
use vansoncraft_color::{Color, Paint};
use vansoncraft_doc::{Appearance, Document, Node, NodeId, NodeKind};
use vansoncraft_geom::{PathData, Point, SubPath};

use super::geometry::{MAX_POINTS, Shape, convex_hull, long_axis, rdp_closed, reduce_closed, rotated, scanline, shape_of, signed_area2};
use super::{flag, num, r3, targets, whole};
use crate::cmd::{bad, checked_id, str_param};
use crate::{Result, Session};

/// The template layer block-in lines go on by default.
pub(crate) const CONSTRUCTION: &str = "Construction";
/// Most lines one `study.hatch` may make, whatever `maxLines` says.
const HARD_MAX_LINES: u64 = 20_000;
/// Most scanlines (rows times flattened points) one hatch may walk.
const MAX_SCAN_WORK: f64 = 4e8;

/// A colour param (`#rrggbb` or a CSS name the app knows), else `default`.
fn color_param(p: &Value, key: &str, default: &str, c: &str) -> Result<Color> {
    let s = match p.get(key).filter(|v| !v.is_null()) {
        None => default,
        Some(Value::String(s)) => s.as_str(),
        Some(_) => return Err(bad(c, format!("`{key}` must be a colour such as \"#3a7bd5\""))),
    };
    let digits = s.trim().trim_start_matches('#');
    if digits.len() == 8 || digits.len() == 4 {
        return Err(bad(c, format!("`{key}`: #rrggbbaa and #rgba are refused (no alpha)")));
    }
    Color::from_hex(s).ok_or_else(|| bad(c, format!("`{key}` {s:?} is not a colour (#rrggbb)")))
}

/// An open or closed polyline as a path of corner anchors.
fn polyline(points: &[Point], closed: bool) -> PathData {
    PathData::single(SubPath::polyline(points, closed))
}

/// A stroked, unfilled path.
fn line_node(d: &mut Document, path: PathData, color: Color, weight: f64, name: String, data: Vec<(String, String)>) -> Node {
    let id = d.alloc_id();
    let mut n = Node::path(id, path, Appearance::basic(Paint::None, Paint::solid(color), weight));
    n.name = Some(name);
    n.edit_attrs(|a| a.data = data);
    n
}

/// Where to put new art that belongs to `src`: just above it in its parent (above a compound path
/// it is a member of; at the top of a layer given itself).
fn above(d: &Document, src: NodeId) -> Option<(Option<NodeId>, usize)> {
    let n = d.node(src)?;
    if n.is_layer() {
        return Some((Some(src), usize::MAX));
    }
    let mut at = src;
    while let Some(par) = d.parent_of(at)
        && d.node(par).is_some_and(|p| matches!(p.kind, NodeKind::Compound { .. }))
    {
        at = par;
    }
    let (parent, i, _) = d.position(at)?;
    Some((parent, i + 1))
}

/// The top-level template layer named [`CONSTRUCTION`], made (locked, not printing, on top) when
/// the document has none.
fn construction_layer(d: &mut Document) -> Result<NodeId> {
    if let Some(l) = d.layers.iter().find(|l| l.name.as_deref() == Some(CONSTRUCTION) && matches!(l.kind, NodeKind::Layer { template: true, .. })) {
        return Ok(l.id);
    }
    let mut n = crate::cmd::layer::make_layer(d, Some(CONSTRUCTION));
    if let NodeKind::Layer { template, printable, dim_images, .. } = &mut n.kind {
        *template = true;
        *printable = false;
        *dim_images = Some(50);
    }
    n.locked = true;
    Ok(d.insert(None, usize::MAX, n)?)
}

/// One object's block-in: the straight-line ring and, if asked, its long axis.
struct Blocked {
    src: NodeId,
    ring: Vec<Point>,
    axis: Option<(Point, Point)>,
    tolerance: f64,
    /// How far the outline strays from the straight lines at most (pt): `maxSides` can make it more
    /// than the tolerance, and corners of the outline may be cut by up to it.
    deviation: f64,
}

/// The farthest any of `pts` lies from the closed polygon `ring`.
fn max_deviation(pts: &[Point], ring: &[Point]) -> f64 {
    let n = ring.len();
    pts.iter()
        .map(|p| (0..n).filter_map(|i| Some(super::geometry::seg_distance(*p, *ring.get(i)?, *ring.get((i + 1) % n)?))).fold(f64::INFINITY, f64::min))
        .filter(|d| d.is_finite())
        .fold(0.0, f64::max)
}

/// The block-in of one object's shape.
fn block_shape(src: NodeId, shape: &Shape, tol: Option<f64>, max_sides: usize, hull: bool, axis: bool) -> std::result::Result<Blocked, String> {
    let bounds = shape.bounds().ok_or_else(|| format!("object {} has no paths to block in", src.0))?;
    let tolerance = tol.unwrap_or_else(|| (bounds.width().max(bounds.height()) * 0.04).max(0.5));
    let all: Vec<Point> = shape.rings().flat_map(|r| r.points.iter().copied()).collect();
    // The outline's main bends: the largest closed ring, simplified. Open or tiny shapes fall back
    // on their envelope.
    let outer = shape
        .rings()
        .filter(|r| r.closed && r.points.len() >= 3)
        .max_by(|a, b| signed_area2(&a.points).abs().total_cmp(&signed_area2(&b.points).abs()));
    // Simplified within the tolerance first, so the reduction to maxSides works on few points.
    let ring = match outer {
        Some(r) if !hull => rdp_closed(&r.points, tolerance),
        _ => rdp_closed(&convex_hull(&all), tolerance),
    };
    // A ring that is still very long (a tolerance far below the drawing's detail) is simplified
    // more coarsely before the quadratic reduction.
    let mut ring = ring;
    let mut coarse = tolerance;
    while ring.len() > 2000 {
        coarse *= 2.0;
        ring = rdp_closed(&ring, coarse);
    }
    let ring = reduce_closed(ring, max_sides);
    if ring.len() < 3 {
        return Err(format!("object {} is too thin to block in", src.0));
    }
    let axis = if axis { long_axis(&all) } else { None };
    let deviation = max_deviation(&all, &ring);
    Ok(Blocked { src, ring, axis, tolerance, deviation })
}

/// `study.blockIn`.
pub(super) fn block_in(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "study.blockIn";
    let ids = targets(s, p, C)?;
    let tol = match p.get("tolerance").filter(|v| !v.is_null()) {
        Some(_) => Some(num(p, "tolerance", 1.0, 0.01, 10_000.0, C)?),
        None => None,
    };
    let max_sides = whole(p, "maxSides", 12, 3, 64, C)? as usize;
    let hull = flag(p, "hull", C)?;
    let want_axis = flag(p, "axis", C)?;
    let as_art = match str_param(p, "as") {
        None | Some("construction") => false,
        Some("art") => true,
        Some(o) => return Err(bad(C, format!("`as` must be \"construction\" or \"art\", not {o:?}"))),
    };
    let color = color_param(p, "stroke", "#3a7bd5", C)?;
    let weight = num(p, "weight", 0.75, 0.01, 1000.0, C)?;
    let doc = s.doc()?.doc.clone();
    let mut budget = MAX_POINTS;
    let mut blocks = Vec::new();
    for id in &ids {
        let shape = shape_of(&doc, *id, &mut budget).map_err(|e| bad(C, e))?;
        blocks.push(block_shape(*id, &shape, tol, max_sides, hull, want_axis).map_err(|e| bad(C, e))?);
    }
    let (made, layer) = s.edit("Block In", |d, sel| {
        let layer = if as_art { None } else { Some(construction_layer(d)?) };
        let mut made = Vec::new();
        for b in &blocks {
            let data = |what: &str| {
                vec![
                    ("study".to_string(), what.to_string()),
                    ("source".to_string(), b.src.0.to_string()),
                    ("tolerance".to_string(), format!("{}", r3(b.tolerance))),
                    ("hull".to_string(), hull.to_string()),
                ]
            };
            let place = match layer {
                Some(l) => (Some(l), usize::MAX),
                None => above(d, b.src).ok_or(crate::EngineError::NoNode(b.src))?,
            };
            let ring = line_node(d, polyline(&b.ring, true), color, weight, format!("Block-in {}", b.src.0), data("blockIn"));
            let mut nodes = vec![ring];
            if let Some((a, z)) = b.axis {
                nodes.push(line_node(d, polyline(&[a, z], false), color, weight, format!("Axis {}", b.src.0), data("axis")));
            }
            for (k, n) in nodes.into_iter().enumerate() {
                let index = if place.1 == usize::MAX { usize::MAX } else { place.1 + k };
                made.push(d.insert(place.0, index, n)?);
            }
        }
        if as_art {
            sel.set(made.iter().copied());
        }
        Ok((made, layer))
    })?;
    let mut out = json!({
        "ids": made.iter().map(|i| i.0).collect::<Vec<_>>(),
        "sides": blocks.iter().map(|b| b.ring.len()).collect::<Vec<_>>(),
        "maxDeviation": blocks.iter().map(|b| r3(b.deviation)).collect::<Vec<_>>(),
    });
    if let Some(l) = layer {
        out["layer"] = json!(l.0);
    }
    if want_axis {
        out["axes"] = json!(
            blocks.iter().filter_map(|b| b.axis).map(|(a, z)| r3((-(z.y - a.y)).atan2(z.x - a.x).to_degrees().rem_euclid(180.0))).collect::<Vec<_>>()
        );
    }
    Ok(out)
}

/// The settings of one `study.hatch`.
struct Hatching {
    spacing: f64,
    /// Lines shorter than this (pt) are left out: specks where a band only grazes the shape.
    min_length: f64,
    angles: Vec<f64>,
    inset: f64,
    jitter: f64,
    seed: u64,
}

/// A small, fixed pseudo-random sequence (SplitMix64): the same seed gives the same lines on
/// every machine and in every version that keeps this function.
fn splitmix(state: &mut u64) -> f64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z >> 11) as f64 / (1u64 << 53) as f64
}

/// `p` with the rounding noise of the turn back taken off (to a millionth of a point) and kept
/// inside the shape's bounds, so a line never pokes out of its shape by 1e-14 and an
/// edges-included search such as `document.find {within}` still finds it.
fn settle(p: Point, b: vansoncraft_geom::Rect) -> Point {
    let r = |v: f64| (v * 1e6).round() / 1e6;
    Point::new(r(p.x).clamp(b.x0, b.x1), r(p.y).clamp(b.y0, b.y1))
}

/// The hatching lines inside `shape`, in document coordinates.
fn hatch_lines(shape: &Shape, h: &Hatching, cap: usize) -> std::result::Result<Vec<(Point, Point)>, String> {
    let Some(bounds) = shape.bounds() else { return Ok(Vec::new()) };
    let mut out = Vec::new();
    let mut rng = h.seed;
    for &deg in &h.angles {
        // Turned so that lines at `deg` (as seen) run level.
        let a = deg.to_radians();
        let turned = rotated(shape, a);
        let Some(b) = turned.bounds() else { continue };
        let rows = ((b.height() / h.spacing).ceil()).max(0.0);
        if rows * turned.point_count() as f64 > MAX_SCAN_WORK || rows > 1e6 {
            return Err("the shape is too large for this spacing: raise `spacing`".into());
        }
        let back = |x: f64, y: f64| {
            let (s, c) = (-a).sin_cos();
            Point::new(x * c - y * s, x * s + y * c)
        };
        for k in 0..rows as u64 {
            let wobble = if h.jitter > 0.0 { (splitmix(&mut rng) - 0.5) * 2.0 * h.jitter * h.spacing } else { 0.0 };
            let y = b.y0 + h.spacing * (k as f64 + 0.5) + wobble;
            for (x0, x1) in scanline(&turned, y) {
                let (x0, x1) = (x0 + h.inset, x1 - h.inset);
                if x1 - x0 > h.min_length.max(0.05) {
                    // Refused as soon as it is too many, before the lines pile up in memory.
                    if out.len() >= cap {
                        return Err(format!("the hatching needs more than {cap} lines (maxLines): raise `spacing` or `maxLines`"));
                    }
                    out.push((settle(back(x0, y), bounds), settle(back(x1, y), bounds)));
                }
            }
        }
    }
    Ok(out)
}

/// Whether `shape` fills any area by its fill rule: some scanline across it has a stretch inside.
/// (Not the signed area: the two lobes of a figure eight cancel out to none, yet both are filled.)
fn has_area(shape: &Shape) -> bool {
    let Some(b) = shape.bounds() else { return false };
    if b.height() <= 1e-9 || b.width() <= 1e-9 {
        return false;
    }
    (0..64).any(|k| scanline(shape, b.y0 + b.height() * (k as f64 + 0.5) / 64.0).iter().any(|(x0, x1)| x1 - x0 > 1e-9))
}

/// `study.hatch`.
pub(super) fn hatch(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "study.hatch";
    let ids = targets(s, p, C)?;
    let spacing = num(p, "spacing", 4.0, 0.25, 1000.0, C)?;
    let angle = num(p, "angle", 45.0, -3600.0, 3600.0, C)?;
    let cross = flag(p, "cross", C)?;
    let cross_angle = num(p, "crossAngle", angle + 90.0, -3600.0, 3600.0, C)?;
    let weight = num(p, "weight", 0.5, 0.01, 1000.0, C)?;
    let color = color_param(p, "color", "#000000", C)?;
    let inset = num(p, "inset", 0.0, 0.0, 1000.0, C)?;
    let min_length = num(p, "minLength", 0.0, 0.0, 1000.0, C)?;
    let jitter = num(p, "jitter", 0.0, 0.0, 0.5, C)?;
    let seed = whole(p, "seed", 1, 0, u64::MAX, C)?;
    let max_lines = whole(p, "maxLines", 2000, 1, HARD_MAX_LINES, C)? as usize;
    let mut angles = vec![angle];
    if cross {
        angles.push(cross_angle);
    }
    let h = Hatching { spacing, min_length, angles, inset, jitter, seed };
    let doc = s.doc()?.doc.clone();
    let mut budget = MAX_POINTS;
    let mut sets = Vec::new();
    let mut count = 0usize;
    for id in &ids {
        let shape = shape_of(&doc, *id, &mut budget).map_err(|e| bad(C, e))?;
        if !has_area(&shape) {
            return Err(bad(C, format!("object {} has no area to hatch", id.0)));
        }
        let lines = hatch_lines(&shape, &h, max_lines.saturating_sub(count)).map_err(|e| bad(C, e))?;
        count += lines.len();
        if count > max_lines {
            return Err(bad(C, format!("the hatching needs more than {max_lines} lines (maxLines): raise `spacing` or `maxLines`")));
        }
        sets.push((*id, lines));
    }
    let recipe = vec![
        ("study".to_string(), "hatch".to_string()),
        ("version".to_string(), "1".to_string()),
        ("spacing".to_string(), format!("{}", r3(spacing))),
        ("angle".to_string(), format!("{}", r3(angle))),
        ("cross".to_string(), if cross { format!("{}", r3(cross_angle)) } else { "none".to_string() }),
        ("jitter".to_string(), format!("{}", r3(jitter))),
        ("seed".to_string(), seed.to_string()),
        ("inset".to_string(), format!("{}", r3(inset))),
        ("minLength".to_string(), format!("{}", r3(min_length))),
        ("weight".to_string(), format!("{}", r3(weight))),
        ("color".to_string(), color.to_hex()),
    ];
    let groups = s.edit("Hatch", |d, sel| {
        let mut groups = Vec::new();
        for (src, lines) in &sets {
            let (parent, index) = above(d, *src).ok_or(crate::EngineError::NoNode(*src))?;
            let gid = d.alloc_id();
            let mut g = Node::new(gid, NodeKind::Group { children: Vec::new(), clip: false });
            g.name = Some(format!("Hatch {}° {} pt", r3(angle), r3(spacing)));
            let mut data = recipe.clone();
            data.push(("source".to_string(), src.0.to_string()));
            g.edit_attrs(|a| a.data = data);
            d.insert(parent, index, g)?;
            for (a, b) in lines {
                let n = line_node(d, polyline(&[*a, *b], false), color, weight, "Hatch line".to_string(), Vec::new());
                d.insert(Some(gid), usize::MAX, n)?;
            }
            groups.push(gid);
        }
        sel.set(groups.iter().copied());
        Ok(groups)
    })?;
    Ok(json!({"groups": groups.iter().map(|g| g.0).collect::<Vec<_>>(), "lines": count}))
}

/// `shape` scaled by `k` about `from` and moved to `to`.
fn moved(shape: &Shape, from: Point, to: Point, k: f64) -> Shape {
    shape.map(|p| to + (p - from) * k)
}

/// `study.compare`.
pub(super) fn compare(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "study.compare";
    let a_id = checked_id(s, p.get("a").ok_or_else(|| bad(C, "missing `a` (the study's id)"))?, C)?;
    let b_id = checked_id(s, p.get("b").ok_or_else(|| bad(C, "missing `b` (the reference's id)"))?, C)?;
    let normalize = match p.get("normalize") {
        None | Some(Value::Null) => true,
        Some(Value::Bool(v)) => *v,
        Some(_) => return Err(bad(C, "`normalize` must be true or false")),
    };
    let res = whole(p, "resolution", 256, 32, 1024, C)? as usize;
    let image = flag(p, "image", C)?;
    let doc = s.doc()?.doc.clone();
    let mut budget = MAX_POINTS;
    let a = shape_of(&doc, a_id, &mut budget).map_err(|e| bad(C, e))?;
    let b = shape_of(&doc, b_id, &mut budget).map_err(|e| bad(C, e))?;
    let (Some(ab), Some(bb)) = (a.bounds(), b.bounds()) else { return Err(bad(C, "both objects need paths")) };
    if ab.height() <= 1e-9 || bb.height() <= 1e-9 || ab.width() <= 1e-9 || bb.width() <= 1e-9 {
        return Err(bad(C, "both objects need an area"));
    }
    let k = if normalize { bb.height() / ab.height() } else { 1.0 };
    let a = if normalize { moved(&a, ab.center(), bb.center(), k) } else { a };
    let grid = a.bounds().map_or(bb, |r| r.union(bb));
    let cell = grid.width().max(grid.height()) / res as f64;
    let (cols, rows) = (((grid.width() / cell).ceil() as usize).clamp(1, res), ((grid.height() / cell).ceil() as usize).clamp(1, res));
    let (mut both, mut only_a, mut only_b) = (0usize, 0usize, 0usize);
    let (mut ca, mut cb) = ((0.0, 0.0, 0usize), (0.0, 0.0, 0usize));
    let mut px = if image { vec![255u8; cols * rows * 4] } else { Vec::new() };
    for row in 0..rows {
        let y = grid.y0 + (row as f64 + 0.5) * cell;
        let (sa, sb) = (scanline(&a, y), scanline(&b, y));
        // The spans are sorted: walk each with one cursor as x grows.
        let (mut ka, mut kb) = (0usize, 0usize);
        for col in 0..cols {
            let x = grid.x0 + (col as f64 + 0.5) * cell;
            let covered = |spans: &[(f64, f64)], k: &mut usize| {
                while spans.get(*k).is_some_and(|s| s.1 <= x) {
                    *k += 1;
                }
                spans.get(*k).is_some_and(|s| x >= s.0 && x < s.1)
            };
            let (ia, ib) = (covered(&sa, &mut ka), covered(&sb, &mut kb));
            if ia {
                ca = (ca.0 + x, ca.1 + y, ca.2 + 1);
            }
            if ib {
                cb = (cb.0 + x, cb.1 + y, cb.2 + 1);
            }
            let colour = match (ia, ib) {
                (true, true) => {
                    both += 1;
                    Some([150, 150, 150])
                }
                (true, false) => {
                    only_a += 1;
                    Some([220, 40, 40])
                }
                (false, true) => {
                    only_b += 1;
                    Some([40, 90, 220])
                }
                _ => None,
            };
            if let (Some(rgb), Some(dst)) = (colour, px.get_mut((row * cols + col) * 4..(row * cols + col) * 4 + 3)) {
                dst.copy_from_slice(&rgb);
            }
        }
    }
    let union = both + only_a + only_b;
    if ca.2 == 0 || cb.2 == 0 || union == 0 {
        return Err(bad(C, "an object covers no area at this resolution"));
    }
    let ratio = |n: usize, d: usize| if d > 0 { r3(n as f64 / d as f64) } else { 0.0 };
    let mut out = json!({
        "iou": ratio(both, union),
        "extra": ratio(only_a, both + only_a),
        "missing": ratio(only_b, both + only_b),
        "aspectA": r3(ab.width() / ab.height()),
        "aspectB": r3(bb.width() / bb.height()),
        "centroidOffset": {"x": r3(ca.0 / ca.2 as f64 - cb.0 / cb.2 as f64), "y": r3(ca.1 / ca.2 as f64 - cb.1 / cb.2 as f64)},
        "scale": r3(k),
        "method": format!("both objects' paths filled by their fill rule and sampled at the centres of a {cols} × {rows} grid; with normalize, a is first scaled by height and centred on b. It measures silhouettes, not likeness: a good drawing can differ from its reference on purpose."),
    });
    if image {
        let png = vansoncraft_render::Rendered { width: cols as u32, height: rows as u32, pixels: px }.to_png().map_err(crate::EngineError::Other)?;
        out["pngBase64"] = json!(vansoncraft_format::base64_encode(&png));
    }
    Ok(out)
}
