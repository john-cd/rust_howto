<!-- TODO Reviewed 10-25-2026 -->

# TO DO List

Every open task is tracked in a GitHub issue. This file lists what is left and, for each implementable item, a self-contained brief for Jules (which has no GitHub access). Status notes go in the issue; rows stay terse.

## Notes from John (unclear; clarify before work)

- [ ] Blank line before `ANCHOR_END`? (style question for the `{{#include}}` markers)
- [ ] `{{!crate }}` (syntax idea; no issue yet)

Review items for John (`tools/`, `.github/actions`, the `.py` scripts, `LICENSE`, `bookmarks.json`) live in epic #2113. `cargo husky` is obsolete: `prek` (`prek.toml`) runs the hooks.

WAIT move code examples from `later/` to `bk/` when code and text are ready.

---

## Open issues

- [#2113](https://github.com/john-cd/rust_howto/issues/2113) — **Epic** — Human decisions, reviews and manual steps (CI billing, license, analytics, editorial reviews). Not for Jules.
- [#2114](https://github.com/john-cd/rust_howto/issues/2114) — **Epic** — Manual and live-service validation runs (Oracle, Meilisearch, S3, NATS, desktop GUI). Needs services or a display.
- [#2115](https://github.com/john-cd/rust_howto/issues/2115) — **Epic** — Work that needs macOS, FreeBSD, Windows or hardware.
- [#2116](https://github.com/john-cd/rust_howto/issues/2116) — **Epic** — Waiting on an upstream release or advisory fix (zip, zenoh, urdf-rs, cpal, egui).
- [#2117](https://github.com/john-cd/rust_howto/issues/2117) — **Epic** — Deferred recipes, example cleanup and book quality.
- [#2118](https://github.com/john-cd/rust_howto/issues/2118) — **Epic** — Tooling, CI and repository infrastructure.
- [#2145](https://github.com/john-cd/rust_howto/issues/2145) — Migrate the remaining crate badge lines to `{{#badges}}` (P4).
- [#2133](https://github.com/john-cd/rust_howto/issues/2133) — Close the last 59 example-test convention gaps (P3).

## Briefs for Jules

General rules for every brief: work from a clean checkout of `develop` on a topic branch; keep history linear; never delete or reword `TODO`, `TODO Reviewed` or human review comments; every example has a private `main()`, a `pub fn run()` (multi-file examples) and a test that calls `main()` with real assertions; no `unwrap()` outside unwrap demos; no `allow(dead_code)`; comments end with a period. Commit as `type(scope): summary` with the issue number. Format with `just fmtall` (nightly rustfmt is pinned in `rustfmt-toolchain.toml`). Run commands from the workspace directory (`bk/`, `later/`, `tools/`), not the repo root.

### Brief A — #2133 example-test convention gaps (P3)

- **Goal:** `python3 scripts/check_example_tests.py` (run from the repo root) reports no non-compliant files outside justified exceptions (today 59 of 1372).
- **Files:** the list printed by the script without `--summary`. Types: 51 `no tests` (mostly `main.rs` dispatchers such as `bk/crates/cats/filesystem/examples/tempfile/main.rs`), 4 `tests make no assertions` (`async_traits3.rs`, `jwt_authentication/main.rs`, `riker.rs`, `crate_indices/main.rs`), 2 `no test calls main()` (`oidc.rs`, `http_mock.rs`), 2 `missing pub fn run()` (`doc_comment.rs`, `cli_demo.rs`), plus `memory_management/examples/interner_allocations.rs` and `tools/example-integrity/src/main.rs`.
- **Steps:**
  1. For a dispatcher, add a `#[cfg(test)]` test that calls `main()` (adapt to its return type) and assert an observable effect where the modules expose one.
  2. If a module needs a service, network or display, mark its own test `#[ignore = "requires ..."]` (the reason must start with `requires` or `needs`) and keep the dispatcher test off that path.
  3. For files without `pub fn run()`, add it and call it from the directory's `main.rs`.
  4. For tests without assertions, assert on returned values, not just "does not panic".
  5. `interner_allocations.rs` uses a special harness (`test = false`): add a short rationale comment instead of forcing a test.
  6. One commit per crate.
- **Checks:** `python3 scripts/check_example_tests.py --summary`; in `bk/` and `tools/`: `just ca`, `just nta`; `python3 -m unittest discover scripts/tests`.
- **Done when:** the TOTAL line shows every file compliant, or each remaining file is listed with a reason in the issue.


### Brief E — improve example comments (#2117)

- **Goal:** each example has a short module doc and comments that explain why, ending with a period.
- **Steps:** one crate per commit, starting with `bk/crates/language/examples/`. Add or fix `//!` and `//` comments: purpose, non-obvious lines, expected output. Do not change behavior, `// ANCHOR` markers or rustdoc. Other threads edit nearby, so rebase on `develop` before each push.
- **Checks:** in `bk/`: `just ca`, `just nta`, `just spell`.

### Brief F — port remaining dependency upgrades (#2117)

- **Done** in PR #2139: `slint` 1.18, `symphonia` 0.6, `crux_core` 0.20, `spacepackets` 0.19. `egui`/`eframe` 0.37 was already current and no example uses `sprs`. `egui-notify`, `egui-phosphor` and `egui_tracing` wait on egui releases (notes in `later/crates/cats/gui/Cargo.toml`, tracked in #2116).

### Not for Jules

- #2113 needs John (accounts, decisions, editorial review). #2114 needs live services or a display. #2115 needs macOS, FreeBSD, Windows or hardware. #2116 waits for upstream releases: re-check each listed crate on crates.io and drop the matching `deny.toml` exception only when `cargo deny check advisories` stays clean.

---

## Review: `dead_code` allowances and ignored tests

Regenerated 2026-10-30 from develop (@ 400837c) by scanning `allow(...dead_code...)` and `expect(...dead_code...)` attributes (including multi-line and `cfg_attr` forms) and `#[ignore]` in `bk/crates`, `later/crates`, `xmpl`, `playground` and `tools`. Since 2026-10-29 the examples were cleaned up (#2079): most allowances became `#[expect(dead_code, reason = ...)]`, and the entries below are the ones that remain.

Kept as `allow` on purpose: `cornucopia/generated.rs` (generated by the `cornucopia` CLI; regenerate with the matching version, see #2116), `my_game_generated.rs` (FlatBuffers output) and `attributes/allow_dead_code.rs` (the example demonstrates the attribute). Every `expect` carries the reason it is needed (code that only runs against a live server, display or socket). Entries marked "exercised by the test" are candidates to drop the `expect` once a test uses the item directly. Ignored runs are tracked in #2114 and #2115.

### `dead_code` allowances and expectations (31)

#### `database`

- [bk/crates/cats/database/examples/postgres/cornucopia/generated.rs:6](bk/crates/cats/database/examples/postgres/cornucopia/generated.rs#L6) — `allow` (no reason)
- [bk/crates/cats/database/examples/postgres/cornucopia/generated.rs:11](bk/crates/cats/database/examples/postgres/cornucopia/generated.rs#L11) — `allow` (no reason)

#### `development_tools_debugging`

- [bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs:43](bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs#L43) — `expect`: alternative global-subscriber setup; only one subscriber can be installed per process
- [bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs:85](bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs#L85) — `expect`: alternative global-subscriber setup; only one subscriber can be installed per process

#### `development_tools_ffi`

- [later/crates/cats/development_tools_ffi/examples/node/napi.rs:5](later/crates/cats/development_tools_ffi/examples/node/napi.rs#L5) — `allow`: the exports are only called through the generated addon

#### `development_tools_profiling`

- [bk/crates/cats/development_tools_profiling/examples/puffin_server.rs:19](bk/crates/cats/development_tools_profiling/examples/puffin_server.rs#L19) — `expect`: held only to keep the server running

#### `encoding`

- [bk/crates/cats/encoding/examples/binary_encoders/my_game_generated.rs:6](bk/crates/cats/encoding/examples/binary_encoders/my_game_generated.rs#L6) — `allow` (no reason)

#### `finance`

- [later/crates/cats/finance/examples/market_data/yahoo_history.rs:18](later/crates/cats/finance/examples/market_data/yahoo_history.rs#L18) — `expect`: `main` needs the network; the tests parse a saved response

#### `gui`

- [later/crates/cats/gui/examples/immediate_mode_gui/egui_table.rs:65](later/crates/cats/gui/examples/immediate_mode_gui/egui_table.rs#L65) — `expect`: requires a GUI window and display

#### `language`

- [bk/crates/language/examples/constants_and_statics/constants.rs:85](bk/crates/language/examples/constants_and_statics/constants.rs#L85) — `expect`: the point is that an unused constant is never dropped
- [bk/crates/language/examples/attributes/allow_dead_code.rs:9](bk/crates/language/examples/attributes/allow_dead_code.rs#L9) — `allow` (no reason)

#### `multimedia_audio`

- [later/crates/cats/multimedia_audio/examples/audio/rodio_source.rs:75](later/crates/cats/multimedia_audio/examples/audio/rodio_source.rs#L75) — `expect`: this optional playback helper requires a device; the example stays device-free
- [later/crates/cats/multimedia_audio/examples/audio/rodio_file.rs:42](later/crates/cats/multimedia_audio/examples/audio/rodio_file.rs#L42) — `expect`: this optional playback helper requires a device; the example stays device-free

#### `other`

- [later/crates/other/examples/cloud/aws_message_bus.rs:3](later/crates/other/examples/cloud/aws_message_bus.rs#L3) — `expect`: needs an AWS account (SQS and SNS); the tests use mocks
- [later/crates/other/examples/cloud/aws_ses.rs:3](later/crates/other/examples/cloud/aws_ses.rs#L3) — `expect`: needs AWS credentials (SES); the tests use mocks
- [later/crates/other/examples/cloud/aws_s3_compatible.rs:3](later/crates/other/examples/cloud/aws_s3_compatible.rs#L3) — `expect`: needs an S3-compatible service; the tests use mocks
- [later/crates/other/examples/cloud/aws_cloudwatch.rs:3](later/crates/other/examples/cloud/aws_cloudwatch.rs#L3) — `expect`: needs AWS credentials (CloudWatch); the tests use mocks
- [later/crates/other/examples/cloud/kubernetes.rs:3](later/crates/other/examples/cloud/kubernetes.rs#L3) — `expect`: needs a Kubernetes cluster; the tests use mocks
- [later/crates/other/examples/cloud/aws_lambda_invoke.rs:3](later/crates/other/examples/cloud/aws_lambda_invoke.rs#L3) — `expect`: needs AWS credentials (Lambda); the tests use mocks

#### `science_geo`

- [later/crates/cats/science_geo/examples/geo/geocode.rs:61](later/crates/cats/science_geo/examples/geo/geocode.rs#L61) — `expect`: needs the network and is subject to the Nominatim usage policy

#### `wasm`

- [later/crates/cats/wasm/examples/leptos_client/grpc_web_client.rs:3](later/crates/cats/wasm/examples/leptos_client/grpc_web_client.rs#L3) — `expect`: targets the browser (wasm32); native builds cannot call it
- [later/crates/cats/wasm/examples/leptos_client/gloo_storage.rs:3](later/crates/cats/wasm/examples/leptos_client/gloo_storage.rs#L3) — `expect`: targets the browser (wasm32); native builds cannot call it
- [later/crates/cats/wasm/examples/leptos_client/gloo_fetch.rs:3](later/crates/cats/wasm/examples/leptos_client/gloo_fetch.rs#L3) — `expect`: targets the browser (wasm32); native builds cannot call it
- [later/crates/cats/wasm/examples/leptos_client/icons.rs:3](later/crates/cats/wasm/examples/leptos_client/icons.rs#L3) — `expect`: targets the browser (wasm32); native builds cannot call it
- [later/crates/cats/wasm/examples/interfacing_with_javascript/serde_wasm_bindgen.rs:1](later/crates/cats/wasm/examples/interfacing_with_javascript/serde_wasm_bindgen.rs#L1) — `expect`: targets the browser (wasm32); native builds cannot call it
- [later/crates/cats/wasm/examples/interfacing_with_javascript/promises.rs:1](later/crates/cats/wasm/examples/interfacing_with_javascript/promises.rs#L1) — `expect`: targets the browser (wasm32); native builds cannot call it

#### `web_programming_http_client`

- [bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs:68](bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs#L68) — `expect`: makes real network requests; `main` uses canned pages instead

#### `web_programming_http_server`

- [bk/crates/cats/web_programming_http_server/examples/grpc/tonic_client.rs:45](bk/crates/cats/web_programming_http_server/examples/grpc/tonic_client.rs#L45) — `expect`: needs a running `Greeter` server
- [bk/crates/cats/web_programming_http_server/examples/grpc/grpc_web.rs:39](bk/crates/cats/web_programming_http_server/examples/grpc/grpc_web.rs#L39) — `expect`: serves until its shutdown signal is received
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs:23](bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs#L23) — `expect`: binds a TLS socket; exercised by the test
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs:40](bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs#L40) — `expect`: stops the TLS server; exercised by the test

### Ignored tests (127)

#### `aerospace_drones`

- [later/crates/cats/aerospace_drones/examples/drones/drones1.rs:166](later/crates/cats/aerospace_drones/examples/drones/drones1.rs#L166) - requires real drone
- [later/crates/cats/aerospace_drones/examples/drones/main.rs:18](later/crates/cats/aerospace_drones/examples/drones/main.rs#L18) - requires real drone

#### `aerospace_unmanned_aerial_vehicles`

- [later/crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/uavs.rs:121](later/crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/uavs.rs#L121) - requires real drone
- [later/crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/main.rs:18](later/crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/main.rs#L18) - requires real drone

#### `clap_builder_xmpl`

- [xmpl/clap_builder_xmpl/src/main.rs:58](xmpl/clap_builder_xmpl/src/main.rs#L58) - main() parses the real process arguments, which belong to the test harness
- [xmpl/clap_builder_xmpl/src/cli/args.rs:159](xmpl/clap_builder_xmpl/src/cli/args.rs#L159) - needs a parent-controlled CLI environment

#### `command_line_interface`

- [bk/crates/cats/command_line_interface/examples/ratatui.rs:285](bk/crates/cats/command_line_interface/examples/ratatui.rs#L285) - requires an interactive terminal and key input
- [bk/crates/cats/command_line_interface/examples/crossterm.rs:98](bk/crates/cats/command_line_interface/examples/crossterm.rs#L98) - requires an interactive terminal and key input
- [bk/crates/cats/command_line_interface/examples/user_interaction/inquire.rs:51](bk/crates/cats/command_line_interface/examples/user_interaction/inquire.rs#L51) - Requires user input

#### `computer_vision`

- [later/crates/cats/computer_vision/examples/opencv/faces.rs:52](later/crates/cats/computer_vision/examples/opencv/faces.rs#L52) - needs OpenCV and a cascade XML path in OPENCV_HAAR_CASCADE

#### `config`

- [bk/crates/cats/config/examples/env/envy.rs:65](bk/crates/cats/config/examples/env/envy.rs#L65) - needs a parent-controlled environment

#### `cryptography`

- [bk/crates/cats/cryptography/examples/secrets/keyring.rs:74](bk/crates/cats/cryptography/examples/secrets/keyring.rs#L74) - requires an OS credential store
- [bk/crates/cats/cryptography/examples/secrets/keyring.rs:81](bk/crates/cats/cryptography/examples/secrets/keyring.rs#L81) - needs an OS credential store

#### `cryptography_cryptocurrencies`

- [later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs:166](later/crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs#L166) - requires ALLOY_RPC_URL for an Ethereum JSON-RPC endpoint

#### `data_structures`

- [bk/crates/cats/data_structures/examples/maps/multimap.rs:61](bk/crates/cats/data_structures/examples/maps/multimap.rs#L61) - requires external resources

#### `database`

- [bk/crates/cats/database/examples/amqp/lapin.rs:144](bk/crates/cats/database/examples/amqp/lapin.rs#L144) - needs RabbitMQ
- [bk/crates/cats/database/examples/mysql/mysql_async.rs:61](bk/crates/cats/database/examples/mysql/mysql_async.rs#L61) - needs MySQL or MariaDB
- [bk/crates/cats/database/examples/postgres/create_tables.rs:69](bk/crates/cats/database/examples/postgres/create_tables.rs#L69) - needs Postgres
- [bk/crates/cats/database/examples/postgres/insert_query_data.rs:110](bk/crates/cats/database/examples/postgres/insert_query_data.rs#L110) - needs Postgres
- [bk/crates/cats/database/examples/postgres/aggregate_data.rs:85](bk/crates/cats/database/examples/postgres/aggregate_data.rs#L85) - needs Postgres
- [bk/crates/cats/database/examples/postgres/cornucopia.rs:88](bk/crates/cats/database/examples/postgres/cornucopia.rs#L88) - needs Postgres
- [bk/crates/cats/database/examples/search/elasticsearch.rs:317](bk/crates/cats/database/examples/search/elasticsearch.rs#L317) - Requires a disposable Elasticsearch service configured through ELASTIC_URL
- [bk/crates/cats/database/examples/mssql/tiberius.rs:69](bk/crates/cats/database/examples/mssql/tiberius.rs#L69) - needs SQL Server
- [bk/crates/cats/database/examples/nosql/mongodb.rs:80](bk/crates/cats/database/examples/nosql/mongodb.rs#L80) - needs MongoDB reachable through MONGO_URI
- [bk/crates/cats/database/examples/nosql/redis.rs:94](bk/crates/cats/database/examples/nosql/redis.rs#L94) - needs a reachable Redis server
- [bk/crates/cats/database/examples/nosql/redis.rs:102](bk/crates/cats/database/examples/nosql/redis.rs#L102) - needs a running Redis server
- [bk/crates/cats/database/examples/connection_pool/deadpool2.rs:135](bk/crates/cats/database/examples/connection_pool/deadpool2.rs#L135) - requires a reachable PostgreSQL service and PG__* environment variables
- [bk/crates/cats/database/examples/connection_pool/deadpool2.rs:194](bk/crates/cats/database/examples/connection_pool/deadpool2.rs#L194) - needs a running PostgreSQL server
- [bk/crates/cats/database/examples/oracle/diesel_oci.rs:110](bk/crates/cats/database/examples/oracle/diesel_oci.rs#L110) - requires external oracle db and oracle client library
- [bk/crates/cats/database/examples/oracle/sibyl.rs:67](bk/crates/cats/database/examples/oracle/sibyl.rs#L67) - requires Oracle Instant Client and a reachable Oracle service
- [bk/crates/cats/database/examples/oracle/oracle.rs:59](bk/crates/cats/database/examples/oracle/oracle.rs#L59) - requires Oracle Free, Instant Client, and ORACLE_* settings
- [bk/crates/cats/database/examples/nats/jetstream.rs:266](bk/crates/cats/database/examples/nats/jetstream.rs#L266) - needs a NATS server with JetStream enabled (NATS_URL)
- [bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs:206](bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs#L206) - needs Cassandra at 127.0.0.1:9042
- [bk/crates/cats/database/tests/postgres/main.rs:21](bk/crates/cats/database/tests/postgres/main.rs#L21) - needs Postgres

#### `database_implementations`

- [bk/crates/cats/database_implementations/examples/search_engines/meilisearch.rs:120](bk/crates/cats/database_implementations/examples/search_engines/meilisearch.rs#L120) - requires a running Meilisearch service

#### `development_tools`

- [bk/crates/cats/development_tools/examples/documentation/mdbook_utils_broken_links.rs:70](bk/crates/cats/development_tools/examples/documentation/mdbook_utils_broken_links.rs#L70) - needs `mdbook-utils` on the PATH (cargo install mdbook-utils)
- [bk/crates/cats/development_tools/examples/documentation/mdbook_utils_broken_links.rs:84](bk/crates/cats/development_tools/examples/documentation/mdbook_utils_broken_links.rs#L84) - needs `mdbook-utils` on the PATH (cargo install mdbook-utils)
- [bk/crates/cats/development_tools/examples/crate_registries/crates_io.rs:131](bk/crates/cats/development_tools/examples/crate_registries/crates_io.rs#L131) - requires crates.io API access

#### `development_tools_debugging`

- [bk/crates/cats/development_tools_debugging/examples/log/log_syslog.rs:38](bk/crates/cats/development_tools_debugging/examples/log/log_syslog.rs#L38) - requires a Unix syslog daemon
- [bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs:161](bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs#L161) - needs a parent-controlled filter environment

#### `development_tools_ffi`

- [later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs:188](later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs#L188) - needs maturin, Python, and an active Python virtual environment
- [later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs:58](later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs#L58) - needs Ruby
- [later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs:51](later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs#L51) - needs Ruby headers and libruby
- [later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs:62](later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs#L62) - needs Ruby headers and libruby

#### `development_tools_testing`

- [bk/crates/cats/development_tools_testing/examples/test_runners/tests1.rs:89](bk/crates/cats/development_tools_testing/examples/test_runners/tests1.rs#L89) - This test takes an hour to run. Only run it manually when needed
- [bk/crates/cats/development_tools_testing/examples/test_techniques/env_vars.rs:115](bk/crates/cats/development_tools_testing/examples/test_techniques/env_vars.rs#L115) - spawned by the parent test with a controlled environment

#### `email`

- [bk/crates/cats/email/examples/lettre.rs:133](bk/crates/cats/email/examples/lettre.rs#L133) - needs SMTP credentials and a reachable SMTP relay
- [bk/crates/cats/email/examples/imap/main.rs:25](bk/crates/cats/email/examples/imap/main.rs#L25) - needs IMAP_HOST, IMAP_USER, IMAP_PASSWORD and a reachable IMAP server

#### `filesystem`

- [bk/crates/cats/filesystem/examples/open.rs:38](bk/crates/cats/filesystem/examples/open.rs#L38) - Relies on default programs that may not be installed by default.

#### `finance`

- [later/crates/cats/finance/examples/market_data/yahoo_history.rs:99](later/crates/cats/finance/examples/market_data/yahoo_history.rs#L99) - needs network access
- [later/crates/cats/finance/examples/market_data/main.rs:19](later/crates/cats/finance/examples/market_data/main.rs#L19) - needs network access

#### `game_engines`

- [later/crates/cats/game_engines/examples/game_engines/ggez.rs:138](later/crates/cats/game_engines/examples/game_engines/ggez.rs#L138) - opens a graphics window
- [later/crates/cats/game_engines/examples/game_engines/macroquad.rs:85](later/crates/cats/game_engines/examples/game_engines/macroquad.rs#L85) - opens a graphics window

#### `games`

- [later/crates/cats/games/examples/games.rs:92](later/crates/cats/games/examples/games.rs#L92) - requires interactive example runtime

#### `gui`

- [later/crates/cats/gui/examples/2d_renderers/minifb.rs:124](later/crates/cats/gui/examples/2d_renderers/minifb.rs#L124) - requires interactive example runtime
- [later/crates/cats/gui/examples/2d_renderers/femtovg.rs:201](later/crates/cats/gui/examples/2d_renderers/femtovg.rs#L201) - requires an interactive desktop display and OpenGL context
- [later/crates/cats/gui/examples/2d_renderers/vello.rs:175](later/crates/cats/gui/examples/2d_renderers/vello.rs#L175) - requires a compatible GPU adapter
- [later/crates/cats/gui/examples/web/wry_window.rs:187](later/crates/cats/gui/examples/web/wry_window.rs#L187) - requires a display
- [later/crates/cats/gui/examples/clipboard/arboard.rs:53](later/crates/cats/gui/examples/clipboard/arboard.rs#L53) - requires a desktop clipboard and overwrites its current text
- [later/crates/cats/gui/examples/clipboard/arboard.rs:72](later/crates/cats/gui/examples/clipboard/arboard.rs#L72) - needs a desktop session with a system clipboard
- [later/crates/cats/gui/examples/clipboard/main.rs:18](later/crates/cats/gui/examples/clipboard/main.rs#L18) - needs a desktop session with a system clipboard
- [later/crates/cats/gui/examples/window_creation/tao.rs:76](later/crates/cats/gui/examples/window_creation/tao.rs#L76) - opens a window
- [later/crates/cats/gui/examples/window_creation/baseview.rs:95](later/crates/cats/gui/examples/window_creation/baseview.rs#L95) - requires interactive example runtime
- [later/crates/cats/gui/examples/window_creation/winit.rs:75](later/crates/cats/gui/examples/window_creation/winit.rs#L75) - opens a native window
- [later/crates/cats/gui/examples/window_creation/software_pixels.rs:135](later/crates/cats/gui/examples/window_creation/software_pixels.rs#L135) - opens a native window; close it to finish
- [later/crates/cats/gui/examples/file_dialogs/main.rs:18](later/crates/cats/gui/examples/file_dialogs/main.rs#L18) - needs a desktop session that can show a native file dialog
- [later/crates/cats/gui/examples/file_dialogs/rfd.rs:58](later/crates/cats/gui/examples/file_dialogs/rfd.rs#L58) - opens a native file dialog
- [later/crates/cats/gui/examples/gtk/relm4.rs:107](later/crates/cats/gui/examples/gtk/relm4.rs#L107) - requires an interactive GTK session
- [later/crates/cats/gui/examples/gtk/gtk4.rs:108](later/crates/cats/gui/examples/gtk/gtk4.rs#L108) - requires a GTK display, e.g. xvfb-run
- [later/crates/cats/gui/examples/immediate_mode_gui/egui.rs:134](later/crates/cats/gui/examples/immediate_mode_gui/egui.rs#L134) - requires an interactive display
- [later/crates/cats/gui/examples/immediate_mode_gui/main.rs:23](later/crates/cats/gui/examples/immediate_mode_gui/main.rs#L23) - needs a desktop session that can open an egui window
- [later/crates/cats/gui/examples/retained_mode_gui/iced.rs:74](later/crates/cats/gui/examples/retained_mode_gui/iced.rs#L74) - opens a window
- [later/crates/cats/gui/examples/retained_mode_gui/xilem_main.rs:22](later/crates/cats/gui/examples/retained_mode_gui/xilem_main.rs#L22) - requires an interactive desktop session
- [later/crates/cats/gui/examples/retained_mode_gui/slint.rs:73](later/crates/cats/gui/examples/retained_mode_gui/slint.rs#L73) - requires an interactive Slint session
- [later/crates/cats/gui/examples/retained_mode_gui/floem.rs:61](later/crates/cats/gui/examples/retained_mode_gui/floem.rs#L61) - opens a window
- [later/crates/cats/gui/examples/retained_mode_gui/xilem.rs:69](later/crates/cats/gui/examples/retained_mode_gui/xilem.rs#L69) - requires an interactive example runtime

#### `multimedia`

- [later/crates/cats/multimedia/examples/multimedia/play_sound.rs:77](later/crates/cats/multimedia/examples/multimedia/play_sound.rs#L77) - needs an audio output device
- [later/crates/cats/multimedia/examples/multimedia/play_sound.rs:86](later/crates/cats/multimedia/examples/multimedia/play_sound.rs#L86) - needs an audio output device
- [later/crates/cats/multimedia/examples/multimedia/main.rs:18](later/crates/cats/multimedia/examples/multimedia/main.rs#L18) - requires an audio output device

#### `multimedia_audio`

- [later/crates/cats/multimedia_audio/examples/audio/audio.rs:87](later/crates/cats/multimedia_audio/examples/audio/audio.rs#L87) - requires audio device
- [later/crates/cats/multimedia_audio/examples/audio/main.rs:33](later/crates/cats/multimedia_audio/examples/audio/main.rs#L33) - requires an audio output device

#### `network_programming`

- [bk/crates/cats/network_programming/examples/server/glommio.rs:140](bk/crates/cats/network_programming/examples/server/glommio.rs#L140) - serves on 127.0.0.1:8080 until the process is stopped
- [bk/crates/cats/network_programming/examples/server/listen_unused.rs:79](bk/crates/cats/network_programming/examples/server/listen_unused.rs#L79) - blocks until a client connects to the printed port
- [bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs:109](bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs#L109) - requires a local upstream on 127.0.0.1:8000 and a proxy listener
- [bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs:128](bk/crates/cats/network_programming/examples/reverse_proxy/ngrok.rs#L128) - needs NGROK_AUTHTOKEN and access to the ngrok service

#### `os`

- [bk/crates/cats/os/examples/signals/main.rs:21](bk/crates/cats/os/examples/signals/main.rs#L21) - the Ctrl-C handler can be registered only once per process, and `ctrlc_example::tests::run_completes` already does so
- [bk/crates/cats/os/examples/external_commands/continuous.rs:65](bk/crates/cats/os/examples/external_commands/continuous.rs#L65) - requires access to a Linux systemd journal

#### `os_windows_apis`

- [bk/crates/cats/os_windows_apis/examples/windows/winapi.rs:60](bk/crates/cats/os_windows_apis/examples/windows/winapi.rs#L60) - displays a Windows message box
- [bk/crates/cats/os_windows_apis/examples/windows/windows.rs:37](bk/crates/cats/os_windows_apis/examples/windows/windows.rs#L37) - displays a Windows message box

#### `other`

- [later/crates/other/examples/cloud/aws_s3_compatible.rs:223](later/crates/other/examples/cloud/aws_s3_compatible.rs#L223) - requires an S3-compatible server on localhost:9000
- [later/crates/other/examples/cloud/aws_lambda.rs:88](later/crates/other/examples/cloud/aws_lambda.rs#L88) - requires the AWS Lambda runtime environment (AWS_LAMBDA_RUNTIME_API)
- [later/crates/other/examples/cloud/shuttle.rs:48](later/crates/other/examples/cloud/shuttle.rs#L48) - requires Shuttle runtime
- [later/crates/other/examples/cloud/aws_sdk.rs:44](later/crates/other/examples/cloud/aws_sdk.rs#L44) - needs AWS credentials and network access
- [later/crates/other/examples/cloud/kubernetes.rs:257](later/crates/other/examples/cloud/kubernetes.rs#L257) - needs a cluster (kind or k3d)

#### `rendering_graphics_api`

- [later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/wgpu_adapter_info.rs:53](later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/wgpu_adapter_info.rs#L53) - needs a GPU adapter
- [later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs:209](later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs#L209) - requires a compatible GPU adapter
- [later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs:216](later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs#L216) - requires a compatible GPU adapter

#### `science_robotics`

- [later/crates/cats/science_robotics/examples/robot_operating_systems/ros2_pubsub.rs:78](later/crates/cats/science_robotics/examples/robot_operating_systems/ros2_pubsub.rs#L78) - needs a ROS 2 environment

#### `templ`

- [tools/templ/src/main.rs:217](tools/templ/src/main.rs#L217) - main() parses the real process arguments, which belong to the test harness

#### `text_editors`

- [bk/crates/cats/text_editors/examples/language_server_protocol/lsp_client.rs:209](bk/crates/cats/text_editors/examples/language_server_protocol/lsp_client.rs#L209) - needs rust-analyzer on the PATH

#### `virtualization`

- [later/crates/cats/virtualization/examples/using_containers/docker_client.rs:88](later/crates/cats/virtualization/examples/using_containers/docker_client.rs#L88) - needs a Docker daemon
- [later/crates/cats/virtualization/examples/using_containers/main.rs:19](later/crates/cats/virtualization/examples/using_containers/main.rs#L19) - needs a Docker daemon
- [later/crates/cats/virtualization/examples/using_containers/redis_test.rs:53](later/crates/cats/virtualization/examples/using_containers/redis_test.rs#L53) - needs a Docker daemon

#### `web_programming`

- [bk/crates/cats/web_programming/examples/scraping/extract_links.rs:76](bk/crates/cats/web_programming/examples/scraping/extract_links.rs#L76) - needs network
- [bk/crates/cats/web_programming/examples/scraping/unique.rs:83](bk/crates/cats/web_programming/examples/scraping/unique.rs#L83) - needs network
- [bk/crates/cats/web_programming/examples/scraping/broken.rs:172](bk/crates/cats/web_programming/examples/scraping/broken.rs#L172) - needs network
- [bk/crates/cats/web_programming/examples/mime/request.rs:98](bk/crates/cats/web_programming/examples/mime/request.rs#L98) - needs network

#### `web_programming_http_client`

- [bk/crates/cats/web_programming_http_client/examples/apis/rest_head.rs:44](bk/crates/cats/web_programming_http_client/examples/apis/rest_head.rs#L44) - needs network
- [bk/crates/cats/web_programming_http_client/examples/apis/rest_get.rs:99](bk/crates/cats/web_programming_http_client/examples/apis/rest_get.rs#L99) - needs network
- [bk/crates/cats/web_programming_http_client/examples/requests/get1.rs:38](bk/crates/cats/web_programming_http_client/examples/requests/get1.rs#L38) - needs network
- [bk/crates/cats/web_programming_http_client/examples/requests/header.rs:87](bk/crates/cats/web_programming_http_client/examples/requests/header.rs#L87) - needs network
- [bk/crates/cats/web_programming_http_client/examples/requests/get.rs:35](bk/crates/cats/web_programming_http_client/examples/requests/get.rs#L35) - needs network
- [bk/crates/cats/web_programming_http_client/examples/download/download.rs:74](bk/crates/cats/web_programming_http_client/examples/download/download.rs#L74) - needs network
- [bk/crates/cats/web_programming_http_client/examples/download/post_file.rs:261](bk/crates/cats/web_programming_http_client/examples/download/post_file.rs#L261) - needs network
- [bk/crates/cats/web_programming_http_client/examples/download/partial.rs:461](bk/crates/cats/web_programming_http_client/examples/download/partial.rs#L461) - needs network
- [bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs:98](bk/crates/cats/web_programming_http_client/examples/http_clients/hyper.rs#L98) - needs network
- [bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs:39](bk/crates/cats/web_programming_http_client/examples/http_clients/ureq.rs#L39) - needs network

#### `web_programming_http_server`

- [bk/crates/cats/web_programming_http_server/examples/actix_web.rs:59](bk/crates/cats/web_programming_http_server/examples/actix_web.rs#L59) - binds 127.0.0.1:8080 and serves until stopped
- [bk/crates/cats/web_programming_http_server/examples/hyper_server.rs:106](bk/crates/cats/web_programming_http_server/examples/hyper_server.rs#L106) - binds 127.0.0.1:3000 and serves until stopped
- [bk/crates/cats/web_programming_http_server/examples/async_graphql.rs:111](bk/crates/cats/web_programming_http_server/examples/async_graphql.rs#L111) - starts a long-running GraphQL server
- [bk/crates/cats/web_programming_http_server/examples/axum.rs:174](bk/crates/cats/web_programming_http_server/examples/axum.rs#L174) - binds 127.0.0.1:3000 and serves until stopped
- [bk/crates/cats/web_programming_http_server/examples/grpc/tonic_client.rs:95](bk/crates/cats/web_programming_http_server/examples/grpc/tonic_client.rs#L95) - needs the tonic server example listening on [::1]:50051
- [bk/crates/cats/web_programming_http_server/examples/grpc/main.rs:24](bk/crates/cats/web_programming_http_server/examples/grpc/main.rs#L24) - serves on [::1]:50051 until stopped
- [bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs:95](bk/crates/cats/web_programming_http_server/examples/grpc/tonic.rs#L95) - binds [::1]:50051 and serves until stopped
- [bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs:142](bk/crates/cats/web_programming_http_server/examples/middleware/tower_http.rs#L142) - binds 127.0.0.1:3000 and serves until stopped

#### `web_programming_websocket`

- [later/crates/cats/web_programming_websocket/examples/websocket/main.rs:23](later/crates/cats/web_programming_websocket/examples/websocket/main.rs#L23) - needs network access to the public echo service
- [later/crates/cats/web_programming_websocket/examples/websocket/tungstenite.rs:50](later/crates/cats/web_programming_websocket/examples/websocket/tungstenite.rs#L50) - needs network access to the public echo service
