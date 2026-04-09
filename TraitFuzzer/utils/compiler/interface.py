import subprocess
import time
import logging
import tempfile
import os
import signal
from enum import Enum
from pathlib import Path
from dataclasses import dataclass
from typing import Optional, Dict

class CompilationStatus(Enum):
    SUCCESS = "SUCCESS"
    ERROR = "ERROR"
    HANG = "HANG"
    CRASH = "CRASH" # ICE
    UNKNOWN = "UNKNOWN"

@dataclass
class CompilationResult:
    status: CompilationStatus
    return_code: Optional[int]
    stdout: str
    stderr: str
    duration: float

class RustCompiler:
    def __init__(
        self,
        timeout: int = 5,
        rustc_cmd=None,
        default_args=None,
        compiler_kind: Optional[str] = None,
        working_dir: Optional[str] = None,
        env: Optional[Dict[str, str]] = None,
        auto_no_core: bool = False,
    ):
        self.timeout = timeout
        self.logger = logging.getLogger(__name__)
        self.rustc_cmd = list(rustc_cmd) if rustc_cmd else ["rustc"]
        self.default_args = list(default_args) if default_args else []
        self.compiler_kind = compiler_kind or self._detect_compiler_kind(self.rustc_cmd)
        self.command_name = Path(str(self.rustc_cmd[0])).name.lower() if self.rustc_cmd else "rustc"
        self.working_dir = str(working_dir) if working_dir else None
        self.env = dict(env) if env else None
        self.auto_no_core = bool(auto_no_core)

    @staticmethod
    def _detect_compiler_kind(cmd: list) -> str:
        if not cmd:
            return "rustc"
        name = Path(str(cmd[0])).name.lower()
        if name in {"gccrs", "rust1", "crab1"}:
            return "gccrs"
        return "rustc"

    @staticmethod
    def _is_internal_compiler_error(stderr: str, stdout: str = "") -> bool:
        text = f"{stderr or ''}\n{stdout or ''}".lower()
        if not text:
            return False

        ice_markers = (
            "internal compiler error",
            "thread 'rustc' panicked",
            "query stack during panic",
            "we would appreciate a bug report",
            "this is a bug in the compiler",
            "rust1: internal compiler error",
            "gccrs: internal compiler error",
        )
        return any(marker in text for marker in ice_markers)

    def compile(
        self,
        source_path: Path,
        output_path: Optional[Path] = None,
        extra_args: Optional[list] = None,
    ) -> CompilationResult:
        """
        Compiles the given Rust source file.
        """
        source_path = Path(source_path).resolve()
        output_path = Path(output_path).resolve() if output_path is not None else None

        extra_args = list(extra_args) if extra_args else []
        merged_args = [*self.default_args, *extra_args]
        if self.compiler_kind == "gccrs" and self.command_name == "gccrs" and output_path is None:
            if all(flag not in merged_args for flag in ("-c", "-S", "-E")):
                merged_args.append("-c")
        if self.compiler_kind == "gccrs":
            exp_flag = "-frust-incomplete-and-experimental-compiler-do-not-use"
            if exp_flag not in merged_args:
                merged_args.append(exp_flag)

        cmd = [*self.rustc_cmd, str(source_path), *merged_args]
        if output_path:
            cmd.extend(["-o", str(output_path)])
        
        # Add basic flags to speed up check-only builds if we don't need binaries
        # For fuzzing we might want to just check: cmd.extend(["--emit", "metadata"])
        # But to catch code generation bugs we might need full compilation.
        # Let's stick to default for now, maybe add -Z mir-opt-level=0 if needed.
        
        start_time = time.time()
        try:
            # Compile into a temporary directory to avoid polluting the working tree with
            # artifacts like `temp_iter_*`, `libtemp_iter_*.rlib`, `.d`, etc.
            # Using both `cwd` and `--out-dir` keeps most outputs contained.
            with tempfile.TemporaryDirectory(prefix="trait_fuzzer_rustc_") as tmp:
                tmp_dir = Path(tmp)
                compile_source_path = source_path

                if self.compiler_kind == "gccrs" and self.auto_no_core:
                    transformed = self._make_no_core_variant(source_path, tmp_dir)
                    if transformed is not None:
                        compile_source_path = transformed

                cmd = [*self.rustc_cmd, str(compile_source_path), *merged_args]
                if output_path:
                    cmd.extend(["-o", str(output_path)])

                # Only redirect artifacts when the caller doesn't request a specific output path.
                # If `-o` is used, rustc will write that file where requested.
                if output_path is None and self.compiler_kind == "rustc":
                    cmd_with_outdir = cmd + ["--out-dir", str(tmp_dir)]
                else:
                    cmd_with_outdir = cmd

                run_cwd = self.working_dir or str(tmp_dir)
                run_env = os.environ.copy()
                if self.env:
                    run_env.update(self.env)

                process = subprocess.Popen(
                    cmd_with_outdir,
                    cwd=run_cwd,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                    start_new_session=True,
                    env=run_env,
                )
                try:
                    stdout, stderr = process.communicate(timeout=self.timeout)
                except subprocess.TimeoutExpired:
                    self._terminate_process_tree(process.pid)
                    self._kill_lingering_gccrs_workers(source_path)
                    duration = time.time() - start_time
                    self.logger.warning(f"Compilation timed out for {source_path}")
                    return CompilationResult(
                        status=CompilationStatus.HANG,
                        return_code=None,
                        stdout="",
                        stderr="TimeoutExpired",
                        duration=duration
                    )

            duration = time.time() - start_time

            status = CompilationStatus.SUCCESS if process.returncode == 0 else CompilationStatus.ERROR
            
            # Check for Internal Compiler Error (ICE)
            if self._is_internal_compiler_error(stderr, stdout):
                status = CompilationStatus.CRASH

            # Best-effort cleanup for the explicit output_path.
            # Most artifacts are already confined to the temporary directory above.
            if output_path is not None:
                try:
                    output_path = Path(output_path)
                    if output_path.exists():
                        output_path.unlink()
                except Exception as e:
                    self.logger.warning(f"Failed to cleanup output artifact for {source_path}: {e}")

            return CompilationResult(
                status=status,
                return_code=process.returncode,
                stdout=stdout,
                stderr=stderr,
                duration=duration
            )
        except FileNotFoundError as e:
            duration = time.time() - start_time
            self.logger.error(f"Compiler binary not found for {source_path}: {e}")
            return CompilationResult(
                status=CompilationStatus.ERROR,
                return_code=127,
                stdout="",
                stderr=f"CompilerNotFound: {e}",
                duration=duration,
            )
        except Exception as e:
            duration = time.time() - start_time
            self.logger.error(f"Execution failed: {e}")
            return CompilationResult(
                status=CompilationStatus.UNKNOWN,
                return_code=-1,
                stdout="",
                stderr=str(e),
                duration=duration
            )

    def _make_no_core_variant(self, source_path: Path, tmp_dir: Path) -> Optional[Path]:
        try:
            text = source_path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            return None

        if "#![no_core]" in text:
            return None

        no_core_prelude = (
            "#![feature(no_core, lang_items)]\n"
            "#![no_core]\n"
            "\n"
            "#[lang = \"sized\"]\n"
            "pub trait Sized {}\n"
            "\n"
        )

        transformed_path = tmp_dir / source_path.name
        try:
            transformed_path.write_text(no_core_prelude + text, encoding="utf-8")
            return transformed_path
        except Exception:
            return None

    def _terminate_process_tree(self, pid: int):
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

    def _kill_lingering_gccrs_workers(self, source_path: Path):
        if self.compiler_kind != "gccrs":
            return
        try:
            uid = os.getuid()
            src_sig = str(source_path)
            for proc_dir in Path("/proc").iterdir():
                if not proc_dir.name.isdigit():
                    continue
                cmdline_file = proc_dir / "cmdline"
                status_file = proc_dir / "status"
                try:
                    raw = cmdline_file.read_bytes()
                    if not raw:
                        continue
                    cmdline = raw.replace(b"\x00", b" ").decode("utf-8", errors="ignore")
                    if ("rust1" not in cmdline and "crab1" not in cmdline and "gccrs" not in cmdline):
                        continue
                    if src_sig not in cmdline:
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
