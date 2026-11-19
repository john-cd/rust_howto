#!/usr/bin/env bash
# Purpose: Generate the book's HTML and optional PDF.
# Usage: ./scripts/book/build_book.sh <book-root>.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

root="$(realpath "$1")/"

# Make sure that `book.toml` is available
# See `quick.sh`
if [ ! -f "${root}book.toml" ]; then
    cp -f "${root}book.toml.bak" "${root}book.toml"
fi

case "${BUILD_PDF:-0}" in
    1|true)
        for tool in mdbook-pandoc pandoc tectonic rsvg-convert; do
            if ! command -v "$tool" >/dev/null; then
                printf 'PDF build requires %s on PATH.\n' "$tool" >&2
                exit 1
            fi
        done
        if [[ "$OSTYPE" == msys* && -z "${TECTONIC_CACHE_DIR:-}" ]]; then
            TECTONIC_CACHE_DIR="$(cygpath -am "${CODE_CACHE_ROOT:-D:/cache}/tectonic")"
            export TECTONIC_CACHE_DIR
        fi
        export MDBOOK_OUTPUT__PANDOC__OPTIONAL=false
        ;;
    *)
        # Skip the optional renderer even when installed, including on Windows.
        export MDBOOK_OUTPUT__PANDOC__OPTIONAL=true
        export MDBOOK_OUTPUT__PANDOC__COMMAND=mdbook-pandoc-disabled-for-html-build
        export MDBOOK_OUTPUT__PANDOC__PROFILE__PDF__TO=gfm
        export MDBOOK_OUTPUT__PANDOC__PROFILE__PDF__OUTPUT_FILE=book.md
        ;;
esac

mdbook build "$root"

echo "DONE"
