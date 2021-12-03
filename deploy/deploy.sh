#!/usr/bin/env bash
set -ueo pipefail

local_repo_path="$(cd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" && pwd)"
machine="${DEPLOY_MACHINE:-}"
repo="${DEPLOY_REPO:-$local_repo_path}"
private_key_file="${DEPLOY_PRIVATE_KEY_FILE:-}"
dry_run=
verbose=
logs_only=

function usage() {
  cat <<EOF
deploy.sh deploy node to remote or local machine
Options
-m MACHINE             deploy to this machine
-r REPO                repo path (default: grandparent directory of this file)
-k PRIVATE_KEY_FILE    set the private key to use
-h                     show usage
-l                     follow logs only
-v                     increase verbosity
-d, -n                 dry run
EOF
}

while getopts "m:r:k:hlvdn-" opt; do
  case $opt in
  m)
    machine="$OPTARG"
    ;;
  r)
    repo="$OPTARG"
    ;;
  k)
    private_key_file="$OPTARG"
    ;;
  h)
    usage
    exit
    ;;
  l)
    logs_only=y
    ;;
  v)
    verbose=
    ;;
  d | n)
    dry_run=y
    ;;
  -)
    break
    ;;
  \?)
    echo "Invalid option: -$OPTARG" >&2
    usage
    exit 1
    ;;
  esac
done

use_ssh="$machine"
repo_name="$(basename "$repo")"
repo_path="$local_repo_path"
if [[ -n "$use_ssh" ]]; then
  repo_path="$repo_name"
fi

function quote_command() {
  python3 -c 'import shlex; import sys; print(shlex.join(sys.argv[1:]))' "$@"
}

function run_command() {
  command=("${@}")
  if [[ -n "$dry_run" ]]; then
    quote_command "${command[@]}"
  else
    if [[ -n "$verbose" ]]; then
      quote_command "${command[@]}"
    fi
    "${command[@]}"
  fi
}

function run_maybe_remote_command() {
  command=("${@}")
  if [[ -n "$use_ssh" ]]; then
    # ssh requires a single argument
    command=(ssh "$machine" "$(quote_command "$@")")
  fi
  run_command "${command[@]}"
}

function copy_repo() {
  if [[ -z "$use_ssh" ]]; then
    return
  fi
  run_command rsync -avz --progress --exclude target --cvs-exclude -h "$repo/" "$machine:$repo_name/"
}

function start_node() {
  run_maybe_remote_command env DOCKER_BUILDKIT=1 docker build "$repo_path" -t zhenxunge-node
  run_maybe_remote_command docker-compose -f "$repo_path/docker-compose.yml" up -d
}

function follow_node_logs() {
  run_maybe_remote_command docker-compose -f "$repo_path/docker-compose.yml" logs -f
}

function sanity_check() {
  local_tools=(rsync python3)
  remote_tools=(docker docker-compose)
  if ! command -v "${local_tools[@]}" >/dev/null; then
    echo "You need ${local_tools[*]} installed in the local machine." >&2
    exit 1
  fi
  if ! run_maybe_remote_command command -v "${remote_tools[@]}" >/dev/null; then
    echo "You need ${remote_tools[*]} installed in the remote machine." >&2
    exit 1
  fi
  run_maybe_remote_command mkdir -p "$repo_name"
}

function deploy() {
  sanity_check
  if [[ -z "$logs_only" ]]; then
    copy_repo
    start_node
  fi
  follow_node_logs
}

deploy
