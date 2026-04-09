import sys
import os
import json
import logging
import argparse
import random
import shutil
import signal
import subprocess
import time
import tempfile
import math
import collections
import hashlib
import re
from pathlib import Path
from typing import Dict, List, Optional
from concurrent.futures import ThreadPoolExecutor
import multiprocessing


from mutation.mutator_pool import MutatorPool
from mutation_crossfeature.main_cross import (
    compile_variants as compile_cross_variants,
    infer_mutation_target_ast_first,
    run_one_seed_one_operator,
    should_skip_by_target_guard,
)
from mutation_crossfeature.mutator_registry import build_mutators, default_operator_keys
from LLM import LLMConnector
from utils.compiler import RustCompiler, CompilationStatus
from utils.ttcg_model import TTCGModel

class SimpleFileLock:
    def __init__(self, lock_file: Path, timeout: float = 300.0):
        self.lock_file = lock_file
        self.timeout = timeout
    
    def __enter__(self):
        start = time.time()
        while True:
            try:
                # Atomic creation on Windows/POSIX
                self.lock_file.mkdir(parents=True, exist_ok=False)
                return self
            except FileExistsError:
                if time.time() - start > self.timeout:
                    raise TimeoutError(f"Could not acquire lock {self.lock_file} in {self.timeout}s")
                time.sleep(0.5)
            except Exception:
                time.sleep(0.5)
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        try:
            self.lock_file.rmdir()
        except Exception:
            pass

# Add project root to path
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

def setup_logging(config, worker_index: int = 0):
    log_dir = Path(config["paths"]["logs"])
    log_dir.mkdir(exist_ok=True)
    
    # Each worker gets its own log file
    log_filename = f"fuzzer_worker_{worker_index}.log"
    
    # Configure logging with a specific format
    logging.basicConfig(
        filename=log_dir / log_filename,
        level=logging.INFO,
        format=f"[Worker-{worker_index}] %(asctime)s - %(levelname)s - %(message)s",
        force=True  # Reset any existing handlers
    )
    
    # Also print to console usage (only if enabled, or ALWAYS for worker 0 as a sample)
    if worker_index == 0 or config.get("fuzzer", {}).get("worker_console_logging", False):
        console = logging.StreamHandler()
        console.setLevel(logging.INFO)
        console.setFormatter(logging.Formatter(f"[Worker-{worker_index}] %(message)s"))
        logging.getLogger('').addHandler(console)


def load_config(config_path):
    with open(config_path, 'r') as f:
        return json.load(f)


def _dir_size_bytes(path: Path) -> int:
    total = 0
    try:
        for p in path.rglob('*'):
            if p.is_file():
                try:
                    total += p.stat().st_size
                except OSError:
                    pass
    except Exception:
        return total
    return total


def _iter_case_dirs_in(status_dir: Path) -> List[Path]:
    if not status_dir.exists() or not status_dir.is_dir():
        return []
    out: List[Path] = []
    for case_dir in status_dir.iterdir():
        if case_dir.is_dir() and case_dir.name.startswith('case_'):
            out.append(case_dir)
    return out


def _case_dirs_by_status(results_dir: Path) -> Dict[str, List[Path]]:
    out: Dict[str, List[Path]] = {}
    if not results_dir.exists():
        return out
    for status_dir in results_dir.iterdir():
        if status_dir.is_dir():
            out[status_dir.name] = _iter_case_dirs_in(status_dir)
    return out


def _sort_oldest_first(paths: List[Path]) -> List[Path]:
    now = time.time()
    return sorted(paths, key=lambda p: p.stat().st_mtime if p.exists() else now)


def _prune_oldest(case_dirs: List[Path], max_keep: int, label: str, log_first_only: bool = False):
    if max_keep < 0:
        return
    case_dirs[:] = _sort_oldest_first(case_dirs)
    logged = False
    while len(case_dirs) > max_keep:
        victim = case_dirs.pop(0)
        try:
            shutil.rmtree(victim)
            if not log_first_only or not logged:
                logging.info("Pruned old %s case: %s", label, victim)
                logged = True
        except FileNotFoundError:
            # Race condition: another worker deleted it first. That's fine.
            pass
        except Exception as e:
            logging.warning("Failed to prune %s: %s", victim, e)
            # Do not break; try to prune others to avoid disk fill-up.
            continue



def _prune_oldest_files(files: List[Path], max_keep: int, label: str, log_first_only: bool = False):
    if max_keep < 0:
        return
    files.sort(key=lambda p: p.stat().st_mtime)
    logged = False
    while len(files) > max_keep:
        victim = files.pop(0)
        try:
            victim.unlink()
            if not log_first_only or not logged:
                logging.info("Pruned old %s file: %s", label, victim)
                logged = True
        except FileNotFoundError:
            # Race condition: another worker deleted it first. That's fine.
            pass
        except Exception as e:
            logging.warning("Failed to prune %s: %s", victim, e)
            continue

# When pruning triggers, prune down to a lower watermark to avoid prune-on-every-case
# churn. This is intentionally not user-configurable.
_PRUNE_WATERMARK = 0.90


_BUG_LOCATION_RE = re.compile(r"([A-Za-z0-9_./\\-]+\.(?:h|hpp|c|cc|cpp|rs)):(\d+)")
_ICE_SIG_RE = re.compile(
    r"internal compiler error:\s*in\s*([^,\n]+),\s*at\s*([^\n]+)",
    re.IGNORECASE,
)
_TMP_TRAIT_FUZZER_DIR_RE = re.compile(r"/tmp/trait_fuzzer_rustc_[^/]+/")
_TEMP_CASE_RS_RE = re.compile(r"(^|/)temp_w\d+_iter_\d+_[^/:]+\.rs$")


def _normalize_bug_path(path: str) -> str:
    p = str(path or "").strip().replace("\\", "/")
    if not p:
        return p

    # Remove per-run random temp directory names.
    p = _TMP_TRAIT_FUZZER_DIR_RE.sub("/tmp/trait_fuzzer_rustc_<tmp>/", p)

    # Collapse per-iteration generated source names to a stable marker.
    if _TEMP_CASE_RS_RE.search(p):
        return "temp_fuzzer_case.rs"

    return p


def _extract_bug_location(stderr: str) -> Optional[str]:
    """Best-effort extraction of a compiler bug location like path/file.h:123."""
    text = str(stderr or "")
    if not text:
        return None

    # Highest-priority key: ICE signature from compiler diagnostic header.
    # Example: "internal compiler error: in operator(), at rust/typecheck/...cc:271"
    ice = _ICE_SIG_RE.search(text)
    if ice:
        fn_name = ice.group(1).strip()
        at_loc = _normalize_bug_path(ice.group(2).strip())
        if at_loc:
            return f"ICE::{fn_name}@{at_loc}"

    matches = _BUG_LOCATION_RE.findall(text)
    if not matches:
        return None

    # Prefer Rust/GCC-internal paths if present.
    for p, line in matches:
        low = p.lower()
        if "rust/" in low or "gcc/" in low or "rustc" in low:
            return f"{_normalize_bug_path(p)}:{line}"

    # Fallback: still normalize temp/generated file paths to reduce dedup noise.
    for p, line in matches:
        norm = _normalize_bug_path(p)
        if norm:
            return f"{norm}:{line}"

    p, line = matches[0]
    return f"{_normalize_bug_path(p)}:{line}"


def _dedup_crash_status_by_location(
    results_root: Path,
    compiler_ns: str,
    status_name: str,
    stderr: str,
) -> tuple:
    """If crash location is seen before, route status from crash -> dup.

    Returns (effective_status, bug_location, duplicated).
    """
    if str(status_name) != "crash":
        return status_name, None, False

    bug_location = _extract_bug_location(stderr)
    if not bug_location:
        return status_name, None, False

    results_root = Path(results_root)
    ns = str(compiler_ns)
    crash_dir = results_root / ns / "crash"
    crash_dir.mkdir(parents=True, exist_ok=True)

    index_file = crash_dir / ".bug_locations.txt"
    lock_dir = results_root / f".{ns}_crash_dedup_lock.dir"

    try:
        with SimpleFileLock(lock_dir, timeout=30):
            seen = set()
            if index_file.exists():
                try:
                    seen = {
                        ln.strip()
                        for ln in index_file.read_text(encoding="utf-8", errors="ignore").splitlines()
                        if ln.strip()
                    }
                except Exception:
                    seen = set()

            if bug_location in seen:
                return "dup", bug_location, True

            try:
                with open(index_file, "a", encoding="utf-8") as f:
                    f.write(bug_location + "\n")
            except Exception:
                pass

            return "crash", bug_location, False
    except Exception:
        return status_name, bug_location, False


def enforce_results_limits(
    results_dir: Path,
    max_cases: Optional[int],
    max_results_gb: Optional[float],
    min_free_gb: Optional[float],
    keep_success_cases: int,
    keep_error_cases: int,
    keep_fate_cases: int,
) -> bool:
    """Prune SUCCESS/ERROR/FATE cases under one result namespace directory.

    Returns True if safe to continue. If disk is still below min_free_gb after
    pruning prunable categories, returns False.
    """
    results_dir = Path(results_dir)
    results_dir.mkdir(parents=True, exist_ok=True)

    prune_watermark = _PRUNE_WATERMARK

    # 1) Per-category pruning policy
    by_status = _case_dirs_by_status(results_dir)
    success_dirs = by_status.get("success", [])
    error_dirs = by_status.get("error", [])

    if keep_success_cases >= 0:
        if len(success_dirs) > keep_success_cases:
            target = int(keep_success_cases * prune_watermark)
            target = min(target, keep_success_cases)
            _prune_oldest(success_dirs, target, label="success", log_first_only=True)
    if keep_error_cases >= 0:
        if len(error_dirs) > keep_error_cases:
            target = int(keep_error_cases * prune_watermark)
            target = min(target, keep_error_cases)
            _prune_oldest(error_dirs, target, label="error", log_first_only=True)

    fate_dirs = by_status.get("fate", [])
    if keep_fate_cases >= 0:
        if len(fate_dirs) > keep_fate_cases:
            target = int(keep_fate_cases * prune_watermark)
            target = min(target, keep_fate_cases)
            _prune_oldest(fate_dirs, target, label="fate", log_first_only=True)

    # 2) Global caps (apply ONLY to prunable categories: success+error)
    prunable_dirs = _sort_oldest_first(success_dirs + error_dirs)
    if max_cases is not None:
        if len(prunable_dirs) > max_cases:
            target = int(max_cases * prune_watermark)
            target = min(target, max_cases)
        else:
            target = max_cases

        logged = False
        while len(prunable_dirs) > target:
            victim = prunable_dirs.pop(0)
            try:
                shutil.rmtree(victim)
                if not logged:
                    logging.info("Pruned old prunable case (max-cases): %s", victim)
                    logged = True
            except Exception as e:
                logging.warning("Failed to prune %s: %s", victim, e)
                break

    if max_results_gb is not None:
        max_results_bytes = int(max_results_gb * 1024**3)
        # Only measure/prune within success+error folders
        prunable_size = _dir_size_bytes(results_dir / "success") + _dir_size_bytes(results_dir / "error")
        prunable_dirs = _sort_oldest_first(_iter_case_dirs_in(results_dir / "success") + _iter_case_dirs_in(results_dir / "error"))

        if prunable_size > max_results_bytes:
            target_bytes = int(max_results_bytes * prune_watermark)
            target_bytes = min(target_bytes, max_results_bytes)
        else:
            target_bytes = max_results_bytes

        logged = False
        while prunable_dirs and prunable_size > target_bytes:
            victim = prunable_dirs.pop(0)
            try:
                victim_size = _dir_size_bytes(victim)
                shutil.rmtree(victim)
                prunable_size = max(0, prunable_size - victim_size)
                if not logged:
                    logging.info("Pruned old prunable case (max-results-gb): %s", victim)
                    logged = True
            except Exception as e:
                logging.warning("Failed to prune %s: %s", victim, e)
                break

    # 3) Disk free guard (only stop if still low after pruning prunable categories)
    if min_free_gb is not None:
        try:
            usage = shutil.disk_usage(str(results_dir))
        except FileNotFoundError:
            usage = shutil.disk_usage(str(results_dir.parent))
        min_free_bytes = int(min_free_gb * 1024**3)
        if usage.free < min_free_bytes:
            logging.warning(
                "Low disk space after pruning prunable results: free=%.2fGB < min_free=%.2fGB. Stopping.",
                usage.free / 1024**3,
                min_free_gb,
            )
            return False

    return True


def pick_next_new_seed_dir(seeds_dir: Path, prefix: str = "new") -> Path:
    seeds_dir = Path(seeds_dir)
    seeds_dir.mkdir(parents=True, exist_ok=True)
    max_n = 0
    for p in seeds_dir.iterdir():
        if not p.is_dir():
            continue
        name = p.name
        if not name.startswith(prefix):
            continue
        suffix = name[len(prefix):]
        if suffix.isdigit():
            max_n = max(max_n, int(suffix))
    out = seeds_dir / f"{prefix}{max_n + 1}"
    out.mkdir(parents=True, exist_ok=True)
    return out


def enforce_seed_file_cap(seed_dir: Path, max_files: int):
    seed_dir = Path(seed_dir)
    if max_files <= 0 or not seed_dir.exists():
        return
    files = [p for p in seed_dir.glob('*.rs') if p.is_file()]
    files = _sort_oldest_first(files)
    while len(files) > max_files:
        victim = files.pop(0)
        try:
            victim.unlink()
            logging.info("Pruned old promoted seed: %s", victim)
        except Exception as e:
            logging.warning("Failed to prune promoted seed %s: %s", victim, e)
            break


def maybe_roll_promoted_dir(current_dir: Path, seeds_dir: Path, prefix: str, max_files: int) -> Path:
    """If current promoted dir already contains >= max_files seeds, roll to the next newN dir.

    This matches the expected behavior: new1 fills up, then new2 is created, etc.
    """
    current_dir = Path(current_dir)
    if max_files <= 0:
        return current_dir
    try:
        files = [p for p in current_dir.glob('*.rs') if p.is_file()]
        if len(files) >= int(max_files):
            nxt = pick_next_new_seed_dir(Path(seeds_dir), prefix=str(prefix or "new"))
            logging.info(
                "Promoted dir full (%d/%d): rolling to %s",
                len(files),
                int(max_files),
                nxt,
            )
            return nxt
    except Exception:
        # Be conservative: if we can't inspect, keep writing to the current dir.
        return current_dir
    return current_dir


def parse_args_and_config(argv: Optional[List[str]] = None):
    # Stage 1: only parse --config so we can load defaults from the config file.
    pre = argparse.ArgumentParser(add_help=False)
    pre.add_argument("--config", default="config.json", help="Path to configuration file")
    pre_args, remaining = pre.parse_known_args(argv)

    config = load_config(pre_args.config)
    run_cfg = config.get("run", {})

    # Stage 2: full parser with config-backed defaults. CLI overrides config.
    parser = argparse.ArgumentParser(
        description="Trait-Fuzzer V1.0",
        parents=[pre],
    )

    # Strategy selection
    parser.set_defaults(structural_only=run_cfg.get("structural_only", True))
    parser.add_argument(
        "--structural-only",
        dest="structural_only",
        action="store_true",
        help="Only use AST structural mutators (default from config.run.structural_only)",
    )
    parser.add_argument(
        "--all-strategies",
        dest="structural_only",
        action="store_false",
        help="Enable all non-structural mutation strategies",
    )

    # Safety controls for long runs
    parser.add_argument(
        "--max-cases",
        type=int,
        default=run_cfg.get("max_cases", None),
        help="Max number of saved cases under results/ (oldest pruned).",
    )
    parser.add_argument(
        "--max-results-gb",
        type=float,
        default=run_cfg.get("max_results_gb", None),
        help="Max size of results/ in GB (oldest pruned).",
    )
    parser.add_argument(
        "--min-free-gb",
        type=float,
        default=run_cfg.get("min_free_gb", None),
        help="Stop fuzzing if free disk space drops below this GB threshold.",
    )

    # Retention & promotion policy
    parser.add_argument(
        "--keep-success-cases",
        type=int,
        default=int(run_cfg.get("keep_success_cases", 2000)),
        help="How many SUCCESS cases to keep under results/rustc/success/ (oldest pruned)",
    )
    parser.add_argument(
        "--keep-error-cases",
        type=int,
        default=int(run_cfg.get("keep_error_cases", 2000)),
        help="How many ERROR cases to keep under results/rustc/error/ (oldest pruned)",
    )
    parser.add_argument(
        "--keep-fate-cases",
        type=int,
        default=int(run_cfg.get("keep_fate_cases", 100)),
        help="How many FATE cases to keep under results/<compiler>/fate/ (oldest pruned)",
    )

    parser.set_defaults(detect_miscompilation=bool(run_cfg.get("detect_miscompilation", True)))
    parser.add_argument(
        "--detect-miscompilation",
        dest="detect_miscompilation",
        action="store_true",
        help="Enable Miscompilation detection (nightly vs next-solver)",
    )
    parser.add_argument(
        "--no-detect-miscompilation",
        dest="detect_miscompilation",
        action="store_false",
        help="Disable Miscompilation detection (faster)",
    )

    parser.set_defaults(promote_success=bool(run_cfg.get("promote_success", True)))
    parser.add_argument(
        "--promote-success",
        dest="promote_success",
        action="store_true",
        help="Promote SUCCESS mutants into seeds/newN",
    )
    parser.add_argument(
        "--no-promote-success",
        dest="promote_success",
        action="store_false",
        help="Do not promote SUCCESS mutants into seeds/newN",
    )
    parser.add_argument(
        "--new-seeds-max",
        type=int,
        default=int(run_cfg.get("new_seeds_max", 2000)),
        help="Max number of promoted seeds per seeds/<newN>/; when full, roll to <new(N+1)>",
    )
    parser.add_argument(
        "--new-seeds-prefix",
        type=str,
        default=str(run_cfg.get("new_seeds_prefix", "new")),
        help="Prefix for promoted seed folders under seeds/ (e.g. new1, new2, ...)",
    )

    args = parser.parse_args(remaining, namespace=pre_args)
    return args, config

class SeedSelector:
    def __init__(self, seeds_dir: Path, fuzzer_cfg: Optional[dict] = None, promoted_prefix: str = "new", shard_index: int = 0, num_shards: int = 1):
        # Allow organizing seeds in subdirectories (e.g. imported official suites).
        self._seeds_dir = Path(seeds_dir)
        self._promoted_prefix = str(promoted_prefix or "new")
        self._shard_index = shard_index
        self._num_shards = max(1, num_shards)


        # Probability of sampling from the promoted pool first.
        # Default is aggressive base:promoted = 9:1.
        fuzzer_cfg = dict(fuzzer_cfg or {})
        self._promoted_pool_prob = float(fuzzer_cfg.get("seed_promoted_pool_prob", 0.10))
        if self._promoted_pool_prob < 0.0:
            self._promoted_pool_prob = 0.0
        if self._promoted_pool_prob > 1.0:
            self._promoted_pool_prob = 1.0

        self._promoted_seeds: set = set()

        all_seeds = list(seeds_dir.rglob("*.rs"))
        # Sort for deterministic sharding
        all_seeds.sort()
        
        # Filter internal-only seeds first, and also ignore empty files
        valid_seeds = []
        empty_count = 0
        for p in all_seeds:
            if not p.is_file():
                continue
            if p.stat().st_size == 0:
                empty_count += 1
                continue
            if not self._is_internal_only_seed(p):
                valid_seeds.append(p)

        if empty_count > 0:
            logging.warning("Ignored %d empty seed files (0 bytes)", empty_count)
        
        # Apply sharding
        self.seeds = []
        for i, seed in enumerate(valid_seeds):
            if i % self._num_shards == self._shard_index:
                self.seeds.append(seed)

        for p in self.seeds:
            if self._is_promoted_seed(p):
                self._promoted_seeds.add(p)
        
        filtered = len(all_seeds) - len(valid_seeds)
        if filtered > 0:
            logging.info(
                "Filtered %d seeds due to internal-only features (rustc_attrs/lang_items/intrinsics/etc.)",
                filtered,
            )
        logging.info(f"Worker {self._shard_index}/{self._num_shards}: Assigned {len(self.seeds)} seeds (total pool: {len(valid_seeds)})")
        self.scores: Dict[Path, int] = {}
        self._ttcg = TTCGModel()
        self._ttcg_cache = {}  # (path, mtime_ns) -> score(int)
        self._scores_initialized = False

        # Seed selection smoothing to avoid getting stuck on one high-score seed.
        # These are optional knobs (read from config.fuzzer.*) with safe defaults.
        self._weight_temperature = float(fuzzer_cfg.get("seed_weight_temperature", 2.0))
        self._pick_decay_beta = float(fuzzer_cfg.get("seed_pick_decay_beta", 1.0))
        self._min_weight = float(fuzzer_cfg.get("seed_min_weight", 1.0))
        self._repeat_window = int(fuzzer_cfg.get("seed_repeat_window", 32))
        self._max_picks_per_seed = fuzzer_cfg.get("seed_max_picks_per_seed", 1)
        self._family_repeat_window = int(fuzzer_cfg.get("seed_family_repeat_window", 8))
        self._family_pick_decay_beta = float(fuzzer_cfg.get("seed_family_pick_decay_beta", 0.75))
        self._family_min_weight = float(fuzzer_cfg.get("seed_family_min_weight", 0.3))
        self._pick_retry_limit = int(fuzzer_cfg.get("seed_pick_retry_limit", 16))

        if self._repeat_window < 0:
            self._repeat_window = 0
        if self._weight_temperature <= 0:
            self._weight_temperature = 1.0
        if self._pick_decay_beta < 0:
            self._pick_decay_beta = 0.0
        if self._min_weight <= 0:
            self._min_weight = 1.0
        if self._family_repeat_window < 0:
            self._family_repeat_window = 0
        if self._family_pick_decay_beta < 0:
            self._family_pick_decay_beta = 0.0
        if self._family_min_weight <= 0:
            self._family_min_weight = 0.1
        if self._pick_retry_limit <= 0:
            self._pick_retry_limit = 1

        # Seed pick tracking
        self._pick_counts: Dict[Path, int] = {}
        self._recent = collections.deque(maxlen=self._repeat_window)
        self._family_pick_counts: Dict[str, int] = {}
        self._family_recent = collections.deque(maxlen=self._family_repeat_window)
        self._seed_family: Dict[Path, str] = {}
        self._banned_families: set = set()

    @staticmethod
    def _is_internal_only_seed(seed_path: Path) -> bool:
        """Heuristically filter seeds that rely on internal-only rustc features.

        Motivation: ICEs triggered by incorrect usage of internal-only features (e.g.
        rustc_attrs/lang_items/intrinsics) are typically not accepted upstream.

        We only scan a small prefix because these attributes are almost always at the top.
        """
        try:
            text = seed_path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            return False

        head = text[:8000]

        # Feature gates that strongly suggest "internal-only" testing seeds.
        internal_features = (
            "rustc_attrs",
            "lang_items",
            "intrinsics",
            "core_intrinsics",
            "rustc_private",
        )

        # Quick substring checks (fast path).
        if "#![feature(" in head and any(f in head for f in internal_features):
            return True

        # Attributes often used with rustc-internal plumbing.
        internal_attr_markers = (
            "#[rustc_",
            "#![rustc_",
            "#[lang =",
            "#![no_core]",
            "#![rustc_attrs]",
        )
        if any(m in head for m in internal_attr_markers):
            return True

        # Intrinsics linkage style.
        if "extern \"rust-intrinsic\"" in head:
            return True

        return False

    def _is_promoted_seed(self, seed_path: Path) -> bool:
        """Return True if the seed lives under seeds/<prefix><digits>/..."""
        try:
            rel = Path(seed_path).resolve().relative_to(self._seeds_dir.resolve())
        except Exception:
            return False
        if not rel.parts:
            return False
        top = rel.parts[0]
        if not top.startswith(self._promoted_prefix):
            return False
        suffix = top[len(self._promoted_prefix):]
        return bool(suffix) and suffix.isdigit()
        
    def _calculate_scores(self):
        logging.info("Calculating seed scores (constraint-choice metric) for all seeds...")
        for seed in self.seeds:
            try:
                self.scores[seed] = self._score_one(seed)
            except Exception as e:
                logging.warning(f"Failed to score {seed.name}: {e}")
                self.scores[seed] = 1

        self._scores_initialized = True

    def _score_one(self, seed: Path) -> int:
        mtime_ns = seed.stat().st_mtime_ns
        cache_key = (str(seed), int(mtime_ns))
        cached = self._ttcg_cache.get(cache_key)
        if cached is None:
            complexity = self._ttcg.calculate_complexity_for_file(seed)

            # New definition (source of truth):
            # score = sum over all constraint injection sites of (# selectable constraints).
            # Exposed by mutation-ast --mode ttcg_metrics as `constraint_choice_sum`.
            score = int(complexity.extra.get("constraint_choice_sum", 0))

            # Backwards-compatible fallback if the Rust tool is older.
            if score <= 0:
                depth = int(complexity.extra.get("depth", 0))
                cycles = int(complexity.extra.get("cycles", 0))
                score = max(1, depth * 20 + cycles)

            cached = int(score)
            self._ttcg_cache[cache_key] = cached

        return max(1, int(cached))

    def add_seed(self, seed: Path, family_id: Optional[str] = None):
        seed = Path(seed)
        if not seed.exists() or not seed.is_file() or seed.suffix != ".rs":
            return
        if seed in self.seeds:
            return
        if self._is_internal_only_seed(seed):
            return
        self.seeds.append(seed)
        if self._is_promoted_seed(seed):
            self._promoted_seeds.add(seed)
        if family_id:
            self._seed_family[seed] = str(family_id)
        else:
            self._seed_family[seed] = self._infer_family_from_seed(seed)
        # If we've already initialized scoring, score this seed now (incremental, cheap).
        if self._scores_initialized or self.scores:
            try:
                self.scores[seed] = self._score_one(seed)
            except Exception:
                self.scores[seed] = 1

    def _choose_pool_candidates(self, eligible: List[Path]) -> List[Path]:
        if not eligible:
            return eligible

        promoted = [s for s in eligible if s in self._promoted_seeds]
        base = [s for s in eligible if s not in self._promoted_seeds]

        # Prefer base pool by default (aggressive 9:1), but fall back if empty.
        want_promoted = random.random() < self._promoted_pool_prob
        if want_promoted and promoted:
            return promoted
        if base:
            return base
        return promoted

    def _eligible_seeds(self) -> List[Path]:
        if not self.seeds:
            return []

        # Drop seeds whose families are banned (fate-excluded).
        eligible = [s for s in self.seeds if self._family_key(s) not in self._banned_families]
        if not eligible:
            return []

        # Enforce a simple budget per seed if configured.
        if self._max_picks_per_seed is not None:
            try:
                max_picks = int(self._max_picks_per_seed)
            except Exception:
                max_picks = -1
            if max_picks >= 0:
                return [s for s in eligible if self._pick_counts.get(s, 0) < max_picks]

        return list(eligible)

    def _weight_for_seed(self, seed: Path) -> float:
        base = float(self.scores.get(seed, 1))

        # Temperature > 1 flattens distribution; < 1 makes it greedier.
        shaped = math.pow(max(1.0, base), 1.0 / self._weight_temperature)

        # Penalize repeatedly selected seeds to avoid being stuck.
        picked = float(self._pick_counts.get(seed, 0))
        decayed = shaped / math.pow(1.0 + picked, self._pick_decay_beta)
        # Family-level suppression to avoid one lineage dominating.
        family = self._family_key(seed)
        fam_picks = float(self._family_pick_counts.get(family, 0))
        fam_factor = 1.0 / math.pow(1.0 + fam_picks, self._family_pick_decay_beta)
        fam_weight = max(self._family_min_weight, fam_factor)

        return max(self._min_weight, decayed) * fam_weight

    def _weighted_choice(self, candidates: List[Path]) -> Optional[Path]:
        if not candidates:
            return None
        weights = [self._weight_for_seed(s) for s in candidates]
        return random.choices(candidates, weights=weights, k=1)[0]

    def _record_pick(self, seed: Path):
        self._pick_counts[seed] = self._pick_counts.get(seed, 0) + 1
        if self._repeat_window > 0:
            self._recent.append(seed)
        family = self._family_key(seed)
        self._family_pick_counts[family] = self._family_pick_counts.get(family, 0) + 1
        if self._family_repeat_window > 0:
            self._family_recent.append(family)

    def _family_key(self, seed: Path) -> str:
        if seed in self._seed_family:
            return self._seed_family[seed]
        fam = self._infer_family_from_seed(seed)
        self._seed_family[seed] = fam
        return fam

    def _infer_family_from_seed(self, seed: Path) -> str:
        # If filename encodes family: <stem>__fam__<family>
        try:
            stem = seed.stem
            if "__fam__" in stem:
                return stem.split("__fam__", 1)[1]
        except Exception:
            pass
        # Fallback: treat each original seed as its own family
        return str(seed.resolve())

    def get_family(self, seed: Path) -> str:
        return self._family_key(seed)

    def ban_family(self, family: str):
        if family is None:
            return
        self._banned_families.add(str(family))

    def _candidates_within_window(self, eligible: List[Path]) -> List[Path]:
        if self._repeat_window <= 0 or len(eligible) <= 1:
            seed_candidates = eligible
        else:
            recent_set = set(self._recent)
            seed_candidates = [s for s in eligible if s not in recent_set]
            # If all seeds are "recent", relax the constraint.
            if not seed_candidates:
                seed_candidates = eligible

        if self._family_repeat_window <= 0 or len(seed_candidates) <= 1:
            return seed_candidates
        recent_families = set(self._family_recent)
        fam_candidates = [s for s in seed_candidates if self._family_key(s) not in recent_families]
        return fam_candidates if fam_candidates else seed_candidates

    def select(self, strategy="random"):
        if not self.seeds:
            return None

        def _pick_with_retry(pick_fn):
            # Seeds under promoted pool can be pruned on disk (new_seeds_max). If we
            # still have stale paths in memory, drop them lazily and retry.
            for _ in range(self._pick_retry_limit):
                picked = pick_fn()
                if picked is None:
                    return None
                if Path(picked).exists():
                    return picked
                self.remove_seed(Path(picked))
            return None
            
        if strategy == "ttcg_metric":
            if not self.scores:
                self._calculate_scores()

            # Prefer not repeating seeds within a short window.
            eligible = self._eligible_seeds()
            if not eligible:
                return None

            pool = self._choose_pool_candidates(eligible)
            candidates = self._candidates_within_window(pool)

            def _do_pick():
                return self._weighted_choice(candidates)

            picked = _pick_with_retry(_do_pick)
            if picked is not None:
                self._record_pick(picked)
            return picked
            
        else:
            eligible = self._eligible_seeds()
            if not eligible:
                return None
            pool = self._choose_pool_candidates(eligible)
            candidates = self._candidates_within_window(pool)
            def _do_pick():
                if not candidates:
                    return None
                return random.choice(candidates)

            picked = _pick_with_retry(_do_pick)
            if picked is not None:
                self._record_pick(picked)
            return picked

    def remove_seed(self, seed: Path):
        try:
            self.seeds.remove(seed)
        except ValueError:
            return
        self.scores.pop(seed, None)
        self._promoted_seeds.discard(seed)
        self._pick_counts.pop(seed, None)
        family = self._seed_family.get(seed)
        self._seed_family.pop(seed, None)
        if self._repeat_window > 0:
            try:
                self._recent = collections.deque([s for s in self._recent if s != seed], maxlen=self._repeat_window)
            except Exception:
                pass
        # Best-effort cleanup for family tracking
        if family is None:
            try:
                family = self._infer_family_from_seed(seed)
            except Exception:
                family = None
        if family is not None:
            self._family_pick_counts.pop(family, None)
            if self._family_repeat_window > 0:
                try:
                    self._family_recent = collections.deque(
                        [f for f in self._family_recent if f != family],
                        maxlen=self._family_repeat_window,
                    )
                except Exception:
                    pass
def ensure_mutation_tool_built() -> Path:
    """Builds the mutation-AST tool once and returns the path to the binary."""
    logging.info("Building mutation tool (mutation-AST)...")
    manifest_dir = Path("mutation/mutation-AST")
    try:
        # Build in release mode for performance.
        # Use +nightly to handle unstable features and lockfile v4.
        # Use RUSTFLAGS="-A warnings" to silence dead code/style warnings that annoy the user.
        build_env = os.environ.copy()
        build_env["RUSTFLAGS"] = "-A warnings"
        
        subprocess.run(
            ["cargo", "+nightly", "build", "--release"], 
            cwd=str(manifest_dir.resolve()),
            check=True,
            env=build_env
        )
    except subprocess.CalledProcessError as e:
        logging.error(f"Failed to build mutation tool: {e.stderr}")
        raise e

    # Windows adds .exe
    bin_name = "mutation-ast"
    if os.name == 'nt':
        bin_name += ".exe"
    
    bin_path = manifest_dir / "target" / "release" / bin_name
    if not bin_path.exists():
        # Fallback to debug if release not found? Or maybe user built debug.
        # But we explicitly asked for release above.
        raise FileNotFoundError(f"Mutation tool binary not found at {bin_path}")
        
    logging.info(f"Mutation tool built successfully at {bin_path}")
    return bin_path.resolve()

def worker_main(worker_index: int, total_workers: int, mutation_bin_path: Path):

    try:
        args, config = parse_args_and_config()
        setup_logging(config, worker_index)

        logging.info("Trait-Fuzzer started with config: %s", args.config)
        
        # Initialize components
        compiler_cfg = config.get("compiler", {})
        compiler = RustCompiler(
            timeout=config["fuzzer"]["max_time_per_case_sec"],
            rustc_cmd=compiler_cfg.get("rustc_cmd"),
        )
        mutator_pool = MutatorPool(config)

        # Unified TTCG model (Rust syn-based extractor via mutation-ast --mode ttcg_metrics)
        ttcg_model = TTCGModel()

        mutation_mode_raw = str(config.get("fuzzer", {}).get("mutation_mode", "trait-constraint"))
        mutation_modes = {m.strip().lower() for m in mutation_mode_raw.split(",") if m.strip()}
        enable_cross_feature = "cross-feature" in mutation_modes
        cross_feature_seed_fallback = bool(config.get("fuzzer", {}).get("cross_feature_seed_fallback", True))
        cross_feature_ctx = None
        repo_root = Path(__file__).resolve().parent

        obligation_pattern = re.compile(r"\bobligation(s)?\b", re.IGNORECASE)

        def _count_obligations(text: str) -> int:
            return len(obligation_pattern.findall(text or ""))

        def _normalize_rustc_cmd(cmd_cfg) -> List[str]:
            if isinstance(cmd_cfg, list) and len(cmd_cfg) > 0:
                return [str(x) for x in cmd_cfg]
            if isinstance(cmd_cfg, str) and cmd_cfg.strip():
                return [cmd_cfg.strip()]
            return ["rustc"]

        rustc_obligation_cmd = _normalize_rustc_cmd(compiler_cfg.get("rustc_cmd"))
        obligation_timeout = int(config["fuzzer"].get("max_time_per_case_sec", 20))

        obligation_script_mode = int(config["fuzzer"].get("obligation_compiler_mode", 1))
        obligation_cache: Dict[str, Dict[str, object]] = {}

        def _resolve_obligation_script_path() -> Optional[Path]:
            configured = config.get("fuzzer", {}).get("obligation_script_path")
            if configured:
                p = Path(str(configured))
                if not p.is_absolute():
                    p = repo_root / p
                if p.exists():
                    return p

            candidates = [
                repo_root / "utils" / "structure_complexity" / "trait_query_stats.sh",
                repo_root.parent.parent / "Traitor" / "trait-fuzzer" / "utils" / "TSA" / "trait_query_stats.sh",
            ]
            for p in candidates:
                if p.exists():
                    return p
            return None

        obligation_script_path = _resolve_obligation_script_path()
        if obligation_script_path is None:
            logging.info("Obligation script not found; using rustc-log fallback for cross-feature ranking")
        else:
            logging.info("Obligation script detected: %s", obligation_script_path)

        def _measure_obligation_score_with_details(code: str) -> Dict[str, object]:
            code_hash = hashlib.sha256(code.encode("utf-8", errors="ignore")).hexdigest()
            cached = obligation_cache.get(code_hash)
            if cached is not None:
                return dict(cached)

            # Preferred path: use stats script when available.
            if obligation_script_path is not None:
                try:
                    with tempfile.TemporaryDirectory(prefix="main_obligation_script_") as td:
                        td_path = Path(td)
                        src = td_path / "input.rs"
                        out_dir = td_path / "out"
                        src.write_text(code, encoding="utf-8", errors="ignore")
                        proc = subprocess.run(
                            [
                                "bash",
                                str(obligation_script_path),
                                "--file",
                                str(src),
                                "--compiler",
                                str(obligation_script_mode),
                                "--out-dir",
                                str(out_dir),
                                "--top",
                                "15",
                            ],
                            capture_output=True,
                            text=True,
                            timeout=obligation_timeout,
                        )

                        summary_path = out_dir / "summary.txt"
                        score = None
                        if summary_path.exists():
                            text = summary_path.read_text(encoding="utf-8", errors="ignore")
                            m = re.search(r"obligation_lines:\s*(\d+)", text)
                            if m is not None:
                                score = int(m.group(1))
                        if score is None:
                            trait_log_path = out_dir / "trait.log"
                            if trait_log_path.exists():
                                trait_text = trait_log_path.read_text(encoding="utf-8", errors="ignore")
                                score = _count_obligations(trait_text)
                        if score is None:
                            merged = f"{proc.stdout}\n{proc.stderr}"
                            score = _count_obligations(merged)

                        out = {
                            "score": int(score),
                            "method": "obligation_script",
                            "script_exit_code": int(proc.returncode),
                        }
                        obligation_cache[code_hash] = dict(out)
                        return out
                except Exception:
                    pass

            # Fallback 1: compile with explicit rustc trait-selection log.
            try:
                with tempfile.TemporaryDirectory(prefix="main_obligation_log_") as td:
                    td_path = Path(td)
                    src = td_path / "input.rs"
                    out_bin = td_path / "out_bin"
                    src.write_text(code, encoding="utf-8", errors="ignore")
                    env = os.environ.copy()
                    env.setdefault("RUSTC_LOG", "rustc_trait_selection=info")
                    proc = subprocess.run(
                        [*rustc_obligation_cmd, str(src), "-o", str(out_bin), "--out-dir", str(td_path)],
                        capture_output=True,
                        text=True,
                        timeout=obligation_timeout,
                        env=env,
                    )
                    out = {
                        "score": _count_obligations(f"{proc.stdout}\n{proc.stderr}"),
                        "method": "rustc_log_obligation_fallback",
                        "compile_exit_code": int(proc.returncode),
                    }
                    obligation_cache[code_hash] = dict(out)
                    return out
            except Exception:
                pass

            # Fallback 2: plain compile text counting.
            try:
                with tempfile.TemporaryDirectory(prefix="main_obligation_plain_") as td:
                    td_path = Path(td)
                    src = td_path / "input.rs"
                    src.write_text(code, encoding="utf-8", errors="ignore")
                    outcome = compiler.compile(src)
                out = {
                    "score": _count_obligations(f"{outcome.stdout}\n{outcome.stderr}"),
                    "method": "obligation_regex_fallback",
                }
                obligation_cache[code_hash] = dict(out)
                return out
            except Exception:
                out = {"score": 0, "method": "obligation_failed"}
                obligation_cache[code_hash] = dict(out)
                return out

        if enable_cross_feature:
            try:
                cross_cfg_rel = config.get("fuzzer", {}).get(
                    "cross_feature_config",
                    "mutation_crossfeature/config_cross.json",
                )
                cross_cfg_path = Path(cross_cfg_rel)
                if not cross_cfg_path.is_absolute():
                    cross_cfg_path = repo_root / cross_cfg_path
                cross_cfg = load_config(cross_cfg_path)

                cross_results_root = Path(config["paths"]["results"]) / "cross_feature"
                cross_results_root.mkdir(parents=True, exist_ok=True)

                connector = LLMConnector(config)
                mutator_map = build_mutators(connector)
                enabled_ops = list(cross_cfg.get("run", {}).get("enabled_operators", default_operator_keys()))
                valid_ops = [k for k in enabled_ops if k in mutator_map]

                if not valid_ops:
                    logging.warning("cross-feature enabled but no valid operators configured; disabling cross-feature")
                    enable_cross_feature = False
                else:
                    llm_lock = config.get("llm", {}).get("lock_path", "llm_global_lock_smoke.dir")
                    lock_path = Path(llm_lock)
                    if not lock_path.is_absolute():
                        lock_path = repo_root / lock_path

                    cross_feature_ctx = {
                        "repo_root": repo_root,
                        "cross_cfg": cross_cfg,
                        "results_root": cross_results_root,
                        "mutator_map": mutator_map,
                        "valid_ops": valid_ops,
                        "require_trait_ops": set(cross_cfg.get("run", {}).get("require_trait_ops", [])),
                        "lock_path": lock_path,
                        "lock_timeout": int(cross_cfg.get("run", {}).get("llm_lock_timeout_sec", 180)),
                        "lock_stale_after": float(cross_cfg.get("run", {}).get("llm_lock_stale_after_sec", 900)),
                        "save_prompt": bool(cross_cfg.get("run", {}).get("save_prompt", False)),
                        "notip": bool(cross_cfg.get("run", {}).get("notip", False)),
                    }
                    logging.info(
                        "cross-feature mode enabled: operators=%s, results=%s",
                        ",".join(valid_ops),
                        cross_results_root,
                    )
            except Exception as e:
                logging.warning("Failed to initialize cross-feature mode: %s", e)
                enable_cross_feature = False
                cross_feature_ctx = None

        def _run_cross_feature_for_candidate(iter_idx: int, candidate: Dict[str, object]) -> None:
            if cross_feature_ctx is None:
                return
            candidate_code = str(candidate.get("content", ""))
            if not candidate_code.strip():
                return

            repo_root = cross_feature_ctx["repo_root"]
            target, target_source = infer_mutation_target_ast_first(candidate_code, repo_root)

            op_pool = list(cross_feature_ctx["valid_ops"])
            random.shuffle(op_pool)
            applicable_ops: List[str] = []
            for op_key in op_pool:
                if should_skip_by_target_guard(op_key, target, cross_feature_ctx["require_trait_ops"]):
                    continue
                applicable_ops.append(op_key)

            if not applicable_ops:
                logging.info(
                    "[iter %d] cross-feature skipped: no applicable operator for target(type=%s, trait=%s)",
                    iter_idx + 1,
                    target.type_name,
                    target.trait_name or "None",
                )
                return

            results_root = cross_feature_ctx["results_root"]
            results_root.mkdir(parents=True, exist_ok=True)

            candidate_variant_id = str(candidate.get("variant_id", f"iter_{iter_idx+1}"))
            baseline_temp = results_root / f"temp_cross_base_w{worker_index}_iter_{iter_idx+1}.rs"
            base_variants = compile_cross_variants(
                candidate_code,
                baseline_temp,
                config,
                int(config["fuzzer"].get("max_time_per_case_sec", 20)),
            )

            attempted_ops: List[str] = []
            for op_order, selected_op in enumerate(applicable_ops):
                mutator = cross_feature_ctx["mutator_map"].get(selected_op)
                if mutator is None:
                    continue
                attempted_ops.append(selected_op)

                with tempfile.TemporaryDirectory(prefix=f"cross_seed_w{worker_index}_iter_{iter_idx+1}_") as td:
                    seed_path = Path(td) / f"selected_{candidate_variant_id}.rs"
                    seed_path.write_text(candidate_code, encoding="utf-8", errors="ignore")
                    cross_result = run_one_seed_one_operator(
                        index=iter_idx,
                        seed_path=seed_path,
                        seed_code=candidate_code,
                        target=target,
                        base_variants=base_variants,
                        mutator=mutator,
                        config=config,
                        timeout_sec=int(config["fuzzer"].get("max_time_per_case_sec", 20)),
                        lock_path=cross_feature_ctx["lock_path"],
                        lock_timeout=int(cross_feature_ctx["lock_timeout"]),
                        lock_stale_after=float(cross_feature_ctx["lock_stale_after"]),
                        results_root=results_root,
                        save_prompt=bool(cross_feature_ctx["save_prompt"]),
                        notip=bool(cross_feature_ctx["notip"]),
                        op_order=op_order,
                        pass_name=f"selected_{candidate_variant_id}",
                    )

                if cross_result is not None:
                    logging.info(
                        "[iter %d] cross-feature generated case=%s from variant=%s (obligation=%s method=%s, operator=%s)",
                        iter_idx + 1,
                        cross_result.case_id,
                        candidate_variant_id,
                        candidate.get("obligation_score", 0),
                        candidate.get("obligation_method", "unknown"),
                        selected_op,
                    )
                    return

            logging.info(
                "[iter %d] cross-feature attempted (target_source=%s, operators=%s) but produced no case",
                iter_idx + 1,
                target_source,
                ",".join(attempted_ops) if attempted_ops else "none",
            )

        # Option: run only structural AST mutators.
        # This is useful for stabilizing runs and focusing on trait/topology evolution.
        if args.structural_only:
            mutator_pool.strategies = ["ast_structural"]
            mutator_pool.probs = [1.0]

        # Seeds
        seeds_dir = Path(config["paths"]["seeds"])
        results_dir = Path(config["paths"]["results"])
        rustc_results_dir = results_dir / "rustc"
        gccrs_results_dir = results_dir / "gccrs"
        enable_gccrs = bool(compiler_cfg.get("enable_gccrs", False))

        def _enforce_all_results_limits() -> bool:
            if not enforce_results_limits(
                rustc_results_dir,
                max_cases=args.max_cases,
                max_results_gb=args.max_results_gb,
                min_free_gb=args.min_free_gb,
                keep_success_cases=args.keep_success_cases,
                keep_error_cases=args.keep_error_cases,
                keep_fate_cases=args.keep_fate_cases,
            ):
                return False
            if enable_gccrs:
                if not enforce_results_limits(
                    gccrs_results_dir,
                    max_cases=args.max_cases,
                    max_results_gb=args.max_results_gb,
                    min_free_gb=args.min_free_gb,
                    keep_success_cases=-1,
                    keep_error_cases=-1,
                    keep_fate_cases=args.keep_fate_cases,
                ):
                    return False
            return True
        
        selector = SeedSelector(seeds_dir, fuzzer_cfg=config.get("fuzzer", {}), promoted_prefix=args.new_seeds_prefix, shard_index=worker_index, num_shards=total_workers)

        if not selector.seeds:
            logging.warning("No seeds found in %s", seeds_dir)
            return

        # Seeds that `syn` cannot parse (common in rustc UI tests with newer syntax).
        bad_seeds = set()

        logging.info("Found %d seeds", len(selector.seeds))

        promoted_dir: Optional[Path] = None
        if args.promote_success:
            promoted_dir = pick_next_new_seed_dir(seeds_dir, prefix=args.new_seeds_prefix)
            logging.info("Promoting SUCCESS mutants into %s", promoted_dir)

        # If configured, proactively enforce limits before starting.
        if not _enforce_all_results_limits():
            return
        
        # State tracking
        max_choice = {"constraint_choice_sum": 0}

        # Promotion rate limit: each parent seed can contribute at most N SUCCESS mutants
        # into seeds/<newN>/ across the whole run.
        max_promotions_per_seed = int(config.get("fuzzer", {}).get("max_promotions_per_seed", 2))
        promotions_by_seed: Dict[Path, int] = {}
        logged_promotion_cap: set = set()

        enable_next_solver = bool(compiler_cfg.get("enable_next_trait_solver", False))
        # If we enable next-solver, we almost always also want a plain nightly compile for comparison.
        enable_nightly_compile = bool(compiler_cfg.get("enable_nightly_compile", enable_next_solver))
        nightly_rustc_cmd = compiler_cfg.get("rustc_z_cmd", ["rustc", "+nightly"])
        gccrs_cmd_cfg = compiler_cfg.get("gccrs_cmd", ["gccrs"])
        gccrs_extra_args_cfg = compiler_cfg.get(
            "gccrs_extra_args",
            ["-frust-incomplete-and-experimental-compiler-do-not-use"],
        )
        if isinstance(gccrs_cmd_cfg, str):
            gccrs_cmd = [gccrs_cmd_cfg]
        elif isinstance(gccrs_cmd_cfg, list) and len(gccrs_cmd_cfg) > 0:
            gccrs_cmd = list(gccrs_cmd_cfg)
        else:
            gccrs_cmd = ["gccrs"]

        if isinstance(gccrs_extra_args_cfg, str):
            gccrs_extra_args = [gccrs_extra_args_cfg]
        elif isinstance(gccrs_extra_args_cfg, list):
            gccrs_extra_args = list(gccrs_extra_args_cfg)
        else:
            gccrs_extra_args = []

        gccrs_work_dir_cfg = compiler_cfg.get("gccrs_work_dir")
        gccrs_work_dir = str(gccrs_work_dir_cfg) if gccrs_work_dir_cfg else None

        gccrs_env_cfg = compiler_cfg.get("gccrs_env", {})
        gccrs_env = {}
        if isinstance(gccrs_env_cfg, dict):
            gccrs_env = {str(k): str(v) for k, v in gccrs_env_cfg.items()}
        gccrs_auto_no_core = bool(compiler_cfg.get("gccrs_auto_no_core", True))

        def _is_gccrs_cmd(cmd_list) -> bool:
            if not cmd_list:
                return False
            name = Path(str(cmd_list[0])).name.lower()
            return name in {"gccrs", "crab1", "rust1"}

        def _resolve_compiler_bin(cmd_list, work_dir=None):
            if not cmd_list:
                return None
            raw = str(cmd_list[0])
            path_obj = Path(raw)
            if path_obj.is_absolute() and path_obj.exists():
                return str(path_obj)
            if work_dir:
                candidate = Path(work_dir) / raw
                if candidate.exists():
                    return str(candidate)
            via_path = shutil.which(raw)
            if via_path:
                return via_path
            if path_obj.exists():
                return str(path_obj)
            return None

        if enable_gccrs:
            gccrs_bin = _resolve_compiler_bin(gccrs_cmd, gccrs_work_dir)
            gccrs_available = gccrs_bin is not None
            if not gccrs_available:
                logging.warning(
                    "compiler.enable_gccrs=true but gccrs binary not found: cmd=%s work_dir=%s; disabling gccrs for this run",
                    gccrs_cmd,
                    gccrs_work_dir,
                )
                enable_gccrs = False
            else:
                gccrs_cmd = [gccrs_bin, *gccrs_cmd[1:]]
                gccrs_env.update({
                    "LANG": "C",
                    "LC_ALL": "C",
                    "LANGUAGE": "en_US:en",
                })
                logging.info(
                    "gccrs compile enabled (cmd=%s, work_dir=%s)",
                    " ".join(map(str, gccrs_cmd)),
                    gccrs_work_dir or "<none>",
                )

        next_solver_flag = compiler_cfg.get("next_trait_solver_flag", "-Znext-solver=coherence")
        parallel_compile = bool(compiler_cfg.get("parallel_compile", False))
        parallel_workers = int(compiler_cfg.get("parallel_workers", 3))
        if parallel_compile:
            logging.info("Parallel compile enabled (workers=%d)", parallel_workers)

        # Config Parameters
        iterations = config["fuzzer"]["iterations"]
        mutations_per_seed = int(config["fuzzer"].get("mutations_per_seed", 1))
        injection_mutations_per_round = int(
            config["fuzzer"].get("injection_mutations_per_round", 1)
        )
        projection_mutations_per_round = int(
            config["fuzzer"].get("projection_mutations_per_round", injection_mutations_per_round)
        )
        structural_mutations_per_round = int(
            config["fuzzer"].get("structural_mutations_per_round", 1)
        )
        mutation_max_retries = int(config["fuzzer"].get("mutation_max_retries", 10))
        choice_pick_tries = int(config["fuzzer"].get("choice_pick_tries", 20))
        seed_strategy = config["fuzzer"].get("seed_selection_strategy", "random")

        # Fuzzing Loop
        for i in range(iterations):
            # 1. Select Seed
            seed_path = selector.select(seed_strategy)
            while seed_path is not None and seed_path in bad_seeds and selector.seeds:
                seed_path = selector.select(seed_strategy)
            if seed_path is None:
                logging.warning("No usable seeds available")
                return
            if not Path(seed_path).exists():
                selector.remove_seed(Path(seed_path))
                logging.info("Selected seed missing on disk; skipping iteration %d", i + 1)
                continue
            
            # Read seed content once
            abs_seed_path = seed_path.resolve()
            if not abs_seed_path.exists():
                logging.error(f"Seed file not found: {abs_seed_path}")
                continue
            
            with open(abs_seed_path, 'r', encoding='utf-8') as f:
                seed_content = f.read()
            logging.info(f"Read seed {abs_seed_path} (size={len(seed_content)})")

            current_seed_content = seed_content
            best_cross_feature_candidate: Optional[Dict[str, object]] = None

            parent_key = seed_path.resolve()
            ancestor_family = selector.get_family(seed_path)

            # Baseline compilation for the original seed (lazy).
            # Used only to decide whether CRASH/HANG variants should be classified as "fate"
            # (i.e., the seed already CRASH/HANGs before mutation).
            seed_baseline_results = None
            seed_is_fate_by_mode: Dict[str, bool] = {}
            seed_is_miscompilation: Optional[bool] = None
            fuzz_passes = [("original", current_seed_content)]

            for pass_name, pass_content in fuzz_passes:
                # Update content for this pass
                current_seed_content = pass_content
                logging.info(f"--- Starting Fuzz Pass: {pass_name} ---")
                
                # Reset key per-pass state
                seed_baseline_results = None
                seed_is_fate_by_mode = {}
                seed_is_miscompilation = None

                def _compile_seed_baseline() -> Dict[str, object]:
                    nonlocal seed_baseline_results
                    if seed_baseline_results is not None:
                        return seed_baseline_results
    
                    baseline_src = Path(f"temp_seed_baseline_w{worker_index}_iter_{i+1}.rs")
                    with open(baseline_src, "w") as f:
                        f.write(current_seed_content)
    
                    try:
                        base_stable = compiler.compile(baseline_src)
                        out: Dict[str, object] = {"stable": base_stable}
    
                        if enable_nightly_compile:
                            compiler_nightly = RustCompiler(
                                timeout=config["fuzzer"]["max_time_per_case_sec"],
                                rustc_cmd=nightly_rustc_cmd,
                            )
                            out["nightly"] = compiler_nightly.compile(baseline_src)
    
                        if enable_next_solver:
                            compiler_next = RustCompiler(
                                timeout=config["fuzzer"]["max_time_per_case_sec"],
                                rustc_cmd=nightly_rustc_cmd,
                            )
                            out["next"] = compiler_next.compile(baseline_src, extra_args=[next_solver_flag])

                        if enable_gccrs:
                            compiler_gccrs = RustCompiler(
                                timeout=config["fuzzer"]["max_time_per_case_sec"],
                                rustc_cmd=gccrs_cmd,
                                working_dir=gccrs_work_dir,
                                env=gccrs_env,
                                auto_no_core=gccrs_auto_no_core,
                            )
                            out["gccrs"] = compiler_gccrs.compile(
                                baseline_src,
                                extra_args=gccrs_extra_args,
                            )
    
                        seed_baseline_results = out
                        return out
                    finally:
                        try:
                            if baseline_src.exists():
                                baseline_src.unlink()
                        except Exception:
                            pass
    
                def _is_seed_fate_for(mode: str) -> bool:
                    """A seed is 'fate' for a compiler mode if its baseline already CRASH/HANGs there."""
                    nonlocal seed_is_fate_by_mode
                    key = str(mode)
                    if key in seed_is_fate_by_mode:
                        return bool(seed_is_fate_by_mode[key])

                    try:
                        base = _compile_seed_baseline()
                        if key == "rustc":
                            order = {
                                CompilationStatus.CRASH: 4,
                                CompilationStatus.HANG: 3,
                                CompilationStatus.ERROR: 2,
                                CompilationStatus.SUCCESS: 1,
                                CompilationStatus.UNKNOWN: 0,
                            }
                            worst_status = None
                            for mk in ("stable", "nightly", "next"):
                                r = base.get(mk)
                                if r is None:
                                    continue
                                st = r.status  # type: ignore[attr-defined]
                                if worst_status is None or order.get(st, 0) > order.get(worst_status, 0):
                                    worst_status = st
                            val = worst_status in (CompilationStatus.CRASH, CompilationStatus.HANG)
                        elif key == "gccrs":
                            rg = base.get("gccrs")
                            st = rg.status if rg is not None else CompilationStatus.UNKNOWN
                            val = st in (CompilationStatus.CRASH, CompilationStatus.HANG)
                        else:
                            val = False

                        seed_is_fate_by_mode[key] = bool(val)
                        return bool(val)
                    except Exception:
                        seed_is_fate_by_mode[key] = False
                        return False
    
                def _is_seed_miscompilation() -> bool:
                    """A seed is 'miscompilation' if baseline nightly vs next-solver disagree (SUCCESS vs ERROR)."""
                    nonlocal seed_is_miscompilation
                    if seed_is_miscompilation is not None:
                        return seed_is_miscompilation
                    
                    # Check config flag first
                    if not args.detect_miscompilation:
                        seed_is_miscompilation = False
                        return False
    
                    if not (enable_nightly_compile and enable_next_solver):
                        seed_is_miscompilation = False
                        return False
                    try:
                        base = _compile_seed_baseline()
                        rn = base.get("nightly")
                        rx = base.get("next")
                        if rn is None or rx is None:
                            seed_is_miscompilation = False
                            return False
                        nst = rn.status
                        xst = rx.status
                        seed_is_miscompilation = (
                            nst in (CompilationStatus.SUCCESS, CompilationStatus.ERROR)
                            and xst in (CompilationStatus.SUCCESS, CompilationStatus.ERROR)
                            and nst != xst
                        )
                        return bool(seed_is_miscompilation)
                    except Exception:
                        seed_is_miscompilation = False
                        return False
    
                try:
                    rel = seed_path.relative_to(seeds_dir)
                    rel_s = str(rel)
                except Exception:
                    rel_s = str(seed_path)
                logging.info(f"Iteration {i+1}/{iterations}: Selected seed {rel_s}")
    
                # Deduplicate identical mutations produced from the same seed.
                # Keyed by mutator strategy name; values are sha256 hashes of mutated content.
                # This is per selected seed (per outer iteration) to keep memory bounded.
                seen_mutations_by_strategy: Dict[str, set] = {}
    
                # Note: mutation-point/constraint consumption is reset per round (see inside round loop).
    
                def _pick_structural_op() -> Optional[str]:
                    ops = list(getattr(mutator_pool, "structural_ops", []))
                    if not ops:
                        return None
                    subw = getattr(mutator_pool, "structural_subweights", {}) or {}
                    weights = [float(subw.get(op, 0.0)) for op in ops]
                    if not any(w > 0 for w in weights):
                        weights = [1.0] * len(ops)
                    return random.choices(ops, weights=weights, k=1)[0]

                def _pick_injection_op() -> str:
                    if hasattr(mutator_pool, "select_injection_op"):
                        return str(mutator_pool.select_injection_op())
                    return "constraint_injection"
    
                # 2. Variants Loop: each seed runs `mutations_per_seed` rounds.
                # Each round: N TB-injection operators (Mutation II / Mutation IV),
                # then one structural mutation(s).
                variant_index = 0
                rounds = max(0, mutations_per_seed)
                skip_seed_due_to_parse = False
                skip_remaining_rounds = False
                for _round in range(rounds):
                    if skip_seed_due_to_parse or skip_remaining_rounds:
                        break
                    
                    # Update variant_id to include pass_name to avoid collisions
                    # e.g. w0_iter_1_original_var_1 vs w0_iter_1_rewritten_var_1
                    # Avoid repeatedly sampling the same mutation *point* inside the AST mutators (per round).
                    # Keyed by mutator strategy name; values are 0-based candidate indices already tried.
                    used_indices_by_strategy: Dict[str, set] = {}
                    # Keyed by strategy name; values are total candidate count reported by mutation-AST.
                    known_candidate_counts: Dict[str, int] = {}
                    # TB candidate counts per site for TB-injection mutators (per round).
                    known_constraint_counts: Dict[int, int] = {}
                    used_constraints_by_site: Dict[int, set] = {}
                    # Leaf strategies that have no remaining mutation points for this round.
                    exhausted_strategies: set = set()
                    exhausted_in_round: set = set()
                    logged_strategy_start: set = set()
                    round_seed_content = current_seed_content
                    round_seed_path = seed_path
                    round_seed_temp: Optional[Path] = None
                    if round_seed_content != seed_content:
                        round_seed_temp = Path(f"temp_round_seed_w{worker_index}_iter_{i+1}_round_{_round+1}.rs")
                        round_seed_temp.write_text(round_seed_content, encoding="utf-8", errors="ignore")
                        round_seed_path = round_seed_temp
                    logging.info("-" * 60)
                    logging.info("round %d/%d (base=%s)", _round + 1, rounds, round_seed_path.name)
                    try:
                        round_complexity = ttcg_model.calculate_complexity_for_file(round_seed_path)
                        round_constraint_sites = int(round_complexity.extra.get("constraint_sites", 0))
                        round_constraint_choice = int(round_complexity.extra.get("constraint_choice_sum", 0))
                        round_rewrite_sites = int(round_complexity.extra.get("rewrite_sites", 0))
                        round_rewrite_choice = int(round_complexity.extra.get("rewrite_choice_sum", 0))
                        round_lifetime_sites = int(round_complexity.extra.get("lifetime_sites", 0))
                        round_outlive_sites = int(round_complexity.extra.get("outlive_sites", 0))
    
                        logging.info(
                            "mutationⅡ: choice=%d sites=%d",
                            round_constraint_choice,
                            round_constraint_sites,
                        )
                        logging.info(
                            "mutationⅢ: choice=%d sites=%d",
                            round_rewrite_choice,
                            round_rewrite_sites,
                        )
                    except Exception:
                        pass
    
                    # Re-read mutation counts from config per round, allowing dynamic changes if needed.
                    injection_mutations_per_round = int(config["fuzzer"].get("injection_mutations_per_round", 20))
                    projection_mutations_per_round = int(config["fuzzer"].get("projection_mutations_per_round", 20))
                    structural_mutations_per_round = int(config["fuzzer"].get("structural_mutations_per_round", 1))
    
                    # `exhausted_strategies` is reset per seed (outer loop).
                    # `exhausted_in_round` is reset per round (inner loop).
                    # The original code used `exhausted_strategies` for per-seed exhaustion,
                    # but it was reset per round due to its placement.
                    # Let's keep the existing logic for `exhausted_strategies` (reset per round)
                    # and add `exhausted_in_round` for clarity if needed, but for now,
                    # `exhausted_strategies` effectively serves as per-round exhaustion.
                    # The user's change implies `exhausted_strategies` is defined outside this block,
                    # but the provided snippet re-initializes it here. I will follow the snippet.
                    exhausted_strategies = set() # This re-initializes it per round, matching the user's snippet.
                    exhausted_in_round = set() # This is a new variable, also reset per round.
    
                    planned_strategies: List[str] = []
                    if not args.structural_only:
                        if injection_mutations_per_round > 0:
                            for _ in range(injection_mutations_per_round):
                                planned_strategies.append(_pick_injection_op())
                        if projection_mutations_per_round > 0:
                            for _ in range(projection_mutations_per_round):
                                planned_strategies.append("projection_rewrite")
                    
                    if structural_mutations_per_round > 0:
                        for _ in range(structural_mutations_per_round):
                            s = _pick_structural_op()
                            if s is not None:
                                planned_strategies.append(s)
    
                    for planned in planned_strategies:
                        if skip_seed_due_to_parse:
                            break
                        if planned in exhausted_in_round:
                            continue
                        variant_index += 1
                        variant_id = f"w{worker_index}_iter_{i+1}_{pass_name}_var_{variant_index}"
    
    
                        # ------------------------------------------------------------------
                        # Robust Mutation & Compilation Loop
                        # ------------------------------------------------------------------
                        max_retries = mutation_max_retries
                        mutated_content = None
                        current_strategy = None
                        inapplicable_retries = 0
                        skip_iteration_due_to_inapplicable = False
                        skip_inapplicable_reason = None
    
                        def _is_duplicate_mutation(strategy: str, content: str) -> bool:
                            if not strategy or content is None:
                                return False
                            h = hashlib.sha256(content.encode("utf-8", errors="ignore")).hexdigest()
                            seen = seen_mutations_by_strategy.setdefault(strategy, set())
                            if h in seen:
                                return True
                            seen.add(h)
                            return False
    
                        # A. Mutation Retry Loop
                        logged_retry: set = set()
                        for attempt in range(max_retries):
                            try:
                                current_strategy = planned
                                if current_strategy is None:
                                    logging.warning(
                                        f"[{variant_id}] No planned strategy available; stopping further variants."
                                    )
                                    skip_iteration_due_to_inapplicable = True
                                    break
    
                                # Log strategy (once per variant)
                                if attempt == 0:
                                    is_injection = current_strategy in ("constraint_injection", "supertrait_injection")
                                    is_projection = current_strategy == "projection_rewrite"
    
                                    if is_injection and current_strategy not in logged_strategy_start:
                                        logged_strategy_start.add(current_strategy)
                                        color = "\033[38;5;217m"  # light pink
                                        reset = "\033[0m"
                                        if current_strategy == "supertrait_injection":
                                            logging.info(f"{color}MutationⅣ started (supertrait TB injection){reset}")
                                        else:
                                            logging.info(f"{color}MutationⅡ started (TB injection){reset}")
    
                                    if is_projection and "projection_rewrite" not in logged_strategy_start:
                                        logged_strategy_start.add("projection_rewrite")
                                        color = "\033[33m"  # yellow
                                        reset = "\033[0m"
                                        logging.info(f"{color}MutationⅢ started{reset}")

                                    if not is_injection and not is_projection:
                                        mutation_label = "MutationⅠ"
                                        color = "\033[34m"
                                        reset = "\033[0m"
                                        logging.info(
                                            f"{color}{mutation_label}{reset} -> Variant {variant_index}: Strategy {current_strategy}"
                                        )
    
                                # 2. Perform Mutation
                                # Rust AST Mutation
                                rust_mode = current_strategy
                                bin_dir = Path("mutation/mutation-AST")
                                with tempfile.TemporaryDirectory(prefix=f"trait_fuzzer_mut_{variant_id}_") as td:
                                    output_temp = Path(td) / "mutant.rs"

                                    # If we already know this mutator's candidate count for this seed,
                                    # pick an unseen index to avoid re-sampling the same mutation point.
                                    forced_index = None
                                    cand_count = known_candidate_counts.get(current_strategy)
                                    if cand_count is not None:
                                        if cand_count <= 0:
                                            exhausted_strategies.add(current_strategy)
                                            exhausted_in_round.add(current_strategy)
                                            logging.info(
                                                "    [Exhausted] Strategy %s has no remaining mutation points (count=%d); skipping",
                                                current_strategy,
                                                cand_count,
                                            )
                                            skip_iteration_due_to_inapplicable = True
                                            break
                                        used = used_indices_by_strategy.setdefault(current_strategy, set())
                                        if len(used) >= cand_count:
                                            exhausted_strategies.add(current_strategy)
                                            exhausted_in_round.add(current_strategy)
                                            inapplicable_retries += 1
                                            key = (variant_id, current_strategy, "exhausted")
                                            if key not in logged_retry:
                                                action = "skipping" if current_strategy in ("constraint_injection", "supertrait_injection", "add_trait", "add_impl") else "retrying"
                                                logging.info(
                                                    f"    [Exhausted] Strategy {current_strategy} has no remaining mutation points (count={cand_count}); {action}..."
                                                )
                                                logged_retry.add(key)
                                            if current_strategy in ("constraint_injection", "supertrait_injection", "add_trait", "add_impl"):
                                                skip_iteration_due_to_inapplicable = True
                                                skip_inapplicable_reason = "exhausted"
                                                break
                                            if inapplicable_retries >= max_retries:
                                                logging.warning(
                                                    f"[{variant_id}] Strategy inapplicable hit max retries ({max_retries}); skipping this iteration."
                                                )
                                                skip_iteration_due_to_inapplicable = True
                                                break
                                            continue

                                        # Choose a random unused index.
                                        for _ in range(choice_pick_tries):
                                            idx = random.randrange(cand_count)
                                            if idx not in used:
                                                forced_index = idx
                                                break
                                        if forced_index is None:
                                            remaining = [k for k in range(cand_count) if k not in used]
                                            forced_index = random.choice(remaining)

                                    cmd = [
                                        str(mutation_bin_path),
                                        "--input", str(round_seed_path.absolute()),
                                        "--output", str(output_temp.absolute()),
                                        "--mode", rust_mode,
                                        "--emit-choice",
                                    ]

                                    # If TB-injection mutator, use global choice index.
                                    if current_strategy in ("constraint_injection", "supertrait_injection") and forced_index is not None:
                                        cmd.extend(["--constraint-index", str(forced_index)])

                                    # If projection_rewrite, use global choice index.
                                    if current_strategy == "projection_rewrite" and forced_index is not None:
                                        cmd.extend(["--choice-index", str(forced_index)])

                                    if forced_index is not None and current_strategy not in (
                                        "constraint_injection",
                                        "supertrait_injection",
                                        "projection_rewrite",
                                    ):
                                        cmd.extend(["--index", str(forced_index)])

                                    proc = subprocess.run(
                                        cmd,
                                        cwd=str(bin_dir.absolute()),
                                        check=True,         # Will raise CalledProcessError on non-zero exit
                                        capture_output=True,
                                        text=True,
                                        encoding="utf-8",
                                        errors="replace",
                                    )

                                    # Record which mutation point was actually sampled.
                                    try:
                                        m = re.search(
                                            r"MUTATION_CHOICE\s+mode=(\S+)\s+count=(\d+)\s+index=(\d+)\s+mutated=(\d+)",
                                            proc.stderr,
                                        )
                                        if m is not None:
                                            count = int(m.group(2))
                                            index = int(m.group(3))
                                            if current_strategy not in ("constraint_injection", "supertrait_injection", "projection_rewrite"):
                                                known_candidate_counts[current_strategy] = count
                                                used_set = used_indices_by_strategy.setdefault(current_strategy, set())
                                                used_set.add(index)
                                                if count > 0 and len(used_set) >= count:
                                                    exhausted_strategies.add(current_strategy)
                                        # Parse choice_count/choice_index if present.
                                        if current_strategy in ("constraint_injection", "supertrait_injection"):
                                            m2 = re.search(
                                                r"choice_count=(\d+)\s+choice_index=(\d+)",
                                                proc.stderr,
                                            )
                                            if m2 is not None:
                                                ccount = int(m2.group(1))
                                                cidx = int(m2.group(2))
                                                known_candidate_counts[current_strategy] = ccount
                                                used_indices_by_strategy.setdefault(current_strategy, set()).add(cidx)
                                        # Parse choice_count/choice_index if present (projection_rewrite).
                                        if current_strategy == "projection_rewrite":
                                            m3 = re.search(
                                                r"choice_count=(\d+)\s+choice_index=(\d+)",
                                                proc.stderr,
                                            )
                                            if m3 is not None:
                                                pcount = int(m3.group(1))
                                                pidx = int(m3.group(2))
                                                known_candidate_counts[current_strategy] = pcount
                                                used_indices_by_strategy.setdefault(current_strategy, set()).add(pidx)
                                    except Exception:
                                        # Best-effort only; falling back to hash-based dedup is fine.
                                        pass

                                    # If syn cannot parse this seed, blacklist it and move on.
                                    if "Parse failed:" in proc.stderr:
                                        logging.warning(
                                            f"[{variant_id}] Seed not parseable by syn; skipping: {round_seed_path.name}"
                                        )
                                        if round_seed_path == seed_path:
                                            bad_seeds.add(seed_path)
                                            selector.remove_seed(seed_path)
                                        mutated_content = None
                                        skip_seed_due_to_parse = True
                                        skip_iteration_due_to_inapplicable = True
                                        break

                                    # Check for No-Op
                                    if "No mutation performed" in proc.stderr:
                                        if current_strategy in ("add_trait", "add_impl", "constraint_injection", "supertrait_injection"):
                                            exhausted_in_round.add(current_strategy)
                                            logging.info(
                                                f"    [No-Op] Strategy {current_strategy} produced no mutation; skipping without retry."
                                            )
                                            skip_iteration_due_to_inapplicable = True
                                            skip_inapplicable_reason = "noop"
                                            break
                                        inapplicable_retries += 1
                                        key = (variant_id, current_strategy, "noop")
                                        if key not in logged_retry:
                                            logging.info(
                                                f"    [No-Op] Strategy {current_strategy} inapplicable. Retrying..."
                                            )
                                            logged_retry.add(key)
                                        if inapplicable_retries >= max_retries:
                                            logging.warning(
                                                f"[{variant_id}] Strategy inapplicable hit max retries ({max_retries}); skipping this iteration."
                                            )
                                            skip_iteration_due_to_inapplicable = True
                                            break
                                        continue  # Retry loop

                                    # Success case
                                    if output_temp.exists():
                                        with open(output_temp, 'r') as f:
                                            mutated_content = f.read()

                                        if mutated_content is not None and _is_duplicate_mutation(current_strategy, mutated_content):
                                            logging.info(
                                                f"    [Dup] Strategy {current_strategy} produced identical mutant; skipping this variant."
                                            )
                                            mutated_content = None
                                            inapplicable_retries += 1
                                            if inapplicable_retries >= max_retries:
                                                logging.warning(
                                                    f"[{variant_id}] Duplicate mutant hit max retries ({max_retries}); skipping this iteration."
                                                )
                                                skip_iteration_due_to_inapplicable = True
                                                break
                                            continue
                                        break  # Mutated successfully
                                    else:
                                        logging.error(f"[{variant_id}] Rust mutation tool produced no output")
                                        continue
    
                                # If we blacklisted the seed, stop trying more strategies for it.
                                if seed_path in bad_seeds:
                                    break
    
                            except subprocess.CalledProcessError as e:
                                logging.error(f"[{variant_id}] Mutation tool failed: {e.stderr}")
                                continue # Retry
                            except Exception as e:
                                logging.error(f"[{variant_id}] Unexpected error during mutation: {e}")
                                continue
    
                        if skip_iteration_due_to_inapplicable:
                            if skip_inapplicable_reason != "exhausted":
                                logging.info(
                                    "[%s] Strategy inapplicable; skipping this strategy and continuing",
                                    variant_id,
                                )
                            continue
    
                        # B. Compilation & Analysis (Outside Retry Loop)
                        if mutated_content is None:
                            logging.warning(f"[{variant_id}] Failed to produce mutation after {max_retries} attempts.")
                            continue

                        kill_fate_now = False
                        temp_src = None
                        try:
                            # 3. Save & Compile
                            temp_src = Path(f"temp_{variant_id}.rs")
                            with open(temp_src, 'w') as f:
                                f.write(mutated_content)
    
                            # 3. Compile (oracle): stable once, +nightly once, +nightly with -Z next-solver once.
                            def _compile_with(rustc_cmd, extra_args=None):
                                use_gccrs_context = _is_gccrs_cmd(rustc_cmd)
                                comp = RustCompiler(
                                    timeout=config["fuzzer"]["max_time_per_case_sec"],
                                    rustc_cmd=rustc_cmd,
                                    working_dir=gccrs_work_dir if use_gccrs_context else None,
                                    env=gccrs_env if use_gccrs_context else None,
                                    auto_no_core=gccrs_auto_no_core if use_gccrs_context else False,
                                )
                                return comp.compile(temp_src, extra_args=extra_args)
    
                            result_stable = None
                            result_nightly = None
                            result_next = None
                            result_gccrs = None
    
                            should_run_extras = (enable_nightly_compile or enable_next_solver or enable_gccrs)
    
                            if parallel_compile and should_run_extras:
                                with ThreadPoolExecutor(max_workers=parallel_workers) as ex:
                                    futures = {}
                                    futures["stable"] = ex.submit(
                                        _compile_with,
                                        compiler_cfg.get("rustc_cmd"),
                                        None,
                                    )
                                    if enable_nightly_compile:
                                        futures["nightly"] = ex.submit(
                                            _compile_with,
                                            nightly_rustc_cmd,
                                            None,
                                        )
                                    if enable_next_solver:
                                        futures["next"] = ex.submit(
                                            _compile_with,
                                            nightly_rustc_cmd,
                                            [next_solver_flag],
                                        )
                                    if enable_gccrs:
                                        futures["gccrs"] = ex.submit(
                                            _compile_with,
                                            gccrs_cmd,
                                            gccrs_extra_args,
                                        )
    
                                    result_stable = futures["stable"].result()
                                    if "nightly" in futures:
                                        result_nightly = futures["nightly"].result()
                                    if "next" in futures:
                                        result_next = futures["next"].result()
                                    if "gccrs" in futures:
                                        result_gccrs = futures["gccrs"].result()
                            else:
                                result_stable = compiler.compile(temp_src)
    
                                if enable_nightly_compile:
                                    compiler_nightly = RustCompiler(
                                        timeout=config["fuzzer"]["max_time_per_case_sec"],
                                        rustc_cmd=nightly_rustc_cmd,
                                    )
                                    result_nightly = compiler_nightly.compile(temp_src)
    
                                if enable_next_solver:
                                    compiler_next = RustCompiler(
                                        timeout=config["fuzzer"]["max_time_per_case_sec"],
                                        rustc_cmd=nightly_rustc_cmd,
                                    )
                                    result_next = compiler_next.compile(temp_src, extra_args=[next_solver_flag])

                                if enable_gccrs:
                                    compiler_gccrs = RustCompiler(
                                        timeout=config["fuzzer"]["max_time_per_case_sec"],
                                        rustc_cmd=gccrs_cmd,
                                        working_dir=gccrs_work_dir,
                                        env=gccrs_env,
                                        auto_no_core=gccrs_auto_no_core,
                                    )
                                    result_gccrs = compiler_gccrs.compile(
                                        temp_src,
                                        extra_args=gccrs_extra_args,
                                    )
    
                            def _rank(status: CompilationStatus) -> int:
                                order = {
                                    CompilationStatus.CRASH: 4,
                                    CompilationStatus.HANG: 3,
                                    CompilationStatus.ERROR: 2,
                                    CompilationStatus.SUCCESS: 1,
                                    CompilationStatus.UNKNOWN: 0,
                                }
                                return order.get(status, 0)
    
                            # Overall status: take the worst one (useful for triage/logging).
                            result = result_stable
                            for r in (result_nightly, result_next, result_gccrs):
                                if r is not None and _rank(r.status) > _rank(result.status):
                                    result = r

                            # rustc namespace result (stable/nightly/next only)
                            rustc_result = result_stable
                            for r in (result_nightly, result_next):
                                if r is not None and _rank(r.status) > _rank(rustc_result.status):
                                    rustc_result = r
    
                            variant_by_mode = {"stable": result_stable}
                            if result_nightly is not None:
                                variant_by_mode["nightly"] = result_nightly
                            if result_next is not None:
                                variant_by_mode["next"] = result_next
                            if result_gccrs is not None:
                                variant_by_mode["gccrs"] = result_gccrs
    
                            # 4. Categorize & persist
                            # Always keep HANG/CRASH(ICE) and ERROR. SUCCESS is also kept (capped)
                            # and additionally promoted into seeds/newN.
                            rustc_status_name = rustc_result.status.value.lower()
                            rustc_should_persist = True
                            # Allow explicitly disabling SUCCESS persistence (still can be promoted).
                            if rustc_result.status == CompilationStatus.SUCCESS and args.keep_success_cases == 0:
                                rustc_should_persist = False
    
                            # Miscompilation: nightly default vs -Znext-solver disagree (SUCCESS vs ERROR).
                            miscompilation = False
                            if args.detect_miscompilation and enable_nightly_compile and enable_next_solver:
                                if result_nightly is not None and result_next is not None:
                                    nst = result_nightly.status
                                    xst = result_next.status
                                    if (
                                        nst in (CompilationStatus.SUCCESS, CompilationStatus.ERROR)
                                        and xst in (CompilationStatus.SUCCESS, CompilationStatus.ERROR)
                                        and nst != xst
                                    ):
                                        miscompilation = True
    
                            # If baseline already miscompiles, classify as fate (do not place into mis/other).
                            if miscompilation and _is_seed_miscompilation():
                                rustc_status_name = "fate"
                                rustc_should_persist = True
                                miscompilation = False
    
                            # rustc fate policy (compiler-specific)
                            if rustc_should_persist and rustc_result.status in (CompilationStatus.CRASH, CompilationStatus.HANG):
                                # Stop mutating this seed immediately if we found a bug/hang.
                                skip_remaining_rounds = True
                                
                                selector.ban_family(ancestor_family)
                                if _is_seed_fate_for("rustc"):
                                    rustc_status_name = "fate"
                                    logging.info(
                                        "[%s] rustc baseline already %s; classifying as rustc/fate",
                                        variant_id,
                                        rustc_result.status.value,
                                    )
                                    # Kill fate: skip remaining attempts for this seed.
                                    kill_fate_now = True
                                else:
                                    # Prominent alert for real HANG/ICE (non-fate)
                                    alert_color = "\033[1;31m"  # bright red
                                    alert_reset = "\033[0m"
                                    alert_label = "ICE" if rustc_result.status == CompilationStatus.CRASH else "HANG"
                                    logging.error(
                                        f"{alert_color}!!! rustc {alert_label} DETECTED !!!{alert_reset} Stopping further mutation for this seed."
                                    )
                                    # Ensure we break the inner loop immediately too.
                                    kill_fate_now = True

                            # gccrs fate policy (compiler-specific)
                            gccrs_status_name: Optional[str] = None
                            if result_gccrs is not None and result_gccrs.status in (CompilationStatus.CRASH, CompilationStatus.HANG):
                                skip_remaining_rounds = True
                                selector.ban_family(ancestor_family)
                                if _is_seed_fate_for("gccrs"):
                                    gccrs_status_name = "fate"
                                    logging.info(
                                        "[%s] gccrs baseline already %s; classifying as gccrs/fate",
                                        variant_id,
                                        result_gccrs.status.value,
                                    )
                                else:
                                    gccrs_status_name = result_gccrs.status.value.lower()
                                    alert_color = "\033[1;31m"
                                    alert_reset = "\033[0m"
                                    alert_label = "ICE" if result_gccrs.status == CompilationStatus.CRASH else "HANG"
                                    logging.error(
                                        f"{alert_color}!!! gccrs {alert_label} DETECTED !!!{alert_reset} Stopping further mutation for this seed."
                                    )
                                kill_fate_now = True
    
                            # Decide target categories (allow multiple when necessary).
                            dest_targets: List[tuple] = []
                            if rustc_should_persist:
                                dest_targets.append(("rustc", rustc_status_name))
                            if miscompilation:
                                dest_targets.append(("rustc", "miscompilation"))
                            if gccrs_status_name in ("crash", "hang", "fate"):
                                dest_targets.append(("gccrs", gccrs_status_name))
    
                            dest_cases: List[Path] = []
                            dest_case_meta: Dict[Path, Dict[str, str]] = {}
                            if dest_targets:
                                # Per-case safety check: prune prunable categories before we write more.
                                if not _enforce_all_results_limits():
                                    return
    
                                for compiler_ns, ds in dict.fromkeys(dest_targets):
                                    effective_status = ds
                                    bug_location = None
                                    duplicated = False

                                    if ds == "crash":
                                        crash_stderr = ""
                                        if compiler_ns == "gccrs" and result_gccrs is not None:
                                            crash_stderr = result_gccrs.stderr
                                        elif compiler_ns == "rustc" and rustc_result is not None:
                                            crash_stderr = rustc_result.stderr

                                        effective_status, bug_location, duplicated = _dedup_crash_status_by_location(
                                            results_root=results_dir,
                                            compiler_ns=compiler_ns,
                                            status_name=ds,
                                            stderr=crash_stderr,
                                        )
                                        if duplicated:
                                            logging.info(
                                                "[%s] %s crash duplicated by location (%s), storing in dup/",
                                                variant_id,
                                                compiler_ns,
                                                bug_location,
                                            )

                                    dest_dir = results_dir / compiler_ns / effective_status
                                    dest_case = dest_dir / f"case_{variant_id}"
                                    dest_case.mkdir(parents=True, exist_ok=True)
                                    shutil.copy(round_seed_path, dest_case / "before.rs")
                                    shutil.copy(temp_src, dest_case / "after.rs")
                                    dest_cases.append(dest_case)
                                    dest_case_meta[dest_case] = {
                                        "compiler": str(compiler_ns),
                                        "stored_status": str(effective_status),
                                        "bug_location": str(bug_location or ""),
                                    }
    
                            # 5. TTCG & Complexity (unified model)
                            complexity = ttcg_model.calculate_complexity_for_file(temp_src)
                            constraint_sites = int(complexity.extra.get("constraint_sites", 0))
                            constraint_choice_sum = int(complexity.extra.get("constraint_choice_sum", 0))

                            if cross_feature_ctx is not None:
                                obligation_details = _measure_obligation_score_with_details(mutated_content)
                                obligation_score = int(obligation_details.get("score", 0))
                                if (
                                    best_cross_feature_candidate is None
                                    or obligation_score > int(best_cross_feature_candidate.get("obligation_score", -1))
                                ):
                                    best_cross_feature_candidate = {
                                        "content": mutated_content,
                                        "variant_id": variant_id,
                                        "strategy": current_strategy,
                                        "obligation_score": obligation_score,
                                        "obligation_method": str(obligation_details.get("method", "unknown")),
                                    }
    
                            if temp_src is not None and temp_src.exists():
                                temp_src.unlink()
    
                            if constraint_choice_sum > max_choice["constraint_choice_sum"]:
                                max_choice["constraint_choice_sum"] = max(
                                    max_choice["constraint_choice_sum"],
                                    constraint_choice_sum,
                                )
    
                            if dest_cases:
                                # Build a summary string for the status, e.g. "Stable:HANG, Nightly:ICE"
                                status_details = []
                                # Also track which specific versions match the final reported status (the "culprits")
                                culprits = []
                                
                                if result_stable:
                                    status_details.append(f"Stable:{result_stable.status.name}")
                                    if result_stable.status == result.status:
                                        culprits.append("stable")
                                
                                if result_nightly:
                                    status_details.append(f"Nightly:{result_nightly.status.name}")
                                    if result_nightly.status == result.status:
                                        culprits.append("nightly")
                                        
                                if result_next:
                                    status_details.append(f"Next:{result_next.status.name}")
                                    if result_next.status == result.status:
                                        culprits.append("next-solver")

                                if result_gccrs:
                                    status_details.append(f"Gccrs:{result_gccrs.status.name}")
                                    if result_gccrs.status == result.status:
                                        culprits.append("gccrs")
                                        
                                status_summary = ", ".join(status_details)
                                version_str = "/".join(culprits)
    
                                # Log the detailed status to console if it's a Hang/ICE
                                rustc_nonfate_bug = (
                                    rustc_result.status in (CompilationStatus.CRASH, CompilationStatus.HANG)
                                    and not _is_seed_fate_for("rustc")
                                )
                                gccrs_nonfate_bug = (
                                    result_gccrs is not None
                                    and result_gccrs.status in (CompilationStatus.CRASH, CompilationStatus.HANG)
                                    and not _is_seed_fate_for("gccrs")
                                )
                                if rustc_nonfate_bug or gccrs_nonfate_bug:
                                    logging.info(
                                        f"[{variant_id}] Detail: {status_summary}"
                                    )
    
                                # Write Seed and Root as paths relative to the repository root when possible.
                                repo_root = Path(__file__).resolve().parent.parent
                                try:
                                    seed_rel = str(round_seed_path.resolve().relative_to(repo_root))
                                except Exception:
                                    seed_rel = str(round_seed_path)

                                # ancestor_family may be a path string or an identifier; try to make it a Path
                                try:
                                    fam_path = Path(ancestor_family)
                                    try:
                                        root_rel = str(fam_path.resolve().relative_to(repo_root))
                                    except Exception:
                                        root_rel = str(ancestor_family)
                                except Exception:
                                    root_rel = str(ancestor_family)

                                for dc in dest_cases:
                                    meta = dest_case_meta.get(dc, {})
                                    with open(dc / "detail.log", 'w') as f:
                                        f.write(f"Seed: {seed_rel}\n")
                                        f.write(f"Root: {root_rel}\n")
                                        if meta.get("compiler"):
                                            f.write(f"Compiler: {meta.get('compiler')}\n")
                                        f.write(f"Strategy: {current_strategy}\n")
                                        f.write(f"Status: {result.status.value}\n")
                                        if meta.get("stored_status"):
                                            f.write(f"Stored Status: {meta.get('stored_status')}\n")
                                        if meta.get("bug_location"):
                                            f.write(f"Bug Location: {meta.get('bug_location')}\n")
                                        f.write(f"Version: {version_str}\n")
                                        f.write(f"Status Breakdown: {status_summary}\n")
                                        if miscompilation and result_nightly is not None and result_next is not None:
                                            f.write("Miscompilation: nightly vs next-solver mismatch\n")
                                            f.write(f"Nightly: {result_nightly.status.value}\n")
                                            f.write(f"Next: {result_next.status.value}\n")
                                        f.write(f"Constraint Sites: {constraint_sites}\n")
                                        f.write(f"Constraint Choice Sum: {constraint_choice_sum}\n")
                                        f.write("\n=== rustc (stable) ===\n")
                                        f.write(f"Command: {' '.join(map(str, compiler.rustc_cmd))}\n")
                                        f.write(f"Status: {result_stable.status.value}\n")
                                        f.write(f"Duration: {result_stable.duration:.4f}s\n")
                                        f.write(f"Return code: {result_stable.return_code}\n")
                                        f.write(f"Stdout:\n{result_stable.stdout}\n")
                                        f.write(f"Stderr:\n{result_stable.stderr}\n")

                                        if result_nightly is not None:
                                            f.write("\n=== rustc (+nightly) ===\n")
                                            f.write(f"Command: {' '.join(map(str, nightly_rustc_cmd))}\n")
                                            f.write(f"Status: {result_nightly.status.value}\n")
                                            f.write(f"Duration: {result_nightly.duration:.4f}s\n")
                                            f.write(f"Return code: {result_nightly.return_code}\n")
                                            f.write(f"Stdout:\n{result_nightly.stdout}\n")
                                            f.write(f"Stderr:\n{result_nightly.stderr}\n")

                                        if result_next is not None:
                                            f.write("\n=== rustc (-Z next trait-solver) ===\n")
                                            f.write(f"Command: {' '.join(map(str, nightly_rustc_cmd + [next_solver_flag]))}\n")
                                            f.write(f"Status: {result_next.status.value}\n")
                                            f.write(f"Duration: {result_next.duration:.4f}s\n")
                                            f.write(f"Return code: {result_next.return_code}\n")
                                            f.write(f"Stdout:\n{result_next.stdout}\n")
                                            f.write(f"Stderr:\n{result_next.stderr}\n")

                                        if result_gccrs is not None:
                                            f.write("\n=== gccrs ===\n")
                                            f.write(f"Command: {' '.join(map(str, gccrs_cmd + gccrs_extra_args))}\n")
                                            f.write(f"Status: {result_gccrs.status.value}\n")
                                            f.write(f"Duration: {result_gccrs.duration:.4f}s\n")
                                            f.write(f"Return code: {result_gccrs.return_code}\n")
                                            f.write(f"Stdout:\n{result_gccrs.stdout}\n")
                                            f.write(f"Stderr:\n{result_gccrs.stderr}\n")
    
                            logging.info(
                                "[%s] Result summary: rustc=%s%s",
                                variant_id,
                                rustc_result.status.value,
                                f", gccrs={result_gccrs.status.value}" if result_gccrs is not None else "",
                            )
    
                            if skip_remaining_rounds:
                                logging.info("[%s] Kill fate: skipping remaining rounds for this seed", variant_id)
    
                            # Chain evolution: use MutationⅠ output as next round seed
                            if current_strategy in getattr(mutator_pool, "structural_ops", []):
                                current_seed_content = mutated_content
                                # New structure: reset mutation-point tracking for the next round
                                used_indices_by_strategy = {}
                                known_candidate_counts = {}
                                exhausted_strategies = set()
                                seen_mutations_by_strategy = {}
    
                            # Promote SUCCESS mutants into seeds/newN (rolling cap)
                            # Only TB-injection mutators (Mutation II / Mutation IV) are eligible.
                            promote_eligible = (
                                promoted_dir is not None
                                and args.promote_success
                                and current_strategy in ("constraint_injection", "supertrait_injection")
                            )
                            if promote_eligible and result.status == CompilationStatus.SUCCESS:
                                try:
                                    if max_promotions_per_seed <= 0:
                                        raise RuntimeError("SUCCESS promotion disabled by max_promotions_per_seed <= 0")
    
                                    promoted_so_far = int(promotions_by_seed.get(parent_key, 0))
                                    if promoted_so_far >= max_promotions_per_seed:
                                        if parent_key not in logged_promotion_cap:
                                            logging.info(
                                                "[%s] Skip promote (seed cap reached: %d/%d): %s",
                                                variant_id,
                                                promoted_so_far,
                                                max_promotions_per_seed,
                                                seed_path.name,
                                            )
                                            logged_promotion_cap.add(parent_key)
                                        continue
    
                                    # Roll to the next newN directory when the current one is full.
                                    promoted_dir = maybe_roll_promoted_dir(
                                        promoted_dir,
                                        seeds_dir,
                                        args.new_seeds_prefix,
                                        args.new_seeds_max,
                                    )
    
                                    out_path = promoted_dir / f"seed_{variant_id}.rs"
                                    if out_path.exists():
                                        out_path = promoted_dir / f"seed_{variant_id}_{int(time.time())}.rs"
                                    out_path.write_text(mutated_content, encoding='utf-8', errors='ignore')
                                    # Make the newly promoted seed immediately eligible for selection.
                                    try:
                                        selector.add_seed(out_path, family_id=ancestor_family)
                                    except Exception:
                                        pass
    
                                    promotions_by_seed[parent_key] = promoted_so_far + 1
                                except Exception as e:
                                    logging.warning("Failed to promote seed %s: %s", variant_id, e)
    
                        except Exception as e:
                            logging.error(f"[{variant_id}] Variant compilation/analysis failed: {e}")
                        finally:
                            try:
                                if temp_src is not None and temp_src.exists():
                                    temp_src.unlink()
                            except Exception:
                                pass
    
                        if kill_fate_now:
                            break
    
                    if round_seed_temp is not None:
                        try:
                            if round_seed_temp.exists():
                                round_seed_temp.unlink()
                        except Exception:
                            pass

            if cross_feature_ctx is not None:
                if best_cross_feature_candidate is None:
                    if cross_feature_seed_fallback:
                        fallback_details = _measure_obligation_score_with_details(current_seed_content)
                        best_cross_feature_candidate = {
                            "content": current_seed_content,
                            "variant_id": f"seed_fallback_{seed_path.stem}",
                            "strategy": "seed_fallback",
                            "obligation_score": int(fallback_details.get("score", 0)),
                            "obligation_method": str(fallback_details.get("method", "unknown")),
                        }
                        logging.info(
                            "[iter %d] no valid trait-constraint variant; fallback to seed content (score=%s method=%s)",
                            i + 1,
                            best_cross_feature_candidate.get("obligation_score", 0),
                            best_cross_feature_candidate.get("obligation_method", "unknown"),
                        )
                    else:
                        logging.info("[iter %d] cross-feature skipped: no valid trait-constraint variant available", i + 1)

                if best_cross_feature_candidate is not None:
                    logging.info(
                        "[iter %d] selected max-obligation variant=%s score=%s method=%s strategy=%s for cross-feature",
                        i + 1,
                        best_cross_feature_candidate.get("variant_id", "unknown"),
                        best_cross_feature_candidate.get("obligation_score", 0),
                        best_cross_feature_candidate.get("obligation_method", "unknown"),
                        best_cross_feature_candidate.get("strategy", "unknown"),
                    )
                    _run_cross_feature_for_candidate(i, best_cross_feature_candidate)
    
            # End of one outer iteration (one selected seed): prune prunable categories.
            if not _enforce_all_results_limits():
                return

        logging.info("Trait-Fuzzer finished.")
        
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

def main():
    # Parse config to find out how many workers we need
    args, config = parse_args_and_config()
    num_workers = int(config.get("fuzzer", {}).get("workers", 1))

    # Pre-build the mutation tool ONCE in the main process
    try:
        mutation_bin_path = ensure_mutation_tool_built()
    except Exception as e:
        print(f"Critical error building mutation tool: {e}")
        sys.exit(1)

    if num_workers <= 1:
        worker_main(0, 1, mutation_bin_path)
    else:
        print(f"Spawning {num_workers} parallel workers...")
        processes = []
        for i in range(num_workers):
            p = multiprocessing.Process(target=worker_main, args=(i, num_workers, mutation_bin_path))
            p.start()
            processes.append(p)
        
        def signal_handler(sig, frame):
            print("\nShutting down workers...")
            for p in processes:
                if p.is_alive():
                    p.terminate()
            sys.exit(0)

        signal.signal(signal.SIGINT, signal_handler)
        signal.signal(signal.SIGTERM, signal_handler)

        try:
            for p in processes:
                p.join()
        except KeyboardInterrupt:
            signal_handler(None, None)

if __name__ == "__main__":
    # Windows support for multiprocessing
    multiprocessing.freeze_support()
    main()
