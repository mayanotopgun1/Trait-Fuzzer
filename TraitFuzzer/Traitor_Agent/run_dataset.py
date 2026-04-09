import argparse
import json
import random
import shutil
import sys
from pathlib import Path
from typing import Dict, List


PROJECT_ROOT = Path(__file__).resolve().parents[1]
if str(PROJECT_ROOT) not in sys.path:
    sys.path.append(str(PROJECT_ROOT))

from Traitor_Agent.agent import TraitorAgent, load_full_config
from Traitor_Agent.compiler_utils import compile_code


def _load_cfg(path: Path) -> Dict:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def _collect_rs_files(root: Path) -> List[Path]:
    return sorted([p for p in root.rglob("*.rs") if p.is_file() and p.stat().st_size > 0])


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Batch rewrite dataset with TraitorAgent")
    p.add_argument("--config", default="Traitor_Agent/Traitorconfig.json", help="Traitor config path")
    p.add_argument("--max-cases", type=int, default=None, help="Override max cases")
    p.add_argument("--input-dataset", default="", help="Override input dataset path")
    p.add_argument("--output-dir", default="", help="Override output directory path")
    return p.parse_args()


def main() -> int:
    args = parse_args()
    cfg_path = Path(args.config)
    if not cfg_path.is_absolute():
        cfg_path = (PROJECT_ROOT / cfg_path).resolve()

    cfg = _load_cfg(cfg_path)
    paths = cfg.get("paths", {})
    runtime = cfg.get("runtime", {})

    input_dataset = args.input_dataset or paths.get("input_dataset", "seeds")
    output_dir = args.output_dir or paths.get("output_dir", "Traitor_Agent/trait_dense_seeds")
    summary_jsonl = paths.get("summary_jsonl", "Traitor_Agent/run_summary.jsonl")
    surprise_dir_cfg = paths.get("surprise_dir", "Traitor_Agent/surprise")

    input_root = Path(input_dataset)
    if not input_root.is_absolute():
        input_root = (PROJECT_ROOT / input_root).resolve()
    output_root = Path(output_dir)
    if not output_root.is_absolute():
        output_root = (PROJECT_ROOT / output_root).resolve()
    summary_path = Path(summary_jsonl)
    if not summary_path.is_absolute():
        summary_path = (PROJECT_ROOT / summary_path).resolve()
    surprise_root = Path(surprise_dir_cfg)
    if not surprise_root.is_absolute():
        surprise_root = (PROJECT_ROOT / surprise_root).resolve()

    # Keep trait_dense_seeds clean: summary must live outside output directory.
    if str(summary_path).startswith(str(output_root) + "/") or summary_path == output_root:
        summary_path = (PROJECT_ROOT / "Traitor_Agent" / "run_summary.jsonl").resolve()

    if output_root.exists():
        shutil.rmtree(output_root)
    output_root.mkdir(parents=True, exist_ok=True)
    summary_path.parent.mkdir(parents=True, exist_ok=True)
    surprise_root.mkdir(parents=True, exist_ok=True)
    if summary_path.exists():
        summary_path.unlink()

    total_path = PROJECT_ROOT / "Traitor_Agent" / "total"
    total_update_every = int(runtime.get("total_update_every", 20))
    if total_update_every <= 0:
        total_update_every = 20

    files = _collect_rs_files(input_root)
    if not files:
        print(json.dumps({"status": "empty", "input_dataset": str(input_root)}))
        return 0

    if runtime.get("shuffle", True):
        seed = int(runtime.get("seed", 42))
        random.Random(seed).shuffle(files)

    limit_cfg = int(runtime.get("max_cases", 0))
    limit = args.max_cases if args.max_cases is not None else limit_cfg
    if limit and limit > 0:
        files = files[:limit]

    total_original = len(files)

    # TraitorAgent expects llm + traitor_agent + compiler config in one dict.
    full_cfg = {
        "llm": cfg.get("llm", {}),
        "traitor_agent": cfg.get("traitor_agent", {}),
        "compiler": cfg.get("compiler", {}),
    }
    agent = TraitorAgent(full_cfg, project_root=PROJECT_ROOT)
    compile_basis_cmd = list(agent.compile_cmd)

    original_compile_success = 0
    original_compile_fail = 0
    original_crash_or_hang = 0

    stage1_generated = 0
    stage1_dead_structure_complexity = 0
    stage1_dead_error = 0

    stage2_generated = 0
    stage2_dead_error = 0
    stage2_attempted_total = 0
    stage2_compile_pass = 0
    stage2_compile_fail = 0
    surprise_cases = 0

    feature_selected_counts: Dict[str, int] = {}

    processed = 0
    written_counter = 0

    def _current_total(status: str) -> Dict:
        return {
            "status": status,
            "input_dataset": str(input_root),
            "output_dir": str(output_root),
            "summary_jsonl": str(summary_path),
            "compile_basis_cmd": compile_basis_cmd,
            "total_original_seeds": total_original,
            "original_compile_success": original_compile_success,
            "original_compile_fail": original_compile_fail,
            "original_crash_or_hang": original_crash_or_hang,
            "processed_after_filter": processed,
            "stage1_generated_seeds": stage1_generated,
            "stage1_dead_by_structure_complexity": stage1_dead_structure_complexity,
            "stage1_dead_by_tsa": stage1_dead_structure_complexity,
            "stage1_dead_by_error": stage1_dead_error,
            "stage2_generated_seeds": stage2_generated,
            "stage2_dead_by_error": stage2_dead_error,
            "stage2_attempted_total": stage2_attempted_total,
            "stage2_compile_pass": stage2_compile_pass,
            "stage2_compile_fail": stage2_compile_fail,
            "stage2_feature_selected_counts": feature_selected_counts,
            "surprise_cases": surprise_cases,
        }

    def _flush_total(status: str) -> None:
        total_obj = _current_total(status)
        total_path.write_text(json.dumps(total_obj, ensure_ascii=False, indent=2), encoding="utf-8")

    _flush_total("running")

    interrupted = False
    for idx, src in enumerate(files, start=1):
        print(f"[run_dataset] seed {idx}/{len(files)}: {src}", flush=True)
        rel = src.relative_to(input_root)

        try:
            code = src.read_text(encoding="utf-8", errors="ignore")
        except KeyboardInterrupt:
            interrupted = True
            print("[run_dataset] interrupted while reading seed", flush=True)
            break
        original_outcome = compile_code(
            code=code,
            rustc_cmd=agent.compile_cmd,
            extra_args=agent.compile_extra_args,
            timeout_sec=agent.compile_timeout_sec,
        )
        if original_outcome.status != "SUCCESS":
            original_compile_fail += 1
            if original_outcome.status in ("CRASH", "HANG"):
                original_crash_or_hang += 1
            print(f"[run_dataset] skip original compile status={original_outcome.status}", flush=True)
            continue
        original_compile_success += 1
        processed += 1

        try:
            res = agent.transform(code)
        except KeyboardInterrupt:
            interrupted = True
            print("[run_dataset] interrupted during agent transform", flush=True)
            break

        if res.stage1_compile_status != "SUCCESS":
            stage1_dead_error += 1
        elif res.stage1_eligible_for_pool:
            stage1_generated += 1
        else:
            stage1_dead_structure_complexity += 1

        if res.stage2_attempted and res.selected_feature:
            feature_selected_counts[res.selected_feature] = feature_selected_counts.get(res.selected_feature, 0) + 1

        if res.stage2_attempted:
            stage2_attempted_total += 1
            if res.stage2_compile_status == "SUCCESS":
                stage2_compile_pass += 1
            else:
                stage2_compile_fail += 1
                stage2_dead_error += 1

        written_stages: List[str] = []
        output_paths: List[str] = []
        stage2_duplicate_of_stage1 = False

        if res.stage1_eligible_for_pool:
            written_counter += 1
            out_stage1 = output_root / f"seed_{written_counter:06d}.rs"
            out_stage1.write_text(res.stage1_output_code, encoding="utf-8")
            output_paths.append(str(out_stage1))
            written_stages.append("stage1")
            print("[run_dataset] stage1 structure_complexity+SUCCESS -> written", flush=True)

        if res.eligible_for_seed and res.stage2_attempted:
            stage2_duplicate_of_stage1 = (
                res.stage1_eligible_for_pool
                and (res.output_code or "").strip() == (res.stage1_output_code or "").strip()
            )
            if stage2_duplicate_of_stage1:
                print("[run_dataset] stage2 SUCCESS but duplicate of stage1 -> skip duplicate write", flush=True)
            else:
                written_counter += 1
                out_stage2 = output_root / f"seed_{written_counter:06d}.rs"
                out_stage2.write_text(res.output_code, encoding="utf-8")
                output_paths.append(str(out_stage2))
                written_stages.append("stage2")
                stage2_generated += 1
                print("[run_dataset] stage2 SUCCESS -> written", flush=True)

        if not output_paths:
            if res.stage2_attempted:
                print(f"[run_dataset] stage2 failed status={res.stage2_compile_status}", flush=True)
            else:
                print("[run_dataset] stage2 skipped", flush=True)

        output_path = output_paths[0] if output_paths else ""
        written_from_stage = written_stages[0] if written_stages else ""

        surprise_triggered = (
            res.stage1_compile_status in ("CRASH", "HANG")
            or res.stage2_compile_status in ("CRASH", "HANG")
        )
        surprise_path = ""
        if surprise_triggered:
            surprise_cases += 1
            case_dir = surprise_root / f"case_{idx:06d}_{src.stem}"
            case_dir.mkdir(parents=True, exist_ok=True)
            before_path = case_dir / "before.rs"
            after_path = case_dir / "after.rs"
            meta_path = case_dir / "meta.json"

            before_path.write_text(code, encoding="utf-8")
            after_variant = res.output_code if res.stage2_attempted else res.stage1_output_code
            after_path.write_text(after_variant or "", encoding="utf-8")
            surprise_meta = {
                "index": idx,
                "input": str(src),
                "stage": res.stage,
                "selected_feature": res.selected_feature,
                "applied_feature": res.applied_feature,
                "candidate_features": res.candidate_features,
                "feature_applied": res.feature_applied,
                "stage1_compile_status": res.stage1_compile_status,
                "stage2_compile_status": res.stage2_compile_status,
                "stage2_attempted": res.stage2_attempted,
                "final_compile_status": res.final_compile_status,
                "structure_complexity_original": res.structure_complexity_original,
                "stage1_structure_complexity": res.stage1_structure_complexity,
                "stage2_structure_complexity": res.stage2_structure_complexity,
                "delta_structure_complexity": res.delta_structure_complexity,
                "stage1_accepted_by_structure_complexity": res.stage1_accepted_by_structure_complexity,
                "stage2_accepted_by_structure_complexity": res.stage2_accepted_by_structure_complexity,
                "stage1_eligible_for_pool": res.stage1_eligible_for_pool,
                "stage2_eligible_for_pool": res.stage2_eligible_for_pool,
                "stage2_duplicate_of_stage1": stage2_duplicate_of_stage1,
                "written_from_stage": written_from_stage,
                "written_from_stages": written_stages,
                "summary_output": output_path,
                "summary_outputs": output_paths,
            }
            meta_path.write_text(json.dumps(surprise_meta, ensure_ascii=False, indent=2), encoding="utf-8")
            surprise_path = str(case_dir)
            print(f"[run_dataset] surprise captured: {case_dir}", flush=True)

        row = {
            "index": idx,
            "input": str(src),
            "output": output_path,
            "written_from_stage": written_from_stage,
            "stage": res.stage,
            "structure_complexity_original": res.structure_complexity_original,
            "structure_complexity_output": res.structure_complexity_output,
            "stage1_structure_complexity": res.stage1_structure_complexity,
            "stage2_structure_complexity": res.stage2_structure_complexity,
            "delta_structure_complexity": res.delta_structure_complexity,
            "accepted_by_structure_complexity": res.accepted_by_structure_complexity,
            "stage1_accepted_by_structure_complexity": res.stage1_accepted_by_structure_complexity,
            "stage2_accepted_by_structure_complexity": res.stage2_accepted_by_structure_complexity,
            "structure_complexity_method": res.structure_complexity_method,
            "structure_complexity_compiler_mode": res.structure_complexity_compiler_mode,
            "expected_fail_seed": res.expected_fail_seed,
            "selected_feature": res.selected_feature,
            "applied_feature": res.applied_feature,
            "candidate_features": res.candidate_features,
            "feature_applied": res.feature_applied,
            "stage1_compile_status": res.stage1_compile_status,
            "stage2_compile_status": res.stage2_compile_status,
            "stage2_attempted": res.stage2_attempted,
            "final_compile_status": res.final_compile_status,
            "eligible_for_seed": res.eligible_for_seed,
            "stage1_eligible_for_pool": res.stage1_eligible_for_pool,
            "stage2_eligible_for_pool": res.stage2_eligible_for_pool,
            "stage2_duplicate_of_stage1": stage2_duplicate_of_stage1,
            "surprise": surprise_triggered,
            "surprise_path": surprise_path,
            "outputs": output_paths,
            "written_from_stages": written_stages,
        }
        row.update(
            {
                "tsa_original": res.structure_complexity_original,
                "tsa_output": res.structure_complexity_output,
                "stage1_tsa": res.stage1_structure_complexity,
                "stage2_tsa": res.stage2_structure_complexity,
                "delta_tsa": res.delta_structure_complexity,
                "accepted_by_tsa": res.accepted_by_structure_complexity,
                "stage1_accepted_by_tsa": res.stage1_accepted_by_structure_complexity,
                "stage2_accepted_by_tsa": res.stage2_accepted_by_structure_complexity,
                "tsa_method": res.structure_complexity_method,
                "tsa_compiler_mode": res.structure_complexity_compiler_mode,
            }
        )
        with open(summary_path, "a", encoding="utf-8") as f:
            f.write(json.dumps(row, ensure_ascii=False) + "\n")

        if idx % total_update_every == 0:
            _flush_total("running")

    _flush_total("interrupted" if interrupted else "ok")

    result = {
        "status": "interrupted" if interrupted else "ok",
        "input_dataset": str(input_root),
        "output_dir": str(output_root),
        "summary": str(summary_path),
        "total": str(total_path),
        "compile_basis_cmd": compile_basis_cmd,
        "total_original_seeds": total_original,
        "original_compile_success": original_compile_success,
        "original_compile_fail": original_compile_fail,
        "original_crash_or_hang": original_crash_or_hang,
        "stage1_generated_seeds": stage1_generated,
        "stage1_dead_by_structure_complexity": stage1_dead_structure_complexity,
        "stage1_dead_by_tsa": stage1_dead_structure_complexity,
        "stage1_dead_by_error": stage1_dead_error,
        "stage2_generated_seeds": stage2_generated,
        "stage2_dead_by_error": stage2_dead_error,
        "stage2_attempted_total": stage2_attempted_total,
        "stage2_compile_pass": stage2_compile_pass,
        "stage2_compile_fail": stage2_compile_fail,
        "stage2_feature_selected_counts": feature_selected_counts,
        "surprise_cases": surprise_cases,
        "surprise_dir": str(surprise_root),
    }
    print(json.dumps(result, ensure_ascii=False))
    return 130 if interrupted else 0


if __name__ == "__main__":
    raise SystemExit(main())
