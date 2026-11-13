//! `vansoncraft-cli setup` and `vansoncraft-cli doctor`: connect an AI agent's MCP client to this
//! copy of VansonCraft, and check that the connection works.
//!
//! `setup CLIENT` builds the server entry from where this program is installed, so a client never
//! points at a build folder or a temporary download. For Claude Desktop, whose only setting is a
//! JSON file, `--apply` writes the entry itself: it keeps every other server and setting, backs the
//! file up first, writes atomically, replaces an entry of the same name instead of adding a second
//! one, and `--remove` takes out only that entry. Claude Code and Codex manage their own settings
//! with their own commands (`claude mcp add`, `codex mcp add`), which `setup` prints ready to run.
//!
//! `doctor` reports this program, the desktop app beside it, the headless engine (one drawing and
//! preview), what listens on the control port, and every client entry that names VansonCraft.

use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_json::{Map, Value, json};
use vansoncraft_mcp::{Backend, DEFAULT_ADDR, Headless, Remote};

/// The name the server is registered under in every client.
const NAME: &str = "vansoncraft";
/// The largest client settings file read (Claude Code's `~/.claude.json` keeps history and grows).
const MAX_CONFIG_BYTES: u64 = 64 << 20;

const SETUP_USAGE: &str = "\
vansoncraft-cli setup CLIENT [--desktop [--port N] | --headless --folder DIR...] [--apply | --remove] [--config FILE]
  CLIENT: claude-desktop, claude-code or codex.
  --desktop (default): the agent draws in the VansonCraft window, started with `vansoncraft --control N`
    (N: --port, default 7979). Nothing runs when that window is not open.
  --headless --folder DIR: the agent draws in a session of its own, with no window, and may read and
    write files only inside DIR (give --folder more than once for more folders; missing ones are made).
  Without --apply, prints the entry and how to add it. Claude Desktop: --apply writes it into the
  client's settings file (a backup is kept beside it), --remove takes it out again. Claude Code and
  Codex: run the printed command. --config FILE names Claude Desktop's settings file when it is
  somewhere else.";

/// What `setup` registers: this program and its MCP arguments.
#[derive(Debug, Clone, PartialEq)]
struct Entry {
    command: String,
    args: Vec<String>,
}

pub fn setup(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "-h" || a == "--help") || args.is_empty() {
        outln!("{SETUP_USAGE}");
        return Ok(());
    }
    let mut it = args.iter();
    let client = it.next().cloned().unwrap_or_default();
    let (mut headless, mut desktop, mut apply, mut remove) = (false, false, false, false);
    let mut port: u16 = 7979;
    let mut folders: Vec<String> = vec![];
    let mut config: Option<PathBuf> = None;
    while let Some(a) = it.next() {
        match a.as_str() {
            "--desktop" => desktop = true,
            "--headless" => headless = true,
            "--apply" => apply = true,
            "--remove" => remove = true,
            "--port" => port = it.next().and_then(|v| v.parse().ok()).filter(|p| *p > 0).ok_or("--port needs a port from 1 to 65535")?,
            "--folder" => folders.push(it.next().cloned().ok_or("--folder needs a folder")?),
            "--config" => config = Some(PathBuf::from(it.next().ok_or("--config needs a file")?)),
            other => return Err(format!("unknown setup option `{other}`\n\n{SETUP_USAGE}")),
        }
    }
    if desktop && headless {
        return Err("use either --desktop or --headless".into());
    }
    if apply && remove {
        return Err("use either --apply or --remove".into());
    }
    if headless && folders.is_empty() {
        return Err("--headless needs --folder DIR: the folder the agent may read and write (its input and output files)".into());
    }
    if !headless && !folders.is_empty() {
        return Err("--folder is for --headless; in the desktop app, start it with --automation-read-root / --automation-write-root".into());
    }
    if headless && port != 7979 {
        return Err("--port is for --desktop".into());
    }
    if client != "claude-desktop" && (apply || remove || config.is_some()) {
        return Err(format!("--apply, --remove and --config are for claude-desktop; for {client}, run the command setup prints"));
    }
    let entry = if remove { None } else { Some(entry(headless, port, &folders)?) };
    match client.as_str() {
        "claude-desktop" => claude_desktop(entry.as_ref(), config, apply || remove),
        "claude-code" => {
            print_commands(
                entry.as_ref(),
                "Claude Code",
                &["claude", "mcp", "add", "--scope", "user", "--transport", "stdio", NAME, "--"],
                &["claude", "mcp", "remove", "--scope", "user", NAME],
            );
            outln!("\nThen start a new Claude Code session (or run /mcp in one) and ask it to call doc_inspect.");
            Ok(())
        }
        "codex" => {
            print_commands(entry.as_ref(), "Codex", &["codex", "mcp", "add", NAME, "--"], &["codex", "mcp", "remove", NAME]);
            if let Some(e) = &entry {
                outln!(
                    "\nOr add this to {} yourself:\n{}",
                    codex_config().map(|p| p.display().to_string()).unwrap_or_else(|| "~/.codex/config.toml".into()),
                    toml_entry(e)
                );
            }
            outln!("\nThen start a new Codex session (or restart the extension) and ask it to call doc_inspect.");
            Ok(())
        }
        other => Err(format!("unknown client `{other}`: claude-desktop, claude-code or codex\n\n{SETUP_USAGE}")),
    }
}

/// The server entry for this program: `mcp --connect 127.0.0.1:PORT`, or headless confined to
/// `folders` (made when missing, named by absolute path).
fn entry(headless: bool, port: u16, folders: &[String]) -> Result<Entry, String> {
    let command = installed_cli()?;
    let mut args = vec!["mcp".to_string()];
    if headless {
        args.push("--headless".into());
        for f in folders {
            let dir = absolute(Path::new(f))?;
            std::fs::create_dir_all(&dir).map_err(|e| format!("can't make the folder {}: {e}", dir.display()))?;
            let dir = dir.display().to_string();
            args.extend(["--automation-read-root".into(), dir.clone(), "--automation-write-root".into(), dir]);
        }
    } else {
        args.extend(["--connect".into(), format!("127.0.0.1:{port}")]);
    }
    Ok(Entry { command: command.display().to_string(), args })
}

/// `p` as an absolute path (relative to the current folder), without resolving links.
fn absolute(p: &Path) -> Result<PathBuf, String> {
    // Rebuilt from its components: no trailing separator, which would end a quoted Windows
    // argument in `\"` and run it into the next one.
    std::path::absolute(p).map(|a| a.components().collect()).map_err(|e| format!("{}: {e}", p.display()))
}

/// This program's own path, refused when a client entry naming it would soon break: a cargo build
/// folder, the temporary folder, an AppImage's temporary mount.
fn installed_cli() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|e| format!("can't tell where this program is: {e}"))?;
    match unstable_place(&exe) {
        Some(why) => Err(format!(
            "this copy runs from {} ({why}); a client set up with it would stop working when that folder goes. Install VansonCraft (the MSI, or the portable ZIP or Linux tar.gz unpacked into a folder you keep) and run setup from there",
            exe.display()
        )),
        None => Ok(exe),
    }
}

/// Why `exe` is in a place that doesn't last, if it is.
fn unstable_place(exe: &Path) -> Option<&'static str> {
    let parts: Vec<String> = exe.components().map(|c| c.as_os_str().to_string_lossy().to_lowercase()).collect();
    let named_build = parts.windows(2).any(|w| matches!(w, [t, kind] if t == "target" && (kind == "debug" || kind == "release")));
    // Any cargo output folder, whatever CARGO_TARGET_DIR names it: a profile folder holds
    // `.fingerprint`, and cargo marks the target folder itself with CACHEDIR.TAG.
    let fingerprinted = exe.parent().is_some_and(|d| d.join(".fingerprint").is_dir());
    let tagged = exe.ancestors().skip(1).take(3).any(|a| a.join("CACHEDIR.TAG").is_file());
    if named_build || fingerprinted || tagged {
        return Some("a cargo build folder");
    }
    let in_appimage = parts.iter().any(|p| p.starts_with(".mount_"))
        || std::env::var_os("APPDIR").filter(|d| !d.is_empty()).is_some_and(|d| exe.starts_with(Path::new(&d)));
    if in_appimage {
        return Some("inside an AppImage, which is mounted in a temporary folder while it runs; use the tar.gz package for agents");
    }
    // Compared as given and resolved: Windows' 8.3 short names, `\\?\` paths, links.
    let temp = std::env::temp_dir();
    let (exe_real, temp_real) = (std::fs::canonicalize(exe).ok(), std::fs::canonicalize(&temp).ok());
    let in_temp = exe.starts_with(&temp) || exe_real.as_ref().zip(temp_real.as_ref()).is_some_and(|(e, t)| e.starts_with(t));
    if in_temp {
        return Some("the temporary folder");
    }
    None
}

/// Quote `arg` for the shell the user types into (PowerShell or cmd on Windows, a POSIX shell
/// elsewhere).
fn quote(arg: &str) -> String {
    // A backslash separates folders on Windows; a POSIX shell would take it as an escape.
    let safe = if cfg!(windows) { "-_./:=@\\" } else { "-_./:=@" };
    let plain = !arg.is_empty() && arg.chars().all(|c| c.is_ascii_alphanumeric() || safe.contains(c));
    if plain {
        arg.to_string()
    } else if cfg!(windows) {
        format!("\"{arg}\"")
    } else {
        format!("'{}'", arg.replace('\'', "'\\''"))
    }
}

fn print_commands(entry: Option<&Entry>, client: &str, add: &[&str], remove: &[&str]) {
    let line = |words: Vec<String>| words.iter().map(|w| quote(w)).collect::<Vec<_>>().join(" ");
    match entry {
        Some(e) => {
            let mut words: Vec<String> = add.iter().map(|s| s.to_string()).collect();
            words.push(e.command.clone());
            words.extend(e.args.iter().cloned());
            outln!("Add VansonCraft to {client} (one command; it replaces nothing else):\n\n  {}\n", line(words));
            outln!("If an entry named `{NAME}` is there already, remove it first:\n\n  {}", line(remove.iter().map(|s| s.to_string()).collect()));
        }
        None => outln!("Remove VansonCraft from {client}:\n\n  {}", line(remove.iter().map(|s| s.to_string()).collect())),
    }
}

/// The Codex `config.toml` table for `e` (literal strings: no escaping of Windows paths).
fn toml_entry(e: &Entry) -> String {
    let lit = |s: &str| if s.contains('\'') || s.chars().any(char::is_control) { toml_basic(s) } else { format!("'{s}'") };
    let args: Vec<String> = e.args.iter().map(|a| lit(a)).collect();
    format!("[mcp_servers.{NAME}]\ncommand = {}\nargs = [{}]\n", lit(&e.command), args.join(", "))
}

/// `s` as a TOML basic string: double quotes, with `"`, `\` and control characters escaped as TOML
/// defines them.
fn toml_basic(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => out.push_str(&format!("\\u{:04X}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// ---- Claude Desktop ----------------------------------------------------------------------------

/// Claude Desktop's settings file: `%APPDATA%\Claude\claude_desktop_config.json` on Windows (or the
/// Microsoft Store copy's folder when only that one exists), `~/Library/Application
/// Support/Claude/…` on macOS, `~/.config/Claude/…` on Linux.
fn claude_desktop_config() -> Option<PathBuf> {
    const FILE: &str = "claude_desktop_config.json";
    let env = |k: &str| std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from);
    if cfg!(windows) {
        let roaming = env("APPDATA").map(|p| p.join("Claude"));
        let store = env("LOCALAPPDATA").map(|p| p.join("Packages").join("Claude_pzs8sxrjxfjjc").join("LocalCache").join("Roaming").join("Claude"));
        return match (roaming, store) {
            (Some(r), Some(s)) if !r.is_dir() && s.is_dir() => Some(s.join(FILE)),
            (Some(r), _) => Some(r.join(FILE)),
            (None, s) => s.map(|s| s.join(FILE)),
        };
    }
    let home = env("HOME")?;
    Some(if cfg!(target_os = "macos") { home.join("Library/Application Support/Claude").join(FILE) } else { home.join(".config/Claude").join(FILE) })
}

fn read_json(path: &Path) -> Result<Option<Value>, String> {
    match std::fs::metadata(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("can't read {}: {e}", path.display())),
        Ok(m) if m.len() > MAX_CONFIG_BYTES => return Err(format!("{} is larger than {} MB", path.display(), MAX_CONFIG_BYTES >> 20)),
        Ok(_) => {}
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("can't read {}: {e}", path.display()))?;
    // Some editors start the file with a byte-order mark, which JSON parsers refuse.
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
    if text.trim().is_empty() {
        return Ok(Some(json!({})));
    }
    let v: Value = serde_json::from_str(text)
        .map_err(|e| format!("{} is not valid JSON ({e}); fix or move it, then run setup again: nothing was changed", path.display()))?;
    if !v.is_object() {
        return Err(format!("{} does not hold a JSON object: nothing was changed", path.display()));
    }
    Ok(Some(v))
}

fn entry_json(e: &Entry) -> Value {
    json!({"command": e.command, "args": e.args})
}

/// What `--apply` or `--remove` does to Claude Desktop's settings `v`: `Ok(None)` when nothing
/// changes, else the new settings. Everything but `mcpServers.vansoncraft` stays as it was.
fn claude_desktop_change(v: Option<Value>, entry: Option<&Entry>) -> Result<Option<Value>, String> {
    let mut v = v.unwrap_or_else(|| json!({}));
    let Some(root) = v.as_object_mut() else { return Err("the settings are not a JSON object".into()) };
    let servers = root.entry("mcpServers").or_insert_with(|| Value::Object(Map::new()));
    let Some(servers) = servers.as_object_mut() else { return Err("its `mcpServers` is not a JSON object: nothing was changed".into()) };
    match entry {
        Some(e) if servers.get(NAME) == Some(&entry_json(e)) => Ok(None),
        Some(e) => {
            servers.insert(NAME.into(), entry_json(e));
            Ok(Some(v))
        }
        None if servers.remove(NAME).is_none() => Ok(None),
        None => Ok(Some(v)),
    }
}

fn claude_desktop(entry: Option<&Entry>, config: Option<PathBuf>, write: bool) -> Result<(), String> {
    let path = config.or_else(claude_desktop_config).ok_or("can't tell where Claude Desktop keeps its settings: give --config FILE")?;
    if !write {
        let e = entry.ok_or("nothing to print")?;
        let snippet = serde_json::to_string_pretty(&json!({"mcpServers": {NAME: entry_json(e)}})).unwrap_or_default();
        outln!("Claude Desktop reads {}. Add this entry under \"mcpServers\" (keep the others):\n\n{snippet}\n", path.display());
        outln!("Or let setup write it (a backup is kept beside the file): add --apply to this command.");
        outln!("Then quit Claude Desktop completely and start it again.");
        return Ok(());
    }
    let before = read_json(&path)?;
    let others: Vec<String> = before
        .as_ref()
        .and_then(|v| v.get("mcpServers")?.as_object().cloned())
        .map(|s| s.iter().filter(|(k, v)| k.as_str() != NAME && runs_vansoncraft(v)).map(|(k, _)| k.clone()).collect())
        .unwrap_or_default();
    let existed = before.is_some();
    let Some(after) = claude_desktop_change(before, entry)? else {
        match entry {
            Some(_) => outln!("Claude Desktop already runs this VansonCraft as `{NAME}` ({}): nothing changed.", path.display()),
            None => outln!("Claude Desktop has no `{NAME}` server ({}): nothing changed.", path.display()),
        }
        return Ok(());
    };
    if existed {
        let backup = backup_name(&path);
        std::fs::copy(&path, &backup).map_err(|e| format!("can't back {} up: {e}; nothing was changed", path.display()))?;
        outln!("Backed up {} to {}", path.display(), backup.display());
    } else if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("can't make {}: {e}", dir.display()))?;
    }
    let mut text = serde_json::to_string_pretty(&after).map_err(|e| e.to_string())?;
    text.push('\n');
    vansoncraft_format::write_atomic(&path, text.as_bytes()).map_err(|e| format!("can't write {}: {e}", path.display()))?;
    match entry {
        Some(e) => outln!("Claude Desktop runs VansonCraft as `{NAME}`: {} {}", e.command, e.args.join(" ")),
        None => outln!("Removed `{NAME}` from {}.", path.display()),
    }
    outln!("Every other server and setting is as it was (the file now lists its keys in alphabetical order).");
    if !others.is_empty() {
        outln!("Note: {} also run(s) vansoncraft-cli; remove them if they are older copies of this entry.", others.join(", "));
    }
    outln!("Quit Claude Desktop completely and start it again to load it.");
    Ok(())
}

/// A backup name beside `path` that no file has yet: `<name>.vansoncraft-backup-<UTC time>[-n]`.
fn backup_name(path: &Path) -> PathBuf {
    let since = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let stamp = i64::try_from(since.as_secs())
        .ok()
        .and_then(|secs| chrono::DateTime::<chrono::Utc>::from_timestamp(secs, since.subsec_nanos()))
        .map(|t| t.format("%Y%m%d-%H%M%S%.3fZ").to_string())
        .unwrap_or_else(|| "unknown-time".into());
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let mut candidate = path.with_file_name(format!("{name}.vansoncraft-backup-{stamp}"));
    let mut n = 1;
    while candidate.exists() && n < 1000 {
        n += 1;
        candidate = path.with_file_name(format!("{name}.vansoncraft-backup-{stamp}-{n}"));
    }
    candidate
}

/// Whether a client's server entry runs a `vansoncraft-cli`.
fn runs_vansoncraft(v: &Value) -> bool {
    v.get("command").and_then(Value::as_str).is_some_and(|c| {
        let name = Path::new(c).file_stem().map(|s| s.to_string_lossy().to_lowercase()).unwrap_or_default();
        name == "vansoncraft-cli"
    })
}

// ---- doctor ------------------------------------------------------------------------------------

fn codex_config() -> Option<PathBuf> {
    let home = std::env::var_os("CODEX_HOME").filter(|v| !v.is_empty()).map(PathBuf::from).or_else(|| home_dir().map(|h| h.join(".codex")))?;
    Some(home.join("config.toml"))
}

fn home_dir() -> Option<PathBuf> {
    let var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    std::env::var_os(var).filter(|v| !v.is_empty()).map(PathBuf::from)
}

/// The `vansoncraft` entries of every client this machine has settings for, as
/// `(client, settings file, where in it, entry)`.
fn client_entries() -> Vec<(String, PathBuf, String, Result<Entry, String>)> {
    let mut found = vec![];
    let entry_of = |v: &Value| -> Result<Entry, String> {
        let command = v.get("command").and_then(Value::as_str).ok_or("it has no command")?.to_string();
        let args =
            v.get("args").and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_str).map(String::from).collect()).unwrap_or_default();
        Ok(Entry { command, args })
    };
    if let Some(path) = claude_desktop_config() {
        match read_json(&path) {
            Ok(Some(v)) => {
                if let Some(e) = v.get("mcpServers").and_then(|s| s.get(NAME)) {
                    found.push(("Claude Desktop".into(), path, format!("mcpServers.{NAME}"), entry_of(e)));
                }
            }
            Ok(None) => {}
            Err(e) => found.push(("Claude Desktop".into(), path, String::new(), Err(e))),
        }
    }
    if let Some(path) = home_dir().map(|h| h.join(".claude.json")) {
        match read_json(&path) {
            Ok(Some(v)) => {
                if let Some(e) = v.get("mcpServers").and_then(|s| s.get(NAME)) {
                    found.push(("Claude Code (user)".into(), path.clone(), format!("mcpServers.{NAME}"), entry_of(e)));
                }
                for (project, p) in v.get("projects").and_then(Value::as_object).into_iter().flatten() {
                    if let Some(e) = p.get("mcpServers").and_then(|s| s.get(NAME)) {
                        found.push(("Claude Code (local)".into(), path.clone(), format!("projects[{project}].mcpServers.{NAME}"), entry_of(e)));
                    }
                }
            }
            Ok(None) => {}
            Err(e) => found.push(("Claude Code".into(), path, String::new(), Err(e))),
        }
    }
    if let Some(path) = codex_config() {
        let too_large = std::fs::metadata(&path).is_ok_and(|m| m.len() > MAX_CONFIG_BYTES);
        let read = if too_large { Err(std::io::Error::other("larger than 64 MB")) } else { std::fs::read_to_string(&path) };
        match read {
            Ok(text) => match text.parse::<toml::Table>() {
                Ok(t) => {
                    if let Some(e) = t.get("mcp_servers").and_then(|s| s.get(NAME)) {
                        let v = serde_json::to_value(e).unwrap_or(Value::Null);
                        found.push(("Codex".into(), path, format!("[mcp_servers.{NAME}]"), entry_of(&v)));
                    }
                }
                Err(e) => found.push(("Codex".into(), path, String::new(), Err(format!("not valid TOML: {e}")))),
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => found.push(("Codex".into(), path, String::new(), Err(e.to_string()))),
        }
    }
    found
}

/// The program a client entry's `command` starts: the path itself, or for a bare name the first
/// match on `PATH` (with `.exe` on Windows), as the client looks it up.
fn resolve(command: &str) -> Option<PathBuf> {
    let p = Path::new(command);
    if p.components().count() > 1 || p.is_absolute() {
        return p.is_file().then(|| p.to_path_buf());
    }
    let names: Vec<String> =
        if cfg!(windows) && p.extension().is_none() { vec![format!("{command}.exe"), command.to_string()] } else { vec![command.to_string()] };
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path).flat_map(|dir| names.iter().map(move |n| dir.join(n))).find(|c| c.is_file())
}

/// One line of the report.
struct Check {
    level: &'static str,
    what: String,
}

pub fn doctor(args: &[String]) -> Result<(), String> {
    let mut addr = DEFAULT_ADDR.to_string();
    let mut as_json = false;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--connect" => addr = it.next().cloned().ok_or("--connect needs an address")?,
            "--json" => as_json = true,
            "-h" | "--help" => {
                outln!(
                    "vansoncraft-cli doctor [--connect ADDR] [--json]\n  Check this copy of VansonCraft for use by an AI agent: the program, the desktop app beside it, the headless engine, the app on ADDR (default {DEFAULT_ADDR}) and the MCP entries of Claude Desktop, Claude Code and Codex."
                );
                return Ok(());
            }
            other => return Err(format!("unknown doctor option `{other}`")),
        }
    }
    let version = env!("CARGO_PKG_VERSION");
    let mut checks: Vec<Check> = vec![];
    let mut add = |level: &'static str, what: String| checks.push(Check { level, what });
    add("ok", format!("vansoncraft-cli {version}, {} {}", std::env::consts::OS, std::env::consts::ARCH));
    let exe = std::env::current_exe().ok();
    match &exe {
        Some(p) => match unstable_place(p) {
            Some(why) => add("warn", format!("runs from {} ({why}): set clients up from an installed copy", p.display())),
            None => add("ok", format!("installed at {}", p.display())),
        },
        None => add("warn", "can't tell where this program is".into()),
    }
    let app = exe.as_ref().and_then(|p| p.parent()).map(|d| d.join(if cfg!(windows) { "vansoncraft.exe" } else { "vansoncraft" }));
    match app.filter(|a| a.is_file()) {
        Some(a) => add("ok", format!("the desktop app is beside it: {}", a.display())),
        None => add("info", "no desktop app beside this program (headless use only)".into()),
    }

    let t = Instant::now();
    let mut h = Headless::with_document();
    let drew = h
        .call("engine.execute", json!({"command": "shape.rectangle", "params": {"x": 10, "y": 10, "width": 80, "height": 40}}))
        .and_then(|_| h.call("ui.render", json!({"scale": 0.25})));
    match drew {
        Ok(r) if r.get("pngBase64").and_then(Value::as_str).is_some_and(|s| !s.is_empty()) => {
            add("ok", format!("the headless engine drew and previewed a page in {} ms", t.elapsed().as_millis()))
        }
        Ok(_) => add("fail", "the headless engine drew, but its preview came back empty".into()),
        Err(e) => add("fail", format!("the headless engine failed: {e}")),
    }

    match Remote::connect_verified(&addr) {
        Ok(r) => {
            let me = r.identity().cloned().unwrap_or(Value::Null);
            let app_version = me.get("version").and_then(Value::as_str).unwrap_or("?");
            add("ok", format!("VansonCraft {app_version} (process {}) answers on {addr}: desktop mode works", me.get("pid").unwrap_or(&Value::Null)));
            if app_version != version {
                add("warn", format!("the app is version {app_version} and this program {version}: use the same version for both"));
            }
        }
        Err(e) if e.contains("not VansonCraft") => add("fail", e),
        Err(e) if e.contains("did not say what it is") || e.contains("refused the connection") => add("warn", e),
        Err(e) => add(
            "info",
            format!("nothing answers on {addr} ({e}): for desktop mode start `vansoncraft --control {}`", addr.rsplit(':').next().unwrap_or("7979")),
        ),
    }

    let entries = client_entries();
    if entries.is_empty() {
        add("info", "no client has a `vansoncraft` MCP server yet: run `vansoncraft-cli setup claude-desktop|claude-code|codex`".into());
    }
    for (client, file, place, e) in entries {
        match e {
            Err(why) => add("warn", format!("{client}: {} {place}: {why}", file.display())),
            Ok(e) if resolve(&e.command).is_none() => {
                add("fail", format!("{client}: {} {place} runs {}, which does not exist: run setup again", file.display(), e.command))
            }
            Ok(e) => {
                let program = resolve(&e.command).unwrap_or_else(|| PathBuf::from(&e.command));
                let real = |p: &Path| std::fs::canonicalize(p).ok();
                let same = exe.as_ref().is_some_and(|me| program == *me || real(&program).is_some_and(|p| Some(p) == real(me)));
                let mode = if e.args.iter().any(|a| a == "--connect") { "desktop" } else { "headless" };
                add(
                    if same { "ok" } else { "info" },
                    format!(
                        "{client}: {} {place} runs {}{} ({mode}): {}",
                        file.display(),
                        e.command,
                        if same { " (this program)" } else { "" },
                        e.args.join(" ")
                    ),
                );
            }
        }
    }

    let failed = checks.iter().any(|c| c.level == "fail");
    if as_json {
        let list: Vec<Value> = checks.iter().map(|c| json!({"level": c.level, "check": c.what})).collect();
        outln!("{}", serde_json::to_string_pretty(&json!({"ok": !failed, "checks": list})).unwrap_or_default());
    } else {
        for c in &checks {
            outln!("{:<5}{}", c.level, c.what);
        }
    }
    if failed { Err("some checks failed (see above)".into()) } else { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(args: &[&str]) -> Entry {
        Entry { command: "/opt/vansoncraft/bin/vansoncraft-cli".into(), args: args.iter().map(|s| s.to_string()).collect() }
    }

    #[test]
    fn setup_keeps_every_other_setting_and_never_adds_a_second_entry() {
        let before = json!({"globalShortcut": "Ctrl+Space", "mcpServers": {"files": {"command": "npx", "args": ["-y", "x"]}}});
        let one = e(&["mcp", "--connect", "127.0.0.1:7979"]);
        let after = claude_desktop_change(Some(before.clone()), Some(&one)).unwrap().unwrap();
        assert_eq!(after["globalShortcut"], "Ctrl+Space");
        assert_eq!(after["mcpServers"]["files"], before["mcpServers"]["files"]);
        assert_eq!(after["mcpServers"][NAME], json!({"command": one.command, "args": one.args}));
        // The same entry again: nothing to write.
        assert!(claude_desktop_change(Some(after.clone()), Some(&one)).unwrap().is_none());
        // Another mode replaces it; there is still one.
        let two = e(&["mcp", "--headless"]);
        let again = claude_desktop_change(Some(after.clone()), Some(&two)).unwrap().unwrap();
        assert_eq!(again["mcpServers"].as_object().unwrap().len(), 2);
        assert_eq!(again["mcpServers"][NAME]["args"], json!(["mcp", "--headless"]));
        // Removing takes out only VansonCraft's.
        let removed = claude_desktop_change(Some(again), None).unwrap().unwrap();
        assert_eq!(removed["mcpServers"], before["mcpServers"]);
        assert_eq!(removed["globalShortcut"], "Ctrl+Space");
        assert!(claude_desktop_change(Some(removed), None).unwrap().is_none(), "nothing left to remove");
        // No file yet, and a file of something else.
        assert_eq!(claude_desktop_change(None, Some(&one)).unwrap().unwrap()["mcpServers"][NAME]["command"], one.command.as_str());
        assert!(claude_desktop_change(Some(json!({"mcpServers": []})), Some(&one)).is_err());
    }

    #[test]
    fn apply_backs_up_and_writes_the_file_and_refuses_one_it_cannot_read() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("claude_desktop_config.json");
        std::fs::write(&path, r#"{"mcpServers": {"files": {"command": "npx"}}, "theme": "dark"}"#).unwrap();
        let one = e(&["mcp", "--connect", "127.0.0.1:7979"]);
        claude_desktop(Some(&one), Some(path.clone()), true).unwrap();
        let written: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(written["theme"], "dark");
        assert_eq!(written["mcpServers"]["files"]["command"], "npx");
        assert_eq!(written["mcpServers"][NAME]["args"], json!(one.args));
        let backups: Vec<_> =
            std::fs::read_dir(dir.path()).unwrap().flatten().filter(|f| f.file_name().to_string_lossy().contains("vansoncraft-backup")).collect();
        assert_eq!(backups.len(), 1);
        assert!(std::fs::read_to_string(backups[0].path()).unwrap().contains("\"theme\": \"dark\""), "the backup is the file as it was");
        claude_desktop(None, Some(path.clone()), true).unwrap();
        let removed: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert!(removed["mcpServers"].get(NAME).is_none() && removed["mcpServers"]["files"]["command"] == "npx");
        // A file that isn't JSON is left alone.
        std::fs::write(&path, "{ not json").unwrap();
        let err = claude_desktop(Some(&one), Some(path.clone()), true).unwrap_err();
        assert!(err.contains("not valid JSON") && err.contains("nothing was changed"), "{err}");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "{ not json");
    }

    #[test]
    fn a_build_folder_or_a_temporary_folder_is_no_place_to_set_up_from() {
        assert!(unstable_place(Path::new("/home/a/vansoncraft/target/release/vansoncraft-cli")).is_some());
        assert!(unstable_place(Path::new("C:/src/vansoncraft/target/debug/vansoncraft-cli.exe")).is_some());
        assert!(unstable_place(&std::env::temp_dir().join("x").join("vansoncraft-cli")).is_some());
        assert!(unstable_place(Path::new("/tmp/.mount_vansonXYZ/usr/bin/vansoncraft-cli")).is_some());
        if std::env::var_os("APPIMAGE").is_none() {
            assert!(unstable_place(Path::new("/opt/vansoncraft/bin/vansoncraft-cli")).is_none());
        }
    }

    #[test]
    fn printed_commands_and_codex_tables_quote_what_needs_it() {
        let spaced = Entry {
            command: "C:\\Users\\Thu Ha\\AppData\\Local\\Programs\\VansonCraft\\vansoncraft-cli.exe".into(),
            args: vec!["mcp".into(), "--connect".into(), "127.0.0.1:7979".into()],
        };
        let toml = toml_entry(&spaced);
        let parsed: toml::Table = toml.parse().unwrap();
        assert_eq!(parsed["mcp_servers"][NAME]["command"].as_str(), Some(spaced.command.as_str()), "{toml}");
        assert_eq!(parsed["mcp_servers"][NAME]["args"][2].as_str(), Some("127.0.0.1:7979"));
        if cfg!(windows) {
            assert_eq!(quote(&spaced.command), format!("\"{}\"", spaced.command));
        } else {
            assert_eq!(quote("/home/Thu Ha/bin/vansoncraft-cli"), "'/home/Thu Ha/bin/vansoncraft-cli'");
            assert_eq!(quote("it's"), "'it'\\''s'");
        }
        assert_eq!(quote("127.0.0.1:7979"), "127.0.0.1:7979");
        if !cfg!(windows) {
            assert_eq!(quote("a\\b"), "'a\\b'", "a POSIX shell would eat the backslash");
        }
        // A path with an apostrophe, a quote and a control character still makes valid TOML.
        let odd = Entry { command: "/home/Thư's \"bin\"/x\u{1}/vansoncraft-cli".into(), args: vec!["mcp".into()] };
        let parsed: toml::Table = toml_entry(&odd).parse().unwrap();
        assert_eq!(parsed["mcp_servers"][NAME]["command"].as_str(), Some(odd.command.as_str()));
    }

    #[test]
    fn a_cargo_output_folder_is_found_whatever_it_is_called() {
        let dir = tempfile::tempdir().unwrap();
        let profile = dir.path().join("agent-builds").join("release");
        std::fs::create_dir_all(profile.join(".fingerprint")).unwrap();
        assert_eq!(unstable_place(&profile.join("vansoncraft-cli")), Some("a cargo build folder"));
        let tagged = dir.path().join("out");
        std::fs::create_dir_all(tagged.join("x86_64-pc-windows-msvc").join("release")).unwrap();
        std::fs::write(tagged.join("CACHEDIR.TAG"), "Signature: 8a477f597d28d172789f06886806bc55").unwrap();
        assert_eq!(unstable_place(&tagged.join("x86_64-pc-windows-msvc").join("release").join("vansoncraft-cli.exe")), Some("a cargo build folder"));
    }

    #[test]
    fn backups_never_overwrite_one_another_and_bare_names_are_looked_up_on_path() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("claude_desktop_config.json");
        let first = backup_name(&file);
        std::fs::write(&first, "1").unwrap();
        let second = backup_name(&file);
        assert_ne!(first, second);
        assert!(first.file_name().unwrap().to_string_lossy().starts_with("claude_desktop_config.json.vansoncraft-backup-"));
        assert!(resolve("no-such-program-anywhere-vansoncraft").is_none());
        assert!(resolve(&file.display().to_string()).is_none(), "a path that isn't a file");
        assert_eq!(resolve(&first.display().to_string()), Some(first.clone()));
    }
}
