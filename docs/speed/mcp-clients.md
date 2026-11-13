# Connecting Claude and Codex (Windows first)

> **Status:** 2026-11-13. Written against the clients' official documentation (Sources, at the end)
> and the release packages. The compatibility matrix says exactly which combinations were run.

## What runs where

`vansoncraft-cli mcp` is a **local stdio** MCP server: the client (Claude Code, Claude Desktop,
Codex) starts it as a child process on the same computer and talks to it over stdin and stdout. It
has two backends:

| Mode | Start it with | What you get |
|---|---|---|
| **Desktop** | the app with `--control 7979`, then `vansoncraft-cli mcp --connect 127.0.0.1:7979` | the agent draws in the open window; you watch and can edit by hand |
| **Headless** | `vansoncraft-cli mcp --headless --automation-read-root DIR --automation-write-root DIR` | an engine of its own, no window, files only inside DIR; save before the server stops |

"Desktop" reaches the app over a TCP bridge **on your own machine** (127.0.0.1), not over the
Internet. There is no Internet transport, so a cloud chat (claude.ai or ChatGPT in a browser) cannot
reach it: use a local client. Do not open port 7979 to the network to work around this.

With `--connect` the server first asks the program on the port what it is (`app.identity`) and
refuses anything but VansonCraft, so it never drives another app that listens on the same port
(VectorCraft uses 7979 too); nothing answering is an error, never a silent headless session.

## Install once (no Rust, no build)

Download a release from [GitHub Releases](https://github.com/vansyson1308/vansoncraft/releases)
and check it against the release's `SHA256SUMS.txt` (`Get-FileHash <file>` in PowerShell):

| Package | Where the programs are |
|---|---|
| Windows MSI (`vansoncraft-<version>-windows-x64.msi`, installs for the current user) | `%LOCALAPPDATA%\Programs\VansonCraft\vansoncraft.exe` and `vansoncraft-cli.exe`; the Start menu has VansonCraft |
| Windows portable ZIP (`…-windows-x64-portable.zip`) | the folder you unpack it into (keep it; settings stay in `VansonCraftData` beside the programs) |
| Linux tar.gz (`…-linux-x86_64.tar.gz`, glibc 2.35+) | `<folder>/vansoncraft-<version>-linux-x86_64/bin/vansoncraft` and `vansoncraft-cli` |

The packages are not code-signed yet: SmartScreen may say the publisher is unknown. The AppImage
runs the desktop app; for agents use the tar.gz, whose CLI has a lasting path.

Then, in a terminal, from the installed copy (Windows example):

```powershell
& "$env:LOCALAPPDATA\Programs\VansonCraft\vansoncraft-cli.exe" doctor
```

`doctor` checks the program and its version, the desktop app beside it, the headless engine (one
drawing and preview), what answers on 127.0.0.1:7979, and every client entry that names VansonCraft.

## Connect a client (once)

`setup` prints (or, for Claude Desktop, writes) the server entry for **this** installed program, so
the client never points at a build folder or a temporary download:

```powershell
$cli = "$env:LOCALAPPDATA\Programs\VansonCraft\vansoncraft-cli.exe"
& $cli setup claude-desktop --apply            # desktop mode, written into Claude Desktop's settings
& $cli setup claude-code                       # prints the `claude mcp add …` command to run
& $cli setup codex                             # prints the `codex mcp add …` command and the TOML
& $cli setup codex --headless --folder C:\Art  # headless, reading and writing only in C:\Art
```

- **Claude Desktop:** `--apply` keeps every other server and setting, saves a backup beside the
  file (`claude_desktop_config.json.vansoncraft-backup-<time>`), writes atomically, replaces an older
  `vansoncraft` entry instead of adding a second one and does nothing when the entry is already
  right. `--remove` takes out only that entry. Quit Claude Desktop completely and start it again.
  The file is `%APPDATA%\Claude\claude_desktop_config.json` (macOS `~/Library/Application
  Support/Claude/…`); when only the Microsoft Store copy's folder exists, setup uses that one;
  `--config FILE` names another.
- **Claude Code:** run the printed `claude mcp add --scope user --transport stdio vansoncraft -- …`
  (it stores the server in `~/.claude.json`); `claude mcp list` should show it connected. If an
  entry named `vansoncraft` exists, remove it first with the printed `claude mcp remove` command.
- **Codex** (CLI, IDE extension, desktop app; they share `~/.codex/config.toml`, on Windows
  `%USERPROFILE%\.codex`): run the printed `codex mcp add vansoncraft -- …`, or paste the printed
  `[mcp_servers.vansoncraft]` table. `codex mcp list` should show it.

Run `doctor` again afterwards: it reads the three clients' settings and says whether each entry runs
this program.

### Every time you draw

- **Desktop mode:** start the app with the control port, then start (or restart) the client:

  ```powershell
  & "$env:LOCALAPPDATA\Programs\VansonCraft\vansoncraft.exe" --control 7979
  ```

  To confine what the agent's commands read and write, start the app with
  `--automation-read-root C:\Art --automation-write-root C:\Art\out` as well.
- **Headless mode:** nothing to start; the client starts the server.

Nothing is downloaded or built again: the client starts the installed `vansoncraft-cli`.

## For agents setting this up

Before downloading anything, look for an installed copy and reuse it:

1. Windows: `%LOCALAPPDATA%\Programs\VansonCraft\vansoncraft-cli.exe` (the MSI), then
   `Get-Command vansoncraft-cli`, then a folder the user names (portable ZIP). Linux:
   `command -v vansoncraft-cli`, then a folder the user names.
2. Run `vansoncraft-cli --version` and `vansoncraft-cli doctor --json`. A copy that reports the
   version wanted (or newer) and no `fail` is used as it is.
3. Only when there is none, or the user asks for a newer one: download that version's package from
   its release URL (`https://github.com/vansyson1308/vansoncraft/releases/download/v<version>/<file>`),
   check its SHA-256 against that release's `SHA256SUMS.txt`, and install it for the current user
   (`msiexec /i <file>.msi /qn`, no administrator rights) or unpack it into a folder the user keeps.
4. Register it with `vansoncraft-cli setup <client> …` and check with `doctor`. Never clone the
   repository or run cargo to set VansonCraft up; never point a client at a download or build folder.
5. Ask the user which folder the work may read and write, and use exactly that (`--folder`, or the
   app's `--automation-*-root`); never widen access to the whole disk to make a check pass.

## Working conventions for agents

- Coordinates are points, y down, origin at the first artboard's top-left. Bézier `in`/`out`
  handles are absolute points, not offsets.
- `draw_shape.rotation` turns clockwise; `transform.rotate` and `object.rotate` counter-clockwise.
- Check `doc_inspect` → `backend` (`kind`, and `app` with the desktop app's name and version) and
  `uid` before editing, and pass `expect_document: uid` to `command_run`, `run_command`,
  `invoke_menu` and `command_batch`: every command the call runs is checked against it as it runs, in
  the app's own frame, so a tab the person switches to (or a document an earlier step opened) is
  never changed; the step is refused with nothing run.
- New objects never take the id of one an undo took away (headless, and the desktop app with
  `--control`): a stale id fails with "no such object" instead of changing a new object.
- `command_batch {atomic: true}` rolls back when a step fails (`rolledBack: true`): the document
  edits are undone; files a step wrote and documents it opened stay. The ids it created are void.
- When a call times out, inspect before retrying, so nothing is drawn twice.
- Part names can repeat across artboards: scope `document.find` with `within` or `intersects`.
- Drawing method: `vansoncraft://method` and the prompts `speed-drawing`, `composition-variants`,
  `revise-locally`, `memory-exercise`.

## Compatibility matrix

See [the release notes and docs/mcp.md](../mcp.md) for the version each row was checked with.

| Client | Version | OS | Backend | Transport | Result |
|---|---|---|---|---|---|
| MCP protocol tests (`cargo test -p vansoncraft-mcp`) | — | Linux | headless + fake desktop app | in-process / TCP | **PROTOCOL VERIFIED** |
| Scripted MCP client on the release packages (`packaging/smoke/mcp_smoke.py`) | — | Windows Server 2025, Ubuntu 24.04 (GitHub runners) | headless and desktop | stdio, TCP 127.0.0.1 | **PROTOCOL VERIFIED** by the release workflow (see the release notes) |
| Claude Code CLI (`claude -p`, a real model) | see docs/mcp.md | Linux | headless and desktop | stdio | **CLIENT VERIFIED** (see docs/mcp.md › Verified clients) |
| Claude Desktop | — | Windows/macOS | — | — | **NOT VERIFIED** (no desktop client here); `setup claude-desktop --apply` is tested against the documented file format |
| Codex CLI / IDE / desktop app | — | any | — | — | **NOT VERIFIED** (no OpenAI account here); `setup codex` prints the documented command and TOML |
| Any client on Windows | — | Windows | — | — | **NOT VERIFIED** with a real client (the packages are checked on Windows by the scripted client) |

## Prompts to try (Vietnamese)

- *"Vẽ một bình hoa bằng vector, ưu tiên nhịp điệu đường cong và ba nhóm sáng–trung gian–tối. Cho
  tôi ba bố cục khác nhau trên ba artboard có tên. Tôi muốn sửa được từng bông hoa: mỗi bông là một
  group có tên. Dùng `render_preview` chế độ values và `study.values` để kiểm tra, rồi lưu file
  `.vansoncraft` và xuất SVG vào C:\Art\out."*
- *"Giữ bố cục A hiện tại, làm nét ở điểm nhấn mạnh hơn, giảm chi tiết nền và tăng sự liên kết giữa
  các mảng tối. Chỉ tìm đối tượng trong artboard A. So sánh `doc_inspect` trước và sau để chứng minh
  phần còn lại không đổi. Đừng biến bản vẽ thành một ảnh phẳng."*
- *"Cùng chủ đề, làm một bản nghiên cứu nét (chỉ dùng nét, độ dày thay đổi, hatch theo hướng khối)
  và một bản nghiên cứu mảng (chỉ dùng mảng tô ba giá trị, không viền). Dựng khung bằng
  `study.blockIn` trên lớp Construction."*
- *"Đặt ảnh C:\Art\refs\binh.png vào một lớp template tên Reference, xem nó, ẩn đi, vẽ lại từ trí
  nhớ, rồi so sánh hình bóng bằng `study.compare`."*

## Sources

Read on 2026-11-12 and 2026-11-13:

- Claude Code: https://code.claude.com/docs/en/mcp and https://code.claude.com/docs/en/mcp-quickstart
  (`claude mcp add [options] <name> -- <command> [args...]`, scopes `local`, `project`, `user`;
  `~/.claude.json`, on Windows `%USERPROFILE%\.claude.json`; image results shown to the model).
- Claude Desktop: https://modelcontextprotocol.io/docs/develop/connect-local-servers
  (`claude_desktop_config.json` in `%APPDATA%\Claude` or `~/Library/Application Support/Claude`,
  `mcpServers` → `command`, `args`; quit and restart after editing) and
  https://code.claude.com/docs/en/hipaa-setup (the Microsoft Store copy's folder). Local servers are
  not available in claude.ai or Cowork:
  https://support.claude.com/en/articles/11175166-get-started-with-custom-connectors-using-remote-mcp.
- Codex: https://learn.chatgpt.com/docs/extend/mcp?surface=cli (redirected from
  developers.openai.com/codex/mcp), https://learn.chatgpt.com/docs/developer-commands?surface=cli
  (`codex mcp add <name> -- <command>`, `list`, `get`, `remove`),
  https://learn.chatgpt.com/docs/config-file/config-reference (`[mcp_servers.<name>]`, `command`,
  `args`, `startup_timeout_sec` default 10, `tool_timeout_sec` default 60) and
  https://learn.chatgpt.com/docs/windows/windows-app (`%USERPROFILE%\.codex`). Codex passes MCP
  image results to the model unless the result also has `structuredContent` (openai/codex source);
  VansonCraft's results have none.
