//! Connecting to the desktop app with [`Remote::connect_verified`]: the program on the port must
//! say it is VansonCraft (`app.identity`) before any call reaches it, and commands pinned to a
//! document ([`crate::backend::Pinned`]) carry `expectDocument` for the app to check.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::mpsc;

use serde_json::{Value, json};

use crate::backend::Pinned;
use crate::{Backend, Remote};

/// A one-connection stand-in for a program on a control port: `identity` answers `app.identity`
/// (an `ok: false` error when it is a string), everything else is echoed. The requests it got
/// come back on the channel.
fn program(identity: Result<Value, &'static str>) -> (String, mpsc::Receiver<Value>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let Ok((stream, _)) = listener.accept() else { return };
        let mut out = stream.try_clone().unwrap();
        for line in BufReader::new(stream).lines() {
            let Ok(line) = line else { return };
            let msg: Value = serde_json::from_str(&line).unwrap();
            let reply = match (msg["method"].as_str(), &identity) {
                (Some("app.identity"), Ok(me)) => json!({"id": msg["id"], "ok": true, "result": me}),
                (Some("app.identity"), Err(why)) => json!({"id": msg["id"], "ok": false, "error": why}),
                _ => json!({"id": msg["id"], "ok": true, "result": {"echo": msg["params"]}}),
            };
            let _ = tx.send(msg);
            if writeln!(out, "{reply}").is_err() {
                return;
            }
        }
    });
    (addr, rx)
}

#[test]
fn vansoncraft_is_accepted_and_named() {
    let me = json!({"name": "VansonCraft", "appId": "io.github.vansyson1308.vansoncraft", "version": "9.9.9", "pid": 42});
    let (addr, seen) = program(Ok(me.clone()));
    let mut r = Remote::connect_verified(&addr).unwrap();
    assert_eq!(r.identity(), Some(&me));
    assert_eq!(r.app(), Some(me));
    assert_eq!(r.describe(), format!("remote {addr} (VansonCraft 9.9.9)"));
    assert_eq!(seen.recv().unwrap()["method"], "app.identity", "asked before anything else");
    let echoed = r.call("engine.execute", json!({"command": "shape.rectangle", "params": {}})).unwrap();
    assert_eq!(echoed["echo"]["command"], "shape.rectangle");
}

#[test]
fn another_program_on_the_port_is_refused() {
    // An app too old to know `app.identity` (VectorCraft answers this way), and one that names
    // itself otherwise: neither gets a call.
    for (identity, says) in [
        (Err("unknown method `app.identity`"), "unknown method `app.identity`"),
        (Ok(json!({"name": "VectorCraft", "version": "0.24.1"})), "it says it is VectorCraft"),
        (Ok(json!({"version": "1.0"})), "it gave no name"),
    ] {
        let (addr, seen) = program(identity);
        let e = Remote::connect_verified(&addr).err().unwrap();
        assert!(e.contains(&format!("the program listening on {addr} is not VansonCraft")) && e.contains(says), "{e}");
        assert!(e.contains("--control <port>"), "says what to do: {e}");
        let asked: Vec<Value> = seen.try_iter().collect();
        assert_eq!(asked.len(), 1, "only app.identity was sent: {asked:?}");
    }
    // Nothing listening at all.
    let free = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().to_string();
    assert!(Remote::connect_verified(&free).is_err());
}

#[test]
fn pinned_commands_carry_the_document_for_the_app_to_check() {
    let (addr, seen) = program(Ok(json!({"name": "VansonCraft", "version": "9.9.9"})));
    let mut r = Remote::connect_verified(&addr).unwrap();
    let _ = seen.recv().unwrap();
    let mut pinned = Pinned { inner: &mut r, uid: 7 };
    for method in ["engine.execute", "ui.menu.invoke"] {
        pinned.call(method, json!({"command": "shape.rectangle", "params": {}})).unwrap();
        assert_eq!(seen.recv().unwrap()["params"]["expectDocument"], 7, "{method}");
    }
    // Reading the document is not pinned, and closing an undo group never is.
    pinned.call("document.inspect", json!({"depth": 0})).unwrap();
    assert!(seen.recv().unwrap()["params"].get("expectDocument").is_none());
    pinned.call("engine.execute", json!({"command": "edit.undoGroup.end", "params": {"cancel": true}})).unwrap();
    assert!(seen.recv().unwrap()["params"].get("expectDocument").is_none());
}
