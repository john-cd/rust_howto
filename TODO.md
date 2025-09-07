# TODOs

## Now

- polish stack-allocated

- polish directory_traversal

- polish hashing + algorithms

- polish learning.md

- polish crate_selection.

- polish contributing.

- finish to add RBE link #29.

- Go trough VS Code bookmarks - WIP.

- Manually insert rest of cross-links between pages?

- License / legal rethink.

- Indices.
  - script for crates_and_examples.
  - rethink crates_alpha and categories.
  - categories.md.

## Update recipe tables

- improve scripts that generate links / update recipe tables.

## Additional Links / Contents

- move rest of links from TOREVIEW.md.
- finish to review chrome bookmarks and include in text.

## Finish pandoc / typst setup; generate PDF version of book

- pandoc / typst.
- kindle create.

[gumroad~website][gumroad~website]

[gumroad~website]: https://gumroad.com

[blog~write-a-book-with-markdown]

[blog~write-a-book-with-markdown]: <https://pianomanfrazier.com/post/write-a-book-with-markdown/#bookdown-https-bookdown-org-yihui-bookdown>

[write-book-with-obsidian]

[write-book-with-obsidian]: <https://pdworkman.com/write-book-with-obsidian/#getting-it-out-of-obsidian>

[book-creation-with-pandoc-and-markdown]

[book-creation-with-pandoc-and-markdown]: <https://medium.com/@sydasif78/book-creation-with-pandoc-and-markdown-893c7d72cb35>

### Finish preproc directives in mdbook-scrub

- Scrub any left-over {{#example ...}}, {{#crate ...}}... etc and warn. WIP.
- Scrub links to hidden pages instead of having to comment e.g. [p~cross-platform]: # "../../other/cross-platform/index.md".

### Implement directives

- Crate link.
- Crate badge - WIP link tool.
- Category link?
- Category badges.
- Crate blocks.
- Recipe table.

## Review tools and consolidate WIP

- crate-indices WIP.
- autogen - in playground.
- links.
- templ.
- tool_lib.

document WIP.
add tests WIP.
consolidate CLIs.

- finish new tools in book_tooling
  - parser_lib.
  - inline_links.
  - process_directives.
  - link_checker.

-------------------------

## Finish COMING SOON examples

- parser impl.
- crypto.
- database.
- proc macros.
- pyo3, ractor, polars, datafusion, aws_lambda, aws_sdk.

- fake.
- tracing_subscriber.
- flate2.
- salsa.
- crux.
- cssparser.
- quickxml.

- review cancelable example in xmpl.

- move WIP examples from playground.

## Incoporate the added text in drafts

- drafts: Finish.
  - Containers.
  - development tools: FFI - reorg. with other FFI topics.

- other:
  - cross-platform (partially done).
  - data proc,
  - gpu,
  - scripting (partially done).
  - written in rust (partially done).

- review drafts section for what I missed.

## Additional Categories

- move categories back to src, after final review.
  - api bindings ?
  - memory ?
  - caching ?
  - config ?
  - date and time ?
  - cli ?
  - command-line utils ~ dedupe with written_in_rust ?
  - compression ?
  - email ?
  - math.
  - ML ?
  - database impl ?
  - encoding ?
  - template ?
  - text processing.
  - HTTP client ?
  - build utils ?
  - dev tools ?
  - async ?
  - concurrency ?
  - database ?
  - os ?
  - parser impl ?
  - parsing ?
  - rust patterns ?
  - crypto ?
  - auth ?

- `other` section - move what's ready.

## sccache for dev container setup?

## Clean up playground crate

## Move mdbook-utils repo ?

## Setup bacon

## Add git hooks to automate formatting / clippy check / fix before commit

Try `cargo-husky`

```toml
[dev-dependencies.cargo-husky]
version = "1.5.0".
default-features = false.
features = ["user-hooks"].
```

Note that, when user-hooks feature is enabled, other all features are disabled.
You need to prepare all hooks in `.cargo-husky/hooks` directory.
See [`cargo-husky`][c~cargo-husky~docs]↗{{hi:cargo-husky}}.

See scripts/precommit folder.

## Increase speed of CI build (Linux)

### Parallelism

Split ci.sh?
Run cargo build / clippy / nextest in separate CI steps or jobs for coarse parallelism.

### Try to move target and/or /usr/local/cargo to a docker volume instead of docker container FS

Metric:

```sh
time dd if=/dev/zero of=test.dat bs=1024 count=100000.
```

Writes are fast when writing to container FS, volume, but slow on bind mount (to Windows). Need to test on Linux.

### Try to store target and/or /usr/local/cargo in a tmpfs?

`/usr/local/cargo` is small enough to fit in memory (16GB max).

[how-to-use-tmpfs-and-its-functions-in-docker][how-to-use-tmpfs-and-its-functions-in-docker]

[how-to-use-tmpfs-and-its-functions-in-docker]: https://linuxhaxor.net/code/how-to-use-tmpfs-and-its-functions-in-docker.html

### Make gha caching work on CI

Try local / inline caching as well.

[optimize-docker-builds-github-actions-cache][optimize-docker-builds-github-actions-cache]

[optimize-docker-builds-github-actions-cache]: https://cicube.io/blog/optimize-docker-builds-github-actions-cache

### Try to build directly on the host, not in a container?

Write a `.yml` build workflow file for Linux.

-------------------------

## Free further space on CI runner

Already using the `free-disk-space-ubuntu` action.

[2875][2875]

[2875]: https://github.com/actions/runner-images/issues/2875

[free-disk-space-ubuntu][free-disk-space-ubuntu]

[free-disk-space-ubuntu]: https://github.com/marketplace/actions/free-disk-space-ubuntu

See splitting into separate CI steps or jobs above.

-------------------------

## Create CI build / tests on Windows

Add Windows build job to main workflow?

Should we build in Docker via `cargo build --target [<TRIPLE>]` ?
Use cross?

-------------------------

## Make `just` commands fully work on Windows

- make `just` commands fully work on Windows.
  - use [script]?
  - make cygwin bash work on Windows.
- consider cargo make / xtask?

-------------------------

## Review the need for rusty_fork

since we use nextest exclusively.
See nextest execution model: [nextest-how-it-works]

[nextest-how-it-works]: <https://nexte.st/docs/design/how-it-works>

-------------------------

## CI build on MacOS too?

Create a .yml file for MacOS build on GitHub runner?
Add build job to main workflow.

-------------------------

## Fix .github issue templates

-------------------------

## Build rust_rate tool

- retrieve most downloaded / recently downloaded / most relevant crates from crates.io / lib.rs.
- collect other sources of ranking: r/rust, awesome-rust, etc.
- generate a list of crates that should be included in the book.
