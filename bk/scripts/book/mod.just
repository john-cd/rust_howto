# Purpose: Build, serve, and maintain the generated book, its indexes, and references.
# Usage: From bk/, run just book <recipe>.
#
# TODO Reviewed ??-??-2026.

set unstable
set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")
# Executable suffix and Python launcher differ on Windows; recipes themselves run in Bash everywhere.
exe := if os_family() == "windows" { ".exe" } else { "" }
python := if os_family() == "windows" { "python" } else { "python3" }
mdbook_utils_rev := env("MDBOOK_UTILS_REV", "dedddf3699e461cd418c1a044d82aabd3fec62e0")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- BOOK BUILDING -----------------------------------

alias bb := buildbook

# Render unpublished draft chapters without adding them to the published book.
@drafts: _release_mbdbook-scrub
    mdbook build {{ quote(root + "/drafts") }} --dest-dir {{ quote(root + "/book/drafts") }}

# Render future chapters with their own summary and the shared reference definitions.
@future: _release_mbdbook-scrub
    mdbook build {{ quote(root + "/../later") }} --dest-dir {{ quote(root + "/book/future") }}

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
@buildbook: _release_mbdbook-scrub _install-mdbook-indexing refdefs-check _generate-index-category
    just --justfile "{{ source_directory() }}/../bash/mod.just" mdbook-build
    just --justfile "{{ source_directory() }}/../bash/mod.just" sitemap
    just --justfile "{{ source_directory() }}/../bash/mod.just" copystatic

# Merge missing dependency references into the curated crate reference file.
@refdefs-generate: _install-mdbook-utils
    {{ python }} "{{ source_directory() }}/refdefs.py" generate --book-root "{{ root }}"

# Check for missing dependency references without rewriting authoring files.
@refdefs-check: _install-mdbook-utils
    {{ python }} "{{ source_directory() }}/refdefs.py" check --book-root "{{ root }}"
    {{ python }} "{{ source_directory() }}/../indices/update_refdefs_for_examples.py" check --book-root "{{ root }}"

@_install-mdbook-utils:
    cargo install --git https://github.com/john-cd/mdbook-utils --rev {{ mdbook_utils_rev }} --locked mdbook-utils

# Install the pinned word-index preprocessor when a developer renders outside CI.
mdbook_indexing_version := env("MDBOOK_INDEXING_VERSION", "0.2.0")

@_install-mdbook-indexing:
    cargo install --locked --version {{ mdbook_indexing_version }} mdbook-indexing

# Generate the external crates.io category page from its checked-in snapshot.
@_generate-index-category:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ root }}/../tools"
    tmp="$(mktemp)"
    marked_tmp="${tmp}.marked"
    trap 'rm -f "$tmp" "$marked_tmp"' EXIT
    cat "{{ root }}/master/crates.txt" | cargo run --locked -p crate_indices -- category_page_from_metadata --metadata "{{ root }}/master/crate_categories.toml" --book-root "{{ root }}" > "$tmp"
    index="{{ root }}/src/indices/crates_by_category.md"
    if [[ -f "$index" ]]; then
        marker="$(head -n 1 "$index")"
        if [[ "$marker" == '<!-- TODO Reviewed '* ]]; then
            { printf '%s\n' "$marker"; cat "$tmp"; } > "$marked_tmp"
            mv "$marked_tmp" "$tmp"
        fi
    fi
    mv "$tmp" "$index"

# generate index of crates by category
# link crates to published recipes

# Generate the alphabetical crate index with links to published recipes.
@_generate-index-alphabetical:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ root }}/../tools"
    tmp="$(mktemp)"
    marked_tmp="${tmp}.marked"
    trap 'rm -f "$tmp" "$marked_tmp"' EXIT
    cat "{{ root }}/master/crates.txt" | cargo run --locked -p crate_indices -- alphabetical_page --book-root "{{ root }}" > "$tmp"
    index="{{ root }}/src/indices/crates_alphabetical.md"
    if [[ -f "$index" ]]; then
        marker="$(head -n 1 "$index")"
        if [[ "$marker" == '<!-- TODO Reviewed '* ]]; then
            { printf '%s\n' "$marker"; cat "$tmp"; } > "$marked_tmp"
            mv "$marked_tmp" "$tmp"
        fi
    fi
    mv "$tmp" "$index"

# link crates to published recipes

# Build / release the tools, incl. mdbook-scrub preprocessor, if needed
@_release_mbdbook-scrub:
    echo "Build mdbook-scrub, if needed"
    if [ ! -f {{ quote(root + "/../bin/mdbook-scrub" + exe) }} ] || [ ! -f {{ quote(root + "/../bin/mdbook-private" + exe) }} ] || [ ! -f {{ quote(root + "/../bin/mdbook-linkcheck-compat" + exe) }} ]; then just -f {{ quote(root + "/../tools/justfile") }} release; fi

## ---- BOOK SERVING -----------------------------------

alias s := serve

# Serve the book (incl. link checking)
@serve: _install-mdbook-indexing
    echo "Serve the book"
    cd {{ quote(root) }} && mdbook serve -p 3000 -n 127.0.0.1 --open {{ quote(root) }}
    ## NOTE: can conflict with "port" / EXPOSE in the Docker / Docker Compose configuration
    ## Or use: cd {{ root }}/book/html ; python3 -m http.server 3000

alias q := quick

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes
@quick:
    echo "Serve the book (no link checking)"
    bash {{ quote(root + "/quick.sh") }} {{ quote(root) }}
