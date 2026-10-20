# TODOs

> NOTE: This TODO list was regenerated from repository TODO/FIXME markers and GitHub issue references across the codebase, excluding mdbook-utils. Priorities and `[?]` review markers were reassigned heuristically, with `bk/` tasks ranked above `later/` tasks.

- Total TODO/FIXME markers scanned: 85
- Total unique files with TODO/FIXME markers: 68
- Total referenced GitHub issues tracked: 522

## Top-level TODO file tasks

### General

- [ ] P0 review all local example changes
- [ ] P0 merge the pull requests

- [ ] P1 debug dev container
- [?] P1 fix windows install script
- [?] P1 Make `just` commands fully work on Windows. consider cargo make / xtask instead of just?
- [ ] P2 consolidate GitHub workflows (use Docker always)

- [ ] P1 review and close all [?] tasks
- [ ] P2 review changes to mdbook-utils
- [ ] P2 move book tooling to the main monorepo as a git submodule?
- [ ] P2 review all changed examples
- [ ] P3 Go through VS Code bookmarks - WIP.
- [ ] P3 finish to review chrome bookmarks and include in text.

## WRITING

- [ ] P2 polish stack-allocated
- [?] P2 polish directory_traversal
- [ ] P2 polish hashing + algorithms
- [?] P2 polish [contributing](bk/src/appendices/contributing/index.md) [#529](https://github.com/john-cd/rust_howto/issues/529)

## LOGISTICS

- [?] P2 License / legal rethink.
- [ ] P2 Manually insert rest of cross-links between pages?
- [ ] P2 finish to add RBE link [#29](https://github.com/john-cd/rust_howto/issues/29).
- [ ] P2 Indices.
- [ ] P2 Update recipe tables: improve scripts that generate links / update recipe tables.
- [ ] P2 Additional Links / Content: use/move the rest of links in TOREVIEW.md.
- [?] P2 make a plan to use pandoc / typst instead of mdbook. review typ\pandoc-typst-plan.md
- [ ] P2 kindle create. [gumroad~website][gumroad~website] [blog~write-a-book-with-markdown] [write-book-with-obsidian] [book-creation-with-pandoc-and-markdown]
- [ ] P2 Scrub any left-over {{#example ...}}, {{#crate ...}}... etc and warn. WIP.
- [ ] P2 Scrub links to hidden pages instead of having to comment e.g. [p~cross-platform]: # "../../other/cross-platform/index.md".
- [ ] P2 Crate link.
- [ ] P2 Crate badge - WIP link tool.
- [ ] P2 Category link?
- [ ] P2 Category badges.
- [ ] P2 Crate blocks.
- [ ] P2 Recipe table.
- [ ] P2 crate-indices WIP.
- [ ] P2 autogen - in playground.
- [ ] P2 links.
- [ ] P2 templ.
- [ ] P2 tool_lib.
- [ ] P2 document
- [ ] P2 add tests
- [ ] P2 consolidate CLIs.
- [ ] P2 finish new tools in book_tooling
- [ ] P2 drafts: Finish
- [ ] P2 review drafts section for what I missed.
- [ ] P2 move categories back to src, after final review.
- [ ] P2 `other` section - move what's ready.
- [ ] P2 Move mdbook-utils repo ?
- [ ] P2 Setup bacon
- [ ] P2 Add git hooks to automate formatting / clippy check / fix before commit
- [ ] P2 Increase speed of CI build (Linux)
- [ ] P2 Try to move target and/or /usr/local/cargo to a docker volume instead of docker container FS
- [ ] P2 Measure speed metric: `time dd if=/dev/zero of=test.dat bs=1024 count=100000.`
- [ ] P2 Writes are fast when writing to container FS, volume, but slow on bind mount (to Windows). Need to test on Linux.
- [ ] P2 Try to store `target` folder and/or `/usr/local/cargo` in a tmpfs? `/usr/local/cargo` is small enough to fit in memory (16GB max). [how-to-use-tmpfs-and-its-functions-in-docker][how-to-use-tmpfs-and-its-functions-in-docker]
- [ ] P2 Make gha caching work on CI. Try local / inline caching as well. [optimize-docker-builds-github-actions-cache][optimize-docker-builds-github-actions-cache]
- [ ] P2 Try to build directly on the host, not in a container? Write a `.yml` build workflow file for Linux.
- [ ] P2 Free further space on CI runner. Already using the `free-disk-space-ubuntu` action. [2875][2875] [free-disk-space-ubuntu][free-disk-space-ubuntu]
- [ ] P1 Create CI build / tests on Windows

- [ ] P2 CI build on MacOS too? Create a .yml file for MacOS build on GitHub runner? Add build job to main workflow.
- [ ] P2 Fix .github issue templates
- [ ] P2 content/Drafts
- [ ] P2 Crate Integration
- [ ] P2 sccache for dev container setup?

## mdbook-utils

- [ ] P3 Address all TODO comments in mdbook-utils codebase
- [ ] P2 Document all modules, functions, and structs in mdbook-utils
- [ ] P3 Generate `categories.md` stub in mdbook-utils

## HUMAN REVIEW

- [?] P3 [Web] Migrate to modern WebSocket echo service [async_tungstenite.rs] ([#1058](https://github.com/john-cd/rust_howto/issues/1058))
- [?] P1 [Web] Implement Tonic gRPC example and server boilerplate [tonic.rs](bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs) ([#870](https://github.com/john-cd/rust_howto/issues/870))
- [?] P1 [Web] Update Hyper server example to 1.0 API and implement routing [hyper_server.rs](bk/crates/cats/web_programming_http_server/examples/hyper_server.rs) ([#866](https://github.com/john-cd/rust_howto/issues/866))
- [?] P1 [Web] Implement Axum routing, state, and review time limits [axum.rs](bk/crates/cats/web_programming_http_server/examples/axum.rs) ([#865](https://github.com/john-cd/rust_howto/issues/865))
- [?] P1 [Web] Implement Rocket Hello World and route mounting [rocket.rs](bk/crates/cats/web_programming_http_server/examples/rocket.rs) ([#869](https://github.com/john-cd/rust_howto/issues/869))
- [?] P1 [Web] Implement Leptos reactive counter and view mounting [leptos.rs](bk/crates/cats/web_programming_http_server/examples/leptos.rs) ([#867](https://github.com/john-cd/rust_howto/issues/867))
- [?] P1 [Web] Implement Async-GraphQL schema with Axum integration [async_graphql.rs](bk/crates/cats/web_programming_http_server/examples/async_graphql.rs) ([#864](https://github.com/john-cd/rust_howto/issues/864))
- [?] P1 [Web] Implement Actix-Web network requirement test [actix_web.rs](bk/crates/cats/web_programming_http_server/examples/actix_web.rs)
- [?] P1 [Network] Finalize Pingora reverse proxy implementation [pingora.rs](bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs) ([#812](https://github.com/john-cd/rust_howto/issues/812))
- [?] P1 [Network] Complete comprehensive reverse proxy section for Pingora and Rathole [reverse_proxy.md](bk/drafts/categories/network-programming/reverse_proxy.md) ([#424](https://github.com/john-cd/rust_howto/issues/424))
- [?] P1 [Database] Fix heavy test orchestration for Tiberius MSSQL [tiberius.rs](bk/crates/cats/database/examples/mssql/tiberius.rs) ([#1019](https://github.com/john-cd/rust_howto/issues/1019))
- [?] P1 [Database] Implement PostgreSQL aggregation examples [aggregate_data.rs](bk/crates/cats/database/examples/postgres/aggregate_data.rs)
- [?] P2 review cancelable example in `xmpl/`
- [?] P2 move WIP examples from `playground/`
- [?] P2 Clean up playground crate

- [?] P3 [GUI] Implement Winit window creation example [winit.rs](later/crates/cats/gui/examples/window_creation/winit.rs)
- [?] P2 [Parser] Implement `DocumentParser` trait and URL parsing 1427.md ([#1427](https://github.com/john-cd/rust_howto/issues/1427))
- [?] P3 [Test] Implement AFL.rs fuzzing target and panic discovery afl.rs ([#748](https://github.com/john-cd/rust_howto/issues/748))
- [?] P3 [Memory] Review and finalize lazy initialization examples lazy_static.rs ([#939](https://github.com/john-cd/rust_howto/issues/939))
- [?] P3 [Async] Expand stream examples with concurrent processing streams.md ([#645](https://github.com/john-cd/rust_howto/issues/645))

## Main Book and Published Content

### bk/crates/cats/asynchronous/examples/mio.rs

- [ ] P2 add to a chapter on asynchronous programming with mio ([bk/crates/cats/asynchronous/examples/mio.rs](./bk/crates/cats/asynchronous/examples/mio.rs#L47))

### bk/crates/cats/asynchronous/examples/smol.rs

- [ ] P2 add to a chapter on asynchronous programming with smol ([bk/crates/cats/asynchronous/examples/smol.rs](./bk/crates/cats/asynchronous/examples/smol.rs#L41))

### bk/crates/cats/asynchronous/examples/streams/streams2.rs

- [ ] P2 finish; asynchronous/streams.md: add more. streams2.rs is noplayground because it requires a network. rewrite ([#645](https://github.com/john-cd/rust_howto/issues/645)) ([bk/crates/cats/asynchronous/examples/streams/streams2.rs](./bk/crates/cats/asynchronous/examples/streams/streams2.rs#L80))

### bk/crates/cats/caching/examples/in_memory_cache/cached.rs

- [?] P2 review further ([#1354](https://github.com/john-cd/rust_howto/issues/1354)) ([bk/crates/cats/caching/examples/in_memory_cache/cached.rs](./bk/crates/cats/caching/examples/in_memory_cache/cached.rs#L73))

### bk/crates/cats/caching/examples/in_memory_cache/moka.rs

- [ ] P2 See also examples in <https://docs.rs/crate/moka/0.12.10> ([bk/crates/cats/caching/examples/in_memory_cache/moka.rs](./bk/crates/cats/caching/examples/in_memory_cache/moka.rs#L65))

### bk/crates/cats/command_line_interface/examples/ansi_term/colored.rs

- [ ] P2 add to a chapter on command line interfaces with colored ([bk/crates/cats/command_line_interface/examples/ansi_term/colored.rs](./bk/crates/cats/command_line_interface/examples/ansi_term/colored.rs#L32))

### bk/crates/cats/command_line_interface/examples/arguments/clap_complete.rs

- [ ] P2 add to a chapter on command line interfaces with clap_complete ([bk/crates/cats/command_line_interface/examples/arguments/clap_complete.rs](./bk/crates/cats/command_line_interface/examples/arguments/clap_complete.rs#L53))

### bk/crates/cats/compression/examples/compression/flate2.rs

- [ ] P2 read / write a file ([#1009](https://github.com/john-cd/rust_howto/issues/1009)) ([bk/crates/cats/compression/examples/compression/flate2.rs](./bk/crates/cats/compression/examples/compression/flate2.rs#L48))

### bk/crates/cats/compression/examples/tar/tar_compress.rs

- [ ] P2 review; tar_compress.rs is noplayground - fix? ([#255](https://github.com/john-cd/rust_howto/issues/255)) ([bk/crates/cats/compression/examples/tar/tar_compress.rs](./bk/crates/cats/compression/examples/tar/tar_compress.rs#L46))

### bk/crates/cats/compression/examples/tar/tar_decompress.rs

- [ ] P2 review; tar_decompress.rs is noplayground - fix? ([#254](https://github.com/john-cd/rust_howto/issues/254)) ([bk/crates/cats/compression/examples/tar/tar_decompress.rs](./bk/crates/cats/compression/examples/tar/tar_decompress.rs#L28))

### bk/crates/cats/compression/examples/tar/tar_strip_prefix.rs

- [ ] P2 review; tar_strip_prefix.rs is noplayground - fix? ([#256](https://github.com/john-cd/rust_howto/issues/256)) ([bk/crates/cats/compression/examples/tar/tar_strip_prefix.rs](./bk/crates/cats/compression/examples/tar/tar_strip_prefix.rs#L110))

### bk/crates/cats/concurrency/examples/actors/actix.rs

- [ ] P2 finish; test fails: `spawn_local` called from outside of a `task::LocalSet` or LocalRuntime ([#682](https://github.com/john-cd/rust_howto/issues/682)) ([bk/crates/cats/concurrency/examples/actors/actix.rs](./bk/crates/cats/concurrency/examples/actors/actix.rs#L72))

### bk/crates/cats/concurrency/examples/actors/actors.rs

- [ ] P2 finish ([#1011](https://github.com/john-cd/rust_howto/issues/1011)) ([bk/crates/cats/concurrency/examples/actors/actors.rs](./bk/crates/cats/concurrency/examples/actors/actors.rs#L51))

### bk/crates/cats/concurrency/examples/actors/main.rs

- [?] P2 review if this should be gated on a feature instead of the OS ([bk/crates/cats/concurrency/examples/actors/main.rs](./bk/crates/cats/concurrency/examples/actors/main.rs#L26))
- [?] P2 review if this should be gated on a feature instead of the OS ([bk/crates/cats/concurrency/examples/actors/main.rs](./bk/crates/cats/concurrency/examples/actors/main.rs#L29))

### bk/crates/cats/concurrency/examples/actors/riker.rs

- [ ] P2 add to a chapter on actors with riker ([bk/crates/cats/concurrency/examples/actors/riker.rs](./bk/crates/cats/concurrency/examples/actors/riker.rs#L45))

### bk/crates/cats/concurrency/examples/actors/stakker.rs

- [ ] P2 add to a chapter on actors with stakker ([bk/crates/cats/concurrency/examples/actors/stakker.rs](./bk/crates/cats/concurrency/examples/actors/stakker.rs#L70))

### bk/crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs

- [?] P2 review https://github.com/jonhoo/flurry/blob/main/benches/flurry_dashmap.rs ([#1152](https://github.com/john-cd/rust_howto/issues/1152)) ([bk/crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs](./bk/crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs#L121))

### bk/crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs

- [ ] P2 review; rayon_thumbnails: address the need for test jpg data_parallelism: rayon_thumbnails.rs is noplayground - linking with cc ([#261](https://github.com/john-cd/rust_howto/issues/261)) ([bk/crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs](./bk/crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs#L107))

### bk/crates/cats/concurrency/examples/shared_state/global_mut_state.rs

- [?] P2 review lazy_constant.rs and lazy_static.rs and global_mut_state.rs ([#939](https://github.com/john-cd/rust_howto/issues/939)) ([bk/crates/cats/concurrency/examples/shared_state/global_mut_state.rs](./bk/crates/cats/concurrency/examples/shared_state/global_mut_state.rs#L58))

### bk/crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs

- [?] P2 review; threadpool_fractal.rs is noplayground - linking with cc ([#268](https://github.com/john-cd/rust_howto/issues/268)) ([bk/crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs](./bk/crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs#L139))

### bk/crates/cats/config/examples/configuration/confy.rs

- [?] P2 review ([#148](https://github.com/john-cd/rust_howto/issues/148)) ([bk/crates/cats/config/examples/configuration/confy.rs](./bk/crates/cats/config/examples/configuration/confy.rs#L46))

### bk/crates/cats/config/examples/env/envy.rs

- [ ] P2 use Mutex ([bk/crates/cats/config/examples/env/envy.rs](./bk/crates/cats/config/examples/env/envy.rs#L40))

### bk/crates/cats/cryptography/Cargo.toml

- [ ] P2 was pinned because of ed25519 ([bk/crates/cats/cryptography/Cargo.toml](./bk/crates/cats/cryptography/Cargo.toml#L41))
- [ ] P2 try upgrade from 0.6.4 to 0.10.1+ ([bk/crates/cats/cryptography/Cargo.toml](./bk/crates/cats/cryptography/Cargo.toml#L42))

### bk/crates/cats/cryptography/examples/certs/pem_rfc7468.rs

- [ ] P2 finish ([#697](https://github.com/john-cd/rust_howto/issues/697)) ([bk/crates/cats/cryptography/examples/certs/pem_rfc7468.rs](./bk/crates/cats/cryptography/examples/certs/pem_rfc7468.rs#L40))

### bk/crates/cats/cryptography/examples/certs/x509_cert.rs

- [ ] P2 finish ([#705](https://github.com/john-cd/rust_howto/issues/705)) ([bk/crates/cats/cryptography/examples/certs/x509_cert.rs](./bk/crates/cats/cryptography/examples/certs/x509_cert.rs#L39))

### bk/crates/cats/cryptography/examples/sign/ecdsa.rs

- [ ] P2 finish ([#693](https://github.com/john-cd/rust_howto/issues/693)) ([bk/crates/cats/cryptography/examples/sign/ecdsa.rs](./bk/crates/cats/cryptography/examples/sign/ecdsa.rs#L42))

### bk/crates/cats/cryptography/examples/sign/ed25519.rs

- [ ] P2 finish ([#1084](https://github.com/john-cd/rust_howto/issues/1084)) ([bk/crates/cats/cryptography/examples/sign/ed25519.rs](./bk/crates/cats/cryptography/examples/sign/ed25519.rs#L38))

### bk/crates/cats/cryptography/examples/tls/native_tls.rs

- [ ] P2 finish ([#696](https://github.com/john-cd/rust_howto/issues/696)) ([bk/crates/cats/cryptography/examples/tls/native_tls.rs](./bk/crates/cats/cryptography/examples/tls/native_tls.rs#L31))

### bk/crates/cats/cryptography/examples/tls/rustls.rs

- [?] P2 review tokio-rustls; need full integration test; document further; review https://github.com/rustls/rustls/tree/main/examples ([#700](https://github.com/john-cd/rust_howto/issues/700)) ([bk/crates/cats/cryptography/examples/tls/rustls.rs](./bk/crates/cats/cryptography/examples/tls/rustls.rs#L45))

### bk/crates/cats/data_structures/Cargo.toml

- [ ] P2 pinned elsewhere to 0.7.17 for postcard ([bk/crates/cats/data_structures/Cargo.toml](./bk/crates/cats/data_structures/Cargo.toml#L30))

### bk/crates/cats/database/examples/amqp/lapin.rs

- [ ] P2 replace by config in `compose*.yaml` files ([bk/crates/cats/database/examples/amqp/lapin.rs](./bk/crates/cats/database/examples/amqp/lapin.rs#L123))

### bk/crates/cats/database/examples/cassandra/cassandra_protocol.rs

- [ ] P2 write; see also https://docs.rs/cassandra-protocol/latest/cassandra_protocol/index.html ([#1016](https://github.com/john-cd/rust_howto/issues/1016)) ([bk/crates/cats/database/examples/cassandra/cassandra_protocol.rs](./bk/crates/cats/database/examples/cassandra/cassandra_protocol.rs#L120))

### bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs

- [ ] P2 finish; see also https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/crud_operations.rs ([#1017](https://github.com/john-cd/rust_howto/issues/1017)) ([bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs](./bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs#L148))

### bk/crates/cats/database/examples/cassandra/main.rs

- [ ] P2 move to run() ([bk/crates/cats/database/examples/cassandra/main.rs](./bk/crates/cats/database/examples/cassandra/main.rs#L40))
- [ ] P2 handle error gracefully ([bk/crates/cats/database/examples/cassandra/main.rs](./bk/crates/cats/database/examples/cassandra/main.rs#L41))

### bk/crates/cats/database/examples/connection_pool/deadpool2.rs

- [ ] P2 finish; need heavy test ([#46](https://github.com/john-cd/rust_howto/issues/46)) ([bk/crates/cats/database/examples/connection_pool/deadpool2.rs](./bk/crates/cats/database/examples/connection_pool/deadpool2.rs#L82))

### bk/crates/cats/database/examples/mssql/tiberius.rs

- [ ] P2 fix heavy test ([#1019](https://github.com/john-cd/rust_howto/issues/1019)) ([bk/crates/cats/database/examples/mssql/tiberius.rs](./bk/crates/cats/database/examples/mssql/tiberius.rs#L94))

### bk/crates/cats/database/examples/nosql/redis.rs

- [?] P2 review ? ([#1161](https://github.com/john-cd/rust_howto/issues/1161)) ([bk/crates/cats/database/examples/nosql/redis.rs](./bk/crates/cats/database/examples/nosql/redis.rs#L86))

### bk/crates/cats/database/examples/oracle/oracle.rs

- [ ] P2 Troubleshooting "DPI-1047: Cannot locate a 64-bit Oracle Client library" ([bk/crates/cats/database/examples/oracle/oracle.rs](./bk/crates/cats/database/examples/oracle/oracle.rs#L74))

### bk/crates/cats/database/examples/oracle/sibyl.rs

- [ ] P2 To use Sibyl, you need to download the appropriate Instant Client ([bk/crates/cats/database/examples/oracle/sibyl.rs](./bk/crates/cats/database/examples/oracle/sibyl.rs#L65))

### bk/crates/cats/database/examples/postgres/aggregate_data.rs

- [?] P2 review ([#1162](https://github.com/john-cd/rust_howto/issues/1162)) ([bk/crates/cats/database/examples/postgres/aggregate_data.rs](./bk/crates/cats/database/examples/postgres/aggregate_data.rs#L66))

### bk/crates/cats/database/examples/postgres/cornucopia.rs

- [ ] P2 finish ([#708](https://github.com/john-cd/rust_howto/issues/708)) ([bk/crates/cats/database/examples/postgres/cornucopia.rs](./bk/crates/cats/database/examples/postgres/cornucopia.rs#L62))

### bk/crates/cats/database/examples/postgres/main.rs

- [?] P2 review ([#713](https://github.com/john-cd/rust_howto/issues/713)) ([bk/crates/cats/database/examples/postgres/main.rs](./bk/crates/cats/database/examples/postgres/main.rs#L106))

### bk/crates/cats/database/examples/postgres/tokio_postgres.rs

- [ ] P2 finish need heavy test ([#719](https://github.com/john-cd/rust_howto/issues/719)) ([bk/crates/cats/database/examples/postgres/tokio_postgres.rs](./bk/crates/cats/database/examples/postgres/tokio_postgres.rs#L102))

### bk/crates/cats/database/examples/query_builders_orms/diesel1.rs

- [ ] P2 finish ([#709](https://github.com/john-cd/rust_howto/issues/709)) ([bk/crates/cats/database/examples/query_builders_orms/diesel1.rs](./bk/crates/cats/database/examples/query_builders_orms/diesel1.rs#L63))

### bk/crates/cats/database/examples/query_builders_orms/sea_orm.rs

- [ ] P2 finish ([#715](https://github.com/john-cd/rust_howto/issues/715)) ([bk/crates/cats/database/examples/query_builders_orms/sea_orm.rs](./bk/crates/cats/database/examples/query_builders_orms/sea_orm.rs#L66))

### bk/crates/cats/database/examples/query_builders_orms/seaography.rs

- [ ] P2 Add an example of using the generated code from Seaography to interact with the database. ([bk/crates/cats/database/examples/query_builders_orms/seaography.rs](./bk/crates/cats/database/examples/query_builders_orms/seaography.rs#L23))

### bk/crates/cats/database/examples/search/elasticsearch.rs

- [ ] P2 review fix heavy test; secure the connection ([#710](https://github.com/john-cd/rust_howto/issues/710)) ([bk/crates/cats/database/examples/search/elasticsearch.rs](./bk/crates/cats/database/examples/search/elasticsearch.rs#L163))

### bk/crates/cats/database/examples/sqlite/transactions.rs

- [ ] P2 main().unwrap(); ([bk/crates/cats/database/examples/sqlite/transactions.rs](./bk/crates/cats/database/examples/sqlite/transactions.rs#L61))

### bk/crates/cats/database_implementations/examples/databases/surrealdb.rs

- [?] P2 review https://surrealdb.com/docs ([#1148](https://github.com/john-cd/rust_howto/issues/1148)) ([bk/crates/cats/database_implementations/examples/databases/surrealdb.rs](./bk/crates/cats/database_implementations/examples/databases/surrealdb.rs#L108))

### bk/crates/cats/date_and_time/examples/humantime.rs

- [ ] P2 add to a chapter on date and time ([bk/crates/cats/date_and_time/examples/humantime.rs](./bk/crates/cats/date_and_time/examples/humantime.rs#L44))

### bk/crates/cats/development_tools/examples/semver/semver_command.rs

- [ ] P1 make work on Windows ([bk/crates/cats/development_tools/examples/semver/semver_command.rs](./bk/crates/cats/development_tools/examples/semver/semver_command.rs#L2))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs

- [ ] P2 FIX finish; deal with cc ([#897](https://github.com/john-cd/rust_howto/issues/897)) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs#L37))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs

- [ ] P2 finish ([#898](https://github.com/john-cd/rust_howto/issues/898)) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs#L27))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs

- [ ] P2 finish; deal with cc ([#899](https://github.com/john-cd/rust_howto/issues/899)) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs#L38))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs

- [ ] P2 finish; deal with extern ([#900](https://github.com/john-cd/rust_howto/issues/900)) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs#L49))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs

- [ ] P2 finish; deal wth cc ([#1000](https://github.com/john-cd/rust_howto/issues/1000)) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs#L37))

### bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs

- [ ] P2 finish; deal with extern ([#901](https://github.com/john-cd/rust_howto/issues/901)) ([bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs](./bk/crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs#L30))

### bk/crates/cats/development_tools_debugging/examples/log/log_env_variable.rs

- [?] P2 review; test fully and review vs text ([#157](https://github.com/john-cd/rust_howto/issues/157)) ([bk/crates/cats/development_tools_debugging/examples/log/log_env_variable.rs](./bk/crates/cats/development_tools_debugging/examples/log/log_env_variable.rs#L76))

### bk/crates/cats/development_tools_debugging/examples/other/open_observe.rs

- [ ] P2 finish ([#733](https://github.com/john-cd/rust_howto/issues/733)) ([bk/crates/cats/development_tools_debugging/examples/other/open_observe.rs](./bk/crates/cats/development_tools_debugging/examples/other/open_observe.rs#L45))

### bk/crates/cats/development_tools_debugging/examples/other/open_telemetry.rs

- [ ] P2 finish ([#734](https://github.com/john-cd/rust_howto/issues/734)) ([bk/crates/cats/development_tools_debugging/examples/other/open_telemetry.rs](./bk/crates/cats/development_tools_debugging/examples/other/open_telemetry.rs#L42))

### bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber2.rs

- [?] P2 review ([bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber2.rs](./bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber2.rs#L137))

### bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber4.rs

- [ ] P2 The same builder methods used here are also available on `fmt()` if ([bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber4.rs](./bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber4.rs#L54))

### bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs

- [?] P2 review https://docs.rs/fake/4.2.0/fake/index.html ([#1124](https://github.com/john-cd/rust_howto/issues/1124)) ([bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs](./bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs#L146))

### bk/crates/cats/email/examples/lettre.rs

- [?] P2 review; Requires valid SMTP credentials to run ([#1144](https://github.com/john-cd/rust_howto/issues/1144)) ([bk/crates/cats/email/examples/lettre.rs](./bk/crates/cats/email/examples/lettre.rs#L66))

### bk/crates/cats/encoding/Cargo.toml

- [ ] P2 update to 3.0.0 ([bk/crates/cats/encoding/Cargo.toml](./bk/crates/cats/encoding/Cargo.toml#L25))

### bk/crates/cats/encoding/build.rs

- [ ] P2 is prost fixed? ([#1417](https://github.com/john-cd/rust_howto/issues/1417)) ([bk/crates/cats/encoding/build.rs](./bk/crates/cats/encoding/build.rs#L40))

### bk/crates/cats/encoding/examples/binary_encoders/bincode.rs

- [ ] P2 finish ([#1040](https://github.com/john-cd/rust_howto/issues/1040)) ([bk/crates/cats/encoding/examples/binary_encoders/bincode.rs](./bk/crates/cats/encoding/examples/binary_encoders/bincode.rs#L51))

### bk/crates/cats/encoding/examples/binary_encoders/flatbuffers.rs

- [ ] P2 finish; https://flatbuffers.dev/languages/rust/ ([#1043](https://github.com/john-cd/rust_howto/issues/1043)) ([bk/crates/cats/encoding/examples/binary_encoders/flatbuffers.rs](./bk/crates/cats/encoding/examples/binary_encoders/flatbuffers.rs#L30))

### bk/crates/cats/encoding/examples/binary_encoders/prost.rs

- [ ] P2 finish ([#1044](https://github.com/john-cd/rust_howto/issues/1044)) ([bk/crates/cats/encoding/examples/binary_encoders/prost.rs](./bk/crates/cats/encoding/examples/binary_encoders/prost.rs#L46))

### bk/crates/cats/encoding/examples/binary_encoders/protobuf.rs

- [ ] P2 finish ([#1045](https://github.com/john-cd/rust_howto/issues/1045)) ([bk/crates/cats/encoding/examples/binary_encoders/protobuf.rs](./bk/crates/cats/encoding/examples/binary_encoders/protobuf.rs#L38))

### bk/crates/cats/encoding/examples/binary_encoders/rmp_serde.rs

- [ ] P2 finish ([#1046](https://github.com/john-cd/rust_howto/issues/1046)) ([bk/crates/cats/encoding/examples/binary_encoders/rmp_serde.rs](./bk/crates/cats/encoding/examples/binary_encoders/rmp_serde.rs#L42))

### bk/crates/cats/encoding/examples/string_encoding/percent_encoding.rs

- [ ] P2 dedupe with other example in percent_encode ([#1353](https://github.com/john-cd/rust_howto/issues/1353)) ([bk/crates/cats/encoding/examples/string_encoding/percent_encoding.rs](./bk/crates/cats/encoding/examples/string_encoding/percent_encoding.rs#L116))

### bk/crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs

- [?] P2 review ([#1350](https://github.com/john-cd/rust_howto/issues/1350)) ([bk/crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs](./bk/crates/cats/mathematics/examples/trigonometry/latitude_longitude.rs#L58))

### bk/crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs

- [?] P2 review lazy_constant.rs and lazy_static.rs and global_mut_state.rs ([#939](https://github.com/john-cd/rust_howto/issues/939)) ([bk/crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs](./bk/crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs#L45))

### bk/crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs

- [?] P2 review lazy_constant.rs and lazy_static.rs and global_mut_state.rs ([#939](https://github.com/john-cd/rust_howto/issues/939)) ([bk/crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs](./bk/crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs#L98))

### bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs

- [ ] P2 finish ([#811](https://github.com/john-cd/rust_howto/issues/811)) ([bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs](./bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs#L91))

### bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs

- [ ] P1 decide what to implement ([bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs](./bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs#L33))
- [ ] P2 finish ([#812](https://github.com/john-cd/rust_howto/issues/812)) ([bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs](./bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs#L55))

### bk/crates/cats/network_programming/examples/server/glommio.rs

- [ ] P2 finish ([#810](https://github.com/john-cd/rust_howto/issues/810)) ([bk/crates/cats/network_programming/examples/server/glommio.rs](./bk/crates/cats/network_programming/examples/server/glommio.rs#L113))

### bk/crates/cats/network_programming/examples/server/listen_unused.rs

- [ ] P2 finish; listens to a connection forever ([#166](https://github.com/john-cd/rust_howto/issues/166)) ([bk/crates/cats/network_programming/examples/server/listen_unused.rs](./bk/crates/cats/network_programming/examples/server/listen_unused.rs#L51))

### bk/crates/cats/os_windows_apis/examples/windows/winapi.rs

- [?] P2 review / test ([#822](https://github.com/john-cd/rust_howto/issues/822)) ([bk/crates/cats/os_windows_apis/examples/windows/winapi.rs](./bk/crates/cats/os_windows_apis/examples/windows/winapi.rs#L135))

### bk/crates/cats/parser_implementations/examples/html/cssparser.rs

- [ ] P2 finish ([#1092](https://github.com/john-cd/rust_howto/issues/1092)) ([bk/crates/cats/parser_implementations/examples/html/cssparser.rs](./bk/crates/cats/parser_implementations/examples/html/cssparser.rs#L40))

### bk/crates/cats/parser_implementations/examples/html/html5ever.rs

- [ ] P2 finish ([#1090](https://github.com/john-cd/rust_howto/issues/1090)) ([bk/crates/cats/parser_implementations/examples/html/html5ever.rs](./bk/crates/cats/parser_implementations/examples/html/html5ever.rs#L96))

### bk/crates/cats/parser_implementations/examples/xml/quick_xml.rs

- [ ] P2 finish ([#1236](https://github.com/john-cd/rust_howto/issues/1236)) ([bk/crates/cats/parser_implementations/examples/xml/quick_xml.rs](./bk/crates/cats/parser_implementations/examples/xml/quick_xml.rs#L81))

### bk/crates/cats/parser_implementations/examples/xml/xml.rs

- [ ] P2 finish ([#1100](https://github.com/john-cd/rust_howto/issues/1100)) ([bk/crates/cats/parser_implementations/examples/xml/xml.rs](./bk/crates/cats/parser_implementations/examples/xml/xml.rs#L111))

### bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs

- [ ] P2 finish ([#1103](https://github.com/john-cd/rust_howto/issues/1103)) ([bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs](./bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs#L65))

### bk/crates/cats/parser_implementations/examples/xml/xmlparser.rs

- [ ] P2 finish ([#1102](https://github.com/john-cd/rust_howto/issues/1102)) ([bk/crates/cats/parser_implementations/examples/xml/xmlparser.rs](./bk/crates/cats/parser_implementations/examples/xml/xmlparser.rs#L55))

### bk/crates/cats/parser_implementations/examples/yaml/serde_yml.rs

- [?] P2 review ([bk/crates/cats/parser_implementations/examples/yaml/serde_yml.rs](./bk/crates/cats/parser_implementations/examples/yaml/serde_yml.rs#L98))

### bk/crates/cats/parsing/examples/parsing/tree_sitter.rs

- [ ] P2 finish ([#827](https://github.com/john-cd/rust_howto/issues/827)) ([bk/crates/cats/parsing/examples/parsing/tree_sitter.rs](./bk/crates/cats/parsing/examples/parsing/tree_sitter.rs#L65))

### bk/crates/cats/parsing/examples/pest/pest.rs

- [ ] P2 finish ([#826](https://github.com/john-cd/rust_howto/issues/826)) ([bk/crates/cats/parsing/examples/pest/pest.rs](./bk/crates/cats/parsing/examples/pest/pest.rs#L66))

### bk/crates/cats/rust_patterns/examples/functional_programming/frunk.rs

- [ ] P2 finish example ([#1318](https://github.com/john-cd/rust_howto/issues/1318)) ([bk/crates/cats/rust_patterns/examples/functional_programming/frunk.rs](./bk/crates/cats/rust_patterns/examples/functional_programming/frunk.rs#L190))

### bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs

- [ ] P2 finish; provide projection example before going into pin projection; explain risks; explain use cases ([#1120](https://github.com/john-cd/rust_howto/issues/1120)) ([bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs](./bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs#L236))

### bk/crates/cats/web_programming/examples/http_types_and_interfaces/http.rs

- [ ] P2 expand; HTTP header interpretation and generation. LATER ([#1355](https://github.com/john-cd/rust_howto/issues/1355)) ([bk/crates/cats/web_programming/examples/http_types_and_interfaces/http.rs](./bk/crates/cats/web_programming/examples/http_types_and_interfaces/http.rs#L53))

### bk/crates/cats/web_programming/examples/scraping/broken.rs

- [ ] P2 flaky test ([#1419](https://github.com/john-cd/rust_howto/issues/1419)) ([bk/crates/cats/web_programming/examples/scraping/broken.rs](./bk/crates/cats/web_programming/examples/scraping/broken.rs#L77))

### bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs

- [ ] P2 fix - the API no longer returns a crate_id - need to get ([#860](https://github.com/john-cd/rust_howto/issues/860)) ([bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs](./bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs#L58))

### bk/crates/cats/web_programming_http_client/examples/apis/rate_limited.rs

- [ ] P2 fix interaction with https://docs.github.com/en/rest?apiVersion=2022-11-28 ([#177](https://github.com/john-cd/rust_howto/issues/177)) ([bk/crates/cats/web_programming_http_client/examples/apis/rate_limited.rs](./bk/crates/cats/web_programming_http_client/examples/apis/rate_limited.rs#L71))

### bk/crates/cats/web_programming_http_client/examples/apis/rest_post.rs

- [ ] P2 review; rewrite so that a username and password are not required? ([#178](https://github.com/john-cd/rust_howto/issues/178)) ([bk/crates/cats/web_programming_http_client/examples/apis/rest_post.rs](./bk/crates/cats/web_programming_http_client/examples/apis/rest_post.rs#L98))

### bk/crates/cats/web_programming_http_client/examples/download/download.rs

- [ ] P2 review; noplayground because of network use. rewrite? ([#225](https://github.com/john-cd/rust_howto/issues/225)) ([bk/crates/cats/web_programming_http_client/examples/download/download.rs](./bk/crates/cats/web_programming_http_client/examples/download/download.rs#L54))

### bk/crates/cats/web_programming_http_client/examples/download/partial.rs

- [?] P2 review; flaky test ([#176](https://github.com/john-cd/rust_howto/issues/176)) ([bk/crates/cats/web_programming_http_client/examples/download/partial.rs](./bk/crates/cats/web_programming_http_client/examples/download/partial.rs#L129))

### bk/crates/cats/web_programming_http_client/examples/download/post_file.rs

- [ ] P2 main().await?; ([bk/crates/cats/web_programming_http_client/examples/download/post_file.rs](./bk/crates/cats/web_programming_http_client/examples/download/post_file.rs#L42))

### bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs

- [?] P2 review ([#859](https://github.com/john-cd/rust_howto/issues/859)) ([bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs](./bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs#L100))

### bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs

- [ ] P2 write more? ([#862](https://github.com/john-cd/rust_howto/issues/862)) ([bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs](./bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs#L37))

### bk/crates/cats/web_programming_http_server/Cargo.toml

- [ ] P2 trillium: do we need LATER ([#1314](https://github.com/john-cd/rust_howto/issues/1314)) ([bk/crates/cats/web_programming_http_server/Cargo.toml](./bk/crates/cats/web_programming_http_server/Cargo.toml#L29))

### bk/crates/cats/web_programming_http_server/build.rs

- [?] P2 review ([bk/crates/cats/web_programming_http_server/build.rs](./bk/crates/cats/web_programming_http_server/build.rs#L7))

### bk/crates/cats/web_programming_http_server/examples/async_graphql.rs

- [ ] P2 finish; https://github.com/async-graphql/examples ([#864](https://github.com/john-cd/rust_howto/issues/864)) ([bk/crates/cats/web_programming_http_server/examples/async_graphql.rs](./bk/crates/cats/web_programming_http_server/examples/async_graphql.rs#L78))

### bk/crates/cats/web_programming_http_server/examples/axum.rs

- [?] P2 review time limit ([#865](https://github.com/john-cd/rust_howto/issues/865)) ([bk/crates/cats/web_programming_http_server/examples/axum.rs](./bk/crates/cats/web_programming_http_server/examples/axum.rs#L132))

### bk/crates/cats/web_programming_http_server/examples/hyper_server.rs

- [ ] P2 Update Hyper server example to 1.0 API and implement request routing ([#866](https://github.com/john-cd/rust_howto/issues/866)) ([bk/crates/cats/web_programming_http_server/examples/hyper_server.rs](./bk/crates/cats/web_programming_http_server/examples/hyper_server.rs#L78))

### bk/crates/cats/web_programming_http_server/examples/leptos.rs

- [ ] P2 Implement Leptos reactive counter and view mounting ([#867](https://github.com/john-cd/rust_howto/issues/867)) ([bk/crates/cats/web_programming_http_server/examples/leptos.rs](./bk/crates/cats/web_programming_http_server/examples/leptos.rs#L75))

### bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs

- [?] P2 review ([#871](https://github.com/john-cd/rust_howto/issues/871)) ([bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs](./bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs#L106))

### bk/crates/cats/web_programming_http_server/examples/rocket.rs

- [ ] P2 Implement Rocket Hello World and route mounting ([#869](https://github.com/john-cd/rust_howto/issues/869)) ([bk/crates/cats/web_programming_http_server/examples/rocket.rs](./bk/crates/cats/web_programming_http_server/examples/rocket.rs#L17))
- [ ] P2 implement the Rocket example; see https://github.com/john-cd/rust_howto/issues/869 ([#869](https://github.com/john-cd/rust_howto/issues/869)) ([bk/crates/cats/web_programming_http_server/examples/rocket.rs](./bk/crates/cats/web_programming_http_server/examples/rocket.rs#L33))

### bk/drafts/categories/asynchronous/async.md

- [ ] P2 write; review in depth ([#633](https://github.com/john-cd/rust_howto/issues/633)) ([bk/drafts/categories/asynchronous/async.md](./bk/drafts/categories/asynchronous/async.md#L83))

### bk/drafts/categories/asynchronous/async_channels.md

- [ ] P2 async_channels: review add other [`postage`][c~postage~docs]↗{{hi:postage}} channels? ([#215](https://github.com/john-cd/rust_howto/issues/215)) ([bk/drafts/categories/asynchronous/async_channels.md](./bk/drafts/categories/asynchronous/async_channels.md#L89))

### bk/drafts/categories/asynchronous/async_traits.md

- [?] P2 async-traits: review new Rust features ([#216](https://github.com/john-cd/rust_howto/issues/216)) ([bk/drafts/categories/asynchronous/async_traits.md](./bk/drafts/categories/asynchronous/async_traits.md#L40))

### bk/drafts/categories/asynchronous/async_utilities.md

- [?] P2 review ([#906](https://github.com/john-cd/rust_howto/issues/906)) ([bk/drafts/categories/asynchronous/async_utilities.md](./bk/drafts/categories/asynchronous/async_utilities.md#L31))

### bk/drafts/categories/asynchronous/index.md

- [?] P2 review ([#905](https://github.com/john-cd/rust_howto/issues/905)) ([bk/drafts/categories/asynchronous/index.md](./bk/drafts/categories/asynchronous/index.md#L66))

### bk/drafts/categories/asynchronous/streams.md

- [ ] P2 add more. streams2.rs is noplayground because it requires a network. rewrite ([#645](https://github.com/john-cd/rust_howto/issues/645)) ([bk/drafts/categories/asynchronous/streams.md](./bk/drafts/categories/asynchronous/streams.md#L41))

### bk/drafts/categories/asynchronous/tokio.md

- [?] P2 tokio: review ([#223](https://github.com/john-cd/rust_howto/issues/223)) ([bk/drafts/categories/asynchronous/tokio.md](./bk/drafts/categories/asynchronous/tokio.md#L72))

### bk/drafts/categories/authentication/basic_authentication.md

- [ ] P2 expand ([#224](https://github.com/john-cd/rust_howto/issues/224)) ([bk/drafts/categories/authentication/basic_authentication.md](./bk/drafts/categories/authentication/basic_authentication.md#L26))

### bk/drafts/categories/authentication/index.md

- [ ] P2 add Oauth ([#636](https://github.com/john-cd/rust_howto/issues/636)) ([bk/drafts/categories/authentication/index.md](./bk/drafts/categories/authentication/index.md#L27))

### bk/drafts/categories/caching/in_memory_cache.md

- [ ] P2 expand; review in depth ([#227](https://github.com/john-cd/rust_howto/issues/227)) ([bk/drafts/categories/caching/in_memory_cache.md](./bk/drafts/categories/caching/in_memory_cache.md#L48))

### bk/drafts/categories/caching/index.md

- [ ] P2 write ([#1173](https://github.com/john-cd/rust_howto/issues/1173)) ([bk/drafts/categories/caching/index.md](./bk/drafts/categories/caching/index.md#L53))

### bk/drafts/categories/command-line-interface/ansi_terminal.md

- [ ] P1 ansi_terminal: `ansi_term` is archived; write / decide what to cover ([#231](https://github.com/john-cd/rust_howto/issues/231)) ([bk/drafts/categories/command-line-interface/ansi_terminal.md](./bk/drafts/categories/command-line-interface/ansi_terminal.md#L138))

### bk/drafts/categories/command-line-interface/argument_parsing.md

- [?] P2 arguments: review ([#233](https://github.com/john-cd/rust_howto/issues/233)) ([bk/drafts/categories/command-line-interface/argument_parsing.md](./bk/drafts/categories/command-line-interface/argument_parsing.md#L149))

### bk/drafts/categories/command-line-interface/index.md

- [?] P2 review ([#907](https://github.com/john-cd/rust_howto/issues/907)) ([bk/drafts/categories/command-line-interface/index.md](./bk/drafts/categories/command-line-interface/index.md#L48))

### bk/drafts/categories/command-line-interface/tui.md

- [ ] P2 tui: expand ([#234](https://github.com/john-cd/rust_howto/issues/234)) ([bk/drafts/categories/command-line-interface/tui.md](./bk/drafts/categories/command-line-interface/tui.md#L45))

### bk/drafts/categories/command-line-interface/user_interaction.md

- [ ] P2 user_interaction: write ([#235](https://github.com/john-cd/rust_howto/issues/235)) ([bk/drafts/categories/command-line-interface/user_interaction.md](./bk/drafts/categories/command-line-interface/user_interaction.md#L49))

### bk/drafts/categories/command-line-utilities/filesystem_cli.md

- [ ] P2 filesystem: organize ([#237](https://github.com/john-cd/rust_howto/issues/237)) ([bk/drafts/categories/command-line-utilities/filesystem_cli.md](./bk/drafts/categories/command-line-utilities/filesystem_cli.md#L60))

### bk/drafts/categories/command-line-utilities/index.md

- [ ] P2 write; deduplicate with other > written in rust ([#1189](https://github.com/john-cd/rust_howto/issues/1189)) ([bk/drafts/categories/command-line-utilities/index.md](./bk/drafts/categories/command-line-utilities/index.md#L48))

### bk/drafts/categories/command-line-utilities/networking_cli.md

- [ ] P2 networking: write ([#238](https://github.com/john-cd/rust_howto/issues/238)) ([bk/drafts/categories/command-line-utilities/networking_cli.md](./bk/drafts/categories/command-line-utilities/networking_cli.md#L25))

### bk/drafts/categories/command-line-utilities/shells.md

- [ ] P2 shells: expand ([#239](https://github.com/john-cd/rust_howto/issues/239)) ([bk/drafts/categories/command-line-utilities/shells.md](./bk/drafts/categories/command-line-utilities/shells.md#L32))

### bk/drafts/categories/compression/compression.md

- [ ] P2 write ([#1062](https://github.com/john-cd/rust_howto/issues/1062)) ([bk/drafts/categories/compression/compression.md](./bk/drafts/categories/compression/compression.md#L48))

### bk/drafts/categories/compression/index.md

- [ ] P2 write ([#1184](https://github.com/john-cd/rust_howto/issues/1184)) ([bk/drafts/categories/compression/index.md](./bk/drafts/categories/compression/index.md#L27))

### bk/drafts/categories/compression/tar.md

- [ ] P2 tar: review tar_decompress.rs is noplayground - fix? tar_compress.rs is noplayground - fix? tar_strip_prefix.rs is noplayground - fix? ([#253](https://github.com/john-cd/rust_howto/issues/253)) ([bk/drafts/categories/compression/tar.md](./bk/drafts/categories/compression/tar.md#L47))

### bk/drafts/categories/concurrency/_actors.md

- [ ] P2 _actors: organize ([#269](https://github.com/john-cd/rust_howto/issues/269)) ([bk/drafts/categories/concurrency/_actors.md](./bk/drafts/categories/concurrency/_actors.md#L98))

### bk/drafts/categories/concurrency/atomics.md

- [ ] P2 fix ([#1342](https://github.com/john-cd/rust_howto/issues/1342)) ([bk/drafts/categories/concurrency/atomics.md](./bk/drafts/categories/concurrency/atomics.md#L56))

### bk/drafts/categories/concurrency/concurrent_data_structures.md

- [ ] P2 concurrent_data_structures: finish ([#258](https://github.com/john-cd/rust_howto/issues/258)) ([bk/drafts/categories/concurrency/concurrent_data_structures.md](./bk/drafts/categories/concurrency/concurrent_data_structures.md#L73))

### bk/drafts/categories/concurrency/crossbeam.md

- [ ] P2 crossbeam: cleanup ([#259](https://github.com/john-cd/rust_howto/issues/259)) ([bk/drafts/categories/concurrency/crossbeam.md](./bk/drafts/categories/concurrency/crossbeam.md#L53))

### bk/drafts/categories/concurrency/data_parallelism.md

- [ ] P2 data_parallelism: polish; dedupe with multithreading.md ([#260](https://github.com/john-cd/rust_howto/issues/260)) ([bk/drafts/categories/concurrency/data_parallelism.md](./bk/drafts/categories/concurrency/data_parallelism.md#L122))

### bk/drafts/categories/concurrency/explicit_threads.md

- [ ] P1 explicit_threads: polish ([#262](https://github.com/john-cd/rust_howto/issues/262)) ([bk/drafts/categories/concurrency/explicit_threads.md](./bk/drafts/categories/concurrency/explicit_threads.md#L33))

### bk/drafts/categories/concurrency/index.md

- [ ] P2 concurrency: add somewhere ([#263](https://github.com/john-cd/rust_howto/issues/263)) ([bk/drafts/categories/concurrency/index.md](./bk/drafts/categories/concurrency/index.md#L83))

### bk/drafts/categories/concurrency/message_passing.md

- [ ] P2 message_passing: polish ([#264](https://github.com/john-cd/rust_howto/issues/264)) ([bk/drafts/categories/concurrency/message_passing.md](./bk/drafts/categories/concurrency/message_passing.md#L58))

### bk/drafts/categories/concurrency/send_sync.md

- [ ] P2 write; add links? ([#909](https://github.com/john-cd/rust_howto/issues/909)) ([bk/drafts/categories/concurrency/send_sync.md](./bk/drafts/categories/concurrency/send_sync.md#L75))

### bk/drafts/categories/concurrency/shared_state.md

- [ ] P2 shared_state: reorganize; section for Arc; for Mutex / Rwlock; for joint use; for make_mut ([#266](https://github.com/john-cd/rust_howto/issues/266)) ([bk/drafts/categories/concurrency/shared_state.md](./bk/drafts/categories/concurrency/shared_state.md#L82))

### bk/drafts/categories/concurrency/threadpool.md

- [ ] P2 threadpool: polish threadpool_fractal.rs is noplayground - linking with [`cc`][c~cc~docs]↗{{hi:cc}} failed: exit status: 1 - fix? ([#267](https://github.com/john-cd/rust_howto/issues/267)) ([bk/drafts/categories/concurrency/threadpool.md](./bk/drafts/categories/concurrency/threadpool.md#L44))

### bk/drafts/categories/config/configuration.md

- [ ] P2 finish ([#270](https://github.com/john-cd/rust_howto/issues/270)) ([bk/drafts/categories/config/configuration.md](./bk/drafts/categories/config/configuration.md#L54))

### bk/drafts/categories/config/index.md

- [ ] P2 Configuration crates like [`config-rs`][c~config~docs]↗{{hi:config-rs}}, a powerful and flexible crate for layered configuration. It supports merging configurations from various sources, including files, environment variables, and in-memory data. ([bk/drafts/categories/config/index.md](./bk/drafts/categories/config/index.md#L16))
- [ ] P2 write ([#1195](https://github.com/john-cd/rust_howto/issues/1195)) ([bk/drafts/categories/config/index.md](./bk/drafts/categories/config/index.md#L64))

### bk/drafts/categories/cryptography/aead.md

- [ ] P2 write ([#1183](https://github.com/john-cd/rust_howto/issues/1183)) ([bk/drafts/categories/cryptography/aead.md](./bk/drafts/categories/cryptography/aead.md#L57))

### bk/drafts/categories/cryptography/certificates.md

- [ ] P2 write ([#1179](https://github.com/john-cd/rust_howto/issues/1179)) ([bk/drafts/categories/cryptography/certificates.md](./bk/drafts/categories/cryptography/certificates.md#L64))

### bk/drafts/categories/cryptography/cryptography_utilities.md

- [ ] P2 write ([#1180](https://github.com/john-cd/rust_howto/issues/1180)) ([bk/drafts/categories/cryptography/cryptography_utilities.md](./bk/drafts/categories/cryptography/cryptography_utilities.md#L39))

### bk/drafts/categories/cryptography/encryption.md

- [ ] P2 encryption: expand ([#272](https://github.com/john-cd/rust_howto/issues/272)) ([bk/drafts/categories/cryptography/encryption.md](./bk/drafts/categories/cryptography/encryption.md#L33))

### bk/drafts/categories/cryptography/hmac.md

- [ ] P2 write ([#1181](https://github.com/john-cd/rust_howto/issues/1181)) ([bk/drafts/categories/cryptography/hmac.md](./bk/drafts/categories/cryptography/hmac.md#L27))

### bk/drafts/categories/cryptography/index.md

- [?] P2 cryptography: review ([#274](https://github.com/john-cd/rust_howto/issues/274)) ([bk/drafts/categories/cryptography/index.md](./bk/drafts/categories/cryptography/index.md#L95))

### bk/drafts/categories/cryptography/password_hashing.md

- [ ] P2 password_hashing: write ([#275](https://github.com/john-cd/rust_howto/issues/275)) ([bk/drafts/categories/cryptography/password_hashing.md](./bk/drafts/categories/cryptography/password_hashing.md#L87))

### bk/drafts/categories/cryptography/signature.md

- [ ] P2 write ([#1178](https://github.com/john-cd/rust_howto/issues/1178)) ([bk/drafts/categories/cryptography/signature.md](./bk/drafts/categories/cryptography/signature.md#L80))

### bk/drafts/categories/cryptography/tls.md

- [ ] P2 write ([#1182](https://github.com/john-cd/rust_howto/issues/1182)) ([bk/drafts/categories/cryptography/tls.md](./bk/drafts/categories/cryptography/tls.md#L41))

### bk/drafts/categories/data-structures/index.md

- [ ] P2 data-structures: expand ([#280](https://github.com/john-cd/rust_howto/issues/280)) ([bk/drafts/categories/data-structures/index.md](./bk/drafts/categories/data-structures/index.md#L189))

### bk/drafts/categories/database-implementations/databases.md

- [ ] P2 databases: expand / write ([#290](https://github.com/john-cd/rust_howto/issues/290)) ([bk/drafts/categories/database-implementations/databases.md](./bk/drafts/categories/database-implementations/databases.md#L57))

### bk/drafts/categories/database-implementations/index.md

- [ ] P2 database-implementations: expand ([#292](https://github.com/john-cd/rust_howto/issues/292)) ([bk/drafts/categories/database-implementations/index.md](./bk/drafts/categories/database-implementations/index.md#L25))

### bk/drafts/categories/database-implementations/rust_search_engines.md

- [ ] P2 search: expand ([#291](https://github.com/john-cd/rust_howto/issues/291)) ([bk/drafts/categories/database-implementations/rust_search_engines.md](./bk/drafts/categories/database-implementations/rust_search_engines.md#L47))

### bk/drafts/categories/database/amqp.md

- [ ] P2 finish ([#1064](https://github.com/john-cd/rust_howto/issues/1064)) ([bk/drafts/categories/database/amqp.md](./bk/drafts/categories/database/amqp.md#L29))

### bk/drafts/categories/database/connection_pool.md

- [ ] P2 connection_pool: expand; example ([#284](https://github.com/john-cd/rust_howto/issues/284)) ([bk/drafts/categories/database/connection_pool.md](./bk/drafts/categories/database/connection_pool.md#L29))

### bk/drafts/categories/database/index.md

- [ ] P2 organize ([#1065](https://github.com/john-cd/rust_howto/issues/1065)) ([bk/drafts/categories/database/index.md](./bk/drafts/categories/database/index.md#L65))

### bk/drafts/categories/database/mssql.md

- [ ] P2 organize / write ([#1067](https://github.com/john-cd/rust_howto/issues/1067)) ([bk/drafts/categories/database/mssql.md](./bk/drafts/categories/database/mssql.md#L23))

### bk/drafts/categories/database/nosql.md

- [?] P2 review ([#1068](https://github.com/john-cd/rust_howto/issues/1068)) ([bk/drafts/categories/database/nosql.md](./bk/drafts/categories/database/nosql.md#L62))

### bk/drafts/categories/database/postgres.md

- [ ] P2 postgres: `cornucopia` ([#286](https://github.com/john-cd/rust_howto/issues/286)) ([bk/drafts/categories/database/postgres.md](./bk/drafts/categories/database/postgres.md#L76))

### bk/drafts/categories/database/query_builders_orms.md

- [ ] P2 write; move `toasty` example to a file ([#912](https://github.com/john-cd/rust_howto/issues/912)) ([bk/drafts/categories/database/query_builders_orms.md](./bk/drafts/categories/database/query_builders_orms.md#L167))

### bk/drafts/categories/database/search.md

- [ ] P2 database/search.md: expand ([#288](https://github.com/john-cd/rust_howto/issues/288)) ([bk/drafts/categories/database/search.md](./bk/drafts/categories/database/search.md#L37))

### bk/drafts/categories/date-and-time/duration.md

- [?] P2 review ([#913](https://github.com/john-cd/rust_howto/issues/913)) ([bk/drafts/categories/date-and-time/duration.md](./bk/drafts/categories/date-and-time/duration.md#L49))

### bk/drafts/categories/date-and-time/index.md

- [?] P2 review in depth ([#1188](https://github.com/john-cd/rust_howto/issues/1188)) ([bk/drafts/categories/date-and-time/index.md](./bk/drafts/categories/date-and-time/index.md#L38))

### bk/drafts/categories/date-and-time/parse.md

- [ ] P2 write humantime; review ([#914](https://github.com/john-cd/rust_howto/issues/914)) ([bk/drafts/categories/date-and-time/parse.md](./bk/drafts/categories/date-and-time/parse.md#L73))

### bk/drafts/categories/date-and-time/time_crate.md

- [ ] P2 time: write ([#293](https://github.com/john-cd/rust_howto/issues/293)) ([bk/drafts/categories/date-and-time/time_crate.md](./bk/drafts/categories/date-and-time/time_crate.md#L27))

### bk/drafts/categories/development-tools/cargo/cargo.md

- [?] P2 review ([#915](https://github.com/john-cd/rust_howto/issues/915)) ([bk/drafts/categories/development-tools/cargo/cargo.md](./bk/drafts/categories/development-tools/cargo/cargo.md#L104))

### bk/drafts/categories/development-tools/cargo/crate_registries.md

- [ ] P2 write; expand ([#294](https://github.com/john-cd/rust_howto/issues/294)) ([bk/drafts/categories/development-tools/cargo/crate_registries.md](./bk/drafts/categories/development-tools/cargo/crate_registries.md#L33))

### bk/drafts/categories/development-tools/cargo/package_layout.md

- [?] P2 review ([#916](https://github.com/john-cd/rust_howto/issues/916)) ([bk/drafts/categories/development-tools/cargo/package_layout.md](./bk/drafts/categories/development-tools/cargo/package_layout.md#L51))

### bk/drafts/categories/development-tools/compilation/faster_linking.md

- [?] P2 faster_linking: review - some linkers are deprecated ([#242](https://github.com/john-cd/rust_howto/issues/242)) ([bk/drafts/categories/development-tools/compilation/faster_linking.md](./bk/drafts/categories/development-tools/compilation/faster_linking.md#L142))

### bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md

- [ ] P2 write / expand / align table and text ([#245](https://github.com/john-cd/rust_howto/issues/245)) ([bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md](./bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md#L122))

### bk/drafts/categories/development-tools/cross-compilation/cross_compilation.md

- [ ] P2 cross_compilation: expand ([#240](https://github.com/john-cd/rust_howto/issues/240)) ([bk/drafts/categories/development-tools/cross-compilation/cross_compilation.md](./bk/drafts/categories/development-tools/cross-compilation/cross_compilation.md#L55))

### bk/drafts/categories/development-tools/documentation/badges.md

- [ ] P2 badges: expand ([#296](https://github.com/john-cd/rust_howto/issues/296)) ([bk/drafts/categories/development-tools/documentation/badges.md](./bk/drafts/categories/development-tools/documentation/badges.md#L18))

### bk/drafts/categories/development-tools/documentation/documentation.md

- [ ] P2 documentation: add; review ([#297](https://github.com/john-cd/rust_howto/issues/297)) ([bk/drafts/categories/development-tools/documentation/documentation.md](./bk/drafts/categories/development-tools/documentation/documentation.md#L67))

### bk/drafts/categories/development-tools/documentation/mdbook.md

- [ ] P2 mdbook: organize, expand ([#299](https://github.com/john-cd/rust_howto/issues/299)) ([bk/drafts/categories/development-tools/documentation/mdbook.md](./bk/drafts/categories/development-tools/documentation/mdbook.md#L146))

### bk/drafts/categories/development-tools/formatting/formatting.md

- [ ] P2 formatting: add link for formatting attributes ([#300](https://github.com/john-cd/rust_howto/issues/300)) ([bk/drafts/categories/development-tools/formatting/formatting.md](./bk/drafts/categories/development-tools/formatting/formatting.md#L96))

### bk/drafts/categories/development-tools/index.md

- [ ] P2 [index: reorganize; dedupe alternatives / [`log`][c~log~docs]↗{{hi:log}} / config_log]() ([#319](https://github.com/john-cd/rust_howto/issues/319)) ([bk/drafts/categories/development-tools/index.md](./bk/drafts/categories/development-tools/index.md#L95))

### bk/drafts/categories/development-tools/installation/install.md

- [?] P2 review ([#918](https://github.com/john-cd/rust_howto/issues/918)) ([bk/drafts/categories/development-tools/installation/install.md](./bk/drafts/categories/development-tools/installation/install.md#L35))

### bk/drafts/categories/development-tools/installation/rustup.md

- [ ] P2 rustup: expand / clean up ([#302](https://github.com/john-cd/rust_howto/issues/302)) ([bk/drafts/categories/development-tools/installation/rustup.md](./bk/drafts/categories/development-tools/installation/rustup.md#L56))

### bk/drafts/categories/development-tools/other/code_build.md

- [ ] P2 review; dedupe this page and cargo plugins > building; move from other section ([#919](https://github.com/john-cd/rust_howto/issues/919)) ([bk/drafts/categories/development-tools/other/code_build.md](./bk/drafts/categories/development-tools/other/code_build.md#L114))

### bk/drafts/categories/development-tools/other/code_verification.md

- [ ] P2 code_verification: expand; revise refs.incl.md ([#303](https://github.com/john-cd/rust_howto/issues/303)) ([bk/drafts/categories/development-tools/other/code_verification.md](./bk/drafts/categories/development-tools/other/code_verification.md#L110))

### bk/drafts/categories/development-tools/other/miri.md

- [ ] P2 miri: polish ([#304](https://github.com/john-cd/rust_howto/issues/304)) ([bk/drafts/categories/development-tools/other/miri.md](./bk/drafts/categories/development-tools/other/miri.md#L31))

### bk/drafts/categories/development-tools/other/other.md

- [ ] P2 other: expand; revise refs.incl.md ([#305](https://github.com/john-cd/rust_howto/issues/305)) ([bk/drafts/categories/development-tools/other/other.md](./bk/drafts/categories/development-tools/other/other.md#L40))

### bk/drafts/categories/development-tools/transcompilation/transpilers.md

- [ ] P2 write ([#1196](https://github.com/john-cd/rust_howto/issues/1196)) ([bk/drafts/categories/development-tools/transcompilation/transpilers.md](./bk/drafts/categories/development-tools/transcompilation/transpilers.md#L19))

### bk/drafts/categories/development-tools/versioning/versioning.md

- [?] P2 review / align with intro ([#920](https://github.com/john-cd/rust_howto/issues/920)) ([bk/drafts/categories/development-tools/versioning/versioning.md](./bk/drafts/categories/development-tools/versioning/versioning.md#L74))

### bk/drafts/categories/development-tools_build-utils/autocfg.md

- [ ] P2 write ([#1166](https://github.com/john-cd/rust_howto/issues/1166)) ([bk/drafts/categories/development-tools_build-utils/autocfg.md](./bk/drafts/categories/development-tools_build-utils/autocfg.md#L25))

### bk/drafts/categories/development-tools_build-utils/build_cache.md

- [ ] P2 write ([#1167](https://github.com/john-cd/rust_howto/issues/1167)) ([bk/drafts/categories/development-tools_build-utils/build_cache.md](./bk/drafts/categories/development-tools_build-utils/build_cache.md#L19))

### bk/drafts/categories/development-tools_build-utils/build_time_tooling.md

- [?] P2 review ([#921](https://github.com/john-cd/rust_howto/issues/921)) ([bk/drafts/categories/development-tools_build-utils/build_time_tooling.md](./bk/drafts/categories/development-tools_build-utils/build_time_tooling.md#L160))

### bk/drafts/categories/development-tools_build-utils/index.md

- [ ] P2 development-tools_build-utils/index: add ([#306](https://github.com/john-cd/rust_howto/issues/306)) ([bk/drafts/categories/development-tools_build-utils/index.md](./bk/drafts/categories/development-tools_build-utils/index.md#L34))

### bk/drafts/categories/development-tools_cargo-plugins/auditing.md

- [?] P2 review ([#922](https://github.com/john-cd/rust_howto/issues/922)) ([bk/drafts/categories/development-tools_cargo-plugins/auditing.md](./bk/drafts/categories/development-tools_cargo-plugins/auditing.md#L89))

### bk/drafts/categories/development-tools_cargo-plugins/building.md

- [ ] P2 building: expand; cross link ([#309](https://github.com/john-cd/rust_howto/issues/309)) ([bk/drafts/categories/development-tools_cargo-plugins/building.md](./bk/drafts/categories/development-tools_cargo-plugins/building.md#L92))

### bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md

- [ ] P2 code_formatting_linting: expand ([#310](https://github.com/john-cd/rust_howto/issues/310)) ([bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md](./bk/drafts/categories/development-tools_cargo-plugins/code_formatting_linting.md#L94))

### bk/drafts/categories/development-tools_cargo-plugins/code_writing.md

- [?] P2 review ([#924](https://github.com/john-cd/rust_howto/issues/924)) ([bk/drafts/categories/development-tools_cargo-plugins/code_writing.md](./bk/drafts/categories/development-tools_cargo-plugins/code_writing.md#L43))

### bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md

- [?] P2 review ([#923](https://github.com/john-cd/rust_howto/issues/923)) ([bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md](./bk/drafts/categories/development-tools_cargo-plugins/cross_compiling.md#L29))

### bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md

- [?] P2 dependency_management: review ([#597](https://github.com/john-cd/rust_howto/issues/597)) ([bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md](./bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md#L87))

### bk/drafts/categories/development-tools_cargo-plugins/index.md

- [?] P2 review in depth, reorg table ([#311](https://github.com/john-cd/rust_howto/issues/311)) ([bk/drafts/categories/development-tools_cargo-plugins/index.md](./bk/drafts/categories/development-tools_cargo-plugins/index.md#L88))

### bk/drafts/categories/development-tools_cargo-plugins/maintaining.md

- [ ] P1 maintaining: fix; titles; decide what goes where ([#313](https://github.com/john-cd/rust_howto/issues/313)) ([bk/drafts/categories/development-tools_cargo-plugins/maintaining.md](./bk/drafts/categories/development-tools_cargo-plugins/maintaining.md#L71))

### bk/drafts/categories/development-tools_cargo-plugins/performance.md

- [ ] P2 performance: expand ([#314](https://github.com/john-cd/rust_howto/issues/314)) ([bk/drafts/categories/development-tools_cargo-plugins/performance.md](./bk/drafts/categories/development-tools_cargo-plugins/performance.md#L33))

### bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md

- [ ] P2 watching_for_changes: expand ([#315](https://github.com/john-cd/rust_howto/issues/315)) ([bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md](./bk/drafts/categories/development-tools_cargo-plugins/watching_for_changes.md#L63))

### bk/drafts/categories/development-tools_debugging/config_log.md

- [?] P2 review ([#925](https://github.com/john-cd/rust_howto/issues/925)) ([bk/drafts/categories/development-tools_debugging/config_log.md](./bk/drafts/categories/development-tools_debugging/config_log.md#L87))

### bk/drafts/categories/development-tools_debugging/debugging.md

- [ ] P2 write; cross link ([#1344](https://github.com/john-cd/rust_howto/issues/1344)) ([bk/drafts/categories/development-tools_debugging/debugging.md](./bk/drafts/categories/development-tools_debugging/debugging.md#L19))

### bk/drafts/categories/development-tools_debugging/diagnostic_functions.md

- [?] P2 review ([#926](https://github.com/john-cd/rust_howto/issues/926)) ([bk/drafts/categories/development-tools_debugging/diagnostic_functions.md](./bk/drafts/categories/development-tools_debugging/diagnostic_functions.md#L21))

### bk/drafts/categories/development-tools_debugging/distributed_telemetry.md

- [ ] P2 write / expand / organize. what to cover? ([#1343](https://github.com/john-cd/rust_howto/issues/1343)) ([bk/drafts/categories/development-tools_debugging/distributed_telemetry.md](./bk/drafts/categories/development-tools_debugging/distributed_telemetry.md#L74))

### bk/drafts/categories/development-tools_debugging/index.md

- [ ] P2 index: reorganize; dedupe alternatives / `log` / config_log ([#319](https://github.com/john-cd/rust_howto/issues/319)) ([bk/drafts/categories/development-tools_debugging/index.md](./bk/drafts/categories/development-tools_debugging/index.md#L67))

### bk/drafts/categories/development-tools_debugging/log.md

- [?] P2 review ([#927](https://github.com/john-cd/rust_howto/issues/927)) ([bk/drafts/categories/development-tools_debugging/log.md](./bk/drafts/categories/development-tools_debugging/log.md#L83))

### bk/drafts/categories/development-tools_debugging/metrics.md

- [ ] P2 write ([#1345](https://github.com/john-cd/rust_howto/issues/1345)) ([bk/drafts/categories/development-tools_debugging/metrics.md](./bk/drafts/categories/development-tools_debugging/metrics.md#L25))

### bk/drafts/categories/development-tools_debugging/tracing.md

- [?] P2 review ([#322](https://github.com/john-cd/rust_howto/issues/322)) ([bk/drafts/categories/development-tools_debugging/tracing.md](./bk/drafts/categories/development-tools_debugging/tracing.md#L143))

### bk/drafts/categories/development-tools_debugging/tracing_alternatives.md

- [ ] P2 write, organize together with the old log content. ([#649](https://github.com/john-cd/rust_howto/issues/649)) ([bk/drafts/categories/development-tools_debugging/tracing_alternatives.md](./bk/drafts/categories/development-tools_debugging/tracing_alternatives.md#L72))

### bk/drafts/categories/development-tools_profiling/assembly.md

- [ ] P2 assembly: write ([#333](https://github.com/john-cd/rust_howto/issues/333)) ([bk/drafts/categories/development-tools_profiling/assembly.md](./bk/drafts/categories/development-tools_profiling/assembly.md#L22))

### bk/drafts/categories/development-tools_profiling/benchmarking.md

- [ ] P2 benchmarking: write ([#335](https://github.com/john-cd/rust_howto/issues/335)) ([bk/drafts/categories/development-tools_profiling/benchmarking.md](./bk/drafts/categories/development-tools_profiling/benchmarking.md#L104))

### bk/drafts/categories/development-tools_profiling/index.md

- [ ] P2 expand / review ([#337](https://github.com/john-cd/rust_howto/issues/337)) ([bk/drafts/categories/development-tools_profiling/index.md](./bk/drafts/categories/development-tools_profiling/index.md#L69))

### bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md

- [ ] P2 memory: write / organize ([#336](https://github.com/john-cd/rust_howto/issues/336)) ([bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md](./bk/drafts/categories/development-tools_profiling/memory_usage_analysis.md#L53))

### bk/drafts/categories/development-tools_testing/assertions.md

- [ ] P2 write ([#1174](https://github.com/john-cd/rust_howto/issues/1174)) ([bk/drafts/categories/development-tools_testing/assertions.md](./bk/drafts/categories/development-tools_testing/assertions.md#L41))

### bk/drafts/categories/development-tools_testing/code_coverage.md

- [ ] P2 write ([#1176](https://github.com/john-cd/rust_howto/issues/1176)) ([bk/drafts/categories/development-tools_testing/code_coverage.md](./bk/drafts/categories/development-tools_testing/code_coverage.md#L30))

### bk/drafts/categories/development-tools_testing/index.md

- [ ] P2 expand ([#341](https://github.com/john-cd/rust_howto/issues/341)) ([bk/drafts/categories/development-tools_testing/index.md](./bk/drafts/categories/development-tools_testing/index.md#L31))

### bk/drafts/categories/development-tools_testing/mocking.md

- [ ] P2 testing: write ([#340](https://github.com/john-cd/rust_howto/issues/340)) ([bk/drafts/categories/development-tools_testing/mocking.md](./bk/drafts/categories/development-tools_testing/mocking.md#L38))

### bk/drafts/categories/development-tools_testing/test_runners.md

- [ ] P2 testing: write ([#340](https://github.com/john-cd/rust_howto/issues/340)) ([bk/drafts/categories/development-tools_testing/test_runners.md](./bk/drafts/categories/development-tools_testing/test_runners.md#L43))

### bk/drafts/categories/development-tools_testing/testing.md

- [ ] P2 testing: write ([#340](https://github.com/john-cd/rust_howto/issues/340)) ([bk/drafts/categories/development-tools_testing/testing.md](./bk/drafts/categories/development-tools_testing/testing.md#L58))

### bk/drafts/categories/email/email_parsing.md

- [ ] P2 write ([#1204](https://github.com/john-cd/rust_howto/issues/1204)) ([bk/drafts/categories/email/email_parsing.md](./bk/drafts/categories/email/email_parsing.md#L25))

### bk/drafts/categories/email/index.md

- [ ] P2 write ([#343](https://github.com/john-cd/rust_howto/issues/343)) ([bk/drafts/categories/email/index.md](./bk/drafts/categories/email/index.md#L29))

### bk/drafts/categories/email/send_emails.md

- [ ] P2 send_emails: write ([#342](https://github.com/john-cd/rust_howto/issues/342)) ([bk/drafts/categories/email/send_emails.md](./bk/drafts/categories/email/send_emails.md#L51))

### bk/drafts/categories/encoding/_binary_encoders.md

- [ ] P2 binary_encoders: write; add anchors; add examples; add table; add to index / SUMMARY ([#349](https://github.com/john-cd/rust_howto/issues/349)) ([bk/drafts/categories/encoding/_binary_encoders.md](./bk/drafts/categories/encoding/_binary_encoders.md#L118))

### bk/drafts/categories/encoding/complex_encoding.md

- [ ] P2 complex: clean up `toml` ([#350](https://github.com/john-cd/rust_howto/issues/350)) ([bk/drafts/categories/encoding/complex_encoding.md](./bk/drafts/categories/encoding/complex_encoding.md#L54))

### bk/drafts/categories/encoding/csv.md

- [?] P2 review ([#928](https://github.com/john-cd/rust_howto/issues/928)) ([bk/drafts/categories/encoding/csv.md](./bk/drafts/categories/encoding/csv.md#L94))

### bk/drafts/categories/encoding/index.md

- [?] P2 review ([#929](https://github.com/john-cd/rust_howto/issues/929)) ([bk/drafts/categories/encoding/index.md](./bk/drafts/categories/encoding/index.md#L104))

### bk/drafts/categories/encoding/no_external_schema.md

- [ ] P2 write ([#1078](https://github.com/john-cd/rust_howto/issues/1078)) ([bk/drafts/categories/encoding/no_external_schema.md](./bk/drafts/categories/encoding/no_external_schema.md#L35))

### bk/drafts/categories/encoding/serde.md

- [ ] P2 serde: write ([#352](https://github.com/john-cd/rust_howto/issues/352)) ([bk/drafts/categories/encoding/serde.md](./bk/drafts/categories/encoding/serde.md#L65))

### bk/drafts/categories/encoding/string_encoding.md

- [?] P2 review ([#930](https://github.com/john-cd/rust_howto/issues/930)) ([bk/drafts/categories/encoding/string_encoding.md](./bk/drafts/categories/encoding/string_encoding.md#L71))

### bk/drafts/categories/encoding/typecasts.md

- [ ] P2 typecasts: write; reorganize ([#354](https://github.com/john-cd/rust_howto/issues/354)) ([bk/drafts/categories/encoding/typecasts.md](./bk/drafts/categories/encoding/typecasts.md#L98))

### bk/drafts/categories/graphics/index.md

- [ ] P2 write / organize / review in depth ([#1226](https://github.com/john-cd/rust_howto/issues/1226)) ([bk/drafts/categories/graphics/index.md](./bk/drafts/categories/graphics/index.md#L80))

### bk/drafts/categories/hardware-support/index.md

- [ ] P2 . ([bk/drafts/categories/hardware-support/index.md](./bk/drafts/categories/hardware-support/index.md#L21))
- [ ] P2 . ([bk/drafts/categories/hardware-support/index.md](./bk/drafts/categories/hardware-support/index.md#L27))
- [ ] P2 how devices communicate with each other. ([bk/drafts/categories/hardware-support/index.md](./bk/drafts/categories/hardware-support/index.md#L35))
- [ ] P2 expand hardware-support, write missing sections ([#70](https://github.com/john-cd/rust_howto/issues/70)) ([bk/drafts/categories/hardware-support/index.md](./bk/drafts/categories/hardware-support/index.md#L67))

### bk/drafts/categories/hardware-support/peripherals.md

- [ ] P2 replace by `nix`. ([bk/drafts/categories/hardware-support/peripherals.md](./bk/drafts/categories/hardware-support/peripherals.md#L23))
- [ ] P2 write ([#1177](https://github.com/john-cd/rust_howto/issues/1177)) ([bk/drafts/categories/hardware-support/peripherals.md](./bk/drafts/categories/hardware-support/peripherals.md#L48))

### bk/drafts/categories/hardware-support/processor.md

- [ ] P2 processor: expand ([#399](https://github.com/john-cd/rust_howto/issues/399)) ([bk/drafts/categories/hardware-support/processor.md](./bk/drafts/categories/hardware-support/processor.md#L81))

### bk/drafts/categories/memory-management/index.md

- [ ] P2 Resolve TODO/FIXME at line 12 ([bk/drafts/categories/memory-management/index.md](./bk/drafts/categories/memory-management/index.md#L12))
- [ ] P2 memory-management/index: organize; align table and sections; write missing sections; cross link ([#410](https://github.com/john-cd/rust_howto/issues/410)) ([bk/drafts/categories/memory-management/index.md](./bk/drafts/categories/memory-management/index.md#L67))

### bk/drafts/categories/memory-management/lazy_initialization.md

- [ ] P2 cover once_cell in std ([bk/drafts/categories/memory-management/lazy_initialization.md](./bk/drafts/categories/memory-management/lazy_initialization.md#L46))
- [ ] P2 lazy_initialization: write / fix ([#411](https://github.com/john-cd/rust_howto/issues/411)) ([bk/drafts/categories/memory-management/lazy_initialization.md](./bk/drafts/categories/memory-management/lazy_initialization.md#L95))

### bk/drafts/categories/network-programming/index.md

- [ ] P2 write; add cross-links; review; review `email_address`, `fast_chemail` ([#944](https://github.com/john-cd/rust_howto/issues/944)) ([bk/drafts/categories/network-programming/index.md](./bk/drafts/categories/network-programming/index.md#L35))

### bk/drafts/categories/network-programming/reverse_proxy.md

- [ ] P2 Network: Complete comprehensive reverse proxy section for Pingora and Rathole ([#424](https://github.com/john-cd/rust_howto/issues/424)) ([bk/drafts/categories/network-programming/reverse_proxy.md](./bk/drafts/categories/network-programming/reverse_proxy.md#L94))

### bk/drafts/categories/network-programming/server.md

- [ ] P2 server: write; add cross-links: async, web, `http` server ([#425](https://github.com/john-cd/rust_howto/issues/425)) ([bk/drafts/categories/network-programming/server.md](./bk/drafts/categories/network-programming/server.md#L45))

### bk/drafts/categories/os/external_commands.md

- [ ] P2 write which; review ([#946](https://github.com/john-cd/rust_howto/issues/946)) ([bk/drafts/categories/os/external_commands.md](./bk/drafts/categories/os/external_commands.md#L99))

### bk/drafts/categories/os/index.md

- [ ] P2 os/index: fix; organize; add cross links ([#429](https://github.com/john-cd/rust_howto/issues/429)) ([bk/drafts/categories/os/index.md](./bk/drafts/categories/os/index.md#L41))

### bk/drafts/categories/os/low_level_system_calls.md

- [ ] P2 low_level_system_calls: write ([#430](https://github.com/john-cd/rust_howto/issues/430)) ([bk/drafts/categories/os/low_level_system_calls.md](./bk/drafts/categories/os/low_level_system_calls.md#L29))

### bk/drafts/categories/os/rust_os.md

- [ ] P2 write; move to written-in-rust? ([#639](https://github.com/john-cd/rust_howto/issues/639)) ([bk/drafts/categories/os/rust_os.md](./bk/drafts/categories/os/rust_os.md#L23))

### bk/drafts/categories/os_unix-apis/index.md

- [?] P2 review ([#437](https://github.com/john-cd/rust_howto/issues/437)) ([bk/drafts/categories/os_unix-apis/index.md](./bk/drafts/categories/os_unix-apis/index.md#L51))

### bk/drafts/categories/os_unix-apis/unix.md

- [ ] P2 unix: write ([#436](https://github.com/john-cd/rust_howto/issues/436)) ([bk/drafts/categories/os_unix-apis/unix.md](./bk/drafts/categories/os_unix-apis/unix.md#L44))

### bk/drafts/categories/os_windows-apis/index.md

- [ ] P2 review in depth / write ([#950](https://github.com/john-cd/rust_howto/issues/950)) ([bk/drafts/categories/os_windows-apis/index.md](./bk/drafts/categories/os_windows-apis/index.md#L33))

### bk/drafts/categories/os_windows-apis/windows.md

- [ ] P1 windows: write; add examples for "windows" and `winapi` ([#438](https://github.com/john-cd/rust_howto/issues/438)) ([bk/drafts/categories/os_windows-apis/windows.md](./bk/drafts/categories/os_windows-apis/windows.md#L48))

### bk/drafts/categories/parser-implementations/html.md

- [ ] P2 write ([#1187](https://github.com/john-cd/rust_howto/issues/1187)) ([bk/drafts/categories/parser-implementations/html.md](./bk/drafts/categories/parser-implementations/html.md#L63))

### bk/drafts/categories/parser-implementations/index.md

- [ ] P2 write; Review `async-graphql-parser` ([#447](https://github.com/john-cd/rust_howto/issues/447)) ([bk/drafts/categories/parser-implementations/index.md](./bk/drafts/categories/parser-implementations/index.md#L51))

### bk/drafts/categories/parser-implementations/ini.md

- [ ] P2 Write; Also consider: `configparser` ([#1185](https://github.com/john-cd/rust_howto/issues/1185)) ([bk/drafts/categories/parser-implementations/ini.md](./bk/drafts/categories/parser-implementations/ini.md#L27))

### bk/drafts/categories/parser-implementations/json.md

- [ ] P2 json: include in index.md / SUMMARY.md; write ([#440](https://github.com/john-cd/rust_howto/issues/440)) ([bk/drafts/categories/parser-implementations/json.md](./bk/drafts/categories/parser-implementations/json.md#L45))

### bk/drafts/categories/parser-implementations/markdown.md

- [ ] P2 markdown: write ([#442](https://github.com/john-cd/rust_howto/issues/442)) ([bk/drafts/categories/parser-implementations/markdown.md](./bk/drafts/categories/parser-implementations/markdown.md#L45))

### bk/drafts/categories/parser-implementations/programming_languages.md

- [ ] P2 write ([#951](https://github.com/john-cd/rust_howto/issues/951)) ([bk/drafts/categories/parser-implementations/programming_languages.md](./bk/drafts/categories/parser-implementations/programming_languages.md#L50))

### bk/drafts/categories/parser-implementations/toml.md

- [ ] P2 toml: write ([#444](https://github.com/john-cd/rust_howto/issues/444)) ([bk/drafts/categories/parser-implementations/toml.md](./bk/drafts/categories/parser-implementations/toml.md#L45))

### bk/drafts/categories/parser-implementations/xml.md

- [ ] P2 write ([#446](https://github.com/john-cd/rust_howto/issues/446)) ([bk/drafts/categories/parser-implementations/xml.md](./bk/drafts/categories/parser-implementations/xml.md#L67))

### bk/drafts/categories/parser-implementations/yaml.md

- [ ] P2 write ([#1186](https://github.com/john-cd/rust_howto/issues/1186)) ([bk/drafts/categories/parser-implementations/yaml.md](./bk/drafts/categories/parser-implementations/yaml.md#L27))

### bk/drafts/categories/parsing/index.md

- [?] P2 review ([#952](https://github.com/john-cd/rust_howto/issues/952)) ([bk/drafts/categories/parsing/index.md](./bk/drafts/categories/parsing/index.md#L32))

### bk/drafts/categories/parsing/parsing.md

- [ ] P2 parsing: write; examples ([#448](https://github.com/john-cd/rust_howto/issues/448)) ([bk/drafts/categories/parsing/parsing.md](./bk/drafts/categories/parsing/parsing.md#L66))

### bk/drafts/categories/rust-patterns/behavioral_patterns.md

- [ ] P2 write ([#461](https://github.com/john-cd/rust_howto/issues/461)) ([bk/drafts/categories/rust-patterns/behavioral_patterns.md](./bk/drafts/categories/rust-patterns/behavioral_patterns.md#L19))

### bk/drafts/categories/rust-patterns/builder_pattern.md

- [ ] P2 write and add to index and SUMMARY ([#648](https://github.com/john-cd/rust_howto/issues/648)) ([bk/drafts/categories/rust-patterns/builder_pattern.md](./bk/drafts/categories/rust-patterns/builder_pattern.md#L43))

### bk/drafts/categories/rust-patterns/creational_patterns.md

- [ ] P2 write ([#1391](https://github.com/john-cd/rust_howto/issues/1391)) ([bk/drafts/categories/rust-patterns/creational_patterns.md](./bk/drafts/categories/rust-patterns/creational_patterns.md#L35))

### bk/drafts/categories/rust-patterns/error_handling/error_customization.md

- [ ] P2 error_customization: write / organize ([#463](https://github.com/john-cd/rust_howto/issues/463)) ([bk/drafts/categories/rust-patterns/error_handling/error_customization.md](./bk/drafts/categories/rust-patterns/error_handling/error_customization.md#L82))

### bk/drafts/categories/rust-patterns/error_handling/error_handling.md

- [ ] P2 error_handling: fix / organize ([#465](https://github.com/john-cd/rust_howto/issues/465)) ([bk/drafts/categories/rust-patterns/error_handling/error_handling.md](./bk/drafts/categories/rust-patterns/error_handling/error_handling.md#L212))

### bk/drafts/categories/rust-patterns/functional_programming.md

- [ ] P2 functional_programming: organize / align intro and sections / add examples ([#467](https://github.com/john-cd/rust_howto/issues/467)) ([bk/drafts/categories/rust-patterns/functional_programming.md](./bk/drafts/categories/rust-patterns/functional_programming.md#L84))

### bk/drafts/categories/rust-patterns/index.md

- [ ] P2 Resolve TODO/FIXME at line 46 ([bk/drafts/categories/rust-patterns/index.md](./bk/drafts/categories/rust-patterns/index.md#L46))
- [ ] P2 index: organize / write / add cross links ([#469](https://github.com/john-cd/rust_howto/issues/469)) ([bk/drafts/categories/rust-patterns/index.md](./bk/drafts/categories/rust-patterns/index.md#L144))

### bk/drafts/categories/rust-patterns/rust_specific_patterns.md

- [ ] P2 write ([#1393](https://github.com/john-cd/rust_howto/issues/1393)) ([bk/drafts/categories/rust-patterns/rust_specific_patterns.md](./bk/drafts/categories/rust-patterns/rust_specific_patterns.md#L68))

### bk/drafts/categories/rust-patterns/structural_patterns.md

- [ ] P2 write ([#1392](https://github.com/john-cd/rust_howto/issues/1392)) ([bk/drafts/categories/rust-patterns/structural_patterns.md](./bk/drafts/categories/rust-patterns/structural_patterns.md#L13))

### bk/drafts/categories/template-engine/index.md

- [ ] P2 template-engine/index: write ([#482](https://github.com/john-cd/rust_howto/issues/482)) ([bk/drafts/categories/template-engine/index.md](./bk/drafts/categories/template-engine/index.md#L62))

### bk/drafts/categories/template-engine/tera.md

- [ ] P2 tera: write; add sample from tools code ([#483](https://github.com/john-cd/rust_howto/issues/483)) ([bk/drafts/categories/template-engine/tera.md](./bk/drafts/categories/template-engine/tera.md#L23))

### bk/drafts/categories/template-engine/tinytemplate.md

- [ ] P2 tinytemplate: write; add sample from tools code ([#484](https://github.com/john-cd/rust_howto/issues/484)) ([bk/drafts/categories/template-engine/tinytemplate.md](./bk/drafts/categories/template-engine/tinytemplate.md#L23))

### bk/drafts/categories/text-editors/ides.md

- [ ] P2 write ([#486](https://github.com/john-cd/rust_howto/issues/486)) ([bk/drafts/categories/text-editors/ides.md](./bk/drafts/categories/text-editors/ides.md#L54))

### bk/drafts/categories/text-editors/index.md

- [ ] P2 write; review in depth finish the table ([#962](https://github.com/john-cd/rust_howto/issues/962)) ([bk/drafts/categories/text-editors/index.md](./bk/drafts/categories/text-editors/index.md#L44))

### bk/drafts/categories/text-processing/diffing.md

- [ ] P2 write; titles ([#1193](https://github.com/john-cd/rust_howto/issues/1193)) ([bk/drafts/categories/text-processing/diffing.md](./bk/drafts/categories/text-processing/diffing.md#L34))

### bk/drafts/categories/text-processing/index.md

- [ ] P2 review; address NLP ([#963](https://github.com/john-cd/rust_howto/issues/963)) ([bk/drafts/categories/text-processing/index.md](./bk/drafts/categories/text-processing/index.md#L82))

### bk/drafts/categories/text-processing/other_strings.md

- [?] P2 review ([#1194](https://github.com/john-cd/rust_howto/issues/1194)) ([bk/drafts/categories/text-processing/other_strings.md](./bk/drafts/categories/text-processing/other_strings.md#L89))

### bk/drafts/categories/text-processing/regex.md

- [ ] P2 regex: write ([#488](https://github.com/john-cd/rust_howto/issues/488)) ([bk/drafts/categories/text-processing/regex.md](./bk/drafts/categories/text-processing/regex.md#L90))

### bk/drafts/categories/text-processing/string_concat.md

- [?] P2 review ([#964](https://github.com/john-cd/rust_howto/issues/964)) ([bk/drafts/categories/text-processing/string_concat.md](./bk/drafts/categories/text-processing/string_concat.md#L31))

### bk/drafts/categories/text-processing/string_manipulation.md

- [ ] P2 write ([#1190](https://github.com/john-cd/rust_howto/issues/1190)) ([bk/drafts/categories/text-processing/string_manipulation.md](./bk/drafts/categories/text-processing/string_manipulation.md#L50))

### bk/drafts/categories/text-processing/string_parsing.md

- [?] P2 review ([#965](https://github.com/john-cd/rust_howto/issues/965)) ([bk/drafts/categories/text-processing/string_parsing.md](./bk/drafts/categories/text-processing/string_parsing.md#L26))

### bk/drafts/categories/text-processing/string_search.md

- [ ] P2 write ([#1191](https://github.com/john-cd/rust_howto/issues/1191)) ([bk/drafts/categories/text-processing/string_search.md](./bk/drafts/categories/text-processing/string_search.md#L55))

### bk/drafts/categories/text-processing/unicode.md

- [?] P2 review ([#1192](https://github.com/john-cd/rust_howto/issues/1192)) ([bk/drafts/categories/text-processing/unicode.md](./bk/drafts/categories/text-processing/unicode.md#L26))

### bk/drafts/categories/web-programming/http_types_and_interfaces.md

- [ ] P2 write ([#1338](https://github.com/john-cd/rust_howto/issues/1338)) ([bk/drafts/categories/web-programming/http_types_and_interfaces.md](./bk/drafts/categories/web-programming/http_types_and_interfaces.md#L38))

### bk/drafts/categories/web-programming/index.md

- [ ] P2 web-programming/index: organize; need full review; further cross link ([#500](https://github.com/john-cd/rust_howto/issues/500)) ([bk/drafts/categories/web-programming/index.md](./bk/drafts/categories/web-programming/index.md#L89))

### bk/drafts/categories/web-programming/mime.md

- [?] P2 review ([#971](https://github.com/john-cd/rust_howto/issues/971)) ([bk/drafts/categories/web-programming/mime.md](./bk/drafts/categories/web-programming/mime.md#L47))

### bk/drafts/categories/web-programming/scraping.md

- [ ] P2 Call `get_base_url`{{hi:get_base_url}} to retrieve the base [URL][p~url]{{hi:Base URL}}. If the document has a base tag, get the href{{hi:href}} [`select::node::Node::attr`][c~select::node::Node::attr~docs]↗{{hi:select::node::Node::attr}} from base tag. [`select::node::Node::attr`][c~select::node::Node::attr~docs]↗{{hi:select::node::Node::attr}} of the original URL acts as a default. ([bk/drafts/categories/web-programming/scraping.md](./bk/drafts/categories/web-programming/scraping.md#L19))
- [?] P2 review ([#972](https://github.com/john-cd/rust_howto/issues/972)) ([bk/drafts/categories/web-programming/scraping.md](./bk/drafts/categories/web-programming/scraping.md#L48))

### bk/drafts/categories/web-programming/url.md

- [?] P2 review ([#973](https://github.com/john-cd/rust_howto/issues/973)) ([bk/drafts/categories/web-programming/url.md](./bk/drafts/categories/web-programming/url.md#L73))

### bk/drafts/categories/web-programming_http-client/apis.md

- [ ] P2 review / fix ([#974](https://github.com/john-cd/rust_howto/issues/974)) ([bk/drafts/categories/web-programming_http-client/apis.md](./bk/drafts/categories/web-programming_http-client/apis.md#L71))

### bk/drafts/categories/web-programming_http-client/download.md

- [?] P2 review ([#975](https://github.com/john-cd/rust_howto/issues/975)) ([bk/drafts/categories/web-programming_http-client/download.md](./bk/drafts/categories/web-programming_http-client/download.md#L50))

### bk/drafts/categories/web-programming_http-client/http_clients.md

- [ ] P2 http_clients: expand; link to hyper.md in server. ([#504](https://github.com/john-cd/rust_howto/issues/504)) ([bk/drafts/categories/web-programming_http-client/http_clients.md](./bk/drafts/categories/web-programming_http-client/http_clients.md#L54))

### bk/drafts/categories/web-programming_http-client/index.md

- [ ] P2 web-programming_http-client/index: fix, sync table and contents ([#505](https://github.com/john-cd/rust_howto/issues/505)) ([bk/drafts/categories/web-programming_http-client/index.md](./bk/drafts/categories/web-programming_http-client/index.md#L55))

### bk/drafts/categories/web-programming_http-client/requests.md

- [?] P2 review ([#976](https://github.com/john-cd/rust_howto/issues/976)) ([bk/drafts/categories/web-programming_http-client/requests.md](./bk/drafts/categories/web-programming_http-client/requests.md#L54))

### bk/drafts/categories/web-programming_http-server/_actix.md

- [ ] P2 actix: organize / write ([#506](https://github.com/john-cd/rust_howto/issues/506)) ([bk/drafts/categories/web-programming_http-server/_actix.md](./bk/drafts/categories/web-programming_http-server/_actix.md#L30))

### bk/drafts/categories/web-programming_http-server/_axum.md

- [ ] P2 axum: write ([#507](https://github.com/john-cd/rust_howto/issues/507)) ([bk/drafts/categories/web-programming_http-server/_axum.md](./bk/drafts/categories/web-programming_http-server/_axum.md#L30))

### bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md

- [ ] P2 batteries-included_frameworks: write ([#509](https://github.com/john-cd/rust_howto/issues/509)) ([bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md](./bk/drafts/categories/web-programming_http-server/_batteries-included_frameworks.md#L35))

### bk/drafts/categories/web-programming_http-server/_graphql.md

- [ ] P2 graphql: write integrate in seography graphql section? ([#511](https://github.com/john-cd/rust_howto/issues/511)) ([bk/drafts/categories/web-programming_http-server/_graphql.md](./bk/drafts/categories/web-programming_http-server/_graphql.md#L38))

### bk/drafts/categories/web-programming_http-server/_grpc.md

- [ ] P2 grpc: write ([#514](https://github.com/john-cd/rust_howto/issues/514)) ([bk/drafts/categories/web-programming_http-server/_grpc.md](./bk/drafts/categories/web-programming_http-server/_grpc.md#L31))

### bk/drafts/categories/web-programming_http-server/_hyper.md

- [ ] P2 hyper: fix ([#515](https://github.com/john-cd/rust_howto/issues/515)) ([bk/drafts/categories/web-programming_http-server/_hyper.md](./bk/drafts/categories/web-programming_http-server/_hyper.md#L57))

### bk/drafts/categories/web-programming_http-server/cors.md

- [ ] P2 cors: write ([#510](https://github.com/john-cd/rust_howto/issues/510)) ([bk/drafts/categories/web-programming_http-server/cors.md](./bk/drafts/categories/web-programming_http-server/cors.md#L25))

### bk/drafts/categories/web-programming_http-server/index.md

- [ ] P2 write / review in depth ([#977](https://github.com/john-cd/rust_howto/issues/977)) ([bk/drafts/categories/web-programming_http-server/index.md](./bk/drafts/categories/web-programming_http-server/index.md#L93))

### bk/drafts/categories/web-programming_http-server/middleware.md

- [?] P2 review ([#978](https://github.com/john-cd/rust_howto/issues/978)) ([bk/drafts/categories/web-programming_http-server/middleware.md](./bk/drafts/categories/web-programming_http-server/middleware.md#L43))

### bk/drafts/categories/web-programming_http-server/other_frameworks.md

- [ ] P2 other_frameworks: organize ([#518](https://github.com/john-cd/rust_howto/issues/518)) ([bk/drafts/categories/web-programming_http-server/other_frameworks.md](./bk/drafts/categories/web-programming_http-server/other_frameworks.md#L33))

### bk/drafts/categories/web-programming_http-server/static_website_generators.md

- [ ] P2 static_website_generators: write ([#519](https://github.com/john-cd/rust_howto/issues/519)) ([bk/drafts/categories/web-programming_http-server/static_website_generators.md](./bk/drafts/categories/web-programming_http-server/static_website_generators.md#L58))

### bk/drafts/contributing/vscode.md

- [ ] P2 write ([bk/drafts/contributing/vscode.md](./bk/drafts/contributing/vscode.md#L93))

### bk/drafts/indices/crates_and_examples.md

- [ ] P2 create script to generate crate pages ([#1395](https://github.com/john-cd/rust_howto/issues/1395)) ([bk/drafts/indices/crates_and_examples.md](./bk/drafts/indices/crates_and_examples.md#L17))

### bk/drafts/indices/crates_by_category2.md

- [ ] P2 crates_by_category: verify all crates are added to at least one category ([#534](https://github.com/john-cd/rust_howto/issues/534)) ([bk/drafts/indices/crates_by_category2.md](./bk/drafts/indices/crates_by_category2.md#L2))

### bk/master/crates_used_in_drafts_and_later_sections.txt

- [ ] P2 cover the following crates in the book? ([bk/master/crates_used_in_drafts_and_later_sections.txt](./bk/master/crates_used_in_drafts_and_later_sections.txt#L1))

### bk/scripts/examples/convert_example_placeholders.sh

- [ ] P1 does not handle non-category examples ([#1257](https://github.com/john-cd/rust_howto/issues/1257)) ([bk/scripts/examples/convert_example_placeholders.sh](./bk/scripts/examples/convert_example_placeholders.sh#L21))

### bk/scripts/links/replace_inline_links.sh

- [ ] P1 fix ([#1238](https://github.com/john-cd/rust_howto/issues/1238)) ([bk/scripts/links/replace_inline_links.sh](./bk/scripts/links/replace_inline_links.sh#L16))

### bk/scripts/links/suggest_links_for_bare_urls.sh

- [ ] P1 pass a var ([#1243](https://github.com/john-cd/rust_howto/issues/1243)) ([bk/scripts/links/suggest_links_for_bare_urls.sh](./bk/scripts/links/suggest_links_for_bare_urls.sh#L40))

### bk/scripts/spelling/spellcheck.sh

- [ ] P1 Find more correct way to get line number ([bk/scripts/spelling/spellcheck.sh](./bk/scripts/spelling/spellcheck.sh#L84))

### bk/src/appendices/contributing/book_editing_and_example_code_development.md

- [?] P1 development_editing: review ([#523](https://github.com/john-cd/rust_howto/issues/523)) ([bk/src/appendices/contributing/book_editing_and_example_code_development.md](./bk/src/appendices/contributing/book_editing_and_example_code_development.md#L32))

### bk/src/appendices/contributing/index.md

- [ ] P1 Documentation: Finalize Contributing Guide and Styleguides ([#529](https://github.com/john-cd/rust_howto/issues/529)) ([bk/src/appendices/contributing/index.md](./bk/src/appendices/contributing/index.md#L175))

### bk/src/categories/data-structures/index.md

- [ ] P1 data-structures: expand ([#280](https://github.com/john-cd/rust_howto/issues/280)) ([bk/src/categories/data-structures/index.md](./bk/src/categories/data-structures/index.md#L101))

### bk/src/categories/mathematics/complex_numbers.md

- [?] P1 final review ([#935](https://github.com/john-cd/rust_howto/issues/935)) ([bk/src/categories/mathematics/complex_numbers.md](./bk/src/categories/mathematics/complex_numbers.md#L64))

### bk/src/categories/mathematics/trigonometry.md

- [?] P1 final review ([#938](https://github.com/john-cd/rust_howto/issues/938)) ([bk/src/categories/mathematics/trigonometry.md](./bk/src/categories/mathematics/trigonometry.md#L45))

### bk/src/language/macros.md

- [ ] P1 split? ([bk/src/language/macros.md](./bk/src/language/macros.md#L427))

### bk/src/standard-library/dynamic_typing.md

- [?] P1 review ([bk/src/standard-library/dynamic_typing.md](./bk/src/standard-library/dynamic_typing.md#L54))

## Workspace Tooling and Infrastructure

### .devcontainer/ci.sh

- [ ] P1 add cargo plugins to CI script ([#1277](https://github.com/john-cd/rust_howto/issues/1277)) ([.devcontainer/ci.sh](./.devcontainer/ci.sh#L40))

### playground/bits_and_pieces/buildinfo/build-info.rs

- [?] P3 review / convert into example ? ([#1060](https://github.com/john-cd/rust_howto/issues/1060)) ([playground/bits_and_pieces/buildinfo/build-info.rs](./playground/bits_and_pieces/buildinfo/build-info.rs#L49))

### playground/bits_and_pieces/just/justfile

- [ ] P1 would like to use `bash` as the shell, when on Windows, rather than ([#1249](https://github.com/john-cd/rust_howto/issues/1249)) ([playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L9))
- [ ] P3 make cypath conversion work ([#1251](https://github.com/john-cd/rust_howto/issues/1251)) ([playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L52))
- [ ] P1 convert Windows path to Cygwin path ([#1252](https://github.com/john-cd/rust_howto/issues/1252)) ([playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L62))
- [?] P3 review https://github.com/casey/just/issues/2599 ([#1253](https://github.com/john-cd/rust_howto/issues/1253)) ([playground/bits_and_pieces/just/justfile](./playground/bits_and_pieces/just/justfile#L70))

### playground/bits_and_pieces/pre-commit/.pre-commit-config.yaml

- [ ] P3 revise git hooks ([#1264](https://github.com/john-cd/rust_howto/issues/1264)) ([playground/bits_and_pieces/pre-commit/.pre-commit-config.yaml](./playground/bits_and_pieces/pre-commit/.pre-commit-config.yaml#L12))

### playground/bits_and_pieces/walk_dir.rs

- [ ] P3 replace_in_file(path)?; ([#1439](https://github.com/john-cd/rust_howto/issues/1439)) ([playground/bits_and_pieces/walk_dir.rs](./playground/bits_and_pieces/walk_dir.rs#L34))

### tools/mdbook-scrub/src/regexes.rs

- [ ] P2 share RegexAndReplacement code with CLIs (core_lib) ([#1420](https://github.com/john-cd/rust_howto/issues/1420)) ([tools/mdbook-scrub/src/regexes.rs](./tools/mdbook-scrub/src/regexes.rs#L6))

### tools/mdbook-scrub/test_book/src/category_badges.md

- [ ] P2 NOT IMPLEMENTED. ([tools/mdbook-scrub/test_book/src/category_badges.md](./tools/mdbook-scrub/test_book/src/category_badges.md#L3))

### tools/mdbook-scrub/test_book/src/category_links.md

- [ ] P2 NOT IMPLEMENTED. ([tools/mdbook-scrub/test_book/src/category_links.md](./tools/mdbook-scrub/test_book/src/category_links.md#L3))

### tools/mdbook-scrub/test_book/src/crate_badges.md

- [ ] P2 NOT IMPLEMENTED. ([tools/mdbook-scrub/test_book/src/crate_badges.md](./tools/mdbook-scrub/test_book/src/crate_badges.md#L3))

### tools/mdbook-scrub/test_book/src/crate_links.md

- [ ] P2 NOT IMPLEMENTED. ([tools/mdbook-scrub/test_book/src/crate_links.md](./tools/mdbook-scrub/test_book/src/crate_links.md#L3))

### tools/tool_lib/src/crates_io/info.rs

- [ ] P2 unit tests ([#1358](https://github.com/john-cd/rust_howto/issues/1358)) ([tools/tool_lib/src/crates_io/info.rs](./tools/tool_lib/src/crates_io/info.rs#L12))

### tools/tool_lib/src/crates_io/mod.rs

- [ ] P2 unit tests; cleanup ([#1356](https://github.com/john-cd/rust_howto/issues/1356)) ([tools/tool_lib/src/crates_io/mod.rs](./tools/tool_lib/src/crates_io/mod.rs#L53))

### tools/tool_lib/src/read_write/ext.rs

- [ ] P2 file_prefix is still unstable. ([#902](https://github.com/john-cd/rust_howto/issues/902)) ([tools/tool_lib/src/read_write/ext.rs](./tools/tool_lib/src/read_write/ext.rs#L48))

### tools/tool_lib/src/templates/mod.rs

- [ ] P2 finish; unit tests; tinytemplate vs tera? ([#1361](https://github.com/john-cd/rust_howto/issues/1361)) ([tools/tool_lib/src/templates/mod.rs](./tools/tool_lib/src/templates/mod.rs#L71))

### xmpl/clap_builder_xmpl/src/cli/args.rs

- [ ] P3 fix test ([#895](https://github.com/john-cd/rust_howto/issues/895)) ([xmpl/clap_builder_xmpl/src/cli/args.rs](./xmpl/clap_builder_xmpl/src/cli/args.rs#L203))

## Future / Staging Content

### later/crates/cats/aerospace_space_protocols/examples/space_protocols/space_protocols.rs

- [ ] P3 review the space_protocols crate and add more examples for telemetry, ([later/crates/cats/aerospace_space_protocols/examples/space_protocols/space_protocols.rs](./later/crates/cats/aerospace_space_protocols/examples/space_protocols/space_protocols.rs#L42))

### later/crates/cats/computer_vision/examples/opencv/opencv.rs

- [ ] P3 review; expand example; review https://blog.devgenius.io/rust-and-opencv-bb0467bf35ff ([#1079](https://github.com/john-cd/rust_howto/issues/1079)) ([later/crates/cats/computer_vision/examples/opencv/opencv.rs](./later/crates/cats/computer_vision/examples/opencv/opencv.rs#L73))

### later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs

- [ ] P3 write ([later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs](./later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs#L43))

### later/crates/cats/development_tools_ffi/build.rs

- [ ] P3 fix build.rs for ffi ([#1026](https://github.com/john-cd/rust_howto/issues/1026)) ([later/crates/cats/development_tools_ffi/build.rs](./later/crates/cats/development_tools_ffi/build.rs#L80))

### later/crates/cats/development_tools_ffi/examples/c/bindgen.rs

- [ ] P3 fix ([#1001](https://github.com/john-cd/rust_howto/issues/1001)) ([later/crates/cats/development_tools_ffi/examples/c/bindgen.rs](./later/crates/cats/development_tools_ffi/examples/c/bindgen.rs#L61))

### later/crates/cats/development_tools_ffi/examples/c/cbindgen.rs

- [ ] P3 fix; see build.rs ([#1002](https://github.com/john-cd/rust_howto/issues/1002)) ([later/crates/cats/development_tools_ffi/examples/c/cbindgen.rs](./later/crates/cats/development_tools_ffi/examples/c/cbindgen.rs#L63))

### later/crates/cats/development_tools_ffi/examples/cpp/cxx.rs

- [ ] P3 finish ([#738](https://github.com/john-cd/rust_howto/issues/738)) ([later/crates/cats/development_tools_ffi/examples/cpp/cxx.rs](./later/crates/cats/development_tools_ffi/examples/cpp/cxx.rs#L39))

### later/crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs

- [?] P3 review; how to test ([#1080](https://github.com/john-cd/rust_howto/issues/1080)) ([later/crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs](./later/crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs#L53))

### later/crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs

- [ ] P3 finish; reorg as a project using flutter_rust_bridge_codegen ([#1028](https://github.com/john-cd/rust_howto/issues/1028)) ([later/crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs](./later/crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs#L52))

### later/crates/cats/development_tools_ffi/examples/java/jni.rs

- [ ] P3 finish ([#1029](https://github.com/john-cd/rust_howto/issues/1029)) ([later/crates/cats/development_tools_ffi/examples/java/jni.rs](./later/crates/cats/development_tools_ffi/examples/java/jni.rs#L72))

### later/crates/cats/development_tools_ffi/examples/java/main.rs

- [ ] P3 review Windows support for Java FFI examples ([later/crates/cats/development_tools_ffi/examples/java/main.rs](./later/crates/cats/development_tools_ffi/examples/java/main.rs#L1))

### later/crates/cats/development_tools_ffi/examples/node/napi.rs

- [ ] P3 finish; https://lib.rs/crates/napi; https://github.com/napi-rs/package-template ([#1032](https://github.com/john-cd/rust_howto/issues/1032)) ([later/crates/cats/development_tools_ffi/examples/node/napi.rs](./later/crates/cats/development_tools_ffi/examples/node/napi.rs#L30))

### later/crates/cats/development_tools_ffi/examples/node/neon.rs

- [ ] P3 main().unwrap(); ([later/crates/cats/development_tools_ffi/examples/node/neon.rs](./later/crates/cats/development_tools_ffi/examples/node/neon.rs#L57))
- [ ] P3 finish how to test ? ([#1033](https://github.com/john-cd/rust_howto/issues/1033)) ([later/crates/cats/development_tools_ffi/examples/node/neon.rs](./later/crates/cats/development_tools_ffi/examples/node/neon.rs#L60))

### later/crates/cats/development_tools_ffi/examples/objc/objc2.rs

- [ ] P3 finish; fix; https://docs.rs/objc2/latest/objc2/ ([#1034](https://github.com/john-cd/rust_howto/issues/1034)) ([later/crates/cats/development_tools_ffi/examples/objc/objc2.rs](./later/crates/cats/development_tools_ffi/examples/objc/objc2.rs#L50))

### later/crates/cats/development_tools_ffi/examples/python/pyo3.rs

- [ ] P3 finish; fix py examples ([#78](https://github.com/john-cd/rust_howto/issues/78)) ([later/crates/cats/development_tools_ffi/examples/python/pyo3.rs](./later/crates/cats/development_tools_ffi/examples/python/pyo3.rs#L80))

### later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs

- [ ] P3 Add code to build the Python module and demonstrate how to call it ([later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs](./later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs#L54))
- [ ] P3 finish; fix ([#996](https://github.com/john-cd/rust_howto/issues/996)) ([later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs](./later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs#L71))

### later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs

- [ ] P3 finish ([#1035](https://github.com/john-cd/rust_howto/issues/1035)) ([later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs](./later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs#L53))

### later/crates/cats/development_tools_ffi/examples/ruby/main.rs

- [ ] P3 review Windows support for Ruby FFI examples ([later/crates/cats/development_tools_ffi/examples/ruby/main.rs](./later/crates/cats/development_tools_ffi/examples/ruby/main.rs#L4))

### later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs

- [ ] P3 finish ([#1036](https://github.com/john-cd/rust_howto/issues/1036)) ([later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs](./later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs#L51))

### later/crates/cats/development_tools_ffi/examples/uniffi.rs

- [ ] P3 finish; https://github.com/mozilla/uniffi-rs/blob/main/examples/arithmetic/Cargo.toml ([#1037](https://github.com/john-cd/rust_howto/issues/1037)) ([later/crates/cats/development_tools_ffi/examples/uniffi.rs](./later/crates/cats/development_tools_ffi/examples/uniffi.rs#L33))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs

- [ ] P3 finish ([#744](https://github.com/john-cd/rust_howto/issues/744)) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs#L91))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs

- [ ] P3 finish ([#739](https://github.com/john-cd/rust_howto/issues/739)) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/darling.rs#L82))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/main.rs

- [ ] P3 finish; review the following; decide what examples are needed ([#1158](https://github.com/john-cd/rust_howto/issues/1158)) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/main.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/main.rs#L61))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs

- [ ] P3 finish ([#741](https://github.com/john-cd/rust_howto/issues/741)) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/proc_macro2.rs#L71))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs

- [ ] P3 review / expand ([#742](https://github.com/john-cd/rust_howto/issues/742)) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/quote.rs#L39))

### later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs

- [ ] P3 finish. See https://github.com/dtolnay/syn/tree/master/examples ([#743](https://github.com/john-cd/rust_howto/issues/743)) ([later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs](./later/crates/cats/development_tools_procedural_macro_helpers/examples/write_proc_macros/syn.rs#L87))

### later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs

- [ ] P3 finish ([#1157](https://github.com/john-cd/rust_howto/issues/1157)) ([later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs](./later/crates/cats/development_tools_procedural_macro_helpers/src/lib.rs#L86))

### later/crates/cats/embedded/examples/embassy/embassy.rs

- [ ] P3 finish ([#751](https://github.com/john-cd/rust_howto/issues/751)) ([later/crates/cats/embedded/examples/embassy/embassy.rs](./later/crates/cats/embedded/examples/embassy/embassy.rs#L95))

### later/crates/cats/finance/Cargo.toml

- [ ] P3 RustQuant = { version = "0.3", optional = true } ([later/crates/cats/finance/Cargo.toml](./later/crates/cats/finance/Cargo.toml#L18))

### later/crates/cats/finance/examples/quant/rustquant.rs

- [ ] P3 finish; review ([#764](https://github.com/john-cd/rust_howto/issues/764)) ([later/crates/cats/finance/examples/quant/rustquant.rs](./later/crates/cats/finance/examples/quant/rustquant.rs#L188))

### later/crates/cats/game_development/examples/game_development/game_development1.rs

- [?] P3 review ([later/crates/cats/game_development/examples/game_development/game_development1.rs](./later/crates/cats/game_development/examples/game_development/game_development1.rs#L58))

### later/crates/cats/game_development/examples/game_development/glam.rs

- [ ] P3 finish ([#770](https://github.com/john-cd/rust_howto/issues/770)) ([later/crates/cats/game_development/examples/game_development/glam.rs](./later/crates/cats/game_development/examples/game_development/glam.rs#L55))

### later/crates/cats/game_engines/Cargo.toml

- [ ] P3 fyrox = ["dep:fyrox", "dep:fyrox-core"] ([later/crates/cats/game_engines/Cargo.toml](./later/crates/cats/game_engines/Cargo.toml#L24))
- [ ] P3 fyrox-core = { version = "0.36.0", optional = true } ([later/crates/cats/game_engines/Cargo.toml](./later/crates/cats/game_engines/Cargo.toml#L28))
- [?] P3 review on Windows ([later/crates/cats/game_engines/Cargo.toml](./later/crates/cats/game_engines/Cargo.toml#L35))

### later/crates/cats/game_engines/examples/game_engines/bevy.rs

- [ ] P3 Placeholder until the Bevy demo is reintroduced. ([later/crates/cats/game_engines/examples/game_engines/bevy.rs](./later/crates/cats/game_engines/examples/game_engines/bevy.rs#L49))
- [ ] P3 finish ([#767](https://github.com/john-cd/rust_howto/issues/767)) ([later/crates/cats/game_engines/examples/game_engines/bevy.rs](./later/crates/cats/game_engines/examples/game_engines/bevy.rs#L62))

### later/crates/cats/game_engines/examples/game_engines/fyrox.rs

- [ ] P3 finish ([#768](https://github.com/john-cd/rust_howto/issues/768)) ([later/crates/cats/game_engines/examples/game_engines/fyrox.rs](./later/crates/cats/game_engines/examples/game_engines/fyrox.rs#L84))

### later/crates/cats/game_engines/examples/game_engines/ggez.rs

- [ ] P3 finish ([#769](https://github.com/john-cd/rust_howto/issues/769)) ([later/crates/cats/game_engines/examples/game_engines/ggez.rs](./later/crates/cats/game_engines/examples/game_engines/ggez.rs#L85))

### later/crates/cats/game_engines/examples/game_engines/macroquad.rs

- [ ] P3 finish need proper testing ([#771](https://github.com/john-cd/rust_howto/issues/771)) ([later/crates/cats/game_engines/examples/game_engines/macroquad.rs](./later/crates/cats/game_engines/examples/game_engines/macroquad.rs#L53))

### later/crates/cats/games/examples/games.rs

- [?] P3 review ([later/crates/cats/games/examples/games.rs](./later/crates/cats/games/examples/games.rs#L76))

### later/crates/cats/gui/Cargo.toml

- [ ] P3 vizia = ["dep:vizia"] ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L32))
- [ ] P3 size-of v0.1.5 does not compile on Windows ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L54))
- [ ] P3 r3bl_tuify = "0.2.0" ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L55))
- [ ] P3 review GTK installation requirements on Linux ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L57))
- [ ] P3 review GTK installation requirements on Windows ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L58))
- [ ] P3 pinned for vizia to 0.93.1 and to 0.90 for slint ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L62))
- [ ] P3 disabled: incompatible naga 28 compile path ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L67))
- [ ] P3 conflict with Vello ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L69))
- [ ] P3 vizia = { version = "0.4.0", optional = true } ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L70))
- [ ] P3 allow on Windows; see install_deps.ps1 ([later/crates/cats/gui/Cargo.toml](./later/crates/cats/gui/Cargo.toml#L76))

### later/crates/cats/gui/examples/2d_renderers/femtovg.rs

- [ ] P3 finish ([#777](https://github.com/john-cd/rust_howto/issues/777)) ([later/crates/cats/gui/examples/2d_renderers/femtovg.rs](./later/crates/cats/gui/examples/2d_renderers/femtovg.rs#L152))

### later/crates/cats/gui/examples/2d_renderers/main.rs

- [ ] P3 review Vello support for Windows ([later/crates/cats/gui/examples/2d_renderers/main.rs](./later/crates/cats/gui/examples/2d_renderers/main.rs#L11))
- [ ] P3 Webrender support Windows ([later/crates/cats/gui/examples/2d_renderers/main.rs](./later/crates/cats/gui/examples/2d_renderers/main.rs#L16))

### later/crates/cats/gui/examples/2d_renderers/minifb.rs

- [?] P3 review ([later/crates/cats/gui/examples/2d_renderers/minifb.rs](./later/crates/cats/gui/examples/2d_renderers/minifb.rs#L69))

### later/crates/cats/gui/examples/2d_renderers/skia_safe.rs

- [ ] P3 finish ([#786](https://github.com/john-cd/rust_howto/issues/786)) ([later/crates/cats/gui/examples/2d_renderers/skia_safe.rs](./later/crates/cats/gui/examples/2d_renderers/skia_safe.rs#L205))

### later/crates/cats/gui/examples/2d_renderers/vger.rs

- [ ] P3 finish ([#792](https://github.com/john-cd/rust_howto/issues/792)) ([later/crates/cats/gui/examples/2d_renderers/vger.rs](./later/crates/cats/gui/examples/2d_renderers/vger.rs#L77))

### later/crates/cats/gui/examples/clipboard/arboard.rs

- [?] P3 review ([#1155](https://github.com/john-cd/rust_howto/issues/1155)) ([later/crates/cats/gui/examples/clipboard/arboard.rs](./later/crates/cats/gui/examples/clipboard/arboard.rs#L45))

### later/crates/cats/gui/examples/file_dialogs/rfd.rs

- [ ] P3 how to test ([#785](https://github.com/john-cd/rust_howto/issues/785)) ([later/crates/cats/gui/examples/file_dialogs/rfd.rs](./later/crates/cats/gui/examples/file_dialogs/rfd.rs#L33))

### later/crates/cats/gui/examples/gtk/gtk4.rs

- [ ] P3 finish; review https://gtk-rs.org/ ([#780](https://github.com/john-cd/rust_howto/issues/780)) ([later/crates/cats/gui/examples/gtk/gtk4.rs](./later/crates/cats/gui/examples/gtk/gtk4.rs#L109))

### later/crates/cats/gui/examples/gtk/relm4.rs

- [ ] P3 finish; review https://relm4.org/book/stable/ ([#784](https://github.com/john-cd/rust_howto/issues/784)) ([later/crates/cats/gui/examples/gtk/relm4.rs](./later/crates/cats/gui/examples/gtk/relm4.rs#L336))

### later/crates/cats/gui/examples/immediate_mode_gui/egui.rs

- [ ] P3 finish verify the code manually ([#776](https://github.com/john-cd/rust_howto/issues/776)) ([later/crates/cats/gui/examples/immediate_mode_gui/egui.rs](./later/crates/cats/gui/examples/immediate_mode_gui/egui.rs#L60))

### later/crates/cats/gui/examples/retained_mode_gui/floem.rs

- [ ] P3 finish ([#778](https://github.com/john-cd/rust_howto/issues/778)) ([later/crates/cats/gui/examples/retained_mode_gui/floem.rs](./later/crates/cats/gui/examples/retained_mode_gui/floem.rs#L78))

### later/crates/cats/gui/examples/retained_mode_gui/iced.rs

- [ ] P3 finish ([#781](https://github.com/john-cd/rust_howto/issues/781)) ([later/crates/cats/gui/examples/retained_mode_gui/iced.rs](./later/crates/cats/gui/examples/retained_mode_gui/iced.rs#L101))

### later/crates/cats/gui/examples/retained_mode_gui/slint.rs

- [ ] P3 finish; figure how to test - neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set. see vs code Wayland setting ([#787](https://github.com/john-cd/rust_howto/issues/787)) ([later/crates/cats/gui/examples/retained_mode_gui/slint.rs](./later/crates/cats/gui/examples/retained_mode_gui/slint.rs#L34))

### later/crates/cats/gui/examples/text_layout/cosmic_text.rs

- [ ] P3 finish ([#774](https://github.com/john-cd/rust_howto/issues/774)) ([later/crates/cats/gui/examples/text_layout/cosmic_text.rs](./later/crates/cats/gui/examples/text_layout/cosmic_text.rs#L67))

### later/crates/cats/gui/examples/text_layout/parley.rs

- [ ] P3 finish ([#783](https://github.com/john-cd/rust_howto/issues/783)) ([later/crates/cats/gui/examples/text_layout/parley.rs](./later/crates/cats/gui/examples/text_layout/parley.rs#L110))

### later/crates/cats/gui/examples/ui_layout/morphorm.rs

- [ ] P3 finish ([#782](https://github.com/john-cd/rust_howto/issues/782)) ([later/crates/cats/gui/examples/ui_layout/morphorm.rs](./later/crates/cats/gui/examples/ui_layout/morphorm.rs#L114))

### later/crates/cats/gui/examples/ui_layout/taffy.rs

- [ ] P3 finish ([#788](https://github.com/john-cd/rust_howto/issues/788)) ([later/crates/cats/gui/examples/ui_layout/taffy.rs](./later/crates/cats/gui/examples/ui_layout/taffy.rs#L144))

### later/crates/cats/gui/examples/web/dioxus.rs

- [ ] P3 finish ([#775](https://github.com/john-cd/rust_howto/issues/775)) ([later/crates/cats/gui/examples/web/dioxus.rs](./later/crates/cats/gui/examples/web/dioxus.rs#L48))

### later/crates/cats/gui/examples/web/main.rs

- [ ] P3 review Windows support ([later/crates/cats/gui/examples/web/main.rs](./later/crates/cats/gui/examples/web/main.rs#L5))

### later/crates/cats/gui/examples/window_creation/main.rs

- [ ] P3 review Windows support for window creation examples ([later/crates/cats/gui/examples/window_creation/main.rs](./later/crates/cats/gui/examples/window_creation/main.rs#L4))

### later/crates/cats/gui/examples/window_creation/tao.rs

- [ ] P3 finish ([#789](https://github.com/john-cd/rust_howto/issues/789)) ([later/crates/cats/gui/examples/window_creation/tao.rs](./later/crates/cats/gui/examples/window_creation/tao.rs#L84))

### later/crates/cats/gui/examples/window_creation/winit.rs

- [ ] P3 proper test ([later/crates/cats/gui/examples/window_creation/winit.rs](./later/crates/cats/gui/examples/window_creation/winit.rs#L100))

### later/crates/cats/multimedia/examples/multimedia/multimedia1.rs

- [ ] P3 review - this test is not really a unit test, but it does verify that we can create an output stream without panicking. It will fail if there is no audio output device available, which is common in CI environments. We could consider using a mock or virtual audio device for testing in the future. ([later/crates/cats/multimedia/examples/multimedia/multimedia1.rs](./later/crates/cats/multimedia/examples/multimedia/multimedia1.rs#L46))

### later/crates/cats/multimedia_video/examples/video/video.rs

- [?] P3 review; TODO call main?? ([#809](https://github.com/john-cd/rust_howto/issues/809)) ([later/crates/cats/multimedia_video/examples/video/video.rs](./later/crates/cats/multimedia_video/examples/video/video.rs#L50))

### later/crates/cats/no_std/examples/no_std/no_std1.rs

- [?] P3 review ([#814](https://github.com/john-cd/rust_howto/issues/814)) ([later/crates/cats/no_std/examples/no_std/no_std1.rs](./later/crates/cats/no_std/examples/no_std/no_std1.rs#L60))

### later/crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs

- [?] P3 review ([later/crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs](./later/crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs#L52))

### later/crates/cats/rendering/examples/2d_raster_graphics/render.rs

- [ ] P3 write ([#828](https://github.com/john-cd/rust_howto/issues/828)) ([later/crates/cats/rendering/examples/2d_raster_graphics/render.rs](./later/crates/cats/rendering/examples/2d_raster_graphics/render.rs#L24))

### later/crates/cats/rendering_data_formats/examples/data_formats/data_formats.rs

- [ ] P3 write LATER ([#829](https://github.com/john-cd/rust_howto/issues/829)) ([later/crates/cats/rendering_data_formats/examples/data_formats/data_formats.rs](./later/crates/cats/rendering_data_formats/examples/data_formats/data_formats.rs#L21))

### later/crates/cats/rendering_engine/examples/rendering_engines/render.rs

- [ ] P3 write ([#830](https://github.com/john-cd/rust_howto/issues/830)) ([later/crates/cats/rendering_engine/examples/rendering_engines/render.rs](./later/crates/cats/rendering_engine/examples/rendering_engines/render.rs#L23))

### later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/graphics.rs

- [ ] P3 write LATER ([#831](https://github.com/john-cd/rust_howto/issues/831)) ([later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/graphics.rs](./later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/graphics.rs#L23))

### later/crates/cats/science/Cargo.toml

- [ ] P3 upgrade to 0.17.2+ ([later/crates/cats/science/Cargo.toml](./later/crates/cats/science/Cargo.toml#L28))

### later/crates/cats/science/examples/ml/candle.rs

- [ ] P3 finish review https://huggingface.github.io/candle/index.html ([#835](https://github.com/john-cd/rust_howto/issues/835)) ([later/crates/cats/science/examples/ml/candle.rs](./later/crates/cats/science/examples/ml/candle.rs#L112))

### later/crates/cats/science/examples/ml/smartcore.rs

- [ ] P3 finish; kNN example; syn data generation for linear regr.; 70/30 split; etc ([#837](https://github.com/john-cd/rust_howto/issues/837)) ([later/crates/cats/science/examples/ml/smartcore.rs](./later/crates/cats/science/examples/ml/smartcore.rs#L90))

### later/crates/cats/science_geo/examples/geo/geo.rs

- [?] P3 review ([#839](https://github.com/john-cd/rust_howto/issues/839)) ([later/crates/cats/science_geo/examples/geo/geo.rs](./later/crates/cats/science_geo/examples/geo/geo.rs#L42))

### later/crates/cats/science_neuroscience/examples/neuroscience/neuro.rs

- [?] P3 review ([#840](https://github.com/john-cd/rust_howto/issues/840)) ([later/crates/cats/science_neuroscience/examples/neuroscience/neuro.rs](./later/crates/cats/science_neuroscience/examples/neuroscience/neuro.rs#L47))

### later/crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs

- [ ] P3 review / expand ([#841](https://github.com/john-cd/rust_howto/issues/841)) ([later/crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs](./later/crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs#L135))

### later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs

- [ ] P3 review Windows support ([later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs](./later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs#L1))
- [?] P3 review ([#844](https://github.com/john-cd/rust_howto/issues/844)) ([later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs](./later/crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs#L49))

### later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs

- [ ] P3 write ([#843](https://github.com/john-cd/rust_howto/issues/843)) ([later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs](./later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs#L168))

### later/crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs

- [ ] P3 write ([#845](https://github.com/john-cd/rust_howto/issues/845)) ([later/crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs](./later/crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs#L89))

### later/crates/cats/simulation/examples/physics_engines/main.rs

- [ ] P3 add a Windows-compatible physics engine example ([later/crates/cats/simulation/examples/physics_engines/main.rs](./later/crates/cats/simulation/examples/physics_engines/main.rs#L6))

### later/crates/cats/simulation/examples/physics_engines/rapier2d.rs

- [ ] P3 review / add to a chapter on physics engines with rapier2d ([later/crates/cats/simulation/examples/physics_engines/rapier2d.rs](./later/crates/cats/simulation/examples/physics_engines/rapier2d.rs#L75))

### later/crates/cats/simulation/examples/simulation/main.rs

- [ ] P3 review Windows support for simulation examples ([later/crates/cats/simulation/examples/simulation/main.rs](./later/crates/cats/simulation/examples/simulation/main.rs#L1))

### later/crates/cats/simulation/examples/simulation/simulation1.rs

- [?] P3 review ([#846](https://github.com/john-cd/rust_howto/issues/846)) ([later/crates/cats/simulation/examples/simulation/simulation1.rs](./later/crates/cats/simulation/examples/simulation/simulation1.rs#L87))

### later/crates/cats/visualization/examples/visualization/plotly.rs

- [?] P3 [review how to test]( ([#884](https://github.com/john-cd/rust_howto/issues/884)) ([later/crates/cats/visualization/examples/visualization/plotly.rs](./later/crates/cats/visualization/examples/visualization/plotly.rs#L58))

### later/crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs

- [?] P3 review; https://docs.wasmtime.dev/introduction.html https://docs.rs/wasmtime/latest/wasmtime/ ([#855](https://github.com/john-cd/rust_howto/issues/855)) ([later/crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs](./later/crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs#L134))

### later/crates/cats/wasm/examples/yew/yew.rs

- [?] P3 review ([#856](https://github.com/john-cd/rust_howto/issues/856)) ([later/crates/cats/wasm/examples/yew/yew.rs](./later/crates/cats/wasm/examples/yew/yew.rs#L69))

### later/crates/cats/web_programming_websocket/examples/websocket/async_tungstenite.rs

- [ ] P3 finish; echo.websocket.org has moved permanently ([#1058](https://github.com/john-cd/rust_howto/issues/1058)) ([later/crates/cats/web_programming_websocket/examples/websocket/async_tungstenite.rs](./later/crates/cats/web_programming_websocket/examples/websocket/async_tungstenite.rs#L75))

### later/crates/cats/web_programming_websocket/examples/websocket/tokio_tungstenite.rs

- [?] P3 review https://github.com/snapview/tokio-tungstenite/tree/master/examples ([#1145](https://github.com/john-cd/rust_howto/issues/1145)) ([later/crates/cats/web_programming_websocket/examples/websocket/tokio_tungstenite.rs](./later/crates/cats/web_programming_websocket/examples/websocket/tokio_tungstenite.rs#L66))

### later/crates/other/Cargo.toml

- [ ] P3 optional polars and datafusion dependencies in the same package pull in incompatible chrono versions (polars-arrow vs datafusion). ([later/crates/other/Cargo.toml](./later/crates/other/Cargo.toml#L89))

### later/crates/other/examples/cloud/aws_lambda.rs

- [ ] P3 finish ([#878](https://github.com/john-cd/rust_howto/issues/878)) ([later/crates/other/examples/cloud/aws_lambda.rs](./later/crates/other/examples/cloud/aws_lambda.rs#L70))

### later/src/categories/aerospace_protocols/index.md

- [ ] P3 aerospace protocols: write ([#196](https://github.com/john-cd/rust_howto/issues/196)) ([later/src/categories/aerospace_protocols/index.md](./later/src/categories/aerospace_protocols/index.md#L66))

### later/src/categories/aerospace_simulation/aerospace_simulation.md

- [ ] P3 aerospace_simulation: write ([#199](https://github.com/john-cd/rust_howto/issues/199)) ([later/src/categories/aerospace_simulation/aerospace_simulation.md](./later/src/categories/aerospace_simulation/aerospace_simulation.md#L20))

### later/src/categories/aerospace_space-protocols/space_protocols.md

- [ ] P3 space_protocols: write ([#201](https://github.com/john-cd/rust_howto/issues/201)) ([later/src/categories/aerospace_space-protocols/space_protocols.md](./later/src/categories/aerospace_space-protocols/space_protocols.md#L20))

### later/src/categories/compilers/index.md

- [?] P3 review ([#908](https://github.com/john-cd/rust_howto/issues/908)) ([later/src/categories/compilers/index.md](./later/src/categories/compilers/index.md#L80))

### later/src/categories/computer-vision/opencv.md

- [ ] P3 opencv: expand, add example ([#257](https://github.com/john-cd/rust_howto/issues/257)) ([later/src/categories/computer-vision/opencv.md](./later/src/categories/computer-vision/opencv.md#L51))

### later/src/categories/cryptography_cryptocurrencies/index.md

- [ ] P3 cryptocurrencies: write ([#278](https://github.com/john-cd/rust_howto/issues/278)) ([later/src/categories/cryptography_cryptocurrencies/index.md](./later/src/categories/cryptography_cryptocurrencies/index.md#L41))

### later/src/categories/development-tools_ffi/erlang_elixir.md

- [ ] P3 write ([#1070](https://github.com/john-cd/rust_howto/issues/1070)) ([later/src/categories/development-tools_ffi/erlang_elixir.md](./later/src/categories/development-tools_ffi/erlang_elixir.md#L48))

### later/src/categories/development-tools_ffi/flutter.md

- [ ] P3 write; review in depth ([#1071](https://github.com/john-cd/rust_howto/issues/1071)) ([later/src/categories/development-tools_ffi/flutter.md](./later/src/categories/development-tools_ffi/flutter.md#L53))

### later/src/categories/development-tools_ffi/generate_ffi_bindings.md

- [ ] P3 generate_ffi_bindings: write ([#324](https://github.com/john-cd/rust_howto/issues/324)) ([later/src/categories/development-tools_ffi/generate_ffi_bindings.md](./later/src/categories/development-tools_ffi/generate_ffi_bindings.md#L85))

### later/src/categories/development-tools_ffi/index.md

- [ ] P3 write ([#325](https://github.com/john-cd/rust_howto/issues/325)) ([later/src/categories/development-tools_ffi/index.md](./later/src/categories/development-tools_ffi/index.md#L86))

### later/src/categories/development-tools_ffi/java.md

- [ ] P3 write ([#1072](https://github.com/john-cd/rust_howto/issues/1072)) ([later/src/categories/development-tools_ffi/java.md](./later/src/categories/development-tools_ffi/java.md#L35))

### later/src/categories/development-tools_ffi/lua.md

- [ ] P3 write ([#1073](https://github.com/john-cd/rust_howto/issues/1073)) ([later/src/categories/development-tools_ffi/lua.md](./later/src/categories/development-tools_ffi/lua.md#L37))

### later/src/categories/development-tools_ffi/node.md

- [ ] P3 write ([#1074](https://github.com/john-cd/rust_howto/issues/1074)) ([later/src/categories/development-tools_ffi/node.md](./later/src/categories/development-tools_ffi/node.md#L48))

### later/src/categories/development-tools_ffi/objc.md

- [ ] P3 write ([#1075](https://github.com/john-cd/rust_howto/issues/1075)) ([later/src/categories/development-tools_ffi/objc.md](./later/src/categories/development-tools_ffi/objc.md#L35))

### later/src/categories/development-tools_ffi/python.md

- [ ] P3 python_interop: write ([#210](https://github.com/john-cd/rust_howto/issues/210)) ([later/src/categories/development-tools_ffi/python.md](./later/src/categories/development-tools_ffi/python.md#L59))

### later/src/categories/development-tools_ffi/ruby.md

- [ ] P3 write ([#1076](https://github.com/john-cd/rust_howto/issues/1076)) ([later/src/categories/development-tools_ffi/ruby.md](./later/src/categories/development-tools_ffi/ruby.md#L49))

### later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md

- [ ] P3 compile_macros: write ([#327](https://github.com/john-cd/rust_howto/issues/327)) ([later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md](./later/src/categories/development-tools_procedural-macro-helpers/compile_macros.md#L30))

### later/src/categories/development-tools_procedural-macro-helpers/index.md

- [ ] P3 fix ([#332](https://github.com/john-cd/rust_howto/issues/332)) ([later/src/categories/development-tools_procedural-macro-helpers/index.md](./later/src/categories/development-tools_procedural-macro-helpers/index.md#L34))

### later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md

- [ ] P3 tools: write ([#329](https://github.com/john-cd/rust_howto/issues/329)) ([later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md](./later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md#L27))

### later/src/categories/development-tools_procedural-macro-helpers/write_proc_macros.md

- [ ] P3 write_proc_macros: write; compare with macros.md - what should be in here? ([#331](https://github.com/john-cd/rust_howto/issues/331)) ([later/src/categories/development-tools_procedural-macro-helpers/write_proc_macros.md](./later/src/categories/development-tools_procedural-macro-helpers/write_proc_macros.md#L105))

### later/src/categories/embedded/embassy.md

- [ ] P3 embassy: add embassy, others ([#345](https://github.com/john-cd/rust_howto/issues/345)) ([later/src/categories/embedded/embassy.md](./later/src/categories/embedded/embassy.md#L29))

### later/src/categories/embedded/flash.md

- [ ] P3 write ([#1200](https://github.com/john-cd/rust_howto/issues/1200)) ([later/src/categories/embedded/flash.md](./later/src/categories/embedded/flash.md#L18))

### later/src/categories/embedded/hals.md

- [ ] P3 write ([#1203](https://github.com/john-cd/rust_howto/issues/1203)) ([later/src/categories/embedded/hals.md](./later/src/categories/embedded/hals.md#L42))

### later/src/categories/embedded/index.md

- [ ] P3 write; cover ([#346](https://github.com/john-cd/rust_howto/issues/346)) ([later/src/categories/embedded/index.md](./later/src/categories/embedded/index.md#L75))

### later/src/categories/embedded/pacs.md

- [ ] P3 write ([#1198](https://github.com/john-cd/rust_howto/issues/1198)) ([later/src/categories/embedded/pacs.md](./later/src/categories/embedded/pacs.md#L24))

### later/src/categories/embedded/rtos.md

- [ ] P3 write ([#1202](https://github.com/john-cd/rust_howto/issues/1202)) ([later/src/categories/embedded/rtos.md](./later/src/categories/embedded/rtos.md#L20))

### later/src/categories/embedded/sensors.md

- [ ] P3 write ([#1199](https://github.com/john-cd/rust_howto/issues/1199)) ([later/src/categories/embedded/sensors.md](./later/src/categories/embedded/sensors.md#L16))

### later/src/categories/embedded/useful_crates_embedded.md

- [ ] P3 write ([#1201](https://github.com/john-cd/rust_howto/issues/1201)) ([later/src/categories/embedded/useful_crates_embedded.md](./later/src/categories/embedded/useful_crates_embedded.md#L42))

### later/src/categories/emulators/emulators.md

- [ ] P3 emulators: write ([#347](https://github.com/john-cd/rust_howto/issues/347)) ([later/src/categories/emulators/emulators.md](./later/src/categories/emulators/emulators.md#L32))

### later/src/categories/emulators/index.md

- [ ] P3 write ([#348](https://github.com/john-cd/rust_howto/issues/348)) ([later/src/categories/emulators/index.md](./later/src/categories/emulators/index.md#L66))

### later/src/categories/external-ffi-bindings/external_ffi_bindings.md

- [ ] P3 external_ffi_bindings: write ([#355](https://github.com/john-cd/rust_howto/issues/355)) ([later/src/categories/external-ffi-bindings/external_ffi_bindings.md](./later/src/categories/external-ffi-bindings/external_ffi_bindings.md#L20))

### later/src/categories/external-ffi-bindings/index.md

- [?] P3 review ([#356](https://github.com/john-cd/rust_howto/issues/356)) ([later/src/categories/external-ffi-bindings/index.md](./later/src/categories/external-ffi-bindings/index.md#L32))

### later/src/categories/finance/index.md

- [ ] P3 write ([#365](https://github.com/john-cd/rust_howto/issues/365)) ([later/src/categories/finance/index.md](./later/src/categories/finance/index.md#L85))

### later/src/categories/finance/quant.md

- [ ] P3 quant: write ([#364](https://github.com/john-cd/rust_howto/issues/364)) ([later/src/categories/finance/quant.md](./later/src/categories/finance/quant.md#L25))

### later/src/categories/game-development/game_development.md

- [ ] P3 game_development: identify crates; write ([#366](https://github.com/john-cd/rust_howto/issues/366)) ([later/src/categories/game-development/game_development.md](./later/src/categories/game-development/game_development.md#L34))

### later/src/categories/game-development/index.md

- [ ] P3 expand ([#367](https://github.com/john-cd/rust_howto/issues/367)) ([later/src/categories/game-development/index.md](./later/src/categories/game-development/index.md#L67))

### later/src/categories/game-engines/game_engines.md

- [ ] P3 game_engines: write ([#369](https://github.com/john-cd/rust_howto/issues/369)) ([later/src/categories/game-engines/game_engines.md](./later/src/categories/game-engines/game_engines.md#L60))

### later/src/categories/game-engines/index.md

- [ ] P3 expand ([#370](https://github.com/john-cd/rust_howto/issues/370)) ([later/src/categories/game-engines/index.md](./later/src/categories/game-engines/index.md#L41))

### later/src/categories/games/index.md

- [ ] P3 [write; need in depth review; review [are-we-game-yet? games][are-we-game-yet?-games~website] ]() ([#373](https://github.com/john-cd/rust_howto/issues/373)) ([later/src/categories/games/index.md](./later/src/categories/games/index.md#L34))

### later/src/categories/gui/clipboard.md

- [ ] P3 write; expand ? ([#638](https://github.com/john-cd/rust_howto/issues/638)) ([later/src/categories/gui/clipboard.md](./later/src/categories/gui/clipboard.md#L36))

### later/src/categories/gui/file_dialogs.md

- [ ] P3 file_dialogs: write ([#381](https://github.com/john-cd/rust_howto/issues/381)) ([later/src/categories/gui/file_dialogs.md](./later/src/categories/gui/file_dialogs.md#L38))

### later/src/categories/gui/gtk.md

- [ ] P3 gtk: write ([#383](https://github.com/john-cd/rust_howto/issues/383)) ([later/src/categories/gui/gtk.md](./later/src/categories/gui/gtk.md#L54))

### later/src/categories/gui/immediate_mode_gui.md

- [ ] P3 immediate_mode_gui: write ([#385](https://github.com/john-cd/rust_howto/issues/385)) ([later/src/categories/gui/immediate_mode_gui.md](./later/src/categories/gui/immediate_mode_gui.md#L58))

### later/src/categories/gui/index.md

- [ ] P3 organize ([#397](https://github.com/john-cd/rust_howto/issues/397)) ([later/src/categories/gui/index.md](./later/src/categories/gui/index.md#L109))

### later/src/categories/gui/retained_mode_gui.md

- [ ] P3 retained_mode_gui: organize/write ([#389](https://github.com/john-cd/rust_howto/issues/389)) ([later/src/categories/gui/retained_mode_gui.md](./later/src/categories/gui/retained_mode_gui.md#L100))

### later/src/categories/gui/text_layout.md

- [ ] P3 text_layout: write ([#391](https://github.com/john-cd/rust_howto/issues/391)) ([later/src/categories/gui/text_layout.md](./later/src/categories/gui/text_layout.md#L36))

### later/src/categories/gui/ui_layout.md

- [?] P3 review ([#934](https://github.com/john-cd/rust_howto/issues/934)) ([later/src/categories/gui/ui_layout.md](./later/src/categories/gui/ui_layout.md#L45))

### later/src/categories/gui/web_based_gui.md

- [ ] P3 web_based_gui: write ([#394](https://github.com/john-cd/rust_howto/issues/394)) ([later/src/categories/gui/web_based_gui.md](./later/src/categories/gui/web_based_gui.md#L48))

### later/src/categories/gui/window_creation.md

- [ ] P3 window_creation: write ([#396](https://github.com/john-cd/rust_howto/issues/396)) ([later/src/categories/gui/window_creation.md](./later/src/categories/gui/window_creation.md#L53))

### later/src/categories/internationalization/index.md

- [ ] P3 write ([#402](https://github.com/john-cd/rust_howto/issues/402)) ([later/src/categories/internationalization/index.md](./later/src/categories/internationalization/index.md#L80))

### later/src/categories/internationalization/internationalization.md

- [ ] P3 write ([#401](https://github.com/john-cd/rust_howto/issues/401)) ([later/src/categories/internationalization/internationalization.md](./later/src/categories/internationalization/internationalization.md#L26))

### later/src/categories/localization/index.md

- [ ] P3 write ([#405](https://github.com/john-cd/rust_howto/issues/405)) ([later/src/categories/localization/index.md](./later/src/categories/localization/index.md#L30))

### later/src/categories/localization/localization.md

- [ ] P3 localization: write ([#404](https://github.com/john-cd/rust_howto/issues/404)) ([later/src/categories/localization/localization.md](./later/src/categories/localization/localization.md#L22))

### later/src/categories/multimedia/index.md

- [ ] P3 write ([#940](https://github.com/john-cd/rust_howto/issues/940)) ([later/src/categories/multimedia/index.md](./later/src/categories/multimedia/index.md#L66))

### later/src/categories/multimedia/multimedia.md

- [ ] P3 multimedia: write ([#413](https://github.com/john-cd/rust_howto/issues/413)) ([later/src/categories/multimedia/multimedia.md](./later/src/categories/multimedia/multimedia.md#L22))

### later/src/categories/multimedia_audio/audio.md

- [ ] P3 audio: write ([#415](https://github.com/john-cd/rust_howto/issues/415)) ([later/src/categories/multimedia_audio/audio.md](./later/src/categories/multimedia_audio/audio.md#L21))

### later/src/categories/multimedia_audio/index.md

- [ ] P3 fix ([#941](https://github.com/john-cd/rust_howto/issues/941)) ([later/src/categories/multimedia_audio/index.md](./later/src/categories/multimedia_audio/index.md#L57))

### later/src/categories/multimedia_encoding/encoding.md

- [ ] P3 encoding: write ([#417](https://github.com/john-cd/rust_howto/issues/417)) ([later/src/categories/multimedia_encoding/encoding.md](./later/src/categories/multimedia_encoding/encoding.md#L22))

### later/src/categories/multimedia_encoding/index.md

- [ ] P3 write ([#418](https://github.com/john-cd/rust_howto/issues/418)) ([later/src/categories/multimedia_encoding/index.md](./later/src/categories/multimedia_encoding/index.md#L42))

### later/src/categories/multimedia_images/color_handling.md

- [ ] P3 write ([#1223](https://github.com/john-cd/rust_howto/issues/1223)) ([later/src/categories/multimedia_images/color_handling.md](./later/src/categories/multimedia_images/color_handling.md#L24))

### later/src/categories/multimedia_images/images.md

- [ ] P3 images: write ([#420](https://github.com/john-cd/rust_howto/issues/420)) ([later/src/categories/multimedia_images/images.md](./later/src/categories/multimedia_images/images.md#L61))

### later/src/categories/multimedia_images/index.md

- [ ] P3 write ([#942](https://github.com/john-cd/rust_howto/issues/942)) ([later/src/categories/multimedia_images/index.md](./later/src/categories/multimedia_images/index.md#L58))

### later/src/categories/multimedia_images/pixel_buffers.md

- [ ] P3 write ([#1222](https://github.com/john-cd/rust_howto/issues/1222)) ([later/src/categories/multimedia_images/pixel_buffers.md](./later/src/categories/multimedia_images/pixel_buffers.md#L24))

### later/src/categories/multimedia_video/index.md

- [ ] P3 write ([#943](https://github.com/john-cd/rust_howto/issues/943)) ([later/src/categories/multimedia_video/index.md](./later/src/categories/multimedia_video/index.md#L46))

### later/src/categories/multimedia_video/video.md

- [ ] P3 write ([#422](https://github.com/john-cd/rust_howto/issues/422)) ([later/src/categories/multimedia_video/video.md](./later/src/categories/multimedia_video/video.md#L20))

### later/src/categories/no-std/index.md

- [ ] P3 write ([#427](https://github.com/john-cd/rust_howto/issues/427)) ([later/src/categories/no-std/index.md](./later/src/categories/no-std/index.md#L50))

### later/src/categories/no-std/no_std.md

- [ ] P3 no_std: write ([#426](https://github.com/john-cd/rust_howto/issues/426)) ([later/src/categories/no-std/no_std.md](./later/src/categories/no-std/no_std.md#L22))

### later/src/categories/no-std_no-alloc/index.md

- [ ] P3 review; cover use of no_std attribute to remove alloc crate ([#945](https://github.com/john-cd/rust_howto/issues/945)) ([later/src/categories/no-std_no-alloc/index.md](./later/src/categories/no-std_no-alloc/index.md#L63))

### later/src/categories/no-std_no-alloc/no_alloc.md

- [ ] P3 no_alloc: write ([#428](https://github.com/john-cd/rust_howto/issues/428)) ([later/src/categories/no-std_no-alloc/no_alloc.md](./later/src/categories/no-std_no-alloc/no_alloc.md#L22))

### later/src/categories/os_freebsd-apis/freebsd.md

- [ ] P3 freebsd: write ([#433](https://github.com/john-cd/rust_howto/issues/433)) ([later/src/categories/os_freebsd-apis/freebsd.md](./later/src/categories/os_freebsd-apis/freebsd.md#L22))

### later/src/categories/os_freebsd-apis/index.md

- [?] P3 review ([#947](https://github.com/john-cd/rust_howto/issues/947)) ([later/src/categories/os_freebsd-apis/index.md](./later/src/categories/os_freebsd-apis/index.md#L35))

### later/src/categories/os_linux-apis/index.md

- [?] P3 review ([#948](https://github.com/john-cd/rust_howto/issues/948)) ([later/src/categories/os_linux-apis/index.md](./later/src/categories/os_linux-apis/index.md#L69))

### later/src/categories/os_linux-apis/linux.md

- [ ] P3 linux: write ([#434](https://github.com/john-cd/rust_howto/issues/434)) ([later/src/categories/os_linux-apis/linux.md](./later/src/categories/os_linux-apis/linux.md#L22))

### later/src/categories/os_macos-apis/index.md

- [?] P3 review ([#949](https://github.com/john-cd/rust_howto/issues/949)) ([later/src/categories/os_macos-apis/index.md](./later/src/categories/os_macos-apis/index.md#L66))

### later/src/categories/os_macos-apis/macos.md

- [ ] P3 macos: write ([#435](https://github.com/john-cd/rust_howto/issues/435)) ([later/src/categories/os_macos-apis/macos.md](./later/src/categories/os_macos-apis/macos.md#L22))

### later/src/categories/rendering/2d_raster_graphics.md

- [ ] P3 write. decide what to cover. ([#1215](https://github.com/john-cd/rust_howto/issues/1215)) ([later/src/categories/rendering/2d_raster_graphics.md](./later/src/categories/rendering/2d_raster_graphics.md#L35))

### later/src/categories/rendering/2d_renderers.md

- [ ] P3 2d_renderers: write; titles ([#377](https://github.com/john-cd/rust_howto/issues/377)) ([later/src/categories/rendering/2d_renderers.md](./later/src/categories/rendering/2d_renderers.md#L98))

### later/src/categories/rendering/2d_vector_graphics.md

- [ ] P3 write ([#1214](https://github.com/john-cd/rust_howto/issues/1214)) ([later/src/categories/rendering/2d_vector_graphics.md](./later/src/categories/rendering/2d_vector_graphics.md#L53))

### later/src/categories/rendering/3d_renderers.md

- [ ] P3 write. decide what to cover ([#1213](https://github.com/john-cd/rust_howto/issues/1213)) ([later/src/categories/rendering/3d_renderers.md](./later/src/categories/rendering/3d_renderers.md#L66))

### later/src/categories/rendering/index.md

- [ ] P3 write ([#953](https://github.com/john-cd/rust_howto/issues/953)) ([later/src/categories/rendering/index.md](./later/src/categories/rendering/index.md#L78))

### later/src/categories/rendering/svg_rendering.md

- [ ] P3 write. ([#1216](https://github.com/john-cd/rust_howto/issues/1216)) ([later/src/categories/rendering/svg_rendering.md](./later/src/categories/rendering/svg_rendering.md#L30))

### later/src/categories/rendering/text_rendering.md

- [ ] P3 write / decide what to cover ([#1212](https://github.com/john-cd/rust_howto/issues/1212)) ([later/src/categories/rendering/text_rendering.md](./later/src/categories/rendering/text_rendering.md#L48))

### later/src/categories/rendering_data-formats/data_formats.md

- [ ] P3 data_formats: locate crates, write ([#453](https://github.com/john-cd/rust_howto/issues/453)) ([later/src/categories/rendering_data-formats/data_formats.md](./later/src/categories/rendering_data-formats/data_formats.md#L22))

### later/src/categories/rendering_data-formats/index.md

- [?] P3 review ([#954](https://github.com/john-cd/rust_howto/issues/954)) ([later/src/categories/rendering_data-formats/index.md](./later/src/categories/rendering_data-formats/index.md#L48))

### later/src/categories/rendering_engine/index.md

- [ ] P3 write ([#955](https://github.com/john-cd/rust_howto/issues/955)) ([later/src/categories/rendering_engine/index.md](./later/src/categories/rendering_engine/index.md#L34))

### later/src/categories/rendering_engine/rendering_engines.md

- [ ] P3 rendering_engines: write ([#455](https://github.com/john-cd/rust_howto/issues/455)) ([later/src/categories/rendering_engine/rendering_engines.md](./later/src/categories/rendering_engine/rendering_engines.md#L22))

### later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md

- [ ] P3 webgpu: write ([#375](https://github.com/john-cd/rust_howto/issues/375)) ([later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md](./later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md#L61))

### later/src/categories/rendering_graphics-api/index.md

- [ ] P3 write ([#956](https://github.com/john-cd/rust_howto/issues/956)) ([later/src/categories/rendering_graphics-api/index.md](./later/src/categories/rendering_graphics-api/index.md#L62))

### later/src/categories/rendering_graphics-api/native_graphics_apis.md

- [ ] P3 graphics_apis: write ([#457](https://github.com/john-cd/rust_howto/issues/457)) ([later/src/categories/rendering_graphics-api/native_graphics_apis.md](./later/src/categories/rendering_graphics-api/native_graphics_apis.md#L47))

### later/src/categories/rendering_graphics-api/opengl.md

- [ ] P3 write ([#1229](https://github.com/john-cd/rust_howto/issues/1229)) ([later/src/categories/rendering_graphics-api/opengl.md](./later/src/categories/rendering_graphics-api/opengl.md#L40))

### later/src/categories/rendering_graphics-api/shaders.md

- [ ] P3 decide what to cover, write ([#1228](https://github.com/john-cd/rust_howto/issues/1228)) ([later/src/categories/rendering_graphics-api/shaders.md](./later/src/categories/rendering_graphics-api/shaders.md#L77))

### later/src/categories/rendering_graphics-api/vulkan.md

- [ ] P3 decide what to cover / write ([#1227](https://github.com/john-cd/rust_howto/issues/1227)) ([later/src/categories/rendering_graphics-api/vulkan.md](./later/src/categories/rendering_graphics-api/vulkan.md#L30))

### later/src/categories/science/classical_machine_learning.md

- [ ] P3 organize / write ([#473](https://github.com/john-cd/rust_howto/issues/473)) ([later/src/categories/science/classical_machine_learning.md](./later/src/categories/science/classical_machine_learning.md#L47))

### later/src/categories/science/deep_learning.md

- [ ] P3 write ([#1205](https://github.com/john-cd/rust_howto/issues/1205)) ([later/src/categories/science/deep_learning.md](./later/src/categories/science/deep_learning.md#L38))

### later/src/categories/science/index.md

- [ ] P3 write ([#472](https://github.com/john-cd/rust_howto/issues/472)) ([later/src/categories/science/index.md](./later/src/categories/science/index.md#L56))

### later/src/categories/science_geo/geo.md

- [ ] P3 geo: write ([#474](https://github.com/john-cd/rust_howto/issues/474)) ([later/src/categories/science_geo/geo.md](./later/src/categories/science_geo/geo.md#L26))

### later/src/categories/science_geo/index.md

- [ ] P3 write ([#958](https://github.com/john-cd/rust_howto/issues/958)) ([later/src/categories/science_geo/index.md](./later/src/categories/science_geo/index.md#L42))

### later/src/categories/science_neuroscience/index.md

- [ ] P3 write ([#959](https://github.com/john-cd/rust_howto/issues/959)) ([later/src/categories/science_neuroscience/index.md](./later/src/categories/science_neuroscience/index.md#L41))
- [ ] P3 also cover Biology! see lib.rs categorization ([#1197](https://github.com/john-cd/rust_howto/issues/1197)) ([later/src/categories/science_neuroscience/index.md](./later/src/categories/science_neuroscience/index.md#L42))

### later/src/categories/science_neuroscience/neuroscience.md

- [ ] P3 neuro: write ([#476](https://github.com/john-cd/rust_howto/issues/476)) ([later/src/categories/science_neuroscience/neuroscience.md](./later/src/categories/science_neuroscience/neuroscience.md#L22))

### later/src/categories/science_robotics/artificial_intelligence.md

- [ ] P3 write ([#1209](https://github.com/john-cd/rust_howto/issues/1209)) ([later/src/categories/science_robotics/artificial_intelligence.md](./later/src/categories/science_robotics/artificial_intelligence.md#L32))

### later/src/categories/science_robotics/control_systems.md

- [ ] P3 write ([#1208](https://github.com/john-cd/rust_howto/issues/1208)) ([later/src/categories/science_robotics/control_systems.md](./later/src/categories/science_robotics/control_systems.md#L65))

### later/src/categories/science_robotics/hardware_integration.md

- [ ] P3 write ([#1211](https://github.com/john-cd/rust_howto/issues/1211)) ([later/src/categories/science_robotics/hardware_integration.md](./later/src/categories/science_robotics/hardware_integration.md#L29))

### later/src/categories/science_robotics/index.md

- [ ] P3 organize; review in depth ([#480](https://github.com/john-cd/rust_howto/issues/480)) ([later/src/categories/science_robotics/index.md](./later/src/categories/science_robotics/index.md#L87))

### later/src/categories/science_robotics/perception_and_sensors.md

- [ ] P3 write ([#1207](https://github.com/john-cd/rust_howto/issues/1207)) ([later/src/categories/science_robotics/perception_and_sensors.md](./later/src/categories/science_robotics/perception_and_sensors.md#L31))

### later/src/categories/science_robotics/robot_operating_systems.md

- [ ] P3 robotics: organize ([#477](https://github.com/john-cd/rust_howto/issues/477)) ([later/src/categories/science_robotics/robot_operating_systems.md](./later/src/categories/science_robotics/robot_operating_systems.md#L82))

### later/src/categories/science_robotics/robotics_frameworks.md

- [ ] P3 useful_robotics_tools_and_libs: locate libs, organize, write ([#479](https://github.com/john-cd/rust_howto/issues/479)) ([later/src/categories/science_robotics/robotics_frameworks.md](./later/src/categories/science_robotics/robotics_frameworks.md#L67))

### later/src/categories/science_robotics/simulation_visualization.md

- [ ] P3 write ([#1210](https://github.com/john-cd/rust_howto/issues/1210)) ([later/src/categories/science_robotics/simulation_visualization.md](./later/src/categories/science_robotics/simulation_visualization.md#L43))

### later/src/categories/simulation/index.md

- [ ] P3 write ([#961](https://github.com/john-cd/rust_howto/issues/961)) ([later/src/categories/simulation/index.md](./later/src/categories/simulation/index.md#L45))

### later/src/categories/simulation/physics_engines.md

- [ ] P3 write / decide what to cover ([#1206](https://github.com/john-cd/rust_howto/issues/1206)) ([later/src/categories/simulation/physics_engines.md](./later/src/categories/simulation/physics_engines.md#L24))

### later/src/categories/simulation/simulation.md

- [ ] P3 simulation: write ([#481](https://github.com/john-cd/rust_howto/issues/481)) ([later/src/categories/simulation/simulation.md](./later/src/categories/simulation/simulation.md#L24))

### later/src/categories/value-formatting/index.md

- [?] P3 review ([#967](https://github.com/john-cd/rust_howto/issues/967)) ([later/src/categories/value-formatting/index.md](./later/src/categories/value-formatting/index.md#L37))

### later/src/categories/value-formatting/number_formatting.md

- [ ] P3 value-formatting: choose crates, write ([#490](https://github.com/john-cd/rust_howto/issues/490)) ([later/src/categories/value-formatting/number_formatting.md](./later/src/categories/value-formatting/number_formatting.md#L33))

### later/src/categories/virtualization/containerization.md

- [ ] P3 write ([#1225](https://github.com/john-cd/rust_howto/issues/1225)) ([later/src/categories/virtualization/containerization.md](./later/src/categories/virtualization/containerization.md#L46))

### later/src/categories/virtualization/containers.md

- [ ] P3 review/write ([#988](https://github.com/john-cd/rust_howto/issues/988)) ([later/src/categories/virtualization/containers.md](./later/src/categories/virtualization/containers.md#L42))

### later/src/categories/virtualization/index.md

- [ ] P3 write; need in-depth review ([#968](https://github.com/john-cd/rust_howto/issues/968)) ([later/src/categories/virtualization/index.md](./later/src/categories/virtualization/index.md#L90))

### later/src/categories/virtualization/using_containers.md

- [ ] P3 write / fix ([#1224](https://github.com/john-cd/rust_howto/issues/1224)) ([later/src/categories/virtualization/using_containers.md](./later/src/categories/virtualization/using_containers.md#L25))

### later/src/categories/virtualization/virtualization.md

- [ ] P3 virtualization: write link firecracker; move containers section here ([#492](https://github.com/john-cd/rust_howto/issues/492)) ([later/src/categories/virtualization/virtualization.md](./later/src/categories/virtualization/virtualization.md#L22))

### later/src/categories/visualization/index.md

- [?] P3 review ([#969](https://github.com/john-cd/rust_howto/issues/969)) ([later/src/categories/visualization/index.md](./later/src/categories/visualization/index.md#L49))

### later/src/categories/visualization/visualization.md

- [ ] P3 visualization: write ([#494](https://github.com/john-cd/rust_howto/issues/494)) ([later/src/categories/visualization/visualization.md](./later/src/categories/visualization/visualization.md#L37))

### later/src/categories/wasm/index.md

- [ ] P3 write ([#970](https://github.com/john-cd/rust_howto/issues/970)) ([later/src/categories/wasm/index.md](./later/src/categories/wasm/index.md#L54))

### later/src/categories/wasm/interfacing_with_javascript.md

- [ ] P3 write ([#1218](https://github.com/john-cd/rust_howto/issues/1218)) ([later/src/categories/wasm/interfacing_with_javascript.md](./later/src/categories/wasm/interfacing_with_javascript.md#L49))

### later/src/categories/wasm/leptos.md

- [ ] P3 write ([#1220](https://github.com/john-cd/rust_howto/issues/1220)) ([later/src/categories/wasm/leptos.md](./later/src/categories/wasm/leptos.md#L30))

### later/src/categories/wasm/wasm_basics.md

- [?] P3 final review ([#1219](https://github.com/john-cd/rust_howto/issues/1219)) ([later/src/categories/wasm/wasm_basics.md](./later/src/categories/wasm/wasm_basics.md#L67))

### later/src/categories/wasm/wasm_development.md

- [ ] P3 write ([#1217](https://github.com/john-cd/rust_howto/issues/1217)) ([later/src/categories/wasm/wasm_development.md](./later/src/categories/wasm/wasm_development.md#L89))

### later/src/categories/wasm/wasm_standalone_runtimes.md

- [ ] P3 others: organize/write need full review ([#496](https://github.com/john-cd/rust_howto/issues/496)) ([later/src/categories/wasm/wasm_standalone_runtimes.md](./later/src/categories/wasm/wasm_standalone_runtimes.md#L117))

### later/src/categories/wasm/yew.md

- [ ] P3 yew: write / organize ([#498](https://github.com/john-cd/rust_howto/issues/498)) ([later/src/categories/wasm/yew.md](./later/src/categories/wasm/yew.md#L38))

### later/src/categories/web-programming_websocket/index.md

- [?] P3 review ([#979](https://github.com/john-cd/rust_howto/issues/979)) ([later/src/categories/web-programming_websocket/index.md](./later/src/categories/web-programming_websocket/index.md#L47))

### later/src/categories/web-programming_websocket/websocket.md

- [ ] P3 websocket: write ([#520](https://github.com/john-cd/rust_howto/issues/520)) ([later/src/categories/web-programming_websocket/websocket.md](./later/src/categories/web-programming_websocket/websocket.md#L56))

### later/src/other/architecture/architectural_patterns.md

- [ ] P3 [write. create missing examples. cover [`inventory`][c~inventory~docs]↗{{hi:inventory}} crate for DI]()? ([#1231](https://github.com/john-cd/rust_howto/issues/1231)) ([later/src/other/architecture/architectural_patterns.md](./later/src/other/architecture/architectural_patterns.md#L49))

### later/src/other/architecture/common_architectures.md

- [ ] P3 write / detail every architecture ([#1230](https://github.com/john-cd/rust_howto/issues/1230)) ([later/src/other/architecture/common_architectures.md](./later/src/other/architecture/common_architectures.md#L135))

### later/src/other/architecture/index.md

- [ ] P3 write ([#573](https://github.com/john-cd/rust_howto/issues/573)) ([later/src/other/architecture/index.md](./later/src/other/architecture/index.md#L29))

### later/src/other/architecture/software_architecture_process.md

- [ ] P3 write ([#572](https://github.com/john-cd/rust_howto/issues/572)) ([later/src/other/architecture/software_architecture_process.md](./later/src/other/architecture/software_architecture_process.md#L103))

### later/src/other/cloud/_fragment.md

- [ ] P3 decide where that goes ([#578](https://github.com/john-cd/rust_howto/issues/578)) ([later/src/other/cloud/_fragment.md](./later/src/other/cloud/_fragment.md#L21))

### later/src/other/cloud/aws.md

- [ ] P3 aws: write ([#574](https://github.com/john-cd/rust_howto/issues/574)) ([later/src/other/cloud/aws.md](./later/src/other/cloud/aws.md#L60))

### later/src/other/cloud/index.md

- [ ] P3 edit ([#579](https://github.com/john-cd/rust_howto/issues/579)) ([later/src/other/cloud/index.md](./later/src/other/cloud/index.md#L39))

### later/src/other/cloud/rust_native_cloud_development.md

- [ ] P3 write ([#576](https://github.com/john-cd/rust_howto/issues/576)) ([later/src/other/cloud/rust_native_cloud_development.md](./later/src/other/cloud/rust_native_cloud_development.md#L41))

### later/src/other/cross-platform/crux.md

- [ ] P3 crux: add / edit; link to architecture pages ([#582](https://github.com/john-cd/rust_howto/issues/582)) ([later/src/other/cross-platform/crux.md](./later/src/other/cross-platform/crux.md#L32))

### later/src/other/cross-platform/index.md

- [ ] P3 add / write; link to other chapters; conditional compile ([#583](https://github.com/john-cd/rust_howto/issues/583)) ([later/src/other/cross-platform/index.md](./later/src/other/cross-platform/index.md#L79))

### later/src/other/data-processing/csv.md

- [ ] P3 csv: organize / edit ([#585](https://github.com/john-cd/rust_howto/issues/585)) ([later/src/other/data-processing/csv.md](./later/src/other/data-processing/csv.md#L27))

### later/src/other/data-processing/data_engineering.md

- [ ] P3 data_engineering: organize / edit ([#589](https://github.com/john-cd/rust_howto/issues/589)) ([later/src/other/data-processing/data_engineering.md](./later/src/other/data-processing/data_engineering.md#L68))

### later/src/other/data-processing/dataframes.md

- [ ] P3 dataframes: organize / edit ([#587](https://github.com/john-cd/rust_howto/issues/587)) ([later/src/other/data-processing/dataframes.md](./later/src/other/data-processing/dataframes.md#L28))

### later/src/other/data-processing/index.md

- [ ] P3 organize / edit ([#594](https://github.com/john-cd/rust_howto/issues/594)) ([later/src/other/data-processing/index.md](./later/src/other/data-processing/index.md#L27))

### later/src/other/devops/cd_ci.md

- [ ] P3 cd_ci: write; organize links ([#595](https://github.com/john-cd/rust_howto/issues/595)) ([later/src/other/devops/cd_ci.md](./later/src/other/devops/cd_ci.md#L47))

### later/src/other/devops/github_actions.md

- [ ] P3 github_actions - see blessed.rs ([#600](https://github.com/john-cd/rust_howto/issues/600)) ([later/src/other/devops/github_actions.md](./later/src/other/devops/github_actions.md#L138))

### later/src/other/devops/index.md

- [ ] P3 write ([#605](https://github.com/john-cd/rust_howto/issues/605)) ([later/src/other/devops/index.md](./later/src/other/devops/index.md#L63))

### later/src/other/devops/release_automation.md

- [ ] P3 release_automation: write ([#604](https://github.com/john-cd/rust_howto/issues/604)) ([later/src/other/devops/release_automation.md](./later/src/other/devops/release_automation.md#L94))

### later/src/other/devops/version_control.md

- [ ] P3 git_hooks: write ([#602](https://github.com/john-cd/rust_howto/issues/602)) ([later/src/other/devops/version_control.md](./later/src/other/devops/version_control.md#L96))

### later/src/other/gpu/gpu.md

- [ ] P3 gpu: write ([#607](https://github.com/john-cd/rust_howto/issues/607)) ([later/src/other/gpu/gpu.md](./later/src/other/gpu/gpu.md#L23))

### later/src/other/gpu/index.md

- [ ] P3 write ([#608](https://github.com/john-cd/rust_howto/issues/608)) ([later/src/other/gpu/index.md](./later/src/other/gpu/index.md#L29))

### later/src/other/scripting/index.md

- [?] P3 review ([#991](https://github.com/john-cd/rust_howto/issues/991)) ([later/src/other/scripting/index.md](./later/src/other/scripting/index.md#L86))

### later/src/other/scripting/rhai.md

- [ ] P3 rhai: write ([#610](https://github.com/john-cd/rust_howto/issues/610)) ([later/src/other/scripting/rhai.md](./later/src/other/scripting/rhai.md#L49))

### later/src/other/scripting/task_automation.md

- [ ] P3 write; where does this chapter belong ? ([#1232](https://github.com/john-cd/rust_howto/issues/1232)) ([later/src/other/scripting/task_automation.md](./later/src/other/scripting/task_automation.md#L37))

### later/src/other/written-in-rust/development_tools.md

- [ ] P3 organize ([#612](https://github.com/john-cd/rust_howto/issues/612)) ([later/src/other/written-in-rust/development_tools.md](./later/src/other/written-in-rust/development_tools.md#L57))

### later/src/other/written-in-rust/index.md

- [ ] P3 organize; decide if we need other pages or if we consolidate here; table? ([#993](https://github.com/john-cd/rust_howto/issues/993)) ([later/src/other/written-in-rust/index.md](./later/src/other/written-in-rust/index.md#L154))

### later/src/other/written-in-rust/other_tools.md

- [ ] P3 others: review/add ([#615](https://github.com/john-cd/rust_howto/issues/615)) ([later/src/other/written-in-rust/other_tools.md](./later/src/other/written-in-rust/other_tools.md#L47))

### later/src/other/written-in-rust/python_tools.md

- [ ] P3 python_tools: write ([#617](https://github.com/john-cd/rust_howto/issues/617)) ([later/src/other/written-in-rust/python_tools.md](./later/src/other/written-in-rust/python_tools.md#L62))
