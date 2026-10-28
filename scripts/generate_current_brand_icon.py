"""Generate current PhotoStrike runtime artwork from the approved original (Pillow 12.3.0).

Supply the original WebP path, or '-' to read its bytes from standard input.
The source hash is checked before either output is written. The legacy artwork
and its provenance are not modified.
"""
import argparse
from hashlib import sha256
from io import BytesIO
from pathlib import Path
import struct
import sys

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
ORIGINAL_SHA256 = "5a265b0f661494c28af7d8a704f961d2c5b22f79af0d7518d26246abeb7c8af5"
SIZES = (16, 24, 32, 48, 64, 128, 256)


def generate_ico(source):
    payloads = []
    entries = []
    offset = 6 + 16 * len(SIZES)
    for size in SIZES:
        output = BytesIO()
        source.resize((size, size), Image.Resampling.LANCZOS).save(output, format="PNG", compress_level=9)
        payload = output.getvalue()
        entries.append(struct.pack("<BBBBHHII", size % 256, size % 256, 0, 0, 1, 32, len(payload), offset))
        payloads.append(payload)
        offset += len(payload)
    return struct.pack("<HHH", 0, 1, len(SIZES)) + b"".join(entries + payloads)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("original", help="approved original WebP path, or - for standard input")
    args = parser.parse_args()
    raw = sys.stdin.buffer.read() if args.original == "-" else Path(args.original).read_bytes()
    if sha256(raw).hexdigest() != ORIGINAL_SHA256:
        raise ValueError("input does not match the approved original")
    source = Image.open(BytesIO(raw)).convert("RGBA")
    if source.size != (512, 512):
        raise ValueError("approved source must be 512 x 512")
    png = BytesIO()
    source.save(png, format="PNG", compress_level=9)
    ico = generate_ico(source)
    (ROOT / "assets/branding/PhotoStrike-current.png").write_bytes(png.getvalue())
    (ROOT / "assets/branding/PhotoStrike-current.ico").write_bytes(ico)


if __name__ == "__main__":
    main()
