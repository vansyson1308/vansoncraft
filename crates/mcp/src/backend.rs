//! Where MCP tool calls end up: a control-channel method call.

use std::io::{BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use serde_json::{Value, json};

/// Something that answers control-channel methods (`engine.execute`, `document.inspect`,
/// `ui.pointer`, `ui.render`, `app.export`, …). See `vansoncraft_ui_egui::control` for the list.
pub trait Backend {
    /// Call one method. `Ok` carries the `result`, `Err` the error message.
    fn call(&mut self, method: &str, params: Value) -> Result<Value, String>;
    /// True when a real UI is attached (`ui.*` methods like `ui.key` / `ui.inspect` work).
    fn has_ui(&self) -> bool;
    /// Short human description ("headless", "remote 127.0.0.1:7979").
    fn describe(&self) -> String;
    /// The desktop app's own answer to `app.identity` (`{name, appId, version, pid}`), when it was
    /// checked on connecting.
    fn app(&self) -> Option<Value> {
        None
    }
}

/// `inner` with every command it runs pinned to the document `uid` (MCP `expect_document`): each
/// `engine.execute` / `ui.menu.invoke` carries `expectDocument`, which the app (or the headless
/// session) checks in the same step that runs the command, so a tab switched in between, or a
/// document a step opened, makes the next command refuse instead of changing another document.
pub struct Pinned<'a> {
    pub inner: &'a mut dyn Backend,
    pub uid: u64,
}

impl Backend for Pinned<'_> {
    fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        // Closing an undo group is never refused: a batch that stopped because another document
        // became active still closes (and rolls back) its group, which reaches every document the
        // batch changed.
        let closes_group = params.get("command").and_then(Value::as_str) == Some("edit.undoGroup.end");
        let params = match (method, params) {
            ("engine.execute" | "ui.menu.invoke" | "command", Value::Object(mut o)) if !closes_group => {
                o.insert("expectDocument".into(), json!(self.uid));
                Value::Object(o)
            }
            (_, other) => other,
        };
        self.inner.call(method, params)
    }
    fn has_ui(&self) -> bool {
        self.inner.has_ui()
    }
    fn describe(&self) -> String {
        self.inner.describe()
    }
    fn app(&self) -> Option<Value> {
        self.inner.app()
    }
}

/// A running VansonCraft app, reached through its loopback control port.
pub struct Remote {
    addr: String,
    conn: Option<(BufReader<TcpStream>, TcpStream)>,
    next_id: u64,
    /// What the app said it is (`app.identity`), when connected with [`Remote::connect_verified`]:
    /// every new connection asks again before it carries a call.
    identity: Option<Value>,
    /// The last identity check found a new launch of the app; the next call is refused once.
    restarted: bool,
}

/// How long the app has to answer `app.identity` (it answers in its next frame).
const IDENTITY_TIMEOUT: Duration = Duration::from_secs(10);

/// Longest accepted control-channel reply line (bytes, without the newline). The app answers
/// small JSON objects except for images, which reach tens of megabytes; anything longer is refused
/// and the connection dropped, so a faulty endpoint can't make the server buffer without bound
/// (#1727).
const MAX_REPLY_BYTES: usize = 256 * 1024 * 1024;

impl Remote {
    /// Connect to `addr` (e.g. `127.0.0.1:7979`), failing fast when nothing is listening.
    pub fn connect(addr: &str) -> std::io::Result<Self> {
        let mut r = Self { addr: addr.to_string(), conn: None, next_id: 1, identity: None, restarted: false };
        r.reconnect()?;
        Ok(r)
    }

    /// [`Remote::connect`], then make sure the program on `addr` is VansonCraft (`app.identity`),
    /// on this connection and on every later one: another app may listen on the same port (the
    /// default 7979 is also VectorCraft's), and its documents must never be edited by mistake.
    /// An app too old to answer `app.identity` is refused too: it can't check `expectDocument`.
    pub fn connect_verified(addr: &str) -> Result<Self, String> {
        let mut r = Self::connect(addr).map_err(|e| e.to_string())?;
        r.identity = Some(Value::Null);
        if let Err(e) = r.verify() {
            r.conn = None;
            return Err(e);
        }
        Ok(r)
    }

    /// What the app said it is (`{name, appId, version, pid, instance}`), when connected with
    /// [`Remote::connect_verified`].
    pub fn identity(&self) -> Option<&Value> {
        self.identity.as_ref().filter(|v| !v.is_null())
    }

    /// Ask the program on the current connection what it is, and refuse anything but VansonCraft.
    /// A VansonCraft of another launch than the one reached before (the app was restarted) sets
    /// `restarted`, which `prepare_connection` reports.
    fn verify(&mut self) -> Result<(), String> {
        let before = self.identity().cloned();
        let id = self.next_id;
        self.next_id += 1;
        let line = json!({"id": id, "method": "app.identity", "params": {}}).to_string();
        let kept = self.conn.as_ref().and_then(|(_, s)| s.read_timeout().ok().flatten());
        if let Some((_, s)) = self.conn.as_ref() {
            s.set_read_timeout(Some(IDENTITY_TIMEOUT)).map_err(|e| e.to_string())?;
        }
        let reply = self.roundtrip(&line);
        if let Some((_, s)) = self.conn.as_ref() {
            let _ = s.set_read_timeout(kept); // Only lengthens the wait of later calls if it fails.
        }
        let not_ours = |why: String| {
            format!(
                "the program listening on {} is not VansonCraft ({why}); another app may hold this port, or a VansonCraft older than 0.25.0, which can't check documents: start VansonCraft 0.25.0 or later with --control <port> and connect to that port",
                self.addr
            )
        };
        let reply = reply.map_err(|e| {
            format!(
                "the program listening on {} did not say what it is within {} s (`app.identity`: {e}); if it is VansonCraft and busy, try again",
                self.addr,
                IDENTITY_TIMEOUT.as_secs()
            )
        })?;
        let v: Value = serde_json::from_str(reply.trim()).map_err(|_| not_ours("its answer is not JSON".into()))?;
        if v.get("id") != Some(&json!(id)) && v.get("id").is_some() {
            return Err(not_ours("its answer is not to app.identity".into()));
        }
        if v.get("ok").and_then(Value::as_bool) != Some(true) {
            let why = v.get("error").and_then(Value::as_str).unwrap_or("it refused app.identity");
            if v.get("id").is_none() {
                // The control server's own refusals (too many connections) carry no id.
                return Err(format!("VansonCraft at {} refused the connection: {why}", self.addr));
            }
            return Err(not_ours(format!("it answered app.identity with: {why}")));
        }
        let me = v.get("result").cloned().unwrap_or(Value::Null);
        match me.get("name").and_then(Value::as_str) {
            Some("VansonCraft") => {
                let launch = |i: &Value| (i.get("instance").cloned(), i.get("pid").cloned());
                self.restarted = before.as_ref().is_some_and(|b| launch(b) != launch(&me));
                self.identity = Some(me);
                Ok(())
            }
            Some(other) => Err(not_ours(format!("it says it is {other}"))),
            None => Err(not_ours("it gave no name".into())),
        }
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }

    #[cfg(test)]
    pub(crate) fn connection_probe(&self) -> TcpStream {
        self.conn.as_ref().unwrap().1.try_clone().unwrap()
    }

    fn reconnect(&mut self) -> std::io::Result<()> {
        self.conn = None;
        let mut last = std::io::Error::new(std::io::ErrorKind::NotFound, format!("cannot resolve {}", self.addr));
        for sa in self.addr.to_socket_addrs()? {
            match TcpStream::connect_timeout(&sa, Duration::from_millis(800)) {
                Ok(s) => {
                    s.set_nodelay(true).ok();
                    // The app answers within 60 s (its own timeout); leave headroom.
                    s.set_read_timeout(Some(Duration::from_secs(90))).ok();
                    let read = s.try_clone()?;
                    self.conn = Some((BufReader::new(read), s));
                    return Ok(());
                }
                Err(e) => last = e,
            }
        }
        Err(last)
    }

    /// A connection to carry the next call: the current one, or a new one whose program is asked
    /// what it is ([`Remote::connect_verified`]). Fails with nothing sent, and with no connection
    /// kept, when the app can't be reached or isn't VansonCraft. A new launch of the app (it was
    /// restarted) fails once too, keeping the connection: each launch counts document uids and
    /// object ids from 1 again, so what the client holds would name other documents and objects,
    /// and the client must inspect again before the next call goes through.
    fn prepare_connection(&mut self) -> Result<(), String> {
        let ready = self.reach();
        if ready.is_err() {
            self.conn = None;
        }
        ready?;
        if std::mem::take(&mut self.restarted) {
            return Err(
                "was restarted since the last call: the documents, their uids and the object ids from before are gone (a new launch counts them from 1 again); call doc_inspect and work from what it shows"
                    .into(),
            );
        }
        Ok(())
    }

    fn reach(&mut self) -> Result<(), String> {
        if let Some((_, stream)) = self.conn.as_ref() {
            stream.set_nonblocking(true).map_err(|e| e.to_string())?;
            let probe = stream.peek(&mut [0]);
            stream.set_nonblocking(false).map_err(|e| e.to_string())?;
            match probe {
                // Closed by the app, or holding bytes nobody asked for (an app that explains why it
                // closes): either way no reply to the next request can be read from it.
                Ok(_) => self.conn = None,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(_) => self.conn = None,
            }
        }
        if self.conn.is_none() {
            self.reconnect().map_err(|e| format!("is not reachable: {e}"))?;
            // A new connection may reach another program than the first one did (the app was
            // closed and something else took the port): it is asked again.
            if self.identity.is_some() {
                self.verify()?;
            }
        }
        Ok(())
    }

    /// Wait until the app starts to answer, giving up when the call is cancelled
    /// ([`vansoncraft_engine::cancel`]): the app has no way to stop what it runs, but this server
    /// stops waiting for it (the connection is dropped, so a late answer can't be mistaken for
    /// the next call's). Without a cancellable call this is just the read the caller does next.
    fn wait_for_reply(stream: &TcpStream) -> std::io::Result<()> {
        const POLL: Duration = Duration::from_millis(100);
        const LIMIT: Duration = Duration::from_secs(90);
        if !vansoncraft_engine::cancel::is_cancellable() {
            return Ok(());
        }
        let start = std::time::Instant::now();
        let kept = stream.read_timeout()?;
        stream.set_read_timeout(Some(POLL))?;
        let outcome = loop {
            match stream.peek(&mut [0]) {
                // Bytes (or the end of the stream) to read now.
                Ok(_) => break Ok(()),
                Err(e) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {
                    if vansoncraft_engine::cancel::is_cancelled() {
                        break Err(std::io::Error::new(
                            std::io::ErrorKind::Interrupted,
                            "cancelled; the app may still finish what it was asked, which can't be stopped from here",
                        ));
                    }
                    if start.elapsed() >= LIMIT {
                        break Err(e);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => break Err(e),
            }
        };
        stream.set_read_timeout(kept)?;
        outcome
    }

    fn roundtrip(&mut self, line: &str) -> std::io::Result<String> {
        let Some((reader, writer)) = self.conn.as_mut() else {
            return Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "not connected to the app"));
        };
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Self::wait_for_reply(writer)?;
        let mut buf = Vec::new();
        if !crate::server::read_line_capped(reader, &mut buf, MAX_REPLY_BYTES)? {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "control channel closed"));
        }
        if buf.len() > MAX_REPLY_BYTES {
            // A faulty endpoint's endless line is refused rather than buffered: the connection is
            // dropped and the call reports uncertainty, as for any reply that never arrived whole.
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("control reply longer than {MAX_REPLY_BYTES} bytes")));
        }
        String::from_utf8(buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

impl Backend for Remote {
    fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        self.prepare_connection().map_err(|e| format!("VansonCraft app at {}: {e}; `{method}`: nothing was sent", self.addr))?;
        let id = self.next_id;
        self.next_id += 1;
        let line = json!({"id": id, "method": method, "params": params}).to_string();
        let v: Value = self
            .roundtrip(&line)
            .and_then(|reply| {
                serde_json::from_str(reply.trim())
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("bad reply from app: {e}")))
            })
            .map_err(|e| {
                self.conn = None;
                format!("VansonCraft app at {}: `{method}` failed: {e}; operation may have completed; inspect state before retrying", self.addr)
            })?;
        if v.get("id").is_none() && v.get("ok").and_then(Value::as_bool) == Some(false) {
            // The app's refusals carry no id (too many connections, a line it would not read):
            // it ran nothing and closes the connection, so the next call starts a fresh one.
            self.conn = None;
            let why = v.get("error").and_then(Value::as_str).unwrap_or("unknown error");
            return Err(format!("VansonCraft app at {} refused `{method}`: {why}; nothing was run", self.addr));
        }
        if v.get("id") != Some(&json!(id)) {
            // Not the answer to this call: the stream is desynchronised (or the endpoint faulty),
            // so the connection is dropped and the next call starts a fresh one. The operation may
            // still have completed, as with any reply that never arrived whole (#1728).
            self.conn = None;
            return Err(format!(
                "VansonCraft app at {}: reply id mismatch (expected {id}); connection dropped; operation may have completed; inspect state before retrying",
                self.addr
            ));
        }
        if v.get("ok").and_then(Value::as_bool) == Some(true) {
            Ok(v.get("result").cloned().unwrap_or(Value::Null))
        } else {
            Err(v.get("error").and_then(Value::as_str).unwrap_or("unknown error").to_string())
        }
    }

    fn has_ui(&self) -> bool {
        true
    }

    fn app(&self) -> Option<Value> {
        self.identity().cloned()
    }

    fn describe(&self) -> String {
        match self.identity().and_then(|i| i.get("version")).and_then(Value::as_str) {
            Some(v) => format!("remote {} (VansonCraft {v})", self.addr),
            None => format!("remote {}", self.addr),
        }
    }
}
