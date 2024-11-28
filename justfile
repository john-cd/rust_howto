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
  #! /bin/bash
  set -o pipefail
  set -e
  # Create a list of the (last part of) folder names under the `xmpl` directory, space separated
  xmpl=$(find ./xmpl -mindepth 1 -maxdepth 1 -type d | awk -F'/' '{print $(NF)}' | tr '\n' ' ')
  # Run additional examples in the xmpl folder, if any
  for d in $xmpl; do ( echo $d; cargo run --package $d --locked ); done

## The examples under the deps folder have been replaced by tests.
## Should you want to add examples again, add the following to `runall`:
## Run examples that are simple .rs files in deps/examples
#examples=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type f | xargs basename --suffix=.rs | tr '\n' ' ')
#for e in $examples; do ( echo $e; cargo run --example $e --locked || true); done
## Run examples that are in a subfolder of deps/examples
#examples_in_dir=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type d | xargs basename --multiple | tr '\n' ' ')
#for e in $examples_in_dir; do ( echo $e; cargo run --example $e --locked || true ); done

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

# Generate the expanded markdown (input for skeptic) and the book's HTML / JS
[unix]
_mdbook-build-book:
  #! /bin/bash
  set -e
  if [ ! -f ./book.toml ]; then
    cp -f ./book.toml.bak ./book.toml
  fi
  mdbook build

[windows]
_mdbook-build-book:
  mdbook build

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
  #! /bin/bash
  set -o pipefail
  set -e
  # function called by trap
  cleanup() {
    cp -f ./book.toml.bak ./book.toml
    exit
  }
  trap cleanup 1 2 3 6
  if [ -f ./book.toml ]; then
    mv -f ./book.toml ./book.toml.bak
  fi
  # Make sure that the book builds in the same folder than `serve` - override
  # [build]
  # build-dir = "book/html"
  # Also overwrite the title
  MDBOOK_BUILD__BUILD_DIR="book/html" MDBOOK_BOOK='{"title": "QUICK SERVE"}' \
  mdbook serve -p 3001 -n 127.0.0.1 --open
# Note1: Using the env variable MDBOOK_* only seems to override existing values, not erase them.
# Examples:
#   MDBOOK_BOOK="$(toml2json ./book-dev.toml)" mdbook build
#   MDBOOK_OUTPUT__LINKCHECK='{"warning-policy": "ignore"}' MDBOOK_PREPROCESSOR__INDEXING='{"skip_renderer": "html,markdown,linkcheck"}'
# See the doc on overriding mdbook config: https://rust-lang.github.io/mdBook/format/configuration/environment-variables.html
#
# Note2: mdbook watch --open --watcher=poll / native does not have -p -n options.

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

# Create the `crates by category` and `crates (alphabetical)` pages
crate_indices cmd=help *subcmd=empty:
  cargo run -p crate_indices -- {{cmd}} {{subcmd}}

# Autogenerate a chapter (from template)
autogen:
    cargo run -p autogen
# TODO P2 finish

## ---- LINK AND REFERENCE DEFINITION MANAGEMENT -----------------------------------

# Sort and deduplicate reference definitions in the central `*-refs.md` files
[unix]
sortrefs: _removelastslash
  #! /bin/bash
  sort -u -o ./src/refs/crate-refs.md ./src/refs/crate-refs.md
  sort -u -o ./src/refs/other-refs.md ./src/refs/other-refs.md
  sort -u -o ./src/refs/link-refs.md ./src/refs/link-refs.md

[windows]
sortrefs:
  echo "`sortrefs` is not implemented while working in Windows!"

# Remove the last / from URLs in the reference definition files
[unix]
_removelastslash:
   #! /bin/bash
   sed -i 's/[/]$//g' ./src/refs/crate-refs.md
   sed -i 's/[/]$//g' ./src/refs/other-refs.md
   sed -i 's/[/]$//g' ./src/refs/link-refs.md

# List links without corresponding reference definitions and vice versa
check_refdefs:
  #! /bin/bash
  # reference definitions e.g. [label]: http://xyz
  grep -Proh '\[[^\[\]]+?\](?=:)' ./src ./drafts | sort -u > /tmp/defined_refdefs.txt
  # labels preceded by ] e.g. [some_text][label]
  grep -Proh '(?<=\])\[[^ \[\]]+?\]' ./src ./drafts | sort -u > /tmp/used_refdefs.txt
  comm -3 --check-order --output-delimiter="|" /tmp/defined_refdefs.txt /tmp/used_refdefs.txt | sort
  # Counts
  echo "Count of reference definitions without links and vice versa:" $(comm -3 --check-order --output-delimiter="|" /tmp/defined_refdefs.txt /tmp/used_refdefs.txt  | wc -l)
  echo "Count of reference definitions defined in the refs folder:" $(cat  /tmp/defined_refdefs.txt | wc -l)
  echo "Count of reference definitions used in the markdown:" $(cat  /tmp/used_refdefs.txt | wc -l)
# grep -r = recursive, h = no-filename, P = perl regex, o = only-matching
# [a-zA-Z0-9\._:-]

# (BEWARE: can be destructive) Add links to recipes to `<subchapter>.incl.md` files, using the local reference definitions in `refs.incl.md`
[confirm]
fix_recipe_tables:
  #! /bin/bash
  for file in $(find ./src -type f -name "*.md" -not -name "*.incl.md" -not -name "*index.md" -not -name "*refs.md")
  do
    base=$(basename $file)
    dir=$(dirname $file)
    if [ -f "${dir}/refs.incl.md" ]; then
      # grab the labels of the refdefs for the current file
      labels=$(sed -En 's/^\[ex-(.*)\]:\s?'${base}'.*$/\1/p' ${dir}/refs.incl.md)
      # if not empty
      if [ -n "$labels" ]; then
          echo "> ${file}"
          for label in ${labels}
          do
            # if the dest file does not exist or label is not in it
            if [ ! -f "${file%.md}.incl.md" ] || [ $(grep -Pc "\[ex-${label}\]" "${file%.md}.incl.md") -eq 0 ]
            then
              # add link in the corresponding .incl.md (manual editing of the table is still necessary)
              echo "[${label}][ex-${label}]" >> "${file%.md}.incl.md"
            fi
          done
      fi
    fi
  done

# Search the references using a crate name or label fragment and return the  refdefs / URLs and reference-style links
lnk pattern:
  #! /bin/bash
  rg -IN '\[(c-)?[^]]*{{pattern}}[^]]*\].*' ./src/refs
  rg -IN -r'[`$2`][$1$2$3]⮳' '\[(c-)?([^]]*{{pattern}}[^]-]*)([^]]*)\]:\s?(.*)' ./src/refs
#  -N = --no-line-number; -I = --no-filename

## ---- ANCHOR MANAGEMENT -----------------------------------

# List headings that do not have an anchor e.g. {#some-text}. Note that not all headers need one.
list_missing_anchors:
  #! /bin/bash
  for file in $(find ./src -type f \( -name "*.md" -not -name "*index.md" -not -wholename "./src/crates/*.md" \) )
  do
    # grab headings without {, ignoring "## See also", etc...
    header=$(grep -P '^##[^{]+$' $file | sed 's/## See [aA]lso//g')
    if [ -n "$header" ]; then
      echo $file" --> "$header
    fi
  done

# (BEWARE: can be destructive) Generate reference definitions from heading anchors e.g. {#some-text} and add them to `refs.incl.md`
[confirm]
generate_refdefs_from_anchors:
  #! /bin/bash
  for file in $(find ./src -type f \( -name "*.md" -not -name "*index.md" \) )
  do
    echo "> $file"
    base=$(basename $file)
    # parent directory
    parent=$(dirname $file | xargs basename)
    # grab all headings with an anchor; substitute the anchor \1 into a refdef
    link=$(sed -nE 's/^#.*\{#(.+?)\}\s*$/[ex-'${parent}'-\1]: '$base'#\1/p' $file)
    if [ -n "$link" ]; then
      echo "$link" >> "${file%/*}/refs.incl.md"
      # sort and dedupe refdefs
      sort -u -o "${file%/*}/refs.incl.md" "${file%/*}/refs.incl.md"
    fi
  done

## ---- URL MANAGEMENT -----------------------------------

# Check that URLs (to external websites) are valid and working
check_urls:
  -lychee --exclude-all-private --no-ignore --hidden --format detailed --cache "./**/*.md" "./**/*.toml" "./**/*.yaml" "./**/*.yml"
  # We could also check ".devcontainer/*" "./**/*.sh"
  sed -r 's/\[.+?\]: (.+)$/\1/' ./src/refs/*.md | lychee --exclude-all-private --format=detailed --cache -u "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36" -- -
# Somehow lychee ignores links in markdown reference definitions... thus the use of sed to extract URLs
# This does not check whether the reference definitions are used - see below.

# Identify duplicated URLs (noting that they can't always be avoided).
list_duplicated_urls:
  sed -r 's/\[.+?\]: (.+)$/\1/' ./src/refs/*.md | sort | uniq --repeated --count
# -r or -E = use extended regular expressions

# Create a reference definition for bare URLs in the markdown (manual review necessary)
convert_bare_urls:
  #! /bin/bash
  rg --pcre2 --no-line-number --no-filename --only-matching '(?<!: |["`([])(http(?:s)?://(?:www\d?\.|github\.com/)?)([^./]+)(\S+)?' ./src \
    -g '*.md' -g '!*refs.md' -g 'refs.incl.md' -r '[$2-website]: $1$2$3' | sort | sed 's~/$~~'
  rg --pcre2 --no-line-number --no-filename --only-matching '(?<!: |["`([])(http(?:s)?://(?:www\d?\.|github\.com/)?)([^./]+)(\S+)?' ./src \
    -g '*.md' -g '!*refs.md' -g 'refs.incl.md' -r '[`$2`][$2-website]' | sort | sed 's~/$~~'

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
  #! /bin/bash
  grep -Proh '\{\{#include .+?\.rs(:.+?)?\}\}' ./src ./drafts | sed -E 's~\{\{#include .+/([._a-zA-Z0-9]+?\.rs)(:.+?)?\}\}~\1~' | sort -u > examples_in_markdown.txt
  find ./deps/tests -type f -name "*.rs" -exec basename {} \; | sort -u > examples.txt
  comm -13 examples_in_markdown.txt examples.txt
# The script matches e.g. {{#include ../../../deps/tests/cats/development_tools_debugging/type_name_of_val.rs:example}} and extracts the file names
# then compare to the list of test files in deps
# A few files e.g. `main.rs` and `mod.rs` are not true examples and should not be included into the book.

## ---- INCLUDE MANAGEMENT -----------------------------------

# Make sure that the local references i.e. {{#include refs.incl.md}} are included in every file
list_missing_local_ref_includes:
  grep -PrL --exclude=*.incl.md --exclude=*refs.md '\{\{#include refs.incl.md\}\}' ./src
# NOTE: a few files (indices, TOC...) don't need local references

# Make sure that a local TOC i.e. {{#include <subchapter>.incl.md}} is present in each subchapter
list_missing_subchapter_includes:
  #! /bin/bash
  for file in $(find ./src -type f -name "*.md" -not -name "*index.md" -not -name '*.incl.md' -not -name "*refs.md" )
  do
    base=$(basename $file)
    include=$(grep -Poh '(?<=\{\{#include )[_]?'${base%.md}'(?=\.incl\.md\}\})' $file)
    if [ -z "$include" ]; then
      echo $file" -- consider adding --> {{{{#include "${base%.md}".incl.md}}"
    fi
  done

## ---- MAIN TOC MANAGEMENT -----------------------------------

# List (sub)chapters that somehow were not added in SUMMARY.md
list_missing_chapters_in_toc:
  #! /bin/bash
  for file in $(find ./src -type f -name "*.md" -not -name '*.incl.md' -not -name "*refs.md" -not -name "SUMMARY.md")
  do
    rel=$(realpath --relative-to=./src $file)
    in_toc=$(grep -Poh ${rel} ./src/SUMMARY.md)
    if [ -z "$in_toc" ]; then
      base=$(basename $file | awk 'BEGIN{split("a the to at in on with and but or",w); for(i in w)nocap[w[i]]}function cap(word){return toupper(substr(word,1,1)) tolower(substr(word,2))}{for(i=1;i<=NF;++i){printf "%s%s",(i==1||i==NF||!(tolower($i) in nocap)?cap($i):tolower($i)),(i==NF?"\n":" ")}}')
      echo "- ["${base%.md}"]("$rel")"
    fi
  done

## ---- INDICES MANAGEMENT -----------------------------------

# Quick and dirty generation of language/index.md; manual editing required
generate_lang_index:
  #! /bin/bash
  clear
  for file in $(find ./src/language -type f -name "*.md" -not -name '*.incl.md' -not -name "*refs.md" -not -name "index.md")
  do
    base=$(basename $file)
    title=$(echo ${base%.md} | sed 's/.*/\L&/; s/[a-z]*/\u&/g; s/_/ /g')
    echo "| [$title][ex-lang-${base%.md}] |"
  done

## ---- PRE-PUSH -----------------------------------

# Prepare for git push: spell sortrefs fmtall clean clippyall testall _builddocall buildbook
prep: spell sortrefs fmtall clean clippyall testall _builddocall buildbook

# Build the CI Docker image, then push it to DockerHub. The GitHub Action CI workflow will use the Dockerhub image as a cache.
push_ci:
  docker compose -f ./.devcontainer/compose.yaml -f ./.devcontainer/compose-ci.yaml build
  docker push johncd/rust_howto_ci:latest

push_dev:
  docker push johncd/rust_howto_dev:latest
