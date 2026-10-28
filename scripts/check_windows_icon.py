"""Verify compiled PE icon resources are the exact approved ICO frame payloads.

Usage: python scripts/check_windows_icon.py target/debug/photostrike.exe
No DLL is loaded or executed by this check; PE structures are read as bytes.
"""
from pathlib import Path
import struct
import sys

ROOT = Path(__file__).resolve().parents[1]


def read_resources(data):
    pe = struct.unpack_from("<I", data, 0x3C)[0]
    assert data[pe:pe + 4] == b"PE\0\0", "not a PE executable"
    sections = struct.unpack_from("<H", data, pe + 6)[0]
    opt_size = struct.unpack_from("<H", data, pe + 20)[0]
    opt = pe + 24
    magic = struct.unpack_from("<H", data, opt)[0]
    assert magic in (0x10B, 0x20B), "unsupported PE optional header"
    directory = opt + (112 if magic == 0x20B else 96)
    resource_rva, resource_size = struct.unpack_from("<II", data, directory + 2 * 8)
    assert resource_rva and resource_size, "no compiled resources"
    table = opt + opt_size
    ranges = []
    for i in range(sections):
        section = table + i * 40
        virtual_size, rva, raw_size, raw = struct.unpack_from("<IIII", data, section + 8)
        ranges.append((rva, max(virtual_size, raw_size), raw))

    def file_offset(rva):
        for start, length, raw in ranges:
            if start <= rva < start + length:
                return raw + rva - start
        raise AssertionError(f"resource RVA outside sections: {rva}")

    base = file_offset(resource_rva)
    resources = {}

    def walk(relative, path):
        offset = base + relative
        named, ids = struct.unpack_from("<HH", data, offset + 12)
        for i in range(named + ids):
            name, child = struct.unpack_from("<II", data, offset + 16 + i * 8)
            key = name if not name & 0x80000000 else "named"
            if child & 0x80000000:
                walk(child & 0x7FFFFFFF, path + (key,))
            else:
                rva, length = struct.unpack_from("<II", data, base + child)
                start = file_offset(rva)
                payload = data[start:start + length]
                assert len(payload) == length
                resources[path + (key,)] = payload
    walk(0, ())
    return resources


def check(executable):
    data = (ROOT / "assets/branding/PhotoStrike-current.ico").read_bytes()
    reserved, kind, count = struct.unpack_from("<HHH", data)
    assert (reserved, kind) == (0, 1)
    expected = {}
    for i in range(count):
        w, h, colors, reserved, planes, bpp, length, offset = struct.unpack_from("<BBBBHHII", data, 6 + i * 16)
        expected[(w, h, colors, reserved, planes, bpp, length)] = data[offset:offset + length]
    resources = read_resources(Path(executable).read_bytes())
    groups = [(key, payload) for key, payload in resources.items() if key[0] == 14]
    assert len(groups) == 1, f"expected one PhotoStrike icon group, got {len(groups)}"
    group_key, group = groups[0]
    assert struct.unpack_from("<HHH", group) == (0, 1, count)
    matched = set()
    for i in range(count):
        entry = struct.unpack_from("<BBBBHHIH", group, 6 + 14 * i)
        frame, resource_id = entry[:-1], entry[-1]
        assert frame in expected, f"unexpected packaged icon frame: {frame}"
        assert resources[(3, resource_id, group_key[-1])] == expected[frame], "compiled RT_ICON differs from approved derivative"
        matched.add(frame)
    assert matched == set(expected), "compiled icon sizes missing"
    print(f"PASS: {count} compiled Windows icon frames exactly match PhotoStrike-current.ico")


if __name__ == "__main__":
    check(sys.argv[1])
