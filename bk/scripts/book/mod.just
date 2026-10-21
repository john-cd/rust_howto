set unstable
set windows-shell := ["pwsh.exe", "-NoLogo", "-NoProfile", "-Command"]

root := clean(source_directory() + "/../..")
mdbook_utils_rev := env("MDBOOK_UTILS_REV", "dedddf3699e461cd418c1a044d82aabd3fec62e0")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- BOOK BUILDING -----------------------------------

alias bb := buildbook

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
@buildbook: _release_mbdbook-scrub refdefs-check _generate-index-category _mdbook-build-book && _sitemap _copystatic

# Generate the book's HTML / JS
[unix]
@_mdbook-build-book:
  echo "Build book in {{root}}"
  cd {{root}}; {{source_directory()}}/build_book.sh {{root}}

[windows]
@_mdbook-build-book:
  echo "Build book in {{root}}"
  cd {{root}}; mdbook build {{root}}

# Regenerate only the owned dependency-reference fragment.
[unix]
@refdefs-generate: _install-mdbook-utils
  python3 "{{source_directory()}}/refdefs.py" generate --book-root "{{root}}"

[windows]
@refdefs-generate: _install-mdbook-utils
  python "{{source_directory()}}/refdefs.py" generate --book-root "{{root}}"

# Check generated refs without rewriting authoring files.
[unix]
@refdefs-check: _install-mdbook-utils
  python3 "{{source_directory()}}/refdefs.py" check --book-root "{{root}}"

[windows]
@refdefs-check: _install-mdbook-utils
  python "{{source_directory()}}/refdefs.py" check --book-root "{{root}}"

@_install-mdbook-utils:
  cargo install --git https://github.com/john-cd/mdbook-utils --rev {{mdbook_utils_rev}} --locked mdbook-utils

# Generate the external crates.io category page from its checked-in snapshot.
[unix]
@_generate-index-category:
  #!/usr/bin/env bash
  set -euo pipefail
  cd "{{root}}/../tools"
  tmp="$(mktemp)"
  trap 'rm -f "$tmp"' EXIT
  cat "{{root}}/master/crates.txt" | cargo run --locked -p crate_indices -- category_page_from_metadata --metadata "{{root}}/master/crate_categories.toml" > "$tmp"
  mv "$tmp" "{{root}}/src/indices/crates_by_category.md"

[windows]
@_generate-index-category:
  pwsh.exe -NoLogo -NoProfile -File "{{source_directory()}}/generate_category_index.ps1" -BookRoot "{{root}}" -ToolsRoot "{{root}}/../tools"
# [generate index of crates by category](https://github.com/john-cd/rust_howto/issues/1274)

# Add static assets to book output
[unix]
@_copystatic:
  echo "Copy static files"
  cp {{root}}/static/*.* {{root}}/book/html/

[windows]
@_copystatic:
  echo "Copy static files"
  copy {{root}}\static\*.* {{root}}\book\html\

# Generate the sitemap.xml file
[unix]
@_sitemap:
  echo "Generate the sitemap"
  cd {{root}} && mdbook-utils sitemap

[windows]
@_sitemap:
  echo "Generate the sitemap"
  cd {{root}} && mdbook-utils sitemap

# Build / release the tools, incl. mdbook-scrub preprocessor, if needed
[unix]
@_release_mbdbook-scrub:
  echo "Build mdbook-scrub, if needed"
  if [ ! -f {{root}}/../bin/mdbook-scrub ] || [ ! -f {{root}}/../bin/mdbook-private ] || [ ! -f {{root}}/../bin/mdbook-linkcheck-compat ]; then just -f {{root}}/../tools/justfile release; fi

[windows]
@_release_mbdbook-scrub:
  echo "Build mdbook-scrub, if needed"
  if (-not (Test-Path {{root}}\..\bin\mdbook-scrub.exe) -or -not (Test-Path {{root}}\..\bin\mdbook-private.exe) -or -not (Test-Path {{root}}\..\bin\mdbook-linkcheck-compat.exe)) { just -f {{root}}\..\tools\justfile release }

## ---- BOOK SERVING -----------------------------------

alias s := serve

# Serve the book (incl. link checking)
@serve:
  echo "Serve the book"
  cd {{root}} && mdbook serve -p 3000 -n 127.0.0.1 --open {{root}}
  ## NOTE: can conflict with "port" / EXPOSE in the Docker / Docker Compose configuration
  ## Or use: cd {{root}}/book/html ; python3 -m http.server 3000

alias q := quick

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes
[unix]
@quick:
  echo "Serve the book (no link checking)"
  cd {{root}} && ./quick.sh {{root}}

[windows]
@quick: && serve
  echo "No support for quick serving while in Windows!"
