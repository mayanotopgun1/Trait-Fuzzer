#!/usr/bin/env bash
# Note: treereduce passes the candidate filename as the first argument ().
export LANG="C"
export LC_ALL="C"
export LANGUAGE="en_US:en"

INPUT_FILE="${1:-bug.rs}"
MODE="${FUZZ_MODE:-0}"
COMPILER_FLAG="${FUZZ_COMPILER_FLAG:-1}"
HANG_TIMEOUT="${FUZZ_HANG_TIMEOUT:-5}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
GCCRS_BIN="${FUZZ_GCCRS_BIN:-$ROOT_DIR/gccrs/build/gcc/crab1}"
GCCRS_WORK_DIR="${FUZZ_GCCRS_WORK_DIR:-}"
GCCRS_AUTO_NO_CORE="${FUZZ_GCCRS_AUTO_NO_CORE:-1}"

compiler_cmd=("rustc" "+nightly")
case "$COMPILER_FLAG" in
  0)
    compiler_cmd=("rustc")
    ;;
  1)
    compiler_cmd=("rustc" "+nightly")
    ;;
  2)
    compiler_cmd=("rustc" "+nightly" "-Z" "next-solver=globally")
    ;;
  3)
    compiler_cmd=("$GCCRS_BIN" "-frust-incomplete-and-experimental-compiler-do-not-use")
    ;;
esac

if ! [[ "$HANG_TIMEOUT" =~ ^[0-9]+$ ]] || [[ "$HANG_TIMEOUT" -le 0 ]]; then
  HANG_TIMEOUT=5
fi

work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT
cp "$INPUT_FILE" "$work_dir/bug.rs"

if [[ "$COMPILER_FLAG" == "3" && "$GCCRS_AUTO_NO_CORE" == "1" ]]; then
  if ! grep -q '^#!\[no_core\]' "$work_dir/bug.rs"; then
    {
      printf '%s\n' '#![feature(no_core,lang_items)]'
      printf '%s\n' '#![no_core]'
      printf '%s\n' ''
      printf '%s\n' '#[lang = "sized"]'
      printf '%s\n' 'pub trait Sized {}'
      printf '%s\n' ''
      cat "$work_dir/bug.rs"
    } >"$work_dir/bug.rs.tmp"
    mv "$work_dir/bug.rs.tmp" "$work_dir/bug.rs"
  fi
fi

case $MODE in
  0) # Hang check
    set +e
    if [[ "$COMPILER_FLAG" == "3" && -n "$GCCRS_WORK_DIR" ]]; then
      (cd "$GCCRS_WORK_DIR" && timeout --preserve-status "${HANG_TIMEOUT}s" "${compiler_cmd[@]}" "$work_dir/bug.rs") >/dev/null 2>&1
      status=$?
    else
      timeout --preserve-status "${HANG_TIMEOUT}s" "${compiler_cmd[@]}" "$work_dir/bug.rs" >/dev/null 2>&1
      status=$?
    fi
    # 124, 137, and 143 are timeout-related termination status codes.
    [[ $status -eq 124 || $status -eq 137 || $status -eq 143 ]] && exit 0 || exit 1
    ;;
  1) # ICE check
    set +e
    output_log="$work_dir/output.log"
    if [[ "$COMPILER_FLAG" == "3" && -n "$GCCRS_WORK_DIR" ]]; then
      (cd "$GCCRS_WORK_DIR" && timeout --preserve-status 8s "${compiler_cmd[@]}" "$work_dir/bug.rs") >"$output_log" 2>&1
      status=$?
    else
      timeout --preserve-status 8s "${compiler_cmd[@]}" "$work_dir/bug.rs" >"$output_log" 2>&1
      status=$?
    fi
    [[ $status -eq 124 || $status -eq 137 || $status -eq 143 ]] && exit 1
    grep -Eqi "internal compiler error|thread 'rustc' panicked|query stack during panic|this is a bug in the compiler|we would appreciate a bug report|rust1: internal compiler error|gccrs: internal compiler error|crab1: internal compiler error" "$output_log" && exit 0 || exit 1
    ;;
  2) # Standard success, new fail
    rustc +nightly --crate-type lib "$work_dir/bug.rs" >/dev/null 2>&1 || exit 1
    rustc +nightly -Z next-solver=globally --crate-type lib "$work_dir/bug.rs" >/dev/null 2>&1
    [ $? -ne 0 ] && exit 0 || exit 1
    ;;
  3) # Standard fail, new success
    rustc +nightly --crate-type lib "$work_dir/bug.rs" >/dev/null 2>&1 && exit 1
    rustc +nightly -Z next-solver=globally --crate-type lib "$work_dir/bug.rs" >/dev/null 2>&1
    [ $? -eq 0 ] && exit 0 || exit 1
    ;;
esac