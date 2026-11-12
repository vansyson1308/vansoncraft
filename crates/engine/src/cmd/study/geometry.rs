//! Plain polygon geometry for the studies: paths flattened to polylines, the convex hull,
//! simplification within a tolerance, the long axis, and scanline crossings with a fill rule.
//! Everything is bounded: [`MAX_POINTS`] caps how many points one study flattens.

use vectorcraft_doc::{Document, Node, NodeId, NodeKind};
use vectorcraft_geom::{FillRule, PathData, Point, Rect};

/// Most flattened points one study reads; a larger drawing is an error, not a slow reply.
pub(crate) const MAX_POINTS: usize = 400_000;
/// How far (pt) a flattened line may stray from the curve it stands for.
pub(crate) const FLATTEN_TOLERANCE: f64 = 0.1;

/// One flattened subpath.
#[derive(Clone, Debug, Default)]
pub(crate) struct Ring {
    pub points: Vec<Point>,
    pub closed: bool,
}

/// The rings of one path or compound path, and the fill rule they are painted with.
#[derive(Clone, Debug, Default)]
pub(crate) struct Part {
    pub rings: Vec<Ring>,
    pub rule: FillRule,
}

/// The filled area of one object: its paths and compound paths, each with its own fill rule, painted
/// together (their union).
#[derive(Clone, Debug, Default)]
pub(crate) struct Shape {
    pub parts: Vec<Part>,
}

impl Shape {
    pub fn rings(&self) -> impl Iterator<Item = &Ring> {
        self.parts.iter().flat_map(|p| p.rings.iter())
    }
    pub fn point_count(&self) -> usize {
        self.rings().map(|r| r.points.len()).sum()
    }
    pub fn bounds(&self) -> Option<Rect> {
        let mut it = self.rings().flat_map(|r| r.points.iter());
        let first = it.next()?;
        let mut b = Rect::from_points(*first, *first);
        for p in it {
            b = b.union_pt(*p);
        }
        Some(b)
    }
    /// The shape with every point mapped by `f`.
    pub fn map(&self, f: impl Fn(Point) -> Point) -> Shape {
        Shape {
            parts: self
                .parts
                .iter()
                .map(|p| Part {
                    rule: p.rule,
                    rings: p.rings.iter().map(|r| Ring { closed: r.closed, points: r.points.iter().map(|q| f(*q)).collect() }).collect(),
                })
                .collect(),
        }
    }
}

/// The filled area of `id` (itself, or the paths of a group, compound path or layer), skipping
/// guides and hidden objects; each path or compound path keeps its own fill rule. A clip group or
/// clipping layer is refused: what it shows is cut by its clipping path, which a study would have
/// to intersect. `budget` is lowered by the points read; running out is an error.
pub(crate) fn shape_of(d: &Document, id: NodeId, budget: &mut usize) -> Result<Shape, String> {
    let n = d.node(id).ok_or_else(|| format!("no object {}", id.0))?;
    let mut parts = Vec::new();
    collect(n, &mut parts, budget, 0)?;
    Ok(Shape { parts })
}

/// The paths of each object of `ids`, one [`PathItem`] per path (for the line study).
pub(crate) struct PathItem<'a> {
    pub id: NodeId,
    pub name: Option<&'a str>,
    pub path: &'a PathData,
}

/// Every visible, non-guide path at or under `n`, depth first in paint order.
pub(crate) fn paths_in<'a>(n: &'a Node, out: &mut Vec<PathItem<'a>>, depth: usize) {
    if !n.visible || depth > 64 {
        return;
    }
    match &n.kind {
        NodeKind::Path { path, guide: false, .. } => out.push(PathItem { id: n.id, name: n.name.as_deref(), path }),
        _ => {
            for c in n.children().map(Vec::as_slice).unwrap_or_default() {
                paths_in(c, out, depth + 1);
            }
        }
    }
}

fn collect(n: &Node, parts: &mut Vec<Part>, budget: &mut usize, depth: usize) -> Result<(), String> {
    // Nesting deeper than any drawing has is refused rather than followed.
    if depth > 64 {
        return Err("objects nested more than 64 deep".into());
    }
    if !n.visible {
        return Ok(());
    }
    match &n.kind {
        NodeKind::Path { path, rule, guide: false, .. } => {
            let mut rings = Vec::new();
            flatten_into(path, &mut rings, budget)?;
            parts.push(Part { rings, rule: *rule });
            Ok(())
        }
        NodeKind::Path { .. } => Ok(()),
        NodeKind::Compound { children, rule } => {
            let mut rings = Vec::new();
            for c in children.iter().filter(|c| c.visible) {
                if let Some(path) = c.path_data() {
                    flatten_into(path, &mut rings, budget)?;
                }
            }
            parts.push(Part { rings, rule: *rule });
            Ok(())
        }
        NodeKind::Group { clip: true, .. } | NodeKind::Layer { clip: true, .. } => {
            Err(format!("object {} is a clipping group: give the objects inside it", n.id.0))
        }
        _ => {
            for c in n.children().map(Vec::as_slice).unwrap_or_default() {
                collect(c, parts, budget, depth + 1)?;
            }
            Ok(())
        }
    }
}

/// `path` flattened within [`FLATTEN_TOLERANCE`], one ring per subpath with two points or more.
pub(crate) fn flatten_into(path: &PathData, rings: &mut Vec<Ring>, budget: &mut usize) -> Result<(), String> {
    use vectorcraft_geom::PathEl;
    for sp in &path.subpaths {
        if sp.anchors.len() < 2 || sp.anchors.iter().any(|a| !a.p.is_finite() || !a.h_in.is_finite() || !a.h_out.is_finite()) {
            continue;
        }
        let mut bp = vectorcraft_geom::BezPath::new();
        sp.to_bezpath_into(&mut bp);
        let mut pts: Vec<Point> = Vec::new();
        let mut over = false;
        vectorcraft_geom::flatten(bp.elements().iter().copied(), FLATTEN_TOLERANCE, |el| match el {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => {
                if pts.len() >= *budget {
                    over = true;
                } else if pts.last().is_none_or(|q: &Point| q.distance(p) > 1e-9) {
                    pts.push(p);
                }
            }
            _ => {}
        });
        if over {
            return Err(format!("the objects have more than {MAX_POINTS} points once flattened"));
        }
        // A closed ring repeats its first point at the end: drop the repeat.
        if sp.closed && pts.len() > 2 && pts.first().zip(pts.last()).is_some_and(|(a, b)| a.distance(*b) < 1e-9) {
            pts.pop();
        }
        *budget = budget.saturating_sub(pts.len());
        if pts.len() >= 2 {
            rings.push(Ring { points: pts, closed: sp.closed });
        }
    }
    Ok(())
}

/// Twice the signed area of a closed ring (positive counter-clockwise in y-up axes).
pub(crate) fn signed_area2(pts: &[Point]) -> f64 {
    let n = pts.len();
    if n < 3 {
        return 0.0;
    }
    let mut a = 0.0;
    for (i, p) in pts.iter().enumerate() {
        let q = pts.get((i + 1) % n).unwrap_or(p);
        a += p.x * q.y - q.x * p.y;
    }
    a
}

/// The convex hull of `pts` (Andrew's monotone chain), counter-clockwise in y-up axes, without
/// collinear points.
pub(crate) fn convex_hull(pts: &[Point]) -> Vec<Point> {
    let mut v: Vec<Point> = pts.iter().copied().filter(|p| p.is_finite()).collect();
    v.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y)));
    v.dedup_by(|a, b| a.distance(*b) < 1e-9);
    if v.len() < 3 {
        return v;
    }
    let cross = |o: Point, a: Point, b: Point| (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
    let mut lower: Vec<Point> = Vec::new();
    for &p in &v {
        while lower.len() >= 2 && lower.get(lower.len() - 2).zip(lower.last()).is_some_and(|(a, b)| cross(*a, *b, p) <= 0.0) {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper: Vec<Point> = Vec::new();
    for &p in v.iter().rev() {
        while upper.len() >= 2 && upper.get(upper.len() - 2).zip(upper.last()).is_some_and(|(a, b)| cross(*a, *b, p) <= 0.0) {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

/// Distance from `p` to the segment `a`–`b`.
pub(crate) fn seg_distance(p: Point, a: Point, b: Point) -> f64 {
    let ab = b - a;
    let len2 = ab.hypot2();
    if len2 <= 1e-18 {
        return p.distance(a);
    }
    let t = ((p - a).dot(ab) / len2).clamp(0.0, 1.0);
    p.distance(a + ab * t)
}

/// Ramer–Douglas–Peucker on an open polyline: the points kept so that no dropped point lies
/// farther than `tol` from the kept lines. Iterative, so a long ring can't overflow the stack.
pub(crate) fn rdp_open(pts: &[Point], tol: f64) -> Vec<Point> {
    let n = pts.len();
    if n < 3 {
        return pts.to_vec();
    }
    let mut keep = vec![false; n];
    if let Some(k) = keep.first_mut() {
        *k = true;
    }
    if let Some(k) = keep.last_mut() {
        *k = true;
    }
    let mut stack = vec![(0usize, n - 1)];
    while let Some((i, j)) = stack.pop() {
        if j <= i + 1 {
            continue;
        }
        let (Some(&a), Some(&b)) = (pts.get(i), pts.get(j)) else { continue };
        let mut far = (0.0, i);
        for k in i + 1..j {
            if let Some(&p) = pts.get(k) {
                let d = seg_distance(p, a, b);
                if d > far.0 {
                    far = (d, k);
                }
            }
        }
        if far.0 > tol {
            if let Some(k) = keep.get_mut(far.1) {
                *k = true;
            }
            stack.push((i, far.1));
            stack.push((far.1, j));
        }
    }
    pts.iter().zip(keep).filter(|(_, k)| *k).map(|(p, _)| *p).collect()
}

/// [`rdp_open`] for a closed ring: split at the point farthest from the first one.
pub(crate) fn rdp_closed(pts: &[Point], tol: f64) -> Vec<Point> {
    let n = pts.len();
    let Some(&first) = pts.first() else { return vec![] };
    if n < 4 {
        return pts.to_vec();
    }
    let far = (1..n)
        .max_by(|&a, &b| pts.get(a).map_or(0.0, |p| p.distance(first)).total_cmp(&pts.get(b).map_or(0.0, |p| p.distance(first))))
        .unwrap_or(n / 2);
    let mut a: Vec<Point> = pts.get(..=far).map(<[Point]>::to_vec).unwrap_or_default();
    let mut b: Vec<Point> = pts.get(far..).map(<[Point]>::to_vec).unwrap_or_default();
    b.push(first);
    a = rdp_open(&a, tol);
    b = rdp_open(&b, tol);
    // `a` ends where `b` starts, and `b` ends on the first point again.
    a.pop();
    b.pop();
    a.extend(b);
    a
}

/// Visvalingam–Whyatt on a closed ring: drop the point whose triangle with its neighbours is the
/// smallest until at most `max` remain (never fewer than three).
pub(crate) fn reduce_closed(mut pts: Vec<Point>, max: usize) -> Vec<Point> {
    let max = max.max(3);
    while pts.len() > max {
        let n = pts.len();
        let tri = |i: usize| -> f64 {
            let (Some(a), Some(b), Some(c)) = (pts.get((i + n - 1) % n), pts.get(i), pts.get((i + 1) % n)) else { return f64::INFINITY };
            ((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)).abs()
        };
        let Some(smallest) = (0..n).min_by(|&a, &b| tri(a).total_cmp(&tri(b))) else { break };
        pts.remove(smallest);
    }
    pts
}

/// The long axis of a set of points: their centroid and the direction of greatest spread
/// (principal component), with the extent of the points along it.
pub(crate) fn long_axis(pts: &[Point]) -> Option<(Point, Point)> {
    let n = pts.len();
    if n < 2 {
        return None;
    }
    let inv = 1.0 / n as f64;
    let (mx, my) = pts.iter().fold((0.0, 0.0), |(x, y), p| (x + p.x * inv, y + p.y * inv));
    let (mut sxx, mut syy, mut sxy) = (0.0, 0.0, 0.0);
    for p in pts {
        let (dx, dy) = (p.x - mx, p.y - my);
        sxx += dx * dx;
        syy += dy * dy;
        sxy += dx * dy;
    }
    let angle = 0.5 * (2.0 * sxy).atan2(sxx - syy);
    let dir = vectorcraft_geom::Vec2::new(angle.cos(), angle.sin());
    let c = Point::new(mx, my);
    let (lo, hi) = pts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
        let t = (*p - c).dot(dir);
        (lo.min(t), hi.max(t))
    });
    (lo.is_finite() && hi.is_finite() && hi > lo).then(|| (c + dir * lo, c + dir * hi))
}

/// The stretches of the horizontal line at `y` inside `shape`: inside any of its parts by that
/// part's fill rule (open rings closed as a fill would close them), merged and sorted.
pub(crate) fn scanline(shape: &Shape, y: f64) -> Vec<(f64, f64)> {
    let mut spans: Vec<(f64, f64)> = shape.parts.iter().flat_map(|p| part_spans(p, y)).collect();
    if shape.parts.len() < 2 {
        return spans;
    }
    spans.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out: Vec<(f64, f64)> = Vec::with_capacity(spans.len());
    for (x0, x1) in spans {
        match out.last_mut() {
            Some(last) if x0 <= last.1 => last.1 = last.1.max(x1),
            _ => out.push((x0, x1)),
        }
    }
    out
}

/// One part's stretches at `y` by its fill rule.
fn part_spans(part: &Part, y: f64) -> Vec<(f64, f64)> {
    let mut hits: Vec<(f64, i32)> = Vec::new();
    for r in &part.rings {
        let n = r.points.len();
        if n < 3 {
            continue;
        }
        for i in 0..n {
            let (Some(a), Some(b)) = (r.points.get(i), r.points.get((i + 1) % n)) else { continue };
            // Half-open in y, so a vertex on the line is counted once.
            let (up, down) = (a.y <= y && b.y > y, b.y <= y && a.y > y);
            if up || down {
                let t = (y - a.y) / (b.y - a.y);
                hits.push((a.x + t * (b.x - a.x), if up { 1 } else { -1 }));
            }
        }
    }
    hits.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out = Vec::new();
    let mut winding = 0;
    let mut start = 0.0;
    for (x, w) in hits {
        let was = inside(part.rule, winding);
        winding += w;
        let now = inside(part.rule, winding);
        if !was && now {
            start = x;
        } else if was && !now && x > start {
            out.push((start, x));
        }
    }
    out
}

fn inside(rule: FillRule, winding: i32) -> bool {
    match rule {
        FillRule::EvenOdd => winding % 2 != 0,
        _ => winding != 0,
    }
}

/// `shape` turned by `angle` radians about the origin.
pub(crate) fn rotated(shape: &Shape, angle: f64) -> Shape {
    let (s, c) = angle.sin_cos();
    shape.map(|p| Point::new(p.x * c - p.y * s, p.x * s + p.y * c))
}
