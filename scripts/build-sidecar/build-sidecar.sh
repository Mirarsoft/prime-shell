#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
python_bin="${repository_root}/.venv-build/bin/python"
backend_root="${repository_root}/services/python-backend"
schema_root="${repository_root}/packages/app-contracts/schemas"
work_root="${backend_root}/build/pyinstaller"

if [[ ! -x "${python_bin}" ]]; then
  echo "missing ${python_bin}; create the pinned build environment first" >&2
  exit 2
fi

case "$(uname -s)" in
  Darwin) platform_name="macos" ;;
  Linux) platform_name="linux" ;;
  MINGW*|MSYS*|CYGWIN*) platform_name="windows" ;;
  *) echo "unsupported host platform" >&2; exit 2 ;;
esac

case "$(uname -m)" in
  x86_64|amd64) architecture="x86_64" ;;
  arm64|aarch64) architecture="aarch64" ;;
  *) echo "unsupported host architecture" >&2; exit 2 ;;
esac

target_identity="${platform_name}-${architecture}"
schema_hash="$("${python_bin}" "${repository_root}/scripts/verify/schema_hash.py" "${schema_root}")"
build_id="$("${python_bin}" - "${backend_root}/src" "${backend_root}/entrypoint.py" <<'PY'
from __future__ import annotations
import hashlib
import sys
from pathlib import Path

source = Path(sys.argv[1])
entrypoint = Path(sys.argv[2])
digest = hashlib.sha256()
paths = sorted(source.rglob("*.py")) + [entrypoint]
for path in paths:
    digest.update(path.name.encode("utf-8"))
    digest.update(b"\0")
    digest.update(path.read_bytes())
print("sha256:" + digest.hexdigest())
PY
)"

rm -rf "${work_root}"
mkdir -p "${work_root}/metadata" "${work_root}/dist" "${work_root}/work" "${work_root}/spec"

"${python_bin}" - "${work_root}/metadata/build_info.json" "${build_id}" "${schema_hash}" "${target_identity}" <<'PY'
from __future__ import annotations
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
path.write_text(
    json.dumps(
        {
            "buildId": sys.argv[2],
            "schemaHash": sys.argv[3],
            "targetTriple": sys.argv[4],
        },
        indent=2,
        sort_keys=True,
    )
    + "\n",
    encoding="utf-8",
)
PY

"${python_bin}" -m PyInstaller \
  --clean \
  --noconfirm \
  --onedir \
  --console \
  --name prime-shell-python-backend \
  --paths "${backend_root}/src" \
  --add-data "${work_root}/metadata/build_info.json:prime_shell_backend" \
  --distpath "${work_root}/dist" \
  --workpath "${work_root}/work" \
  --specpath "${work_root}/spec" \
  "${backend_root}/entrypoint.py"

output_root="${backend_root}/dist/sidecar/${target_identity}"
rm -rf "${output_root}"
mkdir -p "${output_root}"
cp -a "${work_root}/dist/prime-shell-python-backend" "${output_root}/"

"${python_bin}" - "${output_root}/sidecar-manifest.json" "${build_id}" "${schema_hash}" "${target_identity}" <<'PY'
from __future__ import annotations
import hashlib
import json
import sys
from pathlib import Path

manifest_path = Path(sys.argv[1])
bundle = manifest_path.parent / "prime-shell-python-backend"
files = []
for path in sorted(item for item in bundle.rglob("*") if item.is_file()):
    files.append(
        {
            "path": path.relative_to(bundle).as_posix(),
            "size": path.stat().st_size,
            "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        }
    )
manifest_path.write_text(
    json.dumps(
        {
            "backendVersion": "0.1.0",
            "buildId": sys.argv[2],
            "schemaHash": sys.argv[3],
            "targetTriple": sys.argv[4],
            "bundleBytes": sum(item["size"] for item in files),
            "files": files,
        },
        indent=2,
        sort_keys=True,
    )
    + "\n",
    encoding="utf-8",
)
PY

echo "${output_root}"

