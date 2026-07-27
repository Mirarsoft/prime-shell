from __future__ import annotations

import json
import platform
import sys
from pathlib import Path
from typing import Any

_ZERO_HASH = "sha256:" + ("0" * 64)


def target_identity() -> str:
    os_name = {
        "darwin": "macos",
        "linux": "linux",
        "win32": "windows",
    }.get(sys.platform, sys.platform)
    machine = platform.machine().lower()
    architecture = {
        "amd64": "x86_64",
        "x64": "x86_64",
        "arm64": "aarch64",
    }.get(machine, machine)
    return f"{os_name}-{architecture}"


def load_build_info() -> dict[str, Any]:
    info_path = Path(__file__).with_name("build_info.json")
    if info_path.is_file():
        data = json.loads(info_path.read_text(encoding="utf-8"))
        if isinstance(data, dict):
            return data
    return {
        "buildId": _ZERO_HASH,
        "schemaHash": _ZERO_HASH,
        "targetTriple": target_identity(),
    }

