from __future__ import annotations

import sys

from .protocol import serve


def main() -> int:
    return serve(sys.stdin.buffer)


if __name__ == "__main__":
    raise SystemExit(main())

