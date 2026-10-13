# PhotoStrike

**by PWDS**

**Build an image, layer by layer.**

PhotoStrike is a desktop workspace for raster images and editable layered documents. Compose an image in layers, save it as a `.pstrike` project, and export a PNG for use elsewhere.

### Highlights

- Keep image layers in an editable project.
- Save and reopen `.pstrike` documents.
- Export a finished PNG without changing the source project.

## Current status

Windows alpha source. Synthetic native-control checks covered a two-layer save and reopen, PNG export, and several save guards. Interactive pointer and accessibility review, broad file compatibility, other platforms, and installer packaging remain open.

## What the project keeps

A `.pstrike` file keeps the layered document editable, including its referenced layer assets. PNG export creates a separate flattened image. The bounded Windows checks reopened a two-layer project, confirmed the assets survived a round trip, and compared independent PNG exports; interactive editing still needs review.

## Build and verification

PhotoStrike is a native Rust raster and layered-document editor adapted from PhotoCraft. The clean standalone Windows alpha source passed focused format and automation save-guard tests, 4/4 desktop save tests, and 10/10 save identity tests. Its rebuilt `photostrike.exe` passed a synthetic native-control regression covering two-layer `.pstrike` save/reopen, PNG export, external-edit and occupied-file guards, and malformed-file refusal. The earlier Windows acceptance also verified all four non-manifest assets survived a normal round trip and independent PNG exports were byte-identical. Interactive pointer and accessibility acceptance remain open.

From this source root:

```powershell
cargo build --locked -p photocraft --bin photostrike
cargo run --locked -p photocraft --bin photostrike
```

The Cargo package and internal crate names remain `photocraft-*` for compatibility. Its local workspace, lockfile, UI icons, fonts, dictionary, and contributor data are included. Optional `CRAFT_FONTS_DIR` fonts are not bundled. A fresh machine needs ordinary crates.io access for uncached dependencies. The clean standalone Windows build required successful version-resource compilation.

The OS Save As overwrite prompt has not been clicked end-to-end. A narrow race remains between the final opened-file hash check and replacement rename when another process writes at the same time. RAW development and catalog integration with LightCraft, catalog search, and downloaded models are not implemented here. Other platforms, release packaging, and installers are not verified or supported by this candidate.

See [SOURCE_SCOPE.md](SOURCE_SCOPE.md) for donor attribution, source ownership, and excluded artwork. No new first-party license grant is stated for PhotoStrike changes.
