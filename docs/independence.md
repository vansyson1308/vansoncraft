# VansonCraft as its own project

> **Status:** 2026-11-12. VansonCraft started as a fork of VectorCraft (by the ArtCraft team) at
> version 0.24.1 and became an independent project. This page records what changed, what it still
> uses from elsewhere and why, and what is left for the maintainer.

## What changed

| Area | Now |
|---|---|
| Name | **VansonCraft** everywhere users and agents read it: window, menus, About, documentation, every translated catalog, packaging, MCP (`vansoncraft` server, `vansoncraft://` resources) |
| Code | crates `vansoncraft-*`, apps `apps/vansoncraft`, `apps/vansoncraft-cli`, `apps/vansoncraft-web`, environment variables `VANSONCRAFT_*` |
| App id | `io.github.vansyson1308.vansoncraft` (Linux desktop entry, Flatpak, AppStream, macOS bundle) |
| Icon | an original mark made for VansonCraft (`assets/app-icon/`), in every size, with the macOS DMG window to match |
| Files | native files are saved as `.vansoncraft`; see [Compatibility](#compatibility) for the old names |
| Windows installer | its own `UpgradeCode` and component GUIDs, installed to `Programs\VansonCraft`, publisher `vansyson1308`: it never upgrades, replaces or shares files with an installed VectorCraft |
| Updates | the in-app updater looks only at `vansyson1308/vansoncraft` releases, with no former ids or names, so it never offers another app's builds |
| Links | Help ▸ VansonCraft on GitHub; the upstream project's Discord, website and app page are gone from the app, the packaging and the README |
| Brand | no ArtCraft marks (their licence requires a fork to remove them); `docs/brand/` deleted |
| Credits | About ▸ Contributors lists this repository's contributors (@vansyson1308); the upstream authors are credited by their copyright notice |
| Legal | `LICENSE-MIT` and `NOTICE` keep VectorCraft's copyright line beside VansonCraft's, as the MIT and Apache licences require, and say that the ArtCraft team does not make or endorse VansonCraft |
| CI | self-contained workflows (`.github/workflows/`): `cargo xtask ci`, packaging lint, the Windows installation policy, the pinned Affinity corpus; no shared workflow from another repository, no service that needs its secrets (SonarCloud, CodeRabbit and the upstream release pipeline removed) |
| Rules | [`AGENTS.md`](../AGENTS.md) holds this project's own rules |
| Guard | `cargo xtask brands` fails on VectorCraft, DrawCraft, ArtCraft and Storyteller names in user-visible text, except lines marked `brand-ok` with the reason (old names files still use, the attribution sentence) |

## Compatibility

Everything saved before the rename still opens, and each path has a test:

| Saved before | Still read | Test |
|---|---|---|
| `.vectorcraft` and `.drawcraft` files | open as native documents; Save then asks for a `.vansoncraft` name | `engine` `nativefile::files_saved_before_the_rename_open_and_save_under_the_new_name` |
| the header `"format": "vectorcraft"` | kept: it is the name of the shared document format that the `craft-vectorcraft-format` envelope defines and reads (`vansoncraft_format::FORMAT_NAME`), so files also open in other apps that read the format | `format` `sniff_reads_the_format_member`, `native_v3`, `native_v4` |
| `"generator": "VectorCraft 0.21+"` | Twist effects keep their direction (version numbers carry on from 0.24.1) | `format` twist tests, `doc` `twist::tests::releases_from_0_21_turned_twist_clockwise` |
| SVG editing data in `urn:vectorcraft:editing`, mask options in `data-vectorcraft-mask` | restored | `svg` `editing_data_written_before_the_rename_is_read`, `mask_options_written_before_the_rename_are_read` |
| EPS data in `%VectorCraft_BeginData` sections | restored | `eps` `data_sections_written_before_the_rename_come_back` |
| PDF editing data `vectorcraft-editing.vectorcraft` | restored | `pdf` `the_name_from_before_the_rename_is_read` |
| preferences exported as `vectorcraft-preferences` | imported | `engine` `prefs_file::exported_preferences_import_unchanged` |
| the VectorCraft settings folder | copied whole into VansonCraft's the first time VansonCraft starts without one (settings, libraries, fonts, presets, plug-ins and their grants, Data Recovery); the old folder is left as it is. A DrawCraft `ui.json` is still read once | `profile::tests::the_earlier_settings_folder_is_copied_once_and_kept` |
| Data Recovery copies named `.vectorcraft` | offered, restored and discarded; their area is never tidied away with them | `engine` `recovery::copies_left_before_the_rename_are_offered_and_restored` |
| brush and symbol libraries named `.vectorcraft` | listed | (the library scans take both extensions) |
| the disabled plug-ins list and the launch marker (`vectorcraft-…` formats) | read, so a plug-in disabled after a crash stays disabled | `plugins` `a_disabled_list_from_before_the_rename_still_reads`, `only_a_marker_of_this_version_names_an_item` |
| the built-in names `VectorCraft Generic CMYK (SWOP-like)`, `VectorCraft Default` (PDF preset), `VectorCraft Defaults` (shortcut set) | resolve to today's | `color` `legacy_profile_name_resolves`, `pdf` presets test, the shortcut-set migration test |
| a portable copy (`VectorCraft.portable`, `VectorCraftData`) | its data folder is copied into `VansonCraftData` the first time | `profile::tests::a_portable_copy_from_before_the_rename_keeps_its_settings` |
| templates in `Documents/VectorCraft Templates` | used until a `VansonCraft Templates` folder exists | — |

`.vectorcraft` and `.drawcraft` files are not associated with VansonCraft in Finder, Explorer or
file managers (they belong to whatever app the desktop gives them); File ▸ Open reads them.
Templates keep the `.vctemplate` extension VectorCraft also declares, so with both apps installed
the desktop picks one for them.

Not carried over on purpose: the environment variables (`VECTORCRAFT_*` became `VANSONCRAFT_*`),
the control channel's and MCP's names (`vansoncraft://` resources, the `vansoncraft` server), and the
command format id (`vansoncraft`; `vectorcraft` still resolves to it as an extension).

## What it still uses from elsewhere, and why

| Dependency | Why it stays | To drop it |
|---|---|---|
| Libraries from `github.com/storytold/craft-libs` (`craft-pdf`, `craft-ui`, `craft-update`, `craft-vectorcraft-format`, …), pinned to exact commits in `Cargo.toml` and `Cargo.lock` | open-source libraries (MIT OR Apache-2.0) like any other dependency; a pinned commit cannot change underneath the build | copy them into the repository (with their licences) and point `Cargo.toml` at the copies |
| Forks of egui and winit (`github.com/storytold/egui`, `…/winit`), pinned in `[patch.crates-io]` | the UI needs their fixes | move to upstream egui and winit releases once they carry them, or vendor the forks |
| craft-fonts (optional `CRAFT_FONTS_DIR` build input) | a font collection under the SIL Open Font License; builds without it use system fonts for Japanese | another OFL font source |
| Edit Original of a raster image opens PhotoCraft when it is installed and Preferences › File Handling › Use System Defaults for Edit Original is off | an interoperability feature with another app, off by default | remove `open_in_photocraft` (`apps/vansoncraft/src/host_services.rs`) |
| Historical references in code comments and documentation (upstream issue numbers, sibling apps) | they explain why code is as it is | — |

`craft-update` builds a macOS bundle id and Flatpak id as `ai.storyteller.<id>` internally. VansonCraft
packages use `io.github.vansyson1308.vansoncraft`, so on macOS and in Flatpak the updater does not
recognise the install: its prompt says it cannot update this installation type and asks for a
manual download. On Windows and for AppImages and loose copies it finds the install by its file
name. The menu entry `craft-update` writes beside an AppImage is likewise named
`ai.storyteller.vansoncraft.desktop`, with that `StartupWMClass`, which does not match the window's
app id (`io.github.vansyson1308.vansoncraft`), so the dock may show the AppImage apart from its
entry. A fix belongs in `craft-update` (an app-defined bundle id).

## Left for the maintainer

1. **"forked from"** on GitHub: the repository page still shows it. Only GitHub Support can detach a
   fork (Support › "Detach fork"), or the code can be pushed to a new repository.
2. **Releases:** there are none yet; [`docs/releasing.md`](releasing.md) lists what the first one
   needs (a release workflow, optional signing certificates).
3. **Screenshots** in `docs/images/` were taken again from VansonCraft on 2026-11-12 (the
   Neon Drive and Dusk examples now say "made with/in VansonCraft", and their renders were made
   again). Example files keep the `generator` they were saved with.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-12 | major | First version. |
