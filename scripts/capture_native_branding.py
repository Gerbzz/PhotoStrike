"""Capture a freshly built native PhotoStrike through its existing bounded control API.

The loopback test token/profile are temporary, not account credentials. No user profile,
project, Office computer, release or installer is touched. The PNG comes from the native
renderer, not an HTML recreation. Inspect it before claiming visible-brand acceptance.
"""
import base64
from hashlib import sha256
from io import BytesIO
import json
import os
from pathlib import Path
import secrets
import socket
import subprocess
import sys
import tempfile
import time

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]


def capture(executable, output):
    executable = Path(executable).resolve()
    output = Path(output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    source = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    with tempfile.TemporaryDirectory(prefix="photostrike-brand-native-") as work:
        work = Path(work)
        profile = work / "profile"
        profile.mkdir()
        with socket.socket() as reserve:
            reserve.bind(("127.0.0.1", 0))
            port = reserve.getsockname()[1]
        token = secrets.token_hex(32)
        env = os.environ.copy()
        env["PHOTOSTRIKE_CONFIG_DIR"] = str(profile)
        env.pop("PHOTOSTRIKE_CONTROL_TOKEN_FILE", None)
        env.pop("PHOTOSTRIKE_CONTROL_PORT", None)
        env["PHOTOSTRIKE_CONTROL_TOKEN"] = token
        command = [str(executable), "--safe-gpu", "--control", str(port),
                   "--automation-read-root", str(work), "--automation-write-root", str(work)]
        with (output / "native.log").open("wb") as log:
            process = subprocess.Popen(command, env=env, cwd=work, stdout=log, stderr=subprocess.STDOUT)
            connection = None
            try:
                deadline = time.monotonic() + 120
                while time.monotonic() < deadline:
                    if process.poll() is not None:
                        raise RuntimeError(f"native process exited before capture: {process.returncode}")
                    try:
                        connection = socket.create_connection(("127.0.0.1", port), timeout=1)
                        break
                    except OSError:
                        time.sleep(0.25)
                if connection is None:
                    raise RuntimeError("native app never opened its control endpoint")
                connection.settimeout(90)
                reader = connection.makefile("rb")
                request_id = 0

                def call(method, params=None):
                    nonlocal request_id
                    request_id += 1
                    request = {"id": request_id, "method": method, "params": params or {}}
                    connection.sendall(json.dumps(request).encode() + b"\n")
                    reply = json.loads(reader.readline(16 * 1024 * 1024))
                    assert reply.get("id") == request_id and reply.get("ok"), f"{method}: {reply}"
                    return reply.get("result")

                call("auth", {"token": token})
                state = call("ui.inspect")
                shot = call("ui.screenshot")
                png = base64.b64decode(shot["base64"], validate=True)
                image = Image.open(BytesIO(png)).convert("RGBA")
                assert image.size == (shot["width"], shot["height"])
                assert image.width >= 760 and image.height >= 480, "native window smaller than its minimum"
                assert len(image.getcolors(image.width * image.height)) > 30, "blank native capture"
                screenshot = output / "photostrike-native-branding.png"
                screenshot.write_bytes(png)
                proof = {
                    "source_commit": source,
                    "binary_sha256": sha256(executable.read_bytes()).hexdigest(),
                    "approved_png_sha256": sha256((ROOT / "assets/branding/PhotoStrike-current.png").read_bytes()).hexdigest(),
                    "screenshot_sha256": sha256(png).hexdigest(),
                    "window": state["window"],
                    "capture": "Fresh native photostrike executable; existing ui.screenshot renderer readback; --safe-gpu software compatibility mode",
                    "acceptance_limit": "Synthetic native capture, not pointer/accessibility/installer/release acceptance. Visual review is separate."
                }
                (output / "native-proof.json").write_text(json.dumps(proof, indent=2) + "\n")
                call("app.quit")
                process.wait(timeout=20)
                assert process.returncode == 0, f"native shutdown failed: {process.returncode}"
                print(json.dumps(proof, indent=2))
            finally:
                if connection:
                    connection.close()
                if process.poll() is None:
                    process.terminate()
                    process.wait(timeout=20)


if __name__ == "__main__":
    capture(sys.argv[1], sys.argv[2])
