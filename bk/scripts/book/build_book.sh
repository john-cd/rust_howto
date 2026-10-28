#!/usr/bin/env bash
# Purpose: Generate the book's HTML / JS.
# Usage: ./scripts/build_book.sh.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

# Generate the book's HTML / JS

root="$(realpath $1)/"

# Make sure that `book.toml` is available
# See `quick.sh`
if [ ! -f ${root}book.toml ]; then
    cp -f ${root}book.toml.bak ${root}book.toml
fi

# The PDF profile needs LaTeX/SVG tooling that local machines often lack.
# Render Markdown instead unless the PDF is explicitly requested (BUILD_PDF=1).
if [ "${BUILD_PDF:-0}" != "1" ]; then
    export MDBOOK_OUTPUT__PANDOC__PROFILE__PDF__TO=gfm
    export MDBOOK_OUTPUT__PANDOC__PROFILE__PDF__OUTPUT_FILE=book.md
fi

mdbook build ${root}

echo "DONE"
