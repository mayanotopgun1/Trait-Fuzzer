#!/usr/bin/env python3
"""Batch compile all .rs files under a seeds directory using nightly toolchain.

Usage:
  python3 utils/compile_seeds_nightly.py --seeds seeds --timeout 10

This script protects against individual crashes/hangs and prints a final summary:
  SUCCESS, ERROR, CRASH, HANG, UNKNOWN
"""
from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path
from typing import Dict


ICE_MARKERS = (
    "internal compiler error",
    "thread 'rustc' panicked",
    "thread 'rust1' panicked",
    "this is a bug in the compiler",
    "please submit a full bug report",
)


def is_crash(stdout: str, stderr: str) -> bool:
    text = f"{stderr or ''}\n{stdout or ''}".lower()
    return any(m in text for m in ICE_MARKERS)


def compile_file(path: Path, timeout_sec: int) -> str:
    out_bin = path.parent / (path.stem + "_out")
    # use rustup to run nightly toolchain explicitly
    cmd = ["rustup", "run", "nightly", "rustc", str(path), "-o", str(out_bin)]
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout_sec)
    except subprocess.TimeoutExpired:
        return "HANG"
    except FileNotFoundError:
        return "UNKNOWN"
    except Exception:
        return "UNKNOWN"

    if is_crash(proc.stdout, proc.stderr):
        return "CRASH"
    if proc.returncode == 0:
        return "SUCCESS"
    return "ERROR"


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(description="Compile all .rs seeds with nightly and summarize results")
    p.add_argument("--seeds", default="seeds", help="Directory containing .rs seeds")
    p.add_argument("--timeout", type=int, default=10, help="Per-file compile timeout seconds")
    args = p.parse_args(argv)

    seed_root = Path(args.seeds)
    if not seed_root.exists():
        print(f"Seeds directory not found: {seed_root}")
        return 2

    files = sorted([p for p in seed_root.rglob("*.rs") if p.is_file() and p.stat().st_size > 0])
    if not files:
        print("No .rs files found under", seed_root)
        return 0

    counts: Dict[str, int] = {"SUCCESS": 0, "ERROR": 0, "CRASH": 0, "HANG": 0, "UNKNOWN": 0}

    for idx, f in enumerate(files, start=1):
        try:
            status = compile_file(f, timeout_sec=int(args.timeout))
        except Exception as e:
            status = "UNKNOWN"
        counts.setdefault(status, 0)
        counts[status] += 1
        print(f"[{idx}/{len(files)}] {f} -> {status}")

    print("\nSummary:")
    print(f"  SUCCESS: {counts.get('SUCCESS',0)}")
    print(f"  ERROR:   {counts.get('ERROR',0)}")
    print(f"  CRASH:   {counts.get('CRASH',0)}")
    print(f"  HANG:    {counts.get('HANG',0)}")
    print(f"  UNKNOWN: {counts.get('UNKNOWN',0)}")

    return 0


if __name__ == '__main__':
    raise SystemExit(main())
