#!/usr/bin/env bash
set -euo pipefail

TARGET_USER="${TARGET_USER:-$(id -un)}"
ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
LOG_DIR="$ROOT_DIR/logs"
mkdir -p "$LOG_DIR"

AGENT_CONFIG="$ROOT_DIR/Traitor_Agent/Traitorconfig.json"
PID_DIR="$ROOT_DIR/.pids"
mkdir -p "$PID_DIR"

OLLAMA_PID_FILE="$PID_DIR/traitor_agent_ollama.pid"
RUNNER_PID_FILE="$PID_DIR/traitor_agent_runner.pid"

OLLAMA_LOG="$LOG_DIR/traitor_agent_ollama.log"
RUNNER_LOG="$LOG_DIR/traitor_agent_runner.log"

MODEL_DEFAULT="qwen2.5-coder:14b"

ollama_host_from_config() {
	python3 - <<'PY'
import json
from pathlib import Path
cfg = Path("Traitor_Agent/Traitorconfig.json")
default = "127.0.0.1:12023"
try:
	obj = json.loads(cfg.read_text(encoding="utf-8"))
	api = str(obj.get("llm", {}).get("api_base", "")).strip()
	if api.startswith("http://"):
		print(api[len("http://"):].rstrip("/"))
	elif api.startswith("https://"):
		print(api[len("https://"):].rstrip("/"))
	elif api:
		print(api.rstrip("/"))
	else:
		print(default)
except Exception:
	print(default)
PY
}

OLLAMA_HOST_VALUE="$(cd "$ROOT_DIR" && ollama_host_from_config)"

pid_cmdline() {
	local pid="$1"
	ps -p "$pid" -o args= 2>/dev/null || true
}

pid_owner() {
	local pid="$1"
	ps -o user= -p "$pid" 2>/dev/null | awk '{print $1}' || true
}

is_target_user_pid() {
	local pid="$1"
	local owner
	owner="$(pid_owner "$pid")"
	[[ -n "$owner" && "$owner" == "$TARGET_USER" ]]
}

safe_kill_pidfile() {
	local pid_file="$1"
	local expected_pat="$2"
	local label="$3"

	if [[ ! -f "$pid_file" ]]; then
		echo "No ${label} PID file found."
		return 0
	fi

	local pid
	pid="$(cat "$pid_file" 2>/dev/null || true)"
	if [[ -z "$pid" || ! "$pid" =~ ^[0-9]+$ ]]; then
		echo "[info] invalid ${label} PID file, cleaning up: $pid_file"
		rm -f "$pid_file" || true
		return 0
	fi

	if ! kill -0 "$pid" 2>/dev/null; then
		echo "[info] stale ${label} pid file detected (pid=$pid), cleaning up"
		rm -f "$pid_file" || true
		return 0
	fi

	if ! is_target_user_pid "$pid"; then
		echo "Skip killing ${label} pid=$pid: owner is not $TARGET_USER"
		return 0
	fi

	local cmd
	cmd="$(pid_cmdline "$pid")"
	if ! echo "$cmd" | grep -Eq "$expected_pat"; then
		echo "Skip killing ${label}: cmdline mismatch"
		echo "  expected: $expected_pat"
		echo "  actual:   $cmd"
		return 0
	fi

	echo "Killing ${label} pid=$pid"
	kill "$pid" 2>/dev/null || true
	sleep 1
	if kill -0 "$pid" 2>/dev/null; then
		kill -9 "$pid" 2>/dev/null || true
	fi
	rm -f "$pid_file" || true
}

ollama_ready() {
	OLLAMA_HOST="$OLLAMA_HOST_VALUE" ollama list >/dev/null 2>&1
}

wait_ollama_ready() {
	local tries="${1:-20}"
	local i
	for i in $(seq 1 "$tries"); do
		if ollama_ready; then
			return 0
		fi
		sleep 1
	done
	return 1
}

cleanup_previous_outputs() {
	cd "$ROOT_DIR"
	python3 - <<'PY'
import json
import shutil
from pathlib import Path

root = Path('.').resolve()
cfg_path = root / 'Traitor_Agent' / 'Traitorconfig.json'

out_dir = root / 'Traitor_Agent' / 'trait_dense_seeds'
summary = root / 'Traitor_Agent' / 'run_summary.jsonl'

try:
	cfg = json.loads(cfg_path.read_text(encoding='utf-8'))
	paths = cfg.get('paths', {})
	out_cfg = paths.get('output_dir')
	sum_cfg = paths.get('summary_jsonl')
	if out_cfg:
		p = Path(out_cfg)
		out_dir = p if p.is_absolute() else (root / p)
	if sum_cfg:
		p = Path(sum_cfg)
		summary = p if p.is_absolute() else (root / p)
except Exception:
	pass

if out_dir.exists():
	shutil.rmtree(out_dir)
out_dir.mkdir(parents=True, exist_ok=True)

# Keep output directory clean: summary should not stay inside output dir.
if str(summary).startswith(str(out_dir) + "/") or summary == out_dir:
	summary = root / 'Traitor_Agent' / 'run_summary.jsonl'

if summary.exists() and summary.is_file():
	summary.unlink()

total = root / 'Traitor_Agent' / 'total'
if total.exists() and total.is_file():
	total.unlink()

print(f"[info] cleaned previous outputs: {out_dir}")
print(f"[info] cleaned previous summary: {summary}")
PY
}

start_ollama() {
	local model="${1:-$MODEL_DEFAULT}"
	echo "[info] target ollama host: $OLLAMA_HOST_VALUE"

	if [[ -f "$OLLAMA_PID_FILE" ]]; then
		local pid
		pid="$(cat "$OLLAMA_PID_FILE" 2>/dev/null || true)"
		if [[ -n "$pid" ]] && kill -0 "$pid" 2>/dev/null && ollama_ready; then
			echo "[info] ollama already running (pid=$pid)"
			return 0
		fi
	fi

	safe_kill_pidfile "$OLLAMA_PID_FILE" "ollama[[:space:]]+serve|/usr/local/bin/ollama" "ollama"
	OLLAMA_HOST="$OLLAMA_HOST_VALUE" nohup ollama serve >"$OLLAMA_LOG" 2>&1 &
	echo $! >"$OLLAMA_PID_FILE"

	if ! wait_ollama_ready 25; then
		echo "[error] ollama is not reachable on $OLLAMA_HOST_VALUE"
		echo "[hint] check log: $OLLAMA_LOG"
		return 1
	fi

	OLLAMA_HOST="$OLLAMA_HOST_VALUE" ollama pull "$model" >>"$OLLAMA_LOG" 2>&1 || true
	echo "[ok] ollama started"
}

start_runner_bg() {
	if [[ -f "$RUNNER_PID_FILE" ]]; then
		local pid
		pid="$(cat "$RUNNER_PID_FILE" 2>/dev/null || true)"
		if [[ -n "$pid" ]] && kill -0 "$pid" 2>/dev/null; then
			echo "[info] runner already running (pid=$pid)"
			return 0
		fi
	fi

	cleanup_previous_outputs
	cd "$ROOT_DIR"
	nohup python3 Traitor_Agent/run_dataset.py --config Traitor_Agent/Traitorconfig.json >"$RUNNER_LOG" 2>&1 &
	echo $! >"$RUNNER_PID_FILE"
	echo "[ok] dataset runner started in background (pid=$(cat "$RUNNER_PID_FILE"))"
}

start_runner_fg() {
	cleanup_previous_outputs
	cd "$ROOT_DIR"
	echo "[info] running dataset generation in foreground ..."
	python3 Traitor_Agent/run_dataset.py --config Traitor_Agent/Traitorconfig.json | tee "$RUNNER_LOG"
	echo "[ok] finished. see Traitor_Agent/total"
}

watch_status() {
	echo "=== Traitor_Agent status ==="
	if [[ -f "$OLLAMA_PID_FILE" ]]; then
		echo "ollama pid: $(cat "$OLLAMA_PID_FILE" 2>/dev/null || true)"
	else
		echo "ollama pid: (none)"
	fi
	if [[ -f "$RUNNER_PID_FILE" ]]; then
		echo "runner pid: $(cat "$RUNNER_PID_FILE" 2>/dev/null || true)"
	else
		echo "runner pid: (none)"
	fi
	echo "--- tail runner log ---"
	if [[ ! -f "$RUNNER_LOG" ]]; then
		touch "$RUNNER_LOG"
		echo "[info] runner log not created yet; created empty file: $RUNNER_LOG"
	fi
	tail -n 30 "$RUNNER_LOG" 2>/dev/null || true
	echo "--- total snapshot ---"
	cat "$ROOT_DIR/Traitor_Agent/total" 2>/dev/null || echo "(no total yet)"
}

watch_logs() {
	echo "Choose log to watch:"
	echo "1) runner log"
	echo "2) ollama log"
	read -r choice
	case "$choice" in
		1)
			if [[ ! -f "$RUNNER_LOG" ]]; then
				touch "$RUNNER_LOG"
				echo "[info] runner log not created yet; waiting for new output..."
			fi
			tail -f "$RUNNER_LOG"
			;;
		2)
			if [[ ! -f "$OLLAMA_LOG" ]]; then
				touch "$OLLAMA_LOG"
				echo "[info] ollama log not created yet; waiting for new output..."
			fi
			tail -f "$OLLAMA_LOG"
			;;
		*) echo "Unknown choice" ;;
	esac
}

kill_services() {
	safe_kill_pidfile "$RUNNER_PID_FILE" "python(3)?[[:space:]]+Traitor_Agent/run_dataset\.py|Traitor_Agent/run_dataset\.py" "runner"
	safe_kill_pidfile "$OLLAMA_PID_FILE" "ollama[[:space:]]+serve|/usr/local/bin/ollama" "ollama"
	echo "[ok] stopped"
}

show_occupy() {
	echo "===== ${TARGET_USER} resource usage ====="
	ps -u "$TARGET_USER" -o pid,ppid,%cpu,%mem,rss,etime,cmd --sort=-%cpu | head -n 16 || true
	echo
	du -sh "$ROOT_DIR/Traitor_Agent/trait_dense_seeds" 2>/dev/null || true
	du -sh "$ROOT_DIR/logs" 2>/dev/null || true
}

usage() {
	cat <<'EOF'
Usage: ./run_agent.sh [start|run|fg|kill|watch|tail|occupy]
  start/run [model]  start ollama + run dataset in background
  fg [model]         start ollama + run dataset in foreground
  kill               stop runner + ollama safely
  watch              show status + recent logs + total snapshot
  tail               interactive log tail selector
  occupy             show resource usage

No arguments opens an interactive menu.
EOF
}

main_menu() {
	echo "Enter 1: start  2: kill  3: watch  4: tail  5: occupy  6: fg"
	read -r action
	case "$action" in
		1|start) start_ollama "$MODEL_DEFAULT" && start_runner_bg ;;
		2|kill) kill_services ;;
		3|watch) watch_status ;;
		4|tail) watch_logs ;;
		5|occupy) show_occupy ;;
		6|fg) start_ollama "$MODEL_DEFAULT" && start_runner_fg ;;
		*) echo "Unknown action" ;;
	esac
}

if [[ $# -eq 0 ]]; then
	main_menu
	exit 0
fi

cmd="$1"
model="${2:-$MODEL_DEFAULT}"
case "$cmd" in
	start|run)
		start_ollama "$model"
		start_runner_bg
		;;
	fg)
		start_ollama "$model"
		start_runner_fg
		;;
	kill)
		kill_services
		;;
	watch)
		watch_status
		;;
	tail)
		watch_logs
		;;
	occupy)
		show_occupy
		;;
	*)
		usage
		exit 1
		;;
esac
