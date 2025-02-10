#! /bin/bash
set -uo pipefail

root="$(realpath $1)/"
d="${root}.devcontainer/"
feature="$2"

## "feature" must be the name of a service in `compose-heavy-tests.yaml`
## and the name of a test or test module under `crates`.
##
## Start a service, waiting for it to be running|healthy.
docker compose -f ${d}compose.yaml \
                -f ${d}compose.override.yaml \
                -f ${d}compose-heavy-tests.yaml \
                up --wait "$feature"

## Run tests, which names include the feature name, ignoring the default filter that normally suppress "require_external_svc" tests.
cd "$root"
cargo nextest run --tests --locked --no-tests=warn --no-fail-fast --success-output immediate --ignore-default-filter --all-features -E "test(~${feature}) or binary_id(~${feature})"

## Removes stopped service containers. Stop the containers, if required, before removing
docker compose -f ${d}compose.yaml \
                -f ${d}compose.override.yaml \
                -f ${d}compose-heavy-tests.yaml \
                rm -f --stop --volumes "$feature"
