//! Windows only: embed the app icon and version info (VERSIONINFO) into `vansoncraft.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `VANSONCRAFT_REQUIRE_WINRES=1` turns it into
//! an error (for release builds).

fn main() -> Result<(), String> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/vansoncraft.ico");
    println!("cargo:rerun-if-env-changed=VANSONCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/vansoncraft.ico")
        .set("ProductName", "VansonCraft")
        .set("FileDescription", "VansonCraft vector illustration editor")
        .set("CompanyName", "vansyson1308")
        .set("LegalCopyright", "Copyright (c) the VansonCraft authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "vansoncraft.exe")
        .set("InternalName", "vansoncraft");
    if let Err(e) = res.compile() {
        if std::env::var_os("VANSONCRAFT_REQUIRE_WINRES").is_some() {
            return Err(format!("embedding Windows resources failed: {e}"));
        }
        println!("cargo:warning=vansoncraft.exe built without icon/version resources: {e}");
    }
    Ok(())
}
