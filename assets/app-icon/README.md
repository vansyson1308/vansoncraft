# VansonCraft app icon

**Mark:** a V drawn the way VansonCraft draws paths: a straight segment down to a corner point and a
curve up to a smooth point, whose Bézier handle runs along the curve's tangent to a round knob.

**Palette** (exactly three colours):

| Colour | Hex | Used for |
|---|---|---|
| Deep teal | `#123c46` | the full-bleed field (the VansonCraft colour) |
| Paper | `#f4efe4` | the path |
| Handle yellow | `#f2b134` | the anchor point, its handle and knob |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`, clipped. The macOS files add Apple's
transparent margin (tile = 824/1024 of the canvas); Windows and Linux use the tile edge to edge.

**Provenance:** original artwork made for VansonCraft, written by hand as SVG (a few shapes, no
third-party material). Licence: see `LICENSE.txt`.

## Files

| File | What | Used by |
|---|---|---|
| `vansoncraft.svg` | master vector | source for everything below |
| `vansoncraft-small.svg` | the same vector (the mark is light enough for every size) | `hicolor/scalable` |
| `vansoncraft-1024.png` | full tile, 1024 px | docs, store listings |
| `vansoncraft-macos-512.png` | tile with Apple margin, 512 px | runtime Dock icon (`apps/vansoncraft/src/main.rs`) |
| `vansoncraft.icns` | macOS icon set, 16 to 1024 px | `cargo xtask bundle` (`CFBundleIconFile`) |
| `vansoncraft.ico` | Windows icon, 16 to 256 px | `apps/vansoncraft/build.rs` (embedded in the `.exe`) |
| `hicolor/<size>/apps/io.github.vansyson1308.vansoncraft.png` | Linux theme icons, 16 to 512 px; the 256 one is also the runtime icon on Windows and Linux, the 128 one the brand mark in the app bar and About box (`crates/ui-egui/src/brand.rs`) | `packaging/linux/io.github.vansyson1308.vansoncraft.desktop` |
| `hicolor/scalable/apps/io.github.vansyson1308.vansoncraft.svg` | Linux scalable icon | as above |

## How it reaches each OS

- **macOS:** the app sets the Dock / app-switcher icon at runtime (`ViewportBuilder::with_icon`); the
  `.app` from `cargo xtask bundle` carries `vansoncraft.icns`.
- **Windows:** `build.rs` embeds `vansoncraft.ico` and VERSIONINFO with `winresource` (taskbar, Start
  menu, Explorer, Alt-Tab); the runtime icon covers the title bar.
- **Linux:** install `packaging/linux/io.github.vansyson1308.vansoncraft.desktop` to `share/applications/` and
  `hicolor/` to `share/icons/`. The app sets its Wayland app ID / X11 class to
  `io.github.vansyson1308.vansoncraft`, so the dock matches the window to the launcher.

## Regenerate

Edit or replace `vansoncraft.svg` (and `vansoncraft-small.svg`), then run `packaging/icons.sh`
(needs `resvg`; `iconutil` on macOS for the `.icns`). It writes every PNG, the `.ico` (via
`cargo xtask ico`) and the `.icns`. Without `iconutil`, an `.icns` of PNG entries (`icp4`–`ic14`)
can be written from the macOS renders by any ICNS writer; the one committed was made that way.
