#!/usr/bin/env python3
"""End-to-end check of a VansonCraft release package through its MCP server (stdio).

Runs against an installed or unpacked `vansoncraft-cli`, never a source build: the release
workflow calls it on clean runners with nothing but the package and Python's standard library.

  mcp_smoke.py --cli PATH --work DIR [--version V] [--desktop PORT] [--report FILE]

Headless (always): initialize, list the tools, draw (shapes and Vietnamese type), preview, change
one object and check the others stay as they were, preview a region, save a native file and
export SVG and PNG under a folder whose name has spaces and Vietnamese letters, export over an
existing file, try to write outside the allowed folder (refused), open the saved file again and
compare, and check that `--connect` to a port where nothing listens fails instead of falling
back to a headless session.

--desktop PORT: instead drive a VansonCraft window already started with `--control PORT`: the
server must report the desktop backend and the app's identity, and its edits must reach that
window's document.

--setup: instead check `setup claude-desktop --apply` (twice: the second run changes nothing) and
`doctor --json` from the installed copy, with client settings in a home folder of their own.

Prints one line per check and a JSON report (timings in seconds). Exits 1 on the first failure.
"""

import argparse
import base64
import json
import os
import queue
import socket
import subprocess
import sys
import threading
import time

FOLDER = "Thư mục thử VansonCraft"
NAME = "Bản vẽ thử"
GREETING = "Xin chào Việt Nam: vẽ, xem, sửa"
TOOLS = {"command_run", "command_batch", "doc_inspect", "render_preview", "draw_shape", "add_text", "set_paint", "save_file", "export", "open_file"}


class Failure(Exception):
    pass


def check(cond, what):
    if not cond:
        raise Failure(what)
    print(f"ok   {what}", flush=True)


class Mcp:
    """A newline-delimited JSON-RPC client over the server's stdin and stdout."""

    def __init__(self, argv, env=None):
        self.started = time.monotonic()
        self.proc = subprocess.Popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env)
        self.lines = queue.Queue()
        self.stderr = []
        self.next_id = 1
        threading.Thread(target=self._read, args=(self.proc.stdout, self.lines), daemon=True).start()
        threading.Thread(target=self._drain, daemon=True).start()

    @staticmethod
    def _read(stream, q):
        for raw in stream:
            q.put(raw)
        q.put(None)

    def _drain(self):
        for raw in self.proc.stderr:
            self.stderr.append(raw.decode("utf-8", "replace"))

    def send(self, msg):
        self.proc.stdin.write((json.dumps(msg, ensure_ascii=False) + "\n").encode("utf-8"))
        self.proc.stdin.flush()

    def request(self, method, params=None, timeout=120):
        rid = self.next_id
        self.next_id += 1
        self.send({"jsonrpc": "2.0", "id": rid, "method": method, "params": params or {}})
        end = time.monotonic() + timeout
        while True:
            left = end - time.monotonic()
            if left <= 0:
                raise Failure(f"no answer to {method} within {timeout} s")
            try:
                raw = self.lines.get(timeout=left)
            except queue.Empty:
                continue
            if raw is None:
                raise Failure(f"the server closed its output during {method}: {''.join(self.stderr)[-2000:]}")
            msg = json.loads(raw)
            if msg.get("id") == rid:
                if "error" in msg:
                    raise Failure(f"{method}: JSON-RPC error {msg['error']}")
                return msg["result"]

    def tool(self, name, args=None, expect_error=False):
        r = self.request("tools/call", {"name": name, "arguments": args or {}})
        if bool(r.get("isError")) != expect_error:
            raise Failure(f"{name}({json.dumps(args, ensure_ascii=False)[:300]}): isError={r.get('isError')}: {text_of(r)[:600]}")
        return r

    def close(self, timeout=20):
        try:
            self.proc.stdin.close()
        except OSError:
            pass
        try:
            return self.proc.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            self.proc.kill()
            raise Failure(f"the server did not exit within {timeout} s of its input closing")


def text_of(result):
    return "\n".join(c.get("text", "") for c in result.get("content", []) if c.get("type") == "text")


def json_of(result):
    t = text_of(result)
    try:
        return json.loads(t)
    except ValueError:
        raise Failure(f"expected JSON text, got: {t[:400]}")


def png_of(result):
    images = [c for c in result.get("content", []) if c.get("type") == "image"]
    if not images:
        raise Failure(f"no image block in the result: {text_of(result)[:300]}")
    data = base64.b64decode(images[0]["data"])
    if images[0].get("mimeType") != "image/png" or data[:8] != b"\x89PNG\r\n\x1a\n":
        raise Failure("the image block is not a PNG")
    width, height = int.from_bytes(data[16:20], "big"), int.from_bytes(data[20:24], "big")
    return data, width, height


def nodes(doc):
    """Every node of a doc_inspect layer tree, depth first."""
    out = []

    def walk(items):
        for n in items or []:
            out.append(n)
            walk(n.get("children"))

    walk(doc.get("layers"))
    return out


def free_port():
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    port = s.getsockname()[1]
    s.close()
    return port


def control(port, method, params=None, timeout=60):
    """One request on the desktop app's control channel (JSON lines on 127.0.0.1:port)."""
    with socket.create_connection(("127.0.0.1", port), timeout=timeout) as s:
        s.sendall((json.dumps({"id": 1, "method": method, "params": params or {}}) + "\n").encode("utf-8"))
        buf = b""
        while not buf.endswith(b"\n"):
            chunk = s.recv(65536)
            if not chunk:
                break
            buf += chunk
    r = json.loads(buf)
    if r.get("error"):
        raise Failure(f"control {method}: {r['error']}")
    return r.get("result")


def headless(args, report):
    work = os.path.join(args.work, FOLDER)
    os.makedirs(work, exist_ok=True)
    argv = [args.cli, "mcp", "--headless", "--automation-read-root", work, "--automation-write-root", work]
    t0 = time.monotonic()
    m = Mcp(argv)
    init = m.request("initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "vansoncraft-smoke", "version": "1"}})
    report["initialize_s"] = round(time.monotonic() - t0, 3)
    m.send({"jsonrpc": "2.0", "method": "notifications/initialized"})
    info = init.get("serverInfo", {})
    check(info.get("name") == "vansoncraft", f"server name {info.get('name')}")
    if args.version:
        check(info.get("version") == args.version, f"server version {info.get('version')} (expected {args.version})")
    tools = {t["name"] for t in m.request("tools/list")["tools"]}
    report["tools_ready_s"] = round(time.monotonic() - t0, 3)
    report["tools"] = len(tools)
    check(TOOLS <= tools, f"{len(tools)} tools, including {', '.join(sorted(TOOLS))}")

    doc = json_of(m.tool("doc_inspect"))
    check(doc.get("backend", {}).get("kind") == "headless", "headless backend reported")
    rect = json_of(m.tool("draw_shape", {"shape": "rectangle", "x": 40, "y": 40, "width": 300, "height": 180, "fill": "#1d6f8a", "stroke": "none", "name": "nền"}))["id"]
    ell = json_of(m.tool("draw_shape", {"shape": "ellipse", "x": 380, "y": 60, "width": 140, "height": 140, "fill": "#f4b544", "stroke": "#123c46", "strokeWidth": 3}))["id"]
    txt = json_of(m.tool("add_text", {"x": 40, "y": 280, "text": GREETING, "size": 28, "color": "#123c46"}))["id"]
    check(all(isinstance(i, int) for i in (rect, ell, txt)), f"drew a rectangle ({rect}), an ellipse ({ell}) and Vietnamese type ({txt})")

    t1 = time.monotonic()
    _, w, h = png_of(m.tool("render_preview", {"max_side": 512}))
    report["first_preview_s"] = round(time.monotonic() - t1, 3)
    report["first_preview_since_start_s"] = round(time.monotonic() - t0, 3)
    check(w > 0 and h > 0 and max(w, h) <= 512, f"preview {w}x{h} PNG")

    before = {n["id"]: n for n in nodes(json_of(m.tool("doc_inspect")))}
    m.tool("set_paint", {"ids": [ell], "fill": "#c0392b"})
    after = {n["id"]: n for n in nodes(json_of(m.tool("doc_inspect")))}
    check(after[ell] != before[ell], "the ellipse changed")
    check(after[rect] == before[rect] and after[txt] == before[txt], "the rectangle and the type stayed as they were")
    _, rw, rh = png_of(m.tool("render_preview", {"region": [360, 40, 180, 180], "max_side": 256}))
    check(rw > 0 and rh > 0, f"region preview {rw}x{rh}")

    native = os.path.join(work, NAME + ".vansoncraft")
    svg = os.path.join(work, NAME + ".svg")
    png = os.path.join(work, NAME + ".png")
    m.tool("save_file", {"path": native})
    m.tool("export", {"path": svg})
    m.tool("export", {"path": png})
    check(os.path.getsize(native) > 0, f"saved {native}")
    with open(svg, encoding="utf-8") as f:
        svg_text = f.read()
    check("<svg" in svg_text and ("<path" in svg_text or "<rect" in svg_text), f"exported {svg}")
    with open(png, "rb") as f:
        check(f.read(8) == b"\x89PNG\r\n\x1a\n", f"exported {png}")
    size = os.path.getsize(png)
    m.tool("export", {"path": png, "scale": 2})
    check(os.path.getsize(png) != size, "exporting over an existing file replaces it")

    outside = os.path.join(args.work, "outside.svg")
    refused = m.tool("export", {"path": outside}, expect_error=True)
    check(not os.path.exists(outside), f"writing outside the allowed folder is refused: {text_of(refused)[:120]}")

    m.tool("open_file", {"path": native})
    again = json_of(m.tool("doc_inspect"))
    kinds = sorted(n.get("kind", "") for n in nodes(again) if n.get("kind") != "Layer")
    check(len(kinds) == 3, f"reopened {NAME}.vansoncraft: {kinds}")
    texts = [n.get("text") for n in nodes(again) if n.get("kind") == "Type"]
    check(texts == [GREETING], f"its Vietnamese type came back unchanged: {texts}")
    code = m.close()
    check(code == 0, f"the server exited with {code} when its input closed")

    # An explicit desktop address where no app listens: an error, never a silent headless session.
    port = free_port()
    p = subprocess.run([args.cli, "mcp", "--connect", f"127.0.0.1:{port}"], input=b"", capture_output=True, timeout=30)
    err = p.stderr.decode("utf-8", "replace")
    check(p.returncode != 0 and "cannot connect" in err, f"--connect to a closed port fails: {err.strip()[:160]}")


def desktop(args, report):
    port = args.desktop
    m = Mcp([args.cli, "mcp", "--connect", f"127.0.0.1:{port}"])
    t0 = time.monotonic()
    m.request("initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "vansoncraft-smoke", "version": "1"}})
    m.send({"jsonrpc": "2.0", "method": "notifications/initialized"})
    report["desktop_initialize_s"] = round(time.monotonic() - t0, 3)
    first = m.request("tools/call", {"name": "doc_inspect", "arguments": {}})
    if first.get("isError"):
        # The window shows its Home screen: start a document in it, as an agent would.
        check("no document open" in text_of(first), f"no document open yet: {text_of(first)[:120]}")
        m.tool("command_run", {"id": "file.new", "params": {"width": 600, "height": 400}})
    doc = json_of(m.tool("doc_inspect"))
    backend = doc.get("backend", {})
    check(backend.get("kind") == "remote" and backend.get("desktopWindow") is True, f"desktop backend reported: {backend}")
    app = backend.get("app") or {}
    check(app.get("name") == "VansonCraft", f"the app on the port says it is VansonCraft: {app}")
    if args.version:
        check(app.get("version") == args.version, f"app version {app.get('version')} (expected {args.version})")
    star = json_of(m.tool("draw_shape", {"shape": "star", "cx": 200, "cy": 200, "radius1": 80, "radius2": 36, "points": 5, "fill": "#f4b544", "stroke": "none"}))["id"]
    seen = control(port, "document.inspect", {"ids": [star]})
    check(any(n.get("id") == star for n in (seen or {}).get("nodes", [])), f"the window's own document has the star ({star})")
    data, w, h = png_of(m.tool("ui_screenshot"))
    if args.report:
        with open(os.path.join(os.path.dirname(os.path.abspath(args.report)), "desktop-window.png"), "wb") as f:
            f.write(data)
    check(w > 0 and h > 0, f"window screenshot {w}x{h}")
    code = m.close()
    check(code == 0, f"the desktop bridge exited with {code}")


def setup(args, report):
    """`setup claude-desktop --apply` from the installed copy, twice (the second changes nothing),
    then `doctor --json` must find the entry and its program, with the client settings in a home
    folder of their own (the runner's are never touched)."""
    home = os.path.join(args.work, "home")
    os.makedirs(home, exist_ok=True)
    config = os.path.join(home, "claude_desktop_config.json")
    with open(config, "w", encoding="utf-8") as f:
        json.dump({"mcpServers": {"other": {"command": "other-server"}}, "theme": "dark"}, f)
    env = dict(os.environ, HOME=home, USERPROFILE=home, APPDATA=home, CODEX_HOME=os.path.join(home, ".codex"))
    run = lambda *a: subprocess.run([args.cli, *a], capture_output=True, timeout=60, env=env)
    first = run("setup", "claude-desktop", "--apply", "--config", config)
    check(first.returncode == 0, f"setup claude-desktop --apply: {first.stdout.decode('utf-8', 'replace').strip()[:200]} {first.stderr.decode('utf-8', 'replace')[:300]}")
    second = run("setup", "claude-desktop", "--apply", "--config", config)
    check(second.returncode == 0 and "nothing changed" in second.stdout.decode("utf-8", "replace"), "running setup again changes nothing")
    with open(config, encoding="utf-8") as f:
        written = json.load(f)
    servers = written.get("mcpServers", {})
    check(set(servers) == {"other", "vansoncraft"} and written.get("theme") == "dark", f"one vansoncraft entry, the other server and settings kept: {sorted(servers)}")
    check(os.path.samefile(servers["vansoncraft"]["command"], args.cli), f"the entry runs the installed CLI: {servers['vansoncraft']['command']}")
    backups = [n for n in os.listdir(home) if "vansoncraft-backup" in n]
    check(len(backups) == 1, f"one backup of the file as it was: {backups}")
    for client in ("claude-code", "codex"):
        printed = run("setup", client)
        out = printed.stdout.decode("utf-8", "replace")
        check(printed.returncode == 0 and args.cli in out.replace('"', "").replace("'", ""), f"setup {client} prints the command for the installed CLI")
    # The Claude Desktop file `doctor` reads is the platform's own; point it at this one.
    if sys.platform == "win32":
        os.makedirs(os.path.join(home, "Claude"), exist_ok=True)
        os.replace(config, os.path.join(home, "Claude", "claude_desktop_config.json"))
    else:
        dest = os.path.join(home, ".config", "Claude")
        os.makedirs(dest, exist_ok=True)
        os.replace(config, os.path.join(dest, "claude_desktop_config.json"))
    d = run("doctor", "--json", "--connect", f"127.0.0.1:{free_port()}")
    check(d.returncode == 0, f"doctor passes: {d.stdout.decode('utf-8', 'replace')[:600]}")
    lines = [f"{c['level']} {c['check']}" for c in json.loads(d.stdout)["checks"]]
    check(any(l.startswith("ok Claude Desktop") and "(this program)" in l for l in lines), f"doctor finds the entry: {lines}")
    check(any(l.startswith("ok installed at") for l in lines), "doctor sees an installed copy")
    report["doctor"] = lines


def main():
    # Windows runs Python with a legacy code page on a pipe: the Vietnamese names must still print.
    for stream in (sys.stdout, sys.stderr):
        if hasattr(stream, "reconfigure"):
            stream.reconfigure(encoding="utf-8", errors="replace")
    ap = argparse.ArgumentParser()
    ap.add_argument("--cli", required=True)
    ap.add_argument("--work", required=True)
    ap.add_argument("--version")
    ap.add_argument("--desktop", type=int)
    ap.add_argument("--setup", action="store_true", help="check setup and doctor instead")
    ap.add_argument("--report")
    args = ap.parse_args()
    report = {"cli": args.cli}
    try:
        if args.setup:
            setup(args, report)
        elif args.desktop:
            desktop(args, report)
        else:
            headless(args, report)
    except Failure as e:
        print(f"FAIL {e}", flush=True)
        sys.exit(1)
    finally:
        print(json.dumps(report, ensure_ascii=False), flush=True)
        if args.report:
            with open(args.report, "w", encoding="utf-8") as f:
                json.dump(report, f, ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
