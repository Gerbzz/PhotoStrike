"""Check PhotoStrike's approved source artwork and actual binding/package paths.

Requires Pillow 12.3.0, only for inspecting image bytes. No files are modified.
"""
from hashlib import sha256
from io import BytesIO
from pathlib import Path
import struct
import unittest

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
PNG = ROOT / "assets/branding/PhotoStrike-current.png"
ICO = ROOT / "assets/branding/PhotoStrike-current.ico"
SIZES = (16, 24, 32, 48, 64, 128, 256)


class BrandingContract(unittest.TestCase):
    def test_current_png_matches_approved_original_pixels(self):
        approved = "933f9beafea09ebaeb0470b12d896239d590a996e4a298dca584db9ce95f17df"
        self.assertTrue(PNG.exists(), "current runtime artwork not generated yet")
        self.assertEqual(sha256(PNG.read_bytes()).hexdigest(), approved)
        image = Image.open(PNG).convert("RGBA")
        self.assertEqual(image.size, (512, 512))
        self.assertEqual(sha256(image.tobytes()).hexdigest(), "7481160ed8501287b6a7b0d6cf07d49a3a563700ebfc59d171a33c39f9f50619")

    def test_runtime_embeds_canonical_png_instead_of_procedural_placeholder(self):
        brand = (ROOT / "crates/ui-egui/src/brand.rs").read_text()
        self.assertIn('include_bytes!("../../../assets/branding/PhotoStrike-current.png")', brand)
        self.assertNotIn("let lens =", brand)
        self.assertIn("photocraft_codecs::decode", brand)

    def test_native_entrypoint_binds_shared_icon_with_decoded_dimensions(self):
        native = (ROOT / "apps/photocraft/src/app_icon.rs").read_text()
        self.assertIn("photocraft_ui_egui::brand::icon()", native)
        self.assertNotIn("width: 128", native)
        self.assertIn(".with_icon(app_icon::window_icon())", (ROOT / "apps/photocraft/src/main.rs").read_text())

    def test_windows_resource_binds_only_photostrike_icon(self):
        build = (ROOT / "apps/photocraft/build.rs").read_text()
        self.assertIn('set_icon("../../assets/branding/PhotoStrike-current.ico")', build)
        self.assertIn('cargo:rerun-if-changed=../../assets/branding/PhotoStrike-current.ico', build)
        self.assertIn("PHOTOSTRIKE_REQUIRE_WINRES", build)
        self.assertNotIn("assets/app-icon/photocraft.ico", build)
        cli_build = (ROOT / "apps/photocraft-cli/build.rs").read_text()
        self.assertIn('set_icon("../../assets/branding/PhotoStrike-current.ico")', cli_build)

    def test_packaged_frames_are_only_resizes_of_approved_rgba(self):
        self.assertTrue(ICO.exists(), "current runtime ICO not generated/wired yet")
        data = ICO.read_bytes()
        self.assertEqual(struct.unpack_from("<HHH", data), (0, 1, len(SIZES)))
        source = Image.open(PNG).convert("RGBA")
        for index, size in enumerate(SIZES):
            w, h, colors, reserved, planes, bpp, length, offset = struct.unpack_from("<BBBBHHII", data, 6 + 16 * index)
            self.assertEqual((w or 256, h or 256, colors, reserved, planes, bpp), (size, size, 0, 0, 1, 32))
            actual = Image.open(BytesIO(data[offset:offset + length])).convert("RGBA")
            expected = source.resize((size, size), Image.Resampling.LANCZOS)
            self.assertEqual(actual.size, expected.size)
            self.assertEqual(actual.tobytes(), expected.tobytes(), f"not the approved {size}px derivative")


if __name__ == "__main__":
    unittest.main()
