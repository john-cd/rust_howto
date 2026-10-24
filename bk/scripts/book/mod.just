# TODO Reviewed ??-??-2026

set unstable := true
set windows-shell := ["pwsh.exe", "-NoLogo", "-NoProfile", "-Command"]

root := clean(source_directory() + "/../..")
mdbook_utils_rev := env("MDBOOK_UTILS_REV", "dedddf3699e461cd418c1a044d82aabd3fec62e0")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ source_file() }}

## ---- BOOK BUILDING -----------------------------------

alias bb := buildbook

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
@buildbook: _release_mbdbook-scrub refdefs-check _generate-index-category _mdbook-build-book && _sitemap _copystatic

# Generate the book's HTML / JS
[unix]
@_mdbook-build-book: _install-mdbook-indexing
    echo "Build book in {{ root }}"
    cd {{ quote(root) }}; {{ quote(source_directory() + "/build_book.sh") }} {{ quote(root) }}

[windows]
@_mdbook-build-book: _install-mdbook-indexing
    echo "Build book in {{ root }}"
    Set-Location -LiteralPath {{ quote(root) }}; mdbook build {{ quote(root) }}

# Regenerate only the owned dependency-reference fragment.
[unix]
@refdefs-generate: _install-mdbook-utils
    python3 "{{ source_directory() }}/refdefs.py" generate --book-root "{{ root }}"

[windows]
@refdefs-generate: _install-mdbook-utils
    python "{{ source_directory() }}/refdefs.py" generate --book-root "{{ root }}"

# Check generated refs without rewriting authoring files.
[unix]
@refdefs-check: _install-mdbook-utils
    python3 "{{ source_directory() }}/refdefs.py" check --book-root "{{ root }}"

[windows]
@refdefs-check: _install-mdbook-utils
    python "{{ source_directory() }}/refdefs.py" check --book-root "{{ root }}"

@_install-mdbook-utils:
    cargo install --git https://github.com/john-cd/mdbook-utils --rev {{ mdbook_utils_rev }} --locked mdbook-utils

# Install the pinned word-index preprocessor when a developer renders outside CI.
mdbook_indexing_version := env("MDBOOK_INDEXING_VERSION", "0.2.0")

[unix]
@_install-mdbook-indexing:
    cargo install --locked --version {{ mdbook_indexing_version }} mdbook-indexing

[windows]
@_install-mdbook-indexing:
    cargo install --locked --version {{ mdbook_indexing_version }} mdbook-indexing

# Generate the external crates.io category page from its checked-in snapshot.
[unix]
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

[windows]
@_generate-index-category:
    pwsh.exe -NoLogo -NoProfile -File {{ quote(source_directory() + "/generate_category_index.ps1") }} -BookRoot {{ quote(root) }} -ToolsRoot {{ quote(root + "/../tools") }}

# [generate index of crates by category](https://github.com/john-cd/rust_howto/issues/1274)
# [link crates to published recipes](https://github.com/john-cd/rust_howto/issues/28)

# Generate the alphabetical crate index with links to published recipes.
[unix]
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

[windows]
@_generate-index-alphabetical:
    pwsh.exe -NoLogo -NoProfile -File {{ quote(source_directory() + "/generate_alphabetical_index.ps1") }} -BookRoot {{ quote(root) }} -ToolsRoot {{ quote(root + "/../tools") }}

# [link crates to published recipes](https://github.com/john-cd/rust_howto/issues/27)

# Add static assets to book output
[unix]
@_copystatic:
    echo "Copy static files"
    cp {{ quote(root + "/static") }}/*.* {{ quote(root + "/book/html") }}/

[windows]
@_copystatic:
    echo "Copy static files"
    Copy-Item -Path {{ quote(root + "/static/*.*") }} -Destination {{ quote(root + "/book/html") }}

# Generate the sitemap.xml file
[unix]
@_sitemap:
    echo "Generate the sitemap"
    cd {{ quote(root) }} && mdbook-utils sitemap

[windows]
@_sitemap:
    echo "Generate the sitemap"
    Set-Location -LiteralPath {{ quote(root) }}; mdbook-utils sitemap

# Build / release the tools, incl. mdbook-scrub preprocessor, if needed
[unix]
@_release_mbdbook-scrub:
    echo "Build mdbook-scrub, if needed"
    if [ ! -f {{ quote(root + "/../bin/mdbook-scrub") }} ] || [ ! -f {{ quote(root + "/../bin/mdbook-private") }} ] || [ ! -f {{ quote(root + "/../bin/mdbook-linkcheck-compat") }} ]; then just -f {{ quote(root + "/../tools/justfile") }} release; fi

[windows]
@_release_mbdbook-scrub:
    echo "Build mdbook-scrub, if needed"
    if (-not (Test-Path -LiteralPath {{ quote(root + "/../bin/mdbook-scrub.exe") }}) -or -not (Test-Path -LiteralPath {{ quote(root + "/../bin/mdbook-private.exe") }}) -or -not (Test-Path -LiteralPath {{ quote(root + "/../bin/mdbook-linkcheck-compat.exe") }})) { just -f {{ quote(root + "/../tools/justfile") }} release }

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
[unix]
@quick:
    echo "Serve the book (no link checking)"
    cd {{ quote(root) }} && {{ quote(root + "/quick.sh") }} {{ quote(root) }}

[windows]
@quick:
    Write-Error "Quick serving is not supported on Windows; use 'just s' instead."
    exit 1
