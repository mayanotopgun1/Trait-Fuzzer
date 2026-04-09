import subprocess
import tempfile
import os
import signal
import time
from dataclasses import dataclass
from pathlib import Path
from typing import List, Optional


@dataclass
class CompileOutcome:
    status: str
    return_code: Optional[int]
    stdout: str
    stderr: str


def _is_ice(stderr: str, stdout: str = "") -> bool:
    text = f"{stderr or ''}\n{stdout or ''}".lower()
    markers = (
        "internal compiler error",
        "thread 'rustc' panicked",
        "thread 'rust1' panicked",
        "this is a bug in the compiler",
        "please submit a full bug report",
    )
    return any(m in text for m in markers)


def _status_from_process(return_code: Optional[int], stdout: str, stderr: str) -> str:
    if return_code is None:
        return "HANG"
    if _is_ice(stderr, stdout):
        return "CRASH"
    return "SUCCESS" if return_code == 0 else "ERROR"


def _terminate_process_group(pid: int) -> None:
    if pid <= 0:
        return
    try:
        os.killpg(pid, signal.SIGTERM)
    except Exception:
        pass
    time.sleep(0.5)
    try:
        os.killpg(pid, signal.SIGKILL)
    except Exception:
        pass


def _kill_lingering_compiler_workers(source_path: Path, rustc_cmd: List[str]) -> None:
    try:
        uid = os.getuid()
        src_sig = str(source_path)
        compiler_name = Path(str(rustc_cmd[0])).name.lower() if rustc_cmd else ""
        if compiler_name in {"", "python", "python3"}:
            compiler_name = "rustc"

        for proc_dir in Path("/proc").iterdir():
            if not proc_dir.name.isdigit():
                continue
            cmdline_file = proc_dir / "cmdline"
            status_file = proc_dir / "status"
            try:
                raw = cmdline_file.read_bytes()
                if not raw:
                    continue
                cmdline = raw.replace(b"\x00", b" ").decode("utf-8", errors="ignore").lower()
                if src_sig.lower() not in cmdline:
                    continue
                if (
                    "rustc" not in cmdline
                    and "rust1" not in cmdline
                    and "crab1" not in cmdline
                    and "gccrs" not in cmdline
                    and compiler_name not in cmdline
                ):
                    continue

                proc_uid = None
                for line in status_file.read_text(encoding="utf-8", errors="ignore").splitlines():
                    if line.startswith("Uid:"):
                        fields = line.split()
                        if len(fields) >= 2:
                            proc_uid = int(fields[1])
                        break
                if proc_uid is None or proc_uid != uid:
                    continue
                os.kill(int(proc_dir.name), signal.SIGKILL)
            except Exception:
                continue
    except Exception:
        pass


def compile_code(
    code: str,
    rustc_cmd: List[str],
    extra_args: Optional[List[str]] = None,
    timeout_sec: int = 20,
) -> CompileOutcome:
    extra_args = list(extra_args or [])
    tmp_dir_base = "/tmp" if Path("/tmp").exists() else None
    with tempfile.TemporaryDirectory(prefix="traitor_agent_compile_", dir=tmp_dir_base) as td:
        td_path = Path(td)
        src = td_path / "input.rs"
        out_bin = td_path / "out_bin"
        src.write_text(code, encoding="utf-8")

        cmd = [*rustc_cmd, str(src), "-o", str(out_bin), "--out-dir", str(td_path), *extra_args]
        try:
            proc = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                start_new_session=True,
            )
            stdout, stderr = proc.communicate(timeout=timeout_sec)
            return CompileOutcome(
                status=_status_from_process(proc.returncode, stdout, stderr),
                return_code=proc.returncode,
                stdout=stdout,
                stderr=stderr,
            )
        except subprocess.TimeoutExpired:
            try:
                _terminate_process_group(proc.pid)
            except Exception:
                pass
            _kill_lingering_compiler_workers(src, rustc_cmd)
            return CompileOutcome(status="HANG", return_code=None, stdout="", stderr="TimeoutExpired")
        except FileNotFoundError as e:
            return CompileOutcome(status="ERROR", return_code=127, stdout="", stderr=f"CompilerNotFound: {e}")


def is_expected_fail_seed(source_text: str) -> bool:
    text = source_text.lower()
    markers = (
        "//@ compile-fail",
        "//@ check-fail",
        "//@ build-fail",
        "//~ error",
    )
    return any(m in text for m in markers)
