from __future__ import annotations

import json
import select
import sys
import time
from pathlib import Path
from typing import Any

from wp02_protocol_bounds import Sidecar, request, sidecar_executable

ROOT = Path(__file__).resolve().parents[2]
TERMINAL_STATES = {"Succeeded", "Failed", "Cancelled"}


def read_until_terminal(sidecar: Sidecar, timeout: float = 3) -> list[dict[str, Any]]:
    deadline = time.monotonic() + timeout
    events: list[dict[str, Any]] = []
    while time.monotonic() < deadline:
        value = sidecar.read_frame(timeout=deadline - time.monotonic())
        events.append(value)
        if value.get("kind") == "taskEvent" and value.get("state") in TERMINAL_STATES:
            return events
    raise AssertionError("task did not reach a terminal state")


def verify_success_then_late_cancel(executable: Path) -> dict[str, Any]:
    sidecar = Sidecar(executable)
    try:
        sidecar.send(request("success-race", "spike.count", {"countTo": 1, "intervalMs": 10}))
        accepted = sidecar.read_frame()
        assert accepted["kind"] == "accepted"
        events = read_until_terminal(sidecar)
        assert events[-1]["state"] == "Succeeded"
        sidecar.send(
            {
                "protocol": "generic-app",
                "kind": "cancel",
                "requestId": "late-cancel",
                "traceId": "late-cancel-trace",
                "taskId": accepted["taskId"],
            }
        )
        acknowledgement = sidecar.read_frame()
        assert acknowledgement["kind"] == "cancelAck"
        assert acknowledgement["accepted"] is False
        sidecar.shutdown()
        sidecar.assert_safe_logs()
        return {
            "terminal": events[-1]["state"],
            "terminalCount": sum(
                event.get("state") in TERMINAL_STATES for event in events
            ),
            "lateCancelAccepted": acknowledgement["accepted"],
        }
    finally:
        sidecar.kill()


def verify_cooperative_cancel(executable: Path) -> dict[str, Any]:
    sidecar = Sidecar(executable)
    try:
        sidecar.send(
            request(
                "cooperative-cancel",
                "spike.count",
                {"countTo": 100, "intervalMs": 100},
            )
        )
        accepted = sidecar.read_frame()
        started = time.monotonic()
        sidecar.send(
            {
                "protocol": "generic-app",
                "kind": "cancel",
                "requestId": "cooperative-cancel-request",
                "traceId": "cooperative-cancel-trace",
                "taskId": accepted["taskId"],
            }
        )
        acknowledgement = sidecar.read_frame()
        acknowledgement_ms = (time.monotonic() - started) * 1000
        assert acknowledgement["kind"] == "cancelAck"
        assert acknowledgement["accepted"] is True
        assert acknowledgement_ms <= 250
        events = read_until_terminal(sidecar)
        cooperative_stop_ms = (time.monotonic() - started) * 1000
        assert cooperative_stop_ms <= 2_000
        assert events[-1]["state"] == "Cancelled"
        assert (
            sum(event.get("state") in TERMINAL_STATES for event in events) == 1
        )
        sidecar.shutdown()
        sidecar.assert_safe_logs()
        return {
            "acknowledgementMs": round(acknowledgement_ms, 3),
            "cooperativeStopMs": round(cooperative_stop_ms, 3),
            "terminal": events[-1]["state"],
            "terminalCount": 1,
        }
    finally:
        sidecar.kill()


def verify_deliberate_crash(executable: Path) -> dict[str, Any]:
    sidecar = Sidecar(executable)
    try:
        sidecar.send(request("crash-case", "spike.crash", {}))
        assert sidecar.read_frame()["kind"] == "accepted"
        exit_code = sidecar.process.wait(timeout=3)
        assert exit_code == 70
        sidecar.log_thread.join(timeout=2)
        sidecar.assert_safe_logs()
        return {"accepted": True, "exitCode": exit_code}
    finally:
        sidecar.kill()


def verify_deliberate_hang(executable: Path) -> dict[str, Any]:
    sidecar = Sidecar(executable)
    try:
        sidecar.send(request("hang-case", "spike.hang", {}))
        accepted = sidecar.read_frame()
        started = time.monotonic()
        sidecar.send(
            {
                "protocol": "generic-app",
                "kind": "cancel",
                "requestId": "hang-cancel",
                "traceId": "hang-cancel-trace",
                "taskId": accepted["taskId"],
            }
        )
        acknowledgement = sidecar.read_frame()
        acknowledgement_ms = (time.monotonic() - started) * 1000
        assert acknowledgement["kind"] == "cancelAck"
        assert acknowledgement["accepted"] is True
        assert acknowledgement_ms <= 250
        assert sidecar.process.stdout is not None
        ready, _, _ = select.select([sidecar.process.stdout], [], [], 0.3)
        assert not ready, "deliberate hang unexpectedly claimed a terminal state"
        sidecar.process.kill()
        sidecar.process.wait(timeout=2)
        sidecar.log_thread.join(timeout=2)
        sidecar.assert_safe_logs()
        return {
            "cancelAcknowledgementMs": round(acknowledgement_ms, 3),
            "terminalBeforeEscalation": False,
            "externalEscalationExitCode": sidecar.process.returncode,
        }
    finally:
        sidecar.kill()


def main() -> int:
    executable = sidecar_executable()
    evidence = {
        "status": "passed",
        "executable": executable.name,
        "successCancelRace": verify_success_then_late_cancel(executable),
        "cooperativeCancellation": verify_cooperative_cancel(executable),
        "deliberateCrash": verify_deliberate_crash(executable),
        "deliberateHang": verify_deliberate_hang(executable),
        "restartCircuitNoReplayProof": (
            "cargo test --locked -- --include-ignored"
        ),
        "nativeContainmentProof": "pnpm native:verify",
    }
    print(json.dumps(evidence, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
