# TODOs



- [ ] polish stack-allocated
- [ ] polish directory_traversal
- [ ] polish hashing + algorithms
- [ ] polish [learning.md](bk\src\learning.md)
- [ ] polish [crate_selection](bk\src\crate_selection.md)
- [x] polish [contributing](bk\src\appendices\contributing\index.md) [#529](https://github.com/john-cd/rust_howto/issues/529)
- [ ] finish to add RBE link #29.
- [ ] Go through VS Code bookmarks - WIP.
- [ ] Manually insert rest of cross-links between pages?
- [ ] License / legal rethink.
- [ ] Indices.
  - [ ] create script for crates_and_examples.
  - [ ] rethink crates_alpha and categories.
  - [ ] categories.md.

## Recipe Tables

- [ ] Update recipe tables: improve scripts that generate links / update recipe tables.

## Additional Links / Content

- [ ] Additional Links / Content: use/move the rest of links in TOREVIEW.md.
- [ ] HUMAN finish to review chrome bookmarks and include in text.

## Finish pandoc / typst setup; generate PDF version of book

- [ ] make a plan to use pandoc / typst instead of mdbook.
- [ ] kindle create. [gumroad~website][gumroad~website] [blog~write-a-book-with-markdown] [write-book-with-obsidian] [book-creation-with-pandoc-and-markdown]


[gumroad~website]: https://gumroad.com

[blog~write-a-book-with-markdown]: <https://pianomanfrazier.com/post/write-a-book-with-markdown/#bookdown-https-bookdown-org-yihui-bookdown>

[write-book-with-obsidian]: <https://pdworkman.com/write-book-with-obsidian/#getting-it-out-of-obsidian>

[book-creation-with-pandoc-and-markdown]: <https://medium.com/@sydasif78/book-creation-with-pandoc-and-markdown-893c7d72cb35>

### Finish preproc directives in mdbook-scrub

- [ ] Scrub any left-over {{#example ...}}, {{#crate ...}}... etc and warn. WIP.
- [ ] Scrub links to hidden pages instead of having to comment e.g. [p~cross-platform]: # "../../other/cross-platform/index.md".

### Implement directives

- [ ] Crate link.
- [ ] Crate badge - WIP link tool.
- [ ] Category link?
- [ ] Category badges.
- [ ] Crate blocks.
- [ ] Recipe table.

## Review tools and consolidate WIP

- [ ] crate-indices WIP.
- [ ] autogen - in playground.
- [ ] links.
- [ ] templ.
- [ ] tool_lib.

- [ ] document WIP.
- [ ] add tests WIP.
- [ ] consolidate CLIs.

- [ ] finish new tools in book_tooling
  - [ ] parser_lib.
  - [ ] inline_links.
  - [ ] process_directives.
  - [ ] link_checker.

-------------------------

## In-Code Implementation Tasks

### Web & Networking

- [x] [Web] Migrate to modern WebSocket echo service [async_tungstenite.rs](later\crates\cats\web_programming_websocket\examples\async_tungstenite.rs) ([#1058](https://github.com/john-cd/rust_howto/issues/1058))
- [x] [Web] Implement Tonic gRPC example and server boilerplate [tonic.rs](bk\crates\cats\web_programming_http_server\examples\grpc\tonic.rs) ([#870](https://github.com/john-cd/rust_howto/issues/870))
- [ ] [Web] Update Hyper server example to 1.0 API and implement routing [hyper_server.rs](bk\crates\cats\web_programming_http_server\examples\hyper_server.rs) ([#866](https://github.com/john-cd/rust_howto/issues/866))
- [ ] [Web] Implement Axum routing, state, and review time limits [axum.rs](bk\crates\cats\web_programming_http_server\examples\axum.rs) ([#865](https://github.com/john-cd/rust_howto/issues/865))
- [ ] [Web] Implement Rocket Hello World and route mounting [rocket.rs](bk\crates\cats\web_programming_http_server\examples\rocket.rs) ([#869](https://github.com/john-cd/rust_howto/issues/869))
- [ ] [Web] Implement Leptos reactive counter and view mounting [leptos.rs](bk\crates\cats\web_programming_http_server\examples\leptos.rs) ([#867](https://github.com/john-cd/rust_howto/issues/867))
- [ ] [Web] Implement Async-GraphQL schema with Axum integration [async_graphql.rs](bk\crates\cats\web_programming_http_server\examples\async_graphql.rs) ([#864](https://github.com/john-cd/rust_howto/issues/864))
- [ ] [Web] Implement Actix-Web network requirement test [actix_web.rs](bk\crates\cats\web_programming_http_server\examples\actix_web.rs)
- [ ] [Network] Finalize Pingora reverse proxy implementation [pingora.rs](bk\crates\cats\network_programming\examples\reverse_proxy\pingora.rs) ([#812](https://github.com/john-cd/rust_howto/issues/812))
- [ ] [Network] Complete comprehensive reverse proxy section for Pingora and Rathole [reverse_proxy.md](bk\drafts\categories\network-programming\reverse_proxy.md) ([#424](https://github.com/john-cd/rust_howto/issues/424))

### Database & Storage
- [ ] [Database] Fix heavy test orchestration for Tiberius MSSQL [tiberius.rs](bk\crates\cats\database\examples\mssql\tiberius.rs) ([#1019](https://github.com/john-cd/rust_howto/issues/1019))
- [ ] [Database] Implement PostgreSQL aggregation examples [aggregate_data.rs](bk\crates\cats\database\examples\postgres\aggregate_data.rs)

### Systems & Tooling

- [ ] [GUI] Implement Winit window creation example [winit.rs](later\crates\cats\gui\examples\window_creation\winit.rs)
- [ ] [Parser] Implement `DocumentParser` trait and URL parsing 1427.md (#1427)
- [ ] [Test] Implement AFL.rs fuzzing target and panic discovery afl.rs (#748)
- [ ] [Memory] Review and finalize lazy initialization examples lazy_static.rs (#939)
- [ ] [Async] Expand stream examples with concurrent processing streams.md (#645)

### mdbook-utils (Companion Tool)
- [ ] Address all TODO comments in mdbook-utils codebase
- [ ] Document all modules, functions, and structs in mdbook-utils
- [ ] Generate `categories.md` stub in mdbook-utils

-------------------------

## Pending Section Content
- [ ] review cancelable example in `xmpl\`
- [ ] move WIP examples from `playground\`

## Incoporate the added text in drafts

- [ ] drafts: Finish.
  - [ ] Containers.
  - [ ] development tools: FFI - reorg. with other FFI topics.

- [ ] other:
  - [ ] cross-platform (partially done).
  - [ ] data proc,
  - [ ] gpu,
  - [ ] scripting (partially done).
  - [ ] written in rust (partially done).

- [ ] review drafts section for what I missed.

## Additional Categories

- [ ] move categories back to src, after final review.
  - [ ] api bindings ?
  - [ ] memory ?
  - [ ] caching ?
  - [ ] config ?
  - [ ] date and time ?
  - [ ] cli ?
  - [ ] command-line utils ~ dedupe with written_in_rust ?
  - [ ] compression ?
  - [ ] email ?
  - [ ] math.
  - [ ] ML ?
  - [ ] database impl ?
  - [ ] encoding ?
  - [ ] template ?
  - [ ] text processing.
  - [ ] HTTP client ?
  - [ ] build utils ?
  - [ ] dev tools ?
  - [ ] async ?
  - [ ] concurrency ?
  - [ ] database ?
  - [ ] os ?
  - [ ] parser impl ?
  - [ ] parsing ?
  - [ ] rust patterns ?
  - [ ] crypto ?
  - [ ] auth ?

- [ ] `other` section - move what's ready.

- [ ] sccache for dev container setup?

- [ ] Clean up playground crate

- [ ] Move mdbook-utils repo ?

- [ ] Setup bacon

- [ ] Add git hooks to automate formatting / clippy check / fix before commit

- [ ] Try `cargo-husky` or `prek`
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

- [ ] Increase speed of CI build (Linux)
  - [ ] Parallelism
    - [ ] Split ci.sh?
    - [ ] Run cargo build / clippy / nextest in separate CI steps or jobs for coarse parallelism.
- [ ] Try to move target and/or /usr/local/cargo to a docker volume instead of docker container FS
- [ ] Measure speed metric: `time dd if=/dev/zero of=test.dat bs=1024 count=100000.`
- [ ] Writes are fast when writing to container FS, volume, but slow on bind mount (to Windows). Need to test on Linux.

- [ ] Try to store `target` folder and/or `/usr/local/cargo` in a tmpfs? `/usr/local/cargo` is small enough to fit in memory (16GB max). [how-to-use-tmpfs-and-its-functions-in-docker][how-to-use-tmpfs-and-its-functions-in-docker]

[how-to-use-tmpfs-and-its-functions-in-docker]: https://linuxhaxor.net/code/how-to-use-tmpfs-and-its-functions-in-docker.html

- [ ] Make gha caching work on CI. Try local / inline caching as well. [optimize-docker-builds-github-actions-cache][optimize-docker-builds-github-actions-cache]

[optimize-docker-builds-github-actions-cache]: https://cicube.io/blog/optimize-docker-builds-github-actions-cache

- [ ] Try to build directly on the host, not in a container? Write a `.yml` build workflow file for Linux.

-------------------------

- [ ] Free further space on CI runner. Already using the `free-disk-space-ubuntu` action. [2875][2875] [free-disk-space-ubuntu][free-disk-space-ubuntu]

[2875]: https://github.com/actions/runner-images/issues/2875
[free-disk-space-ubuntu]: https://github.com/marketplace/actions/free-disk-space-ubuntu

-------------------------

- [ ] Create CI build / tests on Windows
  - [ ] Add Windows build job to main workflow? Should we build in Docker via `cargo build --target [<TRIPLE>]` ? Use `cross`?

-------------------------

- [ ] Make `just` commands fully work on Windows
  - [ ] use [script]?
  - [ ] make cygwin bash work on Windows.
- [ ] consider cargo make / xtask instead of just?

-------------------------

- [ ] Review the need for `rusty_fork`, since we use nextest exclusively. See nextest execution model: [nextest-how-it-works]

[nextest-how-it-works]: <https://nexte.st/docs/design/how-it-works>

-------------------------

- [ ]  CI build on MacOS too? Create a .yml file for MacOS build on GitHub runner? Add build job to main workflow.

-------------------------

- [ ] Fix .github issue templates

-------------------------

- [ ] content/Drafts
  - [ ] **DevOps/Cloud**: Complete sections for AWS, GitHub Actions, and release automation.
  - [ ] **Data Processing**: Finish CSV, Dataframes, and Data Engineering chapters.
  - [ ] **Wasm**: Add more practical examples for JS interfacing.

- [ ] Crate Integration
  - [ ] **Pending Examples**: Integrate and document crates like `tantivy`, `tree-sitter`, and `opentelemetry`.
  - [ ] **Cleanup**: Review and enable commented-out dependencies in various `Cargo.toml` files.
