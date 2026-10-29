#!/usr/bin/env bash
# Purpose: Run additional examples (under the `crates/xmpl` folder).
# Usage: ./scripts/runall.sh <root_folder>.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

# Run additional examples (under the `crates/xmpl` folder)

set -o pipefail
set -e

root="$(realpath "$1")/"
# Create a list of the (last part of) folder names under the `xmpl` directory, space separated
while IFS= read -r -d '' path; do
  d="${path##*/}"
  echo "$d"
  cargo run --package "$d" --locked
done < <(find "$root" -mindepth 1 -maxdepth 1 -type d -not -name "target" -not -name "scripts" -print0)

echo "DONE"
