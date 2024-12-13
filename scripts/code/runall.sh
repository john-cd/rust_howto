#! /bin/bash
set -euo pipefail

# Run additional examples (under the `xmpl` folder)
# Usage: /code/scripts/runall.sh

set -o pipefail
set -e

root="/code/"
# Create a list of the (last part of) folder names under the `xmpl` directory, space separated
xmpl=$(find ${root}xmpl -mindepth 1 -maxdepth 1 -type d | awk -F'/' '{print $(NF)}' | tr '\n' ' ')

# Run additional examples in the xmpl folder, if any
for d in $xmpl; do ( echo $d; cargo run --package $d --locked ); done

## The examples under the deps folder have been replaced by tests.
## Should you want to add examples again, add the following to `runall`:
## Run examples that are simple .rs files in deps/examples
#examples=$(find ${root}deps/examples -mindepth 1 -maxdepth 1 -type f | xargs basename --suffix=.rs | tr '\n' ' ')
#for e in $examples; do ( echo $e; cargo run --example $e --locked || true); done
## Run examples that are in a subfolder of deps/examples
#examples_in_dir=$(find ${root}deps/examples -mindepth 1 -maxdepth 1 -type d | xargs basename --multiple | tr '\n' ' ')
#for e in $examples_in_dir; do ( echo $e; cargo run --example $e --locked || true ); done

echo "DONE"
