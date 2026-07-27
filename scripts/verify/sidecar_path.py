from __future__ import annotations

import platform
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
os_name = {"darwin": "macos", "linux": "linux", "win32": "windows"}[sys.platform]
machine = {"amd64": "x86_64", "x64": "x86_64", "arm64": "aarch64"}.get(
    platform.machine().lower(), platform.machine().lower()
)
executable = "prime-shell-python-backend.exe" if sys.platform == "win32" else "prime-shell-python-backend"
print(
    ROOT
    / "services"
    / "python-backend"
    / "dist"
    / "sidecar"
    / f"{os_name}-{machine}"
    / "prime-shell-python-backend"
    / executable
)

