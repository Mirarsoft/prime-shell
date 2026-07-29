from __future__ import annotations

import json
import os
import select
import subprocess
import sys
import threading
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any, BinaryIO

ROOT = Path(__file__).resolve().parents[2]
FRAME_MAX_BYTES = 1024 * 1024
LOG_MAX_BYTES = 64 * 1024
ALLOWED_LOG_KEYS = {
    "timestamp",
    "level",
    "component",
    "event",
    "requestId",
    "traceId",
    "errorCode",
}


def sidecar_executable() -> Path:
    output = subprocess.check_output(
        [sys.executable, str(ROOT / "scripts" / "verify" / "sidecar_path.py")],
        text=True,
    )
    path = Path(output.strip())
    if not path.is_file():
        raise AssertionError(f"packaged sidecar not found: {path}")
    return path


@dataclass
class LogSummary:
    lines: int = 0
    maximum_bytes: int = 0
    invalid_lines: int = 0
    unexpected_keys: int = 0
    unsafe_values: int = 0


class Sidecar:
    def __init__(self, executable: Path) -> None:
        self.process = subprocess.Popen(
            [str(executable)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=executable.parent,
            env={"LANG": "C.UTF-8", "LC_ALL": "C.UTF-8"},
            bufsize=0,
        )
        self.logs = LogSummary()
        assert self.process.stderr is not None
        self.log_thread = threading.Thread(
            target=self._drain_logs,
            args=(self.process.stderr,),
            name="wp02-log-drain",
            daemon=True,
        )
        self.log_thread.start()
        hello = self.read_frame(64 * 1024)
        assert hello["kind"] == "hello"

    def _drain_logs(self, stream: BinaryIO) -> None:
        while True:
            line = stream.readline(LOG_MAX_BYTES + 2)
            if not line:
                return
            self.logs.lines += 1
            self.logs.maximum_bytes = max(self.logs.maximum_bytes, len(line))
            if len(line) > LOG_MAX_BYTES + 1 or not line.endswith(b"\n"):
                self.logs.invalid_lines += 1
                continue
            try:
                value = json.loads(line)
            except (UnicodeDecodeError, json.JSONDecodeError):
                self.logs.invalid_lines += 1
                continue
            if not isinstance(value, dict):
                self.logs.invalid_lines += 1
                continue
            if not set(value).issubset(ALLOWED_LOG_KEYS):
                self.logs.unexpected_keys += 1
            serialized = json.dumps(value, ensure_ascii=False)
            if "/" in serialized or "\\" in serialized or "secret-sentinel" in serialized:
                self.logs.unsafe_values += 1

    def send(self, value: dict[str, Any]) -> None:
        assert self.process.stdin is not None
        self.process.stdin.write(
            json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode(
                "utf-8"
            )
            + b"\n"
        )
        self.process.stdin.flush()

    def write(self, value: bytes) -> None:
        assert self.process.stdin is not None
        self.process.stdin.write(value)
        self.process.stdin.flush()

    def read_frame(self, maximum: int = FRAME_MAX_BYTES, timeout: float = 3) -> dict[str, Any]:
        assert self.process.stdout is not None
        ready, _, _ = select.select([self.process.stdout], [], [], timeout)
        if not ready:
            raise AssertionError("timed out waiting for protocol output")
        line = self.process.stdout.readline(maximum + 2)
        if not line.endswith(b"\n") or len(line) > maximum + 1:
            raise AssertionError("protocol frame is missing or oversized")
        value = json.loads(line)
        if not isinstance(value, dict):
            raise AssertionError("protocol frame must be an object")
        return value

    def shutdown(self) -> None:
        if self.process.poll() is None:
            self.send({"protocol": "generic-app", "kind": "shutdown"})
            if self.process.wait(timeout=3) != 0:
                raise AssertionError("sidecar shutdown failed")
        self.log_thread.join(timeout=2)
        if self.log_thread.is_alive():
            raise AssertionError("stderr drain did not finish")
        for stream in (self.process.stdin, self.process.stdout, self.process.stderr):
            if stream is not None:
                stream.close()

    def kill(self) -> None:
        if self.process.poll() is None:
            self.process.kill()
            self.process.wait(timeout=2)
        self.log_thread.join(timeout=2)
        for stream in (self.process.stdin, self.process.stdout, self.process.stderr):
            if stream is not None:
                stream.close()

    def assert_safe_logs(self) -> None:
        assert self.logs.lines > 0
        assert self.logs.maximum_bytes <= LOG_MAX_BYTES + 1
        assert self.logs.invalid_lines == 0
        assert self.logs.unexpected_keys == 0
        assert self.logs.unsafe_values == 0


def request(
    request_id: str,
    operation: str,
    payload: dict[str, Any],
) -> dict[str, Any]:
    return {
        "protocol": "generic-app",
        "kind": "request",
        "requestId": request_id,
        "traceId": f"{request_id}-trace",
        "operation": operation,
        "payload": payload,
    }


def verify_invalid_and_unknown(executable: Path) -> LogSummary:
    sidecar = Sidecar(executable)
    try:
        sidecar.write(b"\xff\n")
        assert sidecar.read_frame()["error"]["code"] == "PROTOCOL_ERROR"
        unknown_kind = request("unknown-kind", "spike.echo", {"text": "safe"})
        unknown_kind["kind"] = "future"
        sidecar.send(unknown_kind)
        assert sidecar.read_frame()["error"]["code"] == "VALIDATION_ERROR"
        sidecar.send(request("unknown-operation", "spike.future", {}))
        assert sidecar.read_frame()["error"]["code"] == "VALIDATION_ERROR"
        sidecar.send(
            {"protocol": "generic-app", "kind": "shutdown", "unexpected": True}
        )
        assert sidecar.read_frame()["error"]["code"] == "VALIDATION_ERROR"
        sidecar.shutdown()
        sidecar.assert_safe_logs()
        return sidecar.logs
    finally:
        sidecar.kill()


def verify_oversize(executable: Path) -> LogSummary:
    sidecar = Sidecar(executable)
    try:
        sidecar.write(b"x" * (FRAME_MAX_BYTES + 8193))
        rejected = sidecar.read_frame()
        assert rejected["error"]["code"] == "RESOURCE_EXHAUSTED"
        assert sidecar.process.wait(timeout=3) == 2
        sidecar.log_thread.join(timeout=2)
        sidecar.assert_safe_logs()
        return sidecar.logs
    finally:
        sidecar.kill()


def verify_slow_consumer_and_log_flood(executable: Path) -> tuple[int, int, LogSummary]:
    sidecar = Sidecar(executable)
    try:
        sidecar.send(request("bounded-count", "spike.count", {"countTo": 600, "intervalMs": 10}))
        accepted = sidecar.read_frame()
        assert accepted["kind"] == "accepted"
        time.sleep(1)
        events: list[dict[str, Any]] = []
        while not events or events[-1].get("state") not in {
            "Succeeded",
            "Failed",
            "Cancelled",
        }:
            events.append(sidecar.read_frame(timeout=8))
        assert len(events) == 601
        assert [event["sequence"] for event in events] == list(range(1, 602))
        assert events[-1]["state"] == "Succeeded"
        assert (
            sum(
                event.get("state") in {"Succeeded", "Failed", "Cancelled"}
                for event in events
            )
            == 1
        )
        sidecar.shutdown()
        sidecar.assert_safe_logs()
        assert sidecar.logs.lines >= 602
        return len(events), events[-1]["sequence"], sidecar.logs
    finally:
        sidecar.kill()


def main() -> int:
    executable = sidecar_executable()
    invalid_logs = verify_invalid_and_unknown(executable)
    oversize_logs = verify_oversize(executable)
    event_count, final_sequence, flood_logs = verify_slow_consumer_and_log_flood(
        executable
    )
    print(
        json.dumps(
            {
                "status": "passed",
                "platform": sys.platform,
                "architecture": os.uname().machine,
                "executable": executable.name,
                "frameMaximumBytes": FRAME_MAX_BYTES,
                "logMaximumBytes": LOG_MAX_BYTES,
                "pendingRequestMaximum": 64,
                "backendEventMaximum": 256,
                "invalidUtf8": "rejected",
                "unknownKind": "rejected",
                "unknownOperation": "rejected",
                "oversizedFrame": "rejected",
                "slowConsumer": "passed",
                "eventCount": event_count,
                "finalSequence": final_sequence,
                "terminalCount": 1,
                "stderrFlood": {
                    "status": "passed",
                    "lines": flood_logs.lines,
                    "maximumBytes": flood_logs.maximum_bytes,
                    "invalidLines": flood_logs.invalid_lines,
                    "unexpectedKeys": flood_logs.unexpected_keys,
                    "unsafeValues": flood_logs.unsafe_values,
                },
                "invalidCaseLogLines": invalid_logs.lines,
                "oversizeCaseLogLines": oversize_logs.lines,
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
