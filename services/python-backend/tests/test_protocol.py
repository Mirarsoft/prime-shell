from __future__ import annotations

import io
import json
import os
import select
import subprocess
import sys
import time
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT / "services" / "python-backend" / "src"
sys.path.insert(0, str(SOURCE))

from prime_shell_backend.protocol import (  # noqa: E402
    FRAME_MAX_BYTES,
    BoundedLineReader,
    FrameTooLarge,
    handle_request,
    hello,
)


class ProtocolUnitTests(unittest.TestCase):
    def start_backend(self) -> subprocess.Popen[bytes]:
        environment = {
            "PYTHONPATH": str(SOURCE),
            "PYTHONIOENCODING": "utf-8",
            "PYTHONUNBUFFERED": "1",
        }
        process = subprocess.Popen(
            [sys.executable, "-m", "prime_shell_backend"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=environment,
            cwd=ROOT,
            bufsize=0,
        )
        self.addCleanup(self.stop_backend, process)
        hello_frame = self.read_frame(process)
        self.assertEqual(hello_frame["kind"], "hello")
        return process

    @staticmethod
    def stop_backend(process: subprocess.Popen[bytes]) -> None:
        if process.poll() is None:
            try:
                ProtocolUnitTests.send_frame(
                    process, {"protocol": "generic-app", "kind": "shutdown"}
                )
                process.wait(timeout=2)
            except (BrokenPipeError, subprocess.TimeoutExpired):
                process.kill()
                process.wait(timeout=2)
        for stream in (process.stdin, process.stdout, process.stderr):
            if stream is not None:
                stream.close()

    @staticmethod
    def send_frame(
        process: subprocess.Popen[bytes], value: dict[str, object]
    ) -> None:
        assert process.stdin is not None
        process.stdin.write(
            json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode(
                "utf-8"
            )
            + b"\n"
        )
        process.stdin.flush()

    def read_frame(
        self, process: subprocess.Popen[bytes], timeout: float = 2
    ) -> dict[str, object]:
        assert process.stdout is not None
        ready, _, _ = select.select([process.stdout], [], [], timeout)
        self.assertTrue(ready, "timed out waiting for backend protocol frame")
        line = process.stdout.readline(FRAME_MAX_BYTES + 2)
        self.assertTrue(line.endswith(b"\n"), "backend frame must be newline-delimited")
        self.assertLessEqual(len(line), FRAME_MAX_BYTES + 1)
        value = json.loads(line)
        self.assertIsInstance(value, dict)
        return value

    def test_shared_valid_echo_fixture(self) -> None:
        fixture_path = (
            ROOT
            / "packages"
            / "app-contracts"
            / "fixtures"
            / "valid"
            / "echo-request.json"
        )
        request = json.loads(fixture_path.read_text(encoding="utf-8"))
        response = handle_request(request)
        self.assertEqual(response["kind"], "result")
        self.assertEqual(response["payload"], request["payload"])

    def test_shared_unknown_operation_fixture(self) -> None:
        fixture_path = (
            ROOT
            / "packages"
            / "app-contracts"
            / "fixtures"
            / "invalid"
            / "unknown-operation.json"
        )
        request = json.loads(fixture_path.read_text(encoding="utf-8"))
        response = handle_request(request)
        self.assertEqual(response["error"]["code"], "VALIDATION_ERROR")

    def test_handshake_lists_the_exact_spike_operations(self) -> None:
        value = hello()
        self.assertEqual(value["kind"], "hello")
        self.assertEqual(
            value["supportedOperations"],
            [
                "spike.echo",
                "spike.count",
                "spike.crash",
                "spike.hang",
                "spike.largeRejected",
            ],
        )

    def test_unicode_echo_is_exact(self) -> None:
        text = "Hello — مرحبا — こんにちは 👋"
        response = handle_request(
            {
                "protocol": "generic-app",
                "kind": "request",
                "requestId": "request-1",
                "traceId": "trace-1",
                "operation": "spike.echo",
                "payload": {"text": text},
            }
        )
        self.assertIsNotNone(response)
        self.assertEqual(response["payload"]["text"], text)

    def test_unknown_operation_is_rejected(self) -> None:
        response = handle_request(
            {
                "protocol": "generic-app",
                "kind": "request",
                "requestId": "request-2",
                "traceId": "trace-2",
                "operation": "spike.future",
                "payload": {},
            }
        )
        self.assertEqual(response["kind"], "error")
        self.assertEqual(response["error"]["code"], "VALIDATION_ERROR")

    def test_crlf_is_accepted(self) -> None:
        reader = BoundedLineReader(io.BytesIO(b'{"kind":"test"}\r\n'))
        self.assertEqual(reader.read_line(64), b'{"kind":"test"}')

    def test_oversized_frame_is_bounded(self) -> None:
        reader = BoundedLineReader(io.BytesIO(b"x" * (FRAME_MAX_BYTES + 8193)))
        with self.assertRaises(FrameTooLarge):
            reader.read_line(FRAME_MAX_BYTES)

    def test_malformed_frame_returns_protocol_error(self) -> None:
        process = self.start_backend()
        assert process.stdin is not None
        process.stdin.write(b'{"not valid"\n')
        process.stdin.flush()
        error = self.read_frame(process)
        self.assertEqual(error["error"]["code"], "PROTOCOL_ERROR")

    def test_count_progress_is_monotonic_with_one_terminal(self) -> None:
        process = self.start_backend()
        self.send_frame(
            process,
            {
                "protocol": "generic-app",
                "kind": "request",
                "requestId": "count-request",
                "traceId": "count-trace",
                "operation": "spike.count",
                "payload": {"countTo": 3, "intervalMs": 10},
            },
        )
        accepted = self.read_frame(process)
        self.assertEqual(accepted["kind"], "accepted")
        events = [self.read_frame(process) for _ in range(4)]
        self.assertEqual([event["sequence"] for event in events], [1, 2, 3, 4])
        self.assertEqual(
            [event["progress"]["current"] for event in events[:3]], [1, 2, 3]
        )
        self.assertEqual(events[-1]["state"], "Succeeded")
        self.assertEqual(
            sum(event["state"] in {"Succeeded", "Failed", "Cancelled"} for event in events),
            1,
        )

    def test_cancel_ack_is_responsive_and_terminal_is_unique(self) -> None:
        process = self.start_backend()
        self.send_frame(
            process,
            {
                "protocol": "generic-app",
                "kind": "request",
                "requestId": "cancel-count-request",
                "traceId": "cancel-count-trace",
                "operation": "spike.count",
                "payload": {"countTo": 100, "intervalMs": 20},
            },
        )
        accepted = self.read_frame(process)
        task_id = accepted["taskId"]
        self.read_frame(process)
        started = time.monotonic()
        self.send_frame(
            process,
            {
                "protocol": "generic-app",
                "kind": "cancel",
                "requestId": "cancel-request",
                "traceId": "cancel-trace",
                "taskId": task_id,
            },
        )
        observed: list[dict[str, object]] = []
        acknowledgement: dict[str, object] | None = None
        terminal: dict[str, object] | None = None
        while acknowledgement is None or terminal is None:
            value = self.read_frame(process)
            observed.append(value)
            if value.get("kind") == "cancelAck":
                acknowledgement = value
            if value.get("kind") == "taskEvent" and value.get("state") in {
                "Succeeded",
                "Failed",
                "Cancelled",
            }:
                terminal = value
        acknowledgement_ms = (time.monotonic() - started) * 1000
        self.assertTrue(acknowledgement["accepted"])
        self.assertLessEqual(acknowledgement_ms, 250)
        self.assertEqual(terminal["state"], "Cancelled")
        self.assertEqual(
            sum(
                value.get("state") in {"Succeeded", "Failed", "Cancelled"}
                for value in observed
            ),
            1,
        )

    def test_large_operation_is_rejected_without_allocating_payload(self) -> None:
        response = handle_request(
            {
                "protocol": "generic-app",
                "kind": "request",
                "requestId": "large-request",
                "traceId": "large-trace",
                "operation": "spike.largeRejected",
                "payload": {"requestedBytes": FRAME_MAX_BYTES + 1},
            }
        )
        assert response is not None
        self.assertEqual(response["error"]["code"], "RESOURCE_EXHAUSTED")

    def test_deliberate_crash_exits_the_single_sidecar(self) -> None:
        process = self.start_backend()
        self.send_frame(
            process,
            {
                "protocol": "generic-app",
                "kind": "request",
                "requestId": "crash-request",
                "traceId": "crash-trace",
                "operation": "spike.crash",
                "payload": {},
            },
        )
        self.assertEqual(self.read_frame(process)["kind"], "accepted")
        self.assertEqual(process.wait(timeout=2), 70)

    def test_hang_keeps_control_reader_responsive(self) -> None:
        process = self.start_backend()
        self.send_frame(
            process,
            {
                "protocol": "generic-app",
                "kind": "request",
                "requestId": "hang-request",
                "traceId": "hang-trace",
                "operation": "spike.hang",
                "payload": {},
            },
        )
        accepted = self.read_frame(process)
        started = time.monotonic()
        self.send_frame(
            process,
            {
                "protocol": "generic-app",
                "kind": "cancel",
                "requestId": "hang-cancel",
                "traceId": "hang-cancel-trace",
                "taskId": accepted["taskId"],
            },
        )
        acknowledgement = self.read_frame(process)
        self.assertEqual(acknowledgement["kind"], "cancelAck")
        self.assertTrue(acknowledgement["accepted"])
        self.assertLessEqual((time.monotonic() - started) * 1000, 250)
        process.kill()
        process.wait(timeout=2)


if __name__ == "__main__":
    unittest.main()
