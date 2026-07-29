#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

EVIDENCE_DIR="$ROOT/artifacts/wp02/native-runtime"
EVIDENCE_FILE="$EVIDENCE_DIR/native-runtime-evidence.json"
PROCESS_EVIDENCE_FILE="$EVIDENCE_DIR/process-cleanup-evidence.json"
mkdir -p "$EVIDENCE_DIR"

DEB_PATH="$(find "$ROOT/apps/desktop/src-tauri/target/release/bundle/deb" -maxdepth 1 -name '*.deb' -print -quit)"
if [[ -z "$DEB_PATH" || ! -f "$DEB_PATH" ]]; then
  echo "Tauri deb bundle was not found." >&2
  exit 1
fi

PACKAGE_NAME="$(dpkg-deb -f "$DEB_PATH" Package)"
sudo apt-get install -y "$DEB_PATH"

APP_BINARY="$(
  dpkg-query -L "$PACKAGE_NAME" |
    awk '$0 ~ /^\/usr\/bin\/[^/]+$/ { print; exit }'
)"
if [[ -z "$APP_BINARY" ]]; then
  echo "Could not locate installed app binary for package $PACKAGE_NAME." >&2
  exit 1
fi

rm -f "$EVIDENCE_FILE"
timeout 45s xvfb-run -a env \
  PRIME_SHELL_NATIVE_RUNTIME_VERIFY=1 \
  PRIME_SHELL_RUNTIME_EVIDENCE="$EVIDENCE_FILE" \
  "$APP_BINARY"

python3 - "$EVIDENCE_FILE" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
if not path.is_file():
    raise SystemExit("native runtime evidence file was not written")

data = json.loads(path.read_text(encoding="utf-8"))
expected = "Hello — مرحبا — こんにちは 👋"
checks = {
    "status": data.get("status") == "passed",
    "windowRendered": data.get("windowRendered") is True,
    "fluentRendered": data.get("fluentRendered") is True,
    "releaseCspViolationCount": data.get("releaseCspViolationCount") == 0,
    "backendReady": data.get("backendReady") is True,
    "countCancelAccepted": data.get("countCancelAccepted") is True,
    "cancelAcknowledgementMs": 0
    <= float(data.get("cancelAcknowledgementMs", -1))
    <= 250,
    "cooperativeStopMs": 0 <= float(data.get("cooperativeStopMs", -1)) <= 2000,
    "countTerminalState": data.get("countTerminalState") == "Cancelled",
    "countTerminalCount": data.get("countTerminalCount") == 1,
    "countSequencesMonotonic": data.get("countSequencesMonotonic") is True,
    "countProgressObserved": data.get("countProgressObserved") is True,
    "uiProgressMinimumIntervalMs": float(
        data.get("uiProgressMinimumIntervalMs", -1)
    )
    >= 90,
    "backendStateAfterCount": data.get("backendStateAfterCount") == "Ready",
    "unicodeExactMatch": data.get("unicodeExactMatch") is True,
    "unicodeInput": data.get("unicodeInput") == expected,
    "unicodeOutput": data.get("unicodeOutput") == expected,
    "safeErrorPath": str(data.get("safeErrorPath", "")).startswith(
        "RESOURCE_EXHAUSTED:"
    ),
}
failed = [name for name, ok in checks.items() if not ok]
if failed:
    raise SystemExit(f"native runtime evidence failed checks: {failed}")

rendered = data.get("renderedText", "")
for snippet in (
    "Packaged Backend Resilience",
    "Synthetic count task",
    "Terminal state: Cancelled",
    "Backend: Ready",
    expected,
):
    if snippet not in rendered:
        raise SystemExit(f"rendered text missing {snippet!r}")

summary = {
    "status": "passed",
    "evidence": path.name,
    "windowRendered": data["windowRendered"],
    "fluentRendered": data["fluentRendered"],
    "releaseCspViolationCount": data["releaseCspViolationCount"],
    "backendReady": data["backendReady"],
    "backendVersion": data["backendVersion"],
    "cancelAcknowledgementMs": data["cancelAcknowledgementMs"],
    "cooperativeStopMs": data["cooperativeStopMs"],
    "countTerminalState": data["countTerminalState"],
    "countTerminalCount": data["countTerminalCount"],
    "countSequences": data["countSequences"],
    "uiProgressMinimumIntervalMs": data["uiProgressMinimumIntervalMs"],
    "unicodeExactMatch": data["unicodeExactMatch"],
    "safeErrorPath": data["safeErrorPath"],
}
print(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True))
PY

sleep 1
if pgrep -af 'prime-shell-python-backend' >/tmp/prime-shell-sidecar-processes.txt; then
  cat /tmp/prime-shell-sidecar-processes.txt >&2
  echo "Packaged sidecar process survived native host shutdown." >&2
  exit 1
fi

echo '{"sidecarProcessesAfterNativeClose":0}'

FORCED_HOST_PID_FILE="$EVIDENCE_DIR/forced-host.pid"
FORCED_SIDECAR_PID_FILE="$EVIDENCE_DIR/forced-sidecar.pid"
rm -f \
  "$FORCED_HOST_PID_FILE" \
  "$FORCED_SIDECAR_PID_FILE" \
  "$PROCESS_EVIDENCE_FILE"
xvfb-run -a bash -c '
  app_binary="$1"
  host_pid_file="$2"
  sidecar_pid_file="$3"
  env \
    PRIME_SHELL_NATIVE_FORCED_CLOSE_VERIFY=1 \
    PRIME_SHELL_FORCED_CLOSE_SIDECAR_PID_FILE="$sidecar_pid_file" \
    "$app_binary" &
  app_pid=$!
  printf "%s\n" "$app_pid" >"$host_pid_file"
  wait "$app_pid"
' bash \
  "$APP_BINARY" \
  "$FORCED_HOST_PID_FILE" \
  "$FORCED_SIDECAR_PID_FILE" &
FORCED_WRAPPER_PID=$!

for _ in $(seq 1 100); do
  if [[ -s "$FORCED_HOST_PID_FILE" && -s "$FORCED_SIDECAR_PID_FILE" ]]; then
    break
  fi
  sleep 0.1
done

if [[ ! -s "$FORCED_HOST_PID_FILE" ]]; then
  kill "$FORCED_WRAPPER_PID" 2>/dev/null || true
  wait "$FORCED_WRAPPER_PID" 2>/dev/null || true
  echo "Forced-close native host PID was not recorded." >&2
  exit 1
fi

FORCED_APP_PID="$(cat "$FORCED_HOST_PID_FILE")"
if [[ ! -s "$FORCED_SIDECAR_PID_FILE" ]]; then
  kill "$FORCED_APP_PID" "$FORCED_WRAPPER_PID" 2>/dev/null || true
  wait "$FORCED_WRAPPER_PID" 2>/dev/null || true
  echo "Forced-close sidecar PID was not recorded." >&2
  exit 1
fi

FORCED_SIDECAR_PID="$(cat "$FORCED_SIDECAR_PID_FILE")"
if [[ ! "$FORCED_SIDECAR_PID" =~ ^[0-9]+$ ]] ||
  ! kill -0 "$FORCED_SIDECAR_PID" 2>/dev/null; then
  kill "$FORCED_APP_PID" "$FORCED_WRAPPER_PID" 2>/dev/null || true
  wait "$FORCED_WRAPPER_PID" 2>/dev/null || true
  echo "Recorded forced-close sidecar PID is not active." >&2
  exit 1
fi
SIDECAR_BEFORE_FORCED_CLOSE=1

kill -KILL "$FORCED_APP_PID"
for _ in $(seq 1 30); do
  if ! kill -0 "$FORCED_SIDECAR_PID" 2>/dev/null; then
    break
  fi
  sleep 0.1
done

SIDECAR_AFTER_FORCED_CLOSE=0
if kill -0 "$FORCED_SIDECAR_PID" 2>/dev/null; then
  SIDECAR_AFTER_FORCED_CLOSE=1
  ps -o pid=,ppid=,stat=,comm= -p "$FORCED_SIDECAR_PID" >&2 || true
fi

for _ in $(seq 1 30); do
  if ! kill -0 "$FORCED_WRAPPER_PID" 2>/dev/null; then
    break
  fi
  sleep 0.1
done
if kill -0 "$FORCED_WRAPPER_PID" 2>/dev/null; then
  kill "$FORCED_WRAPPER_PID" 2>/dev/null || true
fi
wait "$FORCED_WRAPPER_PID" 2>/dev/null || true

python3 - "$PROCESS_EVIDENCE_FILE" "$SIDECAR_BEFORE_FORCED_CLOSE" "$SIDECAR_AFTER_FORCED_CLOSE" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
before = int(sys.argv[2])
after = int(sys.argv[3])
evidence = {
    "status": "passed" if before == 1 and after == 0 else "failed",
    "sidecarProcessesBeforeForcedClose": before,
    "sidecarProcessesAfterNormalClose": 0,
    "sidecarProcessesAfterForcedClose": after,
    "containmentDeadlineSeconds": 3,
}
path.write_text(
    json.dumps(evidence, indent=2, sort_keys=True) + "\n",
    encoding="utf-8",
)
print(json.dumps(evidence, indent=2, sort_keys=True))
if evidence["status"] != "passed":
    raise SystemExit("forced native-host close left a sidecar process")
PY
