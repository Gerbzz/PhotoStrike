# PhotoStrike artwork

The retained `PhotoStrike.png` is the exact approved corrected suite butterfly from the
user-provided `Strike-Logos-GitHub-2026-10-25` handoff. Its SHA-256 is
`e649f49529a8e5f19372b70a96572e2708cd6d6615b764446d78869a20eca5f8`.
The complete 512 × 512 RGBA canvas, centering and padding are unchanged. Historical
hashes are recorded in `provenance.json`; earlier crop/mask/centering history is
retained under `previous_provenance` rather than attributed to the new artwork.

The current runtime uses `PhotoStrike-current.png`, a lossless 512 × 512 RGBA
decode of the approved current artwork. Its SHA-256 is
`933f9beafea09ebaeb0470b12d896239d590a996e4a298dca584db9ce95f17df`.
Its decoded RGBA SHA-256 is
`7481160ed8501287b6a7b0d6cf07d49a3a563700ebfc59d171a33c39f9f50619`.
The earlier PNG, ICO and provenance remain unchanged.

- `crates/ui-egui/src/brand.rs` embeds the current PNG bytes for the existing title-bar mark.
- `apps/photocraft/src/app_icon.rs` gives the native window the same decoded pixels and dimensions.
- `apps/photocraft/build.rs` embeds `PhotoStrike-current.ico` into `photostrike.exe`, retaining the
  `PHOTOSTRIKE_REQUIRE_WINRES=1` hard-failure gate for a missing resource compiler.

The current ICO is only the current PNG resized with Pillow 12.3.0 LANCZOS into 16, 24, 32, 48,
64, 128 and 256 pixel PNG frames. No crop, recolor, redraw or new mark is involved.
Regenerate it with `python scripts/generate_current_brand_icon.py path/to/approved-original.webp`;
the generator verifies the input hash before writing. Verify with
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
