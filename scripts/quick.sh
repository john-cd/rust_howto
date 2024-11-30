#! /bin/bash

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes

set -o pipefail
set -e

# Function called by trap
cleanup() {
    cp -f ./book.toml.bak ./book.toml
    exit
}

trap cleanup 1 2 3 6

if [ -f ./book.toml ]; then
  mv -f ./book.toml ./book.toml.bak
fi

# Make sure that the book builds in the same folder than `serve`
# Also overwrite the title
MDBOOK_BUILD__BUILD_DIR="book/html" MDBOOK_BOOK='{"title": "QUICK SERVE"}' \
mdbook serve -p 3001 -n 127.0.0.1 --open

echo "DONE"

# Note1: Using the env variable MDBOOK_* only seems to override existing values, not erase them.
# Examples:
#   MDBOOK_BOOK="$(toml2json ./book-dev.toml)" mdbook build
#   MDBOOK_OUTPUT__LINKCHECK='{"warning-policy": "ignore"}' MDBOOK_PREPROCESSOR__INDEXING='{"skip_renderer": "html,markdown,linkcheck"}'
# See the doc on overriding mdbook config: https://rust-lang.github.io/mdBook/format/configuration/environment-variables.html
#
# Note2: mdbook watch --open --watcher=poll / native does not have -p -n options.
