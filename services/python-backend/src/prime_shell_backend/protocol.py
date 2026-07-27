from __future__ import annotations

import json
import sys
from dataclasses import dataclass
from typing import Any, BinaryIO

from . import __version__
from .metadata import load_build_info

HANDSHAKE_MAX_BYTES = 64 * 1024
FRAME_MAX_BYTES = 1024 * 1024
LOG_MAX_BYTES = 64 * 1024
TEXT_MAX_CHARACTERS = 262_144
PROTOCOL = "generic-app"
SUPPORTED_OPERATIONS = ("spike.echo",)


class FrameTooLarge(ValueError):
    pass


@dataclass
class BoundedLineReader:
    stream: BinaryIO
    buffer: bytearray

    def __init__(self, stream: BinaryIO) -> None:
        self.stream = stream
        self.buffer = bytearray()

    def read_line(self, maximum: int) -> bytes | None:
        while True:
            newline = self.buffer.find(b"\n")
            if newline >= 0:
                line = bytes(self.buffer[:newline])
                del self.buffer[: newline + 1]
                if line.endswith(b"\r"):
                    line = line[:-1]
                if len(line) > maximum:
                    raise FrameTooLarge("frame exceeds configured maximum")
                return line

            if len(self.buffer) > maximum:
                raise FrameTooLarge("frame exceeds configured maximum")

            read_chunk = getattr(self.stream, "read1", self.stream.read)
            chunk = read_chunk(8192)
            if not chunk:
                if not self.buffer:
                    return None
                line = bytes(self.buffer)
                self.buffer.clear()
                if len(line) > maximum:
                    raise FrameTooLarge("frame exceeds configured maximum")
                return line
            self.buffer.extend(chunk)


def _bounded_json(value: dict[str, Any], maximum: int) -> bytes:
    encoded = json.dumps(
        value, ensure_ascii=False, separators=(",", ":"), sort_keys=True
    ).encode("utf-8")
    if len(encoded) > maximum:
        raise FrameTooLarge("outbound frame exceeds configured maximum")
    return encoded


def write_protocol(value: dict[str, Any]) -> None:
    sys.stdout.buffer.write(_bounded_json(value, FRAME_MAX_BYTES) + b"\n")
    sys.stdout.buffer.flush()


def write_log(level: str, event: str, **fields: Any) -> None:
    allowed = {
        "timestamp": None,
        "level": level,
        "component": "python-backend",
        "event": event,
    }
    for key in ("requestId", "traceId", "errorCode"):
        candidate = fields.get(key)
        if isinstance(candidate, str):
            allowed[key] = candidate[:128]
    encoded = _bounded_json(allowed, LOG_MAX_BYTES)
    sys.stderr.buffer.write(encoded + b"\n")
    sys.stderr.buffer.flush()


def hello() -> dict[str, Any]:
    build = load_build_info()
    return {
        "protocol": PROTOCOL,
        "kind": "hello",
        "protocolMin": 1,
        "protocolMax": 1,
        "backendVersion": __version__,
        "buildId": build["buildId"],
        "targetTriple": build["targetTriple"],
        "pythonVersion": platform_python_version(),
        "schemaHash": build["schemaHash"],
        "supportedOperations": list(SUPPORTED_OPERATIONS),
    }


def platform_python_version() -> str:
    return ".".join(str(part) for part in sys.version_info[:3])


def safe_error(
    code: str,
    message: str,
    request_id: str | None = None,
    trace_id: str = "backend-protocol",
) -> dict[str, Any]:
    return {
        "protocol": PROTOCOL,
        "kind": "error",
        "requestId": request_id,
        "traceId": trace_id[:128],
        "error": {
            "code": code,
            "message": message[:256],
        },
    }


def handle_request(value: Any) -> dict[str, Any] | None:
    if not isinstance(value, dict):
        return safe_error("VALIDATION_ERROR", "Request must be an object.")

    if value.get("protocol") == PROTOCOL and value.get("kind") == "shutdown":
        if set(value) != {"protocol", "kind"}:
            return safe_error(
                "VALIDATION_ERROR", "Shutdown contains unexpected fields."
            )
        return None

    request_id = value.get("requestId")
    trace_id = value.get("traceId")
    safe_request_id = request_id if isinstance(request_id, str) else None
    safe_trace_id = trace_id if isinstance(trace_id, str) else "backend-validation"

    expected_keys = {
        "protocol",
        "kind",
        "requestId",
        "traceId",
        "operation",
        "payload",
    }
    if set(value) != expected_keys:
        return safe_error(
            "VALIDATION_ERROR",
            "Request fields are invalid.",
            safe_request_id,
            safe_trace_id,
        )
    if (
        value["protocol"] != PROTOCOL
        or value["kind"] != "request"
        or not isinstance(request_id, str)
        or not 1 <= len(request_id) <= 128
        or not isinstance(trace_id, str)
        or not 1 <= len(trace_id) <= 128
    ):
        return safe_error(
            "VALIDATION_ERROR",
            "Request envelope is invalid.",
            safe_request_id,
            safe_trace_id,
        )

    operation = value["operation"]
    if operation not in SUPPORTED_OPERATIONS:
        return safe_error(
            "VALIDATION_ERROR",
            "Operation is not supported.",
            request_id,
            trace_id,
        )

    payload = value["payload"]
    if (
        not isinstance(payload, dict)
        or set(payload) != {"text"}
        or not isinstance(payload["text"], str)
        or len(payload["text"]) > TEXT_MAX_CHARACTERS
    ):
        return safe_error(
            "VALIDATION_ERROR",
            "Echo payload is invalid.",
            request_id,
            trace_id,
        )

    write_log("info", "echo_completed", requestId=request_id, traceId=trace_id)
    return {
        "protocol": PROTOCOL,
        "kind": "result",
        "requestId": request_id,
        "traceId": trace_id,
        "operation": "spike.echo",
        "payload": {"text": payload["text"]},
    }


def serve(stdin: BinaryIO) -> int:
    write_protocol(hello())
    write_log("info", "backend_ready")
    reader = BoundedLineReader(stdin)

    while True:
        try:
            line = reader.read_line(FRAME_MAX_BYTES)
        except FrameTooLarge:
            write_protocol(
                safe_error(
                    "RESOURCE_EXHAUSTED",
                    "Input frame exceeds the 1 MiB limit.",
                )
            )
            write_log(
                "error",
                "frame_rejected",
                errorCode="RESOURCE_EXHAUSTED",
            )
            return 2

        if line is None:
            write_log("info", "eof_shutdown")
            return 0
        if not line:
            write_protocol(safe_error("PROTOCOL_ERROR", "Empty frame."))
            continue

        try:
            decoded = line.decode("utf-8")
            value = json.loads(decoded)
        except (UnicodeDecodeError, json.JSONDecodeError):
            write_protocol(safe_error("PROTOCOL_ERROR", "Malformed UTF-8 JSON."))
            write_log("warning", "malformed_frame", errorCode="PROTOCOL_ERROR")
            continue

        response = handle_request(value)
        if response is None:
            write_log("info", "requested_shutdown")
            return 0
        write_protocol(response)
