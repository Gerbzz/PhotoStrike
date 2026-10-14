# README screenshot provenance

The README images are direct `ui.screenshot` captures of the native PhotoStrike Windows alpha at 1440 × 900 on 2026-10-14. No interface elements were drawn or composited into the screenshots. The open `coastal-study.png` is original sample artwork generated for this documentation, not a client or stock asset.

The retained executable's SHA-256 is `8DE457AA55E434E6AB0B1C4C68AE45808E4A3D8CEC6499E52C65B190B783EA08`. Its Cargo depfile points to a clean verification source checkout. All 1,478 currently tracked or unignored source-tree paths were compared against that checkout: 68 documentation/configuration/artwork paths were absent there, and eight files differed (README, Cargo manifest license metadata, and Windows version-resource copyright text). Rust source files and `Cargo.lock` matched. This supports showing current UI behavior; it does not represent a new build or a release verification.

The app was launched with a task-local configuration directory, a localhost authenticated control listener, and read/write roots restricted to an isolated sample folder. The screenshot sequence opened the sample PNG and captured the workspace; created two visible editable type layers with the native `type.create` command; saved a `.pstrike` project and captured its Layers view; then selected the Adjustments tab, added a Brightness/Contrast adjustment layer, and captured its selected mask. The app reported type bounds `[84, 117, 583, 167]` and `[84, 192, 301, 212]`; its history listed the new type layers and the new adjustment layer. The final bounded capture folder used about 2 MiB. No builds or dependency downloads were performed for these captures.

| Asset | SHA-256 |
| --- | --- |
| `photostrike-workspace.png` | `0B80853D70F527838BCBEAC8AC4B6568E15E2F824714853BF2FDE8B16DCBA3CB` |
| `photostrike-layers.png` | `87C70F3BA7E95D7F8335726299EDBD247F8B6BBA853ED9218C4A7F6B638630EC` |
| `photostrike-adjustments.png` | `CE0A52295BB36B75CA38A743257CFF6765014518FC671E5B4567FE7844BAC98B` |
| `coastal-study.png` | `D1B24F67CC3845FB0A3C7EA2789F1CCDFD7037EDF3D12D7C955D27A6B3805554` |
