# Approved PhotoStrike runtime artwork

`PhotoStrike.png` is the existing approved suite butterfly, not a new logo. Its unchanged
SHA-256 is `e60969fe0ecb462b8e1927da66fae076b2b92ac9a5bfc32c4cc13b9bcc2e1e25`.
Its 656 × 656 RGBA pixels and original crop/mask/centering history remain documented in the
unchanged `provenance.json` (restored in `5d55d4a`, centered in `c5ed0365`).

- `crates/ui-egui/src/brand.rs` embeds those exact PNG bytes for the existing title-bar mark.
- `apps/photocraft/src/app_icon.rs` gives the native window the same decoded pixels and dimensions.
- `apps/photocraft/build.rs` embeds `PhotoStrike.ico` into `photostrike.exe`, retaining the
  `PHOTOSTRIKE_REQUIRE_WINRES=1` hard-failure gate for a missing resource compiler.

The ICO is only the approved PNG resized with Pillow 12.3.0 LANCZOS into 16, 24, 32, 48,
64, 128 and 256 pixel PNG frames. No crop, recolor, redraw or new mark is involved.
Regenerate it with `python scripts/generate_brand_icon.py`; verify with
`python scripts/check_branding.py`. The latter checks every decoded frame against its
source resize. `scripts/check_windows_icon.py` also compares actual compiled PE icon
resource payloads byte for byte with these ICO frames.

The existing File fidelity workflow retains its Linux/Windows format checks and adds
**Required native Windows branding**: exact-head runtime/title-bar tests, a native build
with required Windows resources, compiled-icon verification and a freshly launched native
software-compatibility-mode capture. The `native-proof.json` artifact records the exact
source commit, binary/asset/capture hashes and capture method. Inspect that new PNG before
claiming visible acceptance; older README screenshots are not new runtime evidence.

No first-party license grant, release or installer is introduced. The donor CLI retains
its original PhotoCraft identity/artwork as attributed source; it is not a PhotoStrike
distribution. Installers, Linux desktop icon installation, macOS bundles, other platforms,
interactive pointer/accessibility acceptance and release notices remain separate gates.
