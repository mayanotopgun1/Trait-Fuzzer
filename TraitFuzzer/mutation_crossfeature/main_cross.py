from __future__ import annotations

import argparse
import json
import logging
import random
import re
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional, Tuple

PROJECT_ROOT = Path(__file__).resolve().parents[1]
if str(PROJECT_ROOT) not in sys.path:
    sys.path.append(str(PROJECT_ROOT))

from LLM import LLMConnector
from LLM.agents.trait_rewriter import TraitRewriterAgent
from mutation_crossfeature.base_mutator import LLMMutatorBase, MutationTarget
from mutation_crossfeature.mutator_registry import build_mutators, default_operator_keys
from utils.compiler import RustCompiler, CompilationStatus


class SimpleFileLock:
    def __init__(self, lock_file: Path, timeout: float = 180.0, stale_after: float = 900.0):
        self.lock_file = Path(lock_file)
        self.timeout = timeout
        self.stale_after = stale_after

    def _lock_age_seconds(self) -> Optional[float]:
        try:
            st = self.lock_file.stat()
            return max(0.0, time.time() - float(st.st_mtime))
        except Exception:
            return None

    def _try_cleanup_stale_lock(self) -> bool:
        if not self.lock_file.exists():
            return False
        age = self._lock_age_seconds()
        if age is None or age < self.stale_after:
            return False
        try:
            self.lock_file.rmdir()
            logging.warning("Removed stale LLM lock: %s (age=%.1fs)", self.lock_file, age)
            return True
        except Exception:
            return False

    def __enter__(self):
        start = time.time()
        last_wait_log = 0.0
        while True:
            try:
                self.lock_file.mkdir(parents=True, exist_ok=False)
                return self
            except FileExistsError:
                if self._try_cleanup_stale_lock():
                    continue
                if time.time() - start > self.timeout:
                    raise TimeoutError(f"Could not acquire lock {self.lock_file} in {self.timeout}s")
                now = time.time()
                if now - last_wait_log >= 10.0:
                    age = self._lock_age_seconds()
                    if age is None:
                        logging.info("Waiting for LLM lock: %s", self.lock_file)
                    else:
                        logging.info("Waiting for LLM lock: %s (age=%.1fs)", self.lock_file, age)
                    last_wait_log = now
                time.sleep(0.5)

    def __exit__(self, exc_type, exc, tb):
        try:
            self.lock_file.rmdir()
        except Exception:
            pass


@dataclass
class CrossResult:
    seed: Path
    case_id: str
    operator_key: str
    status: str
    baseline_status: str
    compile_status: CompilationStatus
    compile_duration: float


def rank_status(status: CompilationStatus) -> int:
    order = {
        CompilationStatus.CRASH: 4,
        CompilationStatus.HANG: 3,
        CompilationStatus.ERROR: 2,
        CompilationStatus.SUCCESS: 1,
        CompilationStatus.UNKNOWN: 0,
    }
    return order.get(status, 0)


def worst_result(variant_results: Dict[str, object]):
    chosen = None
    for result in variant_results.values():
        if result is None:
            continue
        if chosen is None or rank_status(result.status) > rank_status(chosen.status):
            chosen = result
    return chosen


def load_config(path: Path) -> Dict:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Cross-feature experiment runner (multi-mutator)")
    parser.add_argument("--config", default="config.json", help="Path to config.json")
    parser.add_argument(
        "--cross-config",
        default="mutation_crossfeature/config_cross.json",
        help="Path to cross-feature config file",
    )
    parser.add_argument(
        "--operators",
        default="",
        help="Comma-separated operator keys to override config (e.g. lifetime_1,ownership_1)",
    )
    parser.add_argument("--seeds-dir", default=None, help="Seed root directory (default from config.paths.seeds)")
    parser.add_argument(
        "--results-dir",
        default=None,
        help="Output root directory for categorized cases",
    )
    parser.add_argument("--limit", type=int, default=None, help="Max number of seeds to process (0/all = all)")
    parser.add_argument(
        "--seed-selection",
        default="",
        help="Seed selection strategy override: random or deterministic",
    )
    parser.add_argument(
        "--shuffle-seed",
        type=int,
        default=None,
        help="Optional RNG seed for reproducible random seed selection",
    )
    parser.add_argument("--timeout", type=int, default=None, help="Override compiler timeout seconds")
    parser.add_argument("--log-level", default="INFO", help="Logging level: DEBUG/INFO/WARNING/ERROR")
    return parser.parse_args()


def setup_logging(level: str) -> None:
    lv = getattr(logging, str(level).upper(), logging.INFO)
    logging.basicConfig(level=lv, format="[cross] %(asctime)s - %(levelname)s - %(message)s")


def collect_seeds(
    seeds_dir: Path,
    limit: int,
    strategy: str = "random",
    shuffle_seed: Optional[int] = None,
) -> List[Path]:
    seeds = sorted([p for p in seeds_dir.rglob("*.rs") if p.is_file() and p.stat().st_size > 0])
    if not seeds:
        return seeds

    strategy = (strategy or "random").strip().lower()
    if strategy not in {"random", "deterministic"}:
        strategy = "random"

    if strategy == "random":
        rng = random.Random(shuffle_seed) if shuffle_seed is not None else random.Random()
        rng.shuffle(seeds)

    if limit > 0:
        seeds = seeds[:limit]
    return seeds


def classify_status(mutated: CompilationStatus, baseline: CompilationStatus) -> str:
    if mutated in (CompilationStatus.CRASH, CompilationStatus.HANG) and baseline in (
        CompilationStatus.CRASH,
        CompilationStatus.HANG,
    ):
        return "fate"
    return mutated.value.lower()


def parse_operator_override(raw: str) -> List[str]:
    if not raw:
        return []
    return [s.strip() for s in raw.split(",") if s.strip()]


def resolve_enabled_operators(args: argparse.Namespace, cross_cfg: Dict) -> List[str]:
    overridden = parse_operator_override(args.operators)
    if overridden:
        return overridden
    return list(cross_cfg.get("run", {}).get("enabled_operators", default_operator_keys()))


def should_skip_by_target_guard(op_key: str, target: MutationTarget, require_trait_ops: set) -> bool:
    if op_key in require_trait_ops and not target.trait_name:
        return True
    return False


def strip_rust_comments(code: str) -> str:
    """Remove Rust line/block comments while preserving strings/chars/raw strings.

    This is used only for LLM input (`notip` mode) to reduce prompt noise.
    """
    n = len(code)
    i = 0
    out: List[str] = []

    in_string = False
    in_char = False
    in_line_comment = False
    block_depth = 0
    in_raw_string = False
    raw_hashes = 0

    while i < n:
        ch = code[i]
        nxt = code[i + 1] if i + 1 < n else ""

        if in_line_comment:
            if ch == "\n":
                in_line_comment = False
                out.append(ch)
            i += 1
            continue

        if block_depth > 0:
            if ch == "/" and nxt == "*":
                block_depth += 1
                i += 2
                continue
            if ch == "*" and nxt == "/":
                block_depth -= 1
                i += 2
                continue
            i += 1
            continue

        if in_string:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                out.append(code[i + 1])
                i += 2
                continue
            if ch == '"':
                in_string = False
            i += 1
            continue

        if in_char:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                out.append(code[i + 1])
                i += 2
                continue
            if ch == "'":
                in_char = False
            i += 1
            continue

        if in_raw_string:
            out.append(ch)
            if ch == '"':
                j = i + 1
                k = 0
                while j < n and k < raw_hashes and code[j] == "#":
                    j += 1
                    k += 1
                if k == raw_hashes:
                    out.extend("#" * raw_hashes)
                    i = j
                    in_raw_string = False
                    continue
            i += 1
            continue

        # normal state
        if ch == "/" and nxt == "/":
            in_line_comment = True
            i += 2
            continue
        if ch == "/" and nxt == "*":
            block_depth = 1
            i += 2
            continue

        # raw string prefix: r###"..."###
        if ch == "r":
            j = i + 1
            hashes = 0
            while j < n and code[j] == "#":
                hashes += 1
                j += 1
            if j < n and code[j] == '"':
                in_raw_string = True
                raw_hashes = hashes
                out.append("r")
                out.extend("#" * hashes)
                out.append('"')
                i = j + 1
                continue

        if ch == '"':
            in_string = True
            out.append(ch)
            i += 1
            continue

        if ch == "'":
            in_char = True
            out.append(ch)
            i += 1
            continue

        out.append(ch)
        i += 1

    return "".join(out)


def infer_mutation_target(seed_code: str) -> MutationTarget:
    """Pick one concrete-ish target type and an optional related trait from seed text."""
    type_defs = re.findall(
        r"(?m)^\s*(?:pub\s+)?(?:struct|enum|union|type)\s+([A-Za-z_][A-Za-z0-9_]*)",
        seed_code,
    )
    trait_defs = re.findall(r"(?m)^\s*(?:pub\s+)?trait\s+([A-Za-z_][A-Za-z0-9_]*)", seed_code)

    impl_pairs = re.findall(
        r"(?m)^\s*impl(?:\s*<[^>]*>)?\s+([A-Za-z_][A-Za-z0-9_:<>]*)\s+for\s+([A-Za-z_][A-Za-z0-9_:<>]*)",
        seed_code,
    )

    fallback_candidates = re.findall(r"\b([A-Z][A-Za-z0-9_]*)\b", seed_code)
    fallback_blacklist = {
        "Self",
        "Result",
        "Option",
        "Vec",
        "String",
        "Box",
        "Rc",
        "Arc",
        "None",
        "Some",
        "Ok",
        "Err",
        "Copy",
        "Clone",
        "Debug",
        "Default",
        "Send",
        "Sync",
        "Sized",
        "Fn",
        "FnMut",
        "FnOnce",
        "Drop",
    }
    fallback_types = [name for name in fallback_candidates if name not in fallback_blacklist]

    candidates = list(dict.fromkeys(type_defs + [p[1].split("::")[-1].split("<")[0] for p in impl_pairs] + fallback_types))
    if not candidates:
        return MutationTarget(type_name="T", trait_name=trait_defs[0] if trait_defs else None)

    def _score_type(t: str) -> int:
        try:
            return len(re.findall(rf"\b{re.escape(t)}\b", seed_code))
        except Exception:
            return 0

    target_type = max(candidates, key=_score_type)

    target_trait = None
    for tr, ty in impl_pairs:
        ty_base = ty.split("::")[-1].split("<")[0]
        if ty_base == target_type:
            target_trait = tr.split("::")[-1].split("<")[0]
            break
    if target_trait is None and trait_defs:
        target_trait = trait_defs[0]

    return MutationTarget(type_name=target_type, trait_name=target_trait)


def extract_entities_via_ast(seed_code: str, repo_root: Path, timeout_sec: int = 20) -> Optional[Dict]:
    mutation_ast_dir = repo_root / "mutation" / "mutation-AST"
    bin_path = mutation_ast_dir / "target" / "debug" / "mutation-ast"

    with tempfile.TemporaryDirectory(prefix="cross_target_ast_") as td:
        tdp = Path(td)
        in_path = tdp / "input.rs"
        out_path = tdp / "output.rs"
        in_path.write_text(seed_code, encoding="utf-8", errors="ignore")

        cmds = []
        if bin_path.exists():
            cmds.append([
                str(bin_path.absolute()),
                "--input",
                str(in_path.absolute()),
                "--output",
                str(out_path.absolute()),
                "--mode",
                "ttcg_entities",
            ])
        cmds.append([
            "cargo",
            "run",
            "--quiet",
            "--",
            "--input",
            str(in_path.absolute()),
            "--output",
            str(out_path.absolute()),
            "--mode",
            "ttcg_entities",
        ])

        for cmd in cmds:
            try:
                proc = subprocess.run(
                    cmd,
                    cwd=str(mutation_ast_dir.absolute()),
                    capture_output=True,
                    text=True,
                    timeout=timeout_sec,
                    check=True,
                )
                payload = json.loads((proc.stdout or "").strip() or "{}")
                if isinstance(payload, dict) and payload.get("types") is not None:
                    return payload
            except Exception:
                continue
        return None
    return None


def infer_mutation_target_ast_first(seed_code: str, repo_root: Path) -> Tuple[MutationTarget, str]:
    payload = extract_entities_via_ast(seed_code, repo_root)
    if not payload:
        return infer_mutation_target(seed_code), "regex_fallback"

    types = [t for t in payload.get("types", []) if isinstance(t, str) and t]
    traits = [t for t in payload.get("traits", []) if isinstance(t, str) and t]
    impl_edges = payload.get("impl_edges", [])

    if not types:
        return infer_mutation_target(seed_code), "regex_fallback"

    def _score_type(t: str) -> int:
        occur = len(re.findall(rf"\b{re.escape(t)}\b", seed_code))
        edge_bonus = 0
        for e in impl_edges:
            if isinstance(e, dict) and e.get("type") == t:
                edge_bonus += 2
        return occur + edge_bonus

    target_type = max(types, key=_score_type)

    target_trait = None
    for e in impl_edges:
        if isinstance(e, dict) and e.get("type") == target_type and isinstance(e.get("trait"), str):
            target_trait = e.get("trait")
            break
    if target_trait is None and traits:
        target_trait = traits[0]

    return MutationTarget(type_name=target_type, trait_name=target_trait), "ast"


def compile_source(code: str, temp_path: Path, compiler: RustCompiler):
    temp_path.write_text(code, encoding="utf-8", errors="ignore")
    try:
        return compiler.compile(temp_path)
    finally:
        try:
            temp_path.unlink()
        except Exception:
            pass


def compile_variants(code: str, temp_path: Path, config: Dict, timeout_sec: int) -> Dict[str, object]:
    compiler_cfg = config.get("compiler", {})
    stable_cmd = compiler_cfg.get("rustc_cmd")
    nightly_cmd = compiler_cfg.get("rustc_z_cmd", ["rustc", "+nightly"])
    next_solver_flag = compiler_cfg.get("next_trait_solver_flag", "-Znext-solver=globally")
    enable_next_solver = bool(compiler_cfg.get("enable_next_trait_solver", False))
    enable_nightly_compile = bool(compiler_cfg.get("enable_nightly_compile", enable_next_solver))

    out: Dict[str, object] = {}

    stable_compiler = RustCompiler(timeout=timeout_sec, rustc_cmd=stable_cmd)
    out["stable"] = compile_source(code, temp_path, stable_compiler)

    if enable_nightly_compile:
        nightly_compiler = RustCompiler(timeout=timeout_sec, rustc_cmd=nightly_cmd)
        out["nightly"] = compile_source(code, temp_path, nightly_compiler)

    if enable_next_solver:
        next_compiler = RustCompiler(timeout=timeout_sec, rustc_cmd=nightly_cmd)
        next_temp = temp_path.with_name(temp_path.stem + "_next" + temp_path.suffix)
        next_temp.write_text(code, encoding="utf-8", errors="ignore")
        try:
            out["next"] = next_compiler.compile(next_temp, extra_args=[next_solver_flag])
        finally:
            try:
                next_temp.unlink()
            except Exception:
                pass

    return out


def persist_case(
    results_root: Path,
    category: str,
    case_id: str,
    operator_key: str,
    operator_name: str,
    target: MutationTarget,
    seed_path: Path,
    before_code: str,
    after_code: str,
    base_variants: Dict[str, object],
    mut_variants: Dict[str, object],
    prompt_text: Optional[str] = None,
) -> None:
    case_dir = results_root / category / f"case_{case_id}"
    case_dir.mkdir(parents=True, exist_ok=True)

    (case_dir / "before.rs").write_text(before_code, encoding="utf-8", errors="ignore")
    (case_dir / "after.rs").write_text(after_code, encoding="utf-8", errors="ignore")

    base_result = worst_result(base_variants)
    mut_result = worst_result(mut_variants)

    with open(case_dir / "detail.log", "w", encoding="utf-8") as f:
        f.write(f"Seed: {seed_path}\n")
        f.write(f"Operator Key: {operator_key}\n")
        f.write(f"Operator Name: {operator_name}\n")
        f.write(f"Target Type: {target.type_name}\n")
        f.write(f"Target Trait: {target.trait_name or 'None'}\n")
        f.write(f"Baseline Status: {base_result.status.value}\n")
        f.write(f"Mutated Status: {mut_result.status.value}\n")
        f.write(f"Mutated Duration: {mut_result.duration:.4f}s\n")
        for mode_name, r in base_variants.items():
            if r is None:
                continue
            f.write(f"\n=== baseline ({mode_name}) ===\n")
            f.write(f"Status: {r.status.value}\n")
            f.write(f"Return code: {r.return_code}\n")
            f.write(f"Stdout:\n{r.stdout}\n")
            f.write(f"Stderr:\n{r.stderr}\n")

        for mode_name, r in mut_variants.items():
            if r is None:
                continue
            f.write(f"\n=== mutated ({mode_name}) ===\n")
            f.write(f"Status: {r.status.value}\n")
            f.write(f"Return code: {r.return_code}\n")
            f.write(f"Stdout:\n{r.stdout}\n")
            f.write(f"Stderr:\n{r.stderr}\n")

    if prompt_text is not None:
        (case_dir / "llm_prompt.txt").write_text(prompt_text, encoding="utf-8", errors="ignore")


def run_one_seed_one_operator(
    index: int,
    seed_path: Path,
    seed_code: str,
    target: MutationTarget,
    base_variants,
    mutator: LLMMutatorBase,
    config: Dict,
    timeout_sec: int,
    lock_path: Path,
    lock_timeout: int,
    lock_stale_after: float,
    results_root: Path,
    save_prompt: bool,
    notip: bool,
    op_order: int,
    pass_name: str,
) -> Optional[CrossResult]:
    case_id = f"cross_iter_{index + 1}_{seed_path.stem}_{pass_name}_{mutator.meta.key}_op{op_order + 1}"

    llm_seed_code = strip_rust_comments(seed_code) if notip else seed_code

    prompt_text = None
    if save_prompt:
        prompt_text = mutator.build_prompt(llm_seed_code, target)

    # LLM mutation (shared lock to avoid GPU/ollama contention)
    try:
        with SimpleFileLock(lock_path, timeout=float(lock_timeout), stale_after=float(lock_stale_after)):
            mutated_code = mutator.mutate(llm_seed_code, target)
    except TimeoutError:
        logging.warning("[%s] LLM lock timeout, skip", case_id)
        return None
    except Exception as e:
        logging.error("[%s] LLM mutation failed: %s", case_id, e)
        return None

    if not mutated_code:
        logging.info("[%s] Empty/NO_MUTATION output, skipped", case_id)
        return None

    if mutated_code.strip() == seed_code.strip():
        logging.info("[%s] No-op mutation (identical to seed), skipped", case_id)
        return None

    mutated_temp = results_root / f"temp_{case_id}_mutated.rs"
    mut_variants = compile_variants(mutated_code, mutated_temp, config, timeout_sec)
    mut_result = worst_result(mut_variants)
    base_result = worst_result(base_variants)

    category = classify_status(mut_result.status, base_result.status)
    persist_case(
        results_root=results_root,
        category=category,
        case_id=case_id,
        operator_key=mutator.meta.key,
        operator_name=mutator.meta.name,
        target=target,
        seed_path=seed_path,
        before_code=seed_code,
        after_code=mutated_code,
        base_variants=base_variants,
        mut_variants=mut_variants,
        prompt_text=prompt_text,
    )

    logging.info(
        "[%s] [%s] target=(type=%s, trait=%s) %s -> %s",
        case_id,
        mutator.meta.key,
        target.type_name,
        target.trait_name or "None",
        base_result.status.value,
        mut_result.status.value,
    )
    return CrossResult(
        seed=seed_path,
        case_id=case_id,
        operator_key=mutator.meta.key,
        status=category,
        baseline_status=base_result.status.value.lower(),
        compile_status=mut_result.status,
        compile_duration=mut_result.duration,
    )


def load_configs(repo_root: Path, base_config_arg: str, cross_config_arg: str) -> Tuple[Dict, Dict]:
    base_path = Path(base_config_arg)
    if not base_path.is_absolute():
        base_path = repo_root / base_path

    cross_path = Path(cross_config_arg)
    if not cross_path.is_absolute():
        cross_path = repo_root / cross_path

    base_cfg = load_config(base_path)
    cross_cfg = load_config(cross_path)
    return base_cfg, cross_cfg


def main() -> None:
    args = parse_args()
    setup_logging(args.log_level)

    repo_root = Path(__file__).resolve().parents[1]
    config, cross_cfg = load_configs(repo_root, args.config, args.cross_config)

    cross_paths = cross_cfg.get("paths", {})
    seeds_dir_value = args.seeds_dir or cross_paths.get("seeds") or config["paths"]["seeds"]
    seeds_dir = Path(seeds_dir_value)
    if not seeds_dir.is_absolute():
        seeds_dir = repo_root / seeds_dir

    default_results_dir = cross_cfg.get("paths", {}).get("results", "mutation_crossfeature/results")
    results_root = Path(args.results_dir) if args.results_dir else Path(default_results_dir)
    if not results_root.is_absolute():
        results_root = repo_root / results_root
    results_root.mkdir(parents=True, exist_ok=True)

    timeout_sec = int(args.timeout) if args.timeout is not None else int(config["fuzzer"]["max_time_per_case_sec"])

    connector = LLMConnector(config)
    mutator_map = build_mutators(connector)
    trait_rewriter = TraitRewriterAgent(connector)

    enabled_operator_keys = resolve_enabled_operators(args, cross_cfg)
    valid_keys = [k for k in enabled_operator_keys if k in mutator_map]
    invalid_keys = [k for k in enabled_operator_keys if k not in mutator_map]

    if invalid_keys:
        logging.warning("Unknown operator keys ignored: %s", invalid_keys)
    if not valid_keys:
        logging.error("No valid operators selected. Available: %s", sorted(mutator_map.keys()))
        return

    llm_lock = config.get("llm", {}).get("lock_path", "llm_global_lock_smoke.dir")
    lock_path = Path(llm_lock)
    if not lock_path.is_absolute():
        lock_path = repo_root / lock_path

    lock_timeout = int(cross_cfg.get("run", {}).get("llm_lock_timeout_sec", 180))
    lock_stale_after = float(cross_cfg.get("run", {}).get("llm_lock_stale_after_sec", 900))
    save_prompt = bool(cross_cfg.get("run", {}).get("save_prompt", False))
    notip = bool(cross_cfg.get("run", {}).get("notip", False))
    rewrite_enabled = bool(cross_cfg.get("run", {}).get("enable_trait_rewrite", False))
    rewrite_min_chars = int(cross_cfg.get("run", {}).get("rewrite_min_chars", 10))

    limit = args.limit
    if limit is None:
        limit = int(cross_cfg.get("run", {}).get("limit", 0))
    if limit is None:
        limit = 0
    limit = int(limit)

    cfg_run = cross_cfg.get("run", {})
    seed_selection = args.seed_selection.strip().lower() if args.seed_selection else str(
        cfg_run.get("seed_selection_strategy", "random")
    ).strip().lower()
    shuffle_seed = args.shuffle_seed if args.shuffle_seed is not None else cfg_run.get("shuffle_seed")
    if shuffle_seed is not None:
        try:
            shuffle_seed = int(shuffle_seed)
        except Exception:
            shuffle_seed = None

    seeds = collect_seeds(
        seeds_dir,
        limit=limit,
        strategy=seed_selection,
        shuffle_seed=shuffle_seed,
    )
    if not seeds:
        logging.warning("No seeds found in %s", seeds_dir)
        return

    logging.info("Loaded %d seeds from %s", len(seeds), seeds_dir)
    logging.info("Seed selection: strategy=%s shuffle_seed=%s", seed_selection, shuffle_seed)
    logging.info("LLM notip (strip comments): %s", notip)
    logging.info("Trait rewrite mode: %s", rewrite_enabled)
    logging.info("Enabled operators: %s", valid_keys)
    require_trait_ops = set(
        cross_cfg.get(
            "run",
            {},
        ).get(
            "require_trait_ops",
            ["lifetime_2"],
        )
    )
    logging.info("Operator guards: require trait for %s", sorted(require_trait_ops))
    pass_multiplier = 2 if rewrite_enabled else 1
    total_jobs = len(seeds) * len(valid_keys) * pass_multiplier
    logging.info("Planned jobs: %d seeds x %d operators = %d", len(seeds), len(valid_keys), total_jobs)

    stats = {"success": 0, "error": 0, "hang": 0, "crash": 0, "fate": 0, "unknown": 0}
    per_operator_stats: Dict[str, Dict[str, int]] = {k: dict(stats) for k in valid_keys}
    skipped_by_guard: Dict[str, int] = {k: 0 for k in valid_keys}
    completed_jobs = 0

    for idx, seed in enumerate(seeds):
        logging.info("[seed %d/%d] start %s", idx + 1, len(seeds), seed)
        try:
            seed_code = seed.read_text(encoding="utf-8", errors="ignore")
        except Exception as e:
            logging.error("[seed=%s] failed to read: %s", seed, e)
            continue

        fuzz_passes: List[Tuple[str, str, Path]] = [("original", seed_code, seed)]
        if rewrite_enabled:
            try:
                with SimpleFileLock(lock_path, timeout=float(lock_timeout), stale_after=float(lock_stale_after)):
                    rewritten_code = trait_rewriter.rewrite(seed_code)
                if rewritten_code and len(rewritten_code.strip()) >= rewrite_min_chars:
                    rewrite_dir = results_root / "LLM" / "rewrites"
                    rewrite_dir.mkdir(parents=True, exist_ok=True)
                    rewrite_path = rewrite_dir / f"cross_rewrite_iter_{idx + 1}_{seed.stem}.rs"
                    rewrite_path.write_text(rewritten_code, encoding="utf-8", errors="ignore")
                    fuzz_passes.append(("rewritten", rewritten_code, rewrite_path))
                    logging.info("[seed %d/%d] rewrite pass enabled: %s", idx + 1, len(seeds), rewrite_path)
                else:
                    logging.info("[seed %d/%d] rewrite skipped: empty/short output", idx + 1, len(seeds))
            except TimeoutError:
                logging.warning("[seed %d/%d] rewrite skipped due to LLM lock timeout", idx + 1, len(seeds))
            except Exception as e:
                logging.warning("[seed %d/%d] rewrite failed: %s", idx + 1, len(seeds), e)

        for pass_name, pass_code, pass_seed_path in fuzz_passes:
            case_baseline_id = f"cross_iter_{idx + 1}_{seed.stem}_{pass_name}_baseline"
            baseline_temp = results_root / f"temp_{case_baseline_id}.rs"
            base_variants = compile_variants(pass_code, baseline_temp, config, timeout_sec)
            target_mode = str(cross_cfg.get("run", {}).get("target_extraction", "ast_first")).strip().lower()
            if target_mode == "regex":
                target = infer_mutation_target(pass_code)
                target_source = "regex"
            else:
                target, target_source = infer_mutation_target_ast_first(pass_code, repo_root)

            logging.info(
                "[seed %d/%d][%s] selected target: type=%s trait=%s source=%s",
                idx + 1,
                len(seeds),
                pass_name,
                target.type_name,
                target.trait_name or "None",
                target_source,
            )

            for op_index, op_key in enumerate(valid_keys):
                logging.info(
                    "[job %d/%d] seed=%d/%d pass=%s operator=%d/%d key=%s",
                    completed_jobs + 1,
                    total_jobs,
                    idx + 1,
                    len(seeds),
                    pass_name,
                    op_index + 1,
                    len(valid_keys),
                    op_key,
                )

                if should_skip_by_target_guard(op_key, target, require_trait_ops):
                    completed_jobs += 1
                    skipped_by_guard[op_key] = skipped_by_guard.get(op_key, 0) + 1
                    logging.info(
                        "[job %d/%d] skipped by target guard: trait is missing for key=%s",
                        completed_jobs,
                        total_jobs,
                        op_key,
                    )
                    continue

                mutator = mutator_map[op_key]
                outcome = run_one_seed_one_operator(
                    index=idx,
                    seed_path=pass_seed_path,
                    seed_code=pass_code,
                    target=target,
                    base_variants=base_variants,
                    mutator=mutator,
                    config=config,
                    timeout_sec=timeout_sec,
                    lock_path=lock_path,
                    lock_timeout=lock_timeout,
                    lock_stale_after=lock_stale_after,
                    results_root=results_root,
                    save_prompt=save_prompt,
                    notip=notip,
                    op_order=op_index,
                    pass_name=pass_name,
                )
                completed_jobs += 1
                if outcome is None:
                    logging.info("[job %d/%d] finished with no output", completed_jobs, total_jobs)
                    continue
                stats[outcome.status] = stats.get(outcome.status, 0) + 1
                per_operator_stats[outcome.operator_key][outcome.status] = (
                    per_operator_stats[outcome.operator_key].get(outcome.status, 0) + 1
                )
                logging.info("[job %d/%d] categorized=%s", completed_jobs, total_jobs, outcome.status)

    summary_path = results_root / "summary.json"
    with open(summary_path, "w", encoding="utf-8") as f:
        json.dump(
            {
                "total_seeds": len(seeds),
                "selected_operators": valid_keys,
                "ignored_operators": invalid_keys,
                "skipped_by_guard": skipped_by_guard,
                "stats": stats,
                "per_operator_stats": per_operator_stats,
                "results_root": str(results_root),
            },
            f,
            indent=2,
        )

    logging.info("Done. Summary: %s", stats)
    logging.info("Summary file: %s", summary_path)


if __name__ == "__main__":
    main()
