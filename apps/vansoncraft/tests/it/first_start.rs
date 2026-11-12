//! The first start of VansonCraft on a computer where VectorCraft, the project's previous name,
//! kept settings: the whole earlier settings folder comes across before anything writes in
//! VansonCraft's own, except what belongs to a running app (its single-instance endpoint, its
//! launch markers, its logs, the locks of its Data Recovery areas). Linux only: the binary runs
//! without a display, so it stops once it reaches the window.

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn write(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

#[test]
fn the_first_start_copies_the_earlier_settings_folder_before_writing_its_own() {
    let home = tempfile::tempdir().unwrap();
    let config = home.path().join(".config");
    let earlier = config.join("vectorcraft"); // brand-ok: the settings folder before the rename
    write(&earlier.join("ui.json"), "{}");
    write(&earlier.join("Swatches/Mine.ase"), "ase");
    write(&earlier.join("Data Recovery/area-1/Poster-1.vectorcraft"), "copy"); // brand-ok: a copy saved before the rename
    write(&earlier.join("Data Recovery/area-1/.lock"), "");
    let endpoint = r#"{"port": 9, "token": "earlier-app", "hello": "earlier-app"}"#;
    write(&earlier.join("instance.json"), endpoint);
    write(&earlier.join("logs/vectorcraft.log"), "old log"); // brand-ok: the earlier app's log
    write(&earlier.join("Launch/marker"), "running");

    let mut child = Command::new(crate::bin())
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .env_remove("VANSONCRAFT_NO_PREFS")
        .env_remove("VANSONCRAFT_CONFIG_DIR")
        .env_remove("VANSONCRAFT_CONTROL_PORT")
        .env("HOME", home.path())
        .env("XDG_CONFIG_HOME", &config)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(120);
    while child.try_wait().unwrap().is_none() {
        assert!(Instant::now() < deadline, "the app did not stop without a display");
        std::thread::sleep(Duration::from_millis(100));
    }

    let current = config.join("vansoncraft");
    assert_eq!(fs::read_to_string(current.join("Swatches/Mine.ase")).unwrap(), "ase", "the libraries came across");
    assert!(current.join("Data Recovery/area-1/Poster-1.vectorcraft").is_file(), "the Data Recovery copies came across"); // brand-ok: as above
    assert!(!current.join("Data Recovery/area-1/.lock").exists(), "a running app's lock is its own");
    assert!(!current.join("logs/vectorcraft.log").exists(), "the earlier app's logs stay with it"); // brand-ok: as above
    assert!(!current.join("Launch/marker").exists(), "its launch markers too");
    // VansonCraft writes its own endpoint; the earlier app's (which would hand files to it) never
    // comes across.
    let own = fs::read_to_string(current.join("instance.json")).unwrap_or_default();
    assert!(!own.contains("earlier-app"), "{own}");
    assert_eq!(fs::read_to_string(earlier.join("instance.json")).unwrap(), endpoint, "the earlier folder stays as it was");
}
