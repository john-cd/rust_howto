# TODOs

- Spelling

## Manually insert rest of cross-links

## Finish pandoc setup; generate PDF version of book

## Incoporate the added text in `categories`

- drafts: Finish
  - Graphics
  - Internationalization / localization
  - Multimedia
  - Containers
  - WASM?
  - compilers
  - development tools: FFI - reorg. with other FFI topics

- src: TODO

## Add text

- language
- standard library
- links
- other
- crates
- contributing

## Finish 56 COMING SOON examples

- parser impl
- crypto
- database
- proc macros
- pyo3, ractor, polars, datafusion, aws_lambda, aws_sdk

## Finish preproc directives in mdbook-scrub

- Scrub links to hidden pages instead of commenting e.g. [p-cross-platform]: # "../../other/cross-platform/index.md"
- Scrub any left-over {{#example ...}}, {{#crate ...}}, {{#categories ...}}, {{hi: ...}}, etc and warn.
- Crate link
- Crate badge?
- Category link?
- Category badges
- Crate blocks

## Insert category links

## Refresh crate alpha and crate by category indices

---

## Clean up playground

## Move mdbook-utils repo ?

---

## Setup bacon

---

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
