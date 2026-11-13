use super::definitions::{INSPECT_KEYS, SAVE_FILE_OPTIONS, tool_definitions};
use super::handlers::{
    Args, absolute_path, add_text, answer_dialog, apply_paint, draw_path, draw_shape, exec, image_result, max_side, need_ui, req_str, screenshot,
    select_ids, transform,
};
use super::{ARTBOARD_PARAMS, Backend, Map, ToolResult, Value, json, writable_format};
use crate::backend::Pinned;

/// The `menu` argument of the command listings.
pub(super) const MENU_FILTER: &str =
    "Case-insensitive substring of the menu path, its levels joined with \" > \" (e.g. \"Object\", \"Object > Path\")";

/// The `brief` argument of the command listings.
pub(super) const BRIEF: &str = "Leave out each command's params description and its schema and inputSchema (the bulk of the catalogue); run command_list with the id as filter to read them";

/// The `max_side` argument of the window captures.
pub(super) const MAX_SIDE_WINDOW: &str =
    "Window capture only: longest side of the returned image in pixels; a larger capture is scaled down to it (default: the full capture)";

/// The `traceViews` argument of the artboard renders.
pub(super) const TRACE_VIEWS: &str = "Draw Image Trace objects in the view set with imageTrace.setView (outlines, the source image…), as the app's window does; by default they draw their tracing result, as exports do";

/// `traceViews`: false when absent.
pub(super) fn trace_views(a: &Args) -> Result<bool, String> {
    match a.get("traceViews") {
        None => Ok(false),
        Some(v) => v.as_bool().ok_or_else(|| "`traceViews` must be a boolean".into()),
    }
}

/// The command catalogue narrowed by the listing tools' arguments: `filter` (or `query`) on the
/// id, label and menu path, `menu` on the menu path, `enabledOnly`, and `brief` (no `params`
/// text, `schema` or `inputSchema`: the full catalogue is too large for many clients, #2173).
fn filter_commands(all: Value, a: &Args) -> Value {
    let text = |k: &str| a.get(k).and_then(Value::as_str).map(str::to_lowercase).filter(|s| !s.is_empty());
    let needle = text("filter").or_else(|| text("query"));
    let in_menu = text("menu");
    let enabled_only = a.get("enabledOnly").and_then(Value::as_bool).unwrap_or(false);
    let brief = a.get("brief").and_then(Value::as_bool).unwrap_or(false);
    let Value::Array(list) = all else { return all };
    let menu_path = |c: &Value, sep: &str| {
        c.get("menu")
            .and_then(Value::as_array)
            .map(|m| m.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(sep))
            .unwrap_or_default()
            .to_lowercase()
    };
    let hit = |c: &Value| {
        if enabled_only && c.get("enabled").and_then(Value::as_bool) == Some(false) {
            return false;
        }
        if let Some(m) = &in_menu
            && !menu_path(c, " > ").contains(m.as_str())
        {
            return false;
        }
        let Some(n) = &needle else { return true };
        let field = |k: &str| c.get(k).and_then(Value::as_str).unwrap_or("").to_lowercase();
        field("id").contains(n.as_str()) || field("label").contains(n.as_str()) || menu_path(c, " ").contains(n.as_str())
    };
    Value::Array(
        list.into_iter()
            .filter(hit)
            .map(|mut c| {
                if brief && let Some(o) = c.as_object_mut() {
                    for key in ["params", "schema", "inputSchema"] {
                        o.remove(key);
                    }
                }
                c
            })
            .collect(),
    )
}

/// `{kind: "headless"|"remote", describe}`: where the tools act.
pub(super) fn backend_info(b: &dyn Backend) -> Value {
    let describe = b.describe();
    let kind = if describe.starts_with("remote") { "remote" } else { "headless" };
    let mut info = json!({"kind": kind, "describe": describe, "desktopWindow": kind == "remote"});
    if let Some(app) = b.app() {
        info["app"] = app;
    }
    info
}

fn dispatch(b: &mut dyn Backend, name: &str, a: &Args) -> Result<ToolResult, String> {
    let j = |v: Value| Ok(ToolResult::json(&v));
    match name {
        "command_list" => {
            let all = b.call("engine.commands", json!({}))?;
            let mut filters = a.clone();
            if let Some(v) = filters.remove("enabled_only") {
                filters.insert("enabledOnly".into(), v);
            }
            j(filter_commands(all, &filters))
        }
        "command_batch" => command_batch(b, a),
        "render_preview" => render_preview(b, a),
        "ui_screenshot" => {
            let mut args = a.clone();
            args.insert("window".into(), json!(true));
            screenshot(b, &args)
        }
        "list_commands" => {
            let all = b.call("engine.commands", json!({}))?;
            let list = filter_commands(all, a);
            j(json!({"count": list.as_array().map_or(0, Vec::len), "commands": list}))
        }
        "command_run" | "run_command" | "invoke_menu" => {
            let pin = expected_document(a)?;
            let cmd = req_str(a, if name == "command_run" { "id" } else { "command" })?;
            let params = match a.get("params") {
                None | Some(Value::Null) => json!({}),
                Some(v @ Value::Object(_)) => v.clone(),
                Some(_) => return Err("`params` must be an object".into()),
            };
            let method = if name == "invoke_menu" && b.has_ui() { "ui.menu.invoke" } else { "engine.execute" };
            let request = json!({"command": cmd, "params": params});
            let mut reply = match pin {
                Some(uid) => Pinned { inner: b, uid }.call(method, request)?,
                None => b.call(method, request)?,
            };
            // A picture in the reply (`study.render`, `study.values {image}`, `study.compare {image}`)
            // goes back as an image the model can look at, not as base64 text it can't read.
            match reply.as_object_mut().and_then(|o| o.remove("pngBase64")) {
                Some(Value::String(encoded)) => {
                    let png = vansoncraft_format::base64_decode(&encoded).ok_or("the command returned bad base64")?;
                    Ok(image_result(&png, reply))
                }
                _ => j(reply),
            }
        }
        "doc_inspect" | "inspect_document" => {
            // Only the slicing and scope options reach the command, which validates them.
            let slice: Map<String, Value> =
                a.iter().filter(|(k, _)| INSPECT_KEYS.contains(&k.as_str())).map(|(k, v)| (k.clone(), v.clone())).collect();
            let mut v = b.call("document.inspect", Value::Object(slice))?;
            // Which backend the document lives in, so an agent can tell a desktop window from an
            // in-process session (F1 in docs/speed/audit.md).
            if let Some(o) = v.as_object_mut() {
                o.insert("backend".into(), backend_info(b));
            }
            j(v)
        }
        "ui_inspect" | "inspect_ui" => {
            need_ui(b, name)?;
            j(b.call("ui.inspect", json!({}))?)
        }
        "select_tool" => j(b.call("ui.tool.select", json!({"tool": req_str(a, "tool")?}))?),
        "pointer_gesture" => {
            let events = a.get("events").and_then(Value::as_array).filter(|e| !e.is_empty()).ok_or("`events` must be a non-empty array")?;
            for (i, e) in events.iter().enumerate() {
                let kind = e.get("kind").and_then(Value::as_str).unwrap_or("");
                if crate::headless::pointer_kind(kind).is_none() {
                    return Err(format!("event {i}: unknown kind `{kind}` (down, drag, up, move, doubleclick)"));
                }
                if !e.get("x").is_some_and(Value::is_number) || !e.get("y").is_some_and(Value::is_number) {
                    return Err(format!("event {i}: numeric `x` and `y` required"));
                }
                if e.get("backquote").is_some_and(|v| !v.is_boolean()) {
                    return Err(format!("event {i}: `backquote` must be a boolean"));
                }
            }
            if a.get("backquote").is_some_and(|v| !v.is_boolean()) {
                return Err("`backquote` must be a boolean".into());
            }
            if let Some(t) = a.get("tool").and_then(Value::as_str) {
                b.call("ui.tool.select", json!({"tool": t}))?;
            }
            let events: Vec<Value> = events
                .iter()
                .map(|e| {
                    let mut e = e.clone();
                    e["space"] = json!("doc");
                    e
                })
                .collect();
            let mut p = json!({"events": events});
            if let Some(m) = a.get("mods") {
                p["mods"] = m.clone();
            }
            if let Some(backquote) = a.get("backquote") {
                p["backquote"] = backquote.clone();
            }
            j(b.call("ui.pointer", p)?)
        }
        "draw_path" => j(draw_path(b, a)?),
        "draw_shape" => j(draw_shape(b, a)?),
        "set_paint" => {
            if !["fill", "stroke", "strokeWidth"].iter().any(|k| a.contains_key(*k)) {
                return Err("give at least one of `fill`, `stroke`, `strokeWidth`".into());
            }
            let (applied, objects) = apply_paint(b, a, a.get("ids").cloned())?;
            let mut reply = json!({"applied": applied, "objects": objects});
            if objects == 0 {
                reply["note"] = json!(
                    "no object was painted (nothing selected, or `ids` is empty or holds no object this paint applies to): only the paint for new art changed"
                );
            }
            j(reply)
        }
        "press_key" => {
            let mut p = json!({"key": req_str(a, "key")?});
            if let Some(Value::Object(m)) = a.get("mods") {
                for (k, v) in m {
                    p[k.as_str()] = v.clone();
                }
            }
            j(b.call("ui.key", p)?)
        }
        "type_text" => {
            need_ui(b, name)?;
            j(b.call("ui.text", json!({"text": req_str(a, "text")?}))?)
        }
        "answer_dialog" => {
            need_ui(b, name)?;
            j(answer_dialog(b, a)?)
        }
        "open_panel" => {
            need_ui(b, name)?;
            j(b.call("ui.set", json!({"panel": req_str(a, "panel")?}))?)
        }
        "screenshot" => screenshot(b, a),
        "open_file" => {
            let path = absolute_path(b, req_str(a, "path")?)?;
            let mut p = json!({"path": path});
            for k in ["importer", "params"] {
                if let Some(v) = a.get(k).filter(|v| !v.is_null()) {
                    p[k] = v.clone();
                }
            }
            j(b.call("app.open", p)?)
        }
        "save_file" => {
            let mut p = json!({"path": a.get("path").and_then(Value::as_str).map(|p| absolute_path(b, p)).transpose()?});
            for k in SAVE_FILE_OPTIONS {
                if let Some(v) = a.get(k).filter(|v| !v.is_null()) {
                    p[k] = v.clone();
                }
            }
            // The app writes the file before it replies, even with Background Save on, so a failed
            // save is this call's error.
            if b.has_ui() {
                p["wait"] = json!(true);
            }
            j(b.call("app.save", p)?)
        }
        "export" => {
            // One params object for every backend; the named arguments win over `options` (a named
            // artboard or range replaces every artboard choice in `options`).
            let mut params = a.get("options").and_then(Value::as_object).cloned().unwrap_or_default();
            if ["artboard", "range"].iter().any(|k| a.get(*k).is_some_and(|v| !v.is_null())) {
                for k in ARTBOARD_PARAMS {
                    params.remove(k);
                }
            }
            for k in named_export_params(a) {
                if let Some(v) = a.get(*k).filter(|v| !v.is_null()) {
                    params.insert((*k).into(), v.clone());
                }
            }
            if let Some(path) = params.get("path").and_then(Value::as_str) {
                let path = absolute_path(b, path)?;
                params.insert("path".into(), json!(path));
            }
            let cmd = if a.get("selection").and_then(Value::as_bool) == Some(true) { "document.exportSelection" } else { "document.export" };
            j(exec(b, cmd, Value::Object(params))?)
        }
        "add_text" => j(add_text(b, a)?),
        "apply_effect" => {
            let Some(effect) = a.get("effect").and_then(Value::as_str) else {
                let all = exec(b, "effect.list", json!({}))?;
                return j(json!({"catalog": all.get("catalog")}));
            };
            let mut p = json!({"effect": effect, "params": a.get("params").cloned().unwrap_or(json!({}))});
            if let Some(ids) = a.get("ids") {
                p["ids"] = ids.clone();
            }
            j(exec(b, "effect.apply", p)?)
        }
        "pathfinder" => {
            select_ids(b, a)?;
            j(exec(b, &format!("object.pathfinder.{}", req_str(a, "operation")?), json!({}))?)
        }
        "transform" => j(transform(b, a)?),
        "create_graph" => {
            let mut p = Value::Object(a.clone());
            if p.get("type").is_none() {
                p["type"] = json!("column");
            }
            j(exec(b, "graph.create", p)?)
        }
        "text_wrap" => {
            select_ids(b, a)?;
            if a.get("release").and_then(Value::as_bool) == Some(true) {
                return j(exec(b, "object.textWrap.release", json!({}))?);
            }
            let mut p = json!({});
            for k in ["offset", "invert"] {
                if let Some(v) = a.get(k) {
                    p[k] = v.clone();
                }
            }
            j(exec(b, "object.textWrap.make", p)?)
        }
        "undo" => j(exec(b, "edit.undo", json!({}))?),
        "redo" => j(exec(b, "edit.redo", json!({}))?),
        other => Err(format!("unknown tool `{other}` (see tools/list)")),
    }
}

/// The named `export` arguments passed on to `document.export`. `scale` is raster pixels per
/// point, so only a raster format gets it: DXF's own (drawing units per unit, given in `options`)
/// would resize the drawing and leave its units unset.
fn named_export_params(a: &Args) -> &'static [&'static str] {
    let format = writable_format(a.get("format").and_then(Value::as_str), a.get("path").and_then(Value::as_str));
    if format.is_ok_and(|f| f.raster) {
        &["path", "format", "scale", "artboard", "range", "outlineText"]
    } else {
        &["path", "format", "artboard", "range", "outlineText"]
    }
}

/// Run one tool. Failures (unknown tool, bad arguments, command errors) come back as an
/// `isError` result so the model can read and correct them.
pub fn call_tool(b: &mut dyn Backend, name: &str, args: &Value) -> ToolResult {
    let empty = Map::new();
    let a = match args {
        Value::Object(o) => o,
        Value::Null => &empty,
        _ => return ToolResult::error("tool arguments must be a JSON object"),
    };
    match vansoncraft_engine::guard::catch_panic(|| dispatch(b, name, a)) {
        Ok(Ok(r)) => r,
        Ok(Err(e)) => ToolResult::error(format!("{name}: {e}")),
        Err(e) => ToolResult::error(format!("internal error in `{name}`: {e} (please report this bug)")),
    }
}

/// Most steps one `command_batch` may hold, and most bytes its kept step results may total.
/// Batches run sequentially and every result is kept for later steps' `"$N.path"` references, so
/// without caps one batch could do unbounded work and pile up an unbounded reply (#1727).
pub(crate) const MAX_COMMAND_BATCH_STEPS: usize = 256;

pub(crate) const MAX_COMMAND_BATCH_BYTES: usize = 8 * 1024 * 1024;

/// Refuses an argument the tool's schema doesn't list, as `tools/call` does (#713).
pub(crate) fn check_tool_arguments(name: &str, args: &Args) -> Result<(), String> {
    let defs = tool_definitions();
    let props =
        defs.iter().find(|t| t.get("name").and_then(Value::as_str) == Some(name)).and_then(|t| t.get("inputSchema")?.get("properties")?.as_object());
    match props.and_then(|p| args.keys().find(|k| !p.contains_key(*k))) {
        Some(key) => Err(format!("unknown argument `{key}` for `{name}`")),
        None => Ok(()),
    }
}

/// `expect_document`: the uid (from `doc_inspect`) of the document the caller means to change.
/// Every command the call runs is then pinned to it ([`Pinned`]): the app, or the headless
/// session, checks it in the same step that runs the command, so a command finds another document
/// active (a person switched tabs in the desktop app, or a step opened another file) and refuses,
/// with nothing run, instead of changing that document (F2b in docs/speed/audit.md).
fn expected_document(a: &Args) -> Result<Option<u64>, String> {
    match a.get("expect_document").filter(|v| !v.is_null()) {
        None => Ok(None),
        Some(v) => v.as_u64().map(Some).ok_or_else(|| "`expect_document` must be a document uid (a whole number from doc_inspect)".to_string()),
    }
}

fn command_batch(b: &mut dyn Backend, a: &Args) -> Result<ToolResult, String> {
    match expected_document(a)? {
        Some(uid) => batch_steps(&mut Pinned { inner: b, uid }, a),
        None => batch_steps(b, a),
    }
}

/// [`command_batch`] on `b`, pinned to the expected document when there is one ([`Pinned`] lets
/// the atomic group's end through, so it closes, and rolls back, whichever document is active).
fn batch_steps(b: &mut dyn Backend, a: &Args) -> Result<ToolResult, String> {
    let steps = a.get("steps").and_then(Value::as_array).ok_or("`steps` must be an array")?;
    if steps.len() > MAX_COMMAND_BATCH_STEPS {
        return Err(format!("`steps` holds {} steps; at most {MAX_COMMAND_BATCH_STEPS}", steps.len()));
    }
    let stop = match a.get("stop_on_error") {
        None => true,
        Some(v) => v.as_bool().ok_or("`stop_on_error` must be a boolean")?,
    };
    let atomic = match a.get("atomic") {
        None => false,
        Some(v) => v.as_bool().ok_or("`atomic` must be a boolean")?,
    };
    // An atomic batch that fails is taken back unless the caller keeps what ran (F3).
    let keep = match a.get("keep_on_error") {
        None => false,
        Some(v) => v.as_bool().ok_or("`keep_on_error` must be a boolean")?,
    };
    if keep && !atomic {
        return Err("`keep_on_error` is for an atomic batch: give atomic:true too".into());
    }
    if atomic {
        let label = a.get("label").cloned().unwrap_or(Value::Null);
        b.call("engine.execute", json!({"command":"edit.undoGroup.begin","params":{"label":label}}))?;
    } else if a.get("label").is_some() {
        return Err("`label` names the undo step of an atomic batch: give atomic:true too".into());
    }
    let (mut completed, mut failed, mut results) = (0, 0, Vec::new());
    // Each step's result (null for a failed one), for later steps' `"$N.path"` references.
    let mut values: Vec<Value> = vec![];
    // The kept results all go back in the reply, so their total size is capped: one batch can't
    // pile up an unbounded answer (#1727).
    let mut kept_bytes = 0usize;
    // The objects the drawing steps made (#2113).
    let mut created: Vec<Value> = vec![];
    for step in steps {
        let result = vansoncraft_engine::guard::catch_panic(|| {
            let args = step.as_object().ok_or("each step must be an object")?;
            if args.keys().any(|k| k != "id" && k != "params") {
                return Err("unknown step argument (expected id, params)".into());
            }
            let id = req_str(args, "id")?;
            let params = match args.get("params") {
                None | Some(Value::Null) => json!({}),
                Some(v @ Value::Object(_)) => vansoncraft_engine::steps::resolve(v, &values)?,
                Some(_) => return Err("`params` must be an object".into()),
            };
            match id {
                // The drawing tools, with the arguments `tools/call` takes (#2113).
                "draw_path" | "draw_shape" => {
                    let args = params.as_object().cloned().unwrap_or_default();
                    check_tool_arguments(id, &args)?;
                    let made = if id == "draw_path" { draw_path(b, &args)? } else { draw_shape(b, &args)? };
                    if let Some(new) = made.get("id").filter(|v| v.is_u64()) {
                        created.push(new.clone());
                    }
                    Ok(made)
                }
                _ => b.call("engine.execute", json!({"command":id,"params":params})),
            }
        })
        .unwrap_or_else(|e| Err(format!("internal error: {e}")));
        match result {
            Ok(v) => {
                let entry = json!({"ok": true, "result": v});
                let size = entry.to_string().len();
                if kept_bytes.saturating_add(size) > MAX_COMMAND_BATCH_BYTES {
                    // The results already kept stay; this step ran, and its result is replaced by
                    // the reason the batch stops (saying it ran, so nobody runs it again). The loop breaks either way — carrying on would defeat the cap —
                    // and the atomic group below still closes, as for any stopped batch.
                    failed += 1;
                    values.push(Value::Null);
                    results.push(json!({"ok": false, "error": format!("this step ran, but the batch's results exceed {MAX_COMMAND_BATCH_BYTES} bytes: its result is dropped and the batch stops here")}));
                    break;
                }
                kept_bytes += size;
                completed += 1;
                values.push(entry["result"].clone());
                results.push(entry);
            }
            Err(e) => {
                failed += 1;
                values.push(Value::Null);
                results.push(json!({"ok":false,"error":e}));
                if stop {
                    break;
                }
            }
        }
    }
    // The steps that ran stay as one undo step; when one failed, the group is taken back instead
    // (its edits undone in every document it reached), unless `keep_on_error`. Files the steps
    // wrote stay written, and documents they opened stay open.
    let roll_back = atomic && failed > 0 && !keep;
    let mut reply = json!({"completed":completed,"failed":failed,"results":results,"created":created});
    if atomic {
        let end = b.call("engine.execute", json!({"command":"edit.undoGroup.end","params":{"cancel": roll_back}}));
        match end {
            Ok(_) if roll_back => {
                reply["rolledBack"] = json!(true);
                reply["created"] = json!([]);
                reply["note"] = json!(
                    "a step failed, so the edits the steps before it made in the active document and in documents they opened were undone; edits in a document open before the batch that a step switched to stay, as do files written and documents opened; their ids are void"
                );
            }
            Ok(_) => {}
            // The results still go back: the steps ran.
            Err(e) => reply["warning"] = json!(format!("the batch's undo group could not be closed: {e}")),
        }
    }
    let mut r = ToolResult::json(&reply);
    r.is_error = failed > 0;
    Ok(r)
}

/// `render_preview` with a `region`, a `mode` or `showTemplates`: the engine's `study.render`
/// draws it (a zoom into any rectangle, grey, or value groups), in both backends.
fn study_preview(b: &mut dyn Backend, a: &Args, max: u32) -> Result<ToolResult, String> {
    let mut p = json!({"maxSide": max.max(16)});
    for (from, to) in [
        ("artboard", "artboard"),
        ("region", "region"),
        ("mode", "mode"),
        ("levels", "levels"),
        ("thresholds", "thresholds"),
        ("showTemplates", "showTemplates"),
        ("flip", "flip"),
    ] {
        if let Some(v) = a.get(from).filter(|v| !v.is_null()) {
            p[to] = v.clone();
        }
    }
    let mut r = b.call("engine.execute", json!({"command": "study.render", "params": p}))?;
    let encoded = r.get("pngBase64").and_then(Value::as_str).ok_or("renderer returned no image")?.to_string();
    let png = vansoncraft_format::base64_decode(&encoded).ok_or("renderer returned bad base64")?;
    if let Some(o) = r.as_object_mut() {
        o.remove("pngBase64");
    }
    Ok(image_result(&png, r))
}

/// The longer side a default `render_preview` draws a small artboard at.
const PREVIEW_FLOOR: u32 = 512;

fn render_preview(b: &mut dyn Backend, a: &Args) -> Result<ToolResult, String> {
    let max = max_side(a)?.unwrap_or(1024);
    let study = ["region", "mode", "showTemplates", "levels", "thresholds", "flip"].iter().any(|k| a.get(*k).is_some_and(|v| !v.is_null()));
    // A zoomed region is drawn at max_side; a whole artboard at the size the plain preview gives
    // it, so its colour and value views compare pixel for pixel.
    if study && a.get("region").is_some_and(|v| !v.is_null()) {
        return study_preview(b, a, max);
    }
    let index = match a.get("artboard") {
        None => 0,
        Some(v) => v.as_u64().ok_or("`artboard` must be a 0-based artboard index")?,
    };
    let trace_views = trace_views(a)?;
    let doc = b.call("document.inspect", json!({"depth":0}))?;
    let board =
        doc.get("artboards").and_then(Value::as_array).and_then(|v| v.get(usize::try_from(index).ok()?)).ok_or(format!("no artboard {index}"))?;
    let width = board.get("width").and_then(Value::as_f64).ok_or("artboard has no width")?;
    let height = board.get("height").and_then(Value::as_f64).ok_or("artboard has no height")?;
    if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
        return Err("invalid artboard dimensions".into());
    }
    // The headless renderer clamps scale at 0.01. Bound its allocation even for
    // a huge artboard, then downsize to the requested dimensions. A small artboard is drawn larger
    // (F6 in docs/speed/audit.md), up to 16 pixels a point: to max_side when it is given, else to
    // 512 pixels, so a 32 pt icon is not a 32 pixel preview while a page keeps its usual size
    // (image tokens grow with pixels).
    // max_side stays an upper bound: a page is never drawn past 1 pixel a point, and a small
    // artboard is drawn up to 512 pixels (or max_side when smaller).
    let longest = width.max(height);
    let fit = f64::from(max) / longest;
    let scale = fit.min((f64::from(max.min(PREVIEW_FLOOR)) / longest).max(1.0)).clamp(0.01, 16.0);
    if study {
        // The same size as the plain preview, drawn by study.render.
        let side = (longest * scale).round().clamp(16.0, 4096.0) as u32;
        return study_preview(b, a, side);
    }
    if width.max(height) * scale > 4096.0 {
        return Err("artboard is too large for a bounded preview".into());
    }
    let mut p = json!({"scale":scale, "artboard": index});
    if trace_views {
        p["traceViews"] = json!(true);
    }
    let r = b.call("ui.render", p)?;
    let encoded = r.get("pngBase64").and_then(Value::as_str).ok_or("renderer returned no image")?;
    let png = vansoncraft_format::base64_decode(encoded).ok_or("renderer returned bad base64")?;
    // The render was asked for at most 4096 a side.
    let img = super::handlers::decode_png(&png, 8192).map_err(|e| format!("read preview: {e}"))?;
    let img = if img.width() > max || img.height() > max { img.thumbnail(max, max) } else { img };
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Png).map_err(|e| format!("encode preview: {e}"))?;
    Ok(image_result(out.get_ref(), json!({"width":img.width(),"height":img.height()})))
}
