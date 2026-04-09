import json
import random
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List


@dataclass
class ShotExample:
    input_code: str
    output_code: str
    source: str


class FewShotPool:
    def __init__(self, base_dir: Path):
        self.base_dir = Path(base_dir)
        self.stage1_baseline_file = self.base_dir / "stage1_baseline.jsonl"
        self.stage1_experience_file = self.base_dir / "stage1_experience.jsonl"
        self.stage2_generic_file = self.base_dir / "stage2_generic.jsonl"
        self.stage2_feature_files: Dict[str, Path] = {
            "GAT": self.base_dir / "stage2_gat.jsonl",
            "specialization": self.base_dir / "stage2_specialization.jsonl",
            "RPITIT": self.base_dir / "stage2_rpitit.jsonl",
            "TAIT": self.base_dir / "stage2_tait.jsonl",
            "const_trait": self.base_dir / "stage2_const_trait.jsonl",
        }

        # Backward-compatible fallback files
        self.legacy_baseline_file = self.base_dir / "baseline.jsonl"
        self.legacy_experience_file = self.base_dir / "experience.jsonl"

    def _load_jsonl(self, fp: Path, source: str) -> List[ShotExample]:
        if not fp or (not fp.exists()) or (not fp.is_file()):
            return []
        out: List[ShotExample] = []
        for line in fp.read_text(encoding="utf-8", errors="ignore").splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                obj = json.loads(line)
                in_code = str(obj.get("input") or obj.get("before") or "").strip()
                out_code = str(obj.get("output") or obj.get("after") or "").strip()
                if in_code and out_code:
                    out.append(ShotExample(input_code=in_code, output_code=out_code, source=source))
            except Exception:
                continue
        return out

    def _pick(self, rows: List[ShotExample], k: int) -> List[ShotExample]:
        if not rows:
            return []
        n = min(len(rows), max(0, int(k)))
        if n <= 0:
            return []
        return random.sample(rows, k=n)

    def sample_stage1(self, baseline_k: int, experience_k: int) -> List[ShotExample]:
        baseline = self._load_jsonl(self.stage1_baseline_file, "stage1_baseline")
        experience = self._load_jsonl(self.stage1_experience_file, "stage1_experience")

        if not baseline and not experience:
            baseline = self._load_jsonl(self.legacy_baseline_file, "legacy_baseline")
            experience = self._load_jsonl(self.legacy_experience_file, "legacy_experience")

        picked: List[ShotExample] = []
        picked.extend(self._pick(baseline, baseline_k))
        picked.extend(self._pick(experience, experience_k))
        random.shuffle(picked)
        return picked

    def sample_stage2(self, feature: str, generic_k: int, feature_k: int) -> List[ShotExample]:
        generic = self._load_jsonl(self.stage2_generic_file, "stage2_generic")
        feature_file = self.stage2_feature_files.get(feature)
        feature_rows = self._load_jsonl(feature_file, f"stage2_{feature.lower()}") if feature_file else []

        # If dedicated stage2 pools are absent, fallback only to legacy experience
        # (never fallback to stage1 baseline examples).
        if not generic and not feature_rows:
            generic = self._load_jsonl(self.legacy_experience_file, "legacy_experience")

        picked: List[ShotExample] = []
        picked.extend(self._pick(generic, generic_k))
        picked.extend(self._pick(feature_rows, feature_k))

        random.shuffle(picked)
        return picked

    def sample(self, baseline_k: int, experience_k: int) -> List[ShotExample]:
        return self.sample_stage1(baseline_k, experience_k)
