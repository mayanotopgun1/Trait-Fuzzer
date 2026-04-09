import tempfile
from pathlib import Path
from typing import List, Optional

from utils.ttcg_model import TTCGModel


class StructureComplexityMeter:
    def __init__(
        self,
        rustc_cmd: List[str],
        project_root: Optional[Path] = None,
        use_script: bool = True,
        compiler_mode: int = 1,
        extra_args: Optional[List[str]] = None,
        timeout_sec: int = 20,
    ):
        self.rustc_cmd = list(rustc_cmd)
        self.project_root = Path(project_root) if project_root else Path(__file__).resolve().parents[1]
        self.use_script = bool(use_script)
        self.compiler_mode = int(compiler_mode)
        self.extra_args = list(extra_args or [])
        self.timeout_sec = int(timeout_sec)
        # Keep constructor args for compatibility, but metric source is TTCG.
        self.script_path = self._resolve_script_path()
        self._ttcg = TTCGModel(self.project_root / "mutation" / "mutation-AST")

    def _resolve_script_path(self) -> Path:
        return self.project_root / "utils" / "structure_complexity" / "trait_query_stats.sh"

    def _measure_via_ttcg(self, code: str) -> dict:
        try:
            with tempfile.TemporaryDirectory(prefix="structure_complexity_tcs_") as td:
                td_path = Path(td)
                src = td_path / "input.rs"
                src.write_text(code, encoding="utf-8")
                c = self._ttcg.calculate_complexity_for_file(src, timeout_sec=self.timeout_sec)
                score = int(c.extra.get("constraint_choice_sum", 0))
                return {
                    "score": score,
                    "method": "tcs_constraint_choice_sum",
                    "constraint_choice_sum": score,
                    "constraint_sites": int(c.extra.get("constraint_sites", 0)),
                }
        except Exception:
            return {
                "score": 0,
                "method": "tcs_failed",
                "constraint_choice_sum": 0,
                "constraint_sites": 0,
            }

    def measure_with_details(self, code: str) -> dict:
        return self._measure_via_ttcg(code)

    def measure(self, code: str) -> int:
        return int(self.measure_with_details(code).get("score", 0))
