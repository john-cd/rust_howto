# TODOs

> NOTE: This TODO list was regenerated from repository TODO/FIXME markers and GitHub issue references across the codebase, excluding mdbook-utils.

- Total TODO/FIXME markers scanned: 1026
- Total unique files with TODO/FIXME markers: 645
- Total referenced GitHub issues tracked: 589

## Top-level TODO file tasks

- [ ] polish stack-allocated
- [ ] polish directory_traversal
- [ ] polish hashing + algorithms
- [ ] polish [learning.md](bk/src/learning.md)
- [ ] polish [crate_selection](bk/src/crate_selection.md)
- [x] polish [contributing](bk/src/appendices/contributing/index.md) [#529](https://github.com/john-cd/rust_howto/issues/529)
- [ ] finish to add RBE link [#29](https://github.com/john-cd/rust_howto/issues/29).
- [ ] Go through VS Code bookmarks - WIP.
- [ ] Manually insert rest of cross-links between pages?
- [ ] License / legal rethink.
- [ ] Indices.
- [ ] Update recipe tables: improve scripts that generate links / update recipe tables.
- [ ] Additional Links / Content: use/move the rest of links in TOREVIEW.md.
- [ ] HUMAN finish to review chrome bookmarks and include in text.
- [ ] make a plan to use pandoc / typst instead of mdbook.
- [ ] kindle create. [gumroad~website][gumroad~website] [blog~write-a-book-with-markdown] [write-book-with-obsidian] [book-creation-with-pandoc-and-markdown]
- [ ] Scrub any left-over {{#example ...}}, {{#crate ...}}... etc and warn. WIP.
- [ ] Scrub links to hidden pages instead of having to comment e.g. [p~cross-platform]: # "../../other/cross-platform/index.md".
- [ ] Crate link.
- [ ] Crate badge - WIP link tool.
- [ ] Category link?
- [ ] Category badges.
- [ ] Crate blocks.
- [ ] Recipe table.
- [ ] crate-indices WIP.
- [ ] autogen - in playground.
- [ ] links.
- [ ] templ.
- [ ] tool_lib.
- [ ] document WIP.
- [ ] add tests WIP.
- [ ] consolidate CLIs.
- [ ] finish new tools in book_tooling
- [x] [Web] Migrate to modern WebSocket echo service [async_tungstenite.rs](later/crates/cats/web_programming_websocket/examples/async_tungstenite.rs) ([#1058](https://github.com/john-cd/rust_howto/issues/1058))
- [x] [Web] Implement Tonic gRPC example and server boilerplate [tonic.rs](bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs) ([#870](https://github.com/john-cd/rust_howto/issues/870))
- [x] [Web] Update Hyper server example to 1.0 API and implement routing [hyper_server.rs](bk/crates/cats/web_programming_http_server/examples/hyper_server.rs) ([#866](https://github.com/john-cd/rust_howto/issues/866))
- [x] [Web] Implement Axum routing, state, and review time limits [axum.rs](bk/crates/cats/web_programming_http_server/examples/axum.rs) ([#865](https://github.com/john-cd/rust_howto/issues/865))
- [x] [Web] Implement Rocket Hello World and route mounting [rocket.rs](bk/crates/cats/web_programming_http_server/examples/rocket.rs) ([#869](https://github.com/john-cd/rust_howto/issues/869))
- [ ] [Web] Implement Leptos reactive counter and view mounting [leptos.rs](bk/crates/cats/web_programming_http_server/examples/leptos.rs) ([#867](https://github.com/john-cd/rust_howto/issues/867))
- [x] [Web] Implement Async-GraphQL schema with Axum integration [async_graphql.rs](bk/crates/cats/web_programming_http_server/examples/async_graphql.rs) ([#864](https://github.com/john-cd/rust_howto/issues/864))
- [x] [Web] Implement Actix-Web network requirement test [actix_web.rs](bk/crates/cats/web_programming_http_server/examples/actix_web.rs)
- [ ] [Network] Finalize Pingora reverse proxy implementation [pingora.rs](bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs) ([#812](https://github.com/john-cd/rust_howto/issues/812))
- [ ] [Network] Complete comprehensive reverse proxy section for Pingora and Rathole [reverse_proxy.md](bk/drafts/categories/network-programming/reverse_proxy.md) ([#424](https://github.com/john-cd/rust_howto/issues/424))
- [ ] [Database] Fix heavy test orchestration for Tiberius MSSQL [tiberius.rs](bk/crates/cats/database/examples/mssql/tiberius.rs) ([#1019](https://github.com/john-cd/rust_howto/issues/1019))
- [ ] [Database] Implement PostgreSQL aggregation examples [aggregate_data.rs](bk/crates/cats/database/examples/postgres/aggregate_data.rs)
- [ ] [GUI] Implement Winit window creation example [winit.rs](later/crates/cats/gui/examples/window_creation/winit.rs)
- [ ] [Parser] Implement `DocumentParser` trait and URL parsing 1427.md ([#1427](https://github.com/john-cd/rust_howto/issues/1427))
- [ ] [Test] Implement AFL.rs fuzzing target and panic discovery afl.rs ([#748](https://github.com/john-cd/rust_howto/issues/748))
- [ ] [Memory] Review and finalize lazy initialization examples lazy_static.rs ([#939](https://github.com/john-cd/rust_howto/issues/939))
- [ ] [Async] Expand stream examples with concurrent processing streams.md ([#645](https://github.com/john-cd/rust_howto/issues/645))
- [ ] Address all TODO comments in mdbook-utils codebase
- [ ] Document all modules, functions, and structs in mdbook-utils
- [ ] Generate `categories.md` stub in mdbook-utils
- [ ] review cancelable example in `xmpl/`
- [ ] move WIP examples from `playground/`
- [ ] drafts: Finish
- [ ] review drafts section for what I missed.
- [ ] move categories back to src, after final review.
- [ ] `other` section - move what's ready.
- [ ] sccache for dev container setup?
- [ ] Clean up playground crate
- [ ] Move mdbook-utils repo ?
- [ ] Setup bacon
- [ ] Add git hooks to automate formatting / clippy check / fix before commit
- [ ] Try `cargo-husky` or `prek`
- [ ] Increase speed of CI build (Linux)
- [ ] Try to move target and/or /usr/local/cargo to a docker volume instead of docker container FS
- [ ] Measure speed metric: `time dd if=/dev/zero of=test.dat bs=1024 count=100000.`
- [ ] Writes are fast when writing to container FS, volume, but slow on bind mount (to Windows). Need to test on Linux.
- [ ] Try to store `target` folder and/or `/usr/local/cargo` in a tmpfs? `/usr/local/cargo` is small enough to fit in memory (16GB max). [how-to-use-tmpfs-and-its-functions-in-docker][how-to-use-tmpfs-and-its-functions-in-docker]
- [ ] Make gha caching work on CI. Try local / inline caching as well. [optimize-docker-builds-github-actions-cache][optimize-docker-builds-github-actions-cache]
- [ ] Try to build directly on the host, not in a container? Write a `.yml` build workflow file for Linux.
- [ ] Free further space on CI runner. Already using the `free-disk-space-ubuntu` action. [2875][2875] [free-disk-space-ubuntu][free-disk-space-ubuntu]
- [ ] Create CI build / tests on Windows
- [ ] Make `just` commands fully work on Windows
- [ ] consider cargo make / xtask instead of just?
- [ ] Review the need for `rusty_fork`, since we use nextest exclusively. See nextest execution model: [nextest-how-it-works]
- [ ]  CI build on MacOS too? Create a .yml file for MacOS build on GitHub runner? Add build job to main workflow.
- [ ] Fix .github issue templates
- [ ] content/Drafts
- [ ] Crate Integration

## Book Content and Drafts

### bk/drafts/categories/asynchronous/async.md

- [ ] [write; review in depth](https://github.com/john-cd/rust_howto/issues/633) ([bk/drafts/categories/asynchronous/async.md](./bk/drafts/categories/asynchronous/async.md#L83))

### bk/drafts/categories/asynchronous/async_channels.md

- [ ] [async_channels: review](https://github.com/john-cd/rust_howto/issues/215) add other [`postage`][c~postage~docs]↗{{hi:postage}} channels? ([bk/drafts/categories/asynchronous/async_channels.md](./bk/drafts/categories/asynchronous/async_channels.md#L89))

### bk/drafts/categories/asynchronous/async_traits.md

- [ ] [async-traits: review new Rust features](https://github.com/john-cd/rust_howto/issues/216) ([bk/drafts/categories/asynchronous/async_traits.md](./bk/drafts/categories/asynchronous/async_traits.md#L40))

### bk/drafts/categories/asynchronous/async_utilities.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/906) ([bk/drafts/categories/asynchronous/async_utilities.md](./bk/drafts/categories/asynchronous/async_utilities.md#L31))

### bk/drafts/categories/asynchronous/futures.md

- [ ] [futures = Utility [functions][p~functions] for working with Futures and Streams](https://github.com/john-cd/rust_howto/issues/1340) ([bk/drafts/categories/asynchronous/futures.md](./bk/drafts/categories/asynchronous/futures.md#L61))

### bk/drafts/categories/asynchronous/index.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/905) ([bk/drafts/categories/asynchronous/index.md](./bk/drafts/categories/asynchronous/index.md#L66))

### bk/drafts/categories/asynchronous/streams.md

- [ ] Resolve TODO/FIXME at line 35 ([bk/drafts/categories/asynchronous/streams.md](./bk/drafts/categories/asynchronous/streams.md#L35))
- [ ] [add more. streams2.rs is noplayground because it requires a network. rewrite](https://github.com/john-cd/rust_howto/issues/645) ([bk/drafts/categories/asynchronous/streams.md](./bk/drafts/categories/asynchronous/streams.md#L41))

### bk/drafts/categories/asynchronous/tokio.md

- [ ] Resolve TODO/FIXME at line 66 ([bk/drafts/categories/asynchronous/tokio.md](./bk/drafts/categories/asynchronous/tokio.md#L66))
- [ ] [tokio: review](https://github.com/john-cd/rust_howto/issues/223) ([bk/drafts/categories/asynchronous/tokio.md](./bk/drafts/categories/asynchronous/tokio.md#L72))

### bk/drafts/categories/authentication/basic_authentication.md

- [ ] [expand](https://github.com/john-cd/rust_howto/issues/224) ([bk/drafts/categories/authentication/basic_authentication.md](./bk/drafts/categories/authentication/basic_authentication.md#L26))

### bk/drafts/categories/authentication/index.md

- [ ] [add Oauth](https://github.com/john-cd/rust_howto/issues/636) ([bk/drafts/categories/authentication/index.md](./bk/drafts/categories/authentication/index.md#L27))

### bk/drafts/categories/caching/in_memory_cache.md

- [ ] [expand; review in depth](https://github.com/john-cd/rust_howto/issues/227) ([bk/drafts/categories/caching/in_memory_cache.md](./bk/drafts/categories/caching/in_memory_cache.md#L48))

### bk/drafts/categories/caching/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1173) ([bk/drafts/categories/caching/index.md](./bk/drafts/categories/caching/index.md#L53))

### bk/drafts/categories/command-line-interface/ansi_terminal.md

- [ ] [ansi_terminal: `ansi_term` is archived; write / decide what to cover](https://github.com/john-cd/rust_howto/issues/231) ([bk/drafts/categories/command-line-interface/ansi_terminal.md](./bk/drafts/categories/command-line-interface/ansi_terminal.md#L138))

### bk/drafts/categories/command-line-interface/argument_parsing.md

- [ ] [arguments: review](https://github.com/john-cd/rust_howto/issues/233) ([bk/drafts/categories/command-line-interface/argument_parsing.md](./bk/drafts/categories/command-line-interface/argument_parsing.md#L149))

### bk/drafts/categories/command-line-interface/index.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/907) ([bk/drafts/categories/command-line-interface/index.md](./bk/drafts/categories/command-line-interface/index.md#L48))

### bk/drafts/categories/command-line-interface/tui.md

- [ ] Resolve TODO/FIXME at line 33 ([bk/drafts/categories/command-line-interface/tui.md](./bk/drafts/categories/command-line-interface/tui.md#L33))
- [ ] [tui: expand](https://github.com/john-cd/rust_howto/issues/234) ([bk/drafts/categories/command-line-interface/tui.md](./bk/drafts/categories/command-line-interface/tui.md#L45))

### bk/drafts/categories/command-line-interface/user_interaction.md

- [ ] [user_interaction: write](https://github.com/john-cd/rust_howto/issues/235) ([bk/drafts/categories/command-line-interface/user_interaction.md](./bk/drafts/categories/command-line-interface/user_interaction.md#L49))

### bk/drafts/categories/command-line-utilities/filesystem_cli.md

- [ ] [filesystem: organize](https://github.com/john-cd/rust_howto/issues/237) ([bk/drafts/categories/command-line-utilities/filesystem_cli.md](./bk/drafts/categories/command-line-utilities/filesystem_cli.md#L60))

### bk/drafts/categories/command-line-utilities/index.md

- [ ] [write; deduplicate with other > written in rust](https://github.com/john-cd/rust_howto/issues/1189) ([bk/drafts/categories/command-line-utilities/index.md](./bk/drafts/categories/command-line-utilities/index.md#L48))

### bk/drafts/categories/command-line-utilities/networking_cli.md

- [ ] [networking: write](https://github.com/john-cd/rust_howto/issues/238) ([bk/drafts/categories/command-line-utilities/networking_cli.md](./bk/drafts/categories/command-line-utilities/networking_cli.md#L25))

### bk/drafts/categories/command-line-utilities/shells.md

- [ ] [shells: expand](https://github.com/john-cd/rust_howto/issues/239) ([bk/drafts/categories/command-line-utilities/shells.md](./bk/drafts/categories/command-line-utilities/shells.md#L32))

### bk/drafts/categories/compression/compression.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1062) ([bk/drafts/categories/compression/compression.md](./bk/drafts/categories/compression/compression.md#L48))

### bk/drafts/categories/compression/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1184) ([bk/drafts/categories/compression/index.md](./bk/drafts/categories/compression/index.md#L27))

### bk/drafts/categories/compression/tar.md

- [ ] Resolve TODO/FIXME at line 41 ([bk/drafts/categories/compression/tar.md](./bk/drafts/categories/compression/tar.md#L41))
- [ ] [tar: review](https://github.com/john-cd/rust_howto/issues/253) tar_decompress.rs is noplayground - fix? tar_compress.rs is noplayground - fix? tar_strip_prefix.rs is noplayground - fix? ([bk/drafts/categories/compression/tar.md](./bk/drafts/categories/compression/tar.md#L47))

### bk/drafts/categories/concurrency/_actors.md

- [ ] [_actors: organize](https://github.com/john-cd/rust_howto/issues/269) ([bk/drafts/categories/concurrency/_actors.md](./bk/drafts/categories/concurrency/_actors.md#L98))

### bk/drafts/categories/concurrency/atomics.md

- [ ] [fix](https://github.com/john-cd/rust_howto/issues/1342) ([bk/drafts/categories/concurrency/atomics.md](./bk/drafts/categories/concurrency/atomics.md#L56))

### bk/drafts/categories/concurrency/concurrent_data_structures.md

- [ ] [concurrent_data_structures: finish](https://github.com/john-cd/rust_howto/issues/258) ([bk/drafts/categories/concurrency/concurrent_data_structures.md](./bk/drafts/categories/concurrency/concurrent_data_structures.md#L73))

### bk/drafts/categories/concurrency/crossbeam.md

- [ ] Resolve TODO/FIXME at line 47 ([bk/drafts/categories/concurrency/crossbeam.md](./bk/drafts/categories/concurrency/crossbeam.md#L47))
- [ ] [crossbeam: cleanup](https://github.com/john-cd/rust_howto/issues/259) ([bk/drafts/categories/concurrency/crossbeam.md](./bk/drafts/categories/concurrency/crossbeam.md#L53))

### bk/drafts/categories/concurrency/data_parallelism.md

- [ ] Resolve TODO/FIXME at line 116 ([bk/drafts/categories/concurrency/data_parallelism.md](./bk/drafts/categories/concurrency/data_parallelism.md#L116))
- [ ] [data_parallelism: polish; dedupe with multithreading.md](https://github.com/john-cd/rust_howto/issues/260) ([bk/drafts/categories/concurrency/data_parallelism.md](./bk/drafts/categories/concurrency/data_parallelism.md#L122))

### bk/drafts/categories/concurrency/explicit_threads.md

- [ ] Resolve TODO/FIXME at line 27 ([bk/drafts/categories/concurrency/explicit_threads.md](./bk/drafts/categories/concurrency/explicit_threads.md#L27))
- [ ] [explicit_threads: polish](https://github.com/john-cd/rust_howto/issues/262) ([bk/drafts/categories/concurrency/explicit_threads.md](./bk/drafts/categories/concurrency/explicit_threads.md#L33))

### bk/drafts/categories/concurrency/index.md

- [ ] [concurrency: add somewhere](https://github.com/john-cd/rust_howto/issues/263) ([bk/drafts/categories/concurrency/index.md](./bk/drafts/categories/concurrency/index.md#L83))

### bk/drafts/categories/concurrency/message_passing.md

- [ ] Resolve TODO/FIXME at line 52 ([bk/drafts/categories/concurrency/message_passing.md](./bk/drafts/categories/concurrency/message_passing.md#L52))
- [ ] [message_passing: polish](https://github.com/john-cd/rust_howto/issues/264) ([bk/drafts/categories/concurrency/message_passing.md](./bk/drafts/categories/concurrency/message_passing.md#L58))

### bk/drafts/categories/concurrency/send_sync.md

- [ ] Resolve TODO/FIXME at line 57 ([bk/drafts/categories/concurrency/send_sync.md](./bk/drafts/categories/concurrency/send_sync.md#L57))
- [ ] [write; add links?](https://github.com/john-cd/rust_howto/issues/909) ([bk/drafts/categories/concurrency/send_sync.md](./bk/drafts/categories/concurrency/send_sync.md#L63))

### bk/drafts/categories/concurrency/shared_state.md

- [ ] [shared_state: reorganize; section for Arc; for Mutex / Rwlock; for joint use; for make_mut](https://github.com/john-cd/rust_howto/issues/266) ([bk/drafts/categories/concurrency/shared_state.md](./bk/drafts/categories/concurrency/shared_state.md#L82))

### bk/drafts/categories/concurrency/threadpool.md

- [ ] Resolve TODO/FIXME at line 38 ([bk/drafts/categories/concurrency/threadpool.md](./bk/drafts/categories/concurrency/threadpool.md#L38))
- [ ] [threadpool: polish](https://github.com/john-cd/rust_howto/issues/267) threadpool_fractal.rs is noplayground - linking with [`cc`][c~cc~docs]↗{{hi:cc}} failed: exit status: 1 - fix? ([bk/drafts/categories/concurrency/threadpool.md](./bk/drafts/categories/concurrency/threadpool.md#L44))

### bk/drafts/categories/config/configuration.md

- [ ] Resolve TODO/FIXME at line 48 ([bk/drafts/categories/config/configuration.md](./bk/drafts/categories/config/configuration.md#L48))
- [ ] [finish](https://github.com/john-cd/rust_howto/issues/270) ([bk/drafts/categories/config/configuration.md](./bk/drafts/categories/config/configuration.md#L54))

### bk/drafts/categories/config/environment_variables.md

- [ ] Resolve TODO/FIXME at line 51 ([bk/drafts/categories/config/environment_variables.md](./bk/drafts/categories/config/environment_variables.md#L51))
- [ ] [environment_variables: interaction between [config][p~config] and env variables](https://github.com/john-cd/rust_howto/issues/271) ([bk/drafts/categories/config/environment_variables.md](./bk/drafts/categories/config/environment_variables.md#L57))

### bk/drafts/categories/config/index.md

- [ ] Configuration crates like [`config-rs`][c~config~docs]↗{{hi:config-rs}}, a powerful and flexible crate for layered configuration. It supports merging configurations from various sources, including files, environment variables, and in-memory data ([bk/drafts/categories/config/index.md](./bk/drafts/categories/config/index.md#L16))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1195) ([bk/drafts/categories/config/index.md](./bk/drafts/categories/config/index.md#L64))

### bk/drafts/categories/cryptography/aead.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1183) ([bk/drafts/categories/cryptography/aead.md](./bk/drafts/categories/cryptography/aead.md#L57))

### bk/drafts/categories/cryptography/certificates.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1179) ([bk/drafts/categories/cryptography/certificates.md](./bk/drafts/categories/cryptography/certificates.md#L64))

### bk/drafts/categories/cryptography/cryptography_utilities.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1180) ([bk/drafts/categories/cryptography/cryptography_utilities.md](./bk/drafts/categories/cryptography/cryptography_utilities.md#L39))

### bk/drafts/categories/cryptography/encryption.md

- [ ] Resolve TODO/FIXME at line 27 ([bk/drafts/categories/cryptography/encryption.md](./bk/drafts/categories/cryptography/encryption.md#L27))
- [ ] [encryption: expand](https://github.com/john-cd/rust_howto/issues/272) ([bk/drafts/categories/cryptography/encryption.md](./bk/drafts/categories/cryptography/encryption.md#L33))

### bk/drafts/categories/cryptography/hmac.md

- [ ] Resolve TODO/FIXME at line 21 ([bk/drafts/categories/cryptography/hmac.md](./bk/drafts/categories/cryptography/hmac.md#L21))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1181) ([bk/drafts/categories/cryptography/hmac.md](./bk/drafts/categories/cryptography/hmac.md#L27))

### bk/drafts/categories/cryptography/index.md

- [ ] [cryptography: review](https://github.com/john-cd/rust_howto/issues/274) ([bk/drafts/categories/cryptography/index.md](./bk/drafts/categories/cryptography/index.md#L95))

### bk/drafts/categories/cryptography/password_hashing.md

- [ ] Resolve TODO/FIXME at line 81 ([bk/drafts/categories/cryptography/password_hashing.md](./bk/drafts/categories/cryptography/password_hashing.md#L81))
- [ ] [password_hashing: write](https://github.com/john-cd/rust_howto/issues/275) ([bk/drafts/categories/cryptography/password_hashing.md](./bk/drafts/categories/cryptography/password_hashing.md#L87))

### bk/drafts/categories/cryptography/signature.md

- [ ] Resolve TODO/FIXME at line 74 ([bk/drafts/categories/cryptography/signature.md](./bk/drafts/categories/cryptography/signature.md#L74))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1178) ([bk/drafts/categories/cryptography/signature.md](./bk/drafts/categories/cryptography/signature.md#L80))

### bk/drafts/categories/cryptography/tls.md

- [ ] Resolve TODO/FIXME at line 35 ([bk/drafts/categories/cryptography/tls.md](./bk/drafts/categories/cryptography/tls.md#L35))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1182) ([bk/drafts/categories/cryptography/tls.md](./bk/drafts/categories/cryptography/tls.md#L41))

### bk/drafts/categories/data-structures/index.md

- [ ] [data-structures: expand](https://github.com/john-cd/rust_howto/issues/280) ([bk/drafts/categories/data-structures/index.md](./bk/drafts/categories/data-structures/index.md#L189))

### bk/drafts/categories/database-implementations/databases.md

- [ ] Resolve TODO/FIXME at line 51 ([bk/drafts/categories/database-implementations/databases.md](./bk/drafts/categories/database-implementations/databases.md#L51))
- [ ] [databases: expand / write](https://github.com/john-cd/rust_howto/issues/290) ([bk/drafts/categories/database-implementations/databases.md](./bk/drafts/categories/database-implementations/databases.md#L57))

### bk/drafts/categories/database-implementations/index.md

- [ ] [database-implementations: expand](https://github.com/john-cd/rust_howto/issues/292) ([bk/drafts/categories/database-implementations/index.md](./bk/drafts/categories/database-implementations/index.md#L25))

### bk/drafts/categories/database-implementations/rust_search_engines.md

- [ ] [search: expand](https://github.com/john-cd/rust_howto/issues/291) ([bk/drafts/categories/database-implementations/rust_search_engines.md](./bk/drafts/categories/database-implementations/rust_search_engines.md#L47))

### bk/drafts/categories/database/amqp.md

- [ ] Resolve TODO/FIXME at line 23 ([bk/drafts/categories/database/amqp.md](./bk/drafts/categories/database/amqp.md#L23))
- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1064) ([bk/drafts/categories/database/amqp.md](./bk/drafts/categories/database/amqp.md#L29))

### bk/drafts/categories/database/connection_pool.md

- [ ] Resolve TODO/FIXME at line 23 ([bk/drafts/categories/database/connection_pool.md](./bk/drafts/categories/database/connection_pool.md#L23))
- [ ] [connection_pool: expand; example](https://github.com/john-cd/rust_howto/issues/284) ([bk/drafts/categories/database/connection_pool.md](./bk/drafts/categories/database/connection_pool.md#L29))

### bk/drafts/categories/database/index.md

- [ ] [organize](https://github.com/john-cd/rust_howto/issues/1065) ([bk/drafts/categories/database/index.md](./bk/drafts/categories/database/index.md#L65))

### bk/drafts/categories/database/key_value_stores.md

- [ ] Resolve TODO/FIXME at line 44 ([bk/drafts/categories/database/key_value_stores.md](./bk/drafts/categories/database/key_value_stores.md#L44))

### bk/drafts/categories/database/mssql.md

- [ ] Resolve TODO/FIXME at line 17 ([bk/drafts/categories/database/mssql.md](./bk/drafts/categories/database/mssql.md#L17))
- [ ] [organize / write](https://github.com/john-cd/rust_howto/issues/1067) ([bk/drafts/categories/database/mssql.md](./bk/drafts/categories/database/mssql.md#L23))

### bk/drafts/categories/database/nosql.md

- [ ] Resolve TODO/FIXME at line 56 ([bk/drafts/categories/database/nosql.md](./bk/drafts/categories/database/nosql.md#L56))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/1068) ([bk/drafts/categories/database/nosql.md](./bk/drafts/categories/database/nosql.md#L62))

### bk/drafts/categories/database/oracle.md

- [ ] Resolve TODO/FIXME at line 37 ([bk/drafts/categories/database/oracle.md](./bk/drafts/categories/database/oracle.md#L37))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1069) ([bk/drafts/categories/database/oracle.md](./bk/drafts/categories/database/oracle.md#L43))

### bk/drafts/categories/database/postgres.md

- [ ] Resolve TODO/FIXME at line 70 ([bk/drafts/categories/database/postgres.md](./bk/drafts/categories/database/postgres.md#L70))
- [ ] [postgres: `cornucopia`](https://github.com/john-cd/rust_howto/issues/286) ([bk/drafts/categories/database/postgres.md](./bk/drafts/categories/database/postgres.md#L76))

### bk/drafts/categories/database/query_builders_orms.md

- [ ] todos: [] ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L114))
- [ ] model { ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L119))
- [ ] .(create().title("Make pizza")) ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L141))
- [ ] .(create().title("Finish Toasty")) ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L142))
- [ ] .(create().title("Sleep")) ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L143))
- [ ] while let Some() = todos.next().await { ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L153))
- [ ] let = .unwrap() ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L154))
- [ ] println!("{#?}") ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L155))
- [ ] Resolve TODO/FIXME at line 161 ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L161))
- [ ] [write; move `toasty` example to a file](https://github.com/john-cd/rust_howto/issues/912) ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L167))

### bk/drafts/categories/database/search.md

- [ ] Resolve TODO/FIXME at line 31 ([bk/drafts/categories/database/search.md](./bk/drafts/categories/database/search.md#L31))
- [ ] [database/search.md: expand](https://github.com/john-cd/rust_howto/issues/288) ([bk/drafts/categories/database/search.md](./bk/drafts/categories/database/search.md#L37))

### bk/drafts/categories/database/sqlite.md

- [ ] Resolve TODO/FIXME at line 45 ([bk/drafts/categories/database/sqlite.md](./bk/drafts/categories/database/sqlite.md#L45))

### bk/drafts/categories/date-and-time/duration.md

- [ ] Resolve TODO/FIXME at line 43 ([bk/drafts/categories/date-and-time/duration.md](./bk/drafts/categories/date-and-time/duration.md#L43))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/913) ([bk/drafts/categories/date-and-time/duration.md](./bk/drafts/categories/date-and-time/duration.md#L49))

### bk/drafts/categories/date-and-time/index.md

- [ ] [review in depth](https://github.com/john-cd/rust_howto/issues/1188) ([bk/drafts/categories/date-and-time/index.md](./bk/drafts/categories/date-and-time/index.md#L38))

### bk/drafts/categories/date-and-time/parse.md

- [ ] Resolve TODO/FIXME at line 67 ([bk/drafts/categories/date-and-time/parse.md](./bk/drafts/categories/date-and-time/parse.md#L67))
- [ ] [write humantime; review](https://github.com/john-cd/rust_howto/issues/914) ([bk/drafts/categories/date-and-time/parse.md](./bk/drafts/categories/date-and-time/parse.md#L73))

### bk/drafts/categories/date-and-time/time_crate.md

- [ ] Resolve TODO/FIXME at line 21 ([bk/drafts/categories/date-and-time/time_crate.md](./bk/drafts/categories/date-and-time/time_crate.md#L21))
- [ ] [time: write](https://github.com/john-cd/rust_howto/issues/293) ([bk/drafts/categories/date-and-time/time_crate.md](./bk/drafts/categories/date-and-time/time_crate.md#L27))

### bk/drafts/categories/development-tools/cargo/cargo.md

- [ ] Resolve TODO/FIXME at line 98 ([bk/drafts/categories/development-tools/cargo/cargo.md](./bk/drafts/categories/development-tools/cargo/cargo.md#L98))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/915) ([bk/drafts/categories/development-tools/cargo/cargo.md](./bk/drafts/categories/development-tools/cargo/cargo.md#L104))

### bk/drafts/categories/development-tools/cargo/crate_registries.md

- [ ] Resolve TODO/FIXME at line 27 ([bk/drafts/categories/development-tools/cargo/crate_registries.md](./bk/drafts/categories/development-tools/cargo/crate_registries.md#L27))
- [ ] [write; expand](https://github.com/john-cd/rust_howto/issues/294) ([bk/drafts/categories/development-tools/cargo/crate_registries.md](./bk/drafts/categories/development-tools/cargo/crate_registries.md#L33))

### bk/drafts/categories/development-tools/cargo/package_layout.md

- [ ] Resolve TODO/FIXME at line 45 ([bk/drafts/categories/development-tools/cargo/package_layout.md](./bk/drafts/categories/development-tools/cargo/package_layout.md#L45))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/916) ([bk/drafts/categories/development-tools/cargo/package_layout.md](./bk/drafts/categories/development-tools/cargo/package_layout.md#L51))

### bk/drafts/categories/development-tools/compilation/faster_linking.md

- [ ] [faster_linking: review - some linkers are deprecated](https://github.com/john-cd/rust_howto/issues/242) ([bk/drafts/categories/development-tools/compilation/faster_linking.md](./bk/drafts/categories/development-tools/compilation/faster_linking.md#L142))

### bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md

- [ ] [write / expand / align table and text](https://github.com/john-cd/rust_howto/issues/245) ([bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md](./bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md#L122))

### bk/drafts/categories/development-tools/cross-compilation/cross_compilation.md

- [ ] [cross_compilation: expand](https://github.com/john-cd/rust_howto/issues/240) ([bk/drafts/categories/development-tools/cross-compilation/cross_compilation.md](./bk/drafts/categories/development-tools/cross-compilation/cross_compilation.md#L55))

### bk/drafts/categories/development-tools/documentation/badges.md

- [ ] Resolve TODO/FIXME at line 13 ([bk/drafts/categories/development-tools/documentation/badges.md](./bk/drafts/categories/development-tools/documentation/badges.md#L13))
- [ ] [badges: expand](https://github.com/john-cd/rust_howto/issues/296) ([bk/drafts/categories/development-tools/documentation/badges.md](./bk/drafts/categories/development-tools/documentation/badges.md#L18))

### bk/drafts/categories/development-tools/documentation/documentation.md

- [ ] Resolve TODO/FIXME at line 61 ([bk/drafts/categories/development-tools/documentation/documentation.md](./bk/drafts/categories/development-tools/documentation/documentation.md#L61))
- [ ] [documentation: add; review](https://github.com/john-cd/rust_howto/issues/297) ([bk/drafts/categories/development-tools/documentation/documentation.md](./bk/drafts/categories/development-tools/documentation/documentation.md#L67))

### bk/drafts/categories/development-tools/documentation/mdbook.md

- [ ] Resolve TODO/FIXME at line 140 ([bk/drafts/categories/development-tools/documentation/mdbook.md](./bk/drafts/categories/development-tools/documentation/mdbook.md#L140))
- [ ] [mdbook: organize, expand](https://github.com/john-cd/rust_howto/issues/299) ([bk/drafts/categories/development-tools/documentation/mdbook.md](./bk/drafts/categories/development-tools/documentation/mdbook.md#L146))

### bk/drafts/categories/development-tools/formatting/formatting.md

- [ ] Resolve TODO/FIXME at line 90 ([bk/drafts/categories/development-tools/formatting/formatting.md](./bk/drafts/categories/development-tools/formatting/formatting.md#L90))
- [ ] [formatting: add link for formatting attributes](https://github.com/john-cd/rust_howto/issues/300) ([bk/drafts/categories/development-tools/formatting/formatting.md](./bk/drafts/categories/development-tools/formatting/formatting.md#L96))

### bk/drafts/categories/development-tools/index.md

- [ ] [index: reorganize; dedupe alternatives / [`log`][c~log~docs]↗{{hi:log}} / config_log](https://github.com/john-cd/rust_howto/issues/319) ([bk/drafts/categories/development-tools/index.md](./bk/drafts/categories/development-tools/index.md#L95))

### bk/drafts/categories/development-tools/installation/install.md

- [ ] Resolve TODO/FIXME at line 29 ([bk/drafts/categories/development-tools/installation/install.md](./bk/drafts/categories/development-tools/installation/install.md#L29))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/918) ([bk/drafts/categories/development-tools/installation/install.md](./bk/drafts/categories/development-tools/installation/install.md#L35))

### bk/drafts/categories/development-tools/installation/rustup.md

- [ ] Resolve TODO/FIXME at line 50 ([bk/drafts/categories/development-tools/installation/rustup.md](./bk/drafts/categories/development-tools/installation/rustup.md#L50))
- [ ] [rustup: expand / clean up](https://github.com/john-cd/rust_howto/issues/302) ([bk/drafts/categories/development-tools/installation/rustup.md](./bk/drafts/categories/development-tools/installation/rustup.md#L56))

### bk/drafts/categories/development-tools/other/code_build.md

- [ ] [review; dedupe this page and cargo plugins > building; move from other section](https://github.com/john-cd/rust_howto/issues/919) ([bk/drafts/categories/development-tools/other/code_build.md](./bk/drafts/categories/development-tools/other/code_build.md#L114))

### bk/drafts/categories/development-tools/other/code_verification.md

- [ ] [code_verification: expand; revise refs.incl.md](https://github.com/john-cd/rust_howto/issues/303) ([bk/drafts/categories/development-tools/other/code_verification.md](./bk/drafts/categories/development-tools/other/code_verification.md#L110))

### bk/drafts/categories/development-tools/other/miri.md

- [ ] Resolve TODO/FIXME at line 25 ([bk/drafts/categories/development-tools/other/miri.md](./bk/drafts/categories/development-tools/other/miri.md#L25))
- [ ] [miri: polish](https://github.com/john-cd/rust_howto/issues/304) ([bk/drafts/categories/development-tools/other/miri.md](./bk/drafts/categories/development-tools/other/miri.md#L31))

### bk/drafts/categories/development-tools/other/other.md

- [ ] Resolve TODO/FIXME at line 34 ([bk/drafts/categories/development-tools/other/other.md](./bk/drafts/categories/development-tools/other/other.md#L34))
- [ ] [other: expand; revise refs.incl.md](https://github.com/john-cd/rust_howto/issues/305) ([bk/drafts/categories/development-tools/other/other.md](./bk/drafts/categories/development-tools/other/other.md#L40))

### bk/drafts/categories/development-tools/transcompilation/transpilers.md

- [ ] Resolve TODO/FIXME at line 13 ([bk/drafts/categories/development-tools/transcompilation/transpilers.md](./bk/drafts/categories/development-tools/transcompilation/transpilers.md#L13))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1196) ([bk/drafts/categories/development-tools/transcompilation/transpilers.md](./bk/drafts/categories/development-tools/transcompilation/transpilers.md#L19))

### bk/drafts/categories/development-tools/versioning/versioning.md

- [ ] Resolve TODO/FIXME at line 68 ([bk/drafts/categories/development-tools/versioning/versioning.md](./bk/drafts/categories/development-tools/versioning/versioning.md#L68))
- [ ] [review / align with intro](https://github.com/john-cd/rust_howto/issues/920) ([bk/drafts/categories/development-tools/versioning/versioning.md](./bk/drafts/categories/development-tools/versioning/versioning.md#L74))

### bk/drafts/categories/development-tools_build-utils/autocfg.md

- [ ] Resolve TODO/FIXME at line 19 ([bk/drafts/categories/development-tools_build-utils/autocfg.md](./bk/drafts/categories/development-tools_build-utils/autocfg.md#L19))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1166) ([bk/drafts/categories/development-tools_build-utils/autocfg.md](./bk/drafts/categories/development-tools_build-utils/autocfg.md#L25))

### bk/drafts/categories/development-tools_build-utils/build_cache.md

- [ ] Resolve TODO/FIXME at line 13 ([bk/drafts/categories/development-tools_build-utils/build_cache.md](./bk/drafts/categories/development-tools_build-utils/build_cache.md#L13))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1167) ([bk/drafts/categories/development-tools_build-utils/build_cache.md](./bk/drafts/categories/development-tools_build-utils/build_cache.md#L19))

### bk/drafts/categories/development-tools_build-utils/build_time_tooling.md

- [ ] Resolve TODO/FIXME at line 154 ([bk/drafts/categories/development-tools_build-utils/build_time_tooling.md](./bk/drafts/categories/development-tools_build-utils/build_time_tooling.md#L154))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/921) ([bk/drafts/categories/development-tools_build-utils/build_time_tooling.md](./bk/drafts/categories/development-tools_build-utils/build_time_tooling.md#L160))

### bk/drafts/categories/development-tools_build-utils/index.md

- [ ] [development-tools_build-utils/index: add](https://github.com/john-cd/rust_howto/issues/306) ([bk/drafts/categories/development-tools_build-utils/index.md](./bk/drafts/categories/development-tools_build-utils/index.md#L34))

### bk/drafts/categories/development-tools_cargo-plugins/auditing.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/922) ([bk/drafts/categories/development-tools_cargo-plugins/auditing.md](./bk/drafts/categories/development-tools_cargo-plugins/auditing.md#L89))

### bk/drafts/categories/development-tools_cargo-plugins/building.md

- [ ] [building: expand; cross link](https://github.com/john-cd/rust_howto/issues/309) ([bk/drafts/categories/development-tools_cargo-plugins/building.md](./bk/drafts/categories/development-tools_cargo-plugins/building.md#L92))

### bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md

- [ ] Resolve TODO/FIXME at line 88 ([bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md](./bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md#L88))
- [ ] [code_formatting_linting: expand](https://github.com/john-cd/rust_howto/issues/310) ([bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md](./bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md#L94))

### bk/drafts/categories/development-tools_cargo-plugins/code_writing.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/924) ([bk/drafts/categories/development-tools_cargo-plugins/code_writing.md](./bk/drafts/categories/development-tools_cargo-plugins/code_writing.md#L43))

### bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md

- [ ] Resolve TODO/FIXME at line 23 ([bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md](./bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md#L23))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/923) ([bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md](./bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md#L29))

### bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md

- [ ] [dependency_management: review](https://github.com/john-cd/rust_howto/issues/597) ([bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md](./bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md#L87))

### bk/drafts/categories/development-tools_cargo-plugins/index.md

- [ ] [review in depth, reorg table](https://github.com/john-cd/rust_howto/issues/311) ([bk/drafts/categories/development-tools_cargo-plugins/index.md](./bk/drafts/categories/development-tools_cargo-plugins/index.md#L88))

### bk/drafts/categories/development-tools_cargo-plugins/maintaining.md

- [ ] Resolve TODO/FIXME at line 65 ([bk/drafts/categories/development-tools_cargo-plugins/maintaining.md](./bk/drafts/categories/development-tools_cargo-plugins/maintaining.md#L65))
- [ ] [maintaining: fix; titles; decide what goes where](https://github.com/john-cd/rust_howto/issues/313) ([bk/drafts/categories/development-tools_cargo-plugins/maintaining.md](./bk/drafts/categories/development-tools_cargo-plugins/maintaining.md#L71))

### bk/drafts/categories/development-tools_cargo-plugins/performance.md

- [ ] Resolve TODO/FIXME at line 27 ([bk/drafts/categories/development-tools_cargo-plugins/performance.md](./bk/drafts/categories/development-tools_cargo-plugins/performance.md#L27))
- [ ] [performance: expand](https://github.com/john-cd/rust_howto/issues/314) ([bk/drafts/categories/development-tools_cargo-plugins/performance.md](./bk/drafts/categories/development-tools_cargo-plugins/performance.md#L33))

### bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md

- [ ] Resolve TODO/FIXME at line 57 ([bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md](./bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md#L57))
- [ ] [watching_for_changes: expand](https://github.com/john-cd/rust_howto/issues/315) ([bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md](./bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md#L63))

### bk/drafts/categories/development-tools_debugging/config_log.md

- [ ] Resolve TODO/FIXME at line 81 ([bk/drafts/categories/development-tools_debugging/config_log.md](./bk/drafts/categories/development-tools_debugging/config_log.md#L81))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/925) ([bk/drafts/categories/development-tools_debugging/config_log.md](./bk/drafts/categories/development-tools_debugging/config_log.md#L87))

### bk/drafts/categories/development-tools_debugging/debugging.md

- [ ] [write; cross link](https://github.com/john-cd/rust_howto/issues/1344) ([bk/drafts/categories/development-tools_debugging/debugging.md](./bk/drafts/categories/development-tools_debugging/debugging.md#L19))

### bk/drafts/categories/development-tools_debugging/diagnostic_functions.md

- [ ] Resolve TODO/FIXME at line 15 ([bk/drafts/categories/development-tools_debugging/diagnostic_functions.md](./bk/drafts/categories/development-tools_debugging/diagnostic_functions.md#L15))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/926) ([bk/drafts/categories/development-tools_debugging/diagnostic_functions.md](./bk/drafts/categories/development-tools_debugging/diagnostic_functions.md#L21))

### bk/drafts/categories/development-tools_debugging/distributed_telemetry.md

- [ ] [write / expand / organize. what to cover?](https://github.com/john-cd/rust_howto/issues/1343) ([bk/drafts/categories/development-tools_debugging/distributed_telemetry.md](./bk/drafts/categories/development-tools_debugging/distributed_telemetry.md#L74))

### bk/drafts/categories/development-tools_debugging/index.md

- [ ] [index: reorganize; dedupe alternatives / `log` / config_log](https://github.com/john-cd/rust_howto/issues/319) ([bk/drafts/categories/development-tools_debugging/index.md](./bk/drafts/categories/development-tools_debugging/index.md#L67))

### bk/drafts/categories/development-tools_debugging/log.md

- [ ] Resolve TODO/FIXME at line 77 ([bk/drafts/categories/development-tools_debugging/log.md](./bk/drafts/categories/development-tools_debugging/log.md#L77))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/927) ([bk/drafts/categories/development-tools_debugging/log.md](./bk/drafts/categories/development-tools_debugging/log.md#L83))

### bk/drafts/categories/development-tools_debugging/metrics.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1345) ([bk/drafts/categories/development-tools_debugging/metrics.md](./bk/drafts/categories/development-tools_debugging/metrics.md#L25))

### bk/drafts/categories/development-tools_debugging/tracing.md

- [ ] Resolve TODO/FIXME at line 137 ([bk/drafts/categories/development-tools_debugging/tracing.md](./bk/drafts/categories/development-tools_debugging/tracing.md#L137))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/322) ([bk/drafts/categories/development-tools_debugging/tracing.md](./bk/drafts/categories/development-tools_debugging/tracing.md#L143))

### bk/drafts/categories/development-tools_debugging/tracing_alternatives.md

- [ ] Resolve TODO/FIXME at line 66 ([bk/drafts/categories/development-tools_debugging/tracing_alternatives.md](./bk/drafts/categories/development-tools_debugging/tracing_alternatives.md#L66))
- [ ] [write, organize together with the old log content.](https://github.com/john-cd/rust_howto/issues/649) ([bk/drafts/categories/development-tools_debugging/tracing_alternatives.md](./bk/drafts/categories/development-tools_debugging/tracing_alternatives.md#L72))

### bk/drafts/categories/development-tools_profiling/assembly.md

- [ ] Resolve TODO/FIXME at line 16 ([bk/drafts/categories/development-tools_profiling/assembly.md](./bk/drafts/categories/development-tools_profiling/assembly.md#L16))
- [ ] [assembly: write](https://github.com/john-cd/rust_howto/issues/333) ([bk/drafts/categories/development-tools_profiling/assembly.md](./bk/drafts/categories/development-tools_profiling/assembly.md#L22))

### bk/drafts/categories/development-tools_profiling/benchmarking.md

- [ ] hyperfine --warmup 3 'grep -R *' ([bk/drafts/categories/development-tools_profiling/benchmarking.md](./bk/drafts/categories/development-tools_profiling/benchmarking.md#L75))
- [ ] Resolve TODO/FIXME at line 80 ([bk/drafts/categories/development-tools_profiling/benchmarking.md](./bk/drafts/categories/development-tools_profiling/benchmarking.md#L80))
- [ ] [benchmarking: write](https://github.com/john-cd/rust_howto/issues/335) ([bk/drafts/categories/development-tools_profiling/benchmarking.md](./bk/drafts/categories/development-tools_profiling/benchmarking.md#L86))

### bk/drafts/categories/development-tools_profiling/index.md

- [ ] [expand / review](https://github.com/john-cd/rust_howto/issues/337) ([bk/drafts/categories/development-tools_profiling/index.md](./bk/drafts/categories/development-tools_profiling/index.md#L41))

### bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md

- [ ] Resolve TODO/FIXME at line 43 ([bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md](./bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md#L43))
- [ ] [memory: write / organize](https://github.com/john-cd/rust_howto/issues/336) ([bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md](./bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md#L49))

### bk/drafts/categories/development-tools_testing/assertions.md

- [ ] Resolve TODO/FIXME at line 35 ([bk/drafts/categories/development-tools_testing/assertions.md](./bk/drafts/categories/development-tools_testing/assertions.md#L35))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1174) ([bk/drafts/categories/development-tools_testing/assertions.md](./bk/drafts/categories/development-tools_testing/assertions.md#L41))

### bk/drafts/categories/development-tools_testing/code_coverage.md

- [ ] Resolve TODO/FIXME at line 24 ([bk/drafts/categories/development-tools_testing/code_coverage.md](./bk/drafts/categories/development-tools_testing/code_coverage.md#L24))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1176) ([bk/drafts/categories/development-tools_testing/code_coverage.md](./bk/drafts/categories/development-tools_testing/code_coverage.md#L30))

### bk/drafts/categories/development-tools_testing/fuzzing.md

- [ ] Resolve TODO/FIXME at line 66 ([bk/drafts/categories/development-tools_testing/fuzzing.md](./bk/drafts/categories/development-tools_testing/fuzzing.md#L66))
- [ ] [fuzzing: review fuzzing crates](https://github.com/john-cd/rust_howto/issues/339) ([bk/drafts/categories/development-tools_testing/fuzzing.md](./bk/drafts/categories/development-tools_testing/fuzzing.md#L72))

### bk/drafts/categories/development-tools_testing/index.md

- [ ] [expand](https://github.com/john-cd/rust_howto/issues/341) ([bk/drafts/categories/development-tools_testing/index.md](./bk/drafts/categories/development-tools_testing/index.md#L31))

### bk/drafts/categories/development-tools_testing/mocking.md

- [ ] Resolve TODO/FIXME at line 32 ([bk/drafts/categories/development-tools_testing/mocking.md](./bk/drafts/categories/development-tools_testing/mocking.md#L32))
- [ ] [testing: write](https://github.com/john-cd/rust_howto/issues/340) ([bk/drafts/categories/development-tools_testing/mocking.md](./bk/drafts/categories/development-tools_testing/mocking.md#L38))

### bk/drafts/categories/development-tools_testing/property_based_testing.md

- [ ] Resolve TODO/FIXME at line 43 ([bk/drafts/categories/development-tools_testing/property_based_testing.md](./bk/drafts/categories/development-tools_testing/property_based_testing.md#L43))

### bk/drafts/categories/development-tools_testing/test_runners.md

- [ ] Resolve TODO/FIXME at line 37 ([bk/drafts/categories/development-tools_testing/test_runners.md](./bk/drafts/categories/development-tools_testing/test_runners.md#L37))
- [ ] [testing: write](https://github.com/john-cd/rust_howto/issues/340) ([bk/drafts/categories/development-tools_testing/test_runners.md](./bk/drafts/categories/development-tools_testing/test_runners.md#L43))

### bk/drafts/categories/development-tools_testing/testing.md

- [ ] Resolve TODO/FIXME at line 52 ([bk/drafts/categories/development-tools_testing/testing.md](./bk/drafts/categories/development-tools_testing/testing.md#L52))
- [ ] [testing: write](https://github.com/john-cd/rust_howto/issues/340) ([bk/drafts/categories/development-tools_testing/testing.md](./bk/drafts/categories/development-tools_testing/testing.md#L58))

### bk/drafts/categories/email/email_parsing.md

- [ ] Resolve TODO/FIXME at line 19 ([bk/drafts/categories/email/email_parsing.md](./bk/drafts/categories/email/email_parsing.md#L19))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1204) ([bk/drafts/categories/email/email_parsing.md](./bk/drafts/categories/email/email_parsing.md#L25))

### bk/drafts/categories/email/index.md

- [ ] Resolve TODO/FIXME at line 23 ([bk/drafts/categories/email/index.md](./bk/drafts/categories/email/index.md#L23))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/343) ([bk/drafts/categories/email/index.md](./bk/drafts/categories/email/index.md#L29))

### bk/drafts/categories/email/send_emails.md

- [ ] Resolve TODO/FIXME at line 45 ([bk/drafts/categories/email/send_emails.md](./bk/drafts/categories/email/send_emails.md#L45))
- [ ] [send_emails: write](https://github.com/john-cd/rust_howto/issues/342) ([bk/drafts/categories/email/send_emails.md](./bk/drafts/categories/email/send_emails.md#L51))

### bk/drafts/categories/encoding/_binary_encoders.md

- [ ] Resolve TODO/FIXME at line 112 ([bk/drafts/categories/encoding/_binary_encoders.md](./bk/drafts/categories/encoding/_binary_encoders.md#L112))
- [ ] [binary_encoders: write; add anchors; add examples; add table; add to index / SUMMARY](https://github.com/john-cd/rust_howto/issues/349) ([bk/drafts/categories/encoding/_binary_encoders.md](./bk/drafts/categories/encoding/_binary_encoders.md#L118))

### bk/drafts/categories/encoding/complex_encoding.md

- [ ] Resolve TODO/FIXME at line 48 ([bk/drafts/categories/encoding/complex_encoding.md](./bk/drafts/categories/encoding/complex_encoding.md#L48))
- [ ] [complex: clean up `toml`](https://github.com/john-cd/rust_howto/issues/350) ([bk/drafts/categories/encoding/complex_encoding.md](./bk/drafts/categories/encoding/complex_encoding.md#L54))

### bk/drafts/categories/encoding/csv.md

- [ ] Resolve TODO/FIXME at line 88 ([bk/drafts/categories/encoding/csv.md](./bk/drafts/categories/encoding/csv.md#L88))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/928) ([bk/drafts/categories/encoding/csv.md](./bk/drafts/categories/encoding/csv.md#L94))

### bk/drafts/categories/encoding/index.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/929) ([bk/drafts/categories/encoding/index.md](./bk/drafts/categories/encoding/index.md#L104))

### bk/drafts/categories/encoding/no_external_schema.md

- [ ] Resolve TODO/FIXME at line 29 ([bk/drafts/categories/encoding/no_external_schema.md](./bk/drafts/categories/encoding/no_external_schema.md#L29))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1078) ([bk/drafts/categories/encoding/no_external_schema.md](./bk/drafts/categories/encoding/no_external_schema.md#L35))

### bk/drafts/categories/encoding/serde.md

- [ ] Resolve TODO/FIXME at line 59 ([bk/drafts/categories/encoding/serde.md](./bk/drafts/categories/encoding/serde.md#L59))
- [ ] [serde: write](https://github.com/john-cd/rust_howto/issues/352) ([bk/drafts/categories/encoding/serde.md](./bk/drafts/categories/encoding/serde.md#L65))

### bk/drafts/categories/encoding/string_encoding.md

- [ ] Resolve TODO/FIXME at line 65 ([bk/drafts/categories/encoding/string_encoding.md](./bk/drafts/categories/encoding/string_encoding.md#L65))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/930) ([bk/drafts/categories/encoding/string_encoding.md](./bk/drafts/categories/encoding/string_encoding.md#L71))

### bk/drafts/categories/encoding/typecasts.md

- [ ] Resolve TODO/FIXME at line 92 ([bk/drafts/categories/encoding/typecasts.md](./bk/drafts/categories/encoding/typecasts.md#L92))
- [ ] [typecasts: write; reorganize](https://github.com/john-cd/rust_howto/issues/354) ([bk/drafts/categories/encoding/typecasts.md](./bk/drafts/categories/encoding/typecasts.md#L98))

### bk/drafts/categories/graphics/index.md

- [ ] [write / organize / review in depth](https://github.com/john-cd/rust_howto/issues/1226) ([bk/drafts/categories/graphics/index.md](./bk/drafts/categories/graphics/index.md#L80))

### bk/drafts/categories/hardware-support/index.md

- [ ] Resolve TODO/FIXME at line 21 ([bk/drafts/categories/hardware-support/index.md](./bk/drafts/categories/hardware-support/index.md#L21))
- [ ] Resolve TODO/FIXME at line 27 ([bk/drafts/categories/hardware-support/index.md](./bk/drafts/categories/hardware-support/index.md#L27))
- [ ] how devices communicate with each other ([bk/drafts/categories/hardware-support/index.md](./bk/drafts/categories/hardware-support/index.md#L35))
- [ ] [expand hardware-support, write missing sections](https://github.com/john-cd/rust_howto/issues/70) ([bk/drafts/categories/hardware-support/index.md](./bk/drafts/categories/hardware-support/index.md#L67))

### bk/drafts/categories/hardware-support/peripherals.md

- [ ] replace by `nix` ([bk/drafts/categories/hardware-support/peripherals.md](./bk/drafts/categories/hardware-support/peripherals.md#L23))
- [ ] Resolve TODO/FIXME at line 31 ([bk/drafts/categories/hardware-support/peripherals.md](./bk/drafts/categories/hardware-support/peripherals.md#L31))
- [ ] Resolve TODO/FIXME at line 35 ([bk/drafts/categories/hardware-support/peripherals.md](./bk/drafts/categories/hardware-support/peripherals.md#L35))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1177) ([bk/drafts/categories/hardware-support/peripherals.md](./bk/drafts/categories/hardware-support/peripherals.md#L48))

### bk/drafts/categories/hardware-support/processor.md

- [ ] [processor: expand](https://github.com/john-cd/rust_howto/issues/399) ([bk/drafts/categories/hardware-support/processor.md](./bk/drafts/categories/hardware-support/processor.md#L81))

### bk/drafts/categories/memory-management/index.md

- [ ] | Global Statics and Lazy Initialization | | ([bk/drafts/categories/memory-management/index.md](./bk/drafts/categories/memory-management/index.md#L12))
- [ ] [memory-management/index: organize; align table and sections; write missing sections; cross link](https://github.com/john-cd/rust_howto/issues/410) ([bk/drafts/categories/memory-management/index.md](./bk/drafts/categories/memory-management/index.md#L67))

### bk/drafts/categories/memory-management/lazy_initialization.md

- [ ] cover once_cell in std ([bk/drafts/categories/memory-management/lazy_initialization.md](./bk/drafts/categories/memory-management/lazy_initialization.md#L46))
- [ ] Resolve TODO/FIXME at line 88 ([bk/drafts/categories/memory-management/lazy_initialization.md](./bk/drafts/categories/memory-management/lazy_initialization.md#L88))
- [ ] [lazy_initialization: write / fix](https://github.com/john-cd/rust_howto/issues/411) ([bk/drafts/categories/memory-management/lazy_initialization.md](./bk/drafts/categories/memory-management/lazy_initialization.md#L95))

### bk/drafts/categories/network-programming/index.md

- [ ] [write; add cross-links; review; review `email_address`, `fast_chemail`](https://github.com/john-cd/rust_howto/issues/944) ([bk/drafts/categories/network-programming/index.md](./bk/drafts/categories/network-programming/index.md#L35))

### bk/drafts/categories/network-programming/reverse_proxy.md

- [ ] [Network: Complete comprehensive reverse proxy section for Pingora and Rathole](https://github.com/john-cd/rust_howto/issues/424) ([bk/drafts/categories/network-programming/reverse_proxy.md](./bk/drafts/categories/network-programming/reverse_proxy.md#L94))

### bk/drafts/categories/network-programming/server.md

- [ ] Resolve TODO/FIXME at line 39 ([bk/drafts/categories/network-programming/server.md](./bk/drafts/categories/network-programming/server.md#L39))
- [ ] [server: write; add cross-links: async, web, `http` server](https://github.com/john-cd/rust_howto/issues/425) ([bk/drafts/categories/network-programming/server.md](./bk/drafts/categories/network-programming/server.md#L45))

### bk/drafts/categories/os/external_commands.md

- [ ] Resolve TODO/FIXME at line 93 ([bk/drafts/categories/os/external_commands.md](./bk/drafts/categories/os/external_commands.md#L93))
- [ ] [write which; review](https://github.com/john-cd/rust_howto/issues/946) ([bk/drafts/categories/os/external_commands.md](./bk/drafts/categories/os/external_commands.md#L99))

### bk/drafts/categories/os/index.md

- [ ] [os/index: fix; organize; add cross links](https://github.com/john-cd/rust_howto/issues/429) ([bk/drafts/categories/os/index.md](./bk/drafts/categories/os/index.md#L41))

### bk/drafts/categories/os/low_level_system_calls.md

- [ ] Resolve TODO/FIXME at line 23 ([bk/drafts/categories/os/low_level_system_calls.md](./bk/drafts/categories/os/low_level_system_calls.md#L23))
- [ ] [low_level_system_calls: write](https://github.com/john-cd/rust_howto/issues/430) ([bk/drafts/categories/os/low_level_system_calls.md](./bk/drafts/categories/os/low_level_system_calls.md#L29))

### bk/drafts/categories/os/rust_os.md

- [ ] Resolve TODO/FIXME at line 17 ([bk/drafts/categories/os/rust_os.md](./bk/drafts/categories/os/rust_os.md#L17))
- [ ] [write; move to written-in-rust?](https://github.com/john-cd/rust_howto/issues/639) ([bk/drafts/categories/os/rust_os.md](./bk/drafts/categories/os/rust_os.md#L23))

### bk/drafts/categories/os_unix-apis/index.md

- [ ] Resolve TODO/FIXME at line 45 ([bk/drafts/categories/os_unix-apis/index.md](./bk/drafts/categories/os_unix-apis/index.md#L45))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/437) ([bk/drafts/categories/os_unix-apis/index.md](./bk/drafts/categories/os_unix-apis/index.md#L51))

### bk/drafts/categories/os_unix-apis/unix.md

- [ ] Resolve TODO/FIXME at line 38 ([bk/drafts/categories/os_unix-apis/unix.md](./bk/drafts/categories/os_unix-apis/unix.md#L38))
- [ ] [unix: write](https://github.com/john-cd/rust_howto/issues/436) ([bk/drafts/categories/os_unix-apis/unix.md](./bk/drafts/categories/os_unix-apis/unix.md#L44))

### bk/drafts/categories/os_windows-apis/index.md

- [ ] [review in depth / write](https://github.com/john-cd/rust_howto/issues/950) ([bk/drafts/categories/os_windows-apis/index.md](./bk/drafts/categories/os_windows-apis/index.md#L33))

### bk/drafts/categories/os_windows-apis/windows.md

- [ ] Resolve TODO/FIXME at line 42 ([bk/drafts/categories/os_windows-apis/windows.md](./bk/drafts/categories/os_windows-apis/windows.md#L42))
- [ ] [windows: write; add examples for "windows" and `winapi`](https://github.com/john-cd/rust_howto/issues/438) ([bk/drafts/categories/os_windows-apis/windows.md](./bk/drafts/categories/os_windows-apis/windows.md#L48))

### bk/drafts/categories/parser-implementations/html.md

- [ ] Resolve TODO/FIXME at line 57 ([bk/drafts/categories/parser-implementations/html.md](./bk/drafts/categories/parser-implementations/html.md#L57))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1187) ([bk/drafts/categories/parser-implementations/html.md](./bk/drafts/categories/parser-implementations/html.md#L63))

### bk/drafts/categories/parser-implementations/index.md

- [ ] [write; Review `async-graphql-parser`](https://github.com/john-cd/rust_howto/issues/447) ([bk/drafts/categories/parser-implementations/index.md](./bk/drafts/categories/parser-implementations/index.md#L51))

### bk/drafts/categories/parser-implementations/ini.md

- [ ] Resolve TODO/FIXME at line 21 ([bk/drafts/categories/parser-implementations/ini.md](./bk/drafts/categories/parser-implementations/ini.md#L21))
- [ ] [Write; Also consider: `configparser`](https://github.com/john-cd/rust_howto/issues/1185) ([bk/drafts/categories/parser-implementations/ini.md](./bk/drafts/categories/parser-implementations/ini.md#L27))

### bk/drafts/categories/parser-implementations/json.md

- [ ] Resolve TODO/FIXME at line 39 ([bk/drafts/categories/parser-implementations/json.md](./bk/drafts/categories/parser-implementations/json.md#L39))
- [ ] [json: include in index.md / SUMMARY.md; write](https://github.com/john-cd/rust_howto/issues/440) ([bk/drafts/categories/parser-implementations/json.md](./bk/drafts/categories/parser-implementations/json.md#L45))

### bk/drafts/categories/parser-implementations/markdown.md

- [ ] Resolve TODO/FIXME at line 39 ([bk/drafts/categories/parser-implementations/markdown.md](./bk/drafts/categories/parser-implementations/markdown.md#L39))
- [ ] [markdown: write](https://github.com/john-cd/rust_howto/issues/442) ([bk/drafts/categories/parser-implementations/markdown.md](./bk/drafts/categories/parser-implementations/markdown.md#L45))

### bk/drafts/categories/parser-implementations/programming_languages.md

- [ ] Resolve TODO/FIXME at line 44 ([bk/drafts/categories/parser-implementations/programming_languages.md](./bk/drafts/categories/parser-implementations/programming_languages.md#L44))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/951) ([bk/drafts/categories/parser-implementations/programming_languages.md](./bk/drafts/categories/parser-implementations/programming_languages.md#L50))

### bk/drafts/categories/parser-implementations/toml.md

- [ ] Resolve TODO/FIXME at line 39 ([bk/drafts/categories/parser-implementations/toml.md](./bk/drafts/categories/parser-implementations/toml.md#L39))
- [ ] [toml: write](https://github.com/john-cd/rust_howto/issues/444) ([bk/drafts/categories/parser-implementations/toml.md](./bk/drafts/categories/parser-implementations/toml.md#L45))

### bk/drafts/categories/parser-implementations/xml.md

- [ ] Resolve TODO/FIXME at line 61 ([bk/drafts/categories/parser-implementations/xml.md](./bk/drafts/categories/parser-implementations/xml.md#L61))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/446) ([bk/drafts/categories/parser-implementations/xml.md](./bk/drafts/categories/parser-implementations/xml.md#L67))

### bk/drafts/categories/parser-implementations/yaml.md

- [ ] Resolve TODO/FIXME at line 21 ([bk/drafts/categories/parser-implementations/yaml.md](./bk/drafts/categories/parser-implementations/yaml.md#L21))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1186) ([bk/drafts/categories/parser-implementations/yaml.md](./bk/drafts/categories/parser-implementations/yaml.md#L27))

### bk/drafts/categories/parsing/index.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/952) ([bk/drafts/categories/parsing/index.md](./bk/drafts/categories/parsing/index.md#L32))

### bk/drafts/categories/parsing/parsing.md

- [ ] [parsing: write; examples](https://github.com/john-cd/rust_howto/issues/448) ([bk/drafts/categories/parsing/parsing.md](./bk/drafts/categories/parsing/parsing.md#L66))

### bk/drafts/categories/rust-patterns/behavioral_patterns.md

- [ ] Resolve TODO/FIXME at line 13 ([bk/drafts/categories/rust-patterns/behavioral_patterns.md](./bk/drafts/categories/rust-patterns/behavioral_patterns.md#L13))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/461) ([bk/drafts/categories/rust-patterns/behavioral_patterns.md](./bk/drafts/categories/rust-patterns/behavioral_patterns.md#L19))

### bk/drafts/categories/rust-patterns/builder_pattern.md

- [ ] Resolve TODO/FIXME at line 37 ([bk/drafts/categories/rust-patterns/builder_pattern.md](./bk/drafts/categories/rust-patterns/builder_pattern.md#L37))
- [ ] [write and add to index and SUMMARY](https://github.com/john-cd/rust_howto/issues/648) ([bk/drafts/categories/rust-patterns/builder_pattern.md](./bk/drafts/categories/rust-patterns/builder_pattern.md#L43))

### bk/drafts/categories/rust-patterns/creational_patterns.md

- [ ] Resolve TODO/FIXME at line 29 ([bk/drafts/categories/rust-patterns/creational_patterns.md](./bk/drafts/categories/rust-patterns/creational_patterns.md#L29))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1391) ([bk/drafts/categories/rust-patterns/creational_patterns.md](./bk/drafts/categories/rust-patterns/creational_patterns.md#L35))

### bk/drafts/categories/rust-patterns/error_handling/error_customization.md

- [ ] [error_customization: write / organize](https://github.com/john-cd/rust_howto/issues/463) ([bk/drafts/categories/rust-patterns/error_handling/error_customization.md](./bk/drafts/categories/rust-patterns/error_handling/error_customization.md#L82))

### bk/drafts/categories/rust-patterns/error_handling/error_handling.md

- [ ] Resolve TODO/FIXME at line 206 ([bk/drafts/categories/rust-patterns/error_handling/error_handling.md](./bk/drafts/categories/rust-patterns/error_handling/error_handling.md#L206))
- [ ] [error_handling: fix / organize](https://github.com/john-cd/rust_howto/issues/465) ([bk/drafts/categories/rust-patterns/error_handling/error_handling.md](./bk/drafts/categories/rust-patterns/error_handling/error_handling.md#L212))

### bk/drafts/categories/rust-patterns/functional_programming.md

- [ ] Resolve TODO/FIXME at line 78 ([bk/drafts/categories/rust-patterns/functional_programming.md](./bk/drafts/categories/rust-patterns/functional_programming.md#L78))
- [ ] [functional_programming: organize / align intro and sections / add examples](https://github.com/john-cd/rust_howto/issues/467) ([bk/drafts/categories/rust-patterns/functional_programming.md](./bk/drafts/categories/rust-patterns/functional_programming.md#L84))

### bk/drafts/categories/rust-patterns/index.md

- [ ] | Threading | | [`std::thread`][c~std::thread~docs]↗{{hi:std::thread}} and `std::sync` provide basic threading and synchronization primitives. | ([bk/drafts/categories/rust-patterns/index.md](./bk/drafts/categories/rust-patterns/index.md#L46))
- [ ] [index: organize / write / add cross links](https://github.com/john-cd/rust_howto/issues/469) ([bk/drafts/categories/rust-patterns/index.md](./bk/drafts/categories/rust-patterns/index.md#L144))

### bk/drafts/categories/rust-patterns/rust_specific_patterns.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1393) ([bk/drafts/categories/rust-patterns/rust_specific_patterns.md](./bk/drafts/categories/rust-patterns/rust_specific_patterns.md#L68))

### bk/drafts/categories/rust-patterns/structural_patterns.md

- [ ] Resolve TODO/FIXME at line 7 ([bk/drafts/categories/rust-patterns/structural_patterns.md](./bk/drafts/categories/rust-patterns/structural_patterns.md#L7))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1392) ([bk/drafts/categories/rust-patterns/structural_patterns.md](./bk/drafts/categories/rust-patterns/structural_patterns.md#L13))

### bk/drafts/categories/template-engine/index.md

- [ ] Resolve TODO/FIXME at line 56 ([bk/drafts/categories/template-engine/index.md](./bk/drafts/categories/template-engine/index.md#L56))
- [ ] [template-engine/index: write](https://github.com/john-cd/rust_howto/issues/482) ([bk/drafts/categories/template-engine/index.md](./bk/drafts/categories/template-engine/index.md#L62))

### bk/drafts/categories/template-engine/tera.md

- [ ] Resolve TODO/FIXME at line 17 ([bk/drafts/categories/template-engine/tera.md](./bk/drafts/categories/template-engine/tera.md#L17))
- [ ] [tera: write; add sample from tools code](https://github.com/john-cd/rust_howto/issues/483) ([bk/drafts/categories/template-engine/tera.md](./bk/drafts/categories/template-engine/tera.md#L23))

### bk/drafts/categories/template-engine/tinytemplate.md

- [ ] Resolve TODO/FIXME at line 17 ([bk/drafts/categories/template-engine/tinytemplate.md](./bk/drafts/categories/template-engine/tinytemplate.md#L17))
- [ ] [tinytemplate: write; add sample from tools code](https://github.com/john-cd/rust_howto/issues/484) ([bk/drafts/categories/template-engine/tinytemplate.md](./bk/drafts/categories/template-engine/tinytemplate.md#L23))

### bk/drafts/categories/text-editors/ides.md

- [ ] Resolve TODO/FIXME at line 48 ([bk/drafts/categories/text-editors/ides.md](./bk/drafts/categories/text-editors/ides.md#L48))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/486) ([bk/drafts/categories/text-editors/ides.md](./bk/drafts/categories/text-editors/ides.md#L54))

### bk/drafts/categories/text-editors/index.md

- [ ] Resolve TODO/FIXME at line 38 ([bk/drafts/categories/text-editors/index.md](./bk/drafts/categories/text-editors/index.md#L38))
- [ ] [write; review in depth finish the table](https://github.com/john-cd/rust_howto/issues/962) ([bk/drafts/categories/text-editors/index.md](./bk/drafts/categories/text-editors/index.md#L44))

### bk/drafts/categories/text-processing/diffing.md

- [ ] [write; titles](https://github.com/john-cd/rust_howto/issues/1193) ([bk/drafts/categories/text-processing/diffing.md](./bk/drafts/categories/text-processing/diffing.md#L34))

### bk/drafts/categories/text-processing/index.md

- [ ] [review; address NLP](https://github.com/john-cd/rust_howto/issues/963) ([bk/drafts/categories/text-processing/index.md](./bk/drafts/categories/text-processing/index.md#L82))

### bk/drafts/categories/text-processing/other_strings.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/1194) ([bk/drafts/categories/text-processing/other_strings.md](./bk/drafts/categories/text-processing/other_strings.md#L89))

### bk/drafts/categories/text-processing/regex.md

- [ ] [regex: write](https://github.com/john-cd/rust_howto/issues/488) ([bk/drafts/categories/text-processing/regex.md](./bk/drafts/categories/text-processing/regex.md#L90))

### bk/drafts/categories/text-processing/string_concat.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/964) ([bk/drafts/categories/text-processing/string_concat.md](./bk/drafts/categories/text-processing/string_concat.md#L31))

### bk/drafts/categories/text-processing/string_manipulation.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1190) ([bk/drafts/categories/text-processing/string_manipulation.md](./bk/drafts/categories/text-processing/string_manipulation.md#L50))

### bk/drafts/categories/text-processing/string_parsing.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/965) ([bk/drafts/categories/text-processing/string_parsing.md](./bk/drafts/categories/text-processing/string_parsing.md#L26))

### bk/drafts/categories/text-processing/string_search.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1191) ([bk/drafts/categories/text-processing/string_search.md](./bk/drafts/categories/text-processing/string_search.md#L55))

### bk/drafts/categories/text-processing/unicode.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/1192) ([bk/drafts/categories/text-processing/unicode.md](./bk/drafts/categories/text-processing/unicode.md#L26))

### bk/drafts/categories/web-programming/http_types_and_interfaces.md

- [ ] Resolve TODO/FIXME at line 32 ([bk/drafts/categories/web-programming/http_types_and_interfaces.md](./bk/drafts/categories/web-programming/http_types_and_interfaces.md#L32))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1338) ([bk/drafts/categories/web-programming/http_types_and_interfaces.md](./bk/drafts/categories/web-programming/http_types_and_interfaces.md#L38))

### bk/drafts/categories/web-programming/index.md

- [ ] [web-programming/index: organize; need full review; further cross link](https://github.com/john-cd/rust_howto/issues/500) ([bk/drafts/categories/web-programming/index.md](./bk/drafts/categories/web-programming/index.md#L89))

### bk/drafts/categories/web-programming/mime.md

- [ ] Resolve TODO/FIXME at line 41 ([bk/drafts/categories/web-programming/mime.md](./bk/drafts/categories/web-programming/mime.md#L41))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/971) ([bk/drafts/categories/web-programming/mime.md](./bk/drafts/categories/web-programming/mime.md#L47))

### bk/drafts/categories/web-programming/scraping.md

- [ ] Call `get_base_url`{{hi:get_base_url}} to retrieve the base [URL][p~url]{{hi:Base URL}}. If the document has a base tag, get the href{{hi:href}} [`select::node::Node::attr`][c~select::node::Node::attr~docs]↗{{hi:select::node::Node::attr}} from base tag. [`select::node::Node::attr`][c~select::node::Node::attr~docs]↗{{hi:select::node::Node::attr}} of the original URL acts as a default ([bk/drafts/categories/web-programming/scraping.md](./bk/drafts/categories/web-programming/scraping.md#L19))
- [ ] Resolve TODO/FIXME at line 42 ([bk/drafts/categories/web-programming/scraping.md](./bk/drafts/categories/web-programming/scraping.md#L42))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/972) ([bk/drafts/categories/web-programming/scraping.md](./bk/drafts/categories/web-programming/scraping.md#L48))

### bk/drafts/categories/web-programming/url.md

- [ ] Resolve TODO/FIXME at line 67 ([bk/drafts/categories/web-programming/url.md](./bk/drafts/categories/web-programming/url.md#L67))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/973) ([bk/drafts/categories/web-programming/url.md](./bk/drafts/categories/web-programming/url.md#L73))

### bk/drafts/categories/web-programming_http-client/apis.md

- [ ] Resolve TODO/FIXME at line 65 ([bk/drafts/categories/web-programming_http-client/apis.md](./bk/drafts/categories/web-programming_http-client/apis.md#L65))
- [ ] [review / fix](https://github.com/john-cd/rust_howto/issues/974) ([bk/drafts/categories/web-programming_http-client/apis.md](./bk/drafts/categories/web-programming_http-client/apis.md#L71))

### bk/drafts/categories/web-programming_http-client/download.md

- [ ] Resolve TODO/FIXME at line 44 ([bk/drafts/categories/web-programming_http-client/download.md](./bk/drafts/categories/web-programming_http-client/download.md#L44))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/975) ([bk/drafts/categories/web-programming_http-client/download.md](./bk/drafts/categories/web-programming_http-client/download.md#L50))

### bk/drafts/categories/web-programming_http-client/http_clients.md

- [ ] Resolve TODO/FIXME at line 48 ([bk/drafts/categories/web-programming_http-client/http_clients.md](./bk/drafts/categories/web-programming_http-client/http_clients.md#L48))
- [ ] [http_clients: expand; link to hyper.md in server.](https://github.com/john-cd/rust_howto/issues/504) ([bk/drafts/categories/web-programming_http-client/http_clients.md](./bk/drafts/categories/web-programming_http-client/http_clients.md#L54))

### bk/drafts/categories/web-programming_http-client/index.md

- [ ] [web-programming_http-client/index: fix, sync table and contents](https://github.com/john-cd/rust_howto/issues/505) ([bk/drafts/categories/web-programming_http-client/index.md](./bk/drafts/categories/web-programming_http-client/index.md#L55))

### bk/drafts/categories/web-programming_http-client/requests.md

- [ ] Resolve TODO/FIXME at line 48 ([bk/drafts/categories/web-programming_http-client/requests.md](./bk/drafts/categories/web-programming_http-client/requests.md#L48))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/976) ([bk/drafts/categories/web-programming_http-client/requests.md](./bk/drafts/categories/web-programming_http-client/requests.md#L54))

### bk/drafts/categories/web-programming_http-server/_actix.md

- [ ] Resolve TODO/FIXME at line 24 ([bk/drafts/categories/web-programming_http-server/_actix.md](./bk/drafts/categories/web-programming_http-server/_actix.md#L24))
- [ ] [actix: organize / write](https://github.com/john-cd/rust_howto/issues/506) ([bk/drafts/categories/web-programming_http-server/_actix.md](./bk/drafts/categories/web-programming_http-server/_actix.md#L30))

### bk/drafts/categories/web-programming_http-server/_axum.md

- [ ] Resolve TODO/FIXME at line 24 ([bk/drafts/categories/web-programming_http-server/_axum.md](./bk/drafts/categories/web-programming_http-server/_axum.md#L24))
- [ ] [axum: write](https://github.com/john-cd/rust_howto/issues/507) ([bk/drafts/categories/web-programming_http-server/_axum.md](./bk/drafts/categories/web-programming_http-server/_axum.md#L30))

### bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md

- [ ] Resolve TODO/FIXME at line 29 ([bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md](./bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md#L29))
- [ ] [batteries-included_frameworks: write](https://github.com/john-cd/rust_howto/issues/509) ([bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md](./bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md#L35))

### bk/drafts/categories/web-programming_http-server/_graphql.md

- [ ] Resolve TODO/FIXME at line 32 ([bk/drafts/categories/web-programming_http-server/_graphql.md](./bk/drafts/categories/web-programming_http-server/_graphql.md#L32))
- [ ] [graphql: write](https://github.com/john-cd/rust_howto/issues/511) integrate in seography graphql section? ([bk/drafts/categories/web-programming_http-server/_graphql.md](./bk/drafts/categories/web-programming_http-server/_graphql.md#L38))

### bk/drafts/categories/web-programming_http-server/_grpc.md

- [ ] Resolve TODO/FIXME at line 25 ([bk/drafts/categories/web-programming_http-server/_grpc.md](./bk/drafts/categories/web-programming_http-server/_grpc.md#L25))
- [ ] [grpc: write](https://github.com/john-cd/rust_howto/issues/514) ([bk/drafts/categories/web-programming_http-server/_grpc.md](./bk/drafts/categories/web-programming_http-server/_grpc.md#L31))

### bk/drafts/categories/web-programming_http-server/_hyper.md

- [ ] Resolve TODO/FIXME at line 51 ([bk/drafts/categories/web-programming_http-server/_hyper.md](./bk/drafts/categories/web-programming_http-server/_hyper.md#L51))
- [ ] [hyper: fix](https://github.com/john-cd/rust_howto/issues/515) ([bk/drafts/categories/web-programming_http-server/_hyper.md](./bk/drafts/categories/web-programming_http-server/_hyper.md#L57))

### bk/drafts/categories/web-programming_http-server/cors.md

- [ ] Resolve TODO/FIXME at line 19 ([bk/drafts/categories/web-programming_http-server/cors.md](./bk/drafts/categories/web-programming_http-server/cors.md#L19))
- [ ] [cors: write](https://github.com/john-cd/rust_howto/issues/510) ([bk/drafts/categories/web-programming_http-server/cors.md](./bk/drafts/categories/web-programming_http-server/cors.md#L25))

### bk/drafts/categories/web-programming_http-server/index.md

- [ ] [write / review in depth](https://github.com/john-cd/rust_howto/issues/977) ([bk/drafts/categories/web-programming_http-server/index.md](./bk/drafts/categories/web-programming_http-server/index.md#L93))

### bk/drafts/categories/web-programming_http-server/middleware.md

- [ ] Resolve TODO/FIXME at line 37 ([bk/drafts/categories/web-programming_http-server/middleware.md](./bk/drafts/categories/web-programming_http-server/middleware.md#L37))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/978) ([bk/drafts/categories/web-programming_http-server/middleware.md](./bk/drafts/categories/web-programming_http-server/middleware.md#L43))

### bk/drafts/categories/web-programming_http-server/other_frameworks.md

- [ ] Resolve TODO/FIXME at line 27 ([bk/drafts/categories/web-programming_http-server/other_frameworks.md](./bk/drafts/categories/web-programming_http-server/other_frameworks.md#L27))
- [ ] [other_frameworks: organize](https://github.com/john-cd/rust_howto/issues/518) ([bk/drafts/categories/web-programming_http-server/other_frameworks.md](./bk/drafts/categories/web-programming_http-server/other_frameworks.md#L33))

### bk/drafts/categories/web-programming_http-server/static_website_generators.md

- [ ] Resolve TODO/FIXME at line 52 ([bk/drafts/categories/web-programming_http-server/static_website_generators.md](./bk/drafts/categories/web-programming_http-server/static_website_generators.md#L52))
- [ ] [static_website_generators: write](https://github.com/john-cd/rust_howto/issues/519) ([bk/drafts/categories/web-programming_http-server/static_website_generators.md](./bk/drafts/categories/web-programming_http-server/static_website_generators.md#L58))

### bk/drafts/contributing/vscode.md

- [ ] Resolve TODO/FIXME at line 87 ([bk/drafts/contributing/vscode.md](./bk/drafts/contributing/vscode.md#L87))
- [ ] write ([bk/drafts/contributing/vscode.md](./bk/drafts/contributing/vscode.md#L93))

### bk/drafts/indices/crates_and_examples.md

- [ ] [create script to generate crate pages](https://github.com/john-cd/rust_howto/issues/1395) ([bk/drafts/indices/crates_and_examples.md](./bk/drafts/indices/crates_and_examples.md#L17))

### bk/drafts/indices/crates_by_category2.md

- [ ] [crates_by_category: verify all crates are added to at least one category](https://github.com/john-cd/rust_howto/issues/534) ([bk/drafts/indices/crates_by_category2.md](./bk/drafts/indices/crates_by_category2.md#L2))

### bk/src/appendices/contributing/book_editing_and_example_code_development.md

- [ ] [development_editing: review](https://github.com/john-cd/rust_howto/issues/523) ([bk/src/appendices/contributing/book_editing_and_example_code_development.md](./bk/src/appendices/contributing/book_editing_and_example_code_development.md#L32))

### bk/src/appendices/contributing/dev_container_and_docker.md

- [ ] [dev_container_docker: review; rust and Docker; multistage builds](https://github.com/john-cd/rust_howto/issues/525) ([bk/src/appendices/contributing/dev_container_and_docker.md](./bk/src/appendices/contributing/dev_container_and_docker.md#L92))

### bk/src/appendices/contributing/index.md

- [ ] - [list][rust-howto~~repo]↗ ([bk/src/appendices/contributing/index.md](./bk/src/appendices/contributing/index.md#L154))
- [ ] [Documentation: Finalize Contributing Guide and Styleguides](https://github.com/john-cd/rust_howto/issues/529) ([bk/src/appendices/contributing/index.md](./bk/src/appendices/contributing/index.md#L175))

### bk/src/appendices/contributing/topics_of_interest.md

- [ ] Please also consult the [`.md`][rust-howto~~repo]↗ file and the [`drafts`][rust-howto~drafts~repo]↗ and [`later`][rust-howto~bk-later~repo]↗ folders ([bk/src/appendices/contributing/topics_of_interest.md](./bk/src/appendices/contributing/topics_of_interest.md#L65))

### bk/src/categories/data-structures/index.md

- [ ] [data-structures: expand](https://github.com/john-cd/rust_howto/issues/280) ([bk/src/categories/data-structures/index.md](./bk/src/categories/data-structures/index.md#L101))

### bk/src/categories/mathematics/complex_numbers.md

- [ ] [final review](https://github.com/john-cd/rust_howto/issues/935) ([bk/src/categories/mathematics/complex_numbers.md](./bk/src/categories/mathematics/complex_numbers.md#L64))

### bk/src/categories/mathematics/trigonometry.md

- [ ] [final review](https://github.com/john-cd/rust_howto/issues/938) ([bk/src/categories/mathematics/trigonometry.md](./bk/src/categories/mathematics/trigonometry.md#L45))

### bk/src/language/macros.md

- [ ] [`!`][c~std::~docs]↗ is a placeholder for not-yet-implemented code. If executed, it will cause a panic with a message indicating that the functionality is "not yet implemented." It's useful during development to mark areas that still need attention. You may also use the similar [`unimplemented!`][c~std::unimplemented~docs]↗ ([bk/src/language/macros.md](./bk/src/language/macros.md#L119))
- [ ] split? ([bk/src/language/macros.md](./bk/src/language/macros.md#L427))

### bk/src/links/learning_rust.md

- [ ] Resolve TODO/FIXME at line 7 ([bk/src/links/learning_rust.md](./bk/src/links/learning_rust.md#L7))

### bk/src/refs/other-refs.md

- [ ] [c~std::~docs]: https://doc.rust-lang.org/std/macro..html ([bk/src/refs/other-refs.md](./bk/src/refs/other-refs.md#L805))
- [ ] [rust-howto~~repo]: https://github.com/john-cd/rust_howto/blob/main/.md ([bk/src/refs/other-refs.md](./bk/src/refs/other-refs.md#L1446))
- [ ] [rust-howto~~repo~badge]: https://img.shields.io/badge/rust__howto_TODO-steelblue?logo=github ([bk/src/refs/other-refs.md](./bk/src/refs/other-refs.md#L1447))

### bk/src/standard-library/dynamic_typing.md

- [ ] review ([bk/src/standard-library/dynamic_typing.md](./bk/src/standard-library/dynamic_typing.md#L54))

### later/src/categories/aerospace_protocols/index.md

- [ ] [aerospace protocols: write](https://github.com/john-cd/rust_howto/issues/196) ([later/src/categories/aerospace_protocols/index.md](./later/src/categories/aerospace_protocols/index.md#L66))

### later/src/categories/aerospace_simulation/aerospace_simulation.md

- [ ] [aerospace_simulation: write](https://github.com/john-cd/rust_howto/issues/199) ([later/src/categories/aerospace_simulation/aerospace_simulation.md](./later/src/categories/aerospace_simulation/aerospace_simulation.md#L20))

### later/src/categories/aerospace_space-protocols/space_protocols.md

- [ ] [space_protocols: write](https://github.com/john-cd/rust_howto/issues/201) ([later/src/categories/aerospace_space-protocols/space_protocols.md](./later/src/categories/aerospace_space-protocols/space_protocols.md#L20))

### later/src/categories/compilers/index.md

- [ ] Resolve TODO/FIXME at line 15 ([later/src/categories/compilers/index.md](./later/src/categories/compilers/index.md#L15))
- [ ] Resolve TODO/FIXME at line 19 ([later/src/categories/compilers/index.md](./later/src/categories/compilers/index.md#L19))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/908) ([later/src/categories/compilers/index.md](./later/src/categories/compilers/index.md#L78))

### later/src/categories/computer-vision/opencv.md

- [ ] [opencv: expand, add example](https://github.com/john-cd/rust_howto/issues/257) ([later/src/categories/computer-vision/opencv.md](./later/src/categories/computer-vision/opencv.md#L51))

### later/src/categories/cryptography_cryptocurrencies/index.md

- [ ] Resolve TODO/FIXME at line 32 ([later/src/categories/cryptography_cryptocurrencies/index.md](./later/src/categories/cryptography_cryptocurrencies/index.md#L32))
- [ ] [cryptocurrencies: write](https://github.com/john-cd/rust_howto/issues/278) ([later/src/categories/cryptography_cryptocurrencies/index.md](./later/src/categories/cryptography_cryptocurrencies/index.md#L38))

### later/src/categories/development-tools_ffi/erlang_elixir.md

- [ ] find proper crate ([later/src/categories/development-tools_ffi/erlang_elixir.md](./later/src/categories/development-tools_ffi/erlang_elixir.md#L13))
- [ ] Resolve TODO/FIXME at line 47 ([later/src/categories/development-tools_ffi/erlang_elixir.md](./later/src/categories/development-tools_ffi/erlang_elixir.md#L47))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1070) ([later/src/categories/development-tools_ffi/erlang_elixir.md](./later/src/categories/development-tools_ffi/erlang_elixir.md#L53))

### later/src/categories/development-tools_ffi/flutter.md

- [ ] Resolve TODO/FIXME at line 51 ([later/src/categories/development-tools_ffi/flutter.md](./later/src/categories/development-tools_ffi/flutter.md#L51))
- [ ] [write; review in depth](https://github.com/john-cd/rust_howto/issues/1071) ([later/src/categories/development-tools_ffi/flutter.md](./later/src/categories/development-tools_ffi/flutter.md#L57))

### later/src/categories/development-tools_ffi/generate_ffi_bindings.md

- [ ] [generate_ffi_bindings: write](https://github.com/john-cd/rust_howto/issues/324) ([later/src/categories/development-tools_ffi/generate_ffi_bindings.md](./later/src/categories/development-tools_ffi/generate_ffi_bindings.md#L85))

### later/src/categories/development-tools_ffi/index.md

- [ ] Resolve TODO/FIXME at line 80 ([later/src/categories/development-tools_ffi/index.md](./later/src/categories/development-tools_ffi/index.md#L80))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/325) ([later/src/categories/development-tools_ffi/index.md](./later/src/categories/development-tools_ffi/index.md#L91))

### later/src/categories/development-tools_ffi/java.md

- [ ] Resolve TODO/FIXME at line 17 ([later/src/categories/development-tools_ffi/java.md](./later/src/categories/development-tools_ffi/java.md#L17))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1072) ([later/src/categories/development-tools_ffi/java.md](./later/src/categories/development-tools_ffi/java.md#L23))

### later/src/categories/development-tools_ffi/lua.md

- [ ] Resolve TODO/FIXME at line 21 ([later/src/categories/development-tools_ffi/lua.md](./later/src/categories/development-tools_ffi/lua.md#L21))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1073) ([later/src/categories/development-tools_ffi/lua.md](./later/src/categories/development-tools_ffi/lua.md#L27))

### later/src/categories/development-tools_ffi/node.md

- [ ] Resolve TODO/FIXME at line 27 ([later/src/categories/development-tools_ffi/node.md](./later/src/categories/development-tools_ffi/node.md#L27))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1074) ([later/src/categories/development-tools_ffi/node.md](./later/src/categories/development-tools_ffi/node.md#L33))

### later/src/categories/development-tools_ffi/objc.md

- [ ] Resolve TODO/FIXME at line 17 ([later/src/categories/development-tools_ffi/objc.md](./later/src/categories/development-tools_ffi/objc.md#L17))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1075) ([later/src/categories/development-tools_ffi/objc.md](./later/src/categories/development-tools_ffi/objc.md#L23))

### later/src/categories/development-tools_ffi/python.md

- [ ] Resolve TODO/FIXME at line 57 ([later/src/categories/development-tools_ffi/python.md](./later/src/categories/development-tools_ffi/python.md#L57))
- [ ] [python_interop: write](https://github.com/john-cd/rust_howto/issues/210) ([later/src/categories/development-tools_ffi/python.md](./later/src/categories/development-tools_ffi/python.md#L63))

### later/src/categories/development-tools_ffi/ruby.md

- [ ] Resolve TODO/FIXME at line 47 ([later/src/categories/development-tools_ffi/ruby.md](./later/src/categories/development-tools_ffi/ruby.md#L47))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1076) ([later/src/categories/development-tools_ffi/ruby.md](./later/src/categories/development-tools_ffi/ruby.md#L53))

### later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md

- [ ] Resolve TODO/FIXME at line 21 ([later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md](./later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md#L21))
- [ ] [compile_macros: write](https://github.com/john-cd/rust_howto/issues/327) ([later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md](./later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md#L27))

### later/src/categories/development-tools_procedural-macro-helpers/index.md

- [ ] Resolve TODO/FIXME at line 25 ([later/src/categories/development-tools_procedural-macro-helpers/index.md](./later/src/categories/development-tools_procedural-macro-helpers/index.md#L25))
- [ ] [fix](https://github.com/john-cd/rust_howto/issues/332) ([later/src/categories/development-tools_procedural-macro-helpers/index.md](./later/src/categories/development-tools_procedural-macro-helpers/index.md#L31))

### later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md

- [ ] Resolve TODO/FIXME at line 18 ([later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md](./later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md#L18))
- [ ] [tools: write](https://github.com/john-cd/rust_howto/issues/329) ([later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md](./later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md#L24))

### later/src/categories/development-tools_procedural-macro-helpers/write_proc_macros.md

- [ ] [write_proc_macros: write; compare with macros.md - what should be in here?](https://github.com/john-cd/rust_howto/issues/331) ([later/src/categories/development-tools_procedural-macro-helpers/write_proc_macros.md](./later/src/categories/development-tools_procedural-macro-helpers/write_proc_macros.md#L105))

### later/src/categories/embedded/embassy.md

- [ ] Resolve TODO/FIXME at line 20 ([later/src/categories/embedded/embassy.md](./later/src/categories/embedded/embassy.md#L20))
- [ ] [embassy: add embassy, others](https://github.com/john-cd/rust_howto/issues/345) ([later/src/categories/embedded/embassy.md](./later/src/categories/embedded/embassy.md#L26))

### later/src/categories/embedded/flash.md

- [ ] Resolve TODO/FIXME at line 9 ([later/src/categories/embedded/flash.md](./later/src/categories/embedded/flash.md#L9))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1200) ([later/src/categories/embedded/flash.md](./later/src/categories/embedded/flash.md#L15))

### later/src/categories/embedded/hals.md

- [ ] Resolve TODO/FIXME at line 33 ([later/src/categories/embedded/hals.md](./later/src/categories/embedded/hals.md#L33))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1203) ([later/src/categories/embedded/hals.md](./later/src/categories/embedded/hals.md#L39))

### later/src/categories/embedded/index.md

- [ ] Resolve TODO/FIXME at line 40 ([later/src/categories/embedded/index.md](./later/src/categories/embedded/index.md#L40))
- [ ] Resolve TODO/FIXME at line 44 ([later/src/categories/embedded/index.md](./later/src/categories/embedded/index.md#L44))
- [ ] Resolve TODO/FIXME at line 49 ([later/src/categories/embedded/index.md](./later/src/categories/embedded/index.md#L49))
- [ ] [write; cover](https://github.com/john-cd/rust_howto/issues/346) ([later/src/categories/embedded/index.md](./later/src/categories/embedded/index.md#L75))

### later/src/categories/embedded/pacs.md

- [ ] Resolve TODO/FIXME at line 15 ([later/src/categories/embedded/pacs.md](./later/src/categories/embedded/pacs.md#L15))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1198) ([later/src/categories/embedded/pacs.md](./later/src/categories/embedded/pacs.md#L21))

### later/src/categories/embedded/rtos.md

- [ ] Resolve TODO/FIXME at line 11 ([later/src/categories/embedded/rtos.md](./later/src/categories/embedded/rtos.md#L11))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1202) ([later/src/categories/embedded/rtos.md](./later/src/categories/embedded/rtos.md#L17))

### later/src/categories/embedded/sensors.md

- [ ] Resolve TODO/FIXME at line 7 ([later/src/categories/embedded/sensors.md](./later/src/categories/embedded/sensors.md#L7))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1199) ([later/src/categories/embedded/sensors.md](./later/src/categories/embedded/sensors.md#L13))

### later/src/categories/embedded/useful_crates_embedded.md

- [ ] Resolve TODO/FIXME at line 33 ([later/src/categories/embedded/useful_crates_embedded.md](./later/src/categories/embedded/useful_crates_embedded.md#L33))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1201) ([later/src/categories/embedded/useful_crates_embedded.md](./later/src/categories/embedded/useful_crates_embedded.md#L39))

### later/src/categories/emulators/emulators.md

- [ ] Resolve TODO/FIXME at line 23 ([later/src/categories/emulators/emulators.md](./later/src/categories/emulators/emulators.md#L23))
- [ ] [emulators: write](https://github.com/john-cd/rust_howto/issues/347) ([later/src/categories/emulators/emulators.md](./later/src/categories/emulators/emulators.md#L29))

### later/src/categories/emulators/index.md

- [ ] Resolve TODO/FIXME at line 57 ([later/src/categories/emulators/index.md](./later/src/categories/emulators/index.md#L57))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/348) ([later/src/categories/emulators/index.md](./later/src/categories/emulators/index.md#L63))

### later/src/categories/external-ffi-bindings/external_ffi_bindings.md

- [ ] Resolve TODO/FIXME at line 11 ([later/src/categories/external-ffi-bindings/external_ffi_bindings.md](./later/src/categories/external-ffi-bindings/external_ffi_bindings.md#L11))
- [ ] [external_ffi_bindings: write](https://github.com/john-cd/rust_howto/issues/355) ([later/src/categories/external-ffi-bindings/external_ffi_bindings.md](./later/src/categories/external-ffi-bindings/external_ffi_bindings.md#L17))

### later/src/categories/external-ffi-bindings/index.md

- [ ] Resolve TODO/FIXME at line 22 ([later/src/categories/external-ffi-bindings/index.md](./later/src/categories/external-ffi-bindings/index.md#L22))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/356) ([later/src/categories/external-ffi-bindings/index.md](./later/src/categories/external-ffi-bindings/index.md#L28))

### later/src/categories/finance/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/365) ([later/src/categories/finance/index.md](./later/src/categories/finance/index.md#L85))

### later/src/categories/finance/quant.md

- [ ] [quant: write](https://github.com/john-cd/rust_howto/issues/364) ([later/src/categories/finance/quant.md](./later/src/categories/finance/quant.md#L25))

### later/src/categories/game-development/game_development.md

- [ ] [game_development: identify crates; write](https://github.com/john-cd/rust_howto/issues/366) ([later/src/categories/game-development/game_development.md](./later/src/categories/game-development/game_development.md#L34))

### later/src/categories/game-development/index.md

- [ ] [expand](https://github.com/john-cd/rust_howto/issues/367) ([later/src/categories/game-development/index.md](./later/src/categories/game-development/index.md#L67))

### later/src/categories/game-engines/game_engines.md

- [ ] Resolve TODO/FIXME at line 51 ([later/src/categories/game-engines/game_engines.md](./later/src/categories/game-engines/game_engines.md#L51))
- [ ] [game_engines: write](https://github.com/john-cd/rust_howto/issues/369) ([later/src/categories/game-engines/game_engines.md](./later/src/categories/game-engines/game_engines.md#L57))

### later/src/categories/game-engines/index.md

- [ ] [expand](https://github.com/john-cd/rust_howto/issues/370) ([later/src/categories/game-engines/index.md](./later/src/categories/game-engines/index.md#L41))

### later/src/categories/games/index.md

- [ ] Resolve TODO/FIXME at line 25 ([later/src/categories/games/index.md](./later/src/categories/games/index.md#L25))
- [ ] [write; need in depth review; review [are-we-game-yet? games][are-we-game-yet?-games~website] ](https://github.com/john-cd/rust_howto/issues/373) ([later/src/categories/games/index.md](./later/src/categories/games/index.md#L31))

### later/src/categories/gui/clipboard.md

- [ ] Resolve TODO/FIXME at line 27 ([later/src/categories/gui/clipboard.md](./later/src/categories/gui/clipboard.md#L27))
- [ ] [write; expand](https://github.com/john-cd/rust_howto/issues/638)? ([later/src/categories/gui/clipboard.md](./later/src/categories/gui/clipboard.md#L33))

### later/src/categories/gui/file_dialogs.md

- [ ] Resolve TODO/FIXME at line 29 ([later/src/categories/gui/file_dialogs.md](./later/src/categories/gui/file_dialogs.md#L29))
- [ ] [file_dialogs: write](https://github.com/john-cd/rust_howto/issues/381) ([later/src/categories/gui/file_dialogs.md](./later/src/categories/gui/file_dialogs.md#L35))

### later/src/categories/gui/gtk.md

- [ ] Resolve TODO/FIXME at line 45 ([later/src/categories/gui/gtk.md](./later/src/categories/gui/gtk.md#L45))
- [ ] [gtk: write](https://github.com/john-cd/rust_howto/issues/383) ([later/src/categories/gui/gtk.md](./later/src/categories/gui/gtk.md#L51))

### later/src/categories/gui/immediate_mode_gui.md

- [ ] Resolve TODO/FIXME at line 49 ([later/src/categories/gui/immediate_mode_gui.md](./later/src/categories/gui/immediate_mode_gui.md#L49))
- [ ] [immediate_mode_gui: write](https://github.com/john-cd/rust_howto/issues/385) ([later/src/categories/gui/immediate_mode_gui.md](./later/src/categories/gui/immediate_mode_gui.md#L55))

### later/src/categories/gui/index.md

- [ ] [organize](https://github.com/john-cd/rust_howto/issues/397) ([later/src/categories/gui/index.md](./later/src/categories/gui/index.md#L109))

### later/src/categories/gui/retained_mode_gui.md

- [ ] Resolve TODO/FIXME at line 91 ([later/src/categories/gui/retained_mode_gui.md](./later/src/categories/gui/retained_mode_gui.md#L91))
- [ ] [retained_mode_gui: organize/write](https://github.com/john-cd/rust_howto/issues/389) ([later/src/categories/gui/retained_mode_gui.md](./later/src/categories/gui/retained_mode_gui.md#L97))

### later/src/categories/gui/text_layout.md

- [ ] Resolve TODO/FIXME at line 27 ([later/src/categories/gui/text_layout.md](./later/src/categories/gui/text_layout.md#L27))
- [ ] [text_layout: write](https://github.com/john-cd/rust_howto/issues/391) ([later/src/categories/gui/text_layout.md](./later/src/categories/gui/text_layout.md#L33))

### later/src/categories/gui/ui_layout.md

- [ ] Resolve TODO/FIXME at line 36 ([later/src/categories/gui/ui_layout.md](./later/src/categories/gui/ui_layout.md#L36))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/934) ([later/src/categories/gui/ui_layout.md](./later/src/categories/gui/ui_layout.md#L42))

### later/src/categories/gui/web_based_gui.md

- [ ] Resolve TODO/FIXME at line 39 ([later/src/categories/gui/web_based_gui.md](./later/src/categories/gui/web_based_gui.md#L39))
- [ ] [web_based_gui: write](https://github.com/john-cd/rust_howto/issues/394) ([later/src/categories/gui/web_based_gui.md](./later/src/categories/gui/web_based_gui.md#L45))

### later/src/categories/gui/window_creation.md

- [ ] Resolve TODO/FIXME at line 44 ([later/src/categories/gui/window_creation.md](./later/src/categories/gui/window_creation.md#L44))
- [ ] [window_creation: write](https://github.com/john-cd/rust_howto/issues/396) ([later/src/categories/gui/window_creation.md](./later/src/categories/gui/window_creation.md#L50))

### later/src/categories/internationalization/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/402) ([later/src/categories/internationalization/index.md](./later/src/categories/internationalization/index.md#L80))

### later/src/categories/internationalization/internationalization.md

- [ ] Resolve TODO/FIXME at line 17 ([later/src/categories/internationalization/internationalization.md](./later/src/categories/internationalization/internationalization.md#L17))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/401) ([later/src/categories/internationalization/internationalization.md](./later/src/categories/internationalization/internationalization.md#L23))

### later/src/categories/localization/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/405) ([later/src/categories/localization/index.md](./later/src/categories/localization/index.md#L30))

### later/src/categories/localization/localization.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/localization/localization.md](./later/src/categories/localization/localization.md#L13))
- [ ] [localization: write](https://github.com/john-cd/rust_howto/issues/404) ([later/src/categories/localization/localization.md](./later/src/categories/localization/localization.md#L19))

### later/src/categories/multimedia/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/940) ([later/src/categories/multimedia/index.md](./later/src/categories/multimedia/index.md#L66))

### later/src/categories/multimedia/multimedia.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/multimedia/multimedia.md](./later/src/categories/multimedia/multimedia.md#L13))
- [ ] [multimedia: write](https://github.com/john-cd/rust_howto/issues/413) ([later/src/categories/multimedia/multimedia.md](./later/src/categories/multimedia/multimedia.md#L19))

### later/src/categories/multimedia_audio/audio.md

- [ ] [audio: write](https://github.com/john-cd/rust_howto/issues/415) ([later/src/categories/multimedia_audio/audio.md](./later/src/categories/multimedia_audio/audio.md#L21))

### later/src/categories/multimedia_audio/index.md

- [ ] Resolve TODO/FIXME at line 30 ([later/src/categories/multimedia_audio/index.md](./later/src/categories/multimedia_audio/index.md#L30))
- [ ] Resolve TODO/FIXME at line 34 ([later/src/categories/multimedia_audio/index.md](./later/src/categories/multimedia_audio/index.md#L34))
- [ ] [fix](https://github.com/john-cd/rust_howto/issues/941) ([later/src/categories/multimedia_audio/index.md](./later/src/categories/multimedia_audio/index.md#L56))

### later/src/categories/multimedia_encoding/encoding.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/multimedia_encoding/encoding.md](./later/src/categories/multimedia_encoding/encoding.md#L13))
- [ ] [encoding: write](https://github.com/john-cd/rust_howto/issues/417) ([later/src/categories/multimedia_encoding/encoding.md](./later/src/categories/multimedia_encoding/encoding.md#L19))

### later/src/categories/multimedia_encoding/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/418) ([later/src/categories/multimedia_encoding/index.md](./later/src/categories/multimedia_encoding/index.md#L42))

### later/src/categories/multimedia_images/color_handling.md

- [ ] Resolve TODO/FIXME at line 15 ([later/src/categories/multimedia_images/color_handling.md](./later/src/categories/multimedia_images/color_handling.md#L15))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1223) ([later/src/categories/multimedia_images/color_handling.md](./later/src/categories/multimedia_images/color_handling.md#L21))

### later/src/categories/multimedia_images/images.md

- [ ] Resolve TODO/FIXME at line 52 ([later/src/categories/multimedia_images/images.md](./later/src/categories/multimedia_images/images.md#L52))
- [ ] [images: write](https://github.com/john-cd/rust_howto/issues/420) ([later/src/categories/multimedia_images/images.md](./later/src/categories/multimedia_images/images.md#L58))

### later/src/categories/multimedia_images/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/942) ([later/src/categories/multimedia_images/index.md](./later/src/categories/multimedia_images/index.md#L58))

### later/src/categories/multimedia_images/pixel_buffers.md

- [ ] Resolve TODO/FIXME at line 15 ([later/src/categories/multimedia_images/pixel_buffers.md](./later/src/categories/multimedia_images/pixel_buffers.md#L15))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1222) ([later/src/categories/multimedia_images/pixel_buffers.md](./later/src/categories/multimedia_images/pixel_buffers.md#L21))

### later/src/categories/multimedia_video/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/943) ([later/src/categories/multimedia_video/index.md](./later/src/categories/multimedia_video/index.md#L46))

### later/src/categories/multimedia_video/video.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/422) ([later/src/categories/multimedia_video/video.md](./later/src/categories/multimedia_video/video.md#L20))

### later/src/categories/no-std/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/427) ([later/src/categories/no-std/index.md](./later/src/categories/no-std/index.md#L50))

### later/src/categories/no-std/no_std.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/no-std/no_std.md](./later/src/categories/no-std/no_std.md#L13))
- [ ] [no_std: write](https://github.com/john-cd/rust_howto/issues/426) ([later/src/categories/no-std/no_std.md](./later/src/categories/no-std/no_std.md#L19))

### later/src/categories/no-std_no-alloc/index.md

- [ ] [review; cover use of no_std attribute to remove alloc crate](https://github.com/john-cd/rust_howto/issues/945) ([later/src/categories/no-std_no-alloc/index.md](./later/src/categories/no-std_no-alloc/index.md#L63))

### later/src/categories/no-std_no-alloc/no_alloc.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/no-std_no-alloc/no_alloc.md](./later/src/categories/no-std_no-alloc/no_alloc.md#L13))
- [ ] [no_alloc: write](https://github.com/john-cd/rust_howto/issues/428) ([later/src/categories/no-std_no-alloc/no_alloc.md](./later/src/categories/no-std_no-alloc/no_alloc.md#L19))

### later/src/categories/os_freebsd-apis/freebsd.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/os_freebsd-apis/freebsd.md](./later/src/categories/os_freebsd-apis/freebsd.md#L13))
- [ ] [freebsd: write](https://github.com/john-cd/rust_howto/issues/433) ([later/src/categories/os_freebsd-apis/freebsd.md](./later/src/categories/os_freebsd-apis/freebsd.md#L19))

### later/src/categories/os_freebsd-apis/index.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/947) ([later/src/categories/os_freebsd-apis/index.md](./later/src/categories/os_freebsd-apis/index.md#L35))

### later/src/categories/os_linux-apis/index.md

- [ ] Resolve TODO/FIXME at line 60 ([later/src/categories/os_linux-apis/index.md](./later/src/categories/os_linux-apis/index.md#L60))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/948) ([later/src/categories/os_linux-apis/index.md](./later/src/categories/os_linux-apis/index.md#L66))

### later/src/categories/os_linux-apis/linux.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/os_linux-apis/linux.md](./later/src/categories/os_linux-apis/linux.md#L13))
- [ ] [linux: write](https://github.com/john-cd/rust_howto/issues/434) ([later/src/categories/os_linux-apis/linux.md](./later/src/categories/os_linux-apis/linux.md#L19))

### later/src/categories/os_macos-apis/index.md

- [ ] Resolve TODO/FIXME at line 57 ([later/src/categories/os_macos-apis/index.md](./later/src/categories/os_macos-apis/index.md#L57))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/949) ([later/src/categories/os_macos-apis/index.md](./later/src/categories/os_macos-apis/index.md#L63))

### later/src/categories/os_macos-apis/macos.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/os_macos-apis/macos.md](./later/src/categories/os_macos-apis/macos.md#L13))
- [ ] [macos: write](https://github.com/john-cd/rust_howto/issues/435) ([later/src/categories/os_macos-apis/macos.md](./later/src/categories/os_macos-apis/macos.md#L19))

### later/src/categories/rendering/2d_raster_graphics.md

- [ ] [write. decide what to cover.](https://github.com/john-cd/rust_howto/issues/1215) ([later/src/categories/rendering/2d_raster_graphics.md](./later/src/categories/rendering/2d_raster_graphics.md#L35))

### later/src/categories/rendering/2d_renderers.md

- [ ] [2d_renderers: write; titles](https://github.com/john-cd/rust_howto/issues/377) ([later/src/categories/rendering/2d_renderers.md](./later/src/categories/rendering/2d_renderers.md#L98))

### later/src/categories/rendering/2d_vector_graphics.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1214) ([later/src/categories/rendering/2d_vector_graphics.md](./later/src/categories/rendering/2d_vector_graphics.md#L53))

### later/src/categories/rendering/3d_renderers.md

- [ ] [write. decide what to cover](https://github.com/john-cd/rust_howto/issues/1213) ([later/src/categories/rendering/3d_renderers.md](./later/src/categories/rendering/3d_renderers.md#L66))

### later/src/categories/rendering/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/953) ([later/src/categories/rendering/index.md](./later/src/categories/rendering/index.md#L78))

### later/src/categories/rendering/svg_rendering.md

- [ ] Resolve TODO/FIXME at line 21 ([later/src/categories/rendering/svg_rendering.md](./later/src/categories/rendering/svg_rendering.md#L21))
- [ ] [write.](https://github.com/john-cd/rust_howto/issues/1216) ([later/src/categories/rendering/svg_rendering.md](./later/src/categories/rendering/svg_rendering.md#L27))

### later/src/categories/rendering/text_rendering.md

- [ ] [write / decide what to cover](https://github.com/john-cd/rust_howto/issues/1212) ([later/src/categories/rendering/text_rendering.md](./later/src/categories/rendering/text_rendering.md#L48))

### later/src/categories/rendering_data-formats/data_formats.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/rendering_data-formats/data_formats.md](./later/src/categories/rendering_data-formats/data_formats.md#L13))
- [ ] [data_formats: locate crates, write](https://github.com/john-cd/rust_howto/issues/453) ([later/src/categories/rendering_data-formats/data_formats.md](./later/src/categories/rendering_data-formats/data_formats.md#L19))

### later/src/categories/rendering_data-formats/index.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/954) ([later/src/categories/rendering_data-formats/index.md](./later/src/categories/rendering_data-formats/index.md#L48))

### later/src/categories/rendering_engine/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/955) ([later/src/categories/rendering_engine/index.md](./later/src/categories/rendering_engine/index.md#L34))

### later/src/categories/rendering_engine/rendering_engines.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/rendering_engine/rendering_engines.md](./later/src/categories/rendering_engine/rendering_engines.md#L13))
- [ ] [rendering_engines: write](https://github.com/john-cd/rust_howto/issues/455) ([later/src/categories/rendering_engine/rendering_engines.md](./later/src/categories/rendering_engine/rendering_engines.md#L19))

### later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md

- [ ] [webgpu: write](https://github.com/john-cd/rust_howto/issues/375) ([later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md](./later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md#L61))

### later/src/categories/rendering_graphics-api/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/956) ([later/src/categories/rendering_graphics-api/index.md](./later/src/categories/rendering_graphics-api/index.md#L62))

### later/src/categories/rendering_graphics-api/native_graphics_apis.md

- [ ] [graphics_apis: write](https://github.com/john-cd/rust_howto/issues/457) ([later/src/categories/rendering_graphics-api/native_graphics_apis.md](./later/src/categories/rendering_graphics-api/native_graphics_apis.md#L47))

### later/src/categories/rendering_graphics-api/opengl.md

- [ ] Resolve TODO/FIXME at line 31 ([later/src/categories/rendering_graphics-api/opengl.md](./later/src/categories/rendering_graphics-api/opengl.md#L31))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1229) ([later/src/categories/rendering_graphics-api/opengl.md](./later/src/categories/rendering_graphics-api/opengl.md#L37))

### later/src/categories/rendering_graphics-api/shaders.md

- [ ] Resolve TODO/FIXME at line 39 ([later/src/categories/rendering_graphics-api/shaders.md](./later/src/categories/rendering_graphics-api/shaders.md#L39))
- [ ] [decide what to cover, write](https://github.com/john-cd/rust_howto/issues/1228) ([later/src/categories/rendering_graphics-api/shaders.md](./later/src/categories/rendering_graphics-api/shaders.md#L77))

### later/src/categories/rendering_graphics-api/vulkan.md

- [ ] Resolve TODO/FIXME at line 21 ([later/src/categories/rendering_graphics-api/vulkan.md](./later/src/categories/rendering_graphics-api/vulkan.md#L21))
- [ ] [decide what to cover / write](https://github.com/john-cd/rust_howto/issues/1227) ([later/src/categories/rendering_graphics-api/vulkan.md](./later/src/categories/rendering_graphics-api/vulkan.md#L27))

### later/src/categories/science/classical_machine_learning.md

- [ ] Resolve TODO/FIXME at line 38 ([later/src/categories/science/classical_machine_learning.md](./later/src/categories/science/classical_machine_learning.md#L38))
- [ ] [organize / write](https://github.com/john-cd/rust_howto/issues/473) ([later/src/categories/science/classical_machine_learning.md](./later/src/categories/science/classical_machine_learning.md#L44))

### later/src/categories/science/deep_learning.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1205) ([later/src/categories/science/deep_learning.md](./later/src/categories/science/deep_learning.md#L38))

### later/src/categories/science/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/472) ([later/src/categories/science/index.md](./later/src/categories/science/index.md#L56))

### later/src/categories/science_geo/geo.md

- [ ] Resolve TODO/FIXME at line 17 ([later/src/categories/science_geo/geo.md](./later/src/categories/science_geo/geo.md#L17))
- [ ] [geo: write](https://github.com/john-cd/rust_howto/issues/474) ([later/src/categories/science_geo/geo.md](./later/src/categories/science_geo/geo.md#L23))

### later/src/categories/science_geo/index.md

- [ ] Resolve TODO/FIXME at line 33 ([later/src/categories/science_geo/index.md](./later/src/categories/science_geo/index.md#L33))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/958) ([later/src/categories/science_geo/index.md](./later/src/categories/science_geo/index.md#L39))

### later/src/categories/science_neuroscience/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/959) ([later/src/categories/science_neuroscience/index.md](./later/src/categories/science_neuroscience/index.md#L41))
- [ ] [also cover Biology! see lib.rs categorization](https://github.com/john-cd/rust_howto/issues/1197) ([later/src/categories/science_neuroscience/index.md](./later/src/categories/science_neuroscience/index.md#L42))

### later/src/categories/science_neuroscience/neuroscience.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/science_neuroscience/neuroscience.md](./later/src/categories/science_neuroscience/neuroscience.md#L13))
- [ ] [neuro: write](https://github.com/john-cd/rust_howto/issues/476) ([later/src/categories/science_neuroscience/neuroscience.md](./later/src/categories/science_neuroscience/neuroscience.md#L19))

### later/src/categories/science_robotics/artificial_intelligence.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1209) ([later/src/categories/science_robotics/artificial_intelligence.md](./later/src/categories/science_robotics/artificial_intelligence.md#L32))
- [ ] Dataflow Oriented Robotic Architecture ([later/src/categories/science_robotics/artificial_intelligence.md](./later/src/categories/science_robotics/artificial_intelligence.md#L43))

### later/src/categories/science_robotics/control_systems.md

- [ ] Resolve TODO/FIXME at line 56 ([later/src/categories/science_robotics/control_systems.md](./later/src/categories/science_robotics/control_systems.md#L56))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1208) ([later/src/categories/science_robotics/control_systems.md](./later/src/categories/science_robotics/control_systems.md#L62))

### later/src/categories/science_robotics/hardware_integration.md

- [ ] Resolve TODO/FIXME at line 20 ([later/src/categories/science_robotics/hardware_integration.md](./later/src/categories/science_robotics/hardware_integration.md#L20))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1211) ([later/src/categories/science_robotics/hardware_integration.md](./later/src/categories/science_robotics/hardware_integration.md#L26))

### later/src/categories/science_robotics/index.md

- [ ] [organize; review in depth](https://github.com/john-cd/rust_howto/issues/480) ([later/src/categories/science_robotics/index.md](./later/src/categories/science_robotics/index.md#L87))

### later/src/categories/science_robotics/perception_and_sensors.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/1207) ([later/src/categories/science_robotics/perception_and_sensors.md](./later/src/categories/science_robotics/perception_and_sensors.md#L31))

### later/src/categories/science_robotics/robot_operating_systems.md

- [ ] Resolve TODO/FIXME at line 73 ([later/src/categories/science_robotics/robot_operating_systems.md](./later/src/categories/science_robotics/robot_operating_systems.md#L73))
- [ ] [robotics: organize](https://github.com/john-cd/rust_howto/issues/477) ([later/src/categories/science_robotics/robot_operating_systems.md](./later/src/categories/science_robotics/robot_operating_systems.md#L79))

### later/src/categories/science_robotics/robotics_frameworks.md

- [ ] Resolve TODO/FIXME at line 58 ([later/src/categories/science_robotics/robotics_frameworks.md](./later/src/categories/science_robotics/robotics_frameworks.md#L58))
- [ ] [useful_robotics_tools_and_libs: locate libs, organize, write](https://github.com/john-cd/rust_howto/issues/479) ([later/src/categories/science_robotics/robotics_frameworks.md](./later/src/categories/science_robotics/robotics_frameworks.md#L64))

### later/src/categories/science_robotics/simulation_visualization.md

- [ ] Resolve TODO/FIXME at line 34 ([later/src/categories/science_robotics/simulation_visualization.md](./later/src/categories/science_robotics/simulation_visualization.md#L34))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1210) ([later/src/categories/science_robotics/simulation_visualization.md](./later/src/categories/science_robotics/simulation_visualization.md#L40))

### later/src/categories/simulation/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/961) ([later/src/categories/simulation/index.md](./later/src/categories/simulation/index.md#L45))

### later/src/categories/simulation/physics_engines.md

- [ ] Resolve TODO/FIXME at line 15 ([later/src/categories/simulation/physics_engines.md](./later/src/categories/simulation/physics_engines.md#L15))
- [ ] [write / decide what to cover](https://github.com/john-cd/rust_howto/issues/1206) ([later/src/categories/simulation/physics_engines.md](./later/src/categories/simulation/physics_engines.md#L21))

### later/src/categories/simulation/simulation.md

- [ ] Resolve TODO/FIXME at line 15 ([later/src/categories/simulation/simulation.md](./later/src/categories/simulation/simulation.md#L15))
- [ ] [simulation: write](https://github.com/john-cd/rust_howto/issues/481) ([later/src/categories/simulation/simulation.md](./later/src/categories/simulation/simulation.md#L21))

### later/src/categories/value-formatting/index.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/967) ([later/src/categories/value-formatting/index.md](./later/src/categories/value-formatting/index.md#L37))

### later/src/categories/value-formatting/number_formatting.md

- [ ] [value-formatting: choose crates, write](https://github.com/john-cd/rust_howto/issues/490) ([later/src/categories/value-formatting/number_formatting.md](./later/src/categories/value-formatting/number_formatting.md#L33))

### later/src/categories/virtualization/containerization.md

- [ ] Resolve TODO/FIXME at line 37 ([later/src/categories/virtualization/containerization.md](./later/src/categories/virtualization/containerization.md#L37))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1225) ([later/src/categories/virtualization/containerization.md](./later/src/categories/virtualization/containerization.md#L43))

### later/src/categories/virtualization/containers.md

- [ ] Resolve TODO/FIXME at line 33 ([later/src/categories/virtualization/containers.md](./later/src/categories/virtualization/containers.md#L33))
- [ ] [review/write](https://github.com/john-cd/rust_howto/issues/988) ([later/src/categories/virtualization/containers.md](./later/src/categories/virtualization/containers.md#L39))

### later/src/categories/virtualization/index.md

- [ ] [write; need in-depth review](https://github.com/john-cd/rust_howto/issues/968) ([later/src/categories/virtualization/index.md](./later/src/categories/virtualization/index.md#L90))

### later/src/categories/virtualization/using_containers.md

- [ ] Resolve TODO/FIXME at line 16 ([later/src/categories/virtualization/using_containers.md](./later/src/categories/virtualization/using_containers.md#L16))
- [ ] [write / fix](https://github.com/john-cd/rust_howto/issues/1224) ([later/src/categories/virtualization/using_containers.md](./later/src/categories/virtualization/using_containers.md#L22))

### later/src/categories/virtualization/virtualization.md

- [ ] Resolve TODO/FIXME at line 13 ([later/src/categories/virtualization/virtualization.md](./later/src/categories/virtualization/virtualization.md#L13))
- [ ] [virtualization: write](https://github.com/john-cd/rust_howto/issues/492) link firecracker; move containers section here ([later/src/categories/virtualization/virtualization.md](./later/src/categories/virtualization/virtualization.md#L19))

### later/src/categories/visualization/index.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/969) ([later/src/categories/visualization/index.md](./later/src/categories/visualization/index.md#L49))

### later/src/categories/visualization/visualization.md

- [ ] Resolve TODO/FIXME at line 28 ([later/src/categories/visualization/visualization.md](./later/src/categories/visualization/visualization.md#L28))
- [ ] [visualization: write](https://github.com/john-cd/rust_howto/issues/494) ([later/src/categories/visualization/visualization.md](./later/src/categories/visualization/visualization.md#L34))

### later/src/categories/wasm/index.md

- [ ] Resolve TODO/FIXME at line 44 ([later/src/categories/wasm/index.md](./later/src/categories/wasm/index.md#L44))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/970) ([later/src/categories/wasm/index.md](./later/src/categories/wasm/index.md#L50))

### later/src/categories/wasm/interfacing_with_javascript.md

- [ ] Resolve TODO/FIXME at line 40 ([later/src/categories/wasm/interfacing_with_javascript.md](./later/src/categories/wasm/interfacing_with_javascript.md#L40))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1218) ([later/src/categories/wasm/interfacing_with_javascript.md](./later/src/categories/wasm/interfacing_with_javascript.md#L46))

### later/src/categories/wasm/leptos.md

- [ ] Resolve TODO/FIXME at line 21 ([later/src/categories/wasm/leptos.md](./later/src/categories/wasm/leptos.md#L21))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1220) ([later/src/categories/wasm/leptos.md](./later/src/categories/wasm/leptos.md#L27))

### later/src/categories/wasm/wasm_basics.md

- [ ] Resolve TODO/FIXME at line 58 ([later/src/categories/wasm/wasm_basics.md](./later/src/categories/wasm/wasm_basics.md#L58))
- [ ] [final review](https://github.com/john-cd/rust_howto/issues/1219) ([later/src/categories/wasm/wasm_basics.md](./later/src/categories/wasm/wasm_basics.md#L64))

### later/src/categories/wasm/wasm_development.md

- [ ] Resolve TODO/FIXME at line 80 ([later/src/categories/wasm/wasm_development.md](./later/src/categories/wasm/wasm_development.md#L80))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/1217) ([later/src/categories/wasm/wasm_development.md](./later/src/categories/wasm/wasm_development.md#L86))

### later/src/categories/wasm/wasm_standalone_runtimes.md

- [ ] Resolve TODO/FIXME at line 107 ([later/src/categories/wasm/wasm_standalone_runtimes.md](./later/src/categories/wasm/wasm_standalone_runtimes.md#L107))
- [ ] [others: organize/write](https://github.com/john-cd/rust_howto/issues/496) need full review ([later/src/categories/wasm/wasm_standalone_runtimes.md](./later/src/categories/wasm/wasm_standalone_runtimes.md#L113))

### later/src/categories/wasm/yew.md

- [ ] Resolve TODO/FIXME at line 28 ([later/src/categories/wasm/yew.md](./later/src/categories/wasm/yew.md#L28))
- [ ] [yew: write / organize](https://github.com/john-cd/rust_howto/issues/498) ([later/src/categories/wasm/yew.md](./later/src/categories/wasm/yew.md#L34))

### later/src/categories/web-programming_websocket/index.md

- [ ] Resolve TODO/FIXME at line 37 ([later/src/categories/web-programming_websocket/index.md](./later/src/categories/web-programming_websocket/index.md#L37))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/979) ([later/src/categories/web-programming_websocket/index.md](./later/src/categories/web-programming_websocket/index.md#L43))

### later/src/categories/web-programming_websocket/websocket.md

- [ ] [websocket: write](https://github.com/john-cd/rust_howto/issues/520) ([later/src/categories/web-programming_websocket/websocket.md](./later/src/categories/web-programming_websocket/websocket.md#L56))

### later/src/other/architecture/architectural_patterns.md

- [ ] [write. create missing examples. cover [`inventory`][c~inventory~docs]↗{{hi:inventory}} crate for DI](https://github.com/john-cd/rust_howto/issues/1231)? ([later/src/other/architecture/architectural_patterns.md](./later/src/other/architecture/architectural_patterns.md#L49))

### later/src/other/architecture/common_architectures.md

- [ ] [write / detail every architecture](https://github.com/john-cd/rust_howto/issues/1230) ([later/src/other/architecture/common_architectures.md](./later/src/other/architecture/common_architectures.md#L135))

### later/src/other/architecture/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/573) ([later/src/other/architecture/index.md](./later/src/other/architecture/index.md#L29))

### later/src/other/architecture/software_architecture_process.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/572) ([later/src/other/architecture/software_architecture_process.md](./later/src/other/architecture/software_architecture_process.md#L103))

### later/src/other/cloud/_fragment.md

- [ ] Resolve TODO/FIXME at line 12 ([later/src/other/cloud/_fragment.md](./later/src/other/cloud/_fragment.md#L12))
- [ ] [decide where that goes](https://github.com/john-cd/rust_howto/issues/578) ([later/src/other/cloud/_fragment.md](./later/src/other/cloud/_fragment.md#L18))

### later/src/other/cloud/aws.md

- [ ] [aws: write](https://github.com/john-cd/rust_howto/issues/574) ([later/src/other/cloud/aws.md](./later/src/other/cloud/aws.md#L60))

### later/src/other/cloud/index.md

- [ ] [edit](https://github.com/john-cd/rust_howto/issues/579) ([later/src/other/cloud/index.md](./later/src/other/cloud/index.md#L39))

### later/src/other/cloud/rust_native_cloud_development.md

- [ ] Resolve TODO/FIXME at line 32 ([later/src/other/cloud/rust_native_cloud_development.md](./later/src/other/cloud/rust_native_cloud_development.md#L32))
- [ ] [write](https://github.com/john-cd/rust_howto/issues/576) ([later/src/other/cloud/rust_native_cloud_development.md](./later/src/other/cloud/rust_native_cloud_development.md#L41))

### later/src/other/cross-platform/crux.md

- [ ] Resolve TODO/FIXME at line 23 ([later/src/other/cross-platform/crux.md](./later/src/other/cross-platform/crux.md#L23))
- [ ] [crux: add / edit; link to architecture pages](https://github.com/john-cd/rust_howto/issues/582) ([later/src/other/cross-platform/crux.md](./later/src/other/cross-platform/crux.md#L29))

### later/src/other/cross-platform/index.md

- [ ] Resolve TODO/FIXME at line 38 ([later/src/other/cross-platform/index.md](./later/src/other/cross-platform/index.md#L38))
- [ ] Resolve TODO/FIXME at line 48 ([later/src/other/cross-platform/index.md](./later/src/other/cross-platform/index.md#L48))
- [ ] Resolve TODO/FIXME at line 54 ([later/src/other/cross-platform/index.md](./later/src/other/cross-platform/index.md#L54))
- [ ] Resolve TODO/FIXME at line 70 ([later/src/other/cross-platform/index.md](./later/src/other/cross-platform/index.md#L70))
- [ ] [add / write; link to other chapters; conditional compile](https://github.com/john-cd/rust_howto/issues/583) ([later/src/other/cross-platform/index.md](./later/src/other/cross-platform/index.md#L76))

### later/src/other/data-processing/csv.md

- [ ] Resolve TODO/FIXME at line 18 ([later/src/other/data-processing/csv.md](./later/src/other/data-processing/csv.md#L18))
- [ ] [csv: organize / edit](https://github.com/john-cd/rust_howto/issues/585) ([later/src/other/data-processing/csv.md](./later/src/other/data-processing/csv.md#L24))

### later/src/other/data-processing/data_engineering.md

- [ ] Resolve TODO/FIXME at line 59 ([later/src/other/data-processing/data_engineering.md](./later/src/other/data-processing/data_engineering.md#L59))
- [ ] [data_engineering: organize / edit](https://github.com/john-cd/rust_howto/issues/589) ([later/src/other/data-processing/data_engineering.md](./later/src/other/data-processing/data_engineering.md#L65))

### later/src/other/data-processing/dataframes.md

- [ ] Resolve TODO/FIXME at line 19 ([later/src/other/data-processing/dataframes.md](./later/src/other/data-processing/dataframes.md#L19))
- [ ] [dataframes: organize / edit](https://github.com/john-cd/rust_howto/issues/587) ([later/src/other/data-processing/dataframes.md](./later/src/other/data-processing/dataframes.md#L25))

### later/src/other/data-processing/index.md

- [ ] Resolve TODO/FIXME at line 17 ([later/src/other/data-processing/index.md](./later/src/other/data-processing/index.md#L17))
- [ ] [organize / edit](https://github.com/john-cd/rust_howto/issues/594) ([later/src/other/data-processing/index.md](./later/src/other/data-processing/index.md#L23))

### later/src/other/devops/cd_ci.md

- [ ] [cd_ci: write; organize links](https://github.com/john-cd/rust_howto/issues/595) ([later/src/other/devops/cd_ci.md](./later/src/other/devops/cd_ci.md#L47))

### later/src/other/devops/github_actions.md

- [ ] Resolve TODO/FIXME at line 119 ([later/src/other/devops/github_actions.md](./later/src/other/devops/github_actions.md#L119))
- [ ] [github_actions - see blessed.rs](https://github.com/john-cd/rust_howto/issues/600) ([later/src/other/devops/github_actions.md](./later/src/other/devops/github_actions.md#L135))

### later/src/other/devops/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/605) ([later/src/other/devops/index.md](./later/src/other/devops/index.md#L63))

### later/src/other/devops/release_automation.md

- [ ] got archived?? ([later/src/other/devops/release_automation.md](./later/src/other/devops/release_automation.md#L74))
- [ ] [release_automation: write](https://github.com/john-cd/rust_howto/issues/604) ([later/src/other/devops/release_automation.md](./later/src/other/devops/release_automation.md#L94))

### later/src/other/devops/version_control.md

- [ ] The following describes tools that make ([later/src/other/devops/version_control.md](./later/src/other/devops/version_control.md#L7))
- [ ] Resolve TODO/FIXME at line 70 ([later/src/other/devops/version_control.md](./later/src/other/devops/version_control.md#L70))
- [ ] [git_hooks: write](https://github.com/john-cd/rust_howto/issues/602) ([later/src/other/devops/version_control.md](./later/src/other/devops/version_control.md#L94))

### later/src/other/gpu/gpu.md

- [ ] [gpu: write](https://github.com/john-cd/rust_howto/issues/607) ([later/src/other/gpu/gpu.md](./later/src/other/gpu/gpu.md#L23))

### later/src/other/gpu/index.md

- [ ] [write](https://github.com/john-cd/rust_howto/issues/608) ([later/src/other/gpu/index.md](./later/src/other/gpu/index.md#L29))

### later/src/other/scripting/index.md

- [ ] [review](https://github.com/john-cd/rust_howto/issues/991) ([later/src/other/scripting/index.md](./later/src/other/scripting/index.md#L86))

### later/src/other/scripting/rhai.md

- [ ] Resolve TODO/FIXME at line 40 ([later/src/other/scripting/rhai.md](./later/src/other/scripting/rhai.md#L40))
- [ ] [rhai: write](https://github.com/john-cd/rust_howto/issues/610) ([later/src/other/scripting/rhai.md](./later/src/other/scripting/rhai.md#L46))

### later/src/other/scripting/task_automation.md

- [ ] [write; where does this chapter belong](https://github.com/john-cd/rust_howto/issues/1232)? ([later/src/other/scripting/task_automation.md](./later/src/other/scripting/task_automation.md#L37))

### later/src/other/written-in-rust/development_tools.md

- [ ] [organize](https://github.com/john-cd/rust_howto/issues/612) ([later/src/other/written-in-rust/development_tools.md](./later/src/other/written-in-rust/development_tools.md#L57))

### later/src/other/written-in-rust/index.md

- [ ] Resolve TODO/FIXME at line 144 ([later/src/other/written-in-rust/index.md](./later/src/other/written-in-rust/index.md#L144))
- [ ] [organize; decide if we need other pages or if we consolidate here; table?](https://github.com/john-cd/rust_howto/issues/993) ([later/src/other/written-in-rust/index.md](./later/src/other/written-in-rust/index.md#L150))

### later/src/other/written-in-rust/other_tools.md

- [ ] Resolve TODO/FIXME at line 38 ([later/src/other/written-in-rust/other_tools.md](./later/src/other/written-in-rust/other_tools.md#L38))
- [ ] [others: review/add](https://github.com/john-cd/rust_howto/issues/615) ([later/src/other/written-in-rust/other_tools.md](./later/src/other/written-in-rust/other_tools.md#L44))

### later/src/other/written-in-rust/python_tools.md

- [ ] Resolve TODO/FIXME at line 56 ([later/src/other/written-in-rust/python_tools.md](./later/src/other/written-in-rust/python_tools.md#L56))
- [ ] [python_tools: write](https://github.com/john-cd/rust_howto/issues/617) ([later/src/other/written-in-rust/python_tools.md](./later/src/other/written-in-rust/python_tools.md#L59))

### GitHub issue references

#### bk/drafts/categories/asynchronous/async.md

- [ ] Review issue [#633](https://github.com/john-cd/rust_howto/issues/633) in [bk/drafts/categories/asynchronous/async.md](./bk/drafts/categories/asynchronous/async.md#L83)

#### bk/drafts/categories/asynchronous/async_channels.md

- [ ] Review issue [#215](https://github.com/john-cd/rust_howto/issues/215) in [bk/drafts/categories/asynchronous/async_channels.md](./bk/drafts/categories/asynchronous/async_channels.md#L89)

#### bk/drafts/categories/asynchronous/async_traits.md

- [ ] Review issue [#216](https://github.com/john-cd/rust_howto/issues/216) in [bk/drafts/categories/asynchronous/async_traits.md](./bk/drafts/categories/asynchronous/async_traits.md#L40)

#### bk/drafts/categories/asynchronous/async_utilities.md

- [ ] Review issue [#906](https://github.com/john-cd/rust_howto/issues/906) in [bk/drafts/categories/asynchronous/async_utilities.md](./bk/drafts/categories/asynchronous/async_utilities.md#L31)

#### bk/drafts/categories/asynchronous/futures.md

- [ ] Review issue [#1340](https://github.com/john-cd/rust_howto/issues/1340) in [bk/drafts/categories/asynchronous/futures.md](./bk/drafts/categories/asynchronous/futures.md#L61)

#### bk/drafts/categories/asynchronous/index.md

- [ ] Review issue [#905](https://github.com/john-cd/rust_howto/issues/905) in [bk/drafts/categories/asynchronous/index.md](./bk/drafts/categories/asynchronous/index.md#L66)

#### bk/drafts/categories/asynchronous/streams.md

- [ ] Review issue [#645](https://github.com/john-cd/rust_howto/issues/645) in [bk/drafts/categories/asynchronous/streams.md](./bk/drafts/categories/asynchronous/streams.md#L41)

#### bk/drafts/categories/asynchronous/tokio.md

- [ ] Review issue [#223](https://github.com/john-cd/rust_howto/issues/223) in [bk/drafts/categories/asynchronous/tokio.md](./bk/drafts/categories/asynchronous/tokio.md#L72)

#### bk/drafts/categories/authentication/basic_authentication.md

- [ ] Review issue [#224](https://github.com/john-cd/rust_howto/issues/224) in [bk/drafts/categories/authentication/basic_authentication.md](./bk/drafts/categories/authentication/basic_authentication.md#L26)

#### bk/drafts/categories/authentication/index.md

- [ ] Review issue [#636](https://github.com/john-cd/rust_howto/issues/636) in [bk/drafts/categories/authentication/index.md](./bk/drafts/categories/authentication/index.md#L27)

#### bk/drafts/categories/caching/in_memory_cache.md

- [ ] Review issue [#227](https://github.com/john-cd/rust_howto/issues/227) in [bk/drafts/categories/caching/in_memory_cache.md](./bk/drafts/categories/caching/in_memory_cache.md#L48)

#### bk/drafts/categories/caching/index.md

- [ ] Review issue [#1173](https://github.com/john-cd/rust_howto/issues/1173) in [bk/drafts/categories/caching/index.md](./bk/drafts/categories/caching/index.md#L53)

#### bk/drafts/categories/command-line-interface/ansi_terminal.md

- [ ] Review issue [#231](https://github.com/john-cd/rust_howto/issues/231) in [bk/drafts/categories/command-line-interface/ansi_terminal.md](./bk/drafts/categories/command-line-interface/ansi_terminal.md#L138)

#### bk/drafts/categories/command-line-interface/argument_parsing.md

- [ ] Review issue [#233](https://github.com/john-cd/rust_howto/issues/233) in [bk/drafts/categories/command-line-interface/argument_parsing.md](./bk/drafts/categories/command-line-interface/argument_parsing.md#L149)

#### bk/drafts/categories/command-line-interface/index.md

- [ ] Review issue [#907](https://github.com/john-cd/rust_howto/issues/907) in [bk/drafts/categories/command-line-interface/index.md](./bk/drafts/categories/command-line-interface/index.md#L48)

#### bk/drafts/categories/command-line-interface/tui.md

- [ ] Review issue [#234](https://github.com/john-cd/rust_howto/issues/234) in [bk/drafts/categories/command-line-interface/tui.md](./bk/drafts/categories/command-line-interface/tui.md#L45)

#### bk/drafts/categories/command-line-interface/user_interaction.md

- [ ] Review issue [#235](https://github.com/john-cd/rust_howto/issues/235) in [bk/drafts/categories/command-line-interface/user_interaction.md](./bk/drafts/categories/command-line-interface/user_interaction.md#L49)

#### bk/drafts/categories/command-line-utilities/filesystem_cli.md

- [ ] Review issue [#237](https://github.com/john-cd/rust_howto/issues/237) in [bk/drafts/categories/command-line-utilities/filesystem_cli.md](./bk/drafts/categories/command-line-utilities/filesystem_cli.md#L60)

#### bk/drafts/categories/command-line-utilities/index.md

- [ ] Review issue [#1189](https://github.com/john-cd/rust_howto/issues/1189) in [bk/drafts/categories/command-line-utilities/index.md](./bk/drafts/categories/command-line-utilities/index.md#L48)

#### bk/drafts/categories/command-line-utilities/networking_cli.md

- [ ] Review issue [#238](https://github.com/john-cd/rust_howto/issues/238) in [bk/drafts/categories/command-line-utilities/networking_cli.md](./bk/drafts/categories/command-line-utilities/networking_cli.md#L25)

#### bk/drafts/categories/command-line-utilities/shells.md

- [ ] Review issue [#239](https://github.com/john-cd/rust_howto/issues/239) in [bk/drafts/categories/command-line-utilities/shells.md](./bk/drafts/categories/command-line-utilities/shells.md#L32)

#### bk/drafts/categories/compression/compression.md

- [ ] Review issue [#1062](https://github.com/john-cd/rust_howto/issues/1062) in [bk/drafts/categories/compression/compression.md](./bk/drafts/categories/compression/compression.md#L48)

#### bk/drafts/categories/compression/index.md

- [ ] Review issue [#1184](https://github.com/john-cd/rust_howto/issues/1184) in [bk/drafts/categories/compression/index.md](./bk/drafts/categories/compression/index.md#L27)

#### bk/drafts/categories/compression/tar.md

- [ ] Review issue [#253](https://github.com/john-cd/rust_howto/issues/253) in [bk/drafts/categories/compression/tar.md](./bk/drafts/categories/compression/tar.md#L47)

#### bk/drafts/categories/concurrency/_actors.md

- [ ] Review issue [#269](https://github.com/john-cd/rust_howto/issues/269) in [bk/drafts/categories/concurrency/_actors.md](./bk/drafts/categories/concurrency/_actors.md#L98)

#### bk/drafts/categories/concurrency/atomics.md

- [ ] Review issue [#1342](https://github.com/john-cd/rust_howto/issues/1342) in [bk/drafts/categories/concurrency/atomics.md](./bk/drafts/categories/concurrency/atomics.md#L56)

#### bk/drafts/categories/concurrency/concurrent_data_structures.md

- [ ] Review issue [#258](https://github.com/john-cd/rust_howto/issues/258) in [bk/drafts/categories/concurrency/concurrent_data_structures.md](./bk/drafts/categories/concurrency/concurrent_data_structures.md#L73)

#### bk/drafts/categories/concurrency/crossbeam.md

- [ ] Review issue [#259](https://github.com/john-cd/rust_howto/issues/259) in [bk/drafts/categories/concurrency/crossbeam.md](./bk/drafts/categories/concurrency/crossbeam.md#L53)

#### bk/drafts/categories/concurrency/data_parallelism.md

- [ ] Review issue [#260](https://github.com/john-cd/rust_howto/issues/260) in [bk/drafts/categories/concurrency/data_parallelism.md](./bk/drafts/categories/concurrency/data_parallelism.md#L122)

#### bk/drafts/categories/concurrency/explicit_threads.md

- [ ] Review issue [#262](https://github.com/john-cd/rust_howto/issues/262) in [bk/drafts/categories/concurrency/explicit_threads.md](./bk/drafts/categories/concurrency/explicit_threads.md#L33)

#### bk/drafts/categories/concurrency/index.md

- [ ] Review issue [#263](https://github.com/john-cd/rust_howto/issues/263) in [bk/drafts/categories/concurrency/index.md](./bk/drafts/categories/concurrency/index.md#L83)

#### bk/drafts/categories/concurrency/message_passing.md

- [ ] Review issue [#264](https://github.com/john-cd/rust_howto/issues/264) in [bk/drafts/categories/concurrency/message_passing.md](./bk/drafts/categories/concurrency/message_passing.md#L58)

#### bk/drafts/categories/concurrency/send_sync.md

- [ ] Review issue [#909](https://github.com/john-cd/rust_howto/issues/909) in [bk/drafts/categories/concurrency/send_sync.md](./bk/drafts/categories/concurrency/send_sync.md#L63)

#### bk/drafts/categories/concurrency/shared_state.md

- [ ] Review issue [#266](https://github.com/john-cd/rust_howto/issues/266) in [bk/drafts/categories/concurrency/shared_state.md](./bk/drafts/categories/concurrency/shared_state.md#L82)

#### bk/drafts/categories/concurrency/threadpool.md

- [ ] Review issue [#267](https://github.com/john-cd/rust_howto/issues/267) in [bk/drafts/categories/concurrency/threadpool.md](./bk/drafts/categories/concurrency/threadpool.md#L44)

#### bk/drafts/categories/config/configuration.md

- [ ] Review issue [#270](https://github.com/john-cd/rust_howto/issues/270) in [bk/drafts/categories/config/configuration.md](./bk/drafts/categories/config/configuration.md#L54)

#### bk/drafts/categories/config/environment_variables.md

- [ ] Review issue [#271](https://github.com/john-cd/rust_howto/issues/271) in [bk/drafts/categories/config/environment_variables.md](./bk/drafts/categories/config/environment_variables.md#L57)

#### bk/drafts/categories/config/index.md

- [ ] Review issue [#1195](https://github.com/john-cd/rust_howto/issues/1195) in [bk/drafts/categories/config/index.md](./bk/drafts/categories/config/index.md#L64)

#### bk/drafts/categories/cryptography/aead.md

- [ ] Review issue [#1183](https://github.com/john-cd/rust_howto/issues/1183) in [bk/drafts/categories/cryptography/aead.md](./bk/drafts/categories/cryptography/aead.md#L57)

#### bk/drafts/categories/cryptography/certificates.md

- [ ] Review issue [#1179](https://github.com/john-cd/rust_howto/issues/1179) in [bk/drafts/categories/cryptography/certificates.md](./bk/drafts/categories/cryptography/certificates.md#L64)

#### bk/drafts/categories/cryptography/cryptography_utilities.md

- [ ] Review issue [#1180](https://github.com/john-cd/rust_howto/issues/1180) in [bk/drafts/categories/cryptography/cryptography_utilities.md](./bk/drafts/categories/cryptography/cryptography_utilities.md#L39)

#### bk/drafts/categories/cryptography/encryption.md

- [ ] Review issue [#272](https://github.com/john-cd/rust_howto/issues/272) in [bk/drafts/categories/cryptography/encryption.md](./bk/drafts/categories/cryptography/encryption.md#L33)

#### bk/drafts/categories/cryptography/hmac.md

- [ ] Review issue [#1181](https://github.com/john-cd/rust_howto/issues/1181) in [bk/drafts/categories/cryptography/hmac.md](./bk/drafts/categories/cryptography/hmac.md#L27)

#### bk/drafts/categories/cryptography/index.md

- [ ] Review issue [#274](https://github.com/john-cd/rust_howto/issues/274) in [bk/drafts/categories/cryptography/index.md](./bk/drafts/categories/cryptography/index.md#L95)

#### bk/drafts/categories/cryptography/password_hashing.md

- [ ] Review issue [#275](https://github.com/john-cd/rust_howto/issues/275) in [bk/drafts/categories/cryptography/password_hashing.md](./bk/drafts/categories/cryptography/password_hashing.md#L87)

#### bk/drafts/categories/cryptography/signature.md

- [ ] Review issue [#1178](https://github.com/john-cd/rust_howto/issues/1178) in [bk/drafts/categories/cryptography/signature.md](./bk/drafts/categories/cryptography/signature.md#L80)

#### bk/drafts/categories/cryptography/tls.md

- [ ] Review issue [#1182](https://github.com/john-cd/rust_howto/issues/1182) in [bk/drafts/categories/cryptography/tls.md](./bk/drafts/categories/cryptography/tls.md#L41)

#### bk/drafts/categories/data-structures/index.md

- [ ] Review issue [#280](https://github.com/john-cd/rust_howto/issues/280) in [bk/drafts/categories/data-structures/index.md](./bk/drafts/categories/data-structures/index.md#L189)

#### bk/drafts/categories/database-implementations/databases.md

- [ ] Review issue [#290](https://github.com/john-cd/rust_howto/issues/290) in [bk/drafts/categories/database-implementations/databases.md](./bk/drafts/categories/database-implementations/databases.md#L57)

#### bk/drafts/categories/database-implementations/index.md

- [ ] Review issue [#292](https://github.com/john-cd/rust_howto/issues/292) in [bk/drafts/categories/database-implementations/index.md](./bk/drafts/categories/database-implementations/index.md#L25)

#### bk/drafts/categories/database-implementations/rust_search_engines.md

- [ ] Review issue [#291](https://github.com/john-cd/rust_howto/issues/291) in [bk/drafts/categories/database-implementations/rust_search_engines.md](./bk/drafts/categories/database-implementations/rust_search_engines.md#L47)

#### bk/drafts/categories/database/amqp.md

- [ ] Review issue [#1064](https://github.com/john-cd/rust_howto/issues/1064) in [bk/drafts/categories/database/amqp.md](./bk/drafts/categories/database/amqp.md#L29)

#### bk/drafts/categories/database/connection_pool.md

- [ ] Review issue [#284](https://github.com/john-cd/rust_howto/issues/284) in [bk/drafts/categories/database/connection_pool.md](./bk/drafts/categories/database/connection_pool.md#L29)

#### bk/drafts/categories/database/index.md

- [ ] Review issue [#1065](https://github.com/john-cd/rust_howto/issues/1065) in [bk/drafts/categories/database/index.md](./bk/drafts/categories/database/index.md#L65)

#### bk/drafts/categories/database/mssql.md

- [ ] Review issue [#1067](https://github.com/john-cd/rust_howto/issues/1067) in [bk/drafts/categories/database/mssql.md](./bk/drafts/categories/database/mssql.md#L23)

#### bk/drafts/categories/database/nosql.md

- [ ] Review issue [#1068](https://github.com/john-cd/rust_howto/issues/1068) in [bk/drafts/categories/database/nosql.md](./bk/drafts/categories/database/nosql.md#L62)

#### bk/drafts/categories/database/oracle.md

- [ ] Review issue [#1069](https://github.com/john-cd/rust_howto/issues/1069) in [bk/drafts/categories/database/oracle.md](./bk/drafts/categories/database/oracle.md#L43)

#### bk/drafts/categories/database/postgres.md

- [ ] Review issue [#286](https://github.com/john-cd/rust_howto/issues/286) in [bk/drafts/categories/database/postgres.md](./bk/drafts/categories/database/postgres.md#L76)

#### bk/drafts/categories/database/query_builders_orms.md

- [ ] Review issue [#912](https://github.com/john-cd/rust_howto/issues/912) in [bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L167)

#### bk/drafts/categories/database/search.md

- [ ] Review issue [#288](https://github.com/john-cd/rust_howto/issues/288) in [bk/drafts/categories/database/search.md](./bk/drafts/categories/database/search.md#L37)

#### bk/drafts/categories/date-and-time/duration.md

- [ ] Review issue [#913](https://github.com/john-cd/rust_howto/issues/913) in [bk/drafts/categories/date-and-time/duration.md](./bk/drafts/categories/date-and-time/duration.md#L49)

#### bk/drafts/categories/date-and-time/index.md

- [ ] Review issue [#1188](https://github.com/john-cd/rust_howto/issues/1188) in [bk/drafts/categories/date-and-time/index.md](./bk/drafts/categories/date-and-time/index.md#L38)

#### bk/drafts/categories/date-and-time/parse.md

- [ ] Review issue [#914](https://github.com/john-cd/rust_howto/issues/914) in [bk/drafts/categories/date-and-time/parse.md](./bk/drafts/categories/date-and-time/parse.md#L73)

#### bk/drafts/categories/date-and-time/time_crate.md

- [ ] Review issue [#293](https://github.com/john-cd/rust_howto/issues/293) in [bk/drafts/categories/date-and-time/time_crate.md](./bk/drafts/categories/date-and-time/time_crate.md#L27)

#### bk/drafts/categories/development-tools/cargo/cargo.md

- [ ] Review issue [#915](https://github.com/john-cd/rust_howto/issues/915) in [bk/drafts/categories/development-tools/cargo/cargo.md](./bk/drafts/categories/development-tools/cargo/cargo.md#L104)

#### bk/drafts/categories/development-tools/cargo/crate_registries.md

- [ ] Review issue [#294](https://github.com/john-cd/rust_howto/issues/294) in [bk/drafts/categories/development-tools/cargo/crate_registries.md](./bk/drafts/categories/development-tools/cargo/crate_registries.md#L33)

#### bk/drafts/categories/development-tools/cargo/package_layout.md

- [ ] Review issue [#916](https://github.com/john-cd/rust_howto/issues/916) in [bk/drafts/categories/development-tools/cargo/package_layout.md](./bk/drafts/categories/development-tools/cargo/package_layout.md#L51)

#### bk/drafts/categories/development-tools/compilation/faster_linking.md

- [ ] Review issue [#242](https://github.com/john-cd/rust_howto/issues/242) in [bk/drafts/categories/development-tools/compilation/faster_linking.md](./bk/drafts/categories/development-tools/compilation/faster_linking.md#L142)

#### bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md

- [ ] Review issue [#245](https://github.com/john-cd/rust_howto/issues/245) in [bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md](./bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md#L122)

#### bk/drafts/categories/development-tools/cross-compilation/cross_compilation.md

- [ ] Review issue [#240](https://github.com/john-cd/rust_howto/issues/240) in [bk/drafts/categories/development-tools/cross-compilation/cross_compilation.md](./bk/drafts/categories/development-tools/cross-compilation/cross_compilation.md#L55)

#### bk/drafts/categories/development-tools/documentation/badges.md

- [ ] Review issue [#296](https://github.com/john-cd/rust_howto/issues/296) in [bk/drafts/categories/development-tools/documentation/badges.md](./bk/drafts/categories/development-tools/documentation/badges.md#L18)

#### bk/drafts/categories/development-tools/documentation/documentation.md

- [ ] Review issue [#297](https://github.com/john-cd/rust_howto/issues/297) in [bk/drafts/categories/development-tools/documentation/documentation.md](./bk/drafts/categories/development-tools/documentation/documentation.md#L67)

#### bk/drafts/categories/development-tools/documentation/mdbook.md

- [ ] Review issue [#299](https://github.com/john-cd/rust_howto/issues/299) in [bk/drafts/categories/development-tools/documentation/mdbook.md](./bk/drafts/categories/development-tools/documentation/mdbook.md#L146)

#### bk/drafts/categories/development-tools/formatting/formatting.md

- [ ] Review issue [#300](https://github.com/john-cd/rust_howto/issues/300) in [bk/drafts/categories/development-tools/formatting/formatting.md](./bk/drafts/categories/development-tools/formatting/formatting.md#L96)

#### bk/drafts/categories/development-tools/index.md

- [ ] Review issue [#319](https://github.com/john-cd/rust_howto/issues/319) in [bk/drafts/categories/development-tools/index.md](./bk/drafts/categories/development-tools/index.md#L95)

#### bk/drafts/categories/development-tools/installation/install.md

- [ ] Review issue [#918](https://github.com/john-cd/rust_howto/issues/918) in [bk/drafts/categories/development-tools/installation/install.md](./bk/drafts/categories/development-tools/installation/install.md#L35)

#### bk/drafts/categories/development-tools/installation/rustup.md

- [ ] Review issue [#302](https://github.com/john-cd/rust_howto/issues/302) in [bk/drafts/categories/development-tools/installation/rustup.md](./bk/drafts/categories/development-tools/installation/rustup.md#L56)

#### bk/drafts/categories/development-tools/other/code_build.md

- [ ] Review issue [#919](https://github.com/john-cd/rust_howto/issues/919) in [bk/drafts/categories/development-tools/other/code_build.md](./bk/drafts/categories/development-tools/other/code_build.md#L114)

#### bk/drafts/categories/development-tools/other/code_verification.md

- [ ] Review issue [#303](https://github.com/john-cd/rust_howto/issues/303) in [bk/drafts/categories/development-tools/other/code_verification.md](./bk/drafts/categories/development-tools/other/code_verification.md#L110)

#### bk/drafts/categories/development-tools/other/miri.md

- [ ] Review issue [#304](https://github.com/john-cd/rust_howto/issues/304) in [bk/drafts/categories/development-tools/other/miri.md](./bk/drafts/categories/development-tools/other/miri.md#L31)

#### bk/drafts/categories/development-tools/other/other.md

- [ ] Review issue [#305](https://github.com/john-cd/rust_howto/issues/305) in [bk/drafts/categories/development-tools/other/other.md](./bk/drafts/categories/development-tools/other/other.md#L40)

#### bk/drafts/categories/development-tools/transcompilation/transpilers.md

- [ ] Review issue [#1196](https://github.com/john-cd/rust_howto/issues/1196) in [bk/drafts/categories/development-tools/transcompilation/transpilers.md](./bk/drafts/categories/development-tools/transcompilation/transpilers.md#L19)

#### bk/drafts/categories/development-tools/versioning/versioning.md

- [ ] Review issue [#920](https://github.com/john-cd/rust_howto/issues/920) in [bk/drafts/categories/development-tools/versioning/versioning.md](./bk/drafts/categories/development-tools/versioning/versioning.md#L74)

#### bk/drafts/categories/development-tools_build-utils/autocfg.md

- [ ] Review issue [#1166](https://github.com/john-cd/rust_howto/issues/1166) in [bk/drafts/categories/development-tools_build-utils/autocfg.md](./bk/drafts/categories/development-tools_build-utils/autocfg.md#L25)

#### bk/drafts/categories/development-tools_build-utils/build_cache.md

- [ ] Review issue [#1167](https://github.com/john-cd/rust_howto/issues/1167) in [bk/drafts/categories/development-tools_build-utils/build_cache.md](./bk/drafts/categories/development-tools_build-utils/build_cache.md#L19)

#### bk/drafts/categories/development-tools_build-utils/build_time_tooling.md

- [ ] Review issue [#921](https://github.com/john-cd/rust_howto/issues/921) in [bk/drafts/categories/development-tools_build-utils/build_time_tooling.md](./bk/drafts/categories/development-tools_build-utils/build_time_tooling.md#L160)

#### bk/drafts/categories/development-tools_build-utils/index.md

- [ ] Review issue [#306](https://github.com/john-cd/rust_howto/issues/306) in [bk/drafts/categories/development-tools_build-utils/index.md](./bk/drafts/categories/development-tools_build-utils/index.md#L34)

#### bk/drafts/categories/development-tools_cargo-plugins/auditing.md

- [ ] Review issue [#922](https://github.com/john-cd/rust_howto/issues/922) in [bk/drafts/categories/development-tools_cargo-plugins/auditing.md](./bk/drafts/categories/development-tools_cargo-plugins/auditing.md#L89)

#### bk/drafts/categories/development-tools_cargo-plugins/building.md

- [ ] Review issue [#309](https://github.com/john-cd/rust_howto/issues/309) in [bk/drafts/categories/development-tools_cargo-plugins/building.md](./bk/drafts/categories/development-tools_cargo-plugins/building.md#L92)

#### bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md

- [ ] Review issue [#310](https://github.com/john-cd/rust_howto/issues/310) in [bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md](./bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md#L94)

#### bk/drafts/categories/development-tools_cargo-plugins/code_writing.md

- [ ] Review issue [#924](https://github.com/john-cd/rust_howto/issues/924) in [bk/drafts/categories/development-tools_cargo-plugins/code_writing.md](./bk/drafts/categories/development-tools_cargo-plugins/code_writing.md#L43)

#### bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md

- [ ] Review issue [#923](https://github.com/john-cd/rust_howto/issues/923) in [bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md](./bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md#L29)

#### bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md

- [ ] Review issue [#597](https://github.com/john-cd/rust_howto/issues/597) in [bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md](./bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md#L87)

#### bk/drafts/categories/development-tools_cargo-plugins/index.md

- [ ] Review issue [#311](https://github.com/john-cd/rust_howto/issues/311) in [bk/drafts/categories/development-tools_cargo-plugins/index.md](./bk/drafts/categories/development-tools_cargo-plugins/index.md#L88)

#### bk/drafts/categories/development-tools_cargo-plugins/maintaining.md

- [ ] Review issue [#313](https://github.com/john-cd/rust_howto/issues/313) in [bk/drafts/categories/development-tools_cargo-plugins/maintaining.md](./bk/drafts/categories/development-tools_cargo-plugins/maintaining.md#L71)

#### bk/drafts/categories/development-tools_cargo-plugins/performance.md

- [ ] Review issue [#314](https://github.com/john-cd/rust_howto/issues/314) in [bk/drafts/categories/development-tools_cargo-plugins/performance.md](./bk/drafts/categories/development-tools_cargo-plugins/performance.md#L33)

#### bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md

- [ ] Review issue [#315](https://github.com/john-cd/rust_howto/issues/315) in [bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md](./bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md#L63)

#### bk/drafts/categories/development-tools_debugging/config_log.md

- [ ] Review issue [#925](https://github.com/john-cd/rust_howto/issues/925) in [bk/drafts/categories/development-tools_debugging/config_log.md](./bk/drafts/categories/development-tools_debugging/config_log.md#L87)

#### bk/drafts/categories/development-tools_debugging/debugging.md

- [ ] Review issue [#1344](https://github.com/john-cd/rust_howto/issues/1344) in [bk/drafts/categories/development-tools_debugging/debugging.md](./bk/drafts/categories/development-tools_debugging/debugging.md#L19)

#### bk/drafts/categories/development-tools_debugging/diagnostic_functions.md

- [ ] Review issue [#926](https://github.com/john-cd/rust_howto/issues/926) in [bk/drafts/categories/development-tools_debugging/diagnostic_functions.md](./bk/drafts/categories/development-tools_debugging/diagnostic_functions.md#L21)

#### bk/drafts/categories/development-tools_debugging/distributed_telemetry.md

- [ ] Review issue [#1343](https://github.com/john-cd/rust_howto/issues/1343) in [bk/drafts/categories/development-tools_debugging/distributed_telemetry.md](./bk/drafts/categories/development-tools_debugging/distributed_telemetry.md#L74)

#### bk/drafts/categories/development-tools_debugging/index.md

- [ ] Review issue [#319](https://github.com/john-cd/rust_howto/issues/319) in [bk/drafts/categories/development-tools_debugging/index.md](./bk/drafts/categories/development-tools_debugging/index.md#L67)

#### bk/drafts/categories/development-tools_debugging/log.md

- [ ] Review issue [#927](https://github.com/john-cd/rust_howto/issues/927) in [bk/drafts/categories/development-tools_debugging/log.md](./bk/drafts/categories/development-tools_debugging/log.md#L83)

#### bk/drafts/categories/development-tools_debugging/metrics.md

- [ ] Review issue [#1345](https://github.com/john-cd/rust_howto/issues/1345) in [bk/drafts/categories/development-tools_debugging/metrics.md](./bk/drafts/categories/development-tools_debugging/metrics.md#L25)

#### bk/drafts/categories/development-tools_debugging/tracing.md

- [ ] Review issue [#322](https://github.com/john-cd/rust_howto/issues/322) in [bk/drafts/categories/development-tools_debugging/tracing.md](./bk/drafts/categories/development-tools_debugging/tracing.md#L143)

#### bk/drafts/categories/development-tools_debugging/tracing_alternatives.md

- [ ] Review issue [#649](https://github.com/john-cd/rust_howto/issues/649) in [bk/drafts/categories/development-tools_debugging/tracing_alternatives.md](./bk/drafts/categories/development-tools_debugging/tracing_alternatives.md#L72)

#### bk/drafts/categories/development-tools_profiling/assembly.md

- [ ] Review issue [#333](https://github.com/john-cd/rust_howto/issues/333) in [bk/drafts/categories/development-tools_profiling/assembly.md](./bk/drafts/categories/development-tools_profiling/assembly.md#L22)

#### bk/drafts/categories/development-tools_profiling/benchmarking.md

- [ ] Review issue [#335](https://github.com/john-cd/rust_howto/issues/335) in [bk/drafts/categories/development-tools_profiling/benchmarking.md](./bk/drafts/categories/development-tools_profiling/benchmarking.md#L86)

#### bk/drafts/categories/development-tools_profiling/index.md

- [ ] Review issue [#337](https://github.com/john-cd/rust_howto/issues/337) in [bk/drafts/categories/development-tools_profiling/index.md](./bk/drafts/categories/development-tools_profiling/index.md#L41)

#### bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md

- [ ] Review issue [#336](https://github.com/john-cd/rust_howto/issues/336) in [bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md](./bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md#L49)

#### bk/drafts/categories/development-tools_testing/assertions.md

- [ ] Review issue [#1174](https://github.com/john-cd/rust_howto/issues/1174) in [bk/drafts/categories/development-tools_testing/assertions.md](./bk/drafts/categories/development-tools_testing/assertions.md#L41)

#### bk/drafts/categories/development-tools_testing/code_coverage.md

- [ ] Review issue [#1176](https://github.com/john-cd/rust_howto/issues/1176) in [bk/drafts/categories/development-tools_testing/code_coverage.md](./bk/drafts/categories/development-tools_testing/code_coverage.md#L30)

#### bk/drafts/categories/development-tools_testing/fuzzing.md

- [ ] Review issue [#339](https://github.com/john-cd/rust_howto/issues/339) in [bk/drafts/categories/development-tools_testing/fuzzing.md](./bk/drafts/categories/development-tools_testing/fuzzing.md#L72)

#### bk/drafts/categories/development-tools_testing/index.md

- [ ] Review issue [#341](https://github.com/john-cd/rust_howto/issues/341) in [bk/drafts/categories/development-tools_testing/index.md](./bk/drafts/categories/development-tools_testing/index.md#L31)

#### bk/drafts/categories/development-tools_testing/mocking.md

- [ ] Review issue [#340](https://github.com/john-cd/rust_howto/issues/340) in [bk/drafts/categories/development-tools_testing/mocking.md](./bk/drafts/categories/development-tools_testing/mocking.md#L38)

#### bk/drafts/categories/development-tools_testing/test_runners.md

- [ ] Review issue [#340](https://github.com/john-cd/rust_howto/issues/340) in [bk/drafts/categories/development-tools_testing/test_runners.md](./bk/drafts/categories/development-tools_testing/test_runners.md#L43)

#### bk/drafts/categories/development-tools_testing/testing.md

- [ ] Review issue [#340](https://github.com/john-cd/rust_howto/issues/340) in [bk/drafts/categories/development-tools_testing/testing.md](./bk/drafts/categories/development-tools_testing/testing.md#L58)

#### bk/drafts/categories/email/email_parsing.md

- [ ] Review issue [#1204](https://github.com/john-cd/rust_howto/issues/1204) in [bk/drafts/categories/email/email_parsing.md](./bk/drafts/categories/email/email_parsing.md#L25)

#### bk/drafts/categories/email/index.md

- [ ] Review issue [#343](https://github.com/john-cd/rust_howto/issues/343) in [bk/drafts/categories/email/index.md](./bk/drafts/categories/email/index.md#L29)

#### bk/drafts/categories/email/send_emails.md

- [ ] Review issue [#342](https://github.com/john-cd/rust_howto/issues/342) in [bk/drafts/categories/email/send_emails.md](./bk/drafts/categories/email/send_emails.md#L51)

#### bk/drafts/categories/encoding/_binary_encoders.md

- [ ] Review issue [#349](https://github.com/john-cd/rust_howto/issues/349) in [bk/drafts/categories/encoding/_binary_encoders.md](./bk/drafts/categories/encoding/_binary_encoders.md#L118)

#### bk/drafts/categories/encoding/complex_encoding.md

- [ ] Review issue [#350](https://github.com/john-cd/rust_howto/issues/350) in [bk/drafts/categories/encoding/complex_encoding.md](./bk/drafts/categories/encoding/complex_encoding.md#L54)

#### bk/drafts/categories/encoding/csv.md

- [ ] Review issue [#928](https://github.com/john-cd/rust_howto/issues/928) in [bk/drafts/categories/encoding/csv.md](./bk/drafts/categories/encoding/csv.md#L94)

#### bk/drafts/categories/encoding/index.md

- [ ] Review issue [#929](https://github.com/john-cd/rust_howto/issues/929) in [bk/drafts/categories/encoding/index.md](./bk/drafts/categories/encoding/index.md#L104)

#### bk/drafts/categories/encoding/no_external_schema.md

- [ ] Review issue [#1078](https://github.com/john-cd/rust_howto/issues/1078) in [bk/drafts/categories/encoding/no_external_schema.md](./bk/drafts/categories/encoding/no_external_schema.md#L35)

#### bk/drafts/categories/encoding/serde.md

- [ ] Review issue [#352](https://github.com/john-cd/rust_howto/issues/352) in [bk/drafts/categories/encoding/serde.md](./bk/drafts/categories/encoding/serde.md#L65)

#### bk/drafts/categories/encoding/string_encoding.md

- [ ] Review issue [#930](https://github.com/john-cd/rust_howto/issues/930) in [bk/drafts/categories/encoding/string_encoding.md](./bk/drafts/categories/encoding/string_encoding.md#L71)

#### bk/drafts/categories/encoding/typecasts.md

- [ ] Review issue [#354](https://github.com/john-cd/rust_howto/issues/354) in [bk/drafts/categories/encoding/typecasts.md](./bk/drafts/categories/encoding/typecasts.md#L98)

#### bk/drafts/categories/graphics/index.md

- [ ] Review issue [#1226](https://github.com/john-cd/rust_howto/issues/1226) in [bk/drafts/categories/graphics/index.md](./bk/drafts/categories/graphics/index.md#L80)

#### bk/drafts/categories/hardware-support/index.md

- [ ] Review issue [#70](https://github.com/john-cd/rust_howto/issues/70) in [bk/drafts/categories/hardware-support/index.md](./bk/drafts/categories/hardware-support/index.md#L67)

#### bk/drafts/categories/hardware-support/peripherals.md

- [ ] Review issue [#1177](https://github.com/john-cd/rust_howto/issues/1177) in [bk/drafts/categories/hardware-support/peripherals.md](./bk/drafts/categories/hardware-support/peripherals.md#L48)

#### bk/drafts/categories/hardware-support/processor.md

- [ ] Review issue [#399](https://github.com/john-cd/rust_howto/issues/399) in [bk/drafts/categories/hardware-support/processor.md](./bk/drafts/categories/hardware-support/processor.md#L81)

#### bk/drafts/categories/memory-management/index.md

- [ ] Review issue [#410](https://github.com/john-cd/rust_howto/issues/410) in [bk/drafts/categories/memory-management/index.md](./bk/drafts/categories/memory-management/index.md#L67)

#### bk/drafts/categories/memory-management/lazy_initialization.md

- [ ] Review issue [#411](https://github.com/john-cd/rust_howto/issues/411) in [bk/drafts/categories/memory-management/lazy_initialization.md](./bk/drafts/categories/memory-management/lazy_initialization.md#L95)

#### bk/drafts/categories/network-programming/index.md

- [ ] Review issue [#944](https://github.com/john-cd/rust_howto/issues/944) in [bk/drafts/categories/network-programming/index.md](./bk/drafts/categories/network-programming/index.md#L35)

#### bk/drafts/categories/network-programming/reverse_proxy.md

- [ ] Review issue [#424](https://github.com/john-cd/rust_howto/issues/424) in [bk/drafts/categories/network-programming/reverse_proxy.md](./bk/drafts/categories/network-programming/reverse_proxy.md#L94)

#### bk/drafts/categories/network-programming/server.md

- [ ] Review issue [#425](https://github.com/john-cd/rust_howto/issues/425) in [bk/drafts/categories/network-programming/server.md](./bk/drafts/categories/network-programming/server.md#L45)

#### bk/drafts/categories/os/external_commands.md

- [ ] Review issue [#946](https://github.com/john-cd/rust_howto/issues/946) in [bk/drafts/categories/os/external_commands.md](./bk/drafts/categories/os/external_commands.md#L99)

#### bk/drafts/categories/os/index.md

- [ ] Review issue [#429](https://github.com/john-cd/rust_howto/issues/429) in [bk/drafts/categories/os/index.md](./bk/drafts/categories/os/index.md#L41)

#### bk/drafts/categories/os/low_level_system_calls.md

- [ ] Review issue [#430](https://github.com/john-cd/rust_howto/issues/430) in [bk/drafts/categories/os/low_level_system_calls.md](./bk/drafts/categories/os/low_level_system_calls.md#L29)

#### bk/drafts/categories/os/rust_os.md

- [ ] Review issue [#639](https://github.com/john-cd/rust_howto/issues/639) in [bk/drafts/categories/os/rust_os.md](./bk/drafts/categories/os/rust_os.md#L23)

#### bk/drafts/categories/os_unix-apis/index.md

- [ ] Review issue [#437](https://github.com/john-cd/rust_howto/issues/437) in [bk/drafts/categories/os_unix-apis/index.md](./bk/drafts/categories/os_unix-apis/index.md#L51)

#### bk/drafts/categories/os_unix-apis/unix.md

- [ ] Review issue [#436](https://github.com/john-cd/rust_howto/issues/436) in [bk/drafts/categories/os_unix-apis/unix.md](./bk/drafts/categories/os_unix-apis/unix.md#L44)

#### bk/drafts/categories/os_windows-apis/index.md

- [ ] Review issue [#950](https://github.com/john-cd/rust_howto/issues/950) in [bk/drafts/categories/os_windows-apis/index.md](./bk/drafts/categories/os_windows-apis/index.md#L33)

#### bk/drafts/categories/os_windows-apis/windows.md

- [ ] Review issue [#438](https://github.com/john-cd/rust_howto/issues/438) in [bk/drafts/categories/os_windows-apis/windows.md](./bk/drafts/categories/os_windows-apis/windows.md#L48)

#### bk/drafts/categories/parser-implementations/html.md

- [ ] Review issue [#1187](https://github.com/john-cd/rust_howto/issues/1187) in [bk/drafts/categories/parser-implementations/html.md](./bk/drafts/categories/parser-implementations/html.md#L63)

#### bk/drafts/categories/parser-implementations/index.md

- [ ] Review issue [#447](https://github.com/john-cd/rust_howto/issues/447) in [bk/drafts/categories/parser-implementations/index.md](./bk/drafts/categories/parser-implementations/index.md#L51)

#### bk/drafts/categories/parser-implementations/ini.md

- [ ] Review issue [#1185](https://github.com/john-cd/rust_howto/issues/1185) in [bk/drafts/categories/parser-implementations/ini.md](./bk/drafts/categories/parser-implementations/ini.md#L27)

#### bk/drafts/categories/parser-implementations/json.md

- [ ] Review issue [#440](https://github.com/john-cd/rust_howto/issues/440) in [bk/drafts/categories/parser-implementations/json.md](./bk/drafts/categories/parser-implementations/json.md#L45)

#### bk/drafts/categories/parser-implementations/markdown.md

- [ ] Review issue [#442](https://github.com/john-cd/rust_howto/issues/442) in [bk/drafts/categories/parser-implementations/markdown.md](./bk/drafts/categories/parser-implementations/markdown.md#L45)

#### bk/drafts/categories/parser-implementations/programming_languages.md

- [ ] Review issue [#951](https://github.com/john-cd/rust_howto/issues/951) in [bk/drafts/categories/parser-implementations/programming_languages.md](./bk/drafts/categories/parser-implementations/programming_languages.md#L50)

#### bk/drafts/categories/parser-implementations/toml.md

- [ ] Review issue [#444](https://github.com/john-cd/rust_howto/issues/444) in [bk/drafts/categories/parser-implementations/toml.md](./bk/drafts/categories/parser-implementations/toml.md#L45)

#### bk/drafts/categories/parser-implementations/xml.md

- [ ] Review issue [#446](https://github.com/john-cd/rust_howto/issues/446) in [bk/drafts/categories/parser-implementations/xml.md](./bk/drafts/categories/parser-implementations/xml.md#L67)

#### bk/drafts/categories/parser-implementations/yaml.md

- [ ] Review issue [#1186](https://github.com/john-cd/rust_howto/issues/1186) in [bk/drafts/categories/parser-implementations/yaml.md](./bk/drafts/categories/parser-implementations/yaml.md#L27)

#### bk/drafts/categories/parsing/index.md

- [ ] Review issue [#952](https://github.com/john-cd/rust_howto/issues/952) in [bk/drafts/categories/parsing/index.md](./bk/drafts/categories/parsing/index.md#L32)

#### bk/drafts/categories/parsing/parsing.md

- [ ] Review issue [#448](https://github.com/john-cd/rust_howto/issues/448) in [bk/drafts/categories/parsing/parsing.md](./bk/drafts/categories/parsing/parsing.md#L66)

#### bk/drafts/categories/rust-patterns/behavioral_patterns.md

- [ ] Review issue [#461](https://github.com/john-cd/rust_howto/issues/461) in [bk/drafts/categories/rust-patterns/behavioral_patterns.md](./bk/drafts/categories/rust-patterns/behavioral_patterns.md#L19)

#### bk/drafts/categories/rust-patterns/builder_pattern.md

- [ ] Review issue [#648](https://github.com/john-cd/rust_howto/issues/648) in [bk/drafts/categories/rust-patterns/builder_pattern.md](./bk/drafts/categories/rust-patterns/builder_pattern.md#L43)

#### bk/drafts/categories/rust-patterns/creational_patterns.md

- [ ] Review issue [#1391](https://github.com/john-cd/rust_howto/issues/1391) in [bk/drafts/categories/rust-patterns/creational_patterns.md](./bk/drafts/categories/rust-patterns/creational_patterns.md#L35)

#### bk/drafts/categories/rust-patterns/error_handling/error_customization.md

- [ ] Review issue [#463](https://github.com/john-cd/rust_howto/issues/463) in [bk/drafts/categories/rust-patterns/error_handling/error_customization.md](./bk/drafts/categories/rust-patterns/error_handling/error_customization.md#L82)

#### bk/drafts/categories/rust-patterns/error_handling/error_handling.md

- [ ] Review issue [#465](https://github.com/john-cd/rust_howto/issues/465) in [bk/drafts/categories/rust-patterns/error_handling/error_handling.md](./bk/drafts/categories/rust-patterns/error_handling/error_handling.md#L212)

#### bk/drafts/categories/rust-patterns/functional_programming.md

- [ ] Review issue [#467](https://github.com/john-cd/rust_howto/issues/467) in [bk/drafts/categories/rust-patterns/functional_programming.md](./bk/drafts/categories/rust-patterns/functional_programming.md#L84)

#### bk/drafts/categories/rust-patterns/index.md

- [ ] Review issue [#469](https://github.com/john-cd/rust_howto/issues/469) in [bk/drafts/categories/rust-patterns/index.md](./bk/drafts/categories/rust-patterns/index.md#L144)

#### bk/drafts/categories/rust-patterns/rust_specific_patterns.md

- [ ] Review issue [#1393](https://github.com/john-cd/rust_howto/issues/1393) in [bk/drafts/categories/rust-patterns/rust_specific_patterns.md](./bk/drafts/categories/rust-patterns/rust_specific_patterns.md#L68)

#### bk/drafts/categories/rust-patterns/structural_patterns.md

- [ ] Review issue [#1392](https://github.com/john-cd/rust_howto/issues/1392) in [bk/drafts/categories/rust-patterns/structural_patterns.md](./bk/drafts/categories/rust-patterns/structural_patterns.md#L13)

#### bk/drafts/categories/template-engine/index.md

- [ ] Review issue [#482](https://github.com/john-cd/rust_howto/issues/482) in [bk/drafts/categories/template-engine/index.md](./bk/drafts/categories/template-engine/index.md#L62)

#### bk/drafts/categories/template-engine/tera.md

- [ ] Review issue [#483](https://github.com/john-cd/rust_howto/issues/483) in [bk/drafts/categories/template-engine/tera.md](./bk/drafts/categories/template-engine/tera.md#L23)

#### bk/drafts/categories/template-engine/tinytemplate.md

- [ ] Review issue [#484](https://github.com/john-cd/rust_howto/issues/484) in [bk/drafts/categories/template-engine/tinytemplate.md](./bk/drafts/categories/template-engine/tinytemplate.md#L23)

#### bk/drafts/categories/text-editors/ides.md

- [ ] Review issue [#486](https://github.com/john-cd/rust_howto/issues/486) in [bk/drafts/categories/text-editors/ides.md](./bk/drafts/categories/text-editors/ides.md#L54)

#### bk/drafts/categories/text-editors/index.md

- [ ] Review issue [#962](https://github.com/john-cd/rust_howto/issues/962) in [bk/drafts/categories/text-editors/index.md](./bk/drafts/categories/text-editors/index.md#L44)

#### bk/drafts/categories/text-processing/diffing.md

- [ ] Review issue [#1193](https://github.com/john-cd/rust_howto/issues/1193) in [bk/drafts/categories/text-processing/diffing.md](./bk/drafts/categories/text-processing/diffing.md#L34)

#### bk/drafts/categories/text-processing/index.md

- [ ] Review issue [#963](https://github.com/john-cd/rust_howto/issues/963) in [bk/drafts/categories/text-processing/index.md](./bk/drafts/categories/text-processing/index.md#L82)

#### bk/drafts/categories/text-processing/other_strings.md

- [ ] Review issue [#1194](https://github.com/john-cd/rust_howto/issues/1194) in [bk/drafts/categories/text-processing/other_strings.md](./bk/drafts/categories/text-processing/other_strings.md#L89)

#### bk/drafts/categories/text-processing/regex.md

- [ ] Review issue [#488](https://github.com/john-cd/rust_howto/issues/488) in [bk/drafts/categories/text-processing/regex.md](./bk/drafts/categories/text-processing/regex.md#L90)

#### bk/drafts/categories/text-processing/string_concat.md

- [ ] Review issue [#964](https://github.com/john-cd/rust_howto/issues/964) in [bk/drafts/categories/text-processing/string_concat.md](./bk/drafts/categories/text-processing/string_concat.md#L31)

#### bk/drafts/categories/text-processing/string_manipulation.md

- [ ] Review issue [#1190](https://github.com/john-cd/rust_howto/issues/1190) in [bk/drafts/categories/text-processing/string_manipulation.md](./bk/drafts/categories/text-processing/string_manipulation.md#L50)

#### bk/drafts/categories/text-processing/string_parsing.md

- [ ] Review issue [#965](https://github.com/john-cd/rust_howto/issues/965) in [bk/drafts/categories/text-processing/string_parsing.md](./bk/drafts/categories/text-processing/string_parsing.md#L26)

#### bk/drafts/categories/text-processing/string_search.md

- [ ] Review issue [#1191](https://github.com/john-cd/rust_howto/issues/1191) in [bk/drafts/categories/text-processing/string_search.md](./bk/drafts/categories/text-processing/string_search.md#L55)

#### bk/drafts/categories/text-processing/unicode.md

- [ ] Review issue [#1192](https://github.com/john-cd/rust_howto/issues/1192) in [bk/drafts/categories/text-processing/unicode.md](./bk/drafts/categories/text-processing/unicode.md#L26)

#### bk/drafts/categories/web-programming/http_types_and_interfaces.md

- [ ] Review issue [#1338](https://github.com/john-cd/rust_howto/issues/1338) in [bk/drafts/categories/web-programming/http_types_and_interfaces.md](./bk/drafts/categories/web-programming/http_types_and_interfaces.md#L38)

#### bk/drafts/categories/web-programming/index.md

- [ ] Review issue [#500](https://github.com/john-cd/rust_howto/issues/500) in [bk/drafts/categories/web-programming/index.md](./bk/drafts/categories/web-programming/index.md#L89)

#### bk/drafts/categories/web-programming/mime.md

- [ ] Review issue [#971](https://github.com/john-cd/rust_howto/issues/971) in [bk/drafts/categories/web-programming/mime.md](./bk/drafts/categories/web-programming/mime.md#L47)

#### bk/drafts/categories/web-programming/scraping.md

- [ ] Review issue [#972](https://github.com/john-cd/rust_howto/issues/972) in [bk/drafts/categories/web-programming/scraping.md](./bk/drafts/categories/web-programming/scraping.md#L48)

#### bk/drafts/categories/web-programming/url.md

- [ ] Review issue [#973](https://github.com/john-cd/rust_howto/issues/973) in [bk/drafts/categories/web-programming/url.md](./bk/drafts/categories/web-programming/url.md#L73)

#### bk/drafts/categories/web-programming_http-client/apis.md

- [ ] Review issue [#974](https://github.com/john-cd/rust_howto/issues/974) in [bk/drafts/categories/web-programming_http-client/apis.md](./bk/drafts/categories/web-programming_http-client/apis.md#L71)

#### bk/drafts/categories/web-programming_http-client/download.md

- [ ] Review issue [#975](https://github.com/john-cd/rust_howto/issues/975) in [bk/drafts/categories/web-programming_http-client/download.md](./bk/drafts/categories/web-programming_http-client/download.md#L50)

#### bk/drafts/categories/web-programming_http-client/http_clients.md

- [ ] Review issue [#504](https://github.com/john-cd/rust_howto/issues/504) in [bk/drafts/categories/web-programming_http-client/http_clients.md](./bk/drafts/categories/web-programming_http-client/http_clients.md#L54)

#### bk/drafts/categories/web-programming_http-client/index.md

- [ ] Review issue [#505](https://github.com/john-cd/rust_howto/issues/505) in [bk/drafts/categories/web-programming_http-client/index.md](./bk/drafts/categories/web-programming_http-client/index.md#L55)

#### bk/drafts/categories/web-programming_http-client/requests.md

- [ ] Review issue [#976](https://github.com/john-cd/rust_howto/issues/976) in [bk/drafts/categories/web-programming_http-client/requests.md](./bk/drafts/categories/web-programming_http-client/requests.md#L54)

#### bk/drafts/categories/web-programming_http-server/_actix.md

- [ ] Review issue [#506](https://github.com/john-cd/rust_howto/issues/506) in [bk/drafts/categories/web-programming_http-server/_actix.md](./bk/drafts/categories/web-programming_http-server/_actix.md#L30)

#### bk/drafts/categories/web-programming_http-server/_axum.md

- [ ] Review issue [#507](https://github.com/john-cd/rust_howto/issues/507) in [bk/drafts/categories/web-programming_http-server/_axum.md](./bk/drafts/categories/web-programming_http-server/_axum.md#L30)

#### bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md

- [ ] Review issue [#509](https://github.com/john-cd/rust_howto/issues/509) in [bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md](./bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md#L35)

#### bk/drafts/categories/web-programming_http-server/_graphql.md

- [ ] Review issue [#511](https://github.com/john-cd/rust_howto/issues/511) in [bk/drafts/categories/web-programming_http-server/_graphql.md](./bk/drafts/categories/web-programming_http-server/_graphql.md#L38)

#### bk/drafts/categories/web-programming_http-server/_grpc.md

- [ ] Review issue [#514](https://github.com/john-cd/rust_howto/issues/514) in [bk/drafts/categories/web-programming_http-server/_grpc.md](./bk/drafts/categories/web-programming_http-server/_grpc.md#L31)

#### bk/drafts/categories/web-programming_http-server/_hyper.md

- [ ] Review issue [#515](https://github.com/john-cd/rust_howto/issues/515) in [bk/drafts/categories/web-programming_http-server/_hyper.md](./bk/drafts/categories/web-programming_http-server/_hyper.md#L57)

#### bk/drafts/categories/web-programming_http-server/cors.md

- [ ] Review issue [#510](https://github.com/john-cd/rust_howto/issues/510) in [bk/drafts/categories/web-programming_http-server/cors.md](./bk/drafts/categories/web-programming_http-server/cors.md#L25)

#### bk/drafts/categories/web-programming_http-server/index.md

- [ ] Review issue [#977](https://github.com/john-cd/rust_howto/issues/977) in [bk/drafts/categories/web-programming_http-server/index.md](./bk/drafts/categories/web-programming_http-server/index.md#L93)

#### bk/drafts/categories/web-programming_http-server/middleware.md

- [ ] Review issue [#978](https://github.com/john-cd/rust_howto/issues/978) in [bk/drafts/categories/web-programming_http-server/middleware.md](./bk/drafts/categories/web-programming_http-server/middleware.md#L43)

#### bk/drafts/categories/web-programming_http-server/other_frameworks.md

- [ ] Review issue [#518](https://github.com/john-cd/rust_howto/issues/518) in [bk/drafts/categories/web-programming_http-server/other_frameworks.md](./bk/drafts/categories/web-programming_http-server/other_frameworks.md#L33)

#### bk/drafts/categories/web-programming_http-server/static_website_generators.md

- [ ] Review issue [#519](https://github.com/john-cd/rust_howto/issues/519) in [bk/drafts/categories/web-programming_http-server/static_website_generators.md](./bk/drafts/categories/web-programming_http-server/static_website_generators.md#L58)

#### bk/drafts/indices/crates_and_examples.md

- [ ] Review issue [#1395](https://github.com/john-cd/rust_howto/issues/1395) in [bk/drafts/indices/crates_and_examples.md](./bk/drafts/indices/crates_and_examples.md#L17)

#### bk/drafts/indices/crates_by_category2.md

- [ ] Review issue [#534](https://github.com/john-cd/rust_howto/issues/534) in [bk/drafts/indices/crates_by_category2.md](./bk/drafts/indices/crates_by_category2.md#L2)

#### bk/src/appendices/contributing/book_editing_and_example_code_development.md

- [ ] Review issue [#523](https://github.com/john-cd/rust_howto/issues/523) in [bk/src/appendices/contributing/book_editing_and_example_code_development.md](./bk/src/appendices/contributing/book_editing_and_example_code_development.md#L32)

#### bk/src/appendices/contributing/dev_container_and_docker.md

- [ ] Review issue [#525](https://github.com/john-cd/rust_howto/issues/525) in [bk/src/appendices/contributing/dev_container_and_docker.md](./bk/src/appendices/contributing/dev_container_and_docker.md#L92)

#### bk/src/appendices/contributing/index.md

- [ ] Review issue [#529](https://github.com/john-cd/rust_howto/issues/529) in [bk/src/appendices/contributing/index.md](./bk/src/appendices/contributing/index.md#L175)

#### bk/src/categories/data-structures/index.md

- [ ] Review issue [#280](https://github.com/john-cd/rust_howto/issues/280) in [bk/src/categories/data-structures/index.md](./bk/src/categories/data-structures/index.md#L101)

#### bk/src/categories/mathematics/complex_numbers.md

- [ ] Review issue [#935](https://github.com/john-cd/rust_howto/issues/935) in [bk/src/categories/mathematics/complex_numbers.md](./bk/src/categories/mathematics/complex_numbers.md#L64)

#### bk/src/categories/mathematics/trigonometry.md

- [ ] Review issue [#938](https://github.com/john-cd/rust_howto/issues/938) in [bk/src/categories/mathematics/trigonometry.md](./bk/src/categories/mathematics/trigonometry.md#L45)

#### later/src/categories/aerospace_protocols/index.md

- [ ] Review issue [#196](https://github.com/john-cd/rust_howto/issues/196) in [later/src/categories/aerospace_protocols/index.md](./later/src/categories/aerospace_protocols/index.md#L66)

#### later/src/categories/aerospace_simulation/aerospace_simulation.md

- [ ] Review issue [#199](https://github.com/john-cd/rust_howto/issues/199) in [later/src/categories/aerospace_simulation/aerospace_simulation.md](./later/src/categories/aerospace_simulation/aerospace_simulation.md#L20)

#### later/src/categories/aerospace_space-protocols/space_protocols.md

- [ ] Review issue [#201](https://github.com/john-cd/rust_howto/issues/201) in [later/src/categories/aerospace_space-protocols/space_protocols.md](./later/src/categories/aerospace_space-protocols/space_protocols.md#L20)

#### later/src/categories/compilers/index.md

- [ ] Review issue [#908](https://github.com/john-cd/rust_howto/issues/908) in [later/src/categories/compilers/index.md](./later/src/categories/compilers/index.md#L78)

#### later/src/categories/computer-vision/opencv.md

- [ ] Review issue [#257](https://github.com/john-cd/rust_howto/issues/257) in [later/src/categories/computer-vision/opencv.md](./later/src/categories/computer-vision/opencv.md#L51)

#### later/src/categories/cryptography_cryptocurrencies/index.md

- [ ] Review issue [#278](https://github.com/john-cd/rust_howto/issues/278) in [later/src/categories/cryptography_cryptocurrencies/index.md](./later/src/categories/cryptography_cryptocurrencies/index.md#L38)

#### later/src/categories/development-tools_ffi/erlang_elixir.md

- [ ] Review issue [#1070](https://github.com/john-cd/rust_howto/issues/1070) in [later/src/categories/development-tools_ffi/erlang_elixir.md](./later/src/categories/development-tools_ffi/erlang_elixir.md#L53)

#### later/src/categories/development-tools_ffi/flutter.md

- [ ] Review issue [#1071](https://github.com/john-cd/rust_howto/issues/1071) in [later/src/categories/development-tools_ffi/flutter.md](./later/src/categories/development-tools_ffi/flutter.md#L57)

#### later/src/categories/development-tools_ffi/generate_ffi_bindings.md

- [ ] Review issue [#324](https://github.com/john-cd/rust_howto/issues/324) in [later/src/categories/development-tools_ffi/generate_ffi_bindings.md](./later/src/categories/development-tools_ffi/generate_ffi_bindings.md#L85)

#### later/src/categories/development-tools_ffi/index.md

- [ ] Review issue [#325](https://github.com/john-cd/rust_howto/issues/325) in [later/src/categories/development-tools_ffi/index.md](./later/src/categories/development-tools_ffi/index.md#L91)

#### later/src/categories/development-tools_ffi/java.md

- [ ] Review issue [#1072](https://github.com/john-cd/rust_howto/issues/1072) in [later/src/categories/development-tools_ffi/java.md](./later/src/categories/development-tools_ffi/java.md#L23)

#### later/src/categories/development-tools_ffi/lua.md

- [ ] Review issue [#1073](https://github.com/john-cd/rust_howto/issues/1073) in [later/src/categories/development-tools_ffi/lua.md](./later/src/categories/development-tools_ffi/lua.md#L27)

#### later/src/categories/development-tools_ffi/node.md

- [ ] Review issue [#1074](https://github.com/john-cd/rust_howto/issues/1074) in [later/src/categories/development-tools_ffi/node.md](./later/src/categories/development-tools_ffi/node.md#L33)

#### later/src/categories/development-tools_ffi/objc.md

- [ ] Review issue [#1075](https://github.com/john-cd/rust_howto/issues/1075) in [later/src/categories/development-tools_ffi/objc.md](./later/src/categories/development-tools_ffi/objc.md#L23)

#### later/src/categories/development-tools_ffi/python.md

- [ ] Review issue [#210](https://github.com/john-cd/rust_howto/issues/210) in [later/src/categories/development-tools_ffi/python.md](./later/src/categories/development-tools_ffi/python.md#L63)

#### later/src/categories/development-tools_ffi/ruby.md

- [ ] Review issue [#1076](https://github.com/john-cd/rust_howto/issues/1076) in [later/src/categories/development-tools_ffi/ruby.md](./later/src/categories/development-tools_ffi/ruby.md#L53)

#### later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md

- [ ] Review issue [#327](https://github.com/john-cd/rust_howto/issues/327) in [later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md](./later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md#L27)

#### later/src/categories/development-tools_procedural-macro-helpers/index.md

- [ ] Review issue [#332](https://github.com/john-cd/rust_howto/issues/332) in [later/src/categories/development-tools_procedural-macro-helpers/index.md](./later/src/categories/development-tools_procedural-macro-helpers/index.md#L31)

#### later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md

- [ ] Review issue [#329](https://github.com/john-cd/rust_howto/issues/329) in [later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md](./later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md#L24)

#### later/src/categories/development-tools_procedural-macro-helpers/write_proc_macros.md

- [ ] Review issue [#331](https://github.com/john-cd/rust_howto/issues/331) in [later/src/categories/development-tools_procedural-macro-helpers/write_proc_macros.md](./later/src/categories/development-tools_procedural-macro-helpers/write_proc_macros.md#L105)

#### later/src/categories/embedded/embassy.md

- [ ] Review issue [#345](https://github.com/john-cd/rust_howto/issues/345) in [later/src/categories/embedded/embassy.md](./later/src/categories/embedded/embassy.md#L26)

#### later/src/categories/embedded/flash.md

- [ ] Review issue [#1200](https://github.com/john-cd/rust_howto/issues/1200) in [later/src/categories/embedded/flash.md](./later/src/categories/embedded/flash.md#L15)

#### later/src/categories/embedded/hals.md

- [ ] Review issue [#1203](https://github.com/john-cd/rust_howto/issues/1203) in [later/src/categories/embedded/hals.md](./later/src/categories/embedded/hals.md#L39)

#### later/src/categories/embedded/index.md

- [ ] Review issue [#346](https://github.com/john-cd/rust_howto/issues/346) in [later/src/categories/embedded/index.md](./later/src/categories/embedded/index.md#L75)

#### later/src/categories/embedded/pacs.md

- [ ] Review issue [#1198](https://github.com/john-cd/rust_howto/issues/1198) in [later/src/categories/embedded/pacs.md](./later/src/categories/embedded/pacs.md#L21)

#### later/src/categories/embedded/rtos.md

- [ ] Review issue [#1202](https://github.com/john-cd/rust_howto/issues/1202) in [later/src/categories/embedded/rtos.md](./later/src/categories/embedded/rtos.md#L17)

#### later/src/categories/embedded/sensors.md

- [ ] Review issue [#1199](https://github.com/john-cd/rust_howto/issues/1199) in [later/src/categories/embedded/sensors.md](./later/src/categories/embedded/sensors.md#L13)

#### later/src/categories/embedded/useful_crates_embedded.md

- [ ] Review issue [#1201](https://github.com/john-cd/rust_howto/issues/1201) in [later/src/categories/embedded/useful_crates_embedded.md](./later/src/categories/embedded/useful_crates_embedded.md#L39)

#### later/src/categories/emulators/emulators.md

- [ ] Review issue [#347](https://github.com/john-cd/rust_howto/issues/347) in [later/src/categories/emulators/emulators.md](./later/src/categories/emulators/emulators.md#L29)

#### later/src/categories/emulators/index.md

- [ ] Review issue [#348](https://github.com/john-cd/rust_howto/issues/348) in [later/src/categories/emulators/index.md](./later/src/categories/emulators/index.md#L63)

#### later/src/categories/external-ffi-bindings/external_ffi_bindings.md

- [ ] Review issue [#355](https://github.com/john-cd/rust_howto/issues/355) in [later/src/categories/external-ffi-bindings/external_ffi_bindings.md](./later/src/categories/external-ffi-bindings/external_ffi_bindings.md#L17)

#### later/src/categories/external-ffi-bindings/index.md

- [ ] Review issue [#356](https://github.com/john-cd/rust_howto/issues/356) in [later/src/categories/external-ffi-bindings/index.md](./later/src/categories/external-ffi-bindings/index.md#L28)

#### later/src/categories/finance/index.md

- [ ] Review issue [#365](https://github.com/john-cd/rust_howto/issues/365) in [later/src/categories/finance/index.md](./later/src/categories/finance/index.md#L85)

#### later/src/categories/finance/quant.md

- [ ] Review issue [#364](https://github.com/john-cd/rust_howto/issues/364) in [later/src/categories/finance/quant.md](./later/src/categories/finance/quant.md#L25)

#### later/src/categories/game-development/game_development.md

- [ ] Review issue [#366](https://github.com/john-cd/rust_howto/issues/366) in [later/src/categories/game-development/game_development.md](./later/src/categories/game-development/game_development.md#L34)

#### later/src/categories/game-development/index.md

- [ ] Review issue [#367](https://github.com/john-cd/rust_howto/issues/367) in [later/src/categories/game-development/index.md](./later/src/categories/game-development/index.md#L67)

#### later/src/categories/game-engines/game_engines.md

- [ ] Review issue [#369](https://github.com/john-cd/rust_howto/issues/369) in [later/src/categories/game-engines/game_engines.md](./later/src/categories/game-engines/game_engines.md#L57)

#### later/src/categories/game-engines/index.md

- [ ] Review issue [#370](https://github.com/john-cd/rust_howto/issues/370) in [later/src/categories/game-engines/index.md](./later/src/categories/game-engines/index.md#L41)

#### later/src/categories/games/index.md

- [ ] Review issue [#373](https://github.com/john-cd/rust_howto/issues/373) in [later/src/categories/games/index.md](./later/src/categories/games/index.md#L31)

#### later/src/categories/gui/clipboard.md

- [ ] Review issue [#638](https://github.com/john-cd/rust_howto/issues/638) in [later/src/categories/gui/clipboard.md](./later/src/categories/gui/clipboard.md#L33)

#### later/src/categories/gui/file_dialogs.md

- [ ] Review issue [#381](https://github.com/john-cd/rust_howto/issues/381) in [later/src/categories/gui/file_dialogs.md](./later/src/categories/gui/file_dialogs.md#L35)

#### later/src/categories/gui/gtk.md

- [ ] Review issue [#383](https://github.com/john-cd/rust_howto/issues/383) in [later/src/categories/gui/gtk.md](./later/src/categories/gui/gtk.md#L51)

#### later/src/categories/gui/immediate_mode_gui.md

- [ ] Review issue [#385](https://github.com/john-cd/rust_howto/issues/385) in [later/src/categories/gui/immediate_mode_gui.md](./later/src/categories/gui/immediate_mode_gui.md#L55)

#### later/src/categories/gui/index.md

- [ ] Review issue [#397](https://github.com/john-cd/rust_howto/issues/397) in [later/src/categories/gui/index.md](./later/src/categories/gui/index.md#L109)

#### later/src/categories/gui/retained_mode_gui.md

- [ ] Review issue [#389](https://github.com/john-cd/rust_howto/issues/389) in [later/src/categories/gui/retained_mode_gui.md](./later/src/categories/gui/retained_mode_gui.md#L97)

#### later/src/categories/gui/text_layout.md

- [ ] Review issue [#391](https://github.com/john-cd/rust_howto/issues/391) in [later/src/categories/gui/text_layout.md](./later/src/categories/gui/text_layout.md#L33)

#### later/src/categories/gui/ui_layout.md

- [ ] Review issue [#934](https://github.com/john-cd/rust_howto/issues/934) in [later/src/categories/gui/ui_layout.md](./later/src/categories/gui/ui_layout.md#L42)

#### later/src/categories/gui/web_based_gui.md

- [ ] Review issue [#394](https://github.com/john-cd/rust_howto/issues/394) in [later/src/categories/gui/web_based_gui.md](./later/src/categories/gui/web_based_gui.md#L45)

#### later/src/categories/gui/window_creation.md

- [ ] Review issue [#396](https://github.com/john-cd/rust_howto/issues/396) in [later/src/categories/gui/window_creation.md](./later/src/categories/gui/window_creation.md#L50)

#### later/src/categories/internationalization/index.md

- [ ] Review issue [#402](https://github.com/john-cd/rust_howto/issues/402) in [later/src/categories/internationalization/index.md](./later/src/categories/internationalization/index.md#L80)

#### later/src/categories/internationalization/internationalization.md

- [ ] Review issue [#401](https://github.com/john-cd/rust_howto/issues/401) in [later/src/categories/internationalization/internationalization.md](./later/src/categories/internationalization/internationalization.md#L23)

#### later/src/categories/localization/index.md

- [ ] Review issue [#405](https://github.com/john-cd/rust_howto/issues/405) in [later/src/categories/localization/index.md](./later/src/categories/localization/index.md#L30)

#### later/src/categories/localization/localization.md

- [ ] Review issue [#404](https://github.com/john-cd/rust_howto/issues/404) in [later/src/categories/localization/localization.md](./later/src/categories/localization/localization.md#L19)

#### later/src/categories/multimedia/index.md

- [ ] Review issue [#940](https://github.com/john-cd/rust_howto/issues/940) in [later/src/categories/multimedia/index.md](./later/src/categories/multimedia/index.md#L66)

#### later/src/categories/multimedia/multimedia.md

- [ ] Review issue [#413](https://github.com/john-cd/rust_howto/issues/413) in [later/src/categories/multimedia/multimedia.md](./later/src/categories/multimedia/multimedia.md#L19)

#### later/src/categories/multimedia_audio/audio.md

- [ ] Review issue [#415](https://github.com/john-cd/rust_howto/issues/415) in [later/src/categories/multimedia_audio/audio.md](./later/src/categories/multimedia_audio/audio.md#L21)

#### later/src/categories/multimedia_audio/index.md

- [ ] Review issue [#941](https://github.com/john-cd/rust_howto/issues/941) in [later/src/categories/multimedia_audio/index.md](./later/src/categories/multimedia_audio/index.md#L56)

#### later/src/categories/multimedia_encoding/encoding.md

- [ ] Review issue [#417](https://github.com/john-cd/rust_howto/issues/417) in [later/src/categories/multimedia_encoding/encoding.md](./later/src/categories/multimedia_encoding/encoding.md#L19)

#### later/src/categories/multimedia_encoding/index.md

- [ ] Review issue [#418](https://github.com/john-cd/rust_howto/issues/418) in [later/src/categories/multimedia_encoding/index.md](./later/src/categories/multimedia_encoding/index.md#L42)

#### later/src/categories/multimedia_images/color_handling.md

- [ ] Review issue [#1223](https://github.com/john-cd/rust_howto/issues/1223) in [later/src/categories/multimedia_images/color_handling.md](./later/src/categories/multimedia_images/color_handling.md#L21)

#### later/src/categories/multimedia_images/images.md

- [ ] Review issue [#420](https://github.com/john-cd/rust_howto/issues/420) in [later/src/categories/multimedia_images/images.md](./later/src/categories/multimedia_images/images.md#L58)

#### later/src/categories/multimedia_images/index.md

- [ ] Review issue [#942](https://github.com/john-cd/rust_howto/issues/942) in [later/src/categories/multimedia_images/index.md](./later/src/categories/multimedia_images/index.md#L58)

#### later/src/categories/multimedia_images/pixel_buffers.md

- [ ] Review issue [#1222](https://github.com/john-cd/rust_howto/issues/1222) in [later/src/categories/multimedia_images/pixel_buffers.md](./later/src/categories/multimedia_images/pixel_buffers.md#L21)

#### later/src/categories/multimedia_video/index.md

- [ ] Review issue [#943](https://github.com/john-cd/rust_howto/issues/943) in [later/src/categories/multimedia_video/index.md](./later/src/categories/multimedia_video/index.md#L46)

#### later/src/categories/multimedia_video/video.md

- [ ] Review issue [#422](https://github.com/john-cd/rust_howto/issues/422) in [later/src/categories/multimedia_video/video.md](./later/src/categories/multimedia_video/video.md#L20)

#### later/src/categories/no-std/index.md

- [ ] Review issue [#427](https://github.com/john-cd/rust_howto/issues/427) in [later/src/categories/no-std/index.md](./later/src/categories/no-std/index.md#L50)

#### later/src/categories/no-std/no_std.md

- [ ] Review issue [#426](https://github.com/john-cd/rust_howto/issues/426) in [later/src/categories/no-std/no_std.md](./later/src/categories/no-std/no_std.md#L19)

#### later/src/categories/no-std_no-alloc/index.md

- [ ] Review issue [#945](https://github.com/john-cd/rust_howto/issues/945) in [later/src/categories/no-std_no-alloc/index.md](./later/src/categories/no-std_no-alloc/index.md#L63)

#### later/src/categories/no-std_no-alloc/no_alloc.md

- [ ] Review issue [#428](https://github.com/john-cd/rust_howto/issues/428) in [later/src/categories/no-std_no-alloc/no_alloc.md](./later/src/categories/no-std_no-alloc/no_alloc.md#L19)

#### later/src/categories/os_freebsd-apis/freebsd.md

- [ ] Review issue [#433](https://github.com/john-cd/rust_howto/issues/433) in [later/src/categories/os_freebsd-apis/freebsd.md](./later/src/categories/os_freebsd-apis/freebsd.md#L19)

#### later/src/categories/os_freebsd-apis/index.md

- [ ] Review issue [#947](https://github.com/john-cd/rust_howto/issues/947) in [later/src/categories/os_freebsd-apis/index.md](./later/src/categories/os_freebsd-apis/index.md#L35)

#### later/src/categories/os_linux-apis/index.md

- [ ] Review issue [#948](https://github.com/john-cd/rust_howto/issues/948) in [later/src/categories/os_linux-apis/index.md](./later/src/categories/os_linux-apis/index.md#L66)

#### later/src/categories/os_linux-apis/linux.md

- [ ] Review issue [#434](https://github.com/john-cd/rust_howto/issues/434) in [later/src/categories/os_linux-apis/linux.md](./later/src/categories/os_linux-apis/linux.md#L19)

#### later/src/categories/os_macos-apis/index.md

- [ ] Review issue [#949](https://github.com/john-cd/rust_howto/issues/949) in [later/src/categories/os_macos-apis/index.md](./later/src/categories/os_macos-apis/index.md#L63)

#### later/src/categories/os_macos-apis/macos.md

- [ ] Review issue [#435](https://github.com/john-cd/rust_howto/issues/435) in [later/src/categories/os_macos-apis/macos.md](./later/src/categories/os_macos-apis/macos.md#L19)

#### later/src/categories/rendering/2d_raster_graphics.md

- [ ] Review issue [#1215](https://github.com/john-cd/rust_howto/issues/1215) in [later/src/categories/rendering/2d_raster_graphics.md](./later/src/categories/rendering/2d_raster_graphics.md#L35)

#### later/src/categories/rendering/2d_renderers.md

- [ ] Review issue [#377](https://github.com/john-cd/rust_howto/issues/377) in [later/src/categories/rendering/2d_renderers.md](./later/src/categories/rendering/2d_renderers.md#L98)

#### later/src/categories/rendering/2d_vector_graphics.md

- [ ] Review issue [#1214](https://github.com/john-cd/rust_howto/issues/1214) in [later/src/categories/rendering/2d_vector_graphics.md](./later/src/categories/rendering/2d_vector_graphics.md#L53)

#### later/src/categories/rendering/3d_renderers.md

- [ ] Review issue [#1213](https://github.com/john-cd/rust_howto/issues/1213) in [later/src/categories/rendering/3d_renderers.md](./later/src/categories/rendering/3d_renderers.md#L66)

#### later/src/categories/rendering/index.md

- [ ] Review issue [#953](https://github.com/john-cd/rust_howto/issues/953) in [later/src/categories/rendering/index.md](./later/src/categories/rendering/index.md#L78)

#### later/src/categories/rendering/svg_rendering.md

- [ ] Review issue [#1216](https://github.com/john-cd/rust_howto/issues/1216) in [later/src/categories/rendering/svg_rendering.md](./later/src/categories/rendering/svg_rendering.md#L27)

#### later/src/categories/rendering/text_rendering.md

- [ ] Review issue [#1212](https://github.com/john-cd/rust_howto/issues/1212) in [later/src/categories/rendering/text_rendering.md](./later/src/categories/rendering/text_rendering.md#L48)

#### later/src/categories/rendering_data-formats/data_formats.md

- [ ] Review issue [#453](https://github.com/john-cd/rust_howto/issues/453) in [later/src/categories/rendering_data-formats/data_formats.md](./later/src/categories/rendering_data-formats/data_formats.md#L19)

#### later/src/categories/rendering_data-formats/index.md

- [ ] Review issue [#954](https://github.com/john-cd/rust_howto/issues/954) in [later/src/categories/rendering_data-formats/index.md](./later/src/categories/rendering_data-formats/index.md#L48)

#### later/src/categories/rendering_engine/index.md

- [ ] Review issue [#955](https://github.com/john-cd/rust_howto/issues/955) in [later/src/categories/rendering_engine/index.md](./later/src/categories/rendering_engine/index.md#L34)

#### later/src/categories/rendering_engine/rendering_engines.md

- [ ] Review issue [#455](https://github.com/john-cd/rust_howto/issues/455) in [later/src/categories/rendering_engine/rendering_engines.md](./later/src/categories/rendering_engine/rendering_engines.md#L19)

#### later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md

- [ ] Review issue [#375](https://github.com/john-cd/rust_howto/issues/375) in [later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md](./later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md#L61)

#### later/src/categories/rendering_graphics-api/index.md

- [ ] Review issue [#956](https://github.com/john-cd/rust_howto/issues/956) in [later/src/categories/rendering_graphics-api/index.md](./later/src/categories/rendering_graphics-api/index.md#L62)

#### later/src/categories/rendering_graphics-api/native_graphics_apis.md

- [ ] Review issue [#457](https://github.com/john-cd/rust_howto/issues/457) in [later/src/categories/rendering_graphics-api/native_graphics_apis.md](./later/src/categories/rendering_graphics-api/native_graphics_apis.md#L47)

#### later/src/categories/rendering_graphics-api/opengl.md

- [ ] Review issue [#1229](https://github.com/john-cd/rust_howto/issues/1229) in [later/src/categories/rendering_graphics-api/opengl.md](./later/src/categories/rendering_graphics-api/opengl.md#L37)

#### later/src/categories/rendering_graphics-api/shaders.md

- [ ] Review issue [#1228](https://github.com/john-cd/rust_howto/issues/1228) in [later/src/categories/rendering_graphics-api/shaders.md](./later/src/categories/rendering_graphics-api/shaders.md#L77)

#### later/src/categories/rendering_graphics-api/vulkan.md

- [ ] Review issue [#1227](https://github.com/john-cd/rust_howto/issues/1227) in [later/src/categories/rendering_graphics-api/vulkan.md](./later/src/categories/rendering_graphics-api/vulkan.md#L27)

#### later/src/categories/science/classical_machine_learning.md

- [ ] Review issue [#473](https://github.com/john-cd/rust_howto/issues/473) in [later/src/categories/science/classical_machine_learning.md](./later/src/categories/science/classical_machine_learning.md#L44)

#### later/src/categories/science/deep_learning.md

- [ ] Review issue [#1205](https://github.com/john-cd/rust_howto/issues/1205) in [later/src/categories/science/deep_learning.md](./later/src/categories/science/deep_learning.md#L38)

#### later/src/categories/science/index.md

- [ ] Review issue [#472](https://github.com/john-cd/rust_howto/issues/472) in [later/src/categories/science/index.md](./later/src/categories/science/index.md#L56)

#### later/src/categories/science_geo/geo.md

- [ ] Review issue [#474](https://github.com/john-cd/rust_howto/issues/474) in [later/src/categories/science_geo/geo.md](./later/src/categories/science_geo/geo.md#L23)

#### later/src/categories/science_geo/index.md

- [ ] Review issue [#958](https://github.com/john-cd/rust_howto/issues/958) in [later/src/categories/science_geo/index.md](./later/src/categories/science_geo/index.md#L39)

#### later/src/categories/science_neuroscience/index.md

- [ ] Review issue [#959](https://github.com/john-cd/rust_howto/issues/959) in [later/src/categories/science_neuroscience/index.md](./later/src/categories/science_neuroscience/index.md#L41)
- [ ] Review issue [#1197](https://github.com/john-cd/rust_howto/issues/1197) in [later/src/categories/science_neuroscience/index.md](./later/src/categories/science_neuroscience/index.md#L42)

#### later/src/categories/science_neuroscience/neuroscience.md

- [ ] Review issue [#476](https://github.com/john-cd/rust_howto/issues/476) in [later/src/categories/science_neuroscience/neuroscience.md](./later/src/categories/science_neuroscience/neuroscience.md#L19)

#### later/src/categories/science_robotics/artificial_intelligence.md

- [ ] Review issue [#1209](https://github.com/john-cd/rust_howto/issues/1209) in [later/src/categories/science_robotics/artificial_intelligence.md](./later/src/categories/science_robotics/artificial_intelligence.md#L32)

#### later/src/categories/science_robotics/control_systems.md

- [ ] Review issue [#1208](https://github.com/john-cd/rust_howto/issues/1208) in [later/src/categories/science_robotics/control_systems.md](./later/src/categories/science_robotics/control_systems.md#L62)

#### later/src/categories/science_robotics/hardware_integration.md

- [ ] Review issue [#1211](https://github.com/john-cd/rust_howto/issues/1211) in [later/src/categories/science_robotics/hardware_integration.md](./later/src/categories/science_robotics/hardware_integration.md#L26)

#### later/src/categories/science_robotics/index.md

- [ ] Review issue [#480](https://github.com/john-cd/rust_howto/issues/480) in [later/src/categories/science_robotics/index.md](./later/src/categories/science_robotics/index.md#L87)

#### later/src/categories/science_robotics/perception_and_sensors.md

- [ ] Review issue [#1207](https://github.com/john-cd/rust_howto/issues/1207) in [later/src/categories/science_robotics/perception_and_sensors.md](./later/src/categories/science_robotics/perception_and_sensors.md#L31)

#### later/src/categories/science_robotics/robot_operating_systems.md

- [ ] Review issue [#477](https://github.com/john-cd/rust_howto/issues/477) in [later/src/categories/science_robotics/robot_operating_systems.md](./later/src/categories/science_robotics/robot_operating_systems.md#L79)

#### later/src/categories/science_robotics/robotics_frameworks.md

- [ ] Review issue [#479](https://github.com/john-cd/rust_howto/issues/479) in [later/src/categories/science_robotics/robotics_frameworks.md](./later/src/categories/science_robotics/robotics_frameworks.md#L64)

#### later/src/categories/science_robotics/simulation_visualization.md

- [ ] Review issue [#1210](https://github.com/john-cd/rust_howto/issues/1210) in [later/src/categories/science_robotics/simulation_visualization.md](./later/src/categories/science_robotics/simulation_visualization.md#L40)

#### later/src/categories/simulation/index.md

- [ ] Review issue [#961](https://github.com/john-cd/rust_howto/issues/961) in [later/src/categories/simulation/index.md](./later/src/categories/simulation/index.md#L45)

#### later/src/categories/simulation/physics_engines.md

- [ ] Review issue [#1206](https://github.com/john-cd/rust_howto/issues/1206) in [later/src/categories/simulation/physics_engines.md](./later/src/categories/simulation/physics_engines.md#L21)

#### later/src/categories/simulation/simulation.md

- [ ] Review issue [#481](https://github.com/john-cd/rust_howto/issues/481) in [later/src/categories/simulation/simulation.md](./later/src/categories/simulation/simulation.md#L21)

#### later/src/categories/value-formatting/index.md

- [ ] Review issue [#967](https://github.com/john-cd/rust_howto/issues/967) in [later/src/categories/value-formatting/index.md](./later/src/categories/value-formatting/index.md#L37)

#### later/src/categories/value-formatting/number_formatting.md

- [ ] Review issue [#490](https://github.com/john-cd/rust_howto/issues/490) in [later/src/categories/value-formatting/number_formatting.md](./later/src/categories/value-formatting/number_formatting.md#L33)

#### later/src/categories/virtualization/containerization.md

- [ ] Review issue [#1225](https://github.com/john-cd/rust_howto/issues/1225) in [later/src/categories/virtualization/containerization.md](./later/src/categories/virtualization/containerization.md#L43)

#### later/src/categories/virtualization/containers.md

- [ ] Review issue [#988](https://github.com/john-cd/rust_howto/issues/988) in [later/src/categories/virtualization/containers.md](./later/src/categories/virtualization/containers.md#L39)

#### later/src/categories/virtualization/index.md

- [ ] Review issue [#968](https://github.com/john-cd/rust_howto/issues/968) in [later/src/categories/virtualization/index.md](./later/src/categories/virtualization/index.md#L90)

#### later/src/categories/virtualization/using_containers.md

- [ ] Review issue [#1224](https://github.com/john-cd/rust_howto/issues/1224) in [later/src/categories/virtualization/using_containers.md](./later/src/categories/virtualization/using_containers.md#L22)

#### later/src/categories/virtualization/virtualization.md

- [ ] Review issue [#492](https://github.com/john-cd/rust_howto/issues/492) in [later/src/categories/virtualization/virtualization.md](./later/src/categories/virtualization/virtualization.md#L19)

#### later/src/categories/visualization/index.md

- [ ] Review issue [#969](https://github.com/john-cd/rust_howto/issues/969) in [later/src/categories/visualization/index.md](./later/src/categories/visualization/index.md#L49)

#### later/src/categories/visualization/visualization.md

- [ ] Review issue [#494](https://github.com/john-cd/rust_howto/issues/494) in [later/src/categories/visualization/visualization.md](./later/src/categories/visualization/visualization.md#L34)

#### later/src/categories/wasm/index.md

- [ ] Review issue [#970](https://github.com/john-cd/rust_howto/issues/970) in [later/src/categories/wasm/index.md](./later/src/categories/wasm/index.md#L50)

#### later/src/categories/wasm/interfacing_with_javascript.md

- [ ] Review issue [#1218](https://github.com/john-cd/rust_howto/issues/1218) in [later/src/categories/wasm/interfacing_with_javascript.md](./later/src/categories/wasm/interfacing_with_javascript.md#L46)

#### later/src/categories/wasm/leptos.md

- [ ] Review issue [#1220](https://github.com/john-cd/rust_howto/issues/1220) in [later/src/categories/wasm/leptos.md](./later/src/categories/wasm/leptos.md#L27)

#### later/src/categories/wasm/wasm_basics.md

- [ ] Review issue [#1219](https://github.com/john-cd/rust_howto/issues/1219) in [later/src/categories/wasm/wasm_basics.md](./later/src/categories/wasm/wasm_basics.md#L64)

#### later/src/categories/wasm/wasm_development.md

- [ ] Review issue [#1217](https://github.com/john-cd/rust_howto/issues/1217) in [later/src/categories/wasm/wasm_development.md](./later/src/categories/wasm/wasm_development.md#L86)

#### later/src/categories/wasm/wasm_standalone_runtimes.md

- [ ] Review issue [#496](https://github.com/john-cd/rust_howto/issues/496) in [later/src/categories/wasm/wasm_standalone_runtimes.md](./later/src/categories/wasm/wasm_standalone_runtimes.md#L113)

#### later/src/categories/wasm/yew.md

- [ ] Review issue [#498](https://github.com/john-cd/rust_howto/issues/498) in [later/src/categories/wasm/yew.md](./later/src/categories/wasm/yew.md#L34)

#### later/src/categories/web-programming_websocket/index.md

- [ ] Review issue [#979](https://github.com/john-cd/rust_howto/issues/979) in [later/src/categories/web-programming_websocket/index.md](./later/src/categories/web-programming_websocket/index.md#L43)

#### later/src/categories/web-programming_websocket/websocket.md

- [ ] Review issue [#520](https://github.com/john-cd/rust_howto/issues/520) in [later/src/categories/web-programming_websocket/websocket.md](./later/src/categories/web-programming_websocket/websocket.md#L56)

#### later/src/other/architecture/architectural_patterns.md

- [ ] Review issue [#1231](https://github.com/john-cd/rust_howto/issues/1231) in [later/src/other/architecture/architectural_patterns.md](./later/src/other/architecture/architectural_patterns.md#L49)

#### later/src/other/architecture/common_architectures.md

- [ ] Review issue [#1230](https://github.com/john-cd/rust_howto/issues/1230) in [later/src/other/architecture/common_architectures.md](./later/src/other/architecture/common_architectures.md#L135)

#### later/src/other/architecture/index.md

- [ ] Review issue [#573](https://github.com/john-cd/rust_howto/issues/573) in [later/src/other/architecture/index.md](./later/src/other/architecture/index.md#L29)

#### later/src/other/architecture/software_architecture_process.md

- [ ] Review issue [#572](https://github.com/john-cd/rust_howto/issues/572) in [later/src/other/architecture/software_architecture_process.md](./later/src/other/architecture/software_architecture_process.md#L103)

#### later/src/other/cloud/_fragment.md

- [ ] Review issue [#578](https://github.com/john-cd/rust_howto/issues/578) in [later/src/other/cloud/_fragment.md](./later/src/other/cloud/_fragment.md#L18)

#### later/src/other/cloud/aws.md

- [ ] Review issue [#574](https://github.com/john-cd/rust_howto/issues/574) in [later/src/other/cloud/aws.md](./later/src/other/cloud/aws.md#L60)

#### later/src/other/cloud/index.md

- [ ] Review issue [#579](https://github.com/john-cd/rust_howto/issues/579) in [later/src/other/cloud/index.md](./later/src/other/cloud/index.md#L39)

#### later/src/other/cloud/rust_native_cloud_development.md

- [ ] Review issue [#576](https://github.com/john-cd/rust_howto/issues/576) in [later/src/other/cloud/rust_native_cloud_development.md](./later/src/other/cloud/rust_native_cloud_development.md#L41)

#### later/src/other/cross-platform/crux.md

- [ ] Review issue [#582](https://github.com/john-cd/rust_howto/issues/582) in [later/src/other/cross-platform/crux.md](./later/src/other/cross-platform/crux.md#L29)

#### later/src/other/cross-platform/index.md

- [ ] Review issue [#583](https://github.com/john-cd/rust_howto/issues/583) in [later/src/other/cross-platform/index.md](./later/src/other/cross-platform/index.md#L76)

#### later/src/other/data-processing/csv.md

- [ ] Review issue [#585](https://github.com/john-cd/rust_howto/issues/585) in [later/src/other/data-processing/csv.md](./later/src/other/data-processing/csv.md#L24)

#### later/src/other/data-processing/data_engineering.md

- [ ] Review issue [#589](https://github.com/john-cd/rust_howto/issues/589) in [later/src/other/data-processing/data_engineering.md](./later/src/other/data-processing/data_engineering.md#L65)

#### later/src/other/data-processing/dataframes.md

- [ ] Review issue [#587](https://github.com/john-cd/rust_howto/issues/587) in [later/src/other/data-processing/dataframes.md](./later/src/other/data-processing/dataframes.md#L25)

#### later/src/other/data-processing/index.md

- [ ] Review issue [#594](https://github.com/john-cd/rust_howto/issues/594) in [later/src/other/data-processing/index.md](./later/src/other/data-processing/index.md#L23)

#### later/src/other/devops/cd_ci.md

- [ ] Review issue [#595](https://github.com/john-cd/rust_howto/issues/595) in [later/src/other/devops/cd_ci.md](./later/src/other/devops/cd_ci.md#L47)

#### later/src/other/devops/github_actions.md

- [ ] Review issue [#600](https://github.com/john-cd/rust_howto/issues/600) in [later/src/other/devops/github_actions.md](./later/src/other/devops/github_actions.md#L135)

#### later/src/other/devops/index.md

- [ ] Review issue [#605](https://github.com/john-cd/rust_howto/issues/605) in [later/src/other/devops/index.md](./later/src/other/devops/index.md#L63)

#### later/src/other/devops/release_automation.md

- [ ] Review issue [#604](https://github.com/john-cd/rust_howto/issues/604) in [later/src/other/devops/release_automation.md](./later/src/other/devops/release_automation.md#L94)

#### later/src/other/devops/version_control.md

- [ ] Review issue [#602](https://github.com/john-cd/rust_howto/issues/602) in [later/src/other/devops/version_control.md](./later/src/other/devops/version_control.md#L94)

#### later/src/other/gpu/gpu.md

- [ ] Review issue [#607](https://github.com/john-cd/rust_howto/issues/607) in [later/src/other/gpu/gpu.md](./later/src/other/gpu/gpu.md#L23)

#### later/src/other/gpu/index.md

- [ ] Review issue [#608](https://github.com/john-cd/rust_howto/issues/608) in [later/src/other/gpu/index.md](./later/src/other/gpu/index.md#L29)

#### later/src/other/scripting/index.md

- [ ] Review issue [#991](https://github.com/john-cd/rust_howto/issues/991) in [later/src/other/scripting/index.md](./later/src/other/scripting/index.md#L86)

#### later/src/other/scripting/rhai.md

- [ ] Review issue [#610](https://github.com/john-cd/rust_howto/issues/610) in [later/src/other/scripting/rhai.md](./later/src/other/scripting/rhai.md#L46)

#### later/src/other/scripting/task_automation.md

- [ ] Review issue [#1232](https://github.com/john-cd/rust_howto/issues/1232) in [later/src/other/scripting/task_automation.md](./later/src/other/scripting/task_automation.md#L37)

#### later/src/other/written-in-rust/development_tools.md

- [ ] Review issue [#612](https://github.com/john-cd/rust_howto/issues/612) in [later/src/other/written-in-rust/development_tools.md](./later/src/other/written-in-rust/development_tools.md#L57)

#### later/src/other/written-in-rust/index.md

- [ ] Review issue [#993](https://github.com/john-cd/rust_howto/issues/993) in [later/src/other/written-in-rust/index.md](./later/src/other/written-in-rust/index.md#L150)

#### later/src/other/written-in-rust/other_tools.md

- [ ] Review issue [#615](https://github.com/john-cd/rust_howto/issues/615) in [later/src/other/written-in-rust/other_tools.md](./later/src/other/written-in-rust/other_tools.md#L44)

#### later/src/other/written-in-rust/python_tools.md

- [ ] Review issue [#617](https://github.com/john-cd/rust_howto/issues/617) in [later/src/other/written-in-rust/python_tools.md](./later/src/other/written-in-rust/python_tools.md#L59)

## Examples, Crates, and Code

### bk/crates/cats/asynchronous/Cargo.toml

- [ ] do we need an example? mio = "1.0.2" ([bk/crates/cats/asynchronous/Cargo.toml](./bk/crates/cats/asynchronous/Cargo.toml#L28))
- [ ] need an example smol = "2.0.2" ([bk/crates/cats/asynchronous/Cargo.toml](./bk/crates/cats/asynchronous/Cargo.toml#L34))

### bk/crates/cats/asynchronous/examples/async_channels/postage.rs

- [ ] [finish; polish postage.rs example / add examples for other queues; logging; stream, sink](https://github.com/john-cd/rust_howto/issues/80) ([bk/crates/cats/asynchronous/examples/async_channels/postage.rs](./bk/crates/cats/asynchronous/examples/async_channels/postage.rs#L76))

### bk/crates/cats/asynchronous/examples/streams/streams2.rs

- [ ] [finish; asynchronous/streams.md: add more. streams2.rs is noplayground because it requires a network. rewrite](https://github.com/john-cd/rust_howto/issues/645) ([bk/crates/cats/asynchronous/examples/streams/streams2.rs](./bk/crates/cats/asynchronous/examples/streams/streams2.rs#L72))

### bk/crates/cats/caching/examples/in_memory_cache/cached.rs

- [ ] [review further](https://github.com/john-cd/rust_howto/issues/1354) ([bk/crates/cats/caching/examples/in_memory_cache/cached.rs](./bk/crates/cats/caching/examples/in_memory_cache/cached.rs#L66))

### bk/crates/cats/command_line_interface/Cargo.toml

- [ ] add examlep? clap_complete = "4.5.46" ([bk/crates/cats/command_line_interface/Cargo.toml](./bk/crates/cats/command_line_interface/Cargo.toml#L23))
- [ ] add examle? colored = "3.0.0" ([bk/crates/cats/command_line_interface/Cargo.toml](./bk/crates/cats/command_line_interface/Cargo.toml#L24))

### bk/crates/cats/compression/examples/compression/flate2.rs

- [ ] [read / write a file](https://github.com/john-cd/rust_howto/issues/1009) ([bk/crates/cats/compression/examples/compression/flate2.rs](./bk/crates/cats/compression/examples/compression/flate2.rs#L83))

### bk/crates/cats/compression/examples/tar/tar_compress.rs

- [ ] [review; tar_compress.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/255) ([bk/crates/cats/compression/examples/tar/tar_compress.rs](./bk/crates/cats/compression/examples/tar/tar_compress.rs#L32))

### bk/crates/cats/compression/examples/tar/tar_decompress.rs

- [ ] [review; tar_decompress.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/254) ([bk/crates/cats/compression/examples/tar/tar_decompress.rs](./bk/crates/cats/compression/examples/tar/tar_decompress.rs#L24))

### bk/crates/cats/compression/examples/tar/tar_strip_prefix.rs

- [ ] [review; tar_strip_prefix.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/256) ([bk/crates/cats/compression/examples/tar/tar_strip_prefix.rs](./bk/crates/cats/compression/examples/tar/tar_strip_prefix.rs#L36))

### bk/crates/cats/concurrency/Cargo.toml

- [ ] riker = "0.4.2" ([bk/crates/cats/concurrency/Cargo.toml](./bk/crates/cats/concurrency/Cargo.toml#L43))
- [ ] stakker = "0.2.11" ([bk/crates/cats/concurrency/Cargo.toml](./bk/crates/cats/concurrency/Cargo.toml#L44))

### bk/crates/cats/concurrency/examples/actors/actix.rs

- [ ] [finish; test fails: `spawn_local` called from outside of a `task::LocalSet` or LocalRuntime](https://github.com/john-cd/rust_howto/issues/682) ([bk/crates/cats/concurrency/examples/actors/actix.rs](./bk/crates/cats/concurrency/examples/actors/actix.rs#L63))

### bk/crates/cats/concurrency/examples/actors/actors.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1011) ([bk/crates/cats/concurrency/examples/actors/actors.rs](./bk/crates/cats/concurrency/examples/actors/actors.rs#L32))

### bk/crates/cats/concurrency/examples/actors/stakker.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/94) ([bk/crates/cats/concurrency/examples/actors/stakker.rs](./bk/crates/cats/concurrency/examples/actors/stakker.rs#L79))

### bk/crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs

- [ ] [review https://github.com/jonhoo/flurry/blob/main/benches/flurry_dashmap.rs](https://github.com/john-cd/rust_howto/issues/1152) ([bk/crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs](./bk/crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs#L113))

### bk/crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs

- [ ] [review; rayon_thumbnails: address the need for test jpg data_parallelism: rayon_thumbnails.rs is noplayground - linking with cc](https://github.com/john-cd/rust_howto/issues/261) ([bk/crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs](./bk/crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs#L94))

### bk/crates/cats/concurrency/examples/shared_state/global_mut_state.rs

- [ ] [review lazy_constant.rs and lazy_static.rs and global_mut_state.rs](https://github.com/john-cd/rust_howto/issues/939) ([bk/crates/cats/concurrency/examples/shared_state/global_mut_state.rs](./bk/crates/cats/concurrency/examples/shared_state/global_mut_state.rs#L49))

### bk/crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs

- [ ] [review; threadpool_fractal.rs is noplayground - linking with cc](https://github.com/john-cd/rust_howto/issues/268) ([bk/crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs](./bk/crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs#L130))

### bk/crates/cats/config/examples/configuration/confy.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/148) ([bk/crates/cats/config/examples/configuration/confy.rs](./bk/crates/cats/config/examples/configuration/confy.rs#L37))

### bk/crates/cats/cryptography/Cargo.toml

- [ ] need an example? dsa = "0.6.3" ([bk/crates/cats/cryptography/Cargo.toml](./bk/crates/cats/cryptography/Cargo.toml#L28))
- [ ] ecdsa = "0.16.9" ([bk/crates/cats/cryptography/Cargo.toml](./bk/crates/cats/cryptography/Cargo.toml#L31))
- [ ] native-tls = "0.2.12" ([bk/crates/cats/cryptography/Cargo.toml](./bk/crates/cats/cryptography/Cargo.toml#L32))
- [ ] pem-rfc7468 = "0.7.0" ([bk/crates/cats/cryptography/Cargo.toml](./bk/crates/cats/cryptography/Cargo.toml#L34))
- [ ] rustls = "0.23.20" ([bk/crates/cats/cryptography/Cargo.toml](./bk/crates/cats/cryptography/Cargo.toml#L41))
- [ ] tokio-rustls = "0.26.2" ([bk/crates/cats/cryptography/Cargo.toml](./bk/crates/cats/cryptography/Cargo.toml#L44))
- [ ] x509-cert = "0.2.5" ([bk/crates/cats/cryptography/Cargo.toml](./bk/crates/cats/cryptography/Cargo.toml#L45))

### bk/crates/cats/cryptography/examples/certs/pem_rfc7468.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/697) ([bk/crates/cats/cryptography/examples/certs/pem_rfc7468.rs](./bk/crates/cats/cryptography/examples/certs/pem_rfc7468.rs#L40))

### bk/crates/cats/cryptography/examples/certs/x509_cert.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/705) ([bk/crates/cats/cryptography/examples/certs/x509_cert.rs](./bk/crates/cats/cryptography/examples/certs/x509_cert.rs#L65))

### bk/crates/cats/cryptography/examples/sign/ecdsa.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/693) ([bk/crates/cats/cryptography/examples/sign/ecdsa.rs](./bk/crates/cats/cryptography/examples/sign/ecdsa.rs#L66))

### bk/crates/cats/cryptography/examples/sign/ed25519.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1084) ([bk/crates/cats/cryptography/examples/sign/ed25519.rs](./bk/crates/cats/cryptography/examples/sign/ed25519.rs#L67))

### bk/crates/cats/cryptography/examples/tls/native_tls.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/696) ([bk/crates/cats/cryptography/examples/tls/native_tls.rs](./bk/crates/cats/cryptography/examples/tls/native_tls.rs#L37))

### bk/crates/cats/cryptography/examples/tls/rustls.rs

- [ ] [review tokio-rustls; need full integration test; document further; review https://github.com/rustls/rustls/tree/main/examples](https://github.com/john-cd/rust_howto/issues/700) ([bk/crates/cats/cryptography/examples/tls/rustls.rs](./bk/crates/cats/cryptography/examples/tls/rustls.rs#L79))

### bk/crates/cats/data_structures/Cargo.toml

- [ ] Resolve TODO/FIXME at line 17 ([bk/crates/cats/data_structures/Cargo.toml](./bk/crates/cats/data_structures/Cargo.toml#L17))

### bk/crates/cats/database/Cargo.toml

- [ ] cassandra-protocol = { version = "3.3.0", optional = true } ([bk/crates/cats/database/Cargo.toml](./bk/crates/cats/database/Cargo.toml#L38))
- [ ] cdrs-tokio = { version = "8.1.4", features = ["derive"], optional = true } ([bk/crates/cats/database/Cargo.toml](./bk/crates/cats/database/Cargo.toml#L39))
- [ ] cornucopia = { version = "0.9.0", optional = true } ([bk/crates/cats/database/Cargo.toml](./bk/crates/cats/database/Cargo.toml#L40))
- [ ] diesel_migrations = { version = "2.2.0", features = ["sqlite"], optional = true } ([bk/crates/cats/database/Cargo.toml](./bk/crates/cats/database/Cargo.toml#L44))
- [ ] sea-orm = { version = "1.1.3", optional = true } ([bk/crates/cats/database/Cargo.toml](./bk/crates/cats/database/Cargo.toml#L59))
- [ ] seaography = { version = "1.1.2", optional = true } ([bk/crates/cats/database/Cargo.toml](./bk/crates/cats/database/Cargo.toml#L60))
- [ ] uuid = { version = "1", features = ["v4"] } ([bk/crates/cats/database/Cargo.toml](./bk/crates/cats/database/Cargo.toml#L67))

### bk/crates/cats/database/examples/cassandra/cassandra_protocol.rs

- [ ] [write; see also https://docs.rs/cassandra-protocol/latest/cassandra_protocol/index.html](https://github.com/john-cd/rust_howto/issues/1016) ([bk/crates/cats/database/examples/cassandra/cassandra_protocol.rs](./bk/crates/cats/database/examples/cassandra/cassandra_protocol.rs#L103))

### bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs

- [ ] [finish; see also https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/crud_operations.rs](https://github.com/john-cd/rust_howto/issues/1017) ([bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs](./bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs#L125))

### bk/crates/cats/database/examples/connection_pool/deadpool2.rs

- [ ] [finish; need heavy test](https://github.com/john-cd/rust_howto/issues/46) ([bk/crates/cats/database/examples/connection_pool/deadpool2.rs](./bk/crates/cats/database/examples/connection_pool/deadpool2.rs#L73))

### bk/crates/cats/database/examples/mssql/tiberius.rs

- [ ] [fix heavy test](https://github.com/john-cd/rust_howto/issues/1019) ([bk/crates/cats/database/examples/mssql/tiberius.rs](./bk/crates/cats/database/examples/mssql/tiberius.rs#L79))

### bk/crates/cats/database/examples/nosql/redis.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/1161)? ([bk/crates/cats/database/examples/nosql/redis.rs](./bk/crates/cats/database/examples/nosql/redis.rs#L71))

### bk/crates/cats/database/examples/oracle/diesel_oci.rs

- [ ] [finish; debug: Issue: Cannot locate a 64-bit Oracle Client library; need heavy test](https://github.com/john-cd/rust_howto/issues/1020) ([bk/crates/cats/database/examples/oracle/diesel_oci.rs](./bk/crates/cats/database/examples/oracle/diesel_oci.rs#L114))

### bk/crates/cats/database/examples/oracle/oracle.rs

- [ ] [finish; need to fix heavy test](https://github.com/john-cd/rust_howto/issues/1021) ([bk/crates/cats/database/examples/oracle/oracle.rs](./bk/crates/cats/database/examples/oracle/oracle.rs#L73))

### bk/crates/cats/database/examples/oracle/sibyl.rs

- [ ] [finish; review https://lib.rs/crates/sibyl ; also test / review in depth install steps](https://github.com/john-cd/rust_howto/issues/1022) ([bk/crates/cats/database/examples/oracle/sibyl.rs](./bk/crates/cats/database/examples/oracle/sibyl.rs#L88))

### bk/crates/cats/database/examples/postgres/aggregate_data.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/1162) ([bk/crates/cats/database/examples/postgres/aggregate_data.rs](./bk/crates/cats/database/examples/postgres/aggregate_data.rs#L43))

### bk/crates/cats/database/examples/postgres/cornucopia.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/708) ([bk/crates/cats/database/examples/postgres/cornucopia.rs](./bk/crates/cats/database/examples/postgres/cornucopia.rs#L54))

### bk/crates/cats/database/examples/postgres/main.rs

- [ ] aggregate_data::main()? ([bk/crates/cats/database/examples/postgres/main.rs](./bk/crates/cats/database/examples/postgres/main.rs#L21))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/713) ([bk/crates/cats/database/examples/postgres/main.rs](./bk/crates/cats/database/examples/postgres/main.rs#L40))

### bk/crates/cats/database/examples/postgres/tokio_postgres.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/719) need heay test ([bk/crates/cats/database/examples/postgres/tokio_postgres.rs](./bk/crates/cats/database/examples/postgres/tokio_postgres.rs#L99))

### bk/crates/cats/database/examples/query_builders_orms/diesel1.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/709) ([bk/crates/cats/database/examples/query_builders_orms/diesel1.rs](./bk/crates/cats/database/examples/query_builders_orms/diesel1.rs#L87))

### bk/crates/cats/database/examples/query_builders_orms/sea_orm.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/715) ([bk/crates/cats/database/examples/query_builders_orms/sea_orm.rs](./bk/crates/cats/database/examples/query_builders_orms/sea_orm.rs#L53))

### bk/crates/cats/database/examples/search/elasticsearch.rs

- [ ] [review fix heavy test; secure the connection](https://github.com/john-cd/rust_howto/issues/710) ([bk/crates/cats/database/examples/search/elasticsearch.rs](./bk/crates/cats/database/examples/search/elasticsearch.rs#L156))

### bk/crates/cats/database_implementations/examples/databases/surrealdb.rs

- [ ] [review https://surrealdb.com/docs](https://github.com/john-cd/rust_howto/issues/1148) ([bk/crates/cats/database_implementations/examples/databases/surrealdb.rs](./bk/crates/cats/database_implementations/examples/databases/surrealdb.rs#L97))

### bk/crates/cats/date_and_time/Cargo.toml

- [ ] add example  humantime = "2.1.0" ([bk/crates/cats/date_and_time/Cargo.toml](./bk/crates/cats/date_and_time/Cargo.toml#L19))

### bk/crates/cats/development_tools/Cargo.toml

- [ ] add example doc-comment = "0.3.3" ([bk/crates/cats/development_tools/Cargo.toml](./bk/crates/cats/development_tools/Cargo.toml#L19))

### bk/crates/cats/development_tools_build_utils/build.rs

- [ ] [fix](https://github.com/john-cd/rust_howto/issues/998) ([bk/crates/cats/development_tools_build_utils/build.rs](./bk/crates/cats/development_tools_build_utils/build.rs#L63))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs

- [ ] [FIX finish; deal with cc](https://github.com/john-cd/rust_howto/issues/897) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs#L30))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/898) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs#L29))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs

- [ ] [finish; deal with cc](https://github.com/john-cd/rust_howto/issues/899) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs#L26))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs

- [ ] [finish; deal with extern](https://github.com/john-cd/rust_howto/issues/900) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs#L40))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs

- [ ] [finish; deal wth cc](https://github.com/john-cd/rust_howto/issues/1000) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs#L26))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs

- [ ] [finish; deal with extern](https://github.com/john-cd/rust_howto/issues/901) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs#L23))

### bk/crates/cats/development_tools_debugging/Cargo.toml

- [ ] Resolve TODO/FIXME at line 27 ([bk/crates/cats/development_tools_debugging/Cargo.toml](./bk/crates/cats/development_tools_debugging/Cargo.toml#L27))
- [ ] tracing-opentelemetry = { version = "0.31.0", optional = true } ([bk/crates/cats/development_tools_debugging/Cargo.toml](./bk/crates/cats/development_tools_debugging/Cargo.toml#L41))
- [ ] tonic = "0.13.0" # for open_observe example ([bk/crates/cats/development_tools_debugging/Cargo.toml](./bk/crates/cats/development_tools_debugging/Cargo.toml#L43))
- [ ] Resolve TODO/FIXME at line 53 ([bk/crates/cats/development_tools_debugging/Cargo.toml](./bk/crates/cats/development_tools_debugging/Cargo.toml#L53))

### bk/crates/cats/development_tools_debugging/examples/log/log_env_variable.rs

- [ ] [review; test fully and review vs text](https://github.com/john-cd/rust_howto/issues/157) ([bk/crates/cats/development_tools_debugging/examples/log/log_env_variable.rs](./bk/crates/cats/development_tools_debugging/examples/log/log_env_variable.rs#L70))

### bk/crates/cats/development_tools_debugging/examples/other/open_observe.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/733) ([bk/crates/cats/development_tools_debugging/examples/other/open_observe.rs](./bk/crates/cats/development_tools_debugging/examples/other/open_observe.rs#L234))

### bk/crates/cats/development_tools_debugging/examples/other/open_telemetry.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/734) ([bk/crates/cats/development_tools_debugging/examples/other/open_telemetry.rs](./bk/crates/cats/development_tools_debugging/examples/other/open_telemetry.rs#L69))

### bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber4.rs

- [ ] cover same functions on fmt() ([bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber4.rs](./bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber4.rs#L50))

### bk/crates/cats/development_tools_profiling/Cargo.toml

- [ ] Resolve TODO/FIXME at line 29 ([bk/crates/cats/development_tools_profiling/Cargo.toml](./bk/crates/cats/development_tools_profiling/Cargo.toml#L29))

### bk/crates/cats/development_tools_profiling/benches/divan.rs

- [ ] [finish. add to md: https://nikolaivazquez.com/blog/divan/](https://github.com/john-cd/rust_howto/issues/747) cover more attributes; cover blockbox / bencher ([bk/crates/cats/development_tools_profiling/benches/divan.rs](./bk/crates/cats/development_tools_profiling/benches/divan.rs#L52))

### bk/crates/cats/development_tools_profiling/src/memory_usage_analysis/dhat.rs

- [ ] automate: cargo run --features dhat-heap / cargo run --features ([bk/crates/cats/development_tools_profiling/src/memory_usage_analysis/dhat.rs](./bk/crates/cats/development_tools_profiling/src/memory_usage_analysis/dhat.rs#L121))

### bk/crates/cats/development_tools_testing/Cargo.toml

- [ ] afl = "0.15.13" ([bk/crates/cats/development_tools_testing/Cargo.toml](./bk/crates/cats/development_tools_testing/Cargo.toml#L18))

### bk/crates/cats/development_tools_testing/examples/fuzzing/afl.rs

- [ ] [Implement AFL.rs fuzzing target and panic discovery](https://github.com/john-cd/rust_howto/issues/748) ([bk/crates/cats/development_tools_testing/examples/fuzzing/afl.rs](./bk/crates/cats/development_tools_testing/examples/fuzzing/afl.rs#L53))

### bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs

- [ ] [review https://docs.rs/fake/4.2.0/fake/index.html](https://github.com/john-cd/rust_howto/issues/1124) ([bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs](./bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs#L138))

### bk/crates/cats/email/examples/lettre.rs

- [ ] [review; Requires valid SMTP credentials to run](https://github.com/john-cd/rust_howto/issues/1144) ([bk/crates/cats/email/examples/lettre.rs](./bk/crates/cats/email/examples/lettre.rs#L45))

### bk/crates/cats/encoding/Cargo.toml

- [ ] bincode = "2.0.1" ([bk/crates/cats/encoding/Cargo.toml](./bk/crates/cats/encoding/Cargo.toml#L24))
- [ ] capnp = "0.21.0" ([bk/crates/cats/encoding/Cargo.toml](./bk/crates/cats/encoding/Cargo.toml#L27))
- [ ] flatbuffers = "25.1.21" ([bk/crates/cats/encoding/Cargo.toml](./bk/crates/cats/encoding/Cargo.toml#L32))
- [ ] form_urlencoded = "1.2.1" ([bk/crates/cats/encoding/Cargo.toml](./bk/crates/cats/encoding/Cargo.toml#L33))
- [ ] protobuf = "4.31.1-release" ([bk/crates/cats/encoding/Cargo.toml](./bk/crates/cats/encoding/Cargo.toml#L37))
- [ ] rmp-serde = "1.3.0" ([bk/crates/cats/encoding/Cargo.toml](./bk/crates/cats/encoding/Cargo.toml#L43))

### bk/crates/cats/encoding/build.rs

- [ ] [fix prost](https://github.com/john-cd/rust_howto/issues/1417) ([bk/crates/cats/encoding/build.rs](./bk/crates/cats/encoding/build.rs#L30))

### bk/crates/cats/encoding/examples/binary_encoders/bincode.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1040) ([bk/crates/cats/encoding/examples/binary_encoders/bincode.rs](./bk/crates/cats/encoding/examples/binary_encoders/bincode.rs#L56))

### bk/crates/cats/encoding/examples/binary_encoders/capnp.rs

- [ ] [finish; see https://capnproto.org/ https://capnproto.org/rust.html https://capnproto.org/install.html](https://github.com/john-cd/rust_howto/issues/1041) ([bk/crates/cats/encoding/examples/binary_encoders/capnp.rs](./bk/crates/cats/encoding/examples/binary_encoders/capnp.rs#L69))

### bk/crates/cats/encoding/examples/binary_encoders/flatbuffers.rs

- [ ] [finish; https://flatbuffers.dev/languages/rust/](https://github.com/john-cd/rust_howto/issues/1043) ([bk/crates/cats/encoding/examples/binary_encoders/flatbuffers.rs](./bk/crates/cats/encoding/examples/binary_encoders/flatbuffers.rs#L68))

### bk/crates/cats/encoding/examples/binary_encoders/main.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/1234) ([bk/crates/cats/encoding/examples/binary_encoders/main.rs](./bk/crates/cats/encoding/examples/binary_encoders/main.rs#L2))

### bk/crates/cats/encoding/examples/binary_encoders/prost.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1044) ([bk/crates/cats/encoding/examples/binary_encoders/prost.rs](./bk/crates/cats/encoding/examples/binary_encoders/prost.rs#L38))

### bk/crates/cats/encoding/examples/binary_encoders/protobuf.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1045) ([bk/crates/cats/encoding/examples/binary_encoders/protobuf.rs](./bk/crates/cats/encoding/examples/binary_encoders/protobuf.rs#L54))

### bk/crates/cats/encoding/examples/binary_encoders/rmp_serde.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1046) ([bk/crates/cats/encoding/examples/binary_encoders/rmp_serde.rs](./bk/crates/cats/encoding/examples/binary_encoders/rmp_serde.rs#L58))

### bk/crates/cats/encoding/examples/string_encoding/percent_encoding.rs

- [ ] [dedupe with other example in percent_encode](https://github.com/john-cd/rust_howto/issues/1353) ([bk/crates/cats/encoding/examples/string_encoding/percent_encoding.rs](./bk/crates/cats/encoding/examples/string_encoding/percent_encoding.rs#L111))

### bk/crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/1350) ([bk/crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs](./bk/crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs#L49))

### bk/crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs

- [ ] [review lazy_constant.rs and lazy_static.rs and global_mut_state.rs](https://github.com/john-cd/rust_howto/issues/939) ([bk/crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs](./bk/crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs#L36))

### bk/crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs

- [ ] [review lazy_constant.rs and lazy_static.rs and global_mut_state.rs](https://github.com/john-cd/rust_howto/issues/939) ([bk/crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs](./bk/crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs#L89))

### bk/crates/cats/network_programming/Cargo.toml

- [ ] ngrok = { version = "0.15.0", features = [ "axum" ] } ([bk/crates/cats/network_programming/Cargo.toml](./bk/crates/cats/network_programming/Cargo.toml#L22))

### bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/811) ([bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs](./bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs#L83))

### bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/812) ([bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs](./bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs#L79))

### bk/crates/cats/network_programming/examples/server/glommio.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/810) ([bk/crates/cats/network_programming/examples/server/glommio.rs](./bk/crates/cats/network_programming/examples/server/glommio.rs#L104))

### bk/crates/cats/network_programming/examples/server/listen_unused.rs

- [ ] [finish; listens to a connection forever](https://github.com/john-cd/rust_howto/issues/166) ([bk/crates/cats/network_programming/examples/server/listen_unused.rs](./bk/crates/cats/network_programming/examples/server/listen_unused.rs#L42))

### bk/crates/cats/os_windows_apis/Cargo.toml

- [ ] windows = { version = "0.61.1" } ([bk/crates/cats/os_windows_apis/Cargo.toml](./bk/crates/cats/os_windows_apis/Cargo.toml#L21))

### bk/crates/cats/os_windows_apis/examples/windows/winapi.rs

- [ ] [review / test](https://github.com/john-cd/rust_howto/issues/822) ([bk/crates/cats/os_windows_apis/examples/windows/winapi.rs](./bk/crates/cats/os_windows_apis/examples/windows/winapi.rs#L132))

### bk/crates/cats/os_windows_apis/examples/windows/windows.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/823) ([bk/crates/cats/os_windows_apis/examples/windows/windows.rs](./bk/crates/cats/os_windows_apis/examples/windows/windows.rs#L43))

### bk/crates/cats/parser_implementations/Cargo.toml

- [ ] xml5ever = "0.23.0" ([bk/crates/cats/parser_implementations/Cargo.toml](./bk/crates/cats/parser_implementations/Cargo.toml#L24))
- [ ] xmlparser = "0.13.6" ([bk/crates/cats/parser_implementations/Cargo.toml](./bk/crates/cats/parser_implementations/Cargo.toml#L25))
- [ ] cssparser = "0.35.0" ([bk/crates/cats/parser_implementations/Cargo.toml](./bk/crates/cats/parser_implementations/Cargo.toml#L30))
- [ ] html5ever = "0.33.0" ([bk/crates/cats/parser_implementations/Cargo.toml](./bk/crates/cats/parser_implementations/Cargo.toml#L31))
- [ ] markup5ever = "0.16.0" ([bk/crates/cats/parser_implementations/Cargo.toml](./bk/crates/cats/parser_implementations/Cargo.toml#L34))
- [ ] markup5ever_arcdom = "0.1.2" ([bk/crates/cats/parser_implementations/Cargo.toml](./bk/crates/cats/parser_implementations/Cargo.toml#L35))
- [ ] quick-xml = {version = "0.37.2", features = ["serialize"] } ([bk/crates/cats/parser_implementations/Cargo.toml](./bk/crates/cats/parser_implementations/Cargo.toml#L37))

### bk/crates/cats/parser_implementations/examples/html/cssparser.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1092) ([bk/crates/cats/parser_implementations/examples/html/cssparser.rs](./bk/crates/cats/parser_implementations/examples/html/cssparser.rs#L112))

### bk/crates/cats/parser_implementations/examples/html/html5ever.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1090) ([bk/crates/cats/parser_implementations/examples/html/html5ever.rs](./bk/crates/cats/parser_implementations/examples/html/html5ever.rs#L64))

### bk/crates/cats/parser_implementations/examples/xml/quick_xml.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1236) ([bk/crates/cats/parser_implementations/examples/xml/quick_xml.rs](./bk/crates/cats/parser_implementations/examples/xml/quick_xml.rs#L156))

### bk/crates/cats/parser_implementations/examples/xml/xml.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1100) ([bk/crates/cats/parser_implementations/examples/xml/xml.rs](./bk/crates/cats/parser_implementations/examples/xml/xml.rs#L104))

### bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1103) ([bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs](./bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs#L322))

### bk/crates/cats/parser_implementations/examples/xml/xmlparser.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1102) ([bk/crates/cats/parser_implementations/examples/xml/xmlparser.rs](./bk/crates/cats/parser_implementations/examples/xml/xmlparser.rs#L101))

### bk/crates/cats/parsing/Cargo.toml

- [ ] tree-sitter = "0.25.1" ([bk/crates/cats/parsing/Cargo.toml](./bk/crates/cats/parsing/Cargo.toml#L26))

### bk/crates/cats/parsing/examples/parsing/tree_sitter.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/827) ([bk/crates/cats/parsing/examples/parsing/tree_sitter.rs](./bk/crates/cats/parsing/examples/parsing/tree_sitter.rs#L162))

### bk/crates/cats/parsing/examples/pest/pest.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/826) ([bk/crates/cats/parsing/examples/pest/pest.rs](./bk/crates/cats/parsing/examples/pest/pest.rs#L57))

### bk/crates/cats/rust_patterns/examples/functional_programming/either.rs

- [ ] [finish example](https://github.com/john-cd/rust_howto/issues/1317) ([bk/crates/cats/rust_patterns/examples/functional_programming/either.rs](./bk/crates/cats/rust_patterns/examples/functional_programming/either.rs#L46))

### bk/crates/cats/rust_patterns/examples/functional_programming/frunk.rs

- [ ] [finish example](https://github.com/john-cd/rust_howto/issues/1318) ([bk/crates/cats/rust_patterns/examples/functional_programming/frunk.rs](./bk/crates/cats/rust_patterns/examples/functional_programming/frunk.rs#L172))

### bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs

- [ ] [finish; provide projection example before going into pin projection; explain risks; explain use cases](https://github.com/john-cd/rust_howto/issues/1120) ([bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs](./bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs#L227))

### bk/crates/cats/web_programming/Cargo.toml

- [ ] http-body-util = "0.1.2" ([bk/crates/cats/web_programming/Cargo.toml](./bk/crates/cats/web_programming/Cargo.toml#L26))

### bk/crates/cats/web_programming/examples/http_types_and_interfaces/http.rs

- [ ] [expand; HTTP header interpretation and generation.  LATER](https://github.com/john-cd/rust_howto/issues/1355) ([bk/crates/cats/web_programming/examples/http_types_and_interfaces/http.rs](./bk/crates/cats/web_programming/examples/http_types_and_interfaces/http.rs#L44))

### bk/crates/cats/web_programming/examples/scraping/broken.rs

- [ ] [flaky test](https://github.com/john-cd/rust_howto/issues/1419) ([bk/crates/cats/web_programming/examples/scraping/broken.rs](./bk/crates/cats/web_programming/examples/scraping/broken.rs#L78))

### bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs

- [ ] Resolve TODO/FIXME at line 9 ([bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs](./bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs#L9))
- [ ] [fix - the API no longer returns a crate_id - need to get](https://github.com/john-cd/rust_howto/issues/860) ([bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs](./bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs#L147))

### bk/crates/cats/web_programming_http_client/examples/apis/rate_limited.rs

- [ ] [fix interaction with https://docs.github.com/en/rest?apiVersion=2022-11-28](https://github.com/john-cd/rust_howto/issues/177) ([bk/crates/cats/web_programming_http_client/examples/apis/rate_limited.rs](./bk/crates/cats/web_programming_http_client/examples/apis/rate_limited.rs#L89))

### bk/crates/cats/web_programming_http_client/examples/apis/rest_post.rs

- [ ] [review; rewrite so that a username and password are not required?](https://github.com/john-cd/rust_howto/issues/178) ([bk/crates/cats/web_programming_http_client/examples/apis/rest_post.rs](./bk/crates/cats/web_programming_http_client/examples/apis/rest_post.rs#L89))

### bk/crates/cats/web_programming_http_client/examples/download/download.rs

- [ ] [review; noplayground because of network use. rewrite?](https://github.com/john-cd/rust_howto/issues/225) ([bk/crates/cats/web_programming_http_client/examples/download/download.rs](./bk/crates/cats/web_programming_http_client/examples/download/download.rs#L45))

### bk/crates/cats/web_programming_http_client/examples/download/partial.rs

- [ ] [review; flaky test](https://github.com/john-cd/rust_howto/issues/176) ([bk/crates/cats/web_programming_http_client/examples/download/partial.rs](./bk/crates/cats/web_programming_http_client/examples/download/partial.rs#L120))

### bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/859) ([bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs](./bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs#L94))

### bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs

- [ ] [write more?](https://github.com/john-cd/rust_howto/issues/862) ([bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs](./bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs#L28))

### bk/crates/cats/web_programming_http_server/Cargo.toml

- [ ] [trillium: do we need LATER](https://github.com/john-cd/rust_howto/issues/1314) ([bk/crates/cats/web_programming_http_server/Cargo.toml](./bk/crates/cats/web_programming_http_server/Cargo.toml#L26))
- [ ] actix-web = "4.9.0" ([bk/crates/cats/web_programming_http_server/Cargo.toml](./bk/crates/cats/web_programming_http_server/Cargo.toml#L28))
- [ ] async-graphql = { version = "7.0.17", optional = true } ([bk/crates/cats/web_programming_http_server/Cargo.toml](./bk/crates/cats/web_programming_http_server/Cargo.toml#L29))
- [ ] hyper = { version = "1.6.0", features = ["full"] } ([bk/crates/cats/web_programming_http_server/Cargo.toml](./bk/crates/cats/web_programming_http_server/Cargo.toml#L35))
- [ ] leptos = "0.8.2" ([bk/crates/cats/web_programming_http_server/Cargo.toml](./bk/crates/cats/web_programming_http_server/Cargo.toml#L37))

### bk/crates/cats/web_programming_http_server/examples/actix_web.rs

- [ ] Resolve TODO/FIXME at line 36 ([bk/crates/cats/web_programming_http_server/examples/actix_web.rs](./bk/crates/cats/web_programming_http_server/examples/actix_web.rs#L36))

### bk/crates/cats/web_programming_http_server/examples/async_graphql.rs

- [ ] [finish;  https://github.com/async-graphql/examples](https://github.com/john-cd/rust_howto/issues/864) ([bk/crates/cats/web_programming_http_server/examples/async_graphql.rs](./bk/crates/cats/web_programming_http_server/examples/async_graphql.rs#L74))

### bk/crates/cats/web_programming_http_server/examples/axum.rs

- [ ] [review time limit](https://github.com/john-cd/rust_howto/issues/865) ([bk/crates/cats/web_programming_http_server/examples/axum.rs](./bk/crates/cats/web_programming_http_server/examples/axum.rs#L122))

### bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs

- [ ] [Web] Implement Tonic gRPC example and full integration tests (https://github.com/john-cd/rust_howto/issues/870) - COMPLETED ([bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs](./bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs#L87))

### bk/crates/cats/web_programming_http_server/examples/hyper_server.rs

- [ ] [Update Hyper server example to 1.0 API and implement request routing](https://github.com/john-cd/rust_howto/issues/866) ([bk/crates/cats/web_programming_http_server/examples/hyper_server.rs](./bk/crates/cats/web_programming_http_server/examples/hyper_server.rs#L75))

### bk/crates/cats/web_programming_http_server/examples/leptos.rs

- [ ] Resolve TODO/FIXME at line 57 ([bk/crates/cats/web_programming_http_server/examples/leptos.rs](./bk/crates/cats/web_programming_http_server/examples/leptos.rs#L57))
- [ ] [Implement Leptos reactive counter and view mounting](https://github.com/john-cd/rust_howto/issues/867) ([bk/crates/cats/web_programming_http_server/examples/leptos.rs](./bk/crates/cats/web_programming_http_server/examples/leptos.rs#L66))

### bk/crates/cats/web_programming_http_server/examples/loco/main.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/868) ([bk/crates/cats/web_programming_http_server/examples/loco/main.rs](./bk/crates/cats/web_programming_http_server/examples/loco/main.rs#L88))

### bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/871) ([bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs](./bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs#L95))

### bk/crates/cats/web_programming_http_server/examples/rocket.rs

- [ ] [Implement Rocket Hello World and route mounting](https://github.com/john-cd/rust_howto/issues/869) ([bk/crates/cats/web_programming_http_server/examples/rocket.rs](./bk/crates/cats/web_programming_http_server/examples/rocket.rs#L21))

### later/crates/cats/accessibility/Cargo.toml

- [ ] Resolve TODO/FIXME at line 20 ([later/crates/cats/accessibility/Cargo.toml](./later/crates/cats/accessibility/Cargo.toml#L20))

### later/crates/cats/aerospace_simulation/Cargo.toml

- [ ] Resolve TODO/FIXME at line 20 ([later/crates/cats/aerospace_simulation/Cargo.toml](./later/crates/cats/aerospace_simulation/Cargo.toml#L20))

### later/crates/cats/aerospace_space_protocols/examples/space_protocols/space_protocols.rs

- [ ] Resolve TODO/FIXME at line 19 ([later/crates/cats/aerospace_space_protocols/examples/space_protocols/space_protocols.rs](./later/crates/cats/aerospace_space_protocols/examples/space_protocols/space_protocols.rs#L19))

### later/crates/cats/computer_vision/examples/opencv/opencv.rs

- [ ] [review; expand example; review https://blog.devgenius.io/rust-and-opencv-bb0467bf35ff](https://github.com/john-cd/rust_howto/issues/1079) ([later/crates/cats/computer_vision/examples/opencv/opencv.rs](./later/crates/cats/computer_vision/examples/opencv/opencv.rs#L60))

### later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/707) ([later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs](./later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs#L12))

### later/crates/cats/development_tools_ffi/Cargo.toml

- [ ] [fix categories in all Cargo.toml](https://github.com/john-cd/rust_howto/issues/1307) ([later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L14))
- [ ] Resolve TODO/FIXME at line 18 ([later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L18))
- [ ] Resolve TODO/FIXME at line 39 ([later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L39))
- [ ] erlang = [] # "dep:rustler" ([later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L48))
- [ ] flutter = [] # "dep:flutter_rust_bridge" ([later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L49))
- [ ] java = [] # "dep:jni" ([later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L50))
- [ ] objc = [] # "dep:objc2", "dep:objc2-foundation", "dep:objc2-app-kit" ([later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L52))
- [ ] node = [] # "dep:napi", "dep:neon" ([later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L53))
- [ ] ruby = [] # "dep:magnus", "dep:rutie" ([later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L54))

### later/crates/cats/development_tools_ffi/build.rs

- [ ] [fix build.rs for ffi](https://github.com/john-cd/rust_howto/issues/1026) ([later/crates/cats/development_tools_ffi/build.rs](./later/crates/cats/development_tools_ffi/build.rs#L80))

### later/crates/cats/development_tools_ffi/examples/c/bindgen.rs

- [ ] [fix](https://github.com/john-cd/rust_howto/issues/1001) ([later/crates/cats/development_tools_ffi/examples/c/bindgen.rs](./later/crates/cats/development_tools_ffi/examples/c/bindgen.rs#L53))

### later/crates/cats/development_tools_ffi/examples/c/cbindgen.rs

- [ ] [fix; see build.rs](https://github.com/john-cd/rust_howto/issues/1002) ([later/crates/cats/development_tools_ffi/examples/c/cbindgen.rs](./later/crates/cats/development_tools_ffi/examples/c/cbindgen.rs#L48))

### later/crates/cats/development_tools_ffi/examples/cpp/cxx.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/738) ([later/crates/cats/development_tools_ffi/examples/cpp/cxx.rs](./later/crates/cats/development_tools_ffi/examples/cpp/cxx.rs#L33))

### later/crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs

- [ ] [review; how to test](https://github.com/john-cd/rust_howto/issues/1080) ([later/crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs](./later/crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs#L32))

### later/crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs

- [ ] [finish; reorg as a project using flutter_rust_bridge_codegen](https://github.com/john-cd/rust_howto/issues/1028) ([later/crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs](./later/crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs#L35))

### later/crates/cats/development_tools_ffi/examples/java/jni.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1029) ([later/crates/cats/development_tools_ffi/examples/java/jni.rs](./later/crates/cats/development_tools_ffi/examples/java/jni.rs#L72))

### later/crates/cats/development_tools_ffi/examples/lua/mlua2.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1031) ([later/crates/cats/development_tools_ffi/examples/lua/mlua2.rs](./later/crates/cats/development_tools_ffi/examples/lua/mlua2.rs#L55))

### later/crates/cats/development_tools_ffi/examples/node/napi.rs

- [ ] [finish; https://lib.rs/crates/napi; https://github.com/napi-rs/package-template](https://github.com/john-cd/rust_howto/issues/1032) ([later/crates/cats/development_tools_ffi/examples/node/napi.rs](./later/crates/cats/development_tools_ffi/examples/node/napi.rs#L47))

### later/crates/cats/development_tools_ffi/examples/node/neon.rs

- [ ] [finish how to test](https://github.com/john-cd/rust_howto/issues/1033)? ([later/crates/cats/development_tools_ffi/examples/node/neon.rs](./later/crates/cats/development_tools_ffi/examples/node/neon.rs#L31))

### later/crates/cats/development_tools_ffi/examples/objc/objc2.rs

- [ ] [finish; fix; https://docs.rs/objc2/latest/objc2/](https://github.com/john-cd/rust_howto/issues/1034) ([later/crates/cats/development_tools_ffi/examples/objc/objc2.rs](./later/crates/cats/development_tools_ffi/examples/objc/objc2.rs#L34))

### later/crates/cats/development_tools_ffi/examples/python/pyo3.rs

- [ ] [finish; fix py examples](https://github.com/john-cd/rust_howto/issues/78) ([later/crates/cats/development_tools_ffi/examples/python/pyo3.rs](./later/crates/cats/development_tools_ffi/examples/python/pyo3.rs#L73))

### later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs

- [ ] [finish; fix](https://github.com/john-cd/rust_howto/issues/996) ([later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs](./later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs#L64))

### later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1035) ([later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs](./later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs#L42))

### later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1036) ([later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs](./later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs#L41))

### later/crates/cats/development_tools_ffi/examples/uniffi.rs

- [ ] [finish; https://github.com/mozilla/uniffi-rs/blob/main/examples/arithmetic/Cargo.toml](https://github.com/john-cd/rust_howto/issues/1037) ([later/crates/cats/development_tools_ffi/examples/uniffi.rs](./later/crates/cats/development_tools_ffi/examples/uniffi.rs#L25))

### later/crates/cats/development_tools_procedural_macro_helpers/Cargo.toml

- [ ] darling = "0.20.10" ([later/crates/cats/development_tools_procedural_macro_helpers/Cargo.toml](./later/crates/cats/development_tools_procedural_macro_helpers/Cargo.toml#L17))
- [ ] paste = "1.0.15" ([later/crates/cats/development_tools_procedural_macro_helpers/Cargo.toml](./later/crates/cats/development_tools_procedural_macro_helpers/Cargo.toml#L18))
- [ ] proc-macro2 = "1.0.92" ([later/crates/cats/development_tools_procedural_macro_helpers/Cargo.toml](./later/crates/cats/development_tools_procedural_macro_helpers/Cargo.toml#L19))
- [ ] watt = "0.5.0" ([later/crates/cats/development_tools_procedural_macro_helpers/Cargo.toml](./later/crates/cats/development_tools_procedural_macro_helpers/Cargo.toml#L22))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/744) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs#L60))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/739) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs#L72))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/main.rs

- [ ] [finish; review the following; decide what examples are needed](https://github.com/john-cd/rust_howto/issues/1158) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/main.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/main.rs#L26))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/741) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs#L58))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs

- [ ] [review / expand](https://github.com/john-cd/rust_howto/issues/742) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs#L31))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs

- [ ] Resolve TODO/FIXME at line 24 ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs#L24))
- [ ] [finish. See https://github.com/dtolnay/syn/tree/master/examples](https://github.com/john-cd/rust_howto/issues/743) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs#L79))

### later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs

- [ ] write; review https://docs.rs/syn/latest/syn/index.html ([later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs](./later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs#L29))
- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1157) ([later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs](./later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs#L61))

### later/crates/cats/embedded/examples/embassy/embassy.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/751) ([later/crates/cats/embedded/examples/embassy/embassy.rs](./later/crates/cats/embedded/examples/embassy/embassy.rs#L89))

### later/crates/cats/emulators/examples/emulators/emulator.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/752) ([later/crates/cats/emulators/examples/emulators/emulator.rs](./later/crates/cats/emulators/examples/emulators/emulator.rs#L12))

### later/crates/cats/finance/Cargo.toml

- [ ] [RustQuant: heavy build - polars, nalgebra, etc LATER](https://github.com/john-cd/rust_howto/issues/1313) ([later/crates/cats/finance/Cargo.toml](./later/crates/cats/finance/Cargo.toml#L19))

### later/crates/cats/finance/examples/quant/rustquant.rs

- [ ] [finish; review](https://github.com/john-cd/rust_howto/issues/764) ([later/crates/cats/finance/examples/quant/rustquant.rs](./later/crates/cats/finance/examples/quant/rustquant.rs#L53))

### later/crates/cats/game_development/examples/game_development/game_development1.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/766) ([later/crates/cats/game_development/examples/game_development/game_development1.rs](./later/crates/cats/game_development/examples/game_development/game_development1.rs#L12))

### later/crates/cats/game_development/examples/game_development/glam.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/770) ([later/crates/cats/game_development/examples/game_development/glam.rs](./later/crates/cats/game_development/examples/game_development/glam.rs#L47))

### later/crates/cats/game_engines/Cargo.toml

- [ ] bevy = { version = "0.16.0", optional = true } ([later/crates/cats/game_engines/Cargo.toml](./later/crates/cats/game_engines/Cargo.toml#L18))
- [ ] fyrox = { version = "0.36.0", optional = true } ([later/crates/cats/game_engines/Cargo.toml](./later/crates/cats/game_engines/Cargo.toml#L19))
- [ ] ggez = { version = "0.9.3", optional = true } # ggez conflict with wgpu ([later/crates/cats/game_engines/Cargo.toml](./later/crates/cats/game_engines/Cargo.toml#L21))
- [ ] Resolve TODO/FIXME at line 24 ([later/crates/cats/game_engines/Cargo.toml](./later/crates/cats/game_engines/Cargo.toml#L24))

### later/crates/cats/game_engines/examples/game_engines/bevy.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/767) ([later/crates/cats/game_engines/examples/game_engines/bevy.rs](./later/crates/cats/game_engines/examples/game_engines/bevy.rs#L51))

### later/crates/cats/game_engines/examples/game_engines/fyrox.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/768) ([later/crates/cats/game_engines/examples/game_engines/fyrox.rs](./later/crates/cats/game_engines/examples/game_engines/fyrox.rs#L73))

### later/crates/cats/game_engines/examples/game_engines/ggez.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/769) ([later/crates/cats/game_engines/examples/game_engines/ggez.rs](./later/crates/cats/game_engines/examples/game_engines/ggez.rs#L71))

### later/crates/cats/game_engines/examples/game_engines/macroquad.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/771) need proper testing ([later/crates/cats/game_engines/examples/game_engines/macroquad.rs](./later/crates/cats/game_engines/examples/game_engines/macroquad.rs#L43))

### later/crates/cats/gui/Cargo.toml

- [ ] Resolve TODO/FIXME at line 17 ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L17))
- [ ] floem = { version = "0.2.0", optional = true } # review requirements ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L27))
- [ ] gtk = { version = "0.9.5", package = "gtk4", optional = true } # Make sure to use gtk4; for relm4 / gtk4 examples ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L28))
- [ ] size-of v0.1.5 does not compile on Windows ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L33))
- [ ] https://gtk-rs.org/gtk4-rs/git/book/installation_linux.html ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L37))
- [ ] https://gtk-rs.org/gtk4-rs/git/book/installation_windows.html ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L38))
- [ ] vizia = { version = "0.2.0", optional = true } # conflict with skia-safe ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L49))
- [ ] webrender = { version = "0.61.0", optional = true } # conflicts with RustQuant - native library `freetype` ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L50))
- [ ] Resolve TODO/FIXME at line 57 ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L57))

### later/crates/cats/gui/examples/2d_renderers/femtovg.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/777) ([later/crates/cats/gui/examples/2d_renderers/femtovg.rs](./later/crates/cats/gui/examples/2d_renderers/femtovg.rs#L136))

### later/crates/cats/gui/examples/2d_renderers/main.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/1047) ([later/crates/cats/gui/examples/2d_renderers/main.rs](./later/crates/cats/gui/examples/2d_renderers/main.rs#L8))

### later/crates/cats/gui/examples/2d_renderers/skia_safe.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/786) ([later/crates/cats/gui/examples/2d_renderers/skia_safe.rs](./later/crates/cats/gui/examples/2d_renderers/skia_safe.rs#L189))

### later/crates/cats/gui/examples/2d_renderers/vger.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/792) ([later/crates/cats/gui/examples/2d_renderers/vger.rs](./later/crates/cats/gui/examples/2d_renderers/vger.rs#L72))

### later/crates/cats/gui/examples/2d_renderers/webrender.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/793) ([later/crates/cats/gui/examples/2d_renderers/webrender.rs](./later/crates/cats/gui/examples/2d_renderers/webrender.rs#L148))

### later/crates/cats/gui/examples/clipboard/arboard.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/1155) ([later/crates/cats/gui/examples/clipboard/arboard.rs](./later/crates/cats/gui/examples/clipboard/arboard.rs#L31))

### later/crates/cats/gui/examples/clipboard/main.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/1048) ([later/crates/cats/gui/examples/clipboard/main.rs](./later/crates/cats/gui/examples/clipboard/main.rs#L4))

### later/crates/cats/gui/examples/file_dialogs/rfd.rs

- [ ] [how to test](https://github.com/john-cd/rust_howto/issues/785) ([later/crates/cats/gui/examples/file_dialogs/rfd.rs](./later/crates/cats/gui/examples/file_dialogs/rfd.rs#L18))

### later/crates/cats/gui/examples/gtk/gtk4.rs

- [ ] [finish; review https://gtk-rs.org/](https://github.com/john-cd/rust_howto/issues/780) ([later/crates/cats/gui/examples/gtk/gtk4.rs](./later/crates/cats/gui/examples/gtk/gtk4.rs#L94))

### later/crates/cats/gui/examples/gtk/relm4.rs

- [ ] [finish; review https://relm4.org/book/stable/](https://github.com/john-cd/rust_howto/issues/784) ([later/crates/cats/gui/examples/gtk/relm4.rs](./later/crates/cats/gui/examples/gtk/relm4.rs#L211))

### later/crates/cats/gui/examples/immediate_mode_gui/egui.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/776) verify the code manually ([later/crates/cats/gui/examples/immediate_mode_gui/egui.rs](./later/crates/cats/gui/examples/immediate_mode_gui/egui.rs#L45))

### later/crates/cats/gui/examples/retained_mode_gui/floem.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/778) ([later/crates/cats/gui/examples/retained_mode_gui/floem.rs](./later/crates/cats/gui/examples/retained_mode_gui/floem.rs#L64))

### later/crates/cats/gui/examples/retained_mode_gui/iced.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/781) ([later/crates/cats/gui/examples/retained_mode_gui/iced.rs](./later/crates/cats/gui/examples/retained_mode_gui/iced.rs#L86))

### later/crates/cats/gui/examples/retained_mode_gui/main.rs

- [ ] [finish fix](https://github.com/john-cd/rust_howto/issues/1051) ([later/crates/cats/gui/examples/retained_mode_gui/main.rs](./later/crates/cats/gui/examples/retained_mode_gui/main.rs#L15))

### later/crates/cats/gui/examples/retained_mode_gui/slint.rs

- [ ] [finish; figure how to test - neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set. see vs code Wayland setting](https://github.com/john-cd/rust_howto/issues/787) ([later/crates/cats/gui/examples/retained_mode_gui/slint.rs](./later/crates/cats/gui/examples/retained_mode_gui/slint.rs#L20))

### later/crates/cats/gui/examples/retained_mode_gui/vizia.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/1052) ([later/crates/cats/gui/examples/retained_mode_gui/vizia.rs](./later/crates/cats/gui/examples/retained_mode_gui/vizia.rs#L23))

### later/crates/cats/gui/examples/retained_mode_gui/xilem.rs

- [ ] [finish; https://github.com/linebender/xilem ](https://github.com/john-cd/rust_howto/issues/795) ([later/crates/cats/gui/examples/retained_mode_gui/xilem.rs](./later/crates/cats/gui/examples/retained_mode_gui/xilem.rs#L77))

### later/crates/cats/gui/examples/text_layout/cosmic_text.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/774) ([later/crates/cats/gui/examples/text_layout/cosmic_text.rs](./later/crates/cats/gui/examples/text_layout/cosmic_text.rs#L53))

### later/crates/cats/gui/examples/text_layout/parley.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/783) ([later/crates/cats/gui/examples/text_layout/parley.rs](./later/crates/cats/gui/examples/text_layout/parley.rs#L95))

### later/crates/cats/gui/examples/ui_layout/morphorm.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/782) ([later/crates/cats/gui/examples/ui_layout/morphorm.rs](./later/crates/cats/gui/examples/ui_layout/morphorm.rs#L99))

### later/crates/cats/gui/examples/ui_layout/taffy.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/788) ([later/crates/cats/gui/examples/ui_layout/taffy.rs](./later/crates/cats/gui/examples/ui_layout/taffy.rs#L130))

### later/crates/cats/gui/examples/web/dioxus.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/775) ([later/crates/cats/gui/examples/web/dioxus.rs](./later/crates/cats/gui/examples/web/dioxus.rs#L33))

### later/crates/cats/gui/examples/web/tauri/mod.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/790) ([later/crates/cats/gui/examples/web/tauri/mod.rs](./later/crates/cats/gui/examples/web/tauri/mod.rs#L70))

### later/crates/cats/gui/examples/window_creation/baseview.rs

- [ ] [write; review https://github.com/RustAudio/baseview](https://github.com/john-cd/rust_howto/issues/1056) ([later/crates/cats/gui/examples/window_creation/baseview.rs](./later/crates/cats/gui/examples/window_creation/baseview.rs#L23))

### later/crates/cats/gui/examples/window_creation/tao.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/789) ([later/crates/cats/gui/examples/window_creation/tao.rs](./later/crates/cats/gui/examples/window_creation/tao.rs#L68))

### later/crates/cats/gui/examples/window_creation/winit.rs

- [ ] Resolve TODO/FIXME at line 86 ([later/crates/cats/gui/examples/window_creation/winit.rs](./later/crates/cats/gui/examples/window_creation/winit.rs#L86))

### later/crates/cats/internationalization/examples/internationalization/internationalization1.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/796) ([later/crates/cats/internationalization/examples/internationalization/internationalization1.rs](./later/crates/cats/internationalization/examples/internationalization/internationalization1.rs#L12))

### later/crates/cats/localization/examples/localization/localization1.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/797) ([later/crates/cats/localization/examples/localization/localization1.rs](./later/crates/cats/localization/examples/localization/localization1.rs#L12))

### later/crates/cats/multimedia/examples/multimedia/multimedia1.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/805) ([later/crates/cats/multimedia/examples/multimedia/multimedia1.rs](./later/crates/cats/multimedia/examples/multimedia/multimedia1.rs#L12))

### later/crates/cats/multimedia_audio/examples/audio/audio.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/806) ([later/crates/cats/multimedia_audio/examples/audio/audio.rs](./later/crates/cats/multimedia_audio/examples/audio/audio.rs#L12))

### later/crates/cats/multimedia_encoding/examples/encoding/encoding.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/807) ([later/crates/cats/multimedia_encoding/examples/encoding/encoding.rs](./later/crates/cats/multimedia_encoding/examples/encoding/encoding.rs#L12))

### later/crates/cats/multimedia_images/examples/images/images.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/808) ([later/crates/cats/multimedia_images/examples/images/images.rs](./later/crates/cats/multimedia_images/examples/images/images.rs#L12))

### later/crates/cats/multimedia_video/examples/video/video.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/809) ([later/crates/cats/multimedia_video/examples/video/video.rs](./later/crates/cats/multimedia_video/examples/video/video.rs#L12))

### later/crates/cats/no_std/examples/no_std/no_std1.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/814) ([later/crates/cats/no_std/examples/no_std/no_std1.rs](./later/crates/cats/no_std/examples/no_std/no_std1.rs#L12))

### later/crates/cats/no_std_no_alloc/examples/no_alloc/no_alloc.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/815) ([later/crates/cats/no_std_no_alloc/examples/no_alloc/no_alloc.rs](./later/crates/cats/no_std_no_alloc/examples/no_alloc/no_alloc.rs#L12))

### later/crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/817) ([later/crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs](./later/crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs#L14))

### later/crates/cats/os_linux_apis/examples/linux/linux.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/818) ([later/crates/cats/os_linux_apis/examples/linux/linux.rs](./later/crates/cats/os_linux_apis/examples/linux/linux.rs#L14))

### later/crates/cats/os_macos_apis/examples/macos/macos.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/819) ([later/crates/cats/os_macos_apis/examples/macos/macos.rs](./later/crates/cats/os_macos_apis/examples/macos/macos.rs#L14))

### later/crates/cats/rendering/examples/2d_raster_graphics/render.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/828) ([later/crates/cats/rendering/examples/2d_raster_graphics/render.rs](./later/crates/cats/rendering/examples/2d_raster_graphics/render.rs#L12))

### later/crates/cats/rendering_data_formats/examples/data_formats/data_formats.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/829) ([later/crates/cats/rendering_data_formats/examples/data_formats/data_formats.rs](./later/crates/cats/rendering_data_formats/examples/data_formats/data_formats.rs#L12))

### later/crates/cats/rendering_engine/examples/rendering_engines/render.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/830) ([later/crates/cats/rendering_engine/examples/rendering_engines/render.rs](./later/crates/cats/rendering_engine/examples/rendering_engines/render.rs#L12))

### later/crates/cats/rendering_graphics_api/Cargo.toml

- [ ] Resolve TODO/FIXME at line 18 ([later/crates/cats/rendering_graphics_api/Cargo.toml](./later/crates/cats/rendering_graphics_api/Cargo.toml#L18))
- [ ] wgpu = [] # "dep:wgpu" ([later/crates/cats/rendering_graphics_api/Cargo.toml](./later/crates/cats/rendering_graphics_api/Cargo.toml#L22))

### later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/772) ([later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs](./later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs#L156))

### later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/graphics.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/831) ([later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/graphics.rs](./later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/graphics.rs#L12))

### later/crates/cats/science/Cargo.toml

- [ ] Resolve TODO/FIXME at line 17 ([later/crates/cats/science/Cargo.toml](./later/crates/cats/science/Cargo.toml#L17))
- [ ] don't need all candle crates ([later/crates/cats/science/Cargo.toml](./later/crates/cats/science/Cargo.toml#L20))
- [ ] smartcore = { version = "0.4.0", features = ["datasets"] } ([later/crates/cats/science/Cargo.toml](./later/crates/cats/science/Cargo.toml#L31))
- [ ] do we need a feature? candle = ["dep:candle-core", "dep:candle-nn"] ([later/crates/cats/science/Cargo.toml](./later/crates/cats/science/Cargo.toml#L38))

### later/crates/cats/science/examples/ml/candle.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/835) review https://huggingface.github.io/candle/index.html ([later/crates/cats/science/examples/ml/candle.rs](./later/crates/cats/science/examples/ml/candle.rs#L112))

### later/crates/cats/science/examples/ml/smartcore.rs

- [ ] [finish; kNN example; syn data generation for linear regr.; 70/30 split; etc](https://github.com/john-cd/rust_howto/issues/837) ([later/crates/cats/science/examples/ml/smartcore.rs](./later/crates/cats/science/examples/ml/smartcore.rs#L79))

### later/crates/cats/science_geo/examples/geo/geo.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/839) ([later/crates/cats/science_geo/examples/geo/geo.rs](./later/crates/cats/science_geo/examples/geo/geo.rs#L12))

### later/crates/cats/science_neuroscience/examples/neuroscience/neuro.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/840) ([later/crates/cats/science_neuroscience/examples/neuroscience/neuro.rs](./later/crates/cats/science_neuroscience/examples/neuroscience/neuro.rs#L12))

### later/crates/cats/science_robotics/Cargo.toml

- [ ] openrr = { version = "0.1.0", optional = true } # lacks native static library `assimpd` on windows https://github.com/assimp/assimp/blob/master/Build.md ([later/crates/cats/science_robotics/Cargo.toml](./later/crates/cats/science_robotics/Cargo.toml#L19))
- [ ] zenoh = "1.1.0" # https://github.com/john-cd/rust_howto/issues/1441 ([later/crates/cats/science_robotics/Cargo.toml](./later/crates/cats/science_robotics/Cargo.toml#L21))
- [ ] openrr = ["dep:openrr"] ([later/crates/cats/science_robotics/Cargo.toml](./later/crates/cats/science_robotics/Cargo.toml#L32))

### later/crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs

- [ ] [review / exapnd](https://github.com/john-cd/rust_howto/issues/841) ([later/crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs](./later/crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs#L128))

### later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/844) ([later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs](./later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs#L12))

### later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs

- [ ] [write](https://github.com/john-cd/rust_howto/issues/843) ([later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs](./later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs#L155))

### later/crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs

- [ ] [write](https://github.com/john-cd/rust_howto/issues/845) ([later/crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs](./later/crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs#L87))

### later/crates/cats/simulation/examples/simulation/simulation1.rs

- [ ] [write LATER](https://github.com/john-cd/rust_howto/issues/846) ([later/crates/cats/simulation/examples/simulation/simulation1.rs](./later/crates/cats/simulation/examples/simulation/simulation1.rs#L12))

### later/crates/cats/virtualization/Cargo.toml

- [ ] [LATER review wasmi](https://github.com/john-cd/rust_howto/issues/1263) ([later/crates/cats/virtualization/Cargo.toml](./later/crates/cats/virtualization/Cargo.toml#L18))

### later/crates/cats/virtualization/examples/virtualization/virtualization1.rs

- [ ] [write LATER;  cover WASMI cargo add wasmi](https://github.com/john-cd/rust_howto/issues/852) ([later/crates/cats/virtualization/examples/virtualization/virtualization1.rs](./later/crates/cats/virtualization/examples/virtualization/virtualization1.rs#L12))

### later/crates/cats/visualization/examples/visualization/plotly.rs

- [ ] [review how to test](https://github.com/john-cd/rust_howto/issues/884 ([later/crates/cats/visualization/examples/visualization/plotly.rs](./later/crates/cats/visualization/examples/visualization/plotly.rs#L50))

### later/crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs

- [ ] [review; https://docs.wasmtime.dev/introduction.html https://docs.rs/wasmtime/latest/wasmtime/](https://github.com/john-cd/rust_howto/issues/855) ([later/crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs](./later/crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs#L126))

### later/crates/cats/wasm/examples/yew/yew.rs

- [ ] [review](https://github.com/john-cd/rust_howto/issues/856) ([later/crates/cats/wasm/examples/yew/yew.rs](./later/crates/cats/wasm/examples/yew/yew.rs#L59))

### later/crates/cats/web_programming_websocket/Cargo.toml

- [ ] async-tungstenite = { version = "0.29.1", features = [ ([later/crates/cats/web_programming_websocket/Cargo.toml](./later/crates/cats/web_programming_websocket/Cargo.toml#L18))
- [ ] [async-tungstenite: review](https://github.com/john-cd/rust_howto/issues/1291) ([later/crates/cats/web_programming_websocket/Cargo.toml](./later/crates/cats/web_programming_websocket/Cargo.toml#L22))

### later/crates/cats/web_programming_websocket/examples/async_tungstenite.rs

- [ ] [Web] Migrate to wss://echo.websocket.events and enable async-tungstenite example (https://github.com/john-cd/rust_howto/issues/1058) - COMPLETED ([later/crates/cats/web_programming_websocket/examples/async_tungstenite.rs](./later/crates/cats/web_programming_websocket/examples/async_tungstenite.rs#L58))

### later/crates/cats/web_programming_websocket/examples/tokio_tungstenite.rs

- [ ] [review https://github.com/snapview/tokio-tungstenite/tree/master/examples](https://github.com/john-cd/rust_howto/issues/1145) ([later/crates/cats/web_programming_websocket/examples/tokio_tungstenite.rs](./later/crates/cats/web_programming_websocket/examples/tokio_tungstenite.rs#L57))

### later/crates/other/Cargo.toml

- [ ] Resolve TODO/FIXME at line 19 ([later/crates/other/Cargo.toml](./later/crates/other/Cargo.toml#L19))

### later/crates/other/examples/architecture/cqrs.rs

- [ ] get_all_products() -> Vec<Product> ([later/crates/other/examples/architecture/cqrs.rs](./later/crates/other/examples/architecture/cqrs.rs#L247))
- [ ] finish ([later/crates/other/examples/architecture/cqrs.rs](./later/crates/other/examples/architecture/cqrs.rs#L420))

### later/crates/other/examples/cloud/aws_lambda.rs

- [ ] [finish](https://github.com/john-cd/rust_howto/issues/878) ([later/crates/other/examples/cloud/aws_lambda.rs](./later/crates/other/examples/cloud/aws_lambda.rs#L62))

### later/crates/other/examples/cloud/aws_sdk.rs

- [ ] main()?; // Skip running S3 queries in simple unit tests to avoid network ([later/crates/other/examples/cloud/aws_sdk.rs](./later/crates/other/examples/cloud/aws_sdk.rs#L47))

### playground/examples_wip/Cargo.toml

- [ ] Resolve TODO/FIXME at line 16 ([playground/examples_wip/Cargo.toml](./playground/examples_wip/Cargo.toml#L16))

### GitHub issue references

#### bk/crates/cats/asynchronous/examples/async_channels/postage.rs

- [ ] Review issue [#80](https://github.com/john-cd/rust_howto/issues/80) in [bk/crates/cats/asynchronous/examples/async_channels/postage.rs](./bk/crates/cats/asynchronous/examples/async_channels/postage.rs#L76)

#### bk/crates/cats/asynchronous/examples/streams/streams2.rs

- [ ] Review issue [#645](https://github.com/john-cd/rust_howto/issues/645) in [bk/crates/cats/asynchronous/examples/streams/streams2.rs](./bk/crates/cats/asynchronous/examples/streams/streams2.rs#L72)

#### bk/crates/cats/caching/examples/in_memory_cache/cached.rs

- [ ] Review issue [#1354](https://github.com/john-cd/rust_howto/issues/1354) in [bk/crates/cats/caching/examples/in_memory_cache/cached.rs](./bk/crates/cats/caching/examples/in_memory_cache/cached.rs#L66)

#### bk/crates/cats/compression/examples/compression/flate2.rs

- [ ] Review issue [#1009](https://github.com/john-cd/rust_howto/issues/1009) in [bk/crates/cats/compression/examples/compression/flate2.rs](./bk/crates/cats/compression/examples/compression/flate2.rs#L83)

#### bk/crates/cats/compression/examples/tar/tar_compress.rs

- [ ] Review issue [#255](https://github.com/john-cd/rust_howto/issues/255) in [bk/crates/cats/compression/examples/tar/tar_compress.rs](./bk/crates/cats/compression/examples/tar/tar_compress.rs#L32)

#### bk/crates/cats/compression/examples/tar/tar_decompress.rs

- [ ] Review issue [#254](https://github.com/john-cd/rust_howto/issues/254) in [bk/crates/cats/compression/examples/tar/tar_decompress.rs](./bk/crates/cats/compression/examples/tar/tar_decompress.rs#L24)

#### bk/crates/cats/compression/examples/tar/tar_strip_prefix.rs

- [ ] Review issue [#256](https://github.com/john-cd/rust_howto/issues/256) in [bk/crates/cats/compression/examples/tar/tar_strip_prefix.rs](./bk/crates/cats/compression/examples/tar/tar_strip_prefix.rs#L36)

#### bk/crates/cats/concurrency/examples/actors/actix.rs

- [ ] Review issue [#682](https://github.com/john-cd/rust_howto/issues/682) in [bk/crates/cats/concurrency/examples/actors/actix.rs](./bk/crates/cats/concurrency/examples/actors/actix.rs#L63)

#### bk/crates/cats/concurrency/examples/actors/actors.rs

- [ ] Review issue [#1011](https://github.com/john-cd/rust_howto/issues/1011) in [bk/crates/cats/concurrency/examples/actors/actors.rs](./bk/crates/cats/concurrency/examples/actors/actors.rs#L32)

#### bk/crates/cats/concurrency/examples/actors/stakker.rs

- [ ] Review issue [#94](https://github.com/john-cd/rust_howto/issues/94) in [bk/crates/cats/concurrency/examples/actors/stakker.rs](./bk/crates/cats/concurrency/examples/actors/stakker.rs#L79)

#### bk/crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs

- [ ] Review issue [#1152](https://github.com/john-cd/rust_howto/issues/1152) in [bk/crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs](./bk/crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs#L113)

#### bk/crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs

- [ ] Review issue [#261](https://github.com/john-cd/rust_howto/issues/261) in [bk/crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs](./bk/crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs#L94)

#### bk/crates/cats/concurrency/examples/shared_state/global_mut_state.rs

- [ ] Review issue [#939](https://github.com/john-cd/rust_howto/issues/939) in [bk/crates/cats/concurrency/examples/shared_state/global_mut_state.rs](./bk/crates/cats/concurrency/examples/shared_state/global_mut_state.rs#L49)

#### bk/crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs

- [ ] Review issue [#268](https://github.com/john-cd/rust_howto/issues/268) in [bk/crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs](./bk/crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs#L130)

#### bk/crates/cats/config/examples/configuration/confy.rs

- [ ] Review issue [#148](https://github.com/john-cd/rust_howto/issues/148) in [bk/crates/cats/config/examples/configuration/confy.rs](./bk/crates/cats/config/examples/configuration/confy.rs#L37)

#### bk/crates/cats/cryptography/examples/certs/pem_rfc7468.rs

- [ ] Review issue [#697](https://github.com/john-cd/rust_howto/issues/697) in [bk/crates/cats/cryptography/examples/certs/pem_rfc7468.rs](./bk/crates/cats/cryptography/examples/certs/pem_rfc7468.rs#L40)

#### bk/crates/cats/cryptography/examples/certs/x509_cert.rs

- [ ] Review issue [#705](https://github.com/john-cd/rust_howto/issues/705) in [bk/crates/cats/cryptography/examples/certs/x509_cert.rs](./bk/crates/cats/cryptography/examples/certs/x509_cert.rs#L65)

#### bk/crates/cats/cryptography/examples/sign/ecdsa.rs

- [ ] Review issue [#693](https://github.com/john-cd/rust_howto/issues/693) in [bk/crates/cats/cryptography/examples/sign/ecdsa.rs](./bk/crates/cats/cryptography/examples/sign/ecdsa.rs#L66)

#### bk/crates/cats/cryptography/examples/sign/ed25519.rs

- [ ] Review issue [#1084](https://github.com/john-cd/rust_howto/issues/1084) in [bk/crates/cats/cryptography/examples/sign/ed25519.rs](./bk/crates/cats/cryptography/examples/sign/ed25519.rs#L67)

#### bk/crates/cats/cryptography/examples/tls/native_tls.rs

- [ ] Review issue [#696](https://github.com/john-cd/rust_howto/issues/696) in [bk/crates/cats/cryptography/examples/tls/native_tls.rs](./bk/crates/cats/cryptography/examples/tls/native_tls.rs#L37)

#### bk/crates/cats/cryptography/examples/tls/rustls.rs

- [ ] Review issue [#700](https://github.com/john-cd/rust_howto/issues/700) in [bk/crates/cats/cryptography/examples/tls/rustls.rs](./bk/crates/cats/cryptography/examples/tls/rustls.rs#L79)

#### bk/crates/cats/database/examples/cassandra/cassandra_protocol.rs

- [ ] Review issue [#1016](https://github.com/john-cd/rust_howto/issues/1016) in [bk/crates/cats/database/examples/cassandra/cassandra_protocol.rs](./bk/crates/cats/database/examples/cassandra/cassandra_protocol.rs#L103)

#### bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs

- [ ] Review issue [#1017](https://github.com/john-cd/rust_howto/issues/1017) in [bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs](./bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs#L125)

#### bk/crates/cats/database/examples/connection_pool/deadpool2.rs

- [ ] Review issue [#46](https://github.com/john-cd/rust_howto/issues/46) in [bk/crates/cats/database/examples/connection_pool/deadpool2.rs](./bk/crates/cats/database/examples/connection_pool/deadpool2.rs#L73)

#### bk/crates/cats/database/examples/mssql/tiberius.rs

- [ ] Review issue [#1019](https://github.com/john-cd/rust_howto/issues/1019) in [bk/crates/cats/database/examples/mssql/tiberius.rs](./bk/crates/cats/database/examples/mssql/tiberius.rs#L79)

#### bk/crates/cats/database/examples/nosql/redis.rs

- [ ] Review issue [#1161](https://github.com/john-cd/rust_howto/issues/1161) in [bk/crates/cats/database/examples/nosql/redis.rs](./bk/crates/cats/database/examples/nosql/redis.rs#L71)

#### bk/crates/cats/database/examples/oracle/diesel_oci.rs

- [ ] Review issue [#1020](https://github.com/john-cd/rust_howto/issues/1020) in [bk/crates/cats/database/examples/oracle/diesel_oci.rs](./bk/crates/cats/database/examples/oracle/diesel_oci.rs#L114)

#### bk/crates/cats/database/examples/oracle/oracle.rs

- [ ] Review issue [#1021](https://github.com/john-cd/rust_howto/issues/1021) in [bk/crates/cats/database/examples/oracle/oracle.rs](./bk/crates/cats/database/examples/oracle/oracle.rs#L73)

#### bk/crates/cats/database/examples/oracle/sibyl.rs

- [ ] Review issue [#1022](https://github.com/john-cd/rust_howto/issues/1022) in [bk/crates/cats/database/examples/oracle/sibyl.rs](./bk/crates/cats/database/examples/oracle/sibyl.rs#L88)

#### bk/crates/cats/database/examples/postgres/aggregate_data.rs

- [ ] Review issue [#1162](https://github.com/john-cd/rust_howto/issues/1162) in [bk/crates/cats/database/examples/postgres/aggregate_data.rs](./bk/crates/cats/database/examples/postgres/aggregate_data.rs#L43)

#### bk/crates/cats/database/examples/postgres/cornucopia.rs

- [ ] Review issue [#708](https://github.com/john-cd/rust_howto/issues/708) in [bk/crates/cats/database/examples/postgres/cornucopia.rs](./bk/crates/cats/database/examples/postgres/cornucopia.rs#L54)

#### bk/crates/cats/database/examples/postgres/main.rs

- [ ] Review issue [#713](https://github.com/john-cd/rust_howto/issues/713) in [bk/crates/cats/database/examples/postgres/main.rs](./bk/crates/cats/database/examples/postgres/main.rs#L40)

#### bk/crates/cats/database/examples/postgres/tokio_postgres.rs

- [ ] Review issue [#719](https://github.com/john-cd/rust_howto/issues/719) in [bk/crates/cats/database/examples/postgres/tokio_postgres.rs](./bk/crates/cats/database/examples/postgres/tokio_postgres.rs#L99)

#### bk/crates/cats/database/examples/query_builders_orms/diesel1.rs

- [ ] Review issue [#709](https://github.com/john-cd/rust_howto/issues/709) in [bk/crates/cats/database/examples/query_builders_orms/diesel1.rs](./bk/crates/cats/database/examples/query_builders_orms/diesel1.rs#L87)

#### bk/crates/cats/database/examples/query_builders_orms/sea_orm.rs

- [ ] Review issue [#715](https://github.com/john-cd/rust_howto/issues/715) in [bk/crates/cats/database/examples/query_builders_orms/sea_orm.rs](./bk/crates/cats/database/examples/query_builders_orms/sea_orm.rs#L53)

#### bk/crates/cats/database/examples/search/elasticsearch.rs

- [ ] Review issue [#710](https://github.com/john-cd/rust_howto/issues/710) in [bk/crates/cats/database/examples/search/elasticsearch.rs](./bk/crates/cats/database/examples/search/elasticsearch.rs#L156)

#### bk/crates/cats/database_implementations/examples/databases/surrealdb.rs

- [ ] Review issue [#1148](https://github.com/john-cd/rust_howto/issues/1148) in [bk/crates/cats/database_implementations/examples/databases/surrealdb.rs](./bk/crates/cats/database_implementations/examples/databases/surrealdb.rs#L97)

#### bk/crates/cats/development_tools_build_utils/build.rs

- [ ] Review issue [#998](https://github.com/john-cd/rust_howto/issues/998) in [bk/crates/cats/development_tools_build_utils/build.rs](./bk/crates/cats/development_tools_build_utils/build.rs#L63)

#### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs

- [ ] Review issue [#897](https://github.com/john-cd/rust_howto/issues/897) in [bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs#L30)

#### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs

- [ ] Review issue [#898](https://github.com/john-cd/rust_howto/issues/898) in [bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs#L29)

#### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs

- [ ] Review issue [#899](https://github.com/john-cd/rust_howto/issues/899) in [bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs#L26)

#### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs

- [ ] Review issue [#900](https://github.com/john-cd/rust_howto/issues/900) in [bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs#L40)

#### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs

- [ ] Review issue [#1000](https://github.com/john-cd/rust_howto/issues/1000) in [bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs#L26)

#### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs

- [ ] Review issue [#901](https://github.com/john-cd/rust_howto/issues/901) in [bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs#L23)

#### bk/crates/cats/development_tools_debugging/examples/log/log_env_variable.rs

- [ ] Review issue [#157](https://github.com/john-cd/rust_howto/issues/157) in [bk/crates/cats/development_tools_debugging/examples/log/log_env_variable.rs](./bk/crates/cats/development_tools_debugging/examples/log/log_env_variable.rs#L70)

#### bk/crates/cats/development_tools_debugging/examples/other/open_observe.rs

- [ ] Review issue [#733](https://github.com/john-cd/rust_howto/issues/733) in [bk/crates/cats/development_tools_debugging/examples/other/open_observe.rs](./bk/crates/cats/development_tools_debugging/examples/other/open_observe.rs#L234)

#### bk/crates/cats/development_tools_debugging/examples/other/open_telemetry.rs

- [ ] Review issue [#734](https://github.com/john-cd/rust_howto/issues/734) in [bk/crates/cats/development_tools_debugging/examples/other/open_telemetry.rs](./bk/crates/cats/development_tools_debugging/examples/other/open_telemetry.rs#L69)

#### bk/crates/cats/development_tools_profiling/benches/divan.rs

- [ ] Review issue [#747](https://github.com/john-cd/rust_howto/issues/747) in [bk/crates/cats/development_tools_profiling/benches/divan.rs](./bk/crates/cats/development_tools_profiling/benches/divan.rs#L52)

#### bk/crates/cats/development_tools_testing/examples/fuzzing/afl.rs

- [ ] Review issue [#748](https://github.com/john-cd/rust_howto/issues/748) in [bk/crates/cats/development_tools_testing/examples/fuzzing/afl.rs](./bk/crates/cats/development_tools_testing/examples/fuzzing/afl.rs#L53)

#### bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs

- [ ] Review issue [#1124](https://github.com/john-cd/rust_howto/issues/1124) in [bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs](./bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs#L138)

#### bk/crates/cats/email/examples/lettre.rs

- [ ] Review issue [#1144](https://github.com/john-cd/rust_howto/issues/1144) in [bk/crates/cats/email/examples/lettre.rs](./bk/crates/cats/email/examples/lettre.rs#L45)

#### bk/crates/cats/encoding/build.rs

- [ ] Review issue [#1417](https://github.com/john-cd/rust_howto/issues/1417) in [bk/crates/cats/encoding/build.rs](./bk/crates/cats/encoding/build.rs#L30)

#### bk/crates/cats/encoding/examples/binary_encoders/bincode.rs

- [ ] Review issue [#1040](https://github.com/john-cd/rust_howto/issues/1040) in [bk/crates/cats/encoding/examples/binary_encoders/bincode.rs](./bk/crates/cats/encoding/examples/binary_encoders/bincode.rs#L56)

#### bk/crates/cats/encoding/examples/binary_encoders/capnp.rs

- [ ] Review issue [#1041](https://github.com/john-cd/rust_howto/issues/1041) in [bk/crates/cats/encoding/examples/binary_encoders/capnp.rs](./bk/crates/cats/encoding/examples/binary_encoders/capnp.rs#L69)

#### bk/crates/cats/encoding/examples/binary_encoders/flatbuffers.rs

- [ ] Review issue [#1043](https://github.com/john-cd/rust_howto/issues/1043) in [bk/crates/cats/encoding/examples/binary_encoders/flatbuffers.rs](./bk/crates/cats/encoding/examples/binary_encoders/flatbuffers.rs#L68)

#### bk/crates/cats/encoding/examples/binary_encoders/main.rs

- [ ] Review issue [#1234](https://github.com/john-cd/rust_howto/issues/1234) in [bk/crates/cats/encoding/examples/binary_encoders/main.rs](./bk/crates/cats/encoding/examples/binary_encoders/main.rs#L2)

#### bk/crates/cats/encoding/examples/binary_encoders/prost.rs

- [ ] Review issue [#1044](https://github.com/john-cd/rust_howto/issues/1044) in [bk/crates/cats/encoding/examples/binary_encoders/prost.rs](./bk/crates/cats/encoding/examples/binary_encoders/prost.rs#L38)

#### bk/crates/cats/encoding/examples/binary_encoders/protobuf.rs

- [ ] Review issue [#1045](https://github.com/john-cd/rust_howto/issues/1045) in [bk/crates/cats/encoding/examples/binary_encoders/protobuf.rs](./bk/crates/cats/encoding/examples/binary_encoders/protobuf.rs#L54)

#### bk/crates/cats/encoding/examples/binary_encoders/rmp_serde.rs

- [ ] Review issue [#1046](https://github.com/john-cd/rust_howto/issues/1046) in [bk/crates/cats/encoding/examples/binary_encoders/rmp_serde.rs](./bk/crates/cats/encoding/examples/binary_encoders/rmp_serde.rs#L58)

#### bk/crates/cats/encoding/examples/string_encoding/percent_encoding.rs

- [ ] Review issue [#1353](https://github.com/john-cd/rust_howto/issues/1353) in [bk/crates/cats/encoding/examples/string_encoding/percent_encoding.rs](./bk/crates/cats/encoding/examples/string_encoding/percent_encoding.rs#L111)

#### bk/crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs

- [ ] Review issue [#1350](https://github.com/john-cd/rust_howto/issues/1350) in [bk/crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs](./bk/crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs#L49)

#### bk/crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs

- [ ] Review issue [#939](https://github.com/john-cd/rust_howto/issues/939) in [bk/crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs](./bk/crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs#L36)

#### bk/crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs

- [ ] Review issue [#939](https://github.com/john-cd/rust_howto/issues/939) in [bk/crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs](./bk/crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs#L89)

#### bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs

- [ ] Review issue [#811](https://github.com/john-cd/rust_howto/issues/811) in [bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs](./bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs#L83)

#### bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs

- [ ] Review issue [#812](https://github.com/john-cd/rust_howto/issues/812) in [bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs](./bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs#L79)

#### bk/crates/cats/network_programming/examples/server/glommio.rs

- [ ] Review issue [#810](https://github.com/john-cd/rust_howto/issues/810) in [bk/crates/cats/network_programming/examples/server/glommio.rs](./bk/crates/cats/network_programming/examples/server/glommio.rs#L104)

#### bk/crates/cats/network_programming/examples/server/listen_unused.rs

- [ ] Review issue [#166](https://github.com/john-cd/rust_howto/issues/166) in [bk/crates/cats/network_programming/examples/server/listen_unused.rs](./bk/crates/cats/network_programming/examples/server/listen_unused.rs#L42)

#### bk/crates/cats/os_windows_apis/examples/windows/winapi.rs

- [ ] Review issue [#822](https://github.com/john-cd/rust_howto/issues/822) in [bk/crates/cats/os_windows_apis/examples/windows/winapi.rs](./bk/crates/cats/os_windows_apis/examples/windows/winapi.rs#L132)

#### bk/crates/cats/os_windows_apis/examples/windows/windows.rs

- [ ] Review issue [#823](https://github.com/john-cd/rust_howto/issues/823) in [bk/crates/cats/os_windows_apis/examples/windows/windows.rs](./bk/crates/cats/os_windows_apis/examples/windows/windows.rs#L43)

#### bk/crates/cats/parser_implementations/examples/html/cssparser.rs

- [ ] Review issue [#1092](https://github.com/john-cd/rust_howto/issues/1092) in [bk/crates/cats/parser_implementations/examples/html/cssparser.rs](./bk/crates/cats/parser_implementations/examples/html/cssparser.rs#L112)

#### bk/crates/cats/parser_implementations/examples/html/html5ever.rs

- [ ] Review issue [#1090](https://github.com/john-cd/rust_howto/issues/1090) in [bk/crates/cats/parser_implementations/examples/html/html5ever.rs](./bk/crates/cats/parser_implementations/examples/html/html5ever.rs#L64)

#### bk/crates/cats/parser_implementations/examples/xml/quick_xml.rs

- [ ] Review issue [#1236](https://github.com/john-cd/rust_howto/issues/1236) in [bk/crates/cats/parser_implementations/examples/xml/quick_xml.rs](./bk/crates/cats/parser_implementations/examples/xml/quick_xml.rs#L156)

#### bk/crates/cats/parser_implementations/examples/xml/xml.rs

- [ ] Review issue [#1100](https://github.com/john-cd/rust_howto/issues/1100) in [bk/crates/cats/parser_implementations/examples/xml/xml.rs](./bk/crates/cats/parser_implementations/examples/xml/xml.rs#L104)

#### bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs

- [ ] Review issue [#1103](https://github.com/john-cd/rust_howto/issues/1103) in [bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs](./bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs#L322)

#### bk/crates/cats/parser_implementations/examples/xml/xmlparser.rs

- [ ] Review issue [#1102](https://github.com/john-cd/rust_howto/issues/1102) in [bk/crates/cats/parser_implementations/examples/xml/xmlparser.rs](./bk/crates/cats/parser_implementations/examples/xml/xmlparser.rs#L101)

#### bk/crates/cats/parsing/examples/parsing/tree_sitter.rs

- [ ] Review issue [#827](https://github.com/john-cd/rust_howto/issues/827) in [bk/crates/cats/parsing/examples/parsing/tree_sitter.rs](./bk/crates/cats/parsing/examples/parsing/tree_sitter.rs#L162)

#### bk/crates/cats/parsing/examples/pest/pest.rs

- [ ] Review issue [#826](https://github.com/john-cd/rust_howto/issues/826) in [bk/crates/cats/parsing/examples/pest/pest.rs](./bk/crates/cats/parsing/examples/pest/pest.rs#L57)

#### bk/crates/cats/rust_patterns/examples/functional_programming/either.rs

- [ ] Review issue [#1317](https://github.com/john-cd/rust_howto/issues/1317) in [bk/crates/cats/rust_patterns/examples/functional_programming/either.rs](./bk/crates/cats/rust_patterns/examples/functional_programming/either.rs#L46)

#### bk/crates/cats/rust_patterns/examples/functional_programming/frunk.rs

- [ ] Review issue [#1318](https://github.com/john-cd/rust_howto/issues/1318) in [bk/crates/cats/rust_patterns/examples/functional_programming/frunk.rs](./bk/crates/cats/rust_patterns/examples/functional_programming/frunk.rs#L172)

#### bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs

- [ ] Review issue [#1120](https://github.com/john-cd/rust_howto/issues/1120) in [bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs](./bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs#L227)

#### bk/crates/cats/web_programming/examples/http_types_and_interfaces/http.rs

- [ ] Review issue [#1355](https://github.com/john-cd/rust_howto/issues/1355) in [bk/crates/cats/web_programming/examples/http_types_and_interfaces/http.rs](./bk/crates/cats/web_programming/examples/http_types_and_interfaces/http.rs#L44)

#### bk/crates/cats/web_programming/examples/scraping/broken.rs

- [ ] Review issue [#1419](https://github.com/john-cd/rust_howto/issues/1419) in [bk/crates/cats/web_programming/examples/scraping/broken.rs](./bk/crates/cats/web_programming/examples/scraping/broken.rs#L78)

#### bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs

- [ ] Review issue [#860](https://github.com/john-cd/rust_howto/issues/860) in [bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs](./bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs#L147)

#### bk/crates/cats/web_programming_http_client/examples/apis/rate_limited.rs

- [ ] Review issue [#177](https://github.com/john-cd/rust_howto/issues/177) in [bk/crates/cats/web_programming_http_client/examples/apis/rate_limited.rs](./bk/crates/cats/web_programming_http_client/examples/apis/rate_limited.rs#L89)

#### bk/crates/cats/web_programming_http_client/examples/apis/rest_post.rs

- [ ] Review issue [#178](https://github.com/john-cd/rust_howto/issues/178) in [bk/crates/cats/web_programming_http_client/examples/apis/rest_post.rs](./bk/crates/cats/web_programming_http_client/examples/apis/rest_post.rs#L89)

#### bk/crates/cats/web_programming_http_client/examples/download/download.rs

- [ ] Review issue [#225](https://github.com/john-cd/rust_howto/issues/225) in [bk/crates/cats/web_programming_http_client/examples/download/download.rs](./bk/crates/cats/web_programming_http_client/examples/download/download.rs#L45)

#### bk/crates/cats/web_programming_http_client/examples/download/partial.rs

- [ ] Review issue [#176](https://github.com/john-cd/rust_howto/issues/176) in [bk/crates/cats/web_programming_http_client/examples/download/partial.rs](./bk/crates/cats/web_programming_http_client/examples/download/partial.rs#L120)

#### bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs

- [ ] Review issue [#859](https://github.com/john-cd/rust_howto/issues/859) in [bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs](./bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs#L94)

#### bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs

- [ ] Review issue [#862](https://github.com/john-cd/rust_howto/issues/862) in [bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs](./bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs#L28)

#### bk/crates/cats/web_programming_http_server/Cargo.toml

- [ ] Review issue [#1314](https://github.com/john-cd/rust_howto/issues/1314) in [bk/crates/cats/web_programming_http_server/Cargo.toml](./bk/crates/cats/web_programming_http_server/Cargo.toml#L26)

#### bk/crates/cats/web_programming_http_server/examples/async_graphql.rs

- [ ] Review issue [#864](https://github.com/john-cd/rust_howto/issues/864) in [bk/crates/cats/web_programming_http_server/examples/async_graphql.rs](./bk/crates/cats/web_programming_http_server/examples/async_graphql.rs#L74)

#### bk/crates/cats/web_programming_http_server/examples/axum.rs

- [ ] Review issue [#865](https://github.com/john-cd/rust_howto/issues/865) in [bk/crates/cats/web_programming_http_server/examples/axum.rs](./bk/crates/cats/web_programming_http_server/examples/axum.rs#L122)

#### bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs

- [ ] Review issue [#870](https://github.com/john-cd/rust_howto/issues/870) in [bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs](./bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs#L87)

#### bk/crates/cats/web_programming_http_server/examples/hyper_server.rs

- [ ] Review issue [#866](https://github.com/john-cd/rust_howto/issues/866) in [bk/crates/cats/web_programming_http_server/examples/hyper_server.rs](./bk/crates/cats/web_programming_http_server/examples/hyper_server.rs#L75)

#### bk/crates/cats/web_programming_http_server/examples/leptos.rs

- [ ] Review issue [#867](https://github.com/john-cd/rust_howto/issues/867) in [bk/crates/cats/web_programming_http_server/examples/leptos.rs](./bk/crates/cats/web_programming_http_server/examples/leptos.rs#L66)

#### bk/crates/cats/web_programming_http_server/examples/loco/main.rs

- [ ] Review issue [#868](https://github.com/john-cd/rust_howto/issues/868) in [bk/crates/cats/web_programming_http_server/examples/loco/main.rs](./bk/crates/cats/web_programming_http_server/examples/loco/main.rs#L88)

#### bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs

- [ ] Review issue [#871](https://github.com/john-cd/rust_howto/issues/871) in [bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs](./bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs#L95)

#### bk/crates/cats/web_programming_http_server/examples/rocket.rs

- [ ] Review issue [#869](https://github.com/john-cd/rust_howto/issues/869) in [bk/crates/cats/web_programming_http_server/examples/rocket.rs](./bk/crates/cats/web_programming_http_server/examples/rocket.rs#L21)

#### later/crates/cats/computer_vision/examples/opencv/opencv.rs

- [ ] Review issue [#1079](https://github.com/john-cd/rust_howto/issues/1079) in [later/crates/cats/computer_vision/examples/opencv/opencv.rs](./later/crates/cats/computer_vision/examples/opencv/opencv.rs#L60)

#### later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs

- [ ] Review issue [#707](https://github.com/john-cd/rust_howto/issues/707) in [later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs](./later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs#L12)

#### later/crates/cats/development_tools_ffi/Cargo.toml

- [ ] Review issue [#1307](https://github.com/john-cd/rust_howto/issues/1307) in [later/crates/cats/development_tools_ffi/Cargo.toml](./later/crates/cats/development_tools_ffi/Cargo.toml#L14)

#### later/crates/cats/development_tools_ffi/build.rs

- [ ] Review issue [#1026](https://github.com/john-cd/rust_howto/issues/1026) in [later/crates/cats/development_tools_ffi/build.rs](./later/crates/cats/development_tools_ffi/build.rs#L80)

#### later/crates/cats/development_tools_ffi/examples/c/bindgen.rs

- [ ] Review issue [#1001](https://github.com/john-cd/rust_howto/issues/1001) in [later/crates/cats/development_tools_ffi/examples/c/bindgen.rs](./later/crates/cats/development_tools_ffi/examples/c/bindgen.rs#L53)

#### later/crates/cats/development_tools_ffi/examples/c/cbindgen.rs

- [ ] Review issue [#1002](https://github.com/john-cd/rust_howto/issues/1002) in [later/crates/cats/development_tools_ffi/examples/c/cbindgen.rs](./later/crates/cats/development_tools_ffi/examples/c/cbindgen.rs#L48)

#### later/crates/cats/development_tools_ffi/examples/cpp/cxx.rs

- [ ] Review issue [#738](https://github.com/john-cd/rust_howto/issues/738) in [later/crates/cats/development_tools_ffi/examples/cpp/cxx.rs](./later/crates/cats/development_tools_ffi/examples/cpp/cxx.rs#L33)

#### later/crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs

- [ ] Review issue [#1080](https://github.com/john-cd/rust_howto/issues/1080) in [later/crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs](./later/crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs#L32)

#### later/crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs

- [ ] Review issue [#1028](https://github.com/john-cd/rust_howto/issues/1028) in [later/crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs](./later/crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs#L35)

#### later/crates/cats/development_tools_ffi/examples/java/jni.rs

- [ ] Review issue [#1029](https://github.com/john-cd/rust_howto/issues/1029) in [later/crates/cats/development_tools_ffi/examples/java/jni.rs](./later/crates/cats/development_tools_ffi/examples/java/jni.rs#L72)

#### later/crates/cats/development_tools_ffi/examples/lua/mlua2.rs

- [ ] Review issue [#1031](https://github.com/john-cd/rust_howto/issues/1031) in [later/crates/cats/development_tools_ffi/examples/lua/mlua2.rs](./later/crates/cats/development_tools_ffi/examples/lua/mlua2.rs#L55)

#### later/crates/cats/development_tools_ffi/examples/node/napi.rs

- [ ] Review issue [#1032](https://github.com/john-cd/rust_howto/issues/1032) in [later/crates/cats/development_tools_ffi/examples/node/napi.rs](./later/crates/cats/development_tools_ffi/examples/node/napi.rs#L47)

#### later/crates/cats/development_tools_ffi/examples/node/neon.rs

- [ ] Review issue [#1033](https://github.com/john-cd/rust_howto/issues/1033) in [later/crates/cats/development_tools_ffi/examples/node/neon.rs](./later/crates/cats/development_tools_ffi/examples/node/neon.rs#L31)

#### later/crates/cats/development_tools_ffi/examples/objc/objc2.rs

- [ ] Review issue [#1034](https://github.com/john-cd/rust_howto/issues/1034) in [later/crates/cats/development_tools_ffi/examples/objc/objc2.rs](./later/crates/cats/development_tools_ffi/examples/objc/objc2.rs#L34)

#### later/crates/cats/development_tools_ffi/examples/python/pyo3.rs

- [ ] Review issue [#78](https://github.com/john-cd/rust_howto/issues/78) in [later/crates/cats/development_tools_ffi/examples/python/pyo3.rs](./later/crates/cats/development_tools_ffi/examples/python/pyo3.rs#L73)

#### later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs

- [ ] Review issue [#996](https://github.com/john-cd/rust_howto/issues/996) in [later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs](./later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs#L64)

#### later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs

- [ ] Review issue [#1035](https://github.com/john-cd/rust_howto/issues/1035) in [later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs](./later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs#L42)

#### later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs

- [ ] Review issue [#1036](https://github.com/john-cd/rust_howto/issues/1036) in [later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs](./later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs#L41)

#### later/crates/cats/development_tools_ffi/examples/uniffi.rs

- [ ] Review issue [#1037](https://github.com/john-cd/rust_howto/issues/1037) in [later/crates/cats/development_tools_ffi/examples/uniffi.rs](./later/crates/cats/development_tools_ffi/examples/uniffi.rs#L25)

#### later/crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs

- [ ] Review issue [#744](https://github.com/john-cd/rust_howto/issues/744) in [later/crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs#L60)

#### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs

- [ ] Review issue [#739](https://github.com/john-cd/rust_howto/issues/739) in [later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs#L72)

#### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/main.rs

- [ ] Review issue [#1158](https://github.com/john-cd/rust_howto/issues/1158) in [later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/main.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/main.rs#L26)

#### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs

- [ ] Review issue [#741](https://github.com/john-cd/rust_howto/issues/741) in [later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs#L58)

#### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs

- [ ] Review issue [#742](https://github.com/john-cd/rust_howto/issues/742) in [later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs#L31)

#### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs

- [ ] Review issue [#743](https://github.com/john-cd/rust_howto/issues/743) in [later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs#L79)

#### later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs

- [ ] Review issue [#1157](https://github.com/john-cd/rust_howto/issues/1157) in [later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs](./later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs#L61)

#### later/crates/cats/embedded/examples/embassy/embassy.rs

- [ ] Review issue [#751](https://github.com/john-cd/rust_howto/issues/751) in [later/crates/cats/embedded/examples/embassy/embassy.rs](./later/crates/cats/embedded/examples/embassy/embassy.rs#L89)

#### later/crates/cats/emulators/examples/emulators/emulator.rs

- [ ] Review issue [#752](https://github.com/john-cd/rust_howto/issues/752) in [later/crates/cats/emulators/examples/emulators/emulator.rs](./later/crates/cats/emulators/examples/emulators/emulator.rs#L12)

#### later/crates/cats/finance/Cargo.toml

- [ ] Review issue [#1313](https://github.com/john-cd/rust_howto/issues/1313) in [later/crates/cats/finance/Cargo.toml](./later/crates/cats/finance/Cargo.toml#L19)

#### later/crates/cats/finance/examples/quant/rustquant.rs

- [ ] Review issue [#764](https://github.com/john-cd/rust_howto/issues/764) in [later/crates/cats/finance/examples/quant/rustquant.rs](./later/crates/cats/finance/examples/quant/rustquant.rs#L53)

#### later/crates/cats/game_development/examples/game_development/game_development1.rs

- [ ] Review issue [#766](https://github.com/john-cd/rust_howto/issues/766) in [later/crates/cats/game_development/examples/game_development/game_development1.rs](./later/crates/cats/game_development/examples/game_development/game_development1.rs#L12)

#### later/crates/cats/game_development/examples/game_development/glam.rs

- [ ] Review issue [#770](https://github.com/john-cd/rust_howto/issues/770) in [later/crates/cats/game_development/examples/game_development/glam.rs](./later/crates/cats/game_development/examples/game_development/glam.rs#L47)

#### later/crates/cats/game_engines/examples/game_engines/bevy.rs

- [ ] Review issue [#767](https://github.com/john-cd/rust_howto/issues/767) in [later/crates/cats/game_engines/examples/game_engines/bevy.rs](./later/crates/cats/game_engines/examples/game_engines/bevy.rs#L51)

#### later/crates/cats/game_engines/examples/game_engines/fyrox.rs

- [ ] Review issue [#768](https://github.com/john-cd/rust_howto/issues/768) in [later/crates/cats/game_engines/examples/game_engines/fyrox.rs](./later/crates/cats/game_engines/examples/game_engines/fyrox.rs#L73)

#### later/crates/cats/game_engines/examples/game_engines/ggez.rs

- [ ] Review issue [#769](https://github.com/john-cd/rust_howto/issues/769) in [later/crates/cats/game_engines/examples/game_engines/ggez.rs](./later/crates/cats/game_engines/examples/game_engines/ggez.rs#L71)

#### later/crates/cats/game_engines/examples/game_engines/macroquad.rs

- [ ] Review issue [#771](https://github.com/john-cd/rust_howto/issues/771) in [later/crates/cats/game_engines/examples/game_engines/macroquad.rs](./later/crates/cats/game_engines/examples/game_engines/macroquad.rs#L43)

#### later/crates/cats/gui/examples/2d_renderers/femtovg.rs

- [ ] Review issue [#777](https://github.com/john-cd/rust_howto/issues/777) in [later/crates/cats/gui/examples/2d_renderers/femtovg.rs](./later/crates/cats/gui/examples/2d_renderers/femtovg.rs#L136)

#### later/crates/cats/gui/examples/2d_renderers/main.rs

- [ ] Review issue [#1047](https://github.com/john-cd/rust_howto/issues/1047) in [later/crates/cats/gui/examples/2d_renderers/main.rs](./later/crates/cats/gui/examples/2d_renderers/main.rs#L8)

#### later/crates/cats/gui/examples/2d_renderers/skia_safe.rs

- [ ] Review issue [#786](https://github.com/john-cd/rust_howto/issues/786) in [later/crates/cats/gui/examples/2d_renderers/skia_safe.rs](./later/crates/cats/gui/examples/2d_renderers/skia_safe.rs#L189)

#### later/crates/cats/gui/examples/2d_renderers/vger.rs

- [ ] Review issue [#792](https://github.com/john-cd/rust_howto/issues/792) in [later/crates/cats/gui/examples/2d_renderers/vger.rs](./later/crates/cats/gui/examples/2d_renderers/vger.rs#L72)

#### later/crates/cats/gui/examples/2d_renderers/webrender.rs

- [ ] Review issue [#793](https://github.com/john-cd/rust_howto/issues/793) in [later/crates/cats/gui/examples/2d_renderers/webrender.rs](./later/crates/cats/gui/examples/2d_renderers/webrender.rs#L148)

#### later/crates/cats/gui/examples/clipboard/arboard.rs

- [ ] Review issue [#1155](https://github.com/john-cd/rust_howto/issues/1155) in [later/crates/cats/gui/examples/clipboard/arboard.rs](./later/crates/cats/gui/examples/clipboard/arboard.rs#L31)

#### later/crates/cats/gui/examples/clipboard/main.rs

- [ ] Review issue [#1048](https://github.com/john-cd/rust_howto/issues/1048) in [later/crates/cats/gui/examples/clipboard/main.rs](./later/crates/cats/gui/examples/clipboard/main.rs#L4)

#### later/crates/cats/gui/examples/file_dialogs/rfd.rs

- [ ] Review issue [#785](https://github.com/john-cd/rust_howto/issues/785) in [later/crates/cats/gui/examples/file_dialogs/rfd.rs](./later/crates/cats/gui/examples/file_dialogs/rfd.rs#L18)

#### later/crates/cats/gui/examples/gtk/gtk4.rs

- [ ] Review issue [#780](https://github.com/john-cd/rust_howto/issues/780) in [later/crates/cats/gui/examples/gtk/gtk4.rs](./later/crates/cats/gui/examples/gtk/gtk4.rs#L94)

#### later/crates/cats/gui/examples/gtk/relm4.rs

- [ ] Review issue [#784](https://github.com/john-cd/rust_howto/issues/784) in [later/crates/cats/gui/examples/gtk/relm4.rs](./later/crates/cats/gui/examples/gtk/relm4.rs#L211)

#### later/crates/cats/gui/examples/immediate_mode_gui/egui.rs

- [ ] Review issue [#776](https://github.com/john-cd/rust_howto/issues/776) in [later/crates/cats/gui/examples/immediate_mode_gui/egui.rs](./later/crates/cats/gui/examples/immediate_mode_gui/egui.rs#L45)

#### later/crates/cats/gui/examples/retained_mode_gui/floem.rs

- [ ] Review issue [#778](https://github.com/john-cd/rust_howto/issues/778) in [later/crates/cats/gui/examples/retained_mode_gui/floem.rs](./later/crates/cats/gui/examples/retained_mode_gui/floem.rs#L64)

#### later/crates/cats/gui/examples/retained_mode_gui/iced.rs

- [ ] Review issue [#781](https://github.com/john-cd/rust_howto/issues/781) in [later/crates/cats/gui/examples/retained_mode_gui/iced.rs](./later/crates/cats/gui/examples/retained_mode_gui/iced.rs#L86)

#### later/crates/cats/gui/examples/retained_mode_gui/main.rs

- [ ] Review issue [#1051](https://github.com/john-cd/rust_howto/issues/1051) in [later/crates/cats/gui/examples/retained_mode_gui/main.rs](./later/crates/cats/gui/examples/retained_mode_gui/main.rs#L15)

#### later/crates/cats/gui/examples/retained_mode_gui/slint.rs

- [ ] Review issue [#787](https://github.com/john-cd/rust_howto/issues/787) in [later/crates/cats/gui/examples/retained_mode_gui/slint.rs](./later/crates/cats/gui/examples/retained_mode_gui/slint.rs#L20)

#### later/crates/cats/gui/examples/retained_mode_gui/vizia.rs

- [ ] Review issue [#1052](https://github.com/john-cd/rust_howto/issues/1052) in [later/crates/cats/gui/examples/retained_mode_gui/vizia.rs](./later/crates/cats/gui/examples/retained_mode_gui/vizia.rs#L23)

#### later/crates/cats/gui/examples/retained_mode_gui/xilem.rs

- [ ] Review issue [#795](https://github.com/john-cd/rust_howto/issues/795) in [later/crates/cats/gui/examples/retained_mode_gui/xilem.rs](./later/crates/cats/gui/examples/retained_mode_gui/xilem.rs#L77)

#### later/crates/cats/gui/examples/text_layout/cosmic_text.rs

- [ ] Review issue [#774](https://github.com/john-cd/rust_howto/issues/774) in [later/crates/cats/gui/examples/text_layout/cosmic_text.rs](./later/crates/cats/gui/examples/text_layout/cosmic_text.rs#L53)

#### later/crates/cats/gui/examples/text_layout/parley.rs

- [ ] Review issue [#783](https://github.com/john-cd/rust_howto/issues/783) in [later/crates/cats/gui/examples/text_layout/parley.rs](./later/crates/cats/gui/examples/text_layout/parley.rs#L95)

#### later/crates/cats/gui/examples/ui_layout/morphorm.rs

- [ ] Review issue [#782](https://github.com/john-cd/rust_howto/issues/782) in [later/crates/cats/gui/examples/ui_layout/morphorm.rs](./later/crates/cats/gui/examples/ui_layout/morphorm.rs#L99)

#### later/crates/cats/gui/examples/ui_layout/taffy.rs

- [ ] Review issue [#788](https://github.com/john-cd/rust_howto/issues/788) in [later/crates/cats/gui/examples/ui_layout/taffy.rs](./later/crates/cats/gui/examples/ui_layout/taffy.rs#L130)

#### later/crates/cats/gui/examples/web/dioxus.rs

- [ ] Review issue [#775](https://github.com/john-cd/rust_howto/issues/775) in [later/crates/cats/gui/examples/web/dioxus.rs](./later/crates/cats/gui/examples/web/dioxus.rs#L33)

#### later/crates/cats/gui/examples/web/tauri/mod.rs

- [ ] Review issue [#790](https://github.com/john-cd/rust_howto/issues/790) in [later/crates/cats/gui/examples/web/tauri/mod.rs](./later/crates/cats/gui/examples/web/tauri/mod.rs#L70)

#### later/crates/cats/gui/examples/window_creation/baseview.rs

- [ ] Review issue [#1056](https://github.com/john-cd/rust_howto/issues/1056) in [later/crates/cats/gui/examples/window_creation/baseview.rs](./later/crates/cats/gui/examples/window_creation/baseview.rs#L23)

#### later/crates/cats/gui/examples/window_creation/tao.rs

- [ ] Review issue [#789](https://github.com/john-cd/rust_howto/issues/789) in [later/crates/cats/gui/examples/window_creation/tao.rs](./later/crates/cats/gui/examples/window_creation/tao.rs#L68)

#### later/crates/cats/internationalization/examples/internationalization/internationalization1.rs

- [ ] Review issue [#796](https://github.com/john-cd/rust_howto/issues/796) in [later/crates/cats/internationalization/examples/internationalization/internationalization1.rs](./later/crates/cats/internationalization/examples/internationalization/internationalization1.rs#L12)

#### later/crates/cats/localization/examples/localization/localization1.rs

- [ ] Review issue [#797](https://github.com/john-cd/rust_howto/issues/797) in [later/crates/cats/localization/examples/localization/localization1.rs](./later/crates/cats/localization/examples/localization/localization1.rs#L12)

#### later/crates/cats/multimedia/examples/multimedia/multimedia1.rs

- [ ] Review issue [#805](https://github.com/john-cd/rust_howto/issues/805) in [later/crates/cats/multimedia/examples/multimedia/multimedia1.rs](./later/crates/cats/multimedia/examples/multimedia/multimedia1.rs#L12)

#### later/crates/cats/multimedia_audio/examples/audio/audio.rs

- [ ] Review issue [#806](https://github.com/john-cd/rust_howto/issues/806) in [later/crates/cats/multimedia_audio/examples/audio/audio.rs](./later/crates/cats/multimedia_audio/examples/audio/audio.rs#L12)

#### later/crates/cats/multimedia_encoding/examples/encoding/encoding.rs

- [ ] Review issue [#807](https://github.com/john-cd/rust_howto/issues/807) in [later/crates/cats/multimedia_encoding/examples/encoding/encoding.rs](./later/crates/cats/multimedia_encoding/examples/encoding/encoding.rs#L12)

#### later/crates/cats/multimedia_images/examples/images/images.rs

- [ ] Review issue [#808](https://github.com/john-cd/rust_howto/issues/808) in [later/crates/cats/multimedia_images/examples/images/images.rs](./later/crates/cats/multimedia_images/examples/images/images.rs#L12)

#### later/crates/cats/multimedia_video/examples/video/video.rs

- [ ] Review issue [#809](https://github.com/john-cd/rust_howto/issues/809) in [later/crates/cats/multimedia_video/examples/video/video.rs](./later/crates/cats/multimedia_video/examples/video/video.rs#L12)

#### later/crates/cats/no_std/examples/no_std/no_std1.rs

- [ ] Review issue [#814](https://github.com/john-cd/rust_howto/issues/814) in [later/crates/cats/no_std/examples/no_std/no_std1.rs](./later/crates/cats/no_std/examples/no_std/no_std1.rs#L12)

#### later/crates/cats/no_std_no_alloc/examples/no_alloc/no_alloc.rs

- [ ] Review issue [#815](https://github.com/john-cd/rust_howto/issues/815) in [later/crates/cats/no_std_no_alloc/examples/no_alloc/no_alloc.rs](./later/crates/cats/no_std_no_alloc/examples/no_alloc/no_alloc.rs#L12)

#### later/crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs

- [ ] Review issue [#817](https://github.com/john-cd/rust_howto/issues/817) in [later/crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs](./later/crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs#L14)

#### later/crates/cats/os_linux_apis/examples/linux/linux.rs

- [ ] Review issue [#818](https://github.com/john-cd/rust_howto/issues/818) in [later/crates/cats/os_linux_apis/examples/linux/linux.rs](./later/crates/cats/os_linux_apis/examples/linux/linux.rs#L14)

#### later/crates/cats/os_macos_apis/examples/macos/macos.rs

- [ ] Review issue [#819](https://github.com/john-cd/rust_howto/issues/819) in [later/crates/cats/os_macos_apis/examples/macos/macos.rs](./later/crates/cats/os_macos_apis/examples/macos/macos.rs#L14)

#### later/crates/cats/rendering/examples/2d_raster_graphics/render.rs

- [ ] Review issue [#828](https://github.com/john-cd/rust_howto/issues/828) in [later/crates/cats/rendering/examples/2d_raster_graphics/render.rs](./later/crates/cats/rendering/examples/2d_raster_graphics/render.rs#L12)

#### later/crates/cats/rendering_data_formats/examples/data_formats/data_formats.rs

- [ ] Review issue [#829](https://github.com/john-cd/rust_howto/issues/829) in [later/crates/cats/rendering_data_formats/examples/data_formats/data_formats.rs](./later/crates/cats/rendering_data_formats/examples/data_formats/data_formats.rs#L12)

#### later/crates/cats/rendering_engine/examples/rendering_engines/render.rs

- [ ] Review issue [#830](https://github.com/john-cd/rust_howto/issues/830) in [later/crates/cats/rendering_engine/examples/rendering_engines/render.rs](./later/crates/cats/rendering_engine/examples/rendering_engines/render.rs#L12)

#### later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs

- [ ] Review issue [#772](https://github.com/john-cd/rust_howto/issues/772) in [later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs](./later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs#L156)

#### later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/graphics.rs

- [ ] Review issue [#831](https://github.com/john-cd/rust_howto/issues/831) in [later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/graphics.rs](./later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/graphics.rs#L12)

#### later/crates/cats/science/examples/ml/candle.rs

- [ ] Review issue [#835](https://github.com/john-cd/rust_howto/issues/835) in [later/crates/cats/science/examples/ml/candle.rs](./later/crates/cats/science/examples/ml/candle.rs#L112)

#### later/crates/cats/science/examples/ml/smartcore.rs

- [ ] Review issue [#837](https://github.com/john-cd/rust_howto/issues/837) in [later/crates/cats/science/examples/ml/smartcore.rs](./later/crates/cats/science/examples/ml/smartcore.rs#L79)

#### later/crates/cats/science_geo/examples/geo/geo.rs

- [ ] Review issue [#839](https://github.com/john-cd/rust_howto/issues/839) in [later/crates/cats/science_geo/examples/geo/geo.rs](./later/crates/cats/science_geo/examples/geo/geo.rs#L12)

#### later/crates/cats/science_neuroscience/examples/neuroscience/neuro.rs

- [ ] Review issue [#840](https://github.com/john-cd/rust_howto/issues/840) in [later/crates/cats/science_neuroscience/examples/neuroscience/neuro.rs](./later/crates/cats/science_neuroscience/examples/neuroscience/neuro.rs#L12)

#### later/crates/cats/science_robotics/Cargo.toml

- [ ] Review issue [#1441](https://github.com/john-cd/rust_howto/issues/1441) in [later/crates/cats/science_robotics/Cargo.toml](./later/crates/cats/science_robotics/Cargo.toml#L21)

#### later/crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs

- [ ] Review issue [#841](https://github.com/john-cd/rust_howto/issues/841) in [later/crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs](./later/crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs#L128)

#### later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs

- [ ] Review issue [#844](https://github.com/john-cd/rust_howto/issues/844) in [later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs](./later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs#L12)

#### later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs

- [ ] Review issue [#843](https://github.com/john-cd/rust_howto/issues/843) in [later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs](./later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs#L155)

#### later/crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs

- [ ] Review issue [#845](https://github.com/john-cd/rust_howto/issues/845) in [later/crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs](./later/crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs#L87)

#### later/crates/cats/simulation/examples/simulation/simulation1.rs

- [ ] Review issue [#846](https://github.com/john-cd/rust_howto/issues/846) in [later/crates/cats/simulation/examples/simulation/simulation1.rs](./later/crates/cats/simulation/examples/simulation/simulation1.rs#L12)

#### later/crates/cats/virtualization/Cargo.toml

- [ ] Review issue [#1263](https://github.com/john-cd/rust_howto/issues/1263) in [later/crates/cats/virtualization/Cargo.toml](./later/crates/cats/virtualization/Cargo.toml#L18)

#### later/crates/cats/virtualization/examples/virtualization/virtualization1.rs

- [ ] Review issue [#852](https://github.com/john-cd/rust_howto/issues/852) in [later/crates/cats/virtualization/examples/virtualization/virtualization1.rs](./later/crates/cats/virtualization/examples/virtualization/virtualization1.rs#L12)

#### later/crates/cats/visualization/examples/visualization/plotly.rs

- [ ] Review issue [#884](https://github.com/john-cd/rust_howto/issues/884) in [later/crates/cats/visualization/examples/visualization/plotly.rs](./later/crates/cats/visualization/examples/visualization/plotly.rs#L50)

#### later/crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs

- [ ] Review issue [#855](https://github.com/john-cd/rust_howto/issues/855) in [later/crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs](./later/crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs#L126)

#### later/crates/cats/wasm/examples/yew/yew.rs

- [ ] Review issue [#856](https://github.com/john-cd/rust_howto/issues/856) in [later/crates/cats/wasm/examples/yew/yew.rs](./later/crates/cats/wasm/examples/yew/yew.rs#L59)

#### later/crates/cats/web_programming_websocket/Cargo.toml

- [ ] Review issue [#1291](https://github.com/john-cd/rust_howto/issues/1291) in [later/crates/cats/web_programming_websocket/Cargo.toml](./later/crates/cats/web_programming_websocket/Cargo.toml#L22)

#### later/crates/cats/web_programming_websocket/examples/async_tungstenite.rs

- [ ] Review issue [#1058](https://github.com/john-cd/rust_howto/issues/1058) in [later/crates/cats/web_programming_websocket/examples/async_tungstenite.rs](./later/crates/cats/web_programming_websocket/examples/async_tungstenite.rs#L58)

#### later/crates/cats/web_programming_websocket/examples/tokio_tungstenite.rs

- [ ] Review issue [#1145](https://github.com/john-cd/rust_howto/issues/1145) in [later/crates/cats/web_programming_websocket/examples/tokio_tungstenite.rs](./later/crates/cats/web_programming_websocket/examples/tokio_tungstenite.rs#L57)

#### later/crates/other/examples/cloud/aws_lambda.rs

- [ ] Review issue [#878](https://github.com/john-cd/rust_howto/issues/878) in [later/crates/other/examples/cloud/aws_lambda.rs](./later/crates/other/examples/cloud/aws_lambda.rs#L62)

## Tooling, Scripts, and CI

### bk/scripts/book/mod.just

- [ ] [mdbook-utils refdefs](https://github.com/john-cd/rust_howto/issues/1237) ([bk/scripts/book/mod.just](./bk/scripts/book/mod.just#L30))
- [ ] [generate index of crates by category](https://github.com/john-cd/rust_howto/issues/1274) ([bk/scripts/book/mod.just](./bk/scripts/book/mod.just#L34))

### bk/scripts/deps/mod.just

- [ ] [review cargo deny](https://github.com/john-cd/rust_howto/issues/1275) ([bk/scripts/deps/mod.just](./bk/scripts/deps/mod.just#L23))

### bk/scripts/examples/convert_example_placeholders.sh

- [ ] [does not handle non-category examples](https://github.com/john-cd/rust_howto/issues/1257) ([bk/scripts/examples/convert_example_placeholders.sh](./bk/scripts/examples/convert_example_placeholders.sh#L21))

### bk/scripts/gh/mod.just

- [ ] Create an GH issue for each line in the book's Markdown files ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L28))
- [ ] prefix='^\s*\s*:?\s*' ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L35))
- [ ] for in $(sed -nE 's~'"${prefix}"'(.+)$~\1~pg' $file) ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L41))
- [ ] issue_url=$(gh issue create --title "${rel}: ${fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --label "markdown,auto" --assignee "@me") ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L43))
- [ ] =$( echo "$" | sed -E 's~[[]~\\[~g; s~[]]~\\]~g; s~([?()])~\\\1~g' ) ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L46))
- [ ] sed -E -i 's~'"${prefix}${}"'~['"${}"']('"${issue_url}"')~' $file ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L47))
- [ ] Create an GH issue for each line in .rs files (incl. code examples) ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L52))
- [ ] pattern='\s*:?\s*' ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L60))
- [ ] for in $(sed -nE 's~'"${prefix}${pattern}"'(.*)$~\2~pg' $file) ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L66))
- [ ] =$( echo "$" | sed -E 's~[[]~\\[~g; s~[]]~\\]~g; s~([?()])~\\\1~g' ) ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L68))
- [ ] echo ">${}<" ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L69))
- [ ] issue_url=$(gh issue create --title "${rel}: ${fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --label "code example,auto" --assignee "@me") ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L70))
- [ ] sed -E -i 's~'"${pattern}${}"'~['"${}"']('"${issue_url}"')~' $file ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L72))
- [ ] Create an GH issue for each line in other files (*.sh, *.toml, *.yaml...) ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L77))
- [ ] pattern='\s*:?\s*' ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L85))
- [ ] for in $(sed -nE 's~'"${prefix}${pattern}"'(.*)$~\3~p' $file) ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L91))
- [ ] =$( echo "$" | sed -E 's~[[]~\\[~g; s~[]]~\\]~g; s~([?()])~\\\1~g' ) ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L93))
- [ ] echo ">${}<" ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L94))
- [ ] issue_url=$(gh issue create --title "${rel}: ${fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --label "auto" --assignee "@me") ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L95))
- [ ] sed -E -i 's~'"${pattern}${}"'~['"${}"']('"${issue_url}"')~' $file ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L97))
- [ ] =$( echo "$str" | cut -d@ -f1 ) ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L116))
- [ ] if [ "$existing_title" != "${rel}: ${fix}" ] ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L120))
- [ ] echo "title: ${rel}: ${fix}" ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L125))
- [ ] gh issue edit "${id}" --title "${rel}: ${fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --add-assignee "@me" --add-label "auto" ([bk/scripts/gh/mod.just](./bk/scripts/gh/mod.just#L127))

### bk/scripts/links/mod.just

- [ ] [insert preproc directive instead of link. replace link to crates.io category search page to link to category main page.](https://github.com/john-cd/rust_howto/issues/1281) ([bk/scripts/links/mod.just](./bk/scripts/links/mod.just#L15))
- [ ] [fix - one link only; should we have crate landing pages instead of linking to the docs](https://github.com/john-cd/rust_howto/issues/1240)? ([bk/scripts/links/mod.just](./bk/scripts/links/mod.just#L21))
- [ ] [review suggest_links_for_bare_urls](https://github.com/john-cd/rust_howto/issues/1241) ([bk/scripts/links/mod.just](./bk/scripts/links/mod.just#L32))
- [ ] Resolve TODO/FIXME at line 38 ([bk/scripts/links/mod.just](./bk/scripts/links/mod.just#L38))

### bk/scripts/links/replace_inline_links.sh

- [ ] https://github.com/john-cd/rust_howto/issues/1375 ([bk/scripts/links/replace_inline_links.sh](./bk/scripts/links/replace_inline_links.sh#L13))
- [ ] [fix](https://github.com/john-cd/rust_howto/issues/1238) ([bk/scripts/links/replace_inline_links.sh](./bk/scripts/links/replace_inline_links.sh#L16))
- [ ] 1. do not convert links to GitHub issues e.g. https://github.com/john-cd/rust_howto/issues/1375 links ([bk/scripts/links/replace_inline_links.sh](./bk/scripts/links/replace_inline_links.sh#L17))

### bk/scripts/links/replace_inline_links2.sh

- [ ] https://doc.rust-lang.org/std/macro..html ([bk/scripts/links/replace_inline_links2.sh](./bk/scripts/links/replace_inline_links2.sh#L95))

### bk/scripts/links/suggest_links_for_bare_urls.sh

- [ ] [pass a var](https://github.com/john-cd/rust_howto/issues/1243) ([bk/scripts/links/suggest_links_for_bare_urls.sh](./bk/scripts/links/suggest_links_for_bare_urls.sh#L13))

### bk/scripts/precommit/mod.just

- [ ] [finish - install pre-commit or use cargo-husky](https://github.com/john-cd/rust_howto/issues/1245)? ([bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L21))
- [ ] [review](https://github.com/john-cd/rust_howto/issues/1246) ([bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L26))
- [ ] Fix auto-fixable lint issues in staged files ([bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L29))
- [ ] Install/update code automation: pre-commit ([bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L33))
- [ ] [move to Dockerfile](https://github.com/john-cd/rust_howto/issues/1247) ([bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L37))
- [ ] Setup/update pre-commit hooks (optional) ([bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L39))
- [ ] [review git town](https://github.com/john-cd/rust_howto/issues/1246) ([bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L45))

### bk/scripts/spelling/dictionary.txt

- [ ] Resolve TODO/FIXME at line 482 ([bk/scripts/spelling/dictionary.txt](./bk/scripts/spelling/dictionary.txt#L482))
- [ ] Resolve TODO/FIXME at line 1450 ([bk/scripts/spelling/dictionary.txt](./bk/scripts/spelling/dictionary.txt#L1450))
- [ ] Resolve TODO/FIXME at line 3433 ([bk/scripts/spelling/dictionary.txt](./bk/scripts/spelling/dictionary.txt#L3433))

### bk/scripts/spelling/spellcheck.sh

- [ ] Find more correct way to get line number ([bk/scripts/spelling/spellcheck.sh](./bk/scripts/spelling/spellcheck.sh#L84))

### bk/scripts/utils/mod.just

- [ ] [clarify mdbook-utils](https://github.com/john-cd/rust_howto/issues/1254) ([bk/scripts/utils/mod.just](./bk/scripts/utils/mod.just#L17))
- [ ] [clarify templ](https://github.com/john-cd/rust_howto/issues/1254) ([bk/scripts/utils/mod.just](./bk/scripts/utils/mod.just#L24))

### bk/scripts/wikilinks/dewikilink.sh

- [ ] [handle [[...]] without title](https://github.com/john-cd/rust_howto/issues/1244) ([bk/scripts/wikilinks/dewikilink.sh](./bk/scripts/wikilinks/dewikilink.sh#L9))

### playground/bits_and_pieces/buildinfo/build-info.rs

- [ ] [review / convert into example](https://github.com/john-cd/rust_howto/issues/1060)? ([playground/bits_and_pieces/buildinfo/build-info.rs](./playground/bits_and_pieces/buildinfo/build-info.rs#L49))

### playground/bits_and_pieces/github/workflow.yml

- [ ] [clean up](https://github.com/john-cd/rust_howto/issues/1268) ([playground/bits_and_pieces/github/workflow.yml](./playground/bits_and_pieces/github/workflow.yml#L2))
- [ ] [review https://github.com/actions-rust-lang/setup-rust-toolchain/tree/main](https://github.com/john-cd/rust_howto/issues/1269) ([playground/bits_and_pieces/github/workflow.yml](./playground/bits_and_pieces/github/workflow.yml#L43))
- [ ] [consider using https://github.com/marketplace/actions/rust-cache](https://github.com/john-cd/rust_howto/issues/1270) ([playground/bits_and_pieces/github/workflow.yml](./playground/bits_and_pieces/github/workflow.yml#L103))

### playground/bits_and_pieces/just/justfile

- [ ] [would like to use `bash` as the shell, when on Windows, rather than](https://github.com/john-cd/rust_howto/issues/1249) ([playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L9))
- [ ] [make [script] work](https://github.com/john-cd/rust_howto/issues/1250) ([playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L36))
- [ ] [make cypath conversion work](https://github.com/john-cd/rust_howto/issues/1251) ([playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L52))
- [ ] [convert Windows path to Cygwin path](https://github.com/john-cd/rust_howto/issues/1252) ([playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L62))
- [ ] [review https://github.com/casey/just/issues/2599](https://github.com/john-cd/rust_howto/issues/1253) ([playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L70))

### playground/bits_and_pieces/pest_parser/markdown_links.pest

- [ ] Example: https://github.com/john-cd/rust_howto/issues/602 ([playground/bits_and_pieces/pest_parser/markdown_links.pest](./playground/bits_and_pieces/pest_parser/markdown_links.pest#L56))
- [ ] LATER ([playground/bits_and_pieces/pest_parser/markdown_links.pest](./playground/bits_and_pieces/pest_parser/markdown_links.pest#L108))

### playground/bits_and_pieces/pre-commit/.pre-commit-config.yaml

- [ ] [revise git hooks](https://github.com/john-cd/rust_howto/issues/1264) ([playground/bits_and_pieces/pre-commit/.pre-commit-config.yaml](./playground/bits_and_pieces/pre-commit/.pre-commit-config.yaml#L12))

### playground/bits_and_pieces/templates/chapter/index.md.template

- [ ] write ([playground/bits_and_pieces/templates/chapter/index.md.template](./playground/bits_and_pieces/templates/chapter/index.md.template#L13))

### playground/bits_and_pieces/templates/chapter/subchapter.md.template

- [ ] write ([playground/bits_and_pieces/templates/chapter/subchapter.md.template](./playground/bits_and_pieces/templates/chapter/subchapter.md.template#L11))

### playground/bits_and_pieces/tool_lib/src/tera/mod.rs

- [ ] Resolve TODO/FIXME at line 65 ([playground/bits_and_pieces/tool_lib/src/tera/mod.rs](./playground/bits_and_pieces/tool_lib/src/tera/mod.rs#L65))

### playground/bits_and_pieces/walk_dir.rs

- [ ] [replace_in_file(path)?;](https://github.com/john-cd/rust_howto/issues/1439) ([playground/bits_and_pieces/walk_dir.rs](./playground/bits_and_pieces/walk_dir.rs#L34))

### tools/mdbook-scrub/Cargo.toml

- [ ] Resolve TODO/FIXME at line 18 ([tools/mdbook-scrub/Cargo.toml](./tools/mdbook-scrub/Cargo.toml#L18))

### tools/mdbook-scrub/src/regexes.rs

- [ ] [share RegexAndReplacement code with CLIs (core_lib)](https://github.com/john-cd/rust_howto/issues/1420) ([tools/mdbook-scrub/src/regexes.rs](./tools/mdbook-scrub/src/regexes.rs#L6))

### tools/mdbook-scrub/test_book/src/SUMMARY.md

- [ ] Category Directives (NOT IMPLEMENTED) ([tools/mdbook-scrub/test_book/src/SUMMARY.md](./tools/mdbook-scrub/test_book/src/SUMMARY.md#L7))
- [ ] Crate Directives (NOT IMPLEMENTED) ([tools/mdbook-scrub/test_book/src/SUMMARY.md](./tools/mdbook-scrub/test_book/src/SUMMARY.md#L12))

### tools/mdbook-scrub/test_book/src/category_badges.md

- [ ] NOT IMPLEMENTED ([tools/mdbook-scrub/test_book/src/category_badges.md](./tools/mdbook-scrub/test_book/src/category_badges.md#L3))

### tools/mdbook-scrub/test_book/src/category_links.md

- [ ] NOT IMPLEMENTED ([tools/mdbook-scrub/test_book/src/category_links.md](./tools/mdbook-scrub/test_book/src/category_links.md#L3))

### tools/mdbook-scrub/test_book/src/crate_badges.md

- [ ] NOT IMPLEMENTED ([tools/mdbook-scrub/test_book/src/crate_badges.md](./tools/mdbook-scrub/test_book/src/crate_badges.md#L3))

### tools/mdbook-scrub/test_book/src/crate_links.md

- [ ] NOT IMPLEMENTED ([tools/mdbook-scrub/test_book/src/crate_links.md](./tools/mdbook-scrub/test_book/src/crate_links.md#L3))

### tools/templ/src/main.rs

- [ ] Resolve TODO/FIXME at line 27 ([tools/templ/src/main.rs](./tools/templ/src/main.rs#L27))

### tools/tool_lib/Cargo.toml

- [ ] Resolve TODO/FIXME at line 26 ([tools/tool_lib/Cargo.toml](./tools/tool_lib/Cargo.toml#L26))

### tools/tool_lib/src/crates_io/info.rs

- [ ] [unit tests](https://github.com/john-cd/rust_howto/issues/1358) ([tools/tool_lib/src/crates_io/info.rs](./tools/tool_lib/src/crates_io/info.rs#L12))

### tools/tool_lib/src/crates_io/mod.rs

- [ ] Resolve TODO/FIXME at line 21 ([tools/tool_lib/src/crates_io/mod.rs](./tools/tool_lib/src/crates_io/mod.rs#L21))
- [ ] [unit tests; cleanup](https://github.com/john-cd/rust_howto/issues/1356) ([tools/tool_lib/src/crates_io/mod.rs](./tools/tool_lib/src/crates_io/mod.rs#L53))

### tools/tool_lib/src/read_write/ext.rs

- [ ] [file_prefix is still unstable.](https://github.com/john-cd/rust_howto/issues/902) ([tools/tool_lib/src/read_write/ext.rs](./tools/tool_lib/src/read_write/ext.rs#L72))

### tools/tool_lib/src/templates/mod.rs

- [ ] [finish; unit tests; tinytemplate vs tera?](https://github.com/john-cd/rust_howto/issues/1361) ([tools/tool_lib/src/templates/mod.rs](./tools/tool_lib/src/templates/mod.rs#L71))

### GitHub issue references

#### bk/scripts/book/mod.just

- [ ] Review issue [#1237](https://github.com/john-cd/rust_howto/issues/1237) in [bk/scripts/book/mod.just](./bk/scripts/book/mod.just#L30)
- [ ] Review issue [#1274](https://github.com/john-cd/rust_howto/issues/1274) in [bk/scripts/book/mod.just](./bk/scripts/book/mod.just#L34)

#### bk/scripts/deps/mod.just

- [ ] Review issue [#1275](https://github.com/john-cd/rust_howto/issues/1275) in [bk/scripts/deps/mod.just](./bk/scripts/deps/mod.just#L23)

#### bk/scripts/examples/convert_example_placeholders.sh

- [ ] Review issue [#1257](https://github.com/john-cd/rust_howto/issues/1257) in [bk/scripts/examples/convert_example_placeholders.sh](./bk/scripts/examples/convert_example_placeholders.sh#L21)

#### bk/scripts/links/mod.just

- [ ] Review issue [#1281](https://github.com/john-cd/rust_howto/issues/1281) in [bk/scripts/links/mod.just](./bk/scripts/links/mod.just#L15)
- [ ] Review issue [#1240](https://github.com/john-cd/rust_howto/issues/1240) in [bk/scripts/links/mod.just](./bk/scripts/links/mod.just#L21)
- [ ] Review issue [#1241](https://github.com/john-cd/rust_howto/issues/1241) in [bk/scripts/links/mod.just](./bk/scripts/links/mod.just#L32)

#### bk/scripts/links/replace_inline_links.sh

- [ ] Review issue [#1375](https://github.com/john-cd/rust_howto/issues/1375) in [bk/scripts/links/replace_inline_links.sh](./bk/scripts/links/replace_inline_links.sh#L13)
- [ ] Review issue [#1238](https://github.com/john-cd/rust_howto/issues/1238) in [bk/scripts/links/replace_inline_links.sh](./bk/scripts/links/replace_inline_links.sh#L16)
- [ ] Review issue [#1375](https://github.com/john-cd/rust_howto/issues/1375) in [bk/scripts/links/replace_inline_links.sh](./bk/scripts/links/replace_inline_links.sh#L17)

#### bk/scripts/links/suggest_links_for_bare_urls.sh

- [ ] Review issue [#1243](https://github.com/john-cd/rust_howto/issues/1243) in [bk/scripts/links/suggest_links_for_bare_urls.sh](./bk/scripts/links/suggest_links_for_bare_urls.sh#L13)

#### bk/scripts/precommit/mod.just

- [ ] Review issue [#1245](https://github.com/john-cd/rust_howto/issues/1245) in [bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L21)
- [ ] Review issue [#1246](https://github.com/john-cd/rust_howto/issues/1246) in [bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L26)
- [ ] Review issue [#1247](https://github.com/john-cd/rust_howto/issues/1247) in [bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L37)
- [ ] Review issue [#1246](https://github.com/john-cd/rust_howto/issues/1246) in [bk/scripts/precommit/mod.just](./bk/scripts/precommit/mod.just#L45)

#### bk/scripts/utils/mod.just

- [ ] Review issue [#1254](https://github.com/john-cd/rust_howto/issues/1254) in [bk/scripts/utils/mod.just](./bk/scripts/utils/mod.just#L17)
- [ ] Review issue [#1254](https://github.com/john-cd/rust_howto/issues/1254) in [bk/scripts/utils/mod.just](./bk/scripts/utils/mod.just#L24)

#### bk/scripts/wikilinks/dewikilink.sh

- [ ] Review issue [#1244](https://github.com/john-cd/rust_howto/issues/1244) in [bk/scripts/wikilinks/dewikilink.sh](./bk/scripts/wikilinks/dewikilink.sh#L9)

#### playground/bits_and_pieces/buildinfo/build-info.rs

- [ ] Review issue [#1060](https://github.com/john-cd/rust_howto/issues/1060) in [playground/bits_and_pieces/buildinfo/build-info.rs](./playground/bits_and_pieces/buildinfo/build-info.rs#L49)

#### playground/bits_and_pieces/github/workflow.yml

- [ ] Review issue [#1268](https://github.com/john-cd/rust_howto/issues/1268) in [playground/bits_and_pieces/github/workflow.yml](./playground/bits_and_pieces/github/workflow.yml#L2)
- [ ] Review issue [#1269](https://github.com/john-cd/rust_howto/issues/1269) in [playground/bits_and_pieces/github/workflow.yml](./playground/bits_and_pieces/github/workflow.yml#L43)
- [ ] Review issue [#1270](https://github.com/john-cd/rust_howto/issues/1270) in [playground/bits_and_pieces/github/workflow.yml](./playground/bits_and_pieces/github/workflow.yml#L103)

#### playground/bits_and_pieces/just/justfile

- [ ] Review issue [#1249](https://github.com/john-cd/rust_howto/issues/1249) in [playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L9)
- [ ] Review issue [#1250](https://github.com/john-cd/rust_howto/issues/1250) in [playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L36)
- [ ] Review issue [#1251](https://github.com/john-cd/rust_howto/issues/1251) in [playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L52)
- [ ] Review issue [#1252](https://github.com/john-cd/rust_howto/issues/1252) in [playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L62)
- [ ] Review issue [#1253](https://github.com/john-cd/rust_howto/issues/1253) in [playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L70)

#### playground/bits_and_pieces/pest_parser/markdown_links.pest

- [ ] Review issue [#602](https://github.com/john-cd/rust_howto/issues/602) in [playground/bits_and_pieces/pest_parser/markdown_links.pest](./playground/bits_and_pieces/pest_parser/markdown_links.pest#L56)

#### playground/bits_and_pieces/pre-commit/.pre-commit-config.yaml

- [ ] Review issue [#1264](https://github.com/john-cd/rust_howto/issues/1264) in [playground/bits_and_pieces/pre-commit/.pre-commit-config.yaml](./playground/bits_and_pieces/pre-commit/.pre-commit-config.yaml#L12)

#### playground/bits_and_pieces/walk_dir.rs

- [ ] Review issue [#1439](https://github.com/john-cd/rust_howto/issues/1439) in [playground/bits_and_pieces/walk_dir.rs](./playground/bits_and_pieces/walk_dir.rs#L34)

#### tools/mdbook-scrub/src/regexes.rs

- [ ] Review issue [#1420](https://github.com/john-cd/rust_howto/issues/1420) in [tools/mdbook-scrub/src/regexes.rs](./tools/mdbook-scrub/src/regexes.rs#L6)

#### tools/tool_lib/src/crates_io/info.rs

- [ ] Review issue [#1358](https://github.com/john-cd/rust_howto/issues/1358) in [tools/tool_lib/src/crates_io/info.rs](./tools/tool_lib/src/crates_io/info.rs#L12)

#### tools/tool_lib/src/crates_io/mod.rs

- [ ] Review issue [#1356](https://github.com/john-cd/rust_howto/issues/1356) in [tools/tool_lib/src/crates_io/mod.rs](./tools/tool_lib/src/crates_io/mod.rs#L53)

#### tools/tool_lib/src/read_write/ext.rs

- [ ] Review issue [#902](https://github.com/john-cd/rust_howto/issues/902) in [tools/tool_lib/src/read_write/ext.rs](./tools/tool_lib/src/read_write/ext.rs#L72)

#### tools/tool_lib/src/templates/mod.rs

- [ ] Review issue [#1361](https://github.com/john-cd/rust_howto/issues/1361) in [tools/tool_lib/src/templates/mod.rs](./tools/tool_lib/src/templates/mod.rs#L71)

## Repository and Legal

### LICENSE

- [ ] issue of using CC0 for software?? ([LICENSE](./LICENSE#L17))
- [ ] This project, by its very nature, includes code examples for hundreds of Rust crates, and uses ([LICENSE](./LICENSE#L29))
- [ ] Copyright and permission notices for Rust crates are found ([LICENSE](./LICENSE#L33))

### typ/README.md

- [ ] Resolve TODO/FIXME at line 3 ([typ/README.md](./typ/README.md#L3))

### GitHub issue references

#### TODO.md

- [ ] Review issue [#529](https://github.com/john-cd/rust_howto/issues/529) in [TODO.md](./TODO.md#L10)
- [ ] Review issue [#1058](https://github.com/john-cd/rust_howto/issues/1058) in [TODO.md](./TODO.md#L81)
- [ ] Review issue [#870](https://github.com/john-cd/rust_howto/issues/870) in [TODO.md](./TODO.md#L82)
- [ ] Review issue [#866](https://github.com/john-cd/rust_howto/issues/866) in [TODO.md](./TODO.md#L83)
- [ ] Review issue [#865](https://github.com/john-cd/rust_howto/issues/865) in [TODO.md](./TODO.md#L84)
- [ ] Review issue [#869](https://github.com/john-cd/rust_howto/issues/869) in [TODO.md](./TODO.md#L85)
- [ ] Review issue [#867](https://github.com/john-cd/rust_howto/issues/867) in [TODO.md](./TODO.md#L86)
- [ ] Review issue [#864](https://github.com/john-cd/rust_howto/issues/864) in [TODO.md](./TODO.md#L87)
- [ ] Review issue [#812](https://github.com/john-cd/rust_howto/issues/812) in [TODO.md](./TODO.md#L89)
- [ ] Review issue [#424](https://github.com/john-cd/rust_howto/issues/424) in [TODO.md](./TODO.md#L90)
- [ ] Review issue [#1019](https://github.com/john-cd/rust_howto/issues/1019) in [TODO.md](./TODO.md#L93)

## Other

### bk/master/crates_used_in_drafts_and_later_sections.txt

- [ ] cover the following crates in the book? ([bk/master/crates_used_in_drafts_and_later_sections.txt](./bk/master/crates_used_in_drafts_and_later_sections.txt#L1))

### devcontainer/Dockerfile

- [ ] [LATER https://lib.rs/crates/mdbook-open-on-gh add an "Edit this file on GitHub" link on the bottom of every page, linking directly to the source file.](https://github.com/john-cd/rust_howto/issues/1278) ([devcontainer/Dockerfile](./devcontainer/Dockerfile#L151))
- [ ] [LATER reconsider when we can reliably use cached docker images in GitHub Actions; slow and takes space](https://github.com/john-cd/rust_howto/issues/1280) ([devcontainer/Dockerfile](./devcontainer/Dockerfile#L265))

### devcontainer/ci.sh

- [ ] [add cargo plugins to CI script](https://github.com/john-cd/rust_howto/issues/1277) ([devcontainer/ci.sh](./devcontainer/ci.sh#L22))

### devcontainer/compose-ci.yaml

- [ ] [make gha caching work in CI workflow](https://github.com/john-cd/rust_howto/issues/1271) ([devcontainer/compose-ci.yaml](./devcontainer/compose-ci.yaml#L19))

### xmpl/cancelable/Cargo.toml

- [ ] Resolve TODO/FIXME at line 20 ([xmpl/cancelable/Cargo.toml](./xmpl/cancelable/Cargo.toml#L20))

### xmpl/clap_builder_xmpl/src/cli/args.rs

- [ ] [fix test](https://github.com/john-cd/rust_howto/issues/895) ([xmpl/clap_builder_xmpl/src/cli/args.rs](./xmpl/clap_builder_xmpl/src/cli/args.rs#L203))

### GitHub issue references

#### devcontainer/Dockerfile

- [ ] Review issue [#1278](https://github.com/john-cd/rust_howto/issues/1278) in [devcontainer/Dockerfile](./devcontainer/Dockerfile#L151)
- [ ] Review issue [#1280](https://github.com/john-cd/rust_howto/issues/1280) in [devcontainer/Dockerfile](./devcontainer/Dockerfile#L265)

#### devcontainer/ci.sh

- [ ] Review issue [#1277](https://github.com/john-cd/rust_howto/issues/1277) in [devcontainer/ci.sh](./devcontainer/ci.sh#L22)

#### devcontainer/compose-ci.yaml

- [ ] Review issue [#1271](https://github.com/john-cd/rust_howto/issues/1271) in [devcontainer/compose-ci.yaml](./devcontainer/compose-ci.yaml#L19)

#### xmpl/clap_builder_xmpl/src/cli/args.rs

- [ ] Review issue [#895](https://github.com/john-cd/rust_howto/issues/895) in [xmpl/clap_builder_xmpl/src/cli/args.rs](./xmpl/clap_builder_xmpl/src/cli/args.rs#L203)
