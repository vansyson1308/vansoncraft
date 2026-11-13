//! The drawing studies measure what they say and draw editable art, as one undo step, without
//! touching the objects they start from.

use serde_json::{Value, json};
use vansoncraft_doc::{NodeId, NodeKind};

use super::raster::{grey_for, lstar_of_rgb};
use crate::Session;

fn session(w: f64, h: f64) -> Session {
    let mut s = Session::new();
    s.execute("file.new", &json!({"width": w, "height": h})).unwrap();
    s
}

/// A filled shape without a stroke: `shape` is a `shape.*` id.
fn filled(s: &mut Session, shape: &str, x: f64, y: f64, w: f64, h: f64, color: &str) -> u64 {
    let id = s.execute(shape, &json!({"x": x, "y": y, "width": w, "height": h})).unwrap()["id"].as_u64().unwrap();
    s.execute("paint.setFill", &json!({"color": color})).unwrap();
    s.execute("paint.setStroke", &json!({"none": true})).unwrap();
    id
}

fn path(s: &mut Session, d: &str) -> u64 {
    let id = s.execute("path.create", &json!({"d": d})).unwrap()["id"].as_u64().unwrap();
    s.execute("paint.setFill", &json!({"color": "#000000"})).unwrap();
    s.execute("paint.setStroke", &json!({"none": true})).unwrap();
    id
}

fn near(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

fn group<'a>(v: &'a Value, name: &str) -> &'a Value {
    v["groups"].as_array().unwrap().iter().find(|g| g["name"] == name).unwrap()
}

fn json_doc(s: &Session) -> Value {
    serde_json::to_value(&*s.doc().unwrap().doc).unwrap()
}

#[test]
fn lightness_is_cie_lstar() {
    assert!(near(f64::from(lstar_of_rgb(0, 0, 0)), 0.0, 0.01));
    assert!(near(f64::from(lstar_of_rgb(255, 255, 255)), 100.0, 0.01));
    // sRGB 119 is the standard mid grey, L* 50.
    assert!(near(f64::from(lstar_of_rgb(119, 119, 119)), 50.0, 0.3));
    for l in [0.0f32, 20.0, 50.0, 80.0, 100.0] {
        let g = grey_for(l);
        assert!(near(f64::from(lstar_of_rgb(g, g, g)), f64::from(l), 0.5), "{l} → {g}");
    }
}

#[test]
fn values_measure_the_share_and_masses_of_each_group() {
    let mut s = session(100.0, 100.0);
    filled(&mut s, "shape.rectangle", 0.0, 0.0, 50.0, 100.0, "#000000");
    let v = s.execute("study.values", &json!({"maxSide": 100})).unwrap();
    assert!(near(group(&v, "dark")["share"].as_f64().unwrap(), 0.5, 0.02), "{v}");
    assert!(near(group(&v, "light")["share"].as_f64().unwrap(), 0.5, 0.02), "{v}");
    assert_eq!(group(&v, "middle")["share"], 0.0);
    assert_eq!(group(&v, "dark")["masses"], 1);
    assert!(near(v["contrast"].as_f64().unwrap(), 100.0, 1.0), "{v}");
    // A second, separate dark patch is a second mass; a mid grey makes a middle group.
    filled(&mut s, "shape.rectangle", 70.0, 10.0, 20.0, 20.0, "#000000");
    filled(&mut s, "shape.rectangle", 70.0, 60.0, 20.0, 20.0, "#777777");
    let v = s.execute("study.values", &json!({"maxSide": 100, "image": true})).unwrap();
    assert_eq!(group(&v, "dark")["masses"], 2, "{v}");
    assert_eq!(group(&v, "middle")["masses"], 1, "{v}");
    assert!(near(group(&v, "middle")["meanL"].as_f64().unwrap(), 50.0, 1.0), "{v}");
    assert!(v["pngBase64"].as_str().is_some_and(|p| !p.is_empty()));
    // Joining the darks into one shape links the masses.
    filled(&mut s, "shape.rectangle", 45.0, 15.0, 30.0, 10.0, "#000000");
    let v = s.execute("study.values", &json!({"maxSide": 100})).unwrap();
    assert_eq!(group(&v, "dark")["masses"], 1, "{v}");
}

#[test]
fn values_check_their_params() {
    let mut s = session(100.0, 100.0);
    for p in [
        json!({"levels": 1}),
        json!({"levels": 6}),
        json!({"thresholds": [50]}),
        json!({"thresholds": [60, 40]}),
        json!({"thresholds": [0, 50]}),
        json!({"levels": 2, "thresholds": ["a"]}),
        json!({"region": [0, 0, 0, 10]}),
        json!({"region": [0, 0, 10]}),
        json!({"artboard": 3}),
        json!({"maxSide": 4}),
        json!({"minMass": 2}),
    ] {
        assert!(s.execute("study.values", &p).is_err(), "{p} accepted");
    }
    let v = s.execute("study.values", &json!({"levels": 2, "thresholds": [40]})).unwrap();
    assert_eq!(v["thresholds"], json!([40.0]));
}

#[test]
fn render_zooms_into_a_region_and_paints_value_groups() {
    let mut s = session(200.0, 100.0);
    filled(&mut s, "shape.ellipse", 10.0, 10.0, 40.0, 40.0, "#202020");
    let v = s.execute("study.render", &json!({"maxSide": 300})).unwrap();
    assert_eq!((v["width"].as_u64(), v["height"].as_u64()), (Some(300), Some(150)));
    // A 20 pt region at 400 px a side is a 20× zoom.
    let v = s.execute("study.render", &json!({"region": [10, 10, 20, 20], "maxSide": 400, "mode": "values"})).unwrap();
    assert_eq!(v["width"], 400);
    assert!(near(v["scale"].as_f64().unwrap(), 20.0, 1e-6));
    let png = vansoncraft_format::base64_decode(v["pngBase64"].as_str().unwrap()).unwrap();
    let img = image::load_from_memory(&png).unwrap().to_rgb8();
    // Values mode paints two greys only: the ellipse's group and the paper's.
    let mut greys: Vec<u8> = img.pixels().map(|p| p.0[0]).collect();
    greys.sort_unstable();
    greys.dedup();
    assert!(greys.len() <= 3, "{greys:?}");
    assert!(img.pixels().all(|p| p.0[0] == p.0[1] && p.0[1] == p.0[2]));
    assert!(s.execute("study.render", &json!({"mode": "sepia"})).is_err());
    // Turned upside down, the dark disc at the top left is at the bottom left.
    let up = s.execute("study.render", &json!({"maxSide": 200})).unwrap();
    let down = s.execute("study.render", &json!({"maxSide": 200, "flip": "vertical"})).unwrap();
    let px = |v: &Value| image::load_from_memory(&vansoncraft_format::base64_decode(v["pngBase64"].as_str().unwrap()).unwrap()).unwrap().to_rgb8();
    let (up, down) = (px(&up), px(&down));
    assert!(up.get_pixel(30, 30).0[0] < 80 && down.get_pixel(30, 69).0[0] < 80 && down.get_pixel(30, 30).0[0] > 200);
    assert!(s.execute("study.render", &json!({"flip": "sideways"})).is_err());
    assert!(s.execute("study.render", &json!({"scale": 64, "maxSide": 1024})).is_err(), "200 pt at 64× is past 1024 px");
}

#[test]
fn balance_finds_where_the_darks_weigh() {
    let mut s = session(100.0, 100.0);
    let blank = s.execute("study.balance", &json!({})).unwrap();
    assert!(blank["centroid"].is_null());
    assert_eq!(blank["emptyShare"], 1.0);
    filled(&mut s, "shape.rectangle", 0.0, 0.0, 20.0, 20.0, "#000000");
    let v = s.execute("study.balance", &json!({"maxSide": 100})).unwrap();
    assert!(near(v["centroid"]["x"].as_f64().unwrap(), 10.0, 1.0), "{v}");
    assert!(v["offset"]["x"].as_f64().unwrap() < -0.7 && v["offset"]["y"].as_f64().unwrap() < -0.7, "{v}");
    assert!(near(v["quadrants"]["topLeft"].as_f64().unwrap(), 1.0, 0.01), "{v}");
}

#[test]
fn lines_read_direction_straightness_corners_and_inflections() {
    let mut s = session(400.0, 400.0);
    let h = path(&mut s, "M0 100 L200 100");
    let vline = path(&mut s, "M300 0 L300 200");
    let v = s.execute("study.lines", &json!({"ids": [h, vline]})).unwrap();
    let dirs = v["directions"].as_array().unwrap();
    assert!(near(dirs[0]["share"].as_f64().unwrap(), 0.5, 0.01), "{v}");
    assert!(near(dirs[4]["share"].as_f64().unwrap(), 0.5, 0.01), "{v}");
    assert!(near(v["spread"].as_f64().unwrap(), 1.0, 0.01), "perpendicular lines spread every way: {v}");
    assert_eq!(v["straightShare"], 1.0);
    // A rising diagonal reads 45° as seen, although y points down.
    let d = path(&mut s, "M0 200 L100 100");
    let v = s.execute("study.lines", &json!({"ids": [d]})).unwrap();
    assert!(near(v["meanOrientation"].as_f64().unwrap(), 45.0, 0.01), "{v}");
    assert_eq!(v["spread"], 0.0);
    // A rectangle: straight, four corners, no inflection.
    let r = s.execute("shape.rectangle", &json!({"x": 0, "y": 0, "width": 100, "height": 50})).unwrap()["id"].as_u64().unwrap();
    let v = s.execute("study.lines", &json!({"ids": [r]})).unwrap();
    assert_eq!((v["corners"].as_u64(), v["inflections"].as_u64(), v["straightShare"].as_f64()), (Some(4), Some(0), Some(1.0)), "{v}");
    // A circle: curved all round, of one curvature.
    let e = s.execute("shape.ellipse", &json!({"x": 0, "y": 0, "width": 100, "height": 100})).unwrap()["id"].as_u64().unwrap();
    let v = s.execute("study.lines", &json!({"ids": [e]})).unwrap();
    assert_eq!((v["corners"].as_u64(), v["inflections"].as_u64(), v["straightShare"].as_f64()), (Some(0), Some(0), Some(0.0)), "{v}");
    assert!(v["curvatureVariation"].as_f64().unwrap() < 0.1, "{v}");
    // An S curve turns one way, then the other.
    let sc = path(&mut s, "M0 0 C100 0 0 100 100 100");
    let v = s.execute("study.lines", &json!({"ids": [sc]})).unwrap();
    assert_eq!(v["inflections"], 1, "{v}");
    // Groups are searched for their paths.
    s.execute("select.set", &json!({"ids": [h, vline]})).unwrap();
    let g = s.execute("object.group", &json!({})).unwrap();
    let gid = g["id"].as_u64().unwrap_or_else(|| s.doc().unwrap().selection.objects[0].0);
    let v = s.execute("study.lines", &json!({"ids": [gid]})).unwrap();
    assert_eq!(v["paths"], 2, "{v}");
    assert!(s.execute("study.lines", &json!({"ids": [999_999]})).is_err());
}

#[test]
fn proportions_measure_in_units_and_find_plumb_lines() {
    let mut s = session(400.0, 400.0);
    let head = filled(&mut s, "shape.ellipse", 100.0, 0.0, 40.0, 50.0, "#000000");
    let body = filled(&mut s, "shape.rectangle", 100.0, 50.0, 60.0, 300.0, "#000000");
    let v = s.execute("study.proportions", &json!({"ids": [head, body]})).unwrap();
    assert_eq!(v["unit"]["id"], head);
    assert!(near(v["objects"][1]["heightInUnits"].as_f64().unwrap(), 6.0, 0.001), "{v}");
    let lefts: Vec<&Value> = v["alignments"].as_array().unwrap().iter().filter(|a| a["edge"] == "left").collect();
    assert_eq!(lefts.len(), 1, "{v}");
    assert_eq!(lefts[0]["ids"], json!([head, body]));
    // The body's centre is straight below and to the right of the head's: below the level.
    assert!(v["angles"][0]["degrees"].as_f64().unwrap() < -45.0, "{v}");
}

#[test]
fn compare_measures_silhouette_overlap() {
    let mut s = session(400.0, 400.0);
    let a = filled(&mut s, "shape.ellipse", 0.0, 0.0, 100.0, 100.0, "#000000");
    let b = filled(&mut s, "shape.ellipse", 200.0, 200.0, 50.0, 50.0, "#000000");
    let v = s.execute("study.compare", &json!({"a": a, "b": b, "image": true})).unwrap();
    assert!(v["iou"].as_f64().unwrap() > 0.97, "the same shape at another size and place: {v}");
    assert!(v["pngBase64"].is_string());
    let r = filled(&mut s, "shape.rectangle", 200.0, 200.0, 50.0, 50.0, "#000000");
    let v = s.execute("study.compare", &json!({"a": a, "b": r})).unwrap();
    // A circle inside its square covers π/4 of it.
    assert!(near(v["iou"].as_f64().unwrap(), std::f64::consts::FRAC_PI_4, 0.02), "{v}");
    assert!(near(v["missing"].as_f64().unwrap(), 1.0 - std::f64::consts::FRAC_PI_4, 0.02), "{v}");
    let v = s.execute("study.compare", &json!({"a": a, "b": b, "normalize": false})).unwrap();
    assert_eq!(v["iou"], 0.0, "{v}");
    assert!(s.execute("study.compare", &json!({"a": a})).is_err());
}

#[test]
fn block_in_draws_construction_lines_on_a_template_layer_as_one_step() {
    let mut s = session(400.0, 400.0);
    let e = filled(&mut s, "shape.ellipse", 50.0, 50.0, 200.0, 100.0, "#000000");
    let before = s.doc().unwrap().doc.node(NodeId(e)).cloned().unwrap();
    let steps = s.doc().unwrap().history.undo.len();
    let v = s.execute("study.blockIn", &json!({"ids": [e], "maxSides": 8, "axis": true})).unwrap();
    assert_eq!(s.doc().unwrap().history.undo.len(), steps + 1);
    let st = s.doc().unwrap();
    let layer = st.doc.node(NodeId(v["layer"].as_u64().unwrap())).unwrap();
    assert!(matches!(layer.kind, NodeKind::Layer { template: true, printable: false, .. }) && layer.locked);
    assert_eq!(layer.name.as_deref(), Some("Construction"));
    let sides = v["sides"][0].as_u64().unwrap();
    assert!((3..=8).contains(&sides), "{v}");
    // The long axis of a 2:1 ellipse is level.
    assert!(near(v["axes"][0].as_f64().unwrap() % 180.0, 0.0, 1.0) || near(v["axes"][0].as_f64().unwrap(), 180.0, 1.0), "{v}");
    let ring = st.doc.node(NodeId(v["ids"][0].as_u64().unwrap())).unwrap();
    let rb = ring.geometric_bounds().unwrap();
    assert!(near(rb.width(), 200.0, 12.0) && near(rb.height(), 100.0, 12.0), "{rb:?}");
    assert_eq!(st.doc.node(NodeId(e)), Some(&before), "the source is left as it was");
    // Construction lines are left out of exports.
    let values = s.execute("study.values", &json!({"maxSide": 200})).unwrap();
    let with = s.execute("study.values", &json!({"maxSide": 200, "showTemplates": true})).unwrap();
    assert!(group(&with, "dark")["share"].as_f64() >= group(&values, "dark")["share"].as_f64());
    // A second block-in reuses the layer; undo takes the lines away.
    let v2 = s.execute("study.blockIn", &json!({"ids": [e]})).unwrap();
    assert_eq!(v2["layer"], v["layer"]);
    s.execute("edit.undo", &json!({})).unwrap();
    s.execute("edit.undo", &json!({})).unwrap();
    assert!(s.doc().unwrap().doc.node(NodeId(v["layer"].as_u64().unwrap())).is_none());
    // As art, the lines go just above the object and are selected.
    let v = s.execute("study.blockIn", &json!({"ids": [e], "as": "art", "hull": true})).unwrap();
    let id = NodeId(v["ids"][0].as_u64().unwrap());
    let st = s.doc().unwrap();
    assert_eq!(st.doc.parent_of(id), st.doc.parent_of(NodeId(e)));
    assert_eq!(st.doc.position(id).unwrap().1, st.doc.position(NodeId(e)).unwrap().1 + 1);
    assert_eq!(st.selection.objects, vec![id]);
    assert!(s.execute("study.blockIn", &json!({"ids": [e], "as": "paint"})).is_err());
    assert!(s.execute("study.blockIn", &json!({"ids": [e], "stroke": "#11223344"})).is_err());
}

fn hatch_lines(s: &Session, group: u64) -> Vec<vansoncraft_geom::Rect> {
    let st = s.doc().unwrap();
    st.doc.node(NodeId(group)).unwrap().children().unwrap().iter().map(|c| c.geometric_bounds().unwrap()).collect()
}

#[test]
fn hatch_lays_clipped_lines_inside_the_fill_area() {
    let mut s = session(200.0, 200.0);
    let r = filled(&mut s, "shape.rectangle", 0.0, 0.0, 100.0, 100.0, "#ffffff");
    let v = s.execute("study.hatch", &json!({"ids": [r], "spacing": 10, "angle": 0})).unwrap();
    assert_eq!(v["lines"], 10);
    let g = v["groups"][0].as_u64().unwrap();
    for b in hatch_lines(&s, g) {
        assert!(near(b.width(), 100.0, 1e-6) && b.height() < 1e-6 && b.y0 > 0.0 && b.y0 < 100.0, "{b:?}");
    }
    let st = s.doc().unwrap();
    let gn = st.doc.node(NodeId(g)).unwrap();
    let data = &gn.extras().attrs.as_ref().unwrap().data;
    assert!(data.contains(&("study".into(), "hatch".into())) && data.contains(&("spacing".into(), "10".into())), "{data:?}");
    assert_eq!(st.doc.position(NodeId(g)).unwrap().1, st.doc.position(NodeId(r)).unwrap().1 + 1, "above its shape");
    // At an angle the lines still lie inside the shape's bounds exactly (no 1e-14 overshoot).
    let v = s.execute("study.hatch", &json!({"ids": [r], "spacing": 7, "angle": 62})).unwrap();
    for b in hatch_lines(&s, v["groups"][0].as_u64().unwrap()) {
        assert!(b.x0 >= 0.0 && b.y0 >= 0.0 && b.x1 <= 100.0 && b.y1 <= 100.0, "{b:?}");
    }
    // A minimum length leaves out the short lines near a circle's top and bottom.
    let e = filled(&mut s, "shape.ellipse", 120.0, 0.0, 60.0, 60.0, "#ffffff");
    let all = s.execute("study.hatch", &json!({"ids": [e], "spacing": 2, "angle": 0})).unwrap()["lines"].as_u64().unwrap();
    let long = s.execute("study.hatch", &json!({"ids": [e], "spacing": 2, "angle": 0, "minLength": 20})).unwrap();
    assert!(long["lines"].as_u64().unwrap() < all);
    assert!(hatch_lines(&s, long["groups"][0].as_u64().unwrap()).iter().all(|b| b.width() > 20.0));
    // Crossed, the lines go both ways.
    let v = s.execute("study.hatch", &json!({"ids": [r], "spacing": 10, "angle": 0, "cross": true})).unwrap();
    assert_eq!(v["lines"], 20);
}

#[test]
fn hatch_follows_the_fill_rule_around_holes() {
    let mut s = session(200.0, 200.0);
    // The inner square runs the other way: a hole under the non-zero rule.
    let ring = path(&mut s, "M0 0 H100 V100 H0 Z M25 25 V75 H75 V25 Z");
    let v = s.execute("study.hatch", &json!({"ids": [ring], "spacing": 10, "angle": 0})).unwrap();
    let total: f64 = hatch_lines(&s, v["groups"][0].as_u64().unwrap()).iter().map(|b| b.width()).sum();
    assert!(near(total, 10.0 * 100.0 - 5.0 * 50.0, 1e-6), "{total}");
    assert_eq!(v["lines"], 15, "rows through the hole are split in two");
}

#[test]
fn hatch_fills_both_lobes_of_a_figure_eight() {
    let mut s = session(200.0, 200.0);
    // The lobes wind opposite ways: their signed areas cancel, yet both are filled.
    let eight = path(&mut s, "M0 50 C0 0 100 100 100 50 C100 0 0 100 0 50 Z");
    let v = s.execute("study.hatch", &json!({"ids": [eight], "spacing": 5, "angle": 0})).unwrap();
    let lines = hatch_lines(&s, v["groups"][0].as_u64().unwrap());
    assert!(lines.iter().any(|b| b.x1 < 50.0) && lines.iter().any(|b| b.x0 > 50.0), "{lines:?}");
}

#[test]
fn hatch_refuses_too_many_lines_and_changes_nothing() {
    let mut s = session(200.0, 200.0);
    let r = filled(&mut s, "shape.rectangle", 0.0, 0.0, 100.0, 100.0, "#ffffff");
    let before = json_doc(&s);
    let steps = s.doc().unwrap().history.undo.len();
    assert!(s.execute("study.hatch", &json!({"ids": [r], "spacing": 1, "maxLines": 50})).is_err());
    assert_eq!(json_doc(&s), before);
    assert_eq!(s.doc().unwrap().history.undo.len(), steps);
    for p in [json!({"spacing": 0}), json!({"jitter": 0.9}), json!({"maxLines": 50_000}), json!({"color": "nope"})] {
        let mut p = p;
        p["ids"] = json!([r]);
        assert!(s.execute("study.hatch", &p).is_err(), "{p} accepted");
    }
    // An open line has no area.
    let l = s.execute("shape.line", &json!({"x1": 0, "y1": 150, "x2": 100, "y2": 150})).unwrap()["id"].as_u64().unwrap();
    assert!(s.execute("study.hatch", &json!({"ids": [l]})).is_err());
}

#[test]
fn hatch_jitter_is_reproducible_from_its_seed() {
    let make = |seed: u64| {
        let mut s = session(200.0, 200.0);
        let r = filled(&mut s, "shape.ellipse", 0.0, 0.0, 100.0, 100.0, "#ffffff");
        let v = s.execute("study.hatch", &json!({"ids": [r], "spacing": 5, "angle": 30, "jitter": 0.3, "seed": seed})).unwrap();
        hatch_lines(&s, v["groups"][0].as_u64().unwrap())
    };
    assert_eq!(make(7), make(7));
    assert_ne!(make(7), make(8));
}

#[test]
fn studies_survive_a_native_round_trip() {
    let mut s = session(200.0, 200.0);
    let r = filled(&mut s, "shape.ellipse", 0.0, 0.0, 100.0, 100.0, "#ffffff");
    s.execute("study.hatch", &json!({"ids": [r], "spacing": 8, "seed": 3, "jitter": 0.2})).unwrap();
    s.execute("study.blockIn", &json!({"ids": [r]})).unwrap();
    let doc = s.doc().unwrap().doc.clone();
    let bytes = vansoncraft_format::save_file(&doc);
    let back = vansoncraft_format::load(&bytes).unwrap();
    assert_eq!(serde_json::to_value(&back).unwrap(), serde_json::to_value(&*doc).unwrap());
}

#[test]
fn queries_change_nothing() {
    let mut s = session(200.0, 200.0);
    let a = filled(&mut s, "shape.ellipse", 0.0, 0.0, 100.0, 100.0, "#333333");
    let b = filled(&mut s, "shape.rectangle", 120.0, 0.0, 50.0, 100.0, "#999999");
    let before = json_doc(&s);
    let rev = s.doc().unwrap().history.undo.len();
    for (c, p) in [
        ("study.render", json!({"mode": "gray"})),
        ("study.values", json!({})),
        ("study.balance", json!({})),
        ("study.lines", json!({"ids": [a, b]})),
        ("study.proportions", json!({"ids": [a, b]})),
        ("study.compare", json!({"a": a, "b": b})),
    ] {
        s.execute(c, &p).unwrap();
        assert_eq!(json_doc(&s), before, "{c}");
        assert_eq!(s.doc().unwrap().history.undo.len(), rev, "{c}");
    }
}

#[test]
fn a_curve_drawn_with_points_reads_as_a_curve() {
    let mut s = session(400.0, 400.0);
    // A 64-sided polygon standing for a circle.
    let pts: Vec<String> = (0..64)
        .map(|k| {
            let a = k as f64 * std::f64::consts::TAU / 64.0;
            format!("{:.4} {:.4}", 200.0 + 100.0 * a.cos(), 200.0 + 100.0 * a.sin())
        })
        .collect();
    let poly = path(&mut s, &format!("M{} Z", pts.join(" L")));
    let v = s.execute("study.lines", &json!({"ids": [poly]})).unwrap();
    assert!(v["straightShare"].as_f64().unwrap() < 0.05, "{v}");
    assert_eq!((v["corners"].as_u64(), v["inflections"].as_u64()), (Some(0), Some(0)), "{v}");
    assert!(v["curvatureVariation"].as_f64().unwrap() < 0.1, "{v}");
    // A gentle zig-zag bends one way, then the other, at every joint.
    let zig = path(&mut s, "M0 300 L40 296 L80 300 L120 296 L160 300");
    let v = s.execute("study.lines", &json!({"ids": [zig]})).unwrap();
    assert_eq!(v["inflections"], 2, "{v}");
    // A 16-sided block-in turns 22.5° at each joint: straight lines meeting at angles, not a curve.
    let sixteen: Vec<String> = (0..16)
        .map(|k| {
            let a = k as f64 * std::f64::consts::TAU / 16.0;
            format!("{:.4} {:.4}", 200.0 + 100.0 * a.cos(), 200.0 + 100.0 * a.sin())
        })
        .collect();
    let block = path(&mut s, &format!("M{} Z", sixteen.join(" L")));
    let v = s.execute("study.lines", &json!({"ids": [block]})).unwrap();
    assert_eq!((v["straightShare"].as_f64(), v["corners"].as_u64()), (Some(1.0), Some(0)), "{v}");
    // A rectangle drawn with points is still four straight sides and four corners.
    let rect = path(&mut s, "M10 10 L110 10 L110 60 L10 60 Z");
    let v = s.execute("study.lines", &json!({"ids": [rect]})).unwrap();
    assert_eq!((v["corners"].as_u64(), v["straightShare"].as_f64()), (Some(4), Some(1.0)), "{v}");
}

#[test]
fn a_groups_members_keep_their_own_fill_rules_and_clip_groups_are_refused() {
    let mut s = session(300.0, 300.0);
    // Two rings, each with a hole by its own even-odd rule; a group of them.
    let a = path(&mut s, "M0 0 H100 V100 H0 Z M25 25 H75 V75 H25 Z");
    let b = path(&mut s, "M150 0 H250 V100 H150 Z M175 25 H225 V75 H175 Z");
    for id in [a, b] {
        s.execute("path.setFillRule", &json!({"ids": [id], "rule": "evenodd"})).ok();
        let mut d = (*s.doc().unwrap().doc).clone();
        if let Some(NodeKind::Path { rule, .. }) = d.node_mut(NodeId(id)).map(|n| &mut n.kind) {
            *rule = vansoncraft_geom::FillRule::EvenOdd;
        }
        s.doc_mut().unwrap().doc = std::sync::Arc::new(d);
    }
    let g = s.execute("object.group", &json!({"ids": [a, b]})).unwrap()["id"].as_u64().unwrap();
    let v = s.execute("study.hatch", &json!({"ids": [g], "spacing": 10, "angle": 0})).unwrap();
    let total: f64 = hatch_lines(&s, v["groups"][0].as_u64().unwrap()).iter().map(|r| r.width()).sum();
    // Each ring is 10 rows × 100 pt less its hole's 5 rows × 50 pt.
    assert!(near(total, 2.0 * (1000.0 - 250.0), 1e-6), "{total}");
    let mask = s.execute("shape.ellipse", &json!({"x": 0, "y": 0, "width": 60, "height": 60})).unwrap()["id"].as_u64().unwrap();
    let r = s.execute("shape.rectangle", &json!({"x": 0, "y": 150, "width": 60, "height": 60})).unwrap()["id"].as_u64().unwrap();
    let clip = s.execute("object.clippingMask.make", &json!({"ids": [mask, r]})).unwrap();
    let clip_id = clip["id"].as_u64().unwrap_or_else(|| s.doc().unwrap().selection.objects[0].0);
    let err = s.execute("study.hatch", &json!({"ids": [clip_id]})).unwrap_err().to_string();
    assert!(err.contains("clipping group"), "{err}");
}

#[test]
fn balance_can_weigh_contrast_on_a_toned_ground() {
    let mut s = session(100.0, 100.0);
    filled(&mut s, "shape.rectangle", 0.0, 0.0, 100.0, 100.0, "#777777");
    filled(&mut s, "shape.rectangle", 70.0, 40.0, 20.0, 20.0, "#ffffff");
    let dark = s.execute("study.balance", &json!({"maxSide": 100})).unwrap();
    let contrast = s.execute("study.balance", &json!({"maxSide": 100, "weigh": "contrast"})).unwrap();
    // By darkness the grey ground weighs almost everywhere; by contrast the light square does.
    assert!(dark["offset"]["x"].as_f64().unwrap().abs() < 0.2, "{dark}");
    assert!(contrast["offset"]["x"].as_f64().unwrap() > 0.5, "{contrast}");
    assert!(near(contrast["ground"]["L"].as_f64().unwrap(), 50.0, 1.0), "{contrast}");
    assert!(s.execute("study.balance", &json!({"weigh": "mass"})).is_err());
    let v = s.execute("study.values", &json!({"maxSide": 100})).unwrap();
    assert_eq!(v["extremes"]["lightL"], 100.0, "{v}");
    assert!(near(v["extremes"]["darkL"].as_f64().unwrap(), 50.0, 1.0), "{v}");
}

#[test]
fn block_in_reports_how_far_the_outline_strays() {
    let mut s = session(400.0, 400.0);
    let e = filled(&mut s, "shape.ellipse", 0.0, 0.0, 200.0, 200.0, "#000000");
    let few = s.execute("study.blockIn", &json!({"ids": [e], "maxSides": 4})).unwrap();
    let many = s.execute("study.blockIn", &json!({"ids": [e], "maxSides": 24, "tolerance": 1})).unwrap();
    let (f, m) = (few["maxDeviation"][0].as_f64().unwrap(), many["maxDeviation"][0].as_f64().unwrap());
    // Four lines with their corners on a 100 pt circle stray at least as far as a square's do,
    // 100 − 70.7 at the arcs' middles.
    assert!(f >= 100.0 - 100.0 / 2f64.sqrt() - 0.5 && f < 60.0, "{few}");
    assert!(m < 2.0 && m < f, "{many}");
}
