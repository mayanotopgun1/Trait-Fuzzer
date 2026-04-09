#!/usr/bin/env bash
set -euo pipefail
export LANG="C"
export LC_ALL="C"
export LANGUAGE="en_US:en"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

MODE="0"
SOURCE_FILE="$SCRIPT_DIR/bug.rs"
TREE_TIMEOUT="10"
COMPILER_FLAG="1"
HANG_TIMEOUT="5"
MODE_SET=0
COMPILER_SET=0
HANG_TIMEOUT_SET=0

prompt_mode() {
  while true; do
    read -r -p "bug type (0=hang, 1=ice): " MODE
    case "$MODE" in
      0|1) return 0 ;;
      *) echo "Invalid bug type. Please input 0 or 1." ;;
    esac
  done
}

prompt_compiler_flag() {
  while true; do
    read -r -p "compiler flag (0=stable, 1=nightly, 2=next-solver, 3=gccrs): " COMPILER_FLAG
    case "$COMPILER_FLAG" in
      0|1|2|3) return 0 ;;
      *) echo "Invalid compiler flag. Please input 0, 1, 2, or 3." ;;
    esac
  done
}

prompt_hang_timeout() {
  while true; do
    read -r -p "timeout seconds for hang check: " HANG_TIMEOUT
    if [[ "$HANG_TIMEOUT" =~ ^[0-9]+$ ]] && [[ "$HANG_TIMEOUT" -gt 0 ]]; then
      return 0
    fi
    echo "Invalid timeout. Please input a positive integer."
  done
}

while [[ $# -gt 0 ]]; do
  case $1 in
    --mode) MODE="$2"; MODE_SET=1; shift 2 ;;
    --compiler-flag) COMPILER_FLAG="$2"; COMPILER_SET=1; shift 2 ;;
    --hang-timeout) HANG_TIMEOUT="$2"; HANG_TIMEOUT_SET=1; shift 2 ;;
    *) SOURCE_FILE="$1"; shift ;;
  esac
done

if [[ "$MODE_SET" -eq 0 ]]; then
  prompt_mode
elif [[ "${MODE:-}" != "0" && "${MODE:-}" != "1" ]]; then
  prompt_mode
fi

if [[ "$COMPILER_SET" -eq 0 ]]; then
  prompt_compiler_flag
elif [[ "${COMPILER_FLAG:-}" != "0" && "${COMPILER_FLAG:-}" != "1" && "${COMPILER_FLAG:-}" != "2" && "${COMPILER_FLAG:-}" != "3" ]]; then
  prompt_compiler_flag
fi

if [[ "$MODE" == "0" ]]; then
  if [[ "$HANG_TIMEOUT_SET" -eq 0 ]]; then
    prompt_hang_timeout
  elif ! [[ "${HANG_TIMEOUT:-}" =~ ^[0-9]+$ ]] || [[ "$HANG_TIMEOUT" -le 0 ]]; then
    prompt_hang_timeout
  fi
fi

case "$MODE" in
  0)
    TREE_TIMEOUT="$((HANG_TIMEOUT + 8))"
    if [[ "$TREE_TIMEOUT" -lt 10 ]]; then
      TREE_TIMEOUT="10"
    fi
    ;;
  1) TREE_TIMEOUT="12" ;;
  *) TREE_TIMEOUT="10" ;;
esac

# Export variables for test.sh.
export FUZZ_MODE="$MODE"
export FUZZ_COMPILER_FLAG="$COMPILER_FLAG"
export FUZZ_HANG_TIMEOUT="$HANG_TIMEOUT"
export FUZZ_GCCRS_BIN="${FUZZ_GCCRS_BIN:-$ROOT_DIR/gccrs/build/gcc/crab1}"
export FUZZ_GCCRS_WORK_DIR="${FUZZ_GCCRS_WORK_DIR:-$ROOT_DIR/gccrs/build}"
export FUZZ_GCCRS_AUTO_NO_CORE="${FUZZ_GCCRS_AUTO_NO_CORE:-1}"
chmod +x "$SCRIPT_DIR/test.sh"

rm -f "$SCRIPT_DIR/treereduce.out"
# Pass the script path directly; @@ is replaced with a temporary filename.
exec treereduce-rust -s "$SOURCE_FILE" "$SCRIPT_DIR/test.sh" @@ --timeout "$TREE_TIMEOUT" -j 1 -o "$SCRIPT_DIR/treereduce.out"