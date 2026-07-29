from __future__ import annotations

import json
import os
import sys
import threading
import time
from dataclasses import dataclass
from typing import Any, BinaryIO

from . import __version__
from .metadata import load_build_info

HANDSHAKE_MAX_BYTES = 64 * 1024
FRAME_MAX_BYTES = 1024 * 1024
LOG_MAX_BYTES = 64 * 1024
TEXT_MAX_CHARACTERS = 262_144
PROTOCOL = "generic-app"
SUPPORTED_OPERATIONS = (
    "spike.echo",
    "spike.count",
    "spike.crash",
    "spike.hang",
    "spike.largeRejected",
)
COUNT_MAX = 10_000
COUNT_INTERVAL_MIN_MS = 10
COUNT_INTERVAL_MAX_MS = 1_000
LARGE_REJECTED_MIN_BYTES = FRAME_MAX_BYTES + 1
_STDOUT_LOCK = threading.Lock()
_STDERR_LOCK = threading.Lock()


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
    encoded = _bounded_json(value, FRAME_MAX_BYTES) + b"\n"
    with _STDOUT_LOCK:
        sys.stdout.buffer.write(encoded)
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
    with _STDERR_LOCK:
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


@dataclass(frozen=True)
class ValidatedRequest:
    request_id: str
    trace_id: str
    operation: str
    payload: dict[str, Any]


@dataclass
class ActiveTask:
    request: ValidatedRequest
    task_id: str
    cancel: threading.Event


class SidecarRuntime:
    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._active: ActiveTask | None = None
        self._task_sequence = 0

    def start(self, request: ValidatedRequest) -> dict[str, Any]:
        with self._lock:
            if self._active is not None:
                return safe_error(
                    "CONFLICT",
                    "A long-running task is already active.",
                    request.request_id,
                    request.trace_id,
                )
            self._task_sequence += 1
            task = ActiveTask(
                request=request,
                task_id=f"task-{self._task_sequence}",
                cancel=threading.Event(),
            )
            self._active = task

        accepted = {
            "protocol": PROTOCOL,
            "kind": "accepted",
            "requestId": request.request_id,
            "traceId": request.trace_id,
            "operation": request.operation,
            "taskId": task.task_id,
        }
        write_protocol(accepted)
        worker = threading.Thread(
            target=self._run_task,
            args=(task,),
            name="prime-shell-task",
            daemon=True,
        )
        worker.start()
        return accepted

    def cancel(self, value: Any) -> dict[str, Any]:
        parsed = validate_cancel(value)
        if isinstance(parsed, dict):
            return parsed
        request_id, trace_id, task_id = parsed
        with self._lock:
            task = self._active
            accepted = task is not None and task.task_id == task_id
            if accepted:
                task.cancel.set()
        return {
            "protocol": PROTOCOL,
            "kind": "cancelAck",
            "requestId": request_id,
            "traceId": trace_id,
            "taskId": task_id,
            "accepted": accepted,
        }

    def request_shutdown(self) -> None:
        with self._lock:
            if self._active is not None:
                self._active.cancel.set()

    def _finish(self, task: ActiveTask) -> None:
        with self._lock:
            if self._active is task:
                self._active = None

    def _event(
        self,
        task: ActiveTask,
        sequence: int,
        state: str,
        *,
        progress: dict[str, int] | None = None,
        result: dict[str, int] | None = None,
        error: dict[str, str] | None = None,
    ) -> None:
        value: dict[str, Any] = {
            "protocol": PROTOCOL,
            "kind": "taskEvent",
            "requestId": task.request.request_id,
            "traceId": task.request.trace_id,
            "taskId": task.task_id,
            "sequence": sequence,
            "state": state,
        }
        if progress is not None:
            value["progress"] = progress
        if result is not None:
            value["result"] = result
        if error is not None:
            value["error"] = error
        write_protocol(value)

    def _run_task(self, task: ActiveTask) -> None:
        try:
            if task.request.operation == "spike.count":
                self._run_count(task)
            elif task.request.operation == "spike.crash":
                write_log(
                    "error",
                    "deliberate_crash",
                    requestId=task.request.request_id,
                    traceId=task.request.trace_id,
                )
                time.sleep(0.02)
                os._exit(70)
            elif task.request.operation == "spike.hang":
                write_log(
                    "warning",
                    "deliberate_hang",
                    requestId=task.request.request_id,
                    traceId=task.request.trace_id,
                )
                while True:
                    time.sleep(0.1)
        except Exception:
            self._event(
                task,
                1,
                "Failed",
                error={
                    "code": "INTERNAL_ERROR",
                    "message": "The synthetic task failed.",
                },
            )
            write_log(
                "error",
                "task_failed",
                requestId=task.request.request_id,
                traceId=task.request.trace_id,
                errorCode="INTERNAL_ERROR",
            )
        finally:
            if task.request.operation != "spike.hang":
                self._finish(task)

    def _run_count(self, task: ActiveTask) -> None:
        target = task.request.payload["countTo"]
        interval_seconds = task.request.payload["intervalMs"] / 1_000
        sequence = 0
        for current in range(1, target + 1):
            if task.cancel.wait(interval_seconds):
                sequence += 1
                self._event(
                    task,
                    sequence,
                    "Cancelled",
                    error={
                        "code": "CANCELLED",
                        "message": "The count task was cancelled.",
                    },
                )
                write_log(
                    "info",
                    "count_cancelled",
                    requestId=task.request.request_id,
                    traceId=task.request.trace_id,
                    errorCode="CANCELLED",
                )
                return
            sequence += 1
            self._event(
                task,
                sequence,
                "Running",
                progress={"current": current, "target": target},
            )
            write_log(
                "info",
                "count_progress",
                requestId=task.request.request_id,
                traceId=task.request.trace_id,
            )

        sequence += 1
        self._event(
            task,
            sequence,
            "Succeeded",
            result={"count": target},
        )
        write_log(
            "info",
            "count_succeeded",
            requestId=task.request.request_id,
            traceId=task.request.trace_id,
        )


def validate_request(value: Any) -> ValidatedRequest | dict[str, Any]:
    if not isinstance(value, dict):
        return safe_error("VALIDATION_ERROR", "Request must be an object.")

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
    if not isinstance(payload, dict):
        return safe_error(
            "VALIDATION_ERROR", "Operation payload is invalid.", request_id, trace_id
        )
    if operation == "spike.echo":
        valid_payload = (
            set(payload) == {"text"}
            and isinstance(payload["text"], str)
            and len(payload["text"]) <= TEXT_MAX_CHARACTERS
        )
    elif operation == "spike.count":
        valid_payload = (
            set(payload) == {"countTo", "intervalMs"}
            and isinstance(payload["countTo"], int)
            and not isinstance(payload["countTo"], bool)
            and 1 <= payload["countTo"] <= COUNT_MAX
            and isinstance(payload["intervalMs"], int)
            and not isinstance(payload["intervalMs"], bool)
            and COUNT_INTERVAL_MIN_MS
            <= payload["intervalMs"]
            <= COUNT_INTERVAL_MAX_MS
        )
    elif operation in {"spike.crash", "spike.hang"}:
        valid_payload = not payload
    else:
        valid_payload = (
            set(payload) == {"requestedBytes"}
            and isinstance(payload["requestedBytes"], int)
            and not isinstance(payload["requestedBytes"], bool)
            and LARGE_REJECTED_MIN_BYTES
            <= payload["requestedBytes"]
            <= 8 * FRAME_MAX_BYTES
        )
    if not valid_payload:
        return safe_error(
            "VALIDATION_ERROR",
            "Operation payload is invalid.",
            request_id,
            trace_id,
        )
    return ValidatedRequest(request_id, trace_id, operation, payload)


def validate_cancel(
    value: Any,
) -> tuple[str, str, str] | dict[str, Any]:
    if not isinstance(value, dict):
        return safe_error("VALIDATION_ERROR", "Cancel must be an object.")
    request_id = value.get("requestId")
    trace_id = value.get("traceId")
    task_id = value.get("taskId")
    safe_request_id = request_id if isinstance(request_id, str) else None
    safe_trace_id = trace_id if isinstance(trace_id, str) else "backend-validation"
    if (
        set(value) != {"protocol", "kind", "requestId", "traceId", "taskId"}
        or value.get("protocol") != PROTOCOL
        or value.get("kind") != "cancel"
        or not isinstance(request_id, str)
        or not 1 <= len(request_id) <= 128
        or not isinstance(trace_id, str)
        or not 1 <= len(trace_id) <= 128
        or not isinstance(task_id, str)
        or not 1 <= len(task_id) <= 128
    ):
        return safe_error(
            "VALIDATION_ERROR",
            "Cancel envelope is invalid.",
            safe_request_id,
            safe_trace_id,
        )
    return request_id, trace_id, task_id


def echo_result(request: ValidatedRequest) -> dict[str, Any]:
    write_log(
        "info",
        "echo_completed",
        requestId=request.request_id,
        traceId=request.trace_id,
    )
    return {
        "protocol": PROTOCOL,
        "kind": "result",
        "requestId": request.request_id,
        "traceId": request.trace_id,
        "operation": "spike.echo",
        "payload": {"text": request.payload["text"]},
    }


def handle_request(value: Any) -> dict[str, Any] | None:
    request = validate_request(value)
    if isinstance(request, dict):
        return request
    if request.operation == "spike.echo":
        return echo_result(request)
    if request.operation == "spike.largeRejected":
        return safe_error(
            "RESOURCE_EXHAUSTED",
            "The requested payload exceeds the configured protocol limit.",
            request.request_id,
            request.trace_id,
        )
    return safe_error(
        "CONFLICT",
        "Long-running operations require the sidecar runtime.",
        request.request_id,
        request.trace_id,
    )


def serve(stdin: BinaryIO) -> int:
    write_protocol(hello())
    write_log("info", "backend_ready")
    reader = BoundedLineReader(stdin)
    runtime = SidecarRuntime()

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

        if (
            isinstance(value, dict)
            and value.get("protocol") == PROTOCOL
            and value.get("kind") == "shutdown"
        ):
            if set(value) != {"protocol", "kind"}:
                write_protocol(
                    safe_error(
                        "VALIDATION_ERROR", "Shutdown contains unexpected fields."
                    )
                )
                continue
            runtime.request_shutdown()
            write_log("info", "requested_shutdown")
            return 0

        if isinstance(value, dict) and value.get("kind") == "cancel":
            write_protocol(runtime.cancel(value))
            continue

        request = validate_request(value)
        if isinstance(request, dict):
            write_protocol(request)
        elif request.operation == "spike.echo":
            write_protocol(echo_result(request))
        elif request.operation == "spike.largeRejected":
            write_protocol(
                safe_error(
                    "RESOURCE_EXHAUSTED",
                    "The requested payload exceeds the configured protocol limit.",
                    request.request_id,
                    request.trace_id,
                )
            )
        else:
            response = runtime.start(request)
            if response["kind"] == "error":
                write_protocol(response)
