//! The audit fixes of docs/speed/audit.md over MCP (F1 backend shown, F2 no id reuse after undo,
//! F3 atomic batches roll back, F6/F7 previews scale up and zoom), and the drawing studies reached
//! through `command_run` and `render_preview`.

use serde_json::{Value, json};

use crate::{Headless, Server};

fn call(s: &mut Server, name: &str, args: Value) -> Value {
    super::call(s, 1, name, args)
}

/// The first text block of a result as JSON.
fn payload(r: &Value) -> Value {
    let text = r["content"].as_array().unwrap().iter().find(|c| c["type"] == "text").unwrap()["text"].as_str().unwrap();
    serde_json::from_str(text).unwrap()
}

fn image(r: &Value) -> image::RgbaImage {
    let data = r["content"].as_array().unwrap().iter().find(|c| c["type"] == "image").unwrap()["data"].as_str().unwrap();
    image::load_from_memory(&vansoncraft_format::base64_decode(data).unwrap()).unwrap().to_rgba8()
}

#[test]
fn doc_inspect_says_which_backend_the_tools_act_in() {
    let mut s = Server::new(Box::new(Headless::with_document()));
    let p = payload(&call(&mut s, "doc_inspect", json!({"depth": 0})));
    assert_eq!(p["backend"]["kind"], "headless");
    assert_eq!(p["backend"]["desktopWindow"], false);
    assert!(p["uid"].is_u64());
}

#[test]
fn a_server_that_fell_back_says_so_once() {
    let mut s = Server::new(Box::new(Headless::with_document())).fell_back_from("127.0.0.1:7979");
    let init = super::rpc(
        &mut s,
        1,
        "initialize",
        json!({"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "t", "version": "1"}}),
    );
    assert!(init["result"]["instructions"].as_str().unwrap().contains("runs headless"), "{init}");
    let first = call(&mut s, "draw_shape", json!({"shape": "rectangle", "x": 0, "y": 0, "width": 10, "height": 10}));
    let blocks = first["content"].as_array().unwrap();
    assert_eq!(blocks.len(), 2, "{first}");
    assert!(blocks[1]["text"].as_str().unwrap().contains("no VansonCraft app answered on 127.0.0.1:7979"));
    // The result itself is still the first block, as JSON.
    assert!(payload(&first)["id"].is_u64());
    let second = call(&mut s, "doc_inspect", json!({"depth": 0}));
    assert_eq!(second["content"].as_array().unwrap().len(), 1, "said once: {second}");
}

#[test]
fn a_stale_id_after_undo_fails_instead_of_painting_a_new_object() {
    let mut s = Server::new(Box::new(Headless::with_document()));
    let rect = payload(&call(&mut s, "draw_shape", json!({"shape": "rectangle", "x": 10, "y": 10, "width": 100, "height": 50})))["id"].clone();
    call(&mut s, "undo", json!({}));
    let ellipse = payload(&call(&mut s, "draw_shape", json!({"shape": "ellipse", "x": 300, "y": 300, "width": 40, "height": 40})))["id"].clone();
    assert_ne!(rect, ellipse);
    let r = call(&mut s, "set_paint", json!({"ids": [rect], "fill": "#ff0000"}));
    assert_eq!(r["isError"], true, "{r}");
}

#[test]
fn a_failed_atomic_batch_leaves_the_document_as_it_was() {
    let mut s = Server::new(Box::new(Headless::with_document()));
    call(&mut s, "draw_shape", json!({"shape": "rectangle", "x": 0, "y": 0, "width": 10, "height": 10}));
    let before = payload(&call(&mut s, "doc_inspect", json!({})));
    let r = call(
        &mut s,
        "command_batch",
        json!({"atomic": true, "label": "Job", "steps": [
            {"id": "draw_shape", "params": {"shape": "ellipse", "x": 50, "y": 50, "width": 20, "height": 20}},
            {"id": "shape.rectangle", "params": {"x": 10, "y": 10, "width": 50, "height": 50}},
            {"id": "no.such.command"}
        ]}),
    );
    assert_eq!(r["isError"], true);
    let p = payload(&r);
    assert_eq!((p["completed"].clone(), p["rolledBack"].clone(), p["created"].clone()), (json!(2), json!(true), json!([])), "{r}");
    let after = payload(&call(&mut s, "doc_inspect", json!({})));
    assert_eq!(after["layers"], before["layers"]);
    assert_eq!(after["history"], before["history"]);
    // Kept on request.
    let r = call(
        &mut s,
        "command_batch",
        json!({"atomic": true, "keep_on_error": true, "steps": [{"id": "shape.rectangle", "params": {"x": 10, "y": 10, "width": 50, "height": 50}}, {"id": "no.such.command"}]}),
    );
    assert!(payload(&r).get("rolledBack").is_none(), "{r}");
    assert_ne!(payload(&call(&mut s, "doc_inspect", json!({})))["layers"], before["layers"]);
    assert_eq!(call(&mut s, "command_batch", json!({"keep_on_error": true, "steps": []}))["isError"], true);
}

#[test]
fn a_small_artboard_previews_large_enough_to_judge() {
    let mut s = Server::new(Box::new(Headless::with_document()));
    call(&mut s, "command_run", json!({"id": "file.new", "params": {"width": 32, "height": 32}}));
    assert_eq!(image(&call(&mut s, "render_preview", json!({}))).dimensions(), (512, 512));
    // max_side bounds it: never past 512 pixels or 16 pixels a point, smaller when asked.
    assert_eq!(image(&call(&mut s, "render_preview", json!({"max_side": 2048}))).dimensions(), (512, 512));
    assert_eq!(image(&call(&mut s, "render_preview", json!({"max_side": 256}))).dimensions(), (256, 256));
    // A page keeps its usual size, also with a larger max_side (image tokens grow with pixels).
    call(&mut s, "command_run", json!({"id": "file.new", "params": {"width": 612, "height": 792}}));
    assert_eq!(image(&call(&mut s, "render_preview", json!({}))).dimensions(), (612, 792));
    assert_eq!(image(&call(&mut s, "render_preview", json!({"max_side": 2048}))).dimensions(), (612, 792));
    // An A4 page as PDFs write it (841.89 pt) previews in values mode without a rounding refusal.
    call(&mut s, "command_run", json!({"id": "file.new", "params": {"width": 595.28, "height": 841.89}}));
    assert_eq!(call(&mut s, "render_preview", json!({"mode": "values"}))["isError"], false);
    assert_eq!(call(&mut s, "command_run", json!({"id": "study.values", "params": {"maxSide": 1000}}))["isError"], false);
}

#[test]
fn render_preview_zooms_into_a_region_and_shows_value_groups() {
    let mut s = Server::new(Box::new(Headless::with_document()));
    call(&mut s, "draw_shape", json!({"shape": "ellipse", "x": 100, "y": 100, "width": 60, "height": 60, "fill": "#303030", "stroke": "none"}));
    let r = call(&mut s, "render_preview", json!({"region": [100, 100, 30, 30], "max_side": 300}));
    assert_eq!(r["isError"], false, "{r}");
    let img = image(&r);
    assert_eq!(img.dimensions(), (300, 300));
    let info: Value = serde_json::from_str(r["content"][1]["text"].as_str().unwrap()).unwrap();
    assert_eq!(info["scale"], 10.0);
    // The region's lower right lies inside the disc.
    let p = img.get_pixel(290, 290);
    assert!(p[0] < 80, "{p:?}");
    let r = call(&mut s, "render_preview", json!({"mode": "values", "levels": 2}));
    let img = image(&r);
    let mut greys: Vec<u8> = img.pixels().map(|p| p[0]).collect();
    greys.sort_unstable();
    greys.dedup();
    assert!(greys.len() <= 2, "two value groups paint two greys: {greys:?}");
    assert_eq!(call(&mut s, "render_preview", json!({"mode": "sepia"}))["isError"], true);
    assert_eq!(call(&mut s, "render_preview", json!({"region": [0, 0, -5, 5]}))["isError"], true);
}

#[test]
fn the_studies_run_through_command_run() {
    let mut s = Server::new(Box::new(Headless::with_document()));
    let id = payload(&call(
        &mut s,
        "draw_shape",
        json!({"shape": "rectangle", "x": 0, "y": 0, "width": 306, "height": 792, "fill": "#000000", "stroke": "none"}),
    ))["id"]
        .clone();
    let v = payload(&call(&mut s, "command_run", json!({"id": "study.values", "params": {}})));
    let dark = v["groups"].as_array().unwrap().iter().find(|g| g["name"] == "dark").unwrap();
    assert!((dark["share"].as_f64().unwrap() - 0.5).abs() < 0.02, "{v}");
    let b = payload(&call(&mut s, "command_run", json!({"id": "study.balance", "params": {}})));
    assert!(b["offset"]["x"].as_f64().unwrap() < -0.4, "{b}");
    let h =
        payload(&call(&mut s, "command_run", json!({"id": "study.hatch", "params": {"ids": [id], "spacing": 20, "angle": 0, "color": "#ffffff"}})));
    assert!(h["lines"].as_u64().unwrap() > 30, "{h}");
    let undone = payload(&call(&mut s, "undo", json!({})));
    assert_eq!(undone["undone"], "Hatch", "{undone}");
}

/// Every node of a `doc_inspect` reply: id → (name, bounds, fill, stroke, strokeWidth).
fn nodes(doc: &Value) -> std::collections::BTreeMap<u64, Value> {
    let mut out = std::collections::BTreeMap::new();
    let mut stack: Vec<&Value> = doc["layers"].as_array().unwrap().iter().collect();
    while let Some(n) = stack.pop() {
        out.insert(n["id"].as_u64().unwrap(), json!([n["name"], n["bounds"], n["fill"], n["stroke"], n["strokeWidth"]]));
        stack.extend(n["children"].as_array().into_iter().flatten());
    }
    out
}

/// Acceptance scenarios A and D in miniature, through MCP only: a vase of flowers built as named,
/// grouped vector parts, measured, one flower changed without touching the rest, saved, reopened
/// and exported as SVG with its structure.
#[test]
fn a_vase_of_flowers_stays_editable_part_by_part() {
    let mut s = Server::new(Box::new(Headless::with_document()));
    call(&mut s, "command_run", json!({"id": "file.new", "params": {"width": 400, "height": 520}}));
    let petal = |cx: f64, cy: f64, a: f64| {
        let (dx, dy) = (a.cos(), a.sin());
        let (px, py) = (-dy, dx);
        let p = |u: f64, v: f64| format!("{:.2} {:.2}", cx + u * dx + v * px, cy + u * dy + v * py);
        format!("M{} C{} {} {} C{} {} {} Z", p(3.0, 0.0), p(12.0, 18.0), p(30.0, 15.0), p(36.0, 0.0), p(30.0, -15.0), p(12.0, -18.0), p(3.0, 0.0))
    };
    let mut steps = vec![
        json!({"id": "layer.new", "params": {"name": "Vase of flowers"}}),
        json!({"id": "draw_shape", "params": {"shape": "rectangle", "x": 0, "y": 0, "width": 400, "height": 520, "fill": "#8c8a74", "stroke": "none", "name": "Ground"}}),
        json!({"id": "draw_path", "params": {"d": "M180 300 C180 330 130 350 130 400 C130 450 160 470 170 480 L230 480 C240 470 272 450 272 400 C272 350 220 330 220 300 Z", "fill": "#2e3b39", "stroke": "none", "name": "Vase body"}}),
    ];
    for (n, (cx, cy)) in [(150.0, 220.0), (250.0, 200.0)].into_iter().enumerate() {
        let first = steps.len();
        for k in 0..5 {
            steps.push(json!({"id": "draw_path", "params": {"d": petal(cx, cy, k as f64 * 1.2566), "fill": "#efe6d2", "stroke": "none", "name": format!("Flower {} petal {}", n + 1, k + 1)}}));
        }
        steps.push(json!({"id": "draw_shape", "params": {"shape": "ellipse", "x": cx - 9.0, "y": cy - 9.0, "width": 18, "height": 18, "fill": "#b8892f", "stroke": "none", "name": format!("Flower {} centre", n + 1)}}));
        let ids: Vec<String> = (first..steps.len()).map(|i| format!("${i}.id")).collect();
        steps.push(json!({"id": "object.group", "params": {"ids": ids}}));
        let g = steps.len() - 1;
        steps.push(json!({"id": "object.setProps", "params": {"id": format!("${g}.id"), "name": format!("Flower {}", n + 1)}}));
    }
    let r = call(&mut s, "command_batch", json!({"atomic": true, "label": "Vase", "steps": steps}));
    assert_eq!(r["isError"], false, "{r}");
    // Three value groups: the dark vase, the middle ground, the light petals.
    let v = payload(&call(&mut s, "command_run", json!({"id": "study.values", "params": {}})));
    for g in v["groups"].as_array().unwrap() {
        assert!(g["share"].as_f64().unwrap() > 0.01, "every group is present: {v}");
    }
    // Change flower 1's petals only.
    let found = payload(&call(&mut s, "command_run", json!({"id": "document.find", "params": {"name": "Flower 1 petal"}})));
    let petals: Vec<u64> = found["matches"].as_array().unwrap().iter().map(|m| m["id"].as_u64().unwrap()).collect();
    assert_eq!(petals.len(), 5);
    let before = nodes(&payload(&call(&mut s, "doc_inspect", json!({}))));
    assert_eq!(call(&mut s, "set_paint", json!({"ids": petals, "fill": "#fbf7ef", "stroke": "#2a2116", "strokeWidth": 1.2}))["isError"], false);
    let after = nodes(&payload(&call(&mut s, "doc_inspect", json!({}))));
    assert_eq!(before.keys().collect::<Vec<_>>(), after.keys().collect::<Vec<_>>(), "no object added or removed");
    let changed: Vec<u64> = before.keys().filter(|id| before[id] != after[id]).copied().collect();
    // The petals, and the group and layer around them, whose visual bounds grow by the stroke.
    assert!(petals.iter().all(|p| changed.contains(p)), "{changed:?}");
    let containers: Vec<&u64> = changed.iter().filter(|id| !petals.contains(id)).collect();
    assert!(containers.len() <= 2, "only flower 1's group and its layer may change around the petals: {containers:?}");
    // Native round trip, then SVG with the parts' names.
    let path = super::tmp("vase.vansoncraft");
    let doc_before = payload(&call(&mut s, "command_run", json!({"id": "document.json", "params": {}})));
    assert_eq!(call(&mut s, "save_file", json!({"path": path.to_str().unwrap()}))["isError"], false);
    assert_eq!(call(&mut s, "open_file", json!({"path": path.to_str().unwrap()}))["isError"], false);
    let mut doc_after = payload(&call(&mut s, "command_run", json!({"id": "document.json", "params": {}})));
    // Saving stamps the modification time; everything else reads back as it was.
    doc_after["metadata"]["modified"] = doc_before["metadata"]["modified"].clone();
    assert_eq!(doc_after, doc_before);
    let svg = super::tmp("vase.svg");
    assert_eq!(call(&mut s, "export", json!({"path": svg.to_str().unwrap()}))["isError"], false);
    let text = std::fs::read_to_string(&svg).unwrap();
    assert!(
        text.contains("data-name=\"Flower 1\"") && text.contains("data-name=\"Vase body\"") && !text.contains("<image"),
        "named vector parts, no raster stand-in"
    );
}

#[test]
fn a_call_meant_for_another_document_runs_nothing() {
    let mut s = Server::new(Box::new(Headless::with_document()));
    let first = payload(&call(&mut s, "doc_inspect", json!({"depth": 0})))["uid"].clone();
    call(&mut s, "command_run", json!({"id": "file.new", "params": {"width": 100, "height": 100}}));
    let r =
        call(&mut s, "command_run", json!({"id": "shape.rectangle", "params": {"x": 0, "y": 0, "width": 5, "height": 5}, "expect_document": first}));
    assert_eq!(r["isError"], true, "{r}");
    let r = call(
        &mut s,
        "command_batch",
        json!({"expect_document": first, "steps": [{"id": "shape.ellipse", "params": {"x": 0, "y": 0, "width": 5, "height": 5}}]}),
    );
    assert_eq!(r["isError"], true, "{r}");
    let now = payload(&call(&mut s, "doc_inspect", json!({"depth": 0})));
    assert_eq!(now["layers"][0]["childCount"], 0, "nothing was drawn in the active document: {now}");
    let r = call(
        &mut s,
        "command_run",
        json!({"id": "shape.rectangle", "params": {"x": 0, "y": 0, "width": 5, "height": 5}, "expect_document": now["uid"]}),
    );
    assert_eq!(r["isError"], false, "{r}");
}

#[test]
fn pictures_a_command_returns_come_back_as_images() {
    let mut s = Server::new(Box::new(Headless::with_document()));
    let a = payload(&call(&mut s, "draw_shape", json!({"shape": "ellipse", "x": 0, "y": 0, "width": 100, "height": 100, "fill": "#000000"})))["id"]
        .clone();
    let b = payload(&call(&mut s, "draw_shape", json!({"shape": "rectangle", "x": 200, "y": 0, "width": 80, "height": 80, "fill": "#000000"})))["id"]
        .clone();
    for (id, params) in [("study.compare", json!({"a": a, "b": b, "image": true})), ("study.render", json!({"mode": "values", "flip": "both"}))] {
        let r = call(&mut s, "command_run", json!({"id": id, "params": params}));
        let blocks = r["content"].as_array().unwrap();
        assert!(blocks.iter().any(|c| c["type"] == "image" && c["mimeType"] == "image/png"), "{id}: {r}");
        let text = blocks.iter().find(|c| c["type"] == "text").unwrap()["text"].as_str().unwrap();
        assert!(!text.contains("pngBase64"), "{id}: the base64 is not repeated as text");
    }
    // render_preview takes flip as well.
    assert!(image(&call(&mut s, "render_preview", json!({"flip": "horizontal"}))).width() > 0);
}
