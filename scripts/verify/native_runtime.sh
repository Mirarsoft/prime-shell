#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

EVIDENCE_DIR="$ROOT/artifacts/wp01-native-runtime"
EVIDENCE_FILE="$EVIDENCE_DIR/native-runtime-evidence.json"
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
for snippet in ("Packaged Unicode Echo", "Backend: Ready", expected):
    if snippet not in rendered:
        raise SystemExit(f"rendered text missing {snippet!r}")

summary = {
    "status": "passed",
    "evidence": str(path),
    "windowRendered": data["windowRendered"],
    "fluentRendered": data["fluentRendered"],
    "releaseCspViolationCount": data["releaseCspViolationCount"],
    "backendReady": data["backendReady"],
    "backendVersion": data["backendVersion"],
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
