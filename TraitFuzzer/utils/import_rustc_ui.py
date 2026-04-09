#!/usr/bin/env python3
"""Import rustc's src/test/ui .rs files into this project's seeds.

Goal: extend seeds/rust-official/ui with *missing* rustc UI tests.

Default behavior is conservative:
- Does NOT overwrite existing destination paths.
- Skips any file under an 'auxiliary' directory (not standalone) unless --include-auxiliary.

Usage examples:
  # If you already have a rust-lang/rust checkout:
  python tools/import_rustc_ui.py --src /path/to/rust --dst seeds/rust-official/ui

  # If you only have the ui directory:
  python tools/import_rustc_ui.py --src /path/to/rust/src/test/ui --dst seeds/rust-official/ui

Notes:
- The fuzzer already has filtering/skip logic for non-parsable/internal-only seeds;
  importing more UI tests is safe but will increase scan time.
"""

from __future__ import annotations

import argparse
import hashlib
import os
import shutil
from dataclasses import dataclass
from pathlib import Path


@dataclass
class Stats:
    scanned: int = 0
    copied: int = 0
    skipped_auxiliary: int = 0
    skipped_existing_same: int = 0
    skipped_existing_conflict: int = 0


def _sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def _resolve_ui_dir(src: Path) -> Path:
    src = src.resolve()
    # Accept direct ui dir paths.
    if src.name == "ui":
        # Legacy layout: src/test/ui
        if src.parent.name == "test" and src.parent.parent.name == "src":
            return src
        # Current layout: tests/ui
        if src.parent.name == "tests":
            return src

    # Accept repo root paths.
    candidates = [
        src / "tests" / "ui",  # current rust-lang/rust layout
        src / "src" / "test" / "ui",  # legacy layout
    ]
    for candidate in candidates:
        if candidate.is_dir():
            return candidate

    raise SystemExit(
        "Could not find rustc ui dir. Provide either a rust-lang/rust repo root or the 'tests/ui' (or legacy 'src/test/ui') dir. "
        f"Got: {src}"
    )


def import_ui(src: Path, dst: Path, *, include_auxiliary: bool) -> Stats:
    ui_dir = _resolve_ui_dir(src)
    dst = dst.resolve()

    if not ui_dir.is_dir():
        raise SystemExit(f"ui_dir is not a directory: {ui_dir}")
    dst.mkdir(parents=True, exist_ok=True)

    stats = Stats()

    for rs_path in ui_dir.rglob("*.rs"):
        if not rs_path.is_file():
            continue

        stats.scanned += 1

        if not include_auxiliary and "auxiliary" in rs_path.parts:
            stats.skipped_auxiliary += 1
            continue

        rel = rs_path.relative_to(ui_dir)
        out_path = dst / rel
        out_path.parent.mkdir(parents=True, exist_ok=True)

        if out_path.exists():
            if _sha256(out_path) == _sha256(rs_path):
                stats.skipped_existing_same += 1
            else:
                stats.skipped_existing_conflict += 1
            continue

        shutil.copy2(rs_path, out_path)
        stats.copied += 1

    return stats


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--src",
        required=True,
        help="rust-lang/rust repo root, OR the src/test/ui directory",
    )
    parser.add_argument(
        "--dst",
        default="seeds/rust-official/ui",
        help="destination seeds directory (default: seeds/rust-official/ui)",
    )
    parser.add_argument(
        "--include-auxiliary",
        action="store_true",
        help="include files under any 'auxiliary' directory (not standalone; usually not useful as seeds)",
    )

    args = parser.parse_args()
    stats = import_ui(Path(args.src), Path(args.dst), include_auxiliary=args.include_auxiliary)

    print(
        "\n".join(
            [
                "import_rustc_ui summary:",
                f"  scanned:                 {stats.scanned}",
                f"  copied:                  {stats.copied}",
                f"  skipped_auxiliary:       {stats.skipped_auxiliary}",
                f"  skipped_existing_same:   {stats.skipped_existing_same}",
                f"  skipped_existing_conflict:{stats.skipped_existing_conflict}",
            ]
        )
    )


if __name__ == "__main__":
    main()
