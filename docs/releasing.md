# Releasing VansonCraft

> **Last updated:** 2026-11-13 · **Change:** major (the release workflow, `.github/workflows/release.yml`:
> Windows x64 and Linux x86_64, checked on fresh runners before and after publishing)

User-facing names say **VansonCraft**. Files, binaries and ids stay lowercase
(`vansoncraft-<version>-<platform>-<arch>.<ext>`, `io.github.vansyson1308.vansoncraft`).

## Releasing

1. **Bump the version** on `main`, in a pull request. It lives in exactly one place,
   `[workspace.package] version` in the root `Cargo.toml`; every crate inherits it and the packaging
   scripts read it from there. Keep counting from 0.24.1, the version the fork started at: files
   record the version that wrote them, and readers rely on it (`vansoncraft_doc::twist`).

   ```sh
   cargo xtask version                 # prints the current version
   cargo xtask version set 0.25.1      # updates Cargo.toml and Cargo.lock
   ```

2. **Run the release workflow** from `main`: Actions › Release › Run workflow, with **publish**
   ticked (and **prerelease** as wanted). Unticked, the same run builds and checks the packages and
   uploads nothing, which is also what every pull request that changes the workflow or the packaging
   scripts runs.
3. **Wait for it to finish.** The release is public only once every check passed; then
   `verify-public` downloads the files again from the release page without a token and runs them.

A published version is never changed: the workflow refuses to publish a version that is already
published, or whose tag already exists, and a stable release must be newer than the latest one.
A fix ships as a new version.

The repository setting **Immutable releases** (Settings › General › Releases) makes GitHub itself
refuse changes to a published release's files and tag. The workflow is built for it: it attaches
every file while the release is a draft. Turning it on is the maintainer's decision.

### What the workflow does

`.github/workflows/release.yml`, every job on GitHub-hosted runners (free for this public
repository; a full run takes about 1.5 to 2 hours of runner time):

| Job | Runner | Permissions | What it does |
|---|---|---|---|
| `source` | ubuntu-24.04 | read | the version from `Cargo.toml`, the commit. A publishing run must be on `main`, with no published release and no tag of its version; a stable one must be newer than the latest release |
| `windows-x64` | windows-2025 | read | `packaging/windows/package.ps1 -Arch x64` from that commit with Rust `RUST_TOOLCHAIN` (pinned in the workflow) and `--locked`: the MSI and the portable ZIP, both with `vansoncraft.exe` and `vansoncraft-cli.exe`. The script checks both executables are x64 (PE machine) and GUI/console as they should be. The job then checks (`dumpbin /dependents`) that neither imports a C runtime DLL (`vcruntime*`, `msvcp*`, `ucrtbase`, `api-ms-win-crt-*`): the runner has the Visual C++ runtimes, a user's Windows may not |
| `linux-x86_64` | ubuntu-22.04 | read | `packaging/linux/package.sh --formats "tar appimage"`: the tar.gz and the AppImage; checks both executables are x86-64 ELF, prints `ldd` and the newest glibc symbol they need (2.35) |
| `stage-dry` / `stage-draft` | ubuntu-24.04 | read / **write** (publishing runs only) | `packaging/release/stage.sh`: fails unless exactly the expected files are there, writes `SHA256SUMS.txt`, `BUILDINFO.json` (version, commit, workflow run, toolchain, each file's OS, architecture, size, SHA-256, signing) and the notes. A publishing run uploads everything to a **draft** release (replacing an earlier draft of that version), downloads it back and checks it against `SHA256SUMS.txt` and GitHub's own digests |
| `smoke-windows-portable` | windows-2025 | read | unpacks the ZIP into a folder whose name has spaces and Vietnamese letters, runs the CLI, `packaging/smoke/mcp_smoke.py` (MCP: tools, drawing, preview, a change to one object, save and export, a write outside the allowed folder refused, `--connect` to a closed port failing), `setup`/`doctor`, and `desktop.ps1` (the app opens, the MCP bridge edits its window's document, cold and warm start) |
| `smoke-windows-msi` | windows-2025 | read | installs VectorCraft 0.24.1 first (its MSI pinned by SHA-256), then the MSI for the current user. Checks: the programs, the uninstall and App Paths entries, the Start menu and desktop shortcuts; the same MCP, setup and desktop checks on the installed copy; the same file installed again (a maintenance run); uninstalling (no files, entries or shortcuts left); VectorCraft unchanged before and after |
| `smoke-linux` | ubuntu-24.04 | read | `packaging/smoke/linux.sh`: the tar.gz unpacked under a Vietnamese folder name, the same checks with the app under Xvfb, the AppImage's `--version` |
| `publish` | ubuntu-24.04 | **write** | only when every smoke job passed: checks the draft still holds exactly the tested files (names and digests) at the tested commit, publishes it (as a pre-release, or as the latest release), and checks that the tag it created names the tested commit |
| `verify-public` | ubuntu-24.04, windows-2025 | none | downloads the files from the public release URLs, checks them against the published `SHA256SUMS.txt`, runs the CLI and the MCP check, and checks where `/releases/latest` leads |

The smoke jobs run on fresh GitHub-hosted runners and use only the package and Python's standard
library.
- Each one first removes the image's Rust toolchain (`~/.cargo`, `~/.rustup`) and fails if `cargo`,
  `rustc` or `rustup` is still found.
- The only repository files they fetch are the scripts in `packaging/smoke/`.
- They test the bytes the draft holds (the stage job downloads them back and hands them on), so
  what is published is what was tested.

The images are not bare machines, though: they carry Python, Visual Studio and the Visual C++
runtimes, which is why the Windows build checks the executables' imports.

Pull requests that change the workflow or the packaging run the same builds and smoke jobs as a dry
run, with read-only tokens.

Between jobs, the packages travel as workflow artifacts kept for one or two days. They are not a
way to get VansonCraft: releases are. The smoke jobs' uploads (reports, logs, screenshots, the
drawings made in the test) hold no program files.

**Tokens:** only `stage-draft` and `publish` get `contents: write`, and only their steps that call
the API hold the token, never an action. `actions/upload-artifact` and `actions/download-artifact`
are referenced by their major tag (`@v4`) and kept current by Dependabot. They were not pinned to a
commit because the commit for the tag could not be confirmed when the workflow was written.

**Signing:** none. VansonCraft has no code-signing certificates, so the packages are unsigned: the
release notes say so, SmartScreen may warn that the publisher is unknown, and the in-app updater,
which installs only signed Windows updates, gives a download link instead. Add signing (below) when
certificates exist; never describe an unsigned release as signed.

**Pre-releases:** a release published as a pre-release is not the repository's "latest" (so
`/releases/latest` does not lead to it) and the in-app updater never offers it. Link to it by its
version: `https://github.com/vansyson1308/vansoncraft/releases/tag/v<version>` and
`https://github.com/vansyson1308/vansoncraft/releases/download/v<version>/<file>`.

**What the in-app updater does with a release:** it reads this repository's published releases,
ignores drafts and pre-releases, and offers the newest stable version above the running one. An
unsigned Windows copy and a Linux tar.gz install get a download link; an AppImage replaces itself.
macOS and Flatpak installs are not recognised by the updater (`craft-update` builds their ids as
`ai.storyteller.<id>`, see `docs/independence.md`) and get the download link too.

## Platforms

The first releases cover **Windows x64** (MSI, portable ZIP) and **Linux x86_64** (tar.gz,
AppImage), the platforms the workflow builds and checks. The packaging scripts for the others below
exist but have no release job yet: a platform is added to the workflow with smoke tests of its own,
never on the strength of a cross-compile alone.

## What gets built

What each platform's packaging script makes. The release workflow runs the Windows x64 script and
the Linux x86_64 script (tar.gz and AppImage) so far; the jobs named below for the other platforms
(ARM64, Flatpak, FreeBSD, the web build, deb and rpm) are what a release job for them would do, not
jobs that run today.

| Platform | Artifacts | Built on |
|---|---|---|
| macOS 11+ (universal: Apple silicon and Intel) | `vansoncraft-<v>-macos-universal.dmg`, `vansoncraft-cli-<v>-macos-universal.zip` | `macos-15` |
| Windows x64 | `vansoncraft-<v>-windows-x64.msi`, `vansoncraft-<v>-windows-x64-portable.zip` | `windows-2025` |
| Windows x86 (32-bit) | `vansoncraft-<v>-windows-x86.msi`, `vansoncraft-<v>-windows-x86-portable.zip` | `windows-latest` |
| Windows on ARM64 | `vansoncraft-<v>-windows-arm64.msi`, `vansoncraft-<v>-windows-arm64-portable.zip` | `windows-latest` (cross-compiled) |
| Linux x86_64 | `vansoncraft-<v>-linux-x86_64.{AppImage,deb,rpm,tar.gz}` | `ubuntu-22.04` |
| Linux aarch64 | `vansoncraft-<v>-linux-aarch64.{AppImage,deb,rpm,tar.gz}` | `ubuntu-22.04-arm` |
| Linux riscv64 | `vansoncraft-<v>-linux-riscv64.tar.gz` (cross-compiled, glibc >= 2.39; CLI smoke-tested under QEMU) | `ubuntu-24.04` |
| Linux AppImage updates | `vansoncraft-<v>-linux-{x86_64,aarch64}.AppImage.zsync` | with the AppImages |
| Flatpak x86_64, aarch64 | `vansoncraft-<v>-linux-{x86_64,aarch64}.flatpak` | `ubuntu-24.04`, `ubuntu-24.04-arm` (repackages the Linux tarball) |
| FreeBSD 14 x86_64 | `vansoncraft-<v>-freebsd-x86_64.tar.gz` | a FreeBSD 14.3 VM on `ubuntu-latest` |
| Web | `vansoncraft-web-<v>.zip`, a static site (see [`packaging/web/README.md`](../packaging/web/README.md)) | `ubuntu-latest` |

`vansoncraft --version` and `vansoncraft-cli --version` print the version from `Cargo.toml`.

**Fonts.** A build can embed the Japanese fonts of the separate font collection
[craft-fonts](https://github.com/storytold/craft-fonts), an optional build input: check it out at a
pinned commit and build with `CRAFT_FONTS_DIR` and `CRAFT_FONTS_REQUIRED=1`, which fails rather than
building without them (the web build embeds only the UI font, BIZ UDPGothic Regular, to stay small). The packages carry each embedded font's licence as `OFL-<family>.txt`
(`copy_font_licences` in `packaging/env.sh`).
The build option is in
[`docs/development.md`](development.md) › Fonts.

### macOS

`packaging/macos/package.sh` builds `aarch64-apple-darwin` and `x86_64-apple-darwin` with
`MACOSX_DEPLOYMENT_TARGET=11.0`, joins them with `lipo` and assembles `VansonCraft.app`:

- `Info.plist` is generated from `Info.plist.in` (bundle id `io.github.vansyson1308.vansoncraft`,
  `LSMinimumSystemVersion` 11.0, the version and the build commit). The icon is
  `assets/app-icon/vansoncraft.icns`. `cargo xtask bundle` fills in the same template for its
  development app.
- **Document types:** `Info.plist.in` declares every format File › Open reads, so Finder opens
  them with VansonCraft (double-click, Open With, a drop on the Dock icon). VansonCraft owns its own
  formats and is an alternate for the others. A test keeps the list in step with the engine's
  `OPEN_EXTS`. AppKit hands the files to the app as open-document events, which
  `apps/vansoncraft/src/open_documents.rs` opens like files named on the command line.
- **Signing** goes inside-out with the hardened runtime and a secure timestamp: the executable
  first, then the bundle, with no `--deep` on the final signature. The entitlements
  (`entitlements.plist`) are deliberately empty.
- **Notarization:** the app is zipped and sent with `xcrun notarytool submit --wait`, then the
  ticket is stapled and checked with `stapler validate` and `spctl`. The app goes on a DMG
  (`hdiutil create -srcfolder`, retried when a CI runner's device is busy, with an `Applications` link
  to drag onto), which is signed, notarized and stapled too; the app mounted from it must pass
  `codesign --verify --strict --deep` (#1085). Its Finder window (background, icon size and positions) comes from
  [`packaging/macos/dmg/`](../packaging/macos/dmg/README.md), and its volume is named `VansonCraft`
  without the version, which the window's background needs; the DMG file name keeps the version.
- **CLI:** the universal `vansoncraft-cli` is signed the same way, zipped, and the zip is
  notarized. A bare executable can't hold a stapled ticket, so Gatekeeper looks the CLI's ticket up
  online the first time a downloaded copy runs.

Locally, without certificates, the script signs ad hoc (`codesign -s -`) and skips notarization,
which is enough to check the bundle and the DMG on your own Mac:

```sh
packaging/macos/package.sh --arch universal   # needs both rustup targets
packaging/macos/package.sh --arch aarch64     # quicker, host only
```

### Windows

`packaging/windows/package.ps1 -Arch x64|x86|arm64` builds with `-C target-feature=+crt-static`,
so neither the MSI nor the portable zip needs the Visual C++ redistributable. The flag is scoped
to the target triple, so host build scripts aren't affected.

- Before packaging, the script checks each binary's PE machine type against `-Arch` (an x64 build
  can never ship labelled arm64), that `vansoncraft.exe` is a GUI program (no console window) and
  that `vansoncraft-cli.exe` is a console program.
- **Portable zip:** it ships `packaging/windows/portable.txt` beside `vansoncraft.exe`. That
  marker (or a `VansonCraft.portable` file) switches on portable mode: `ui.json`
  and the folders next to it (Swatches, Graphic Styles, Fonts, Data Recovery, logs…) go to
  `VansonCraftData\` next to the exe instead of `%APPDATA%\VansonCraft`. The first time
  `VansonCraftData` is created, the settings already in `%APPDATA%\VansonCraft` are copied into it
  (the originals stay), so someone updating a portable zip from before the marker keeps their
  settings. If that folder isn't writable the app logs a warning and uses `%APPDATA%`. Deleting
  `portable.txt` returns settings to the user profile (the `VansonCraftData` folder is left as it
  is). The MSI has no marker. `VANSONCRAFT_CONFIG_DIR` still wins, `VANSONCRAFT_NO_PREFS` never
  looks for the marker, and a portable copy skips the one-time read of the settings saved under the project's earlier names (VectorCraft, DrawCraft).
  The logic is in `apps/vansoncraft/src/profile.rs`. On the other platforms the marker goes where
  the user sees the app, never inside it: on macOS next to `VansonCraft.app` (nothing is written
  inside the signed bundle), for an AppImage next to the `.AppImage` file (`$APPIMAGE`; the mounted
  executable's own folder is read-only). Flatpak isn't supported: `/app` is read-only and the
  sandbox has no folder "beside the app", so a Flatpak install always uses its own config folder.
- `vansoncraft.wxs` (WiX) is a dual-scope package (per-user by default into
  `%LOCALAPPDATA%\Programs\VansonCraft`, per-machine into `Program Files\VansonCraft` (`Program Files (x86)` for the 32-bit build),
  see `packaging/windows/INSTALLATION.md`) with a Start Menu shortcut
  and an App Paths entry (Win+R `vansoncraft`). Same-version upgrades are allowed, so release
  candidates replace each other.
- Double-clicking the MSI opens a setup wizard (WixUI_InstallDir without the licence page; the scope page appears only for a new full-UI install, not for
  maintenance, uninstall or reduced and silent runs):
  a scope page ("Only for me" or "Everyone on this computer"), Welcome, install folder (remembered for
  upgrades in `HKLM` or `HKCU\Software\VansonCraft\InstallDir` according to scope),
  Shortcuts (a ticked "Create a desktop shortcut" box), Ready, a progress page, and a Finish page
  that says where to find the app, with a ticked "Launch VansonCraft" box. The pages name the full
  version (`vansoncraft.en-us.wxl`). `package.ps1` draws the wizard's banner and side bitmaps from
  the app icon, so no WiX stock art ships. `msiexec /qn` still installs silently, with the desktop
  shortcut unless `INSTALLDESKTOPSHORTCUT=""` is passed. Building needs the WiX UI and Util
  extensions: `wix extension add -g WixToolset.UI.wixext/5.0.2 WixToolset.Util.wixext/5.0.2`.
- The ARM64 build is cross-compiled on the x64 runner. A release job for it would install that MSI
  on a Windows 11 ARM64 runner, check that both installed programs are ARM64, run
  `vansoncraft-cli --version` natively and uninstall (no such job yet).
- **Signing:** `packaging/windows/sign.ps1` signs both `.exe` files and then the `.msi` with
  `signtool` (SHA-256, RFC 3161 timestamp), using whichever material is present:
  1. a `.pfx` certificate (`WINDOWS_CERTIFICATE`, base64, and `WINDOWS_CERTIFICATE_PASSWORD`), or
  2. Azure Trusted Signing (`AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`,
     `AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT`, `AZURE_CERT_PROFILE`).

  With neither, it warns and leaves the files unsigned, so test builds still produce installers.
  It is the one place to adapt when the signing setup changes; `WINDOWS_TIMESTAMP_URL` and
  `SIGNTOOL` override the timestamp server and the `signtool.exe` path.

### Linux

`packaging/linux/package.sh` builds the release binaries and packages them as an AppImage, a
`.deb` and an `.rpm` (with [nfpm](https://nfpm.goreleaser.com), from `nfpm.yaml`) and a plain
`.tar.gz` tree (`bin/`, `share/`). The packages install both programs, the desktop entry, the
AppStream metainfo, the MIME type for VansonCraft documents, the icons and the licence files.

**Document types.** The desktop entry's `MimeType` lists every format File › Open reads but PSD,
PSB, `.ps` PostScript, Targa, PCX, Pixar, JPEG 2000, HEIC/HEIF and HP-GL, so file managers list VansonCraft under Open With for them.
`io.github.vansyson1308.vansoncraft.mime.xml` declares the types that shared-mime-info lacks: VansonCraft
documents and templates, `.ait` templates and Affinity files (`application/x-affinity`). An `.af`
file gets the Affinity type by its content, and `.afdesign` and `.afpub` files by their extension.
A test keeps both files in step with the engine's `OPEN_EXTS`.

The jobs run on `ubuntu-22.04`, the oldest GitHub-hosted image, so the binaries only need
glibc 2.35 or newer: Ubuntu 22.04+, Debian 12+, Fedora 36+ and RHEL 10. Moving the job to a newer
image raises that floor, so do it deliberately. After packaging, the job prints the `.deb`'s
metadata and contents, runs `ldd` on the binary and runs each AppImage with `--version`.

**RPM dependencies.** The x86_64 and aarch64 RPMs require 64-bit shared-library
capabilities rather than distribution-specific package names: `libc`, libm's `GLIBC_2.35`,
`libgcc_s`, `libxkbcommon`, and either Vulkan or EGL. Windowing and GPU
libraries are loaded at runtime, so `ldd` alone does not list every requirement.
The glibc capability enforces the same floor as the `.deb`; a distribution with
an older glibc is not supported by these binaries. It names libm, whose `hypot` and `hypotf`
are the binaries' GLIBC_2.35 symbols: RHEL 9's glibc 2.34 provides `libc.so.6(GLIBC_2.35)`
through backports, so a libc capability let it install binaries that can't start. Weak
recommendations still use package names and do not block installation.

Before publishing a Linux release:

- [ ] Check `rpm -qp --requires` on both architectures for the runtime capabilities
  above, including the Vulkan/EGL alternative and glibc floor.
- [ ] Install and start the package on the target RPM distributions. Synthetic
  provider tests verify metadata resolution, not a real desktop installation.

**RISC-V.** The `linux-riscv64` job cross-compiles `riscv64gc-unknown-linux-gnu` on `ubuntu-24.04`
against riscv64 libraries from ports.ubuntu.com (so the floor there is glibc 2.39) and runs
`package.sh --formats tar` with `CROSS_ARCH`, `CROSS_TARGET`, `CROSS_COMPILE` and
`EMULATOR=qemu-riscv64` set: a `.tar.gz` only, with `vansoncraft-cli --version` run under QEMU.

**AppImage updates.** Each AppImage embeds the update information
`gh-releases-zsync|vansyson1308|vansoncraft|latest|vansoncraft-*-linux-<arch>.AppImage.zsync`
(appimagetool `-u`), and `package.sh` writes the matching `.zsync` beside it when `zsyncmake` (the
`zsync` package) is installed. [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate)
and AppImageLauncher then download only the changed blocks from the newest published,
non-pre-release release. The release workflow requires the AppImage and publishes the `.zsync`
when `package.sh` made it (it installs `zsync` for that); a release without it still works, and
AppImageUpdate then downloads the whole file.

**Flatpak.** The `flatpak` jobs (x86_64 on `ubuntu-24.04`, aarch64 on `ubuntu-24.04-arm`) take the
Linux job's tarball and repackage it with `packaging/linux/flatpak-bundle.sh` and
`packaging/linux/flatpak/io.github.vansyson1308.vansoncraft.bundle.yml` into a single-file bundle
(`flatpak install --user vansoncraft-<v>-linux-<arch>.flatpak`), then install it and run
`vansoncraft-cli --version` in the sandbox. No Rust build happens there, so the bundle carries the
same binaries as the AppImage. `packaging/linux/flatpak/io.github.vansyson1308.vansoncraft.yml` is the
from-source manifest for a Flathub submission (its header says how to build it); packaging-lint
keeps the runtime and sandbox permissions of the two manifests identical.

### FreeBSD

GitHub has no FreeBSD runners, so the job builds in a FreeBSD 14.3 VM on `ubuntu-latest` (a
FreeBSD VM action). `packaging/freebsd/package.sh` writes a
`/usr/local`-style tree; users install it with
`tar -xzf vansoncraft-*-freebsd-x86_64.tar.gz --strip-components 1 -C /usr/local`.

### Web

The web job builds `apps/vansoncraft-web` with trunk and zips the static site
(`packaging/web/package.sh`). [`packaging/web/README.md`](../packaging/web/README.md) covers
hosting: any path works, `.wasm` must be served as `application/wasm`, and the sample header
files for common hosts.

## Secrets

When signing is added, give the signing secrets only to the jobs that sign (macOS, Windows), through
a protected environment. The packaging scripts treat
every secret as optional: a missing one produces unsigned artifacts and a warning, never a failed
build.

| Secret | Used for |
|---|---|
| `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `KEYCHAIN_PASSWORD` | the Developer ID certificate (base64 `.p12`), imported into a temporary keychain by `packaging/macos/import-cert.sh` |
| `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID` | notarization (`APPLE_PASSWORD` is an app-specific password) |
| `WINDOWS_CERTIFICATE`, `WINDOWS_CERTIFICATE_PASSWORD` | Windows signing with a `.pfx` |
| `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT`, `AZURE_CERT_PROFILE` | Windows signing with Azure Trusted Signing |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-13 | major | The release workflow: Windows x64 (MSI, portable ZIP) and Linux x86_64 (tar.gz, AppImage), built from one commit with a pinned toolchain, staged as a draft, smoke-tested on fresh runners without the Rust toolchain, published, then downloaded again from the public URLs and checked. A version is published once (no published release, no tag); a stable one must be newer than the latest; the write token only in the steps that call the API; the Windows executables import no C runtime DLL. |
| 2026-11-12 | major | VansonCraft is independent: no release pipeline in the repository yet (the inherited one ran through the upstream project's shared CI); the guide says what exists and what the first release needs. Windows installs go to `Programs\VansonCraft`; AppImage updates come from `vansyson1308/vansoncraft`. |
| 2026-10-14 | minor | RPM glibc floor names libm's `GLIBC_2.35`: RHEL 9's glibc 2.34 provides libc's through backports |
| 2026-10-12 | minor | RPM dependencies use 64-bit runtime capabilities and enforce glibc 2.35 (#1275); real distro install/start-up checks remain pending |
| 2026-10-12 | minor | Portable mode (#1366): the Windows portable zip ships `portable.txt`; where the marker goes on macOS and Linux, and what Flatpak can't do |
