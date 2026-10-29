#!/usr/bin/env bash
# Purpose: Check that URLs to external websites e.g. https://... (typically found in ref defs) are valid and working (e.g. no 404).
# Usage: ./scripts/urls/check_urls.sh [--advisory] <root folder>
#
#   --advisory  Report failures but exit 0. Without it, any failed check exits 1.
#
# Environment: LYCHEE overrides the lychee executable (used by the fixtures in test_check_urls.sh).
#
# TODO Reviewed ??-??-2026.

set -uo pipefail

# This script does NOT check whether reference definitions are used or not.

advisory=0
if [[ ${1:-} == "--advisory" ]]; then
  advisory=1
  shift
fi

if [[ $# -ne 1 || ! -d $1 ]]; then
  echo "Usage: $0 [--advisory] <root folder>" >&2
  exit 2
fi

lychee_bin="${LYCHEE:-lychee}"
root="$(realpath -- "$1")/"
log="$(mktemp "${TMPDIR:-/tmp}/check_urls.XXXXXX")"
trap 'rm -f -- "$log"' EXIT

# Print a classification of the failures that lychee reported in its output.
classify() {
  local label=$1 file=$2
  echo "== ${label}: failure summary =="
  printf 'broken (404/410):        %s\n' "$(grep -Eic '\[(404|410)\]' "$file")"
  printf 'authentication (401/403): %s\n' "$(grep -Eic '\[(401|403)\]' "$file")"
  printf 'throttling (429):        %s\n' "$(grep -Eic '\[429\]' "$file")"
  printf 'timeouts:                %s\n' "$(grep -Eic 'time(d )?out' "$file")"
  printf 'certificate/TLS:         %s\n' "$(grep -Eic 'certificate|tls|ssl' "$file")"
}

# Run one lychee invocation, show its output and classify a nonzero result.
# Lychee exits 2 for link failures; any other nonzero status is an execution error.
run_check() {
  local label=$1
  shift
  "$@" 2>&1 | tee -a "$log"
  local status=${PIPESTATUS[0]}
  if [[ $status -ne 0 ]]; then
    classify "$label" "$log"
    echo "${label}: lychee exited with status ${status}" >&2
  fi
  return "$status"
}

# The glob patterns are quoted so lychee expands them and paths containing
# spaces stay intact.
failed=0
run_check "files" "$lychee_bin" --exclude-all-private --no-ignore --hidden --format detailed --cache \
  "${root}**/*.md" "${root}**/*.toml" "${root}**/*.yaml" "${root}**/*.yml" || failed=1
# We could also check "${root}.devcontainer/*" "${root}**/*.sh"

# Somehow lychee ignores links in markdown reference definitions... thus the use of `sed` to extract URLs first
shopt -s nullglob
ref_files=("${root}src/refs/"*.md)
if [[ ${#ref_files[@]} -eq 0 ]]; then
  echo "No reference definition files found in ${root}src/refs/" >&2
  failed=1
else
  # shellcheck disable=SC2016
  sed -r 's/\[.+?\]: (.+)$/\1/' -- "${ref_files[@]}" | run_check "refdefs" "$lychee_bin" --exclude-all-private --format=detailed --cache \
    -u "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36" -- - || failed=1
fi

if [[ $failed -ne 0 ]]; then
  if [[ $advisory -eq 1 ]]; then
    echo "DONE (failures ignored: --advisory)"
    exit 0
  fi
  echo "FAILED" >&2
  exit 1
fi
echo "DONE"
