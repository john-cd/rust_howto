#! /bin/bash

# Generate the book's HTML / JS
# Usage: ./scripts/build_book.sh

set -e

# Make sure that `book.toml` is available
# See `quick.sh`
if [ ! -f ./book.toml ]; then
    cp -f ./book.toml.bak ./book.toml
fi

mdbook build

echo "DONE"
