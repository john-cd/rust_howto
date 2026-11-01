#!/usr/bin/env bash
# Purpose: Run a test that depends on a configured external service.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

root="$(realpath "$1")"
d="${root}/.devcontainer"
feature="$2"

compose() {
    docker compose --env-file "${d}/.env" \
        -f "${d}/compose.yaml" \
        -f "${d}/compose.override.yaml" \
        -f "${d}/compose-heavy-tests.yaml" \
        "$@"
}

cleanup() {
    local exit_status=$?
    trap - EXIT
    compose rm -f --stop --volumes "$feature" || {
        local cleanup_status=$?
        if (( exit_status == 0 )); then
            exit_status=$cleanup_status
        fi
    }
    exit "$exit_status"
}

trap cleanup EXIT

## "feature" must be the name of a service in `compose-heavy-tests.yaml`
## and the name of a test or test module under `crates`.
##
## Start a service, waiting for it to be running|healthy.
compose up --wait "$feature"

## Run tests, which names include the feature name, ignoring the default filter that normally suppress "require_external_svc" tests.
cd "$root"
cargo nextest run --all-targets --locked --no-tests=warn --no-fail-fast --success-output immediate --ignore-default-filter --all-features -E "test(~${feature}) or binary_id(~${feature})"
