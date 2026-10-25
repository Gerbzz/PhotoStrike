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
        self.assertIn('assets/branding/' + APP + '.png', (ROOT/'README.md').read_text())
        self.assertEqual(json.loads((p.parent/'provenance.json').read_text())['sha256'], APPROVED)

if __name__ == "__main__":
    unittest.main()
