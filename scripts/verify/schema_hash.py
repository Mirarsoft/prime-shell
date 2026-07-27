from __future__ import annotations

import hashlib
import sys
from pathlib import Path


def schema_hash(root: Path) -> str:
    digest = hashlib.sha256()
    for path in sorted(root.rglob("*.json")):
        digest.update(path.relative_to(root).as_posix().encode("utf-8"))
        digest.update(b"\0")
        digest.update(path.read_bytes())
    return "sha256:" + digest.hexdigest()


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: schema_hash.py SCHEMA_DIRECTORY")
    print(schema_hash(Path(sys.argv[1]).resolve()))

