#!/usr/bin/env bash
set -euo pipefail

# Run additional examples (under the `crates/xmpl` folder)
# Usage: ./scripts/runall.sh <root_folder>

set -o pipefail
set -e

root="$(realpath $1)/"
# Create a list of the (last part of) folder names under the `xmpl` directory, space separated
xmpl=$(find ${root} -mindepth 1 -maxdepth 1 -type d -not -name "target" -not -name "scripts" | awk -F'/' '{print $(NF)}' | tr '\n' ' ')

# Run additional examples in the xmpl folder, if any
for d in $xmpl; do ( echo $d; cargo run --package $d --locked ); done

echo "DONE"
