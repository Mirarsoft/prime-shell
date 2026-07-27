from __future__ import annotations

import json
import subprocess
import sys
import time
from pathlib import Path
from typing import Any, BinaryIO

ROOT = Path(__file__).resolve().parents[2]
def sidecar_executable() -> Path:
    output = subprocess.check_output(
        [sys.executable, str(ROOT / "scripts" / "verify" / "sidecar_path.py")],
        text=True,
    )
    return Path(output.strip())


def read_frame(stream: BinaryIO, maximum: int) -> dict[str, Any]:
    line = stream.readline(maximum + 2)
    if not line or len(line) > maximum + 1 or not line.endswith(b"\n"):
        raise AssertionError("missing or oversized protocol frame")
    value = json.loads(line.decode("utf-8"))
    if not isinstance(value, dict):
        raise AssertionError("protocol frame is not an object")
    return value


def start(executable: Path) -> subprocess.Popen[bytes]:
    environment = {
        "LANG": "C.UTF-8",
        "LC_ALL": "C.UTF-8",
    }
    return subprocess.Popen(
        [str(executable)],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        cwd=executable.parent,
        env=environment,
    )


def send(process: subprocess.Popen[bytes], value: dict[str, Any]) -> None:
    assert process.stdin is not None
    process.stdin.write(
        json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
        + b"\n"
    )
    process.stdin.flush()


def main() -> int:
    executable = sidecar_executable()
    if not executable.is_file():
        raise SystemExit(f"packaged sidecar not found: {executable}")
    manifest_path = executable.parent.parent / "sidecar-manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))

    started = time.monotonic()
    process = start(executable)
    assert process.stdout is not None
    hello = read_frame(process.stdout, 64 * 1024)
    handshake_ms = round((time.monotonic() - started) * 1000, 3)
    assert hello["kind"] == "hello"
    assert hello["supportedOperations"] == ["spike.echo"]
    for field in ("buildId", "schemaHash", "targetTriple"):
        assert hello[field] == manifest[field]

    text = "Hello — مرحبا — こんにちは 👋"
    send(
        process,
        {
            "protocol": "generic-app",
            "kind": "request",
            "requestId": "packaged-echo-1",
            "traceId": "packaged-trace-1",
            "operation": "spike.echo",
            "payload": {"text": text},
        },
    )
    result = read_frame(process.stdout, 1024 * 1024)
    assert result["kind"] == "result"
    assert result["payload"]["text"] == text

    send(
        process,
        {
            "protocol": "generic-app",
            "kind": "request",
            "requestId": "unknown-1",
            "traceId": "packaged-trace-2",
            "operation": "spike.future",
            "payload": {},
        },
    )
    unknown = read_frame(process.stdout, 1024 * 1024)
    assert unknown["kind"] == "error"
    assert unknown["error"]["code"] == "VALIDATION_ERROR"

    assert process.stdin is not None
    process.stdin.write(b'{"malformed"\n')
    process.stdin.flush()
    malformed = read_frame(process.stdout, 1024 * 1024)
    assert malformed["error"]["code"] == "PROTOCOL_ERROR"

    send(process, {"protocol": "generic-app", "kind": "shutdown"})
    assert process.wait(timeout=2) == 0
    stderr = process.stderr.read() if process.stderr is not None else b""
    for line in stderr.splitlines():
        value = json.loads(line.decode("utf-8"))
        assert isinstance(value, dict) and "event" in value

    oversized_process = start(executable)
    assert oversized_process.stdout is not None
    read_frame(oversized_process.stdout, 64 * 1024)
    assert oversized_process.stdin is not None
    oversized_process.stdin.write(b"x" * (1024 * 1024 + 8193))
    oversized_process.stdin.flush()
    oversized = read_frame(oversized_process.stdout, 1024 * 1024)
    assert oversized["error"]["code"] == "RESOURCE_EXHAUSTED"
    assert oversized_process.wait(timeout=2) == 2

    print(
        json.dumps(
            {
                "status": "passed",
                "executable": executable.name,
                "externalPythonPathProvided": "PATH" in {
                    "LANG": "C.UTF-8",
                    "LC_ALL": "C.UTF-8",
                },
                "handshakeMs": handshake_ms,
                "bundleBytes": manifest["bundleBytes"],
                "buildId": manifest["buildId"],
                "schemaHash": manifest["schemaHash"],
                "targetTriple": manifest["targetTriple"],
                "unicodeEcho": "passed",
                "unknownOperation": "rejected",
                "malformedFrame": "rejected",
                "oversizedFrame": "rejected",
                "boundedShutdown": "passed",
                "stdoutProtocolOnly": "passed",
                "stderrStructured": "passed",
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
