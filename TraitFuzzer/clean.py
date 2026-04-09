import shutil
import os
import argparse
import json
from pathlib import Path


def remove_files_by_patterns(directory: Path, patterns):
    directory = Path(directory)
    if not directory.exists():
        return

    for pat in patterns:
        for f in directory.glob(pat):
            if not f.is_file():
                continue
            try:
                f.unlink()
                print(f"Removed {f.relative_to(directory)}")
            except Exception as e:
                print(f"Failed to remove {f}: {e}")

def clean_directory(path):
    path = Path(path)
    if path.exists():
        for item in path.iterdir():
            try:
                if item.is_dir():
                    shutil.rmtree(item)
                    print(f"Removed directory {item}")
                else:
                    item.unlink()
                    print(f"Removed file {item}")
            except Exception as e:
                print(f"Failed to remove {item}: {e}")
        print(f"Cleaned {path}")
    else:
        print(f"Directory {path} does not exist")


def _load_config(config_path: Path) -> dict:
    try:
        with open(config_path, "r") as f:
            return json.load(f)
    except Exception as e:
        print(f"Failed to load config {config_path}: {e}")
        return {}


def clean_promoted_seeds(base_dir: Path, config: dict):
    run_cfg = config.get("run", {})
    paths_cfg = config.get("paths", {})
    seeds_rel = paths_cfg.get("seeds", "seeds")
    prefix = str(run_cfg.get("new_seeds_prefix", "new"))

    seeds_dir = (base_dir / seeds_rel).resolve()
    if not seeds_dir.exists() or not seeds_dir.is_dir():
        print(f"Seeds directory {seeds_dir} does not exist")
        return

    removed_any = False
    removed_dirs = []
    for p in seeds_dir.iterdir():
        if not p.is_dir():
            continue
        name = p.name
        if not name.startswith(prefix):
            continue
        suffix = name[len(prefix):]
        if not suffix.isdigit():
            continue
        # Remove only .rs files under promoted dirs, keep directories.
        for f in p.rglob("*.rs"):
            if f.is_file():
                try:
                    f.unlink()
                    removed_any = True
                except Exception as e:
                    print(f"Failed to remove {f}: {e}")
        # Remove empty directories left behind.
        for child in sorted(p.rglob("*"), reverse=True):
            if child.is_dir():
                try:
                    if not any(child.iterdir()):
                        child.rmdir()
                except Exception:
                    pass
        if not any(p.iterdir()):
            try:
                p.rmdir()
            except Exception:
                pass
        if removed_any:
            removed_dirs.append(p)
    if not removed_any:
        print("No promoted seeds found to clean")
    else:
        uniq = sorted({str(p) for p in removed_dirs})
        for d in uniq:
            print(f"Removed promoted dir {d}")


def main():
    parser = argparse.ArgumentParser(description="Clean Trait-Fuzzer results")
    parser.add_argument("--all", action="store_true", help="Clean non-bug results and logs")
    parser.add_argument("--logs", action="store_true", help="Also clean log files")
    parser.add_argument(
        "--clean-bugs",
        action="store_true",
        help="Also clean bug result directories (crash/hang/fate/dup/rewrite/miscompilation)",
    )
    parser.add_argument("--config", default="config.json", help="Path to configuration file")
    args = parser.parse_args()

    base_dir = Path(__file__).parent
    config = _load_config(base_dir / args.config)
    
    # New layout: clean only non-bug results by default.
    clean_directory(base_dir / "results" / "rustc" / "success")
    clean_directory(base_dir / "results" / "rustc" / "error")
    clean_directory(base_dir / "results" / "gccrs" / "success")
    clean_directory(base_dir / "results" / "gccrs" / "error")

    # Backward compatibility: old flat layout non-bug directories.
    clean_directory(base_dir / "results" / "success")
    clean_directory(base_dir / "results" / "error")

    # Bug directories are preserved unless explicitly requested.
    if args.clean_bugs:
        clean_directory(base_dir / "results" / "rustc" / "crash")
        clean_directory(base_dir / "results" / "rustc" / "hang")
        clean_directory(base_dir / "results" / "rustc" / "fate")
        clean_directory(base_dir / "results" / "rustc" / "dup")
        clean_directory(base_dir / "results" / "rustc" / "rewrite")
        clean_directory(base_dir / "results" / "rustc" / "miscompilation")

        clean_directory(base_dir / "results" / "gccrs" / "crash")
        clean_directory(base_dir / "results" / "gccrs" / "hang")
        clean_directory(base_dir / "results" / "gccrs" / "fate")
        clean_directory(base_dir / "results" / "gccrs" / "dup")
        clean_directory(base_dir / "results" / "gccrs" / "rewrite")

        # Legacy bug directories
        clean_directory(base_dir / "results" / "crash")
        clean_directory(base_dir / "results" / "hang")
        clean_directory(base_dir / "results" / "fate")
        clean_directory(base_dir / "results" / "miscompilation")
        clean_directory(base_dir / "results" / "dup")
        clean_directory(base_dir / "results" / "rewrite")
        clean_directory(base_dir / "results" / "gccrs_crash")
        clean_directory(base_dir / "results" / "gccrs_hang")
        print("Bug results cleaned (--clean-bugs enabled)")
    else:
        print("Bug results preserved (use --clean-bugs to remove them)")
    # Rewrites moved under LLM/rewrites
    clean_directory(base_dir / "LLM" / "rewrites")
    clean_directory(base_dir / "results_night" / "success")
    clean_directory(base_dir / "results_night" / "error")
    
    if args.all or args.logs:
        clean_directory(base_dir / "logs")
        # Additionally truncate a couple of long-running log files instead
        # of removing them entirely so processes that hold the files keep working.
        def _truncate_if_exists(path: Path):
            try:
                if path.exists() and path.is_file():
                    with open(path, "w"):
                        pass
                    print(f"Truncated {path}")
            except Exception as e:
                print(f"Failed to truncate {path}: {e}")

        # Common locations: trait-fuzzer dir and project root
        candidates = [base_dir / "my_fuzz.log", base_dir / "ollama.log", base_dir.parent / "my_fuzz.log", base_dir.parent / "ollama.log"]
        for p in candidates:
            _truncate_if_exists(p)
    else:
        print("Logs skipped (use --logs or --all to clean them)")

    # Clean promoted seeds under seeds/<prefix><digits>/ based on config
    clean_promoted_seeds(base_dir, config)
        
    # Clean fuzzer working-directory artifacts.
    # These are produced by rustc and/or the mutation pipeline when runs are interrupted.
    # Examples:
    # - temp_iter_35_var_3 (binary)
    # - libtemp_iter_32_var_4.rlib
    remove_files_by_patterns(
        base_dir,
        patterns=[
            "libtemp*.rlib",
            "libtemp*.rmeta",
            "libtemp*.d",
        ],
    )

    # Remove files or directories in the trait-fuzzer directory that start with 'temp' or 'llm_' (lock dirs)
    for item in base_dir.iterdir():
        if item.name.startswith("temp") or (item.name.startswith("llm_") and item.name.endswith(".dir")):
            try:
                if item.is_dir():
                    shutil.rmtree(item)
                    print(f"Removed directory {item.name}")
                else:
                    item.unlink()
                    print(f"Removed file {item.name}")
            except Exception as e:
                print(f"Failed to remove {item.name}: {e}")

    # Legacy: also clean a small set of common artifacts in the repo root.
    # Keep it conservative to avoid deleting unrelated build outputs.
    project_root = base_dir.parent
    remove_files_by_patterns(project_root, patterns=["temp_*.rs", "libtemp*.rlib"])

if __name__ == "__main__":
    main()
