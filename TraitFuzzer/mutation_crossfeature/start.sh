#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASE_DIR="$ROOT_DIR"
FUZZ_DIR="$ROOT_DIR"
CROSS_DIR="$FUZZ_DIR/mutation_crossfeature"

OLLAMA_CMD="/usr/local/bin/ollama"
OLLAMA_HOST="127.0.0.1:12023"
OLLAMA_MODELS="${OLLAMA_MODELS:-$HOME/.ollama/models}"
PYTHON_CMD="/usr/bin/python"

PID_DIR="$BASE_DIR/.pids"
mkdir -p "$PID_DIR"

CROSS_MAIN_PID_FILE="$PID_DIR/cross_main.pid"
CROSS_OLLAMA_PID_FILE="$PID_DIR/cross_ollama.pid"

CROSS_LOG="$CROSS_DIR/cross_fuzz.log"
OLLAMA_LOG="$CROSS_DIR/cross_ollama.log"

start_ollama_if_needed() {
  if ss -ltnp | grep -q ":12023"; then
    echo "Ollama port 12023 already in use, reusing existing service."
    return 0
  fi

  if [ -f "$CROSS_OLLAMA_PID_FILE" ]; then
    local old_pid
    old_pid=$(cat "$CROSS_OLLAMA_PID_FILE" 2>/dev/null || true)
    if [ -n "$old_pid" ] && kill -0 "$old_pid" 2>/dev/null; then
      echo "Cross ollama already recorded as running (PID $old_pid)."
      return 0
    fi
    rm -f "$CROSS_OLLAMA_PID_FILE" || true
  fi

  echo "Starting ollama for cross-feature..."
  (
    cd "$CROSS_DIR"
    nohup env \
      OLLAMA_DEBUG=1 \
      OLLAMA_HOST="$OLLAMA_HOST" \
      OLLAMA_MODELS="$OLLAMA_MODELS" \
      OLLAMA_NUM_PARALLEL=1 \
      OMP_NUM_THREADS=32 \
      nice -n 10 taskset -c 0-31 "$OLLAMA_CMD" serve \
      > "$OLLAMA_LOG" 2>&1 &
    echo $! > "$CROSS_OLLAMA_PID_FILE"
  )

  sleep 2
  if ss -ltnp | grep -q ":12023"; then
    echo "Ollama started (PID $(cat "$CROSS_OLLAMA_PID_FILE" 2>/dev/null || echo '?'))."
  else
    echo "Failed to start Ollama. Check $OLLAMA_LOG"
    return 1
  fi
}

start_cross_main() {
  if [ -f "$CROSS_MAIN_PID_FILE" ]; then
    local old_pid
    old_pid=$(cat "$CROSS_MAIN_PID_FILE" 2>/dev/null || true)
    if [ -n "$old_pid" ] && kill -0 "$old_pid" 2>/dev/null; then
      echo "main_cross.py already running (PID $old_pid)."
      return 0
    fi
    rm -f "$CROSS_MAIN_PID_FILE" || true
  fi

  echo "Starting main_cross.py..."
  (
    cd "$FUZZ_DIR"
    nohup nice -n 5 "$PYTHON_CMD" mutation_crossfeature/main_cross.py \
      --cross-config mutation_crossfeature/config_cross.json \
      > "$CROSS_LOG" 2>&1 &
    echo $! > "$CROSS_MAIN_PID_FILE"
  )

  echo "Cross main started (PID $(cat "$CROSS_MAIN_PID_FILE" 2>/dev/null || echo '?'))."
}

start_services() {
  start_ollama_if_needed
  start_cross_main
  echo "Started. Logs:"
  echo "  - $CROSS_LOG"
  echo "  - $OLLAMA_LOG"
}

kill_cross_main() {
  if [ ! -f "$CROSS_MAIN_PID_FILE" ]; then
    echo "No cross main PID file found."
    return 0
  fi

  local pid
  pid=$(cat "$CROSS_MAIN_PID_FILE" 2>/dev/null || true)
  if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
    echo "Killing main_cross.py PID $pid and children..."
    pkill -P "$pid" || true
    kill "$pid" || true
    sleep 1
  fi
  rm -f "$CROSS_MAIN_PID_FILE" || true
}

kill_cross_ollama() {
  if [ ! -f "$CROSS_OLLAMA_PID_FILE" ]; then
    echo "No cross ollama PID file found (likely reused external ollama)."
    return 0
  fi

  local pid
  pid=$(cat "$CROSS_OLLAMA_PID_FILE" 2>/dev/null || true)
  if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
    echo "Killing cross ollama PID $pid and children..."
    pkill -P "$pid" || true
    kill "$pid" || true
    sleep 1
  fi
  rm -f "$CROSS_OLLAMA_PID_FILE" || true
}

kill_services() {
  kill_cross_main
  kill_cross_ollama
  echo "Cross-feature services stopped."

  echo "Clean cross-feature outputs and locks? Enter 1 to clean, any other key to skip:"
  read -r do_clean
  if [ "$do_clean" = "1" ]; then
    (cd "$FUZZ_DIR" && "$PYTHON_CMD" mutation_crossfeature/clean_cross.py --all)
  else
    echo "Skip cleanup."
  fi
}

clean_only() {
  (cd "$FUZZ_DIR" && "$PYTHON_CMD" mutation_crossfeature/clean_cross.py --all)
}

watch_logs() {
  echo "Choose log to watch:"
  echo "1) cross_fuzz.log"
  echo "2) cross_ollama.log"
  read -r choice
  case "$choice" in
    1) tail -f "$CROSS_LOG" ;;
    2) tail -f "$OLLAMA_LOG" ;;
    *) echo "Unknown choice" ;;
  esac
}

show_status() {
  echo "==== Cross Feature Status ===="

  if [ -f "$CROSS_MAIN_PID_FILE" ]; then
    local mpid
    mpid=$(cat "$CROSS_MAIN_PID_FILE" 2>/dev/null || true)
    if [ -n "$mpid" ] && kill -0 "$mpid" 2>/dev/null; then
      echo "main_cross.py: running (PID $mpid)"
    else
      echo "main_cross.py: stale pid file"
    fi
  else
    echo "main_cross.py: not managed"
  fi

  if ss -ltnp | grep -q ":12023"; then
    echo "ollama(12023): listening"
  else
    echo "ollama(12023): not listening"
  fi
}

menu() {
  echo "Enter 1: start  2: kill  3: watch  4: status  5: clean"
  read -r action
  case "$action" in
    1|start) start_services ;;
    2|kill) kill_services ;;
    3|watch) watch_logs ;;
    4|status) show_status ;;
    5|clean) clean_only ;;
    *) echo "Unknown action" ;;
  esac
}

if [ $# -gt 0 ]; then
  case "$1" in
    start) start_services ;;
    kill) kill_services ;;
    watch) watch_logs ;;
    status) show_status ;;
    clean) clean_only ;;
    *)
      echo "Usage: $0 [start|kill|watch|status|clean]"
      exit 1
      ;;
  esac
else
  menu
fi
