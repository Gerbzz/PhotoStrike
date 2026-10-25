# Approved PhotoStrike runtime artwork

`PhotoStrike.png` is the exact approved corrected suite butterfly from the
user-provided `Strike-Logos-GitHub-2026-10-25` handoff. Its SHA-256 is
`e649f49529a8e5f19372b70a96572e2708cd6d6615b764446d78869a20eca5f8`.
The complete 512 × 512 RGBA canvas, centering and padding are unchanged. Current
hashes are recorded in `provenance.json`; earlier crop/mask/centering history is
retained under `previous_provenance` rather than attributed to the new artwork.

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

No first-party license grant, release or installer is introduced. The retained CLI
Windows resource now uses the same approved PhotoStrike ICO; its existing product
identity and attribution are otherwise unchanged. This is not a new distribution. Installers, Linux desktop icon installation, macOS bundles, other platforms,
interactive pointer/accessibility acceptance and release notices remain separate gates.
