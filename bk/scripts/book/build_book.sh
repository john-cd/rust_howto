 #!/usr/bin/env bash
set -euo pipefail

# Generate the book's HTML / JS
# Usage: ./scripts/build_book.sh

root="$(realpath $1)/"

# Make sure that `book.toml` is available
# See `quick.sh`
if [ ! -f ${root}book.toml ]; then
    cp -f ${root}book.toml.bak ${root}book.toml
fi

mdbook build ${root}

echo "DONE"
