//! Windows only: embed PhotoStrike version info without donor-brand artwork.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PHOTOSTRIKE_REQUIRE_WINRES=1` turns it
//! into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PHOTOSTRIKE_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "PhotoStrike")
        .set("FileDescription", "PhotoStrike raster editor")
        .set("CompanyName", "PhotoStrike contributors")
        .set("LegalCopyright", "Upstream PhotoCraft: Copyright (c) 2026 ArtCraft Team and the PhotoCraft contributors. PhotoStrike changes: see SOURCE_SCOPE.md.")
        .set("OriginalFilename", "photostrike.exe")
        .set("InternalName", "photostrike");
    if let Err(e) = res.compile() {
        if std::env::var_os("PHOTOSTRIKE_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=photostrike.exe built without version resources: {e}");
    }
}
