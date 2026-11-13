# Connecting Claude and Codex (Windows first)

> **Status:** 2026-11-13. Written against the clients' official documentation (Sources, at the end).
> The compatibility matrix says exactly which combinations were run, and when. The first release,
> [0.25.0](https://github.com/vansyson1308/vansoncraft/releases/tag/v0.25.0), is out (a pre-release).

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

Download a release from [GitHub Releases](https://github.com/vansyson1308/vansoncraft/releases).
The current one is [0.25.0](https://github.com/vansyson1308/vansoncraft/releases/tag/v0.25.0), a
pre-release; link to it by its version, since `/releases/latest` names stable releases only. Check
the download against the release's `SHA256SUMS.txt` (`Get-FileHash <file>` in PowerShell):

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
  `invoke_menu` and `command_batch`: each command the call sends (each step of a batch) is checked
  against it as it runs, in the app's own frame, so a tab the person switches to (or a document an
  earlier step opened) is never changed; the step is refused with nothing run. A command that runs
  others itself (`command.batch`, an action, a script) is checked once, when it starts.
- If the app was restarted, every call that could change something says so and sends nothing
  until `doc_inspect` has gone through: document uids and object ids from before name nothing in
  the new launch.
- New objects never take the id of one an undo took away (headless, and the desktop app with
  `--control`): a stale id fails with "no such object" instead of changing a new object.
- `command_batch {atomic: true}` rolls back when a step fails (`rolledBack: true`): the document
  edits are undone; files a step wrote and documents it opened stay. The ids it created are void.
- When a call times out, inspect before retrying, so nothing is drawn twice.
- Part names can repeat across artboards: scope `document.find` with `within` or `intersects`.
- Drawing method: `vansoncraft://method` and the prompts `speed-drawing`, `composition-variants`,
  `revise-locally`, `memory-exercise`.

## Compatibility matrix

What was run, with which version. Scripted checks are **PROTOCOL VERIFIED**; a real client with a
model is **CLIENT VERIFIED**.

| Client | Version | OS | Backend | Transport | Result |
|---|---|---|---|---|---|
| MCP protocol tests (`cargo test -p vansoncraft-mcp`) | — | Linux | headless + fake desktop app | in-process / TCP | **PROTOCOL VERIFIED**, including the identity check, pinned documents and a restarted app |
| Scripted MCP client (scenarios A–G, I of `acceptance-matrix.md`) | — | Linux | headless | stdio | **PROTOCOL VERIFIED** |
| Desktop app + `vansoncraft-cli mcp --connect` (scenario H) | debug build | Linux, Xvfb, Mesa llvmpipe | desktop | TCP 127.0.0.1:7979 | **PROTOCOL VERIFIED**: `backend.kind` `remote`, a 75-step composition drawn in the window and seen in `ui_screenshot`; closing the app → "not reachable … nothing was sent"; a wrong port → exit 1 with the reason |
| Desktop app, the person switching tabs between the agent's calls (2026-11-13) | debug build | Linux, Xvfb | desktop | TCP | **PROTOCOL VERIFIED**: `command_run` and an atomic `command_batch` with `expect_document` refused ("nothing was run"); nothing drawn in the other document; a program answering like VectorCraft on the port refused |
| Claude Code CLI (`claude -p`, a real model) | 2.1.322 | Linux (container) | headless, `--automation-*-root` | stdio | **CLIENT VERIFIED**, two runs. Run 1 (a binary built before the method notes existed): 13 MCP calls, drawing saved and exported; it reported honestly that `vansoncraft://method/mass` was missing. Run 2 (the same Vietnamese vase brief): read `vansoncraft://method/procedure` and `…/mass`, 47 tool calls in 48 turns, measured with `study.values`, revised twice, saved `.vansoncraft` and SVG |
| Claude Code CLI (`claude -p`, a real model), 2026-11-13 | 2.1.322 | Linux (container), Xvfb | desktop (`--connect 127.0.0.1:7979`) | stdio + TCP | **CLIENT VERIFIED**: 26 tool calls in 66 s; drew sky, sun, mountains and "Chào buổi sáng Việt Nam" in the window, previewed, changed only the sun with `expect_document`, saved `.vansoncraft` and exported SVG and PNG into `Bản vẽ thử/buổi sáng.*`, reopened the file (6 objects). `claude mcp add` as `setup claude-code` prints it: added, `claude mcp list` Connected, a second add refused as a duplicate |
| Claude Desktop | — | Windows/macOS | — | — | **NOT VERIFIED** (no desktop client here); `setup claude-desktop --apply` is tested against the documented file format |
| Codex CLI / IDE / desktop app | — | any | — | — | **NOT VERIFIED** (no OpenAI account here); `setup codex` prints the documented command and TOML |
| Release 0.25.0 packages: the scripted MCP client (`packaging/smoke/mcp_smoke.py`), `setup` and `doctor`, and the desktop app with the bridge ([release run](https://github.com/vansyson1308/vansoncraft/actions/runs/43371867221)) | 0.25.0 | Windows Server 2025 (portable ZIP; MSI installed for the user beside VectorCraft 0.24.1), Ubuntu 24.04 (tar.gz, AppImage `--version`); Rust removed from the runners | headless and desktop | stdio + TCP | **PROTOCOL VERIFIED** on the files the draft release held, before they were published |
| The same files downloaded from the public release URLs, no token (`verify-public`) | 0.25.0 | Ubuntu 24.04 (tar.gz) and Windows Server 2025 (portable ZIP) runners | headless | stdio | **PROTOCOL VERIFIED**: every file matched `SHA256SUMS.txt`; the CLI's version and the scripted MCP client |
| The same files downloaded from the public release URLs, in a Linux container | 0.25.0 | Ubuntu 24.04 container, Xvfb | headless and desktop | stdio + TCP | **PROTOCOL VERIFIED**: every file matched `SHA256SUMS.txt`; 38 of 38 checks of `packaging/smoke/linux.sh` passed (MCP, `setup`/`doctor`, the desktop bridge, the AppImage) |
| In-app updater, the released Linux app made to count as 0.24.0 (`VANSONCRAFT_PRETEND_VERSION`) | 0.25.0 | Linux container, Xvfb | — | — | Help › Check for Updates offered nothing and said the app is up to date. The message names the build's own version, not the one it compared. By the updater's code, pre-releases are never offered; this run shows that nothing was offered, not why. Offering a newer **stable** release was not tried (there is none yet) |
| Any client on Windows (Claude Desktop, Claude Code, Codex with a real model) | — | Windows | — | — | **NOT VERIFIED** (no Windows desktop with a client here; the scripted checks above ran on Windows Server 2025) |

## Measured with 0.25.0

**Where the numbers come from:**
- **release:** the [release run](https://github.com/vansyson1308/vansoncraft/actions/runs/43371867221), on the published files, on GitHub-hosted runners (Windows Server 2025, Ubuntu 24.04, no GPU).
- **range:** the same measurement across that run and the two dry runs before it.
- **here:** a Linux container (no GPU) that downloaded the public files.

These are first baselines, not targets; a desktop with a GPU will differ. The Windows runner's
clock ticks about every 16 ms, so its sub-second MCP timings (0.016, 0.031, 0.062 s) are whole ticks.

| What | Windows | Linux |
|---|---|---|
| Download size | MSI 74.2 MiB, portable ZIP 98.9 MiB | tar.gz 93.6 MiB, AppImage 86.7 MiB |
| Download time (here, measured with curl at 74–96 MB/s) | about 1 s each | about 1 s each |
| Unpack | ZIP 1.3 s (release; range 0.8–1.3 s) | tar.gz 1.7 s (release; range 1.5–1.9 s), 1.8 s (here) |
| MSI install / install again / uninstall | 5.3 s / 0.44 s / 1.3 s (release; install range 3.4–5.3 s) | — |
| MCP `initialize` answered | 0.016–0.031 s | 0.004 s (release), 0.007 s (here) |
| Tool list ready, from the server's start | 0.016–0.031 s | 0.005 s (release), 0.01 s (here) |
| First preview, from the server's start | 0.06 s | 0.04 s (release), 0.06 s (here) |
| Desktop app listening on its port, cold / warm | portable 1.0 s / 0.5 s, installed 2.0 s / 0.5 s (release; cold range 1.0–2.0 s) | 0.7 s / 0.1 s (release; cold range 0.7–4.3 s), 2.0 s / 0.1 s (here) |
| Desktop bridge `initialize`, identity check included | 0.06–0.48 s | 0.21 s (release), 0.23 s (here) |

**Manual steps** for Windows, Claude Desktop and desktop mode:
1. download the MSI;
2. run it;
3. run `setup claude-desktop --apply`;
4. start the app with `--control 7979`;
5. restart Claude Desktop.

Claude Code and Codex take the same five steps; their step 3 is running the command `setup` prints. Headless mode has no step 4.

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

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-13 | minor | The first release, 0.25.0: its link; the matrix with the release workflow's checks on Windows Server 2025 and Ubuntu 24.04, the public download and the updater; first measurements. |
| 2026-11-13 | major | For installed packages: install once, `setup`, `doctor`, what an agent checks before downloading; desktop-mode guards; the matrix with the desktop Claude Code run. |
| 2026-11-12 | major | First version. |
