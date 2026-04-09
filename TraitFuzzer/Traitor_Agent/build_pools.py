import argparse
import json
import random
from pathlib import Path
from typing import Dict, List


PROJECT_ROOT = Path(__file__).resolve().parents[1]


def _load_cfg(path: Path) -> Dict:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def _collect_pairs(category_dir: Path) -> List[Dict[str, str]]:
    rows: List[Dict[str, str]] = []
    if not category_dir.exists():
        return rows

    for before in sorted(category_dir.rglob("before.rs")):
        case_dir = before.parent
        after = case_dir / "after.rs"
        if not before.exists() or not after.exists():
            continue
        b = before.read_text(encoding="utf-8", errors="ignore").strip()
        a = after.read_text(encoding="utf-8", errors="ignore").strip()
        if not b or not a:
            continue
        rows.append({"before": b, "after": a})
    return rows


def _dedup(rows: List[Dict[str, str]]) -> List[Dict[str, str]]:
    seen = set()
    out = []
    for row in rows:
        key = (row.get("before", ""), row.get("after", ""))
        if key in seen:
            continue
        seen.add(key)
        out.append(row)
    return out


def _write_jsonl(path: Path, rows: List[Dict[str, str]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        for r in rows:
            f.write(json.dumps(r, ensure_ascii=False) + "\n")


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Build Traitor_Agent pools from results")
    p.add_argument("--config", default="Traitor_Agent/Traitorconfig.json")
    return p.parse_args()


def main() -> int:
    args = parse_args()
    cfg_path = Path(args.config)
    if not cfg_path.is_absolute():
        cfg_path = (PROJECT_ROOT / cfg_path).resolve()
    cfg = _load_cfg(cfg_path)

    builder = cfg.get("pool_builder", {})
    max_baseline = int(builder.get("max_baseline", 200))
    max_experience = int(builder.get("max_experience", 200))
    results_dir = Path(builder.get("results_dir", "results"))
    if not results_dir.is_absolute():
        results_dir = (PROJECT_ROOT / results_dir).resolve()

    traitor_cfg = cfg.get("traitor_agent", {})
    pool_dir = Path(
        traitor_cfg.get("fewshot_pool_dir")
        or traitor_cfg.get("pool_dir")
        or "Traitor_Agent/pools"
    )
    if not pool_dir.is_absolute():
        pool_dir = (PROJECT_ROOT / pool_dir).resolve()

    baseline_rows: List[Dict[str, str]] = []
    experience_rows: List[Dict[str, str]] = []

    for compiler_ns in ("rustc", "gccrs"):
        base = results_dir / compiler_ns
        baseline_rows.extend(_collect_pairs(base / "success"))
        baseline_rows.extend(_collect_pairs(base / "error"))
        experience_rows.extend(_collect_pairs(base / "crash"))
        experience_rows.extend(_collect_pairs(base / "fate"))
        experience_rows.extend(_collect_pairs(base / "hang"))
        experience_rows.extend(_collect_pairs(base / "rewrite"))

    baseline_rows = _dedup(baseline_rows)
    experience_rows = _dedup(experience_rows)

    rng = random.Random(42)
    rng.shuffle(baseline_rows)
    rng.shuffle(experience_rows)

    baseline_rows = baseline_rows[:max_baseline]
    experience_rows = experience_rows[:max_experience]

    baseline_path = pool_dir / "baseline.jsonl"
    experience_path = pool_dir / "experience.jsonl"
    _write_jsonl(baseline_path, baseline_rows)
    _write_jsonl(experience_path, experience_rows)

    print(
        json.dumps(
            {
                "status": "ok",
                "results_dir": str(results_dir),
                "baseline": str(baseline_path),
                "experience": str(experience_path),
                "baseline_count": len(baseline_rows),
                "experience_count": len(experience_rows),
            },
            ensure_ascii=False,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
