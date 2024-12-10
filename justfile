alias b := build
alias ba := buildall
alias bb := buildbook
alias ca := clippyall
alias f := fmtall
alias fa := fmtall
alias nta := nextestall
alias q := quick
alias s := serve
alias t := test
alias ta := testall

set windows-shell := [ "cmd.exe", "/c" ]

default:
  @just --list --unsorted
# or: @just --choose

## ---- CLEAN ------------------------------------------

# Clean Cargo's `target` and mdbook's `book` directories
clean: &&_clean
  cargo clean
  mdbook clean

[unix]
_clean:
  rm --recursive --force ./doctest_cache/

[windows]
_clean:
  if exist .doctest_cache rmdir /s /q .doctest_cache

## ---- CODE BUILDING -----------------------------------

# Format all code
fmtall:
  cargo +nightly fmt --all

# Check all code
checkall:
  cargo check --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Build the code used by the book (`deps` crate only`)
build:
  cargo build --package dependencies --tests --locked

# Build all code
buildall:
  cargo build --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
# optional: --timings

# Scan all code for common mistakes
clippyall:
  cargo clippy --workspace --all-targets --locked -- -D warnings

# Test the code used by the book (`deps` crate only)
test: _clean_temp_dir && _clean_temp_dir
  cargo test --package deps --tests --locked -- --show-output || true
# Only the code in the `deps` package is tested.
# This used to rely on skeptic to build doctests - see `build.rs` - but skeptic is slow
# NOTE: `mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/` is not reliable
# when dealing with dependencies outside of the std library.
# See: https://doc.rust-lang.org/rustdoc/command-line-arguments.html#-l--library-path-where-to-look-for-dependencies

# Test all code
testall: _clean_temp_dir && _clean_temp_dir
  cargo test --workspace --all-targets --locked || true
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Test all code using nextest
nextestall: _clean_temp_dir && _clean_temp_dir
  cargo nextest run --workspace --all-targets --locked --no-fail-fast || true
  cargo test --doc --workspace --quiet || true # nextest does not handle doctests

# Clean the `deps/temp` folder of most files prior / after testing
_clean_temp_dir:
  cargo run -p clean --quiet

# Run additional examples (under the `xmpl` folder)
[unix]
runall:
  ./scripts/runall.sh

## ---- CODE DOCUMENTATION -----------------------------------

# Build and display the `cargo doc` documentation for a specific package (e.g. deps)
[unix]
doc pkg:
  cargo clean --doc
  cargo doc --no-deps --document-private-items --locked --package {{pkg}}
  cd /cargo-target-rust_howto/target/doc/ ; python3 -m http.server 9000

_builddocall:
  cargo clean --doc
  cargo doc --no-deps --workspace --locked
# optional: --bins --examples

# Build and display the `cargo doc` documentation for all packages
[unix]
docall: _builddocall
  # cargo doc --open does not seem to work when running from a Dev Container in VS Code;
  # the script that opens URLs into an external browser (see `$ echo $BROWSER`) does not handle raw HTML.
  cd /cargo-target-rust_howto/target/doc/ ; python3 -m http.server 9000
  # We could also use `live server` for dynamic reloading.
  # See README.md for other alternatives, such as:
  # xdg-open /cargo-target-rust_howto/target/doc/deps/index.html

## ---- BOOK BUILDING -----------------------------------

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
buildbook: _generate-refdefs _generate-index-category _mdbook-build-book && _sitemap _copystatic

# Generate the book's HTML / JS
[unix]
_mdbook-build-book:
  ./scripts/build_book.sh

[windows]
_mdbook-build-book:
  mdbook build
# TODO P2

# Generate new reference definitions for all crate the book's examples depend on...
_generate-refdefs:
# TODO P3 mdbook-utils refdefs

# Generate the index and the category page.
_generate-index-category:
# TODO P2

# Add static assets to book output
[unix]
_copystatic:
  cp static/*.* book/html/

[windows]
_copystatic:
  copy static\*.* book\html\

# Generate the sitemap.xml file
_sitemap:
  mdbook-utils sitemap

# Serve the book (incl. link checking)
serve:
  mdbook serve -p 3000 -n 127.0.0.1 --open
  ## NOTE: can conflict with "port" / EXPOSE in the Docker / Docker Compose configuration
  ## Or use: cd book/html ; python3 -m http.server 3000

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes
[unix]
quick:
  ./scripts/quick.sh

[windows]
quick:
  echo "No support for quick serving while in Windows!"
  mdbook serve -p 3001 -n 127.0.0.1 --open

## ---- UTILITIES -----------------------------------

# Check spelling in markdown
[unix]
spell:
  .devcontainer/spellcheck.sh list

[windows]
spell:
  echo "Spell check is not implemented for Windows!"

help := 'help'
empty := ''

# Call mdbook-utils to manage links, ref definitions, etc...
utils cmd=help *subcmd=empty:
  mdbook-utils {{cmd}} {{subcmd}}
# TODO P2 clarify

# Run the templating tool e.g to create badges and reference definitions for a given crate or category
templ cmd=help *subcmd=empty:
  cargo run -p templ -- {{cmd}} {{subcmd}}
# TODO P2 clarify

# Create the `crates by category` and `crates (alphabetical)` pages
crate_indices cmd=help *subcmd=empty:
  cargo run -p crate_indices -- {{cmd}} {{subcmd}}
# TODO P2 clarify

# Autogenerate a chapter (from template)
autogen:
    cargo run -p autogen
# TODO P2 finish

## ---- LINK AND REFERENCE DEFINITION MANAGEMENT -----------------------------------

# Sort and deduplicate reference definitions in the central `*-refs.md` files; remove the last / from URLs
[unix]
sortrefs:
  ./scripts/refdefs/sort_refdefs.sh

[windows]
sortrefs:
  echo "`sortrefs` is not implemented while working in Windows!"

# List links without corresponding reference definitions and vice versa
check_refdefs:
  ./scripts/refdefs/check_refdefs.sh

# (BEWARE: modifies files directly) Add links to recipes to `<subchapter>.incl.md` files, using the local reference definitions in `refs.incl.md`
[confirm]
fix_recipe_tables:
  ./scripts/recipe_tables/fix_recipe_tables.sh

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links
lnk pattern:
  ./scripts/refdefs/search.sh {{pattern}}

## ---- ANCHOR MANAGEMENT -----------------------------------

# List book headings that do not have an anchor e.g. {#some-text}. Note that not all headers need one.
list_missing_anchors:
  ./scripts/anchors/list_missing_anchors.sh

# (BEWARE: modifies files directly) Generate reference definitions from heading anchors e.g. {#some-text} and add them to `refs.incl.md`
[confirm]
generate_refdefs_from_anchors:
  ./scripts/anchors/generate_refdefs_from_anchors.sh

## ---- URL MANAGEMENT -----------------------------------

# Check that URLs (to external websites) are valid and working
check_urls:
  ./scripts/urls/check_urls.sh

# List duplicated URLs (noting that they can't always be avoided).
list_duplicated_urls:
  ./scripts/urls/list_duplicated_urls.sh

# Outputs reference-style links and reference definitions to replace bare URLs found in the book's markdown (manual review necessary)
list_bare_urls:
  ./scripts/urls/list_bare_urls.sh
# TODO P2

## ---- CRATE MANAGEMENT -----------------------------------

# Update Cargo.lock dependencies for all projects (incl. the book's examples, tools, and additional examples in `xmpl`)
[confirm]
update:
  cargo update

# Get info about a crate (Rust 1.82+)
info crate:
  cargo info {{crate}}

# Check for security advisories, license issues, etc
check_crates:
  cargo deny check --hide-inclusion-graph
# WIP

## ---- CODE EXAMPLE MANAGEMENT -----------------------------------

# List examples (.rs files) in `deps/tests` that are not included in the book markdown.
list_examples_not_used_in_book:
  ./scripts/examples/list_examples_not_used_in_book.sh
# The script matches e.g. {{#include ../../../deps/tests/cats/development_tools_debugging/type_name_of_val.rs:example}}
# and extracts the file names
# then compare to the list of test files in `deps`
# A few files e.g. `main.rs` and `mod.rs` are not true examples and should not be included into the book.

# Convert {{#example <name>}} placeholders into ```rust... {#include ...}``` blocks and create the necessary code stubs (in subfolders of deps/tests/)
convert_example_placeholders:
  ./scripts/examples/convert_example_placeholders.sh

# List examples that were not included into a module somehow and that are not tested
list_examples_not_in_tests:
  #! /bin/bash
  comm -1 -3 <(find ./deps/tests -type f \( -name "main.rs" -o -name "mod.rs" \) -exec sed -nE 's/mod (\w*)\s*?;/\1/pg' {} \; | sort -u) \
          <(find ./deps/tests -mindepth 2 -type f -name "*.rs" -exec basename -s '.rs' {} \; | sed '/main/d; /mod/d' | sort -u)
  echo "DONE"

## ---- INCLUDE MANAGEMENT -----------------------------------

# Make sure that the local references i.e. {{#include refs.incl.md}} are included in every markdown file; note that a few files (indices, TOC...) don't need local references
list_missing_local_ref_includes:
  ./scripts/includes/list_missing_local_ref_includes.sh

# Make sure that a local TOC i.e. {{#include <subchapter>.incl.md}} is present in each subchapter
list_missing_subchapter_includes:
  ./scripts/includes/list_missing_subchapter_includes.sh

## ---- MAIN TOC MANAGEMENT -----------------------------------

# List (sub)chapters that somehow were not added in `SUMMARY.md`
list_missing_chapters_in_toc:
  ./scripts/main_table_of_contents/list_missing_chapters_in_toc.sh
# Usage: just list_missing_chapters_in_toc >> ./src/SUMMARY.md

## ---- INDICES MANAGEMENT -----------------------------------

# Outputs the contents of index of examples `src/examples_index.md` (reading the local tables of content of all subchapters)
generate_index_of_examples:
  ./scripts/index_of_examples/generate_index_of_examples.sh > src/examples_index.md
# Usage: just generate_index_of_examples

# Add, to `src/refs.incl.md`, missing references that are required for the index of examples (found in `examples_index.md`)
update_refdefs_for_index_of_examples:
  ./scripts/index_of_examples/update_refdefs_for_index_of_examples.sh
# Usage: just update_refdefs_for_index_of_examples

# Quick and dirty generation of language/index.md; manual editing required
generate_language_index:
  ./scripts/language/generate_language_index.sh
# Usage: generate_language_index >> src/language/index.incl.md

# Generate the summary references for the dev tool category
generate_dev_tools_refdefs:
  #! /bin/bash
  root="src/categories/development-tools/"
  mv -f ${root}refs.incl.md ${root}refs.incl.md.bak
  for file in $(find ${root} -mindepth 2 -type f -name "refs.incl.md")
  do
    rel=$(realpath --relative-to=${root} $file)
    rel=$(dirname $rel)'/'
    sed -E 's~^(\[.*\]:\s*?)(.*)$~\1'${rel}'\2~g' $file >> ${root}refs.incl.md
  done
  sort -u -o ${root}refs.incl.md ${root}refs.incl.md

## ---- CHAPTER MANAGEMENT -----------------------------------

# Hides some markdown sections/ pages by adding a _ prefix to all filenames listed in `hiddenfiles.txt`
[confirm]
hide:
  ./scripts/hide/hide.sh

# Make all markdown sections / pages visible
[confirm]
unhide:
  ./scripts/hide/unhide.sh

## ---- PRE-PUSH -----------------------------------

# Prepare for git push: spell sortrefs fmtall clean clippyall testall _builddocall buildbook
prep: spell sortrefs fmtall clean clippyall testall _builddocall buildbook

# Build the CI Docker image, then push it to DockerHub.
push_ci:
  #! /bin/bash
  ## Access private images in DockerHub
  docker login
  cd ./.devcontainer/
  docker buildx bake -f compose.yaml -f compose-ci.yaml --pull --push --allow=fs.read=..
  ## OR: docker compose -f ./.devcontainer/compose.yaml -f ./.devcontainer/compose-ci.yaml build --pull --push

# Push the development Docker image to DockerHub.
push_dev:
  docker push johncd/rust_howto_dev:latest
