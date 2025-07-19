# TODOs

## Now

- language section: write / finish
  - finish to add RBE link #29

- polish std_lib

- polish contributing

- polish links

- finish new tools in wip folder
  - parser_lib
  - inline_links
  - process_directives
  - link_checker

- Close NOW tickets: WIP
- Go trough VS Code bookmarks (WSL): WIP
- Go trough VS Code bookmarks (dev container): WIP

- crate_selection - solo or part of indices?

- move categories back to src, after final review
  - algos
  - data structures
  - api bindings
  - memory
  - caching
  - config
  - date and time
  - cli
  - command-line utils ~ dedupe with written_in_rust
  - compression
  - email
  - math
  - ML
  - database impl
  - encoding
  - filesystem
  - template
  - text processing
  - HTTP client
  - build utils
  - dev tools
  - async
  - concurrency
  - database
  - os
  - parser impl
  - parsing
  - rust patterns
  - crypto
  - auth

- polish intro
  - review `about` examples

- `other` section - move what's ready



- polish thanks
  - License / legal rethink

- indices
  - script for crates_and_examples
  - rethink crates_alpha and categories
  - categories.md

- move WIP examples from playground

- finish
  - auth

- move rest of links from TOREVIEW.md

## Finish examples

- fake
- tracing_subscriber
- flate2
- salsa
- crux
- cssparser
- quickxml

---

## Finish pandoc setup; generate PDF version of book WIP

- pandoc / typst
- kindle create

<https://gumroad.com>
<https://pianomanfrazier.com/post/write-a-book-with-markdown/#bookdown-https-bookdown-org-yihui-bookdown>
<https://pdworkman.com/write-book-with-obsidian/#getting-it-out-of-obsidian>
<https://medium.com/@sydasif78/book-creation-with-pandoc-and-markdown-893c7d72cb35>

---

## sccache for dev container setup?

---

## Review tools and consolidate WIP

- crate-indices WIP
- autogen - in playground
- links
- templ
- tool_lib

document WIP
add tests WIP
consolidate CLIs

## Finish preproc directives in mdbook-scrub

- Scrub any left-over {{#example ...}}, {{#crate ...}}... etc and warn. WIP
- Scrub links to hidden pages instead of having to comment e.g. [p~cross-platform]: # "../../other/cross-platform/index.md"

## Implement directives (in `link`?)

- Crate link
- Crate badge - WIP link tool
- Category link?
- Category badges
- Crate blocks
- Recipe table

## Replace inline links and naked URLs by ref-style links

- fix inline_link script and/or finish pest parser (currently in playground)

## Update recipe tables

- improve scripts that generate links / update recipe tables

---

## Incoporate the added text in src WIP

- src: reorg added text WIP

## Incoporate the added text in drafts

- drafts: Finish
  - Containers
  - development tools: FFI - reorg. with other FFI topics

- other:
  - cross-platform (partially done)
  - data proc,
  - gpu,
  - scripting (partially done)
  - written in rust (partially done)

- review drafts section for what I missed

## Add text

- language - WIP
- standard library WIP
- links
- key crates
- contributing

## Manually insert rest of cross-links WIP

- src: WIP

## Replace ⮳ by ↗ ?

<https://fontawesome.com/icons/external-link?f=classic&s=light>

<https://gist.github.com/miguelmota/322c89234d60de578f37d3c6d30f7e41#file-external_link_arrow-txt>

## Chrome Bookmarks

- finish to review chrome bookmarks and include in text

## Finish COMING SOON examples

- parser impl
- crypto
- database
- proc macros
- pyo3, ractor, polars, datafusion, aws_lambda, aws_sdk

## Clean up playground crate

## Move mdbook-utils repo ?

## Setup bacon

## Add git hooks to automate formatting / clippy check / fix before commit

Try `cargo-husky`

[dev-dependencies.cargo-husky]
version = "1.5.0"
default-features = false
features = ["user-hooks"]

Note that, when user-hooks feature is enabled, other all features are disabled.
You need to prepare all hooks in .cargo-husky/hooks directory
See [cargo-husky](https://lib.rs/crates/cargo-husky)

See scripts/precommit folder

---

## Increase speed of CI build (Linux)

### Parallelism

Split ci.sh?
Run cargo build / clippy / nextest in separate CI steps or jobs for coarse parallelism

### Try to move target and/or /usr/local/cargo to a docker volume instead of docker container FS

Metric:

```sh
time dd if=/dev/zero of=test.dat bs=1024 count=100000
```

Writes are fast when writing to container FS, volume, but slow on bind mount (to Windows). Need to test on Linux

### Try to store target and/or /usr/local/cargo in a tmpfs?

/usr/local/cargo is small enough to fit in memory (16GB max)

[how-to-use-tmpfs-and-its-functions-in-docker](https://linuxhaxor.net/code/how-to-use-tmpfs-and-its-functions-in-docker.html)

### Make gha caching work on CI

Try local / inline caching as well

[optimize-docker-builds-github-actions-cache](https://cicube.io/blog/optimize-docker-builds-github-actions-cache/)

### Try to build directly on the host, not in a container?

Write a .yml build workflow file for Linux

---

## Free further space on CI runner

Already using the `free-disk-space-ubuntu` action

[2875](https://github.com/actions/runner-images/issues/2875)

[free-disk-space-ubuntu](https://github.com/marketplace/actions/free-disk-space-ubuntu)

See splitting into separate CI steps or jobs above

---

## Create CI build / tests on Windows

Add Windows build job to main workflow?

Should we build in Docker via `cargo build --target [<TRIPLE>]` ?
Use cross?

---

## Make `just` commands fully work on Windows

- make `just` commands fully work on Windows
  - use [script]?
  - make cygwin bash work on Windows
- consider cargo make / xtask?

---

## Review the need for rusty_fork

since we use nextest exclusively
See nextest execution model: <https://nexte.st/docs/design/how-it-works/>

---

## CI build on MacOS too?

Create a .yml file for MacOS build on GitHub runner?
Add build job to main workflow

---

## Fix .github issue templates

---

## Build rust_rate tool

- retrieve most downloaded / recently downloaded / most relevant crates from crates.io / lib.rs
- collect other sources of ranking: r/rust, awesome-rust, etc
- generate a list of crates that should be included in the book
