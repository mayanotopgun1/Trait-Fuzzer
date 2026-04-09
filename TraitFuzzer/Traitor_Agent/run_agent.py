import argparse
import json
import sys
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parents[1]
if str(PROJECT_ROOT) not in sys.path:
    sys.path.append(str(PROJECT_ROOT))

from Traitor_Agent.agent import TraitorAgent, load_full_config


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Run Traitor two-stage Rust transformation agent")
    p.add_argument("--input", required=True, help="Input Rust file path")
    p.add_argument("--output", required=True, help="Output Rust file path")
    p.add_argument("--config", default="config.json", help="Project config file path")
    p.add_argument("--summary", default="", help="Optional summary json output path")
    return p.parse_args()


def main() -> int:
    args = parse_args()
    cfg_path = Path(args.config)
    if not cfg_path.is_absolute():
        cfg_path = PROJECT_ROOT / cfg_path

    inp = Path(args.input)
    if not inp.is_absolute():
        inp = (PROJECT_ROOT / inp).resolve()
    out = Path(args.output)
    if not out.is_absolute():
        out = (PROJECT_ROOT / out).resolve()

    full_cfg = load_full_config(cfg_path)
    agent = TraitorAgent(full_cfg, project_root=PROJECT_ROOT)

    src = inp.read_text(encoding="utf-8", errors="ignore")
    res = agent.transform(src)

    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(res.output_code, encoding="utf-8")

    summary = {
        "stage1_output_present": bool((res.stage1_output_code or "").strip()),
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
        "input": str(inp),
        "output": str(out),
    }

    summary.update(
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

    if args.summary:
        sm = Path(args.summary)
        if not sm.is_absolute():
            sm = (PROJECT_ROOT / sm).resolve()
        sm.parent.mkdir(parents=True, exist_ok=True)
        sm.write_text(json.dumps(summary, ensure_ascii=False, indent=2), encoding="utf-8")

    print(json.dumps(summary, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
