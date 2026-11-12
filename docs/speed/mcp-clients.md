# Connecting Claude and Codex (Windows first)

> **Status:** written 2026-11-12 against the official client docs listed at the end. The
> compatibility matrix says exactly which combinations were run.

## What runs where

`vectorcraft-cli mcp` is a **local stdio** MCP server: the client (Claude Code, Claude Desktop,
Codex) starts it as a child process on the same computer and talks to it over stdin/stdout. It has
two backends:

| Mode | Start it with | What you get |
|---|---|---|
| **Desktop (remote)** | the app with `--control 7979`, then `vectorcraft-cli mcp --connect 127.0.0.1:7979` | the agent draws in the open window; you watch and can edit by hand |
| **Headless** | `vectorcraft-cli mcp --headless` | an in-process engine; no window; save before the server stops |

"Remote" here means a TCP bridge to the app **on your own machine** (127.0.0.1), not a server on the
Internet. There is no Internet transport, so a cloud chat (claude.ai in the browser, ChatGPT) cannot
reach it; use a local client. Do not open port 7979 to the network to work around this.

With no flag the server tries 127.0.0.1:7979 once at start-up and falls back to headless. It now
says so: the `initialize` instructions and a note after the first tool result explain that nothing
appears in a window, and `doc_inspect` reports `backend.kind` (`remote` or `headless`) and the
document `uid`. **To draw in the window you are looking at, start the app first and register the
server with `--connect`**, so a missing app is an error instead of a silent fallback.

## Windows set-up

1. **Build** (this fork has no installer releases): install Rust (rustup, MSVC toolchain), then in
   the repository folder:

   ```bash
   cargo install --path apps/vectorcraft-cli --locked
   ```

   The CLI lands in `%USERPROFILE%\.cargo\bin\vectorcraft-cli.exe`. Build the app the same way
   (`cargo install --path apps/vectorcraft --locked`) or run it from the checkout:

   ```bash
   cargo run --release -p vectorcraft -- --control 7979
   ```

2. **Folders.** Make one folder for references and one for results, e.g. `C:\Art\refs` and
   `C:\Art\out`. Chat attachments are **not** files the server can see: put a reference image or SVG
   in `C:\Art\refs` and give the agent its path.
3. **Confine file access (recommended).** For headless work add
   `--automation-read-root C:\Art --automation-write-root C:\Art\out`; for the desktop app start the
   app itself with those flags (`vectorcraft.exe --control 7979 --automation-read-root C:\Art
   --automation-write-root C:\Art\out`), since a running app enforces its own roots.
4. **Register** the server in your client (below). Use the absolute path to the `.exe`; restart the
   client after changing its configuration.

## Client configuration

### Claude Code (CLI, VS Code, JetBrains)

```bash
claude mcp add --transport stdio vectorcraft -- "C:\Users\me\.cargo\bin\vectorcraft-cli.exe" mcp --connect 127.0.0.1:7979
```

Everything after `--` is the server command. Add `--scope user` to have it in every project, or
`--scope project` to write a shared `.mcp.json` (servers in it need approval the first time):

```json
{"mcpServers": {"vectorcraft": {"type": "stdio", "command": "C:\\Users\\me\\.cargo\\bin\\vectorcraft-cli.exe", "args": ["mcp", "--connect", "127.0.0.1:7979"]}}}
```

Check with `claude mcp list` (✔ Connected) or `/mcp` inside a session. For headless work replace
`--connect 127.0.0.1:7979` with `--headless`.

### Claude Desktop

Edit `%APPDATA%\Claude\claude_desktop_config.json` (macOS: `~/Library/Application
Support/Claude/claude_desktop_config.json`; Settings › Developer › Edit Config), then quit and restart:

```json
{
  "mcpServers": {
    "vectorcraft": {
      "command": "C:\\Users\\me\\.cargo\\bin\\vectorcraft-cli.exe",
      "args": ["mcp", "--connect", "127.0.0.1:7979"]
    }
  }
}
```

Logs: `%APPDATA%\Claude\logs\mcp-server-vectorcraft.log` holds the server's stderr (it names the
backend on its first line). Images dropped into a Claude Desktop chat reach the model, not the MCP
server: save the reference into your refs folder and give its path.

### Codex (CLI and IDE extension)

Codex reads MCP servers from `config.toml` (`~/.codex/config.toml`; on Windows the `.codex` folder
in your user profile, or `$CODEX_HOME`). Use a TOML literal string for a Windows path:

```toml
[mcp_servers.vectorcraft]
command = 'C:\Users\me\.cargo\bin\vectorcraft-cli.exe'
args = ["mcp", "--connect", "127.0.0.1:7979"]
startup_timeout_sec = 20
tool_timeout_sec = 120
```

or `codex mcp add vectorcraft -- 'C:\Users\me\.cargo\bin\vectorcraft-cli.exe' mcp --connect 127.0.0.1:7979`.
Check with `codex mcp list` or `/mcp` in the TUI. If Codex runs inside WSL, it starts Linux
programs: build the Linux CLI inside WSL, and note that a Windows app's control port is reached
from WSL2 only with WSL networking set up for it.

## Compatibility matrix

| Client | Version | OS | Backend | Transport | Result |
|---|---|---|---|---|---|
| MCP protocol tests (`cargo test -p vectorcraft-mcp`) | — | Linux | headless + fake remote | in-process / TCP | **PASS** (protocol level only) |
| Scripted MCP client (scenarios A–G, I) | — | Linux | headless | stdio | **PASS** (see `acceptance-matrix.md`) |
| Claude Code CLI (`claude -p`, real model) | 2.1.322 | Linux (container) | headless | stdio | see `acceptance-matrix.md` row "Client: Claude Code" |
| Desktop app + `--connect` | — | Linux + Xvfb | remote | TCP 127.0.0.1 | see row "Desktop live (H)" |
| Claude Desktop | — | Windows/macOS | — | — | **NOT VERIFIED** (no desktop client in this environment) |
| Codex CLI / IDE | — | any | — | — | **NOT VERIFIED** (not installed here; no OpenAI credentials) |
| Any client on Windows | — | Windows | — | — | **NOT VERIFIED** (no Windows machine here; the shared CI gate may build Windows when Actions run on the fork — check the PR's checks) |

## Working conventions for agents

- Coordinates are points, y down, origin at the first artboard's top-left. Bézier `in`/`out`
  handles are absolute points, not offsets.
- `draw_shape.rotation` turns clockwise; `transform.rotate` and `object.rotate` counter-clockwise.
- Check `doc_inspect` → `backend` and `uid` before editing, and pass `expect_document: uid` to
  `command_run` / `command_batch` so a call meant for one document never runs in another; when a call
  times out, inspect before retrying so nothing is drawn twice.
- `command_batch {atomic: true}` now rolls back when a step fails (`rolledBack: true`); files a step
  wrote and documents it opened are not taken back.
- Part names can repeat across artboards: scope `document.find` with `within` or `intersects`.
- Drawing method: `vectorcraft://method` and the prompts `speed-drawing`, `composition-variants`,
  `revise-locally`, `memory-exercise`.

## Prompts to try (Vietnamese)

- *"Vẽ một bình hoa bằng vector, ưu tiên nhịp điệu đường cong và ba nhóm sáng–trung gian–tối. Cho
  tôi ba bố cục khác nhau trên ba artboard có tên. Tôi muốn sửa được từng bông hoa: mỗi bông là một
  group có tên. Dùng `render_preview` chế độ values và `study.values` để kiểm tra, rồi lưu file
  `.vectorcraft` và xuất SVG vào C:\Art\out."*
- *"Giữ bố cục A hiện tại, làm nét ở điểm nhấn mạnh hơn, giảm chi tiết nền và tăng sự liên kết giữa
  các mảng tối. Chỉ tìm đối tượng trong artboard A. So sánh `doc_inspect` trước và sau để chứng minh
  phần còn lại không đổi. Đừng biến bản vẽ thành một ảnh phẳng."*
- *"Cùng chủ đề, làm một bản nghiên cứu nét (chỉ dùng nét, độ dày thay đổi, hatch theo hướng khối)
  và một bản nghiên cứu mảng (chỉ dùng mảng tô ba giá trị, không viền). Dựng khung bằng
  `study.blockIn` trên lớp Construction."*
- *"Đặt ảnh C:\Art\refs\binh.png vào một lớp template tên Reference, xem nó, ẩn đi, vẽ lại từ trí
  nhớ, rồi so sánh hình bóng bằng `study.compare`."*

## Sources

Read on 2026-11-12: Claude Code — https://code.claude.com/docs/en/mcp; Claude Desktop —
https://modelcontextprotocol.io/quickstart/user; Codex — https://learn.chatgpt.com/docs/extend/mcp
(redirected from developers.openai.com/codex/mcp) and
https://learn.chatgpt.com/codex/config-file/config-reference. The Codex Windows path and timeout key
names were read from page summaries, not verbatim text: confirm them against your Codex version.
