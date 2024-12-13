#! /bin/bash
set -uo pipefail

feature=$1

d="/code/.devcontainer/"

## feature must be both the name of a feature in `deps/Cargo.toml`
## and a service in `compose-heavy-tests.yaml`
##
## Start a service, waiting for it to be running|healthy.
docker compose -f ${d}compose.yaml \
                -f ${d}compose.override.yaml \
                -f ${d}compose-heavy-tests.yaml \
                up --wait $feature

## Run tests, which names include the feature name, in the `deps` crate only, with the desired compile feature enabled
cargo nextest run --package dependencies --tests --locked --no-tests=warn --no-fail-fast --success-output immediate --features $feature -- $feature

## Removes stopped service containers. Stop the containers, if required, before removing
docker compose -f ${d}compose.yaml \
                -f ${d}compose.override.yaml \
                -f ${d}compose-heavy-tests.yaml \
                rm -f --stop --volumes $feature
