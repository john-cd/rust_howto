#!/usr/bin/env bash
# Purpose: Run a test that depends on a configured external service.
# Usage: From bk/, run the matching just recipe with service or service:test.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

root="$(realpath "$1")"
d="${root}/.devcontainer"
selection="$2"
service="${selection%%:*}"
test_filter="${selection#*:}"
if [[ "${selection}" != *:* ]]; then
    test_filter="${selection}"
fi

compose() {
    docker compose --env-file "${d}/.env" \
        -f "${d}/compose.yaml" \
        -f "${d}/compose.override.yaml" \
        -f "${d}/compose.heavy-tests.yaml" \
        "$@"
}

cleanup() {
    local exit_status=$?
    trap - EXIT
    compose rm -f --stop --volumes "$service" || {
        local cleanup_status=$?
        if (( exit_status == 0 )); then
            exit_status=$cleanup_status
        fi
    }
    exit "$exit_status"
}

trap cleanup EXIT

## "selection" is a service name, optionally followed by :test-name.
## The test-name filters test names or binary ids.
##
## Start the selected service and wait until it is running or healthy.
compose up --wait "$service"

## Run tests, which names include the selected test-name, ignoring the default filter that normally suppress "require_external_svc" tests.
cd "$root"
cargo nextest run --all-targets --locked --no-tests=warn --no-fail-fast --success-output immediate --ignore-default-filter --all-features -E "test(~${test_filter}) or binary_id(~${test_filter})"
