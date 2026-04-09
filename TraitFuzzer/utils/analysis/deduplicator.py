import hashlib
import json
import logging
import re
from pathlib import Path
from collections import defaultdict

class Deduplicator:
    def __init__(self, results_dir: Path):
        self.results_dir = results_dir

    _ICE_SIG_RE = re.compile(
        r"internal compiler error:\s*in\s*([^,\n]+),\s*at\s*([^\n]+)",
        re.IGNORECASE,
    )
    _TMP_TRAIT_FUZZER_DIR_RE = re.compile(r"/tmp/trait_fuzzer_rustc_[^/]+/")
    _TEMP_CASE_RS_RE = re.compile(r"temp_w\d+_iter_\d+_[^/:]+\.rs")
    _ADDR_RE = re.compile(r"\b0x[0-9a-fA-F]+\b")

    def _normalize_text(self, text: str) -> str:
        x = str(text or "")
        x = x.replace("\\", "/")
        x = self._TMP_TRAIT_FUZZER_DIR_RE.sub("/tmp/trait_fuzzer_rustc_<tmp>/", x)
        x = self._TEMP_CASE_RS_RE.sub("temp_fuzzer_case.rs", x)
        x = self._ADDR_RE.sub("0x<addr>", x)
        return x

    def _extract_section(self, content: str, header: str) -> str:
        marker = f"=== {header} ==="
        start = content.find(marker)
        if start < 0:
            return ""
        rest = content[start + len(marker):]
        next_marker = rest.find("\n=== ")
        if next_marker >= 0:
            return rest[:next_marker]
        return rest

    def _extract_stderr(self, section_text: str) -> str:
        if "Stderr:" in section_text:
            return section_text.split("Stderr:", 1)[1]
        return section_text

    def _stable_signature(self, content: str, compiler_ns: str, category: str) -> str:
        comp = str(compiler_ns or "").lower()
        cat = str(category or "").lower()

        if comp == "gccrs":
            section = self._extract_section(content, "gccrs")
            stderr = self._extract_stderr(section)
            m = self._ICE_SIG_RE.search(stderr)
            if m and cat == "crash":
                fn_name = m.group(1).strip()
                at_loc = self._normalize_text(m.group(2).strip())
                return f"ICE::{fn_name}@{at_loc}"
            normalized = "".join(self._normalize_text(stderr).split())
            if normalized:
                return normalized

        stderr_part = self._extract_stderr(content)
        normalized = "".join(self._normalize_text(stderr_part).split())
        return normalized

    def deduplicate(self, category="crash", compiler_ns=None):
        """
        Scans the results directory for the given category (crash, error, hang),
        hashes their stderr/output, and groups duplicates.
        """
        base_dir = Path(self.results_dir)
        if compiler_ns:
            category_dir = base_dir / str(compiler_ns) / category
        else:
            category_dir = base_dir / category
        if not category_dir.exists():
            logging.warning(f"Category directory {category_dir} does not exist.")
            return {}

        hashes = defaultdict(list)
        
        for case_dir in category_dir.iterdir():
            if not case_dir.is_dir():
                continue
            
            detail_log = case_dir / "detail.log"
            if not detail_log.exists():
                continue
            
            # Simple deduplication strategy: Hash the Stderr content.
            # In a real compiler fuzzer, we might mask line numbers or temporary variables using regex.
            try:
                content = detail_log.read_text(encoding='utf-8', errors='ignore')
                signature = self._stable_signature(content, str(compiler_ns or ""), category)
                if not signature:
                    continue
                file_hash = hashlib.md5(signature.encode('utf-8')).hexdigest()
                hashes[file_hash].append(case_dir.name)
                
            except Exception as e:
                logging.error(f"Failed to process {case_dir}: {e}")

        return hashes

    def save_report(self, unique_bugs, output_file="deduplication_report.json"):
        """
        Saves the deduplication report.
        """
        report = {
            "total_unique": len(unique_bugs),
            "groups": unique_bugs
        }
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=4)
        
        print(f"Deduplication complete. Found {len(unique_bugs)} unique signatures.")

if __name__ == "__main__":
    # Test run
    logging.basicConfig(level=logging.INFO)
    deduper = Deduplicator(Path("trait-fuzzer/results"))
    unique = deduper.deduplicate("crash", compiler_ns="gccrs")
    deduper.save_report(unique)
