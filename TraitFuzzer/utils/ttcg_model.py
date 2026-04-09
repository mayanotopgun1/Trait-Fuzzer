import json
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Optional


@dataclass(frozen=True)
class TTCGComplexity:
    extra: Dict[str, int]


class TTCGModel:
    """Unified TTCG interface for the Python driver.

    Source of truth is the Rust `syn`-based extractor in mutation-AST (crate::ttcg).
    We call it via `cargo run` in `--mode ttcg_metrics` and parse the JSON output.
    """

    def __init__(self, mutation_ast_dir: Optional[Path] = None):
        self.mutation_ast_dir = Path(mutation_ast_dir) if mutation_ast_dir is not None else Path("mutation/mutation-AST")

    def calculate_complexity_for_file(self, rust_file: Path, timeout_sec: int = 20) -> TTCGComplexity:
        rust_file = Path(rust_file)
        # Write to a temp output path (content is irrelevant for metrics mode).
        out_path = rust_file.with_suffix(rust_file.suffix + ".ttcg_out")

        bin_path = self.mutation_ast_dir / "target" / "debug" / "mutation-ast"
        if bin_path.exists():
            cmd = [
                str(bin_path.absolute()),
                "--input",
                str(rust_file.absolute()),
                "--output",
                str(out_path.absolute()),
                "--mode",
                "ttcg_metrics",
            ]
        else:
            cmd = [
                "cargo",
                "run",
                "--quiet",
                "--",
                "--input",
                str(rust_file.absolute()),
                "--output",
                str(out_path.absolute()),
                "--mode",
                "ttcg_metrics",
            ]

        try:
            proc = subprocess.run(
                cmd,
                cwd=str(self.mutation_ast_dir.absolute()),
                capture_output=True,
                text=True,
                timeout=timeout_sec,
                check=True,
            )

            payload = json.loads(proc.stdout.strip() or "{}")
            extra = {k: int(v) for k, v in payload.items() if isinstance(v, int)}
            return TTCGComplexity(extra=extra)
        except Exception:
            # Minimal fallback: if metrics extraction fails, return a neutral score.
            return TTCGComplexity(extra={})
        finally:
            try:
                if out_path.exists():
                    out_path.unlink()
            except Exception:
                pass
