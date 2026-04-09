#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import shutil
from pathlib import Path


def _load_json(path: Path) -> dict:
    try:
        with open(path, "r", encoding="utf-8") as f:
            return json.load(f)
    except Exception:
        return {}


def clean_directory(path: Path, preserve_dirs=None):
    preserve_dirs = set(preserve_dirs or [])
    if not path.exists():
        return
    for item in path.iterdir():
        if item.is_dir() and item.name in preserve_dirs:
            print(f"Preserved directory {item}")
            continue
        try:
            if item.is_dir():
                shutil.rmtree(item)
                print(f"Removed directory {item}")
            else:
                item.unlink()
                print(f"Removed file {item}")
        except Exception as e:
            print(f"Failed to remove {item}: {e}")


def truncate_file(path: Path):
    try:
        if path.exists() and path.is_file():
            with open(path, "w", encoding="utf-8"):
                pass
            print(f"Truncated {path}")
    except Exception as e:
        print(f"Failed to truncate {path}: {e}")


def remove_temp_like(base: Path):
    for item in base.iterdir():
        name = item.name
        should_remove = (
            name.startswith("temp")
            or (name.startswith("llm_") and name.endswith(".dir"))
            or name.startswith("cross_temp")
        )
        if not should_remove:
            continue
        try:
            if item.is_dir():
                shutil.rmtree(item)
                print(f"Removed directory {item}")
            else:
                item.unlink()
                print(f"Removed file {item}")
        except Exception as e:
            print(f"Failed to remove {item}: {e}")


def remove_patterns(base: Path, patterns):
    for pat in patterns:
        for p in base.glob(pat):
            if not p.exists() or p.is_dir():
                continue
            try:
                p.unlink()
                print(f"Removed file {p}")
            except Exception as e:
                print(f"Failed to remove {p}: {e}")


def main():
    parser = argparse.ArgumentParser(description="Clean cross-feature experiment artifacts")
    parser.add_argument("--all", action="store_true", help="Clean results, logs, temp files and lock dirs")
    parser.add_argument("--logs", action="store_true", help="Clean/truncate logs only")
    parser.add_argument("--results", action="store_true", help="Clean result files only")
    parser.add_argument("--config", default="config.json", help="Base config path")
    parser.add_argument(
        "--cross-config",
        default="mutation_crossfeature/config_cross.json",
        help="Cross-feature config path",
    )
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[1]
    cross_dir = Path(__file__).resolve().parent

    base_cfg = _load_json((repo_root / args.config) if not Path(args.config).is_absolute() else Path(args.config))
    cross_cfg_path = Path(args.cross_config)
    if not cross_cfg_path.is_absolute():
        cross_cfg_path = repo_root / cross_cfg_path
    cross_cfg = _load_json(cross_cfg_path)

    cross_paths = cross_cfg.get("paths", {})
    results_rel = cross_paths.get("results", "mutation_crossfeature/results")
    results_dir = (repo_root / results_rel).resolve() if not Path(results_rel).is_absolute() else Path(results_rel)

    clean_logs = bool(args.all or args.logs)
    clean_results = bool(args.all or args.results or (not args.logs and not args.results and not args.all))

    if clean_results:
        clean_directory(results_dir, preserve_dirs={"crash", "hang"})
        print(f"Cleaned results under {results_dir} (preserved: crash, hang)")

    if clean_logs:
        truncate_file(cross_dir / "cross_fuzz.log")
        truncate_file(cross_dir / "cross_ollama.log")
        truncate_file(repo_root / "ollama.log")

    if args.all:
        # lock path comes from base config llm.lock_path
        lock_rel = base_cfg.get("llm", {}).get("lock_path", "llm_global_lock_smoke.dir")
        lock_path = (repo_root / lock_rel) if not Path(lock_rel).is_absolute() else Path(lock_rel)
        if lock_path.exists():
            try:
                if lock_path.is_dir():
                    shutil.rmtree(lock_path)
                else:
                    lock_path.unlink()
                print(f"Removed lock {lock_path}")
            except Exception as e:
                print(f"Failed to remove lock {lock_path}: {e}")

        remove_temp_like(repo_root)
        remove_patterns(repo_root, ["temp_*.rs", "libtemp*.rlib", "libtemp*.rmeta", "libtemp*.d"])


if __name__ == "__main__":
    main()
