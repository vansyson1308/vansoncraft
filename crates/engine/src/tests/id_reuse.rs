//! An id an undo took away is never handed to a new object (F2, docs/speed/audit.md): a script or
//! agent still holding the old id gets "no such object" instead of changing the new object.

use serde_json::json;

use crate::*;

fn session() -> Session {
    let mut s = Session::new();
    s.unique_ids = true;
    s.execute("file.new", &json!({"width": 400, "height": 300})).unwrap();
    s
}

fn id_of(v: serde_json::Value) -> u64 {
    v["id"].as_u64().unwrap()
}

#[test]
fn a_new_object_after_undo_does_not_take_the_undone_objects_id() {
    let mut s = session();
    let rect = id_of(s.execute("shape.rectangle", &json!({"x": 10, "y": 10, "width": 100, "height": 50})).unwrap());
    s.execute("edit.undo", &json!({})).unwrap();
    let ellipse = id_of(s.execute("shape.ellipse", &json!({"x": 300, "y": 200, "width": 40, "height": 40})).unwrap());
    assert_ne!(rect, ellipse);
    // The stale id names nothing now.
    assert!(s.execute("select.set", &json!({"ids": [rect]})).is_err());
    assert!(s.execute("paint.setFill", &json!({"ids": [rect], "color": "#ff0000"})).is_err());
}

#[test]
fn without_unique_ids_the_counter_follows_the_document_as_before() {
    let mut s = Session::new();
    s.execute("file.new", &json!({"width": 400, "height": 300})).unwrap();
    let rect = id_of(s.execute("shape.rectangle", &json!({"x": 10, "y": 10, "width": 100, "height": 50})).unwrap());
    s.execute("edit.undo", &json!({})).unwrap();
    let ellipse = id_of(s.execute("shape.ellipse", &json!({"x": 300, "y": 200, "width": 40, "height": 40})).unwrap());
    assert_eq!(rect, ellipse, "the default keeps the ids recorded journals expect");
}

#[test]
fn undo_and_redo_still_restore_the_same_objects() {
    let mut s = session();
    let rect = id_of(s.execute("shape.rectangle", &json!({"x": 10, "y": 10, "width": 100, "height": 50})).unwrap());
    let before = s.doc().unwrap().doc.clone();
    s.execute("edit.undo", &json!({})).unwrap();
    s.execute("edit.redo", &json!({})).unwrap();
    assert_eq!(*s.doc().unwrap().doc, *before);
    assert!(s.doc().unwrap().doc.node(NodeId(rect)).is_some());
    // Undoing back to the saved state still reads as unchanged.
    let mut s = session();
    s.execute("shape.rectangle", &json!({"x": 10, "y": 10, "width": 100, "height": 50})).unwrap();
    s.execute("edit.undo", &json!({})).unwrap();
    assert!(!s.doc().unwrap().is_dirty());
}

/// A cancelled undo group (an atomic batch that failed) leaves no journal entries, so a replay of
/// the journal never makes its objects: the ids after it must be the ones a replay gives, and the
/// floor is not raised for it. Its ids are void (the MCP reply says `rolledBack`).
#[test]
fn a_cancelled_undo_group_leaves_ids_as_a_replay_makes_them() {
    let mut s = session();
    s.execute("edit.undoGroup.begin", &json!({"label": "Job"})).unwrap();
    let a = id_of(s.execute("shape.rectangle", &json!({"x": 10, "y": 10, "width": 100, "height": 50})).unwrap());
    s.execute("edit.undoGroup.end", &json!({"cancel": true})).unwrap();
    assert!(s.doc().unwrap().doc.node(NodeId(a)).is_none());
    let b = id_of(s.execute("shape.ellipse", &json!({"x": 300, "y": 200, "width": 40, "height": 40})).unwrap());
    let mut replay = session();
    let c = id_of(replay.execute("shape.ellipse", &json!({"x": 300, "y": 200, "width": 40, "height": 40})).unwrap());
    assert_eq!(b, c);
}
