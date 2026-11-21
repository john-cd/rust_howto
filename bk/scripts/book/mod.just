# Purpose: Build, serve, and maintain the generated book, its indexes, and references.
# Usage: From bk/, run just book <recipe>.
#
# TODO Reviewed ??-??-2026.

set unstable
set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

[unix]
set script-interpreter := ["bash", "-eu"]

[windows]
set script-interpreter := ["C:/Program Files/Git/bin/bash.exe", "-eu"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")
# Executable suffix and Python launcher differ on Windows; recipes themselves run in Bash everywhere.
exe := if os_family() == "windows" { ".exe" } else { "" }
python := if os_family() == "windows" { "python" } else { "python3" }
mdbook_indexing_version := env("MDBOOK_INDEXING_VERSION", "0.2.0")

# ---- Task aliases -----------------------------------------------------------

alias bb := buildbook
alias s := serve
alias q := quick

# ---- Book build and preview -------------------------------------------------

# List the available recipes and task modules.
[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- BOOK BUILDING -----------------------------------

# Render future chapters with their own summary and the shared reference definitions.
@future: _release_mbdbook-scrub
    mdbook build {{ quote(root + "/../later") }} --dest-dir {{ quote(root + "/book/future") }}
    @just --justfile "{{ source_file() }}" audit-preview-links

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
@buildbook: _release_mbdbook-scrub _install-mdbook-indexing refdefs-check _generate-index-category audit-visible-todos
    just --justfile "{{ source_directory() }}/../bash/mod.just" mdbook-build
    just --justfile "{{ source_directory() }}/../bash/mod.just" sitemap
    just --justfile "{{ source_directory() }}/../bash/mod.just" copystatic
    @just --justfile "{{ source_file() }}" audit-rendered-links
    @just --justfile "{{ source_file() }}" audit-word-index
    @just --justfile "{{ source_file() }}" audit-playground
    @just --justfile "{{ source_file() }}" audit-search-index

# Reject visible TODO placeholders in the license and published Markdown.
@audit-visible-todos:
    cargo run --manifest-path "{{ root }}/../tools/Cargo.toml" --locked -p book-audit -- --repo-root "{{ root }}/.." visible-todos

# Check all required rendered chapters, local links, and fragments.
@audit-rendered-links:
    cargo run --manifest-path "{{ root }}/../tools/Cargo.toml" --locked -p book-audit -- --repo-root "{{ root }}/.." rendered-links bk/book/html --all-chapters

# Check links emitted by the word-index page.
@audit-word-index:
    cargo run --manifest-path "{{ root }}/../tools/Cargo.toml" --locked -p book-audit -- --repo-root "{{ root }}/.." word-index bk/book/html

# Check runnable Playground imports against the pinned crate list.
@audit-playground:
    cargo run --manifest-path "{{ root }}/../tools/Cargo.toml" --locked -p book-audit -- --repo-root "{{ root }}/.." playground bk/book/html --book-root bk

# Check the generated mdBook search index for visible terms and excluded chapters.
@audit-search-index:
    cargo run --manifest-path "{{ root }}/../tools/Cargo.toml" --locked -p book-audit -- --repo-root "{{ root }}/.." search-index --book-root bk

# Check the staged future preview, optionally including deferred categories.
@audit-preview-links include_deferred="false":
    if [ "{{ include_deferred }}" = "true" ]; then \
        cargo run --manifest-path "{{ root }}/../tools/Cargo.toml" --locked -p book-audit -- --repo-root "{{ root }}/.." preview-links bk/book/future --include-deferred; \
    else \
        cargo run --manifest-path "{{ root }}/../tools/Cargo.toml" --locked -p book-audit -- --repo-root "{{ root }}/.." preview-links bk/book/future; \
    fi

# Generate raw dependency references with `mdbook-utils`, then normalize labels and merge them
# into the curated crate reference file while preserving existing destinations.
@refdefs-generate: _install-mdbook-utils
    {{ python }} "{{ source_directory() }}/refdefs.py" generate --book-root "{{ root }}"

# Check dependency references using the same normalized merge as generation, without writing files.
@refdefs-check: _install-mdbook-utils
    {{ python }} "{{ source_directory() }}/refdefs.py" check --book-root "{{ root }}"
    {{ python }} "{{ source_directory() }}/../indices/update_refdefs_for_examples.py" check --book-root "{{ root }}"

# Use the pinned mdbook-utils revision installed by CI, or install it locally.
@_install-mdbook-utils:
    if [ "${MDBOOK_TOOLS_PREINSTALLED:-false}" = true ]; then \
        command -v mdbook-utils >/dev/null; \
    else \
        {{ python }} "{{ source_directory() }}/refdefs.py" install; \
    fi

# Use the pinned indexing preprocessor installed by CI, or install it locally.
@_install-mdbook-indexing:
    if [ "${MDBOOK_TOOLS_PREINSTALLED:-false}" = true ]; then \
        command -v mdbook-indexing >/dev/null; \
    else \
        cargo install --locked --version {{ mdbook_indexing_version }} mdbook-indexing; \
    fi

# Generate the external crates.io category page from its checked-in snapshot.
[script]
@_generate-index-category:
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
[script]
@_generate-index-alphabetical:
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

# Serve the book (incl. link checking)
@serve: _install-mdbook-indexing
    echo "Serve the book"
    cd {{ quote(root) }} && mdbook serve -p 3000 -n 127.0.0.1 --open {{ quote(root) }}
    ## NOTE: can conflict with "port" / EXPOSE in the Docker / Docker Compose configuration
    ## Or use: cd {{ root }}/book/html ; python3 -m http.server 3000

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes
@quick:
    echo "Serve the book (no link checking)"
    bash {{ quote(root + "/quick.sh") }} {{ quote(root) }}
