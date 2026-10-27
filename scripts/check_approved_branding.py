"""Approved 2026-10-25 branding contract. Requires Pillow; does not modify files."""
from pathlib import Path
from hashlib import sha256
import json
import unittest
from PIL import Image
ROOT = Path(__file__).resolve().parents[1]
APP = 'PhotoStrike'
APPROVED = 'e649f49529a8e5f19372b70a96572e2708cd6d6615b764446d78869a20eca5f8'
class ApprovedBranding(unittest.TestCase):
    def test_exact_approved_source_and_readme(self):
        p = ROOT / 'assets/branding' / (APP + '.png')
        self.assertEqual(sha256(p.read_bytes()).hexdigest(), APPROVED)
        with Image.open(p) as image:
            self.assertEqual(image.size, (512,512))
        docs_icon = ROOT/'docs/branding'/(APP+'.webp')
        self.assertEqual(sha256(docs_icon.read_bytes()).hexdigest(), '5a265b0f661494c28af7d8a704f961d2c5b22f79af0d7518d26246abeb7c8af5')
        with Image.open(docs_icon) as image:
            self.assertEqual(image.format, 'WEBP')
            self.assertEqual(image.size, (512,512))
            image.load()
        self.assertIn('src="docs/branding/' + APP + '.webp"', (ROOT/'README.md').read_text(encoding="utf-8"))
        self.assertEqual(json.loads((p.parent/'provenance.json').read_text(encoding="utf-8"))['sha256'], APPROVED)

if __name__ == "__main__":
    unittest.main()
