from __future__ import annotations

import io
import json
import os
import subprocess
import sys
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

    def test_handshake_is_wp01_only(self) -> None:
        value = hello()
        self.assertEqual(value["kind"], "hello")
        self.assertEqual(value["supportedOperations"], ["spike.echo"])

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
        )
        assert process.stdin is not None
        assert process.stdout is not None
        process.stdout.readline()
        process.stdin.write(b'{"not valid"\n')
        process.stdin.flush()
        error = json.loads(process.stdout.readline())
        self.assertEqual(error["error"]["code"], "PROTOCOL_ERROR")
        process.stdin.write(b'{"protocol":"generic-app","kind":"shutdown"}\n')
        process.stdin.flush()
        process.stdin.close()
        self.assertEqual(process.wait(timeout=2), 0)
        process.stdout.close()
        assert process.stderr is not None
        process.stderr.close()


if __name__ == "__main__":
    unittest.main()
