<!-- TODO Reviewed 10-25-2026 -->

# TO DO List

- [ ] JOHN review tools/
- [ ] JOHN review .github\actions
- [ ] JOHN review / rewrite the .py scripts
- [x] JOHN Review .pre-commit-config.yaml vs prek.toml (prek is now the single source; #1245)
- [ ] JOHN review LICENSE
- [ ] JOHN bookmarks.json
- [ ] cargo husky?
- [x] prek or pre-commit? prek (#1245)

## Tasks

### Urgent

- [x] REVIEW meaningful, not generic, README.md - see for example bk\crates\cats\asynchronous\README.md
- [/] review all TODO comments in the codebase and fix
- [/] fix "function xyz is never used". most book examples should have a private main() function and a pub fn run() function + a test that call main(). run() should be called by the main() function of the example.
- [/] all book examples should have a test that calls the private main() function and verifies the expected behavior. also add tests to verify other functions / code items within the exanple. ensure coverage for edge cases and error handling.
- [ ] review dirty worktrees, create commits, integrate useful branches, discard stale work, and clean up topic refs
- [ ] integrate all useful topic branches into develop and discard the rest

### Scripts, Tools

- [x] add just command for cargo fmt-toml; update Dockerfile and install scripts
- [x] verify that all .sh files can run in Git Bash (#2071; follow-ups in #2072)

### Code Examples

- [/] improve comments within examples. add context and explanations where necessary. end with a period.
- [ ] WAIT move code examples from later/ to bk/ when code + text ready

---

## In progress

### Partial; needs review or work

- [#1719](https://github.com/john-cd/rust_howto/issues/1719) — **P1** — Repair and document the dev container setup and compiler caching
- [#1871](https://github.com/john-cd/rust_howto/issues/1871) — **P2** — Run Windows-blocked validation in a Linux environment D
- **P1** — Port remaining examples across major dependency upgrades (no issue number yet)

### Truly blocked (install)

- [#422](https://github.com/john-cd/rust_howto/issues/422) — **P4** — Video chapter and example F
- [#768](https://github.com/john-cd/rust_howto/issues/768) — **P4** — game_engines/fyrox.rs: finish
- [#786](https://github.com/john-cd/rust_howto/issues/786) — **P4** — 2d_renderers/skia_safe.rs: finish
- [#1034](https://github.com/john-cd/rust_howto/issues/1034) — **P4** — objc/objc2.rs: finish; fix
- [#1037](https://github.com/john-cd/rust_howto/issues/1037) — **P4** — examples/uniffi.rs: finish
- [#1983](https://github.com/john-cd/rust_howto/issues/1983) — **P3** — Finish Windows validation and reference-generator repairs

### Blocked on a live service


### Blocked on a desktop, hardware or manual run

- [#772](https://github.com/john-cd/rust_howto/issues/772) — **P4** — Finish the wgpu compute example G
- [#777](https://github.com/john-cd/rust_howto/issues/777) — **P4** — Finish the femtovg example G
- [#805](https://github.com/john-cd/rust_howto/issues/805) — **P4** — multimedia/multimedia1.rs: write
- [#61](https://github.com/john-cd/rust_howto/issues/61) — **P4** — drones/drones1.rs: write
- [#65](https://github.com/john-cd/rust_howto/issues/65) — **P4** — uavs/uavs.rs: write

### Blocked on an upstream or advisory-held dependency

- [#843](https://github.com/john-cd/rust_howto/issues/843) — **P4** — robotics_frameworks/openrr.rs: write
- [#845](https://github.com/john-cd/rust_howto/issues/845) — **P4** — Add the Zenoh robotics example I
- [#792](https://github.com/john-cd/rust_howto/issues/792) — **P4** — 2d_renderers/vger.rs: finish
- [#1056](https://github.com/john-cd/rust_howto/issues/1056) — **P4** — window_creation/baseview.rs: write; review
- [#775](https://github.com/john-cd/rust_howto/issues/775) — **P4** — Finish the Dioxus example G
- [#248](https://github.com/john-cd/rust_howto/issues/248) — **P1** — replace current badge markdown copy-pasted text by a preprocessor command {#crate ...} {#categories... }

### Blocked on an owner or human decision

- [#1992](https://github.com/john-cd/rust_howto/issues/1992) — **P1** — Triage and integrate unique work from retained example branches
- [#1247](https://github.com/john-cd/rust_howto/issues/1247) — **P3** — precommit/mod.just: move to Dockerfile
- [#1850](https://github.com/john-cd/rust_howto/issues/1850) — **P1** — Re-enable GitHub Actions CI and verify the refactored workflows (review)

### Done; awaiting full validation gates

- [#818](https://github.com/john-cd/rust_howto/issues/818) — **P4** — linux/statx_metadata.rs: temporary-file size and optional birth time through rustix

### Done; needs human review

- [#1959](https://github.com/john-cd/rust_howto/issues/1959) — **P3** — Verify the encoding chapters and Cap'n Proto regeneration B
- [#1979](https://github.com/john-cd/rust_howto/issues/1979) — **P3** — Validate concurrency, memory, hashing, and enumeration recipe pages
- [#1875](https://github.com/john-cd/rust_howto/issues/1875) — **Unprioritized** — Review the flate2 file-I/O example

### Needs a human decision or write-up

- [#1158](https://github.com/john-cd/rust_howto/issues/1158) — **P3** — `write_proc_macros/main.rs`: finish; review the following; decide what examples are needed
- [#639](https://github.com/john-cd/rust_howto/issues/639) — **P3** — os/rust_os.md: write; move to written-in-rust?
- [#366](https://github.com/john-cd/rust_howto/issues/366) — **P4** — game-development/game_development.md: game_development: identify crates; write
- [#1212](https://github.com/john-cd/rust_howto/issues/1212) — **P4** — rendering/text_rendering.md: write / decide what to cover
- [#1213](https://github.com/john-cd/rust_howto/issues/1213) — **P4** — rendering/3d_renderers.md: write. decide what to cover
- [#1215](https://github.com/john-cd/rust_howto/issues/1215) — **P4** — rendering/2d_raster_graphics.md: write. decide what to cover.
- [#1227](https://github.com/john-cd/rust_howto/issues/1227) — **P4** — rendering_graphics-api/vulkan.md: decide what to cover / write
- [#1228](https://github.com/john-cd/rust_howto/issues/1228) — **P4** — rendering_graphics-api/shaders.md: decide what to cover, write
- [#963](https://github.com/john-cd/rust_howto/issues/963) — **P4** — Review natural-language-processing coverage
- [#1206](https://github.com/john-cd/rust_howto/issues/1206) — **P4** — simulation/physics_engines.md: write / decide what to cover
- [#251](https://github.com/john-cd/rust_howto/issues/251) — **P4** — cover DSLs in Rust
- [#19](https://github.com/john-cd/rust_howto/issues/19) — **P3** — review Google Analytics setup
- [#16](https://github.com/john-cd/rust_howto/issues/16) — **P3** — review license
- [#17](https://github.com/john-cd/rust_howto/issues/17) — **P4** — send a pull request to little book of rust books
- [#18](https://github.com/john-cd/rust_howto/issues/18) — **P4** — Consider adding project to goodfirstissue.dev

### Deferred

- [#764](https://github.com/john-cd/rust_howto/issues/764) — **P4** — quant/rustquant.rs: finish; review
- [#1725](https://github.com/john-cd/rust_howto/issues/1725) — **P2** — Review the Pandoc and Typst export plan (review)

### Trackers and not started

- [#1996](https://github.com/john-cd/rust_howto/issues/1996) — **P2** — Coordinate validation gates and verification passes
- [#1997](https://github.com/john-cd/rust_howto/issues/1997) — **P3** — Coordinate tooling, CI, and repository maintenance
- [#1995](https://github.com/john-cd/rust_howto/issues/1995) — **P4** — Coordinate unfinished-example completion
- [#1993](https://github.com/john-cd/rust_howto/issues/1993) — **P4** — Coordinate draft-chapter completion
- [#1955](https://github.com/john-cd/rust_howto/issues/1955) — **P3** — Coordinate new recipes from the local-project review
- [#1698](https://github.com/john-cd/rust_howto/issues/1698) — **P1** — Complete the first edition and integrate the companion tools
- [#1994](https://github.com/john-cd/rust_howto/issues/1994) — **P4** — Plan later-edition chapters and example stubs
- [#1849](https://github.com/john-cd/rust_howto/issues/1849) — **Unprioritized** — Coordinate tasks requiring human decisions
- [#901](https://github.com/john-cd/rust_howto/issues/901) — **P3** — build_time_tooling/cc_defines1.rs: finish; deal with extern
- [#53](https://github.com/john-cd/rust_howto/issues/53) — **P4** — Evaluate whether a slim or Alpine CI image improves the supported builds I

## Review: `dead_code` allowances and ignored tests

Added by the repo-wide Clippy pass (declaration-only snippets) and the blocked-issue isolation pass. For human review: each entry should either gain real use in the example, be removed, or keep a documented reason. Regenerate by searching for `dead_code` and `#[ignore`.

> Update 2026-10-29: file-level `#![allow(dead_code)]` and item-level `#[allow(dead_code)]` were converted to `#[expect(dead_code, reason = ...)]` (verified with clippy `--all-targets`), so any that stops being needed now warns. Not converted (build errors with the container toolchain, or generated/intentional): `cats/encoding`, `cats/data_structures`, `cats/database/.../cornucopia/generated.rs`, `attributes/allow_dead_code.rs`, `ownership_borrowing/borrowing_function.rs`, and `later` crates not built (multimedia_audio, development_tools_ffi napi, wasm interfacing). Giving the declaration-only snippets real uses remains optional.

### `dead_code` allowances (121)

#### `algorithms`

- [bk/crates/cats/algorithms/examples/rand/rand_custom.rs:13](bk/crates/cats/algorithms/examples/rand/rand_custom.rs#L13)

#### `asynchronous`

- [bk/crates/cats/asynchronous/examples/async_traits/async_traits2.rs:12](bk/crates/cats/asynchronous/examples/async_traits/async_traits2.rs#L12)
- [bk/crates/cats/asynchronous/examples/async_traits/rpitit_send.rs:4](bk/crates/cats/asynchronous/examples/async_traits/rpitit_send.rs#L4)
- [bk/crates/cats/asynchronous/examples/joinset_pool.rs:4](bk/crates/cats/asynchronous/examples/joinset_pool.rs#L4)
- [bk/crates/cats/asynchronous/examples/rpitit_send.rs:4](bk/crates/cats/asynchronous/examples/rpitit_send.rs#L4)

#### `authentication`

- [bk/crates/cats/authentication/examples/oauth_client_credentials/client_credentials.rs:4](bk/crates/cats/authentication/examples/oauth_client_credentials/client_credentials.rs#L4)
- [bk/crates/cats/authentication/examples/session_login/axum_login.rs:4](bk/crates/cats/authentication/examples/session_login/axum_login.rs#L4)
- [bk/crates/cats/authentication/examples/session_login/hashing.rs:4](bk/crates/cats/authentication/examples/session_login/hashing.rs#L4)

#### `code_organization`

- [bk/crates/code_organization/examples/modules/module_visibility.rs:4](bk/crates/code_organization/examples/modules/module_visibility.rs#L4)
- [bk/crates/code_organization/examples/modules/module_visibility.rs:49](bk/crates/code_organization/examples/modules/module_visibility.rs#L49)
- [bk/crates/code_organization/examples/visibility/private_access.rs:4](bk/crates/code_organization/examples/visibility/private_access.rs#L4)
- [bk/crates/code_organization/examples/visibility/pub_keyword.rs:4](bk/crates/code_organization/examples/visibility/pub_keyword.rs#L4)
- [bk/crates/code_organization/examples/visibility/public_access.rs:4](bk/crates/code_organization/examples/visibility/public_access.rs#L4)
- [bk/crates/code_organization/examples/visibility/public_by_default.rs:4](bk/crates/code_organization/examples/visibility/public_by_default.rs#L4)
- [bk/crates/code_organization/examples/visibility/visibility_external_code.rs:4](bk/crates/code_organization/examples/visibility/visibility_external_code.rs#L4)
- [bk/crates/code_organization/examples/visibility/visibility_scope.rs:25](bk/crates/code_organization/examples/visibility/visibility_scope.rs#L25)
- [bk/crates/code_organization/examples/visibility/visibility_scope.rs:36](bk/crates/code_organization/examples/visibility/visibility_scope.rs#L36)

#### `concurrency`

- [bk/crates/cats/concurrency/examples/message_passing/flume_commands.rs:4](bk/crates/cats/concurrency/examples/message_passing/flume_commands.rs#L4)
- [bk/crates/cats/concurrency/examples/message_passing/flume_tick.rs:4](bk/crates/cats/concurrency/examples/message_passing/flume_tick.rs#L4)
- [bk/crates/cats/concurrency/examples/send_sync/send_sync_impl.rs:4](bk/crates/cats/concurrency/examples/send_sync/send_sync_impl.rs#L4)

#### `config`

- [bk/crates/cats/config/examples/env/envy.rs:4](bk/crates/cats/config/examples/env/envy.rs#L4)

#### `cryptography`

- [bk/crates/cats/cryptography/examples/password_hashing/scrypt.rs:4](bk/crates/cats/cryptography/examples/password_hashing/scrypt.rs#L4)
- [bk/crates/cats/cryptography/examples/secrets/redacted_secret.rs:4](bk/crates/cats/cryptography/examples/secrets/redacted_secret.rs#L4)

#### `data_structures`

- [bk/crates/cats/data_structures/examples/strings/string_format.rs:92](bk/crates/cats/data_structures/examples/strings/string_format.rs#L92)

#### `database`

- [bk/crates/cats/database/examples/connection_pool/deadpool.rs:4](bk/crates/cats/database/examples/connection_pool/deadpool.rs#L4)
- [bk/crates/cats/database/examples/postgres/cornucopia/generated.rs:6](bk/crates/cats/database/examples/postgres/cornucopia/generated.rs#L6)
- [bk/crates/cats/database/examples/postgres/cornucopia/generated.rs:11](bk/crates/cats/database/examples/postgres/cornucopia/generated.rs#L11)
- [bk/crates/cats/database/examples/sqlite/insert_select.rs:16](bk/crates/cats/database/examples/sqlite/insert_select.rs#L16)
- [bk/crates/cats/database/examples/sqlite/transactions.rs:101](bk/crates/cats/database/examples/sqlite/transactions.rs#L101)

#### `development_tools_debugging`

- [bk/crates/cats/development_tools_debugging/examples/log/log_custom.rs:4](bk/crates/cats/development_tools_debugging/examples/log/log_custom.rs#L4)
- [bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs:4](bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs#L4)

#### `development_tools_ffi`

- [later/crates/cats/development_tools_ffi/examples/node/napi.rs:8](later/crates/cats/development_tools_ffi/examples/node/napi.rs#L8)

#### `development_tools_profiling`

- [bk/crates/cats/development_tools_profiling/examples/puffin_server.rs:4](bk/crates/cats/development_tools_profiling/examples/puffin_server.rs#L4)

#### `development_tools_testing`

- [bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs:4](bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs#L4)
- [bk/crates/cats/development_tools_testing/examples/test_techniques/env_vars.rs:4](bk/crates/cats/development_tools_testing/examples/test_techniques/env_vars.rs#L4)
- [bk/crates/cats/development_tools_testing/examples/test_techniques/paused_time.rs:4](bk/crates/cats/development_tools_testing/examples/test_techniques/paused_time.rs#L4)
- [bk/crates/cats/development_tools_testing/examples/test_techniques/proptest_strategy.rs:4](bk/crates/cats/development_tools_testing/examples/test_techniques/proptest_strategy.rs#L4)
- [bk/crates/cats/development_tools_testing/examples/test_techniques/rstest_cases.rs:4](bk/crates/cats/development_tools_testing/examples/test_techniques/rstest_cases.rs#L4)

#### `email`

- [bk/crates/cats/email/examples/imap/mailchecker.rs:4](bk/crates/cats/email/examples/imap/mailchecker.rs#L4)

#### `encoding`

- [bk/crates/cats/encoding/examples/binary_encoders/my_game_generated.rs:6](bk/crates/cats/encoding/examples/binary_encoders/my_game_generated.rs#L6)
- [bk/crates/cats/encoding/examples/serde/serde_ignored.rs:22](bk/crates/cats/encoding/examples/serde/serde_ignored.rs#L22)
- [bk/crates/cats/encoding/examples/typecasts/zerocopy.rs:192](bk/crates/cats/encoding/examples/typecasts/zerocopy.rs#L192)

#### `finance`

- [later/crates/cats/finance/examples/market_data/yahoo_history.rs:4](later/crates/cats/finance/examples/market_data/yahoo_history.rs#L4)
- [later/crates/cats/finance/examples/quant/rustquant.rs:10](later/crates/cats/finance/examples/quant/rustquant.rs#L10)
- [later/crates/cats/finance/examples/quant/rustquant.rs:34](later/crates/cats/finance/examples/quant/rustquant.rs#L34)

#### `gui`

- [later/crates/cats/gui/examples/immediate_mode_gui/egui_headless.rs:4](later/crates/cats/gui/examples/immediate_mode_gui/egui_headless.rs#L4)
- [later/crates/cats/gui/examples/immediate_mode_gui/egui_kittest.rs:4](later/crates/cats/gui/examples/immediate_mode_gui/egui_kittest.rs#L4)
- [later/crates/cats/gui/examples/immediate_mode_gui/egui_plot.rs:4](later/crates/cats/gui/examples/immediate_mode_gui/egui_plot.rs#L4)
- [later/crates/cats/gui/examples/immediate_mode_gui/egui_table.rs:4](later/crates/cats/gui/examples/immediate_mode_gui/egui_table.rs#L4)

#### `language`

- [bk/crates/language/examples/attributes/allow_dead_code.rs:4](bk/crates/language/examples/attributes/allow_dead_code.rs#L4)
- [bk/crates/language/examples/attributes/attributes_derive.rs:4](bk/crates/language/examples/attributes/attributes_derive.rs#L4)
- [bk/crates/language/examples/attributes/attributes_early_development.rs:4](bk/crates/language/examples/attributes/attributes_early_development.rs#L4)
- [bk/crates/language/examples/attributes/attributes_early_development.rs:29](bk/crates/language/examples/attributes/attributes_early_development.rs#L29)
- [bk/crates/language/examples/constants_and_statics/constants.rs:4](bk/crates/language/examples/constants_and_statics/constants.rs#L4)
- [bk/crates/language/examples/constants_and_statics/statics.rs:4](bk/crates/language/examples/constants_and_statics/statics.rs#L4)
- [bk/crates/language/examples/data_types/unit_never.rs:4](bk/crates/language/examples/data_types/unit_never.rs#L4)
- [bk/crates/language/examples/functions/diverging_functions.rs:14](bk/crates/language/examples/functions/diverging_functions.rs#L14)
- [bk/crates/language/examples/functions/diverging_functions.rs:22](bk/crates/language/examples/functions/diverging_functions.rs#L22)
- [bk/crates/language/examples/functions/generic_functions2.rs:4](bk/crates/language/examples/functions/generic_functions2.rs#L4)
- [bk/crates/language/examples/functions/generic_functions2.rs:55](bk/crates/language/examples/functions/generic_functions2.rs#L55)
- [bk/crates/language/examples/generics/generic_type_parameter.rs:4](bk/crates/language/examples/generics/generic_type_parameter.rs#L4)
- [bk/crates/language/examples/lifetimes/lifetime_parameters.rs:4](bk/crates/language/examples/lifetimes/lifetime_parameters.rs#L4)
- [bk/crates/language/examples/lifetimes/self_referential_struct.rs:6](bk/crates/language/examples/lifetimes/self_referential_struct.rs#L6)
- [bk/crates/language/examples/macros/macros.rs:69](bk/crates/language/examples/macros/macros.rs#L69)
- [bk/crates/language/examples/match/destructure_enum.rs:4](bk/crates/language/examples/match/destructure_enum.rs#L4)
- [bk/crates/language/examples/match/if_let.rs:4](bk/crates/language/examples/match/if_let.rs#L4)
- [bk/crates/language/examples/match/match1.rs:4](bk/crates/language/examples/match/match1.rs#L4)
- [bk/crates/language/examples/ownership_borrowing/borrowing_function.rs:1](bk/crates/language/examples/ownership_borrowing/borrowing_function.rs#L1)
- [bk/crates/language/examples/ownership_borrowing/copy.rs:4](bk/crates/language/examples/ownership_borrowing/copy.rs#L4)
- [bk/crates/language/examples/structs/generic_structs.rs:4](bk/crates/language/examples/structs/generic_structs.rs#L4)
- [bk/crates/language/examples/structs/struct_fields.rs:4](bk/crates/language/examples/structs/struct_fields.rs#L4)
- [bk/crates/language/examples/structs/struct_init.rs:4](bk/crates/language/examples/structs/struct_init.rs#L4)
- [bk/crates/language/examples/structs/struct_update.rs:4](bk/crates/language/examples/structs/struct_update.rs#L4)
- [bk/crates/language/examples/structs/structs.rs:4](bk/crates/language/examples/structs/structs.rs#L4)
- [bk/crates/language/examples/structs/tuple_structs.rs:4](bk/crates/language/examples/structs/tuple_structs.rs#L4)
- [bk/crates/language/examples/trait_objects/dyn_compat.rs:4](bk/crates/language/examples/trait_objects/dyn_compat.rs#L4)
- [bk/crates/language/examples/traits/trait_types.rs:4](bk/crates/language/examples/traits/trait_types.rs#L4)
- [bk/crates/language/examples/traits/traits.rs:4](bk/crates/language/examples/traits/traits.rs#L4)

#### `mathematics`

- [bk/crates/cats/mathematics/examples/additional_numeric_types/typenum.rs:4](bk/crates/cats/mathematics/examples/additional_numeric_types/typenum.rs#L4)

#### `multimedia_audio`

- [later/crates/cats/multimedia_audio/examples/audio/rodio_file.rs:43](later/crates/cats/multimedia_audio/examples/audio/rodio_file.rs#L43)
- [later/crates/cats/multimedia_audio/examples/audio/rodio_source.rs:76](later/crates/cats/multimedia_audio/examples/audio/rodio_source.rs#L76)

#### `os`

- [bk/crates/cats/os/examples/process_management/kill_guard.rs:4](bk/crates/cats/os/examples/process_management/kill_guard.rs#L4)
- [bk/crates/cats/os/examples/signals/tokio_signal.rs:4](bk/crates/cats/os/examples/signals/tokio_signal.rs#L4)

#### `other`

- [later/crates/other/examples/architecture/cqrs.rs:4](later/crates/other/examples/architecture/cqrs.rs#L4)
- [later/crates/other/examples/architecture/layered_architecture.rs:4](later/crates/other/examples/architecture/layered_architecture.rs#L4)
- [later/crates/other/examples/cloud/aws_cloudwatch.rs:4](later/crates/other/examples/cloud/aws_cloudwatch.rs#L4)
- [later/crates/other/examples/cloud/aws_lambda_invoke.rs:4](later/crates/other/examples/cloud/aws_lambda_invoke.rs#L4)
- [later/crates/other/examples/cloud/aws_message_bus.rs:4](later/crates/other/examples/cloud/aws_message_bus.rs#L4)
- [later/crates/other/examples/cloud/aws_s3_compatible.rs:4](later/crates/other/examples/cloud/aws_s3_compatible.rs#L4)
- [later/crates/other/examples/cloud/aws_ses.rs:4](later/crates/other/examples/cloud/aws_ses.rs#L4)
- [later/crates/other/examples/cloud/kubernetes.rs:4](later/crates/other/examples/cloud/kubernetes.rs#L4)
- [later/crates/other/examples/cloud/shuttle.rs:44](later/crates/other/examples/cloud/shuttle.rs#L44)

#### `parser_implementations`

- [bk/crates/cats/parser_implementations/examples/toml/toml.rs:4](bk/crates/cats/parser_implementations/examples/toml/toml.rs#L4)
- [bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs:4](bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs#L4)

#### `rust_patterns`

- [bk/crates/cats/rust_patterns/examples/builder_pattern/builder_pattern.rs:4](bk/crates/cats/rust_patterns/examples/builder_pattern/builder_pattern.rs#L4)
- [bk/crates/cats/rust_patterns/examples/error/thiserror.rs:4](bk/crates/cats/rust_patterns/examples/error/thiserror.rs#L4)
- [bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs:50](bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs#L50)

#### `science_geo`

- [later/crates/cats/science_geo/examples/geo/geocode.rs:4](later/crates/cats/science_geo/examples/geo/geocode.rs#L4)

#### `standard_library`

- [bk/crates/standard_library/examples/box/box_basics.rs:4](bk/crates/standard_library/examples/box/box_basics.rs#L4)
- [bk/crates/standard_library/examples/conversion_traits/from3.rs:4](bk/crates/standard_library/examples/conversion_traits/from3.rs#L4)
- [bk/crates/standard_library/examples/default/default.rs:4](bk/crates/standard_library/examples/default/default.rs#L4)
- [bk/crates/standard_library/examples/derive/derive_more.rs:4](bk/crates/standard_library/examples/derive/derive_more.rs#L4)
- [bk/crates/standard_library/examples/result/result.rs:77](bk/crates/standard_library/examples/result/result.rs#L77)
- [bk/crates/standard_library/examples/result/result.rs:118](bk/crates/standard_library/examples/result/result.rs#L118)

#### `wasm`

- [later/crates/cats/wasm/examples/interfacing_with_javascript/promises.rs:2](later/crates/cats/wasm/examples/interfacing_with_javascript/promises.rs#L2)
- [later/crates/cats/wasm/examples/interfacing_with_javascript/serde_wasm_bindgen.rs:2](later/crates/cats/wasm/examples/interfacing_with_javascript/serde_wasm_bindgen.rs#L2)
- [later/crates/cats/wasm/examples/leptos_client/gloo_fetch.rs:4](later/crates/cats/wasm/examples/leptos_client/gloo_fetch.rs#L4)
- [later/crates/cats/wasm/examples/leptos_client/gloo_storage.rs:4](later/crates/cats/wasm/examples/leptos_client/gloo_storage.rs#L4)
- [later/crates/cats/wasm/examples/leptos_client/grpc_web_client.rs:4](later/crates/cats/wasm/examples/leptos_client/grpc_web_client.rs#L4)

#### `web_programming_http_client`

- [bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs:4](bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs#L4)

#### `web_programming_http_server`

- [bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/cookies.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/cookies.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/debug_handler.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/debug_handler.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/flash_messages.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/flash_messages.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/https_redirect.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/https_redirect.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/typed_header.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/typed_header.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/grpc/grpc_web.rs:4](bk/crates/cats/web_programming_http_server/examples/grpc/grpc_web.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/grpc/tonic_client.rs:4](bk/crates/cats/web_programming_http_server/examples/grpc/tonic_client.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/openapi/schemars.rs:4](bk/crates/cats/web_programming_http_server/examples/openapi/schemars.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/openapi/schemars.rs:16](bk/crates/cats/web_programming_http_server/examples/openapi/schemars.rs#L16)
- [bk/crates/cats/web_programming_http_server/examples/openapi/utoipa.rs:4](bk/crates/cats/web_programming_http_server/examples/openapi/utoipa.rs#L4)

### Ignored tests (75)

#### `aerospace_drones`

- [later/crates/cats/aerospace_drones/examples/drones/drones1.rs:33](later/crates/cats/aerospace_drones/examples/drones/drones1.rs#L33) - requires real drone

#### `aerospace_unmanned_aerial_vehicles`

- [later/crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/uavs.rs:52](later/crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/uavs.rs#L52) - requires real drone

#### `command_line_interface`

- [bk/crates/cats/command_line_interface/examples/crossterm.rs:98](bk/crates/cats/command_line_interface/examples/crossterm.rs#L98) - requires an interactive terminal and key input
- [bk/crates/cats/command_line_interface/examples/user_interaction/inquire.rs:49](bk/crates/cats/command_line_interface/examples/user_interaction/inquire.rs#L49) - Requires user input

#### `cryptography`

- [bk/crates/cats/cryptography/examples/secrets/keyring.rs:80](bk/crates/cats/cryptography/examples/secrets/keyring.rs#L80) - needs an OS credential store
- [bk/crates/cats/cryptography/examples/tls/tokio_rustls.rs:64](bk/crates/cats/cryptography/examples/tls/tokio_rustls.rs#L64) - Requires network

#### `data_structures`

- [bk/crates/cats/data_structures/examples/maps/multimap.rs:61](bk/crates/cats/data_structures/examples/maps/multimap.rs#L61) - requires external resources

#### `database`

- [bk/crates/cats/database/examples/amqp/lapin.rs:144](bk/crates/cats/database/examples/amqp/lapin.rs#L144) - needs RabbitMQ
- [bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs:215](bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs#L215) - needs Cassandra at 127.0.0.1:9042
- [bk/crates/cats/database/examples/connection_pool/deadpool2.rs:135](bk/crates/cats/database/examples/connection_pool/deadpool2.rs#L135) - requires a reachable PostgreSQL service and PG__* environment variables
- [bk/crates/cats/database/examples/mssql/tiberius.rs:87](bk/crates/cats/database/examples/mssql/tiberius.rs#L87) - Requires an external Microsoft SQL Server instance
- [bk/crates/cats/database/examples/nosql/mongodb.rs:80](bk/crates/cats/database/examples/nosql/mongodb.rs#L80) - needs MongoDB reachable through MONGO_URI
- [bk/crates/cats/database/examples/nosql/redis.rs:94](bk/crates/cats/database/examples/nosql/redis.rs#L94) - needs a reachable Redis server
- [bk/crates/cats/database/examples/oracle/diesel_oci.rs:108](bk/crates/cats/database/examples/oracle/diesel_oci.rs#L108) - requires external oracle db and oracle client library
- [bk/crates/cats/database/examples/oracle/oracle.rs:59](bk/crates/cats/database/examples/oracle/oracle.rs#L59) - requires Oracle Free, Instant Client, and ORACLE_* settings
- [bk/crates/cats/database/examples/oracle/sibyl.rs:67](bk/crates/cats/database/examples/oracle/sibyl.rs#L67) - requires Oracle Instant Client and a reachable Oracle service
- [bk/crates/cats/database/examples/postgres/aggregate_data.rs:85](bk/crates/cats/database/examples/postgres/aggregate_data.rs#L85) - needs Postgres
- [bk/crates/cats/database/examples/postgres/cornucopia.rs:88](bk/crates/cats/database/examples/postgres/cornucopia.rs#L88) - needs Postgres
- [bk/crates/cats/database/examples/postgres/create_tables.rs:69](bk/crates/cats/database/examples/postgres/create_tables.rs#L69) - needs Postgres
- [bk/crates/cats/database/examples/postgres/insert_query_data.rs:110](bk/crates/cats/database/examples/postgres/insert_query_data.rs#L110) - needs Postgres
- [bk/crates/cats/database/examples/search/elasticsearch.rs:317](bk/crates/cats/database/examples/search/elasticsearch.rs#L317) - Requires a disposable Elasticsearch service configured through ELASTIC_URL
- [bk/crates/cats/database/tests/postgres/main.rs:21](bk/crates/cats/database/tests/postgres/main.rs#L21) - needs Postgres

#### `development_tools`

- [bk/crates/cats/development_tools/examples/crate_registries/crates_io.rs:131](bk/crates/cats/development_tools/examples/crate_registries/crates_io.rs#L131) - requires crates.io API access

#### `development_tools_debugging`

- [bk/crates/cats/development_tools_debugging/examples/log/log_syslog.rs:38](bk/crates/cats/development_tools_debugging/examples/log/log_syslog.rs#L38) - requires a Unix syslog daemon

#### `development_tools_ffi`

- [later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs:188](later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs#L188) - needs maturin, Python, and an active Python virtual environment
- [later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs:51](later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs#L51) - needs Ruby headers and libruby
- [later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs:58](later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs#L58) - needs Ruby

#### `development_tools_testing`

- [bk/crates/cats/development_tools_testing/examples/test_runners/tests1.rs:89](bk/crates/cats/development_tools_testing/examples/test_runners/tests1.rs#L89) - This test takes an hour to run. Only run it manually when needed

#### `email`

- [bk/crates/cats/email/examples/lettre.rs:104](bk/crates/cats/email/examples/lettre.rs#L104) - needs SMTP credentials and a reachable SMTP relay

#### `filesystem`

- [bk/crates/cats/filesystem/examples/open.rs:38](bk/crates/cats/filesystem/examples/open.rs#L38) - Relies on default programs that may not be installed by default.

#### `finance`

- [later/crates/cats/finance/examples/market_data/yahoo_history.rs:96](later/crates/cats/finance/examples/market_data/yahoo_history.rs#L96) - needs network access

#### `game_engines`

- [later/crates/cats/game_engines/examples/game_engines/ggez.rs:138](later/crates/cats/game_engines/examples/game_engines/ggez.rs#L138) - opens a graphics window
- [later/crates/cats/game_engines/examples/game_engines/macroquad.rs:85](later/crates/cats/game_engines/examples/game_engines/macroquad.rs#L85) - opens a graphics window

#### `games`

- [later/crates/cats/games/examples/games.rs:69](later/crates/cats/games/examples/games.rs#L69) - requires interactive example runtime

#### `gui`

- [later/crates/cats/gui/examples/2d_renderers/femtovg.rs:201](later/crates/cats/gui/examples/2d_renderers/femtovg.rs#L201) - requires an interactive desktop display and OpenGL context
- [later/crates/cats/gui/examples/2d_renderers/minifb.rs:79](later/crates/cats/gui/examples/2d_renderers/minifb.rs#L79) - requires interactive example runtime
- [later/crates/cats/gui/examples/2d_renderers/vello.rs:175](later/crates/cats/gui/examples/2d_renderers/vello.rs#L175) - requires a compatible GPU adapter
- [later/crates/cats/gui/examples/2d_renderers/webrender.rs:136](later/crates/cats/gui/examples/2d_renderers/webrender.rs#L136) - requires interactive example runtime
- [later/crates/cats/gui/examples/clipboard/arboard.rs:53](later/crates/cats/gui/examples/clipboard/arboard.rs#L53) - requires a desktop clipboard and overwrites its current text
- [later/crates/cats/gui/examples/clipboard/arboard.rs:72](later/crates/cats/gui/examples/clipboard/arboard.rs#L72) - needs a desktop session with a system clipboard
- [later/crates/cats/gui/examples/file_dialogs/rfd.rs:58](later/crates/cats/gui/examples/file_dialogs/rfd.rs#L58) - opens a native file dialog
- [later/crates/cats/gui/examples/gtk/gtk4.rs:104](later/crates/cats/gui/examples/gtk/gtk4.rs#L104) - requires an interactive GTK session
- [later/crates/cats/gui/examples/gtk/relm4.rs:99](later/crates/cats/gui/examples/gtk/relm4.rs#L99) - requires an interactive GTK session
- [later/crates/cats/gui/examples/immediate_mode_gui/egui.rs:134](later/crates/cats/gui/examples/immediate_mode_gui/egui.rs#L134) - requires an interactive display
- [later/crates/cats/gui/examples/retained_mode_gui/floem.rs:61](later/crates/cats/gui/examples/retained_mode_gui/floem.rs#L61) - opens a window
- [later/crates/cats/gui/examples/retained_mode_gui/iced.rs:74](later/crates/cats/gui/examples/retained_mode_gui/iced.rs#L74) - opens a window
- [later/crates/cats/gui/examples/retained_mode_gui/xilem.rs:60](later/crates/cats/gui/examples/retained_mode_gui/xilem.rs#L60) - requires an interactive example runtime
- [later/crates/cats/gui/examples/web/wry_window.rs:186](later/crates/cats/gui/examples/web/wry_window.rs#L186) - requires a display
- [later/crates/cats/gui/examples/window_creation/baseview.rs:76](later/crates/cats/gui/examples/window_creation/baseview.rs#L76) - requires interactive example runtime
- [later/crates/cats/gui/examples/window_creation/tao.rs:76](later/crates/cats/gui/examples/window_creation/tao.rs#L76) - opens a window
- [later/crates/cats/gui/examples/window_creation/winit.rs:75](later/crates/cats/gui/examples/window_creation/winit.rs#L75) - opens a native window

#### `multimedia`

- [later/crates/cats/multimedia/examples/multimedia/multimedia1.rs:42](later/crates/cats/multimedia/examples/multimedia/multimedia1.rs#L42) - requires audio output device

#### `multimedia_audio`

- [later/crates/cats/multimedia_audio/examples/audio/audio.rs:86](later/crates/cats/multimedia_audio/examples/audio/audio.rs#L86) - requires audio device

#### `network_programming`

- [bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs:109](bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs#L109) - requires a local upstream on 127.0.0.1:8000 and a proxy listener

#### `os`

- [bk/crates/cats/os/examples/external_commands/continuous.rs:63](bk/crates/cats/os/examples/external_commands/continuous.rs#L63) - requires access to a Linux systemd journal

#### `os_windows_apis`

- [bk/crates/cats/os_windows_apis/examples/windows/winapi.rs:60](bk/crates/cats/os_windows_apis/examples/windows/winapi.rs#L60) - displays a Windows message box
- [bk/crates/cats/os_windows_apis/examples/windows/windows.rs:35](bk/crates/cats/os_windows_apis/examples/windows/windows.rs#L35) - displays a Windows message box

#### `other`

- [later/crates/other/examples/cloud/aws_lambda.rs:88](later/crates/other/examples/cloud/aws_lambda.rs#L88) - requires the AWS Lambda runtime environment (AWS_LAMBDA_RUNTIME_API)
- [later/crates/other/examples/cloud/aws_s3_compatible.rs:86](later/crates/other/examples/cloud/aws_s3_compatible.rs#L86) - requires an S3-compatible server on localhost:9000
- [later/crates/other/examples/cloud/aws_sdk.rs:43](later/crates/other/examples/cloud/aws_sdk.rs#L43) - needs AWS credentials and network access
- [later/crates/other/examples/cloud/kubernetes.rs:234](later/crates/other/examples/cloud/kubernetes.rs#L234) - needs a cluster (kind or k3d)
- [later/crates/other/examples/cloud/shuttle.rs:55](later/crates/other/examples/cloud/shuttle.rs#L55) - requires Shuttle runtime

#### `rendering_graphics_api`

- [later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs:208](later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs#L208) - requires a compatible GPU adapter
- [later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/wgpu_adapter_info.rs:52](later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/wgpu_adapter_info.rs#L52) - needs a GPU adapter

#### `science_robotics`

- [later/crates/cats/science_robotics/examples/robot_operating_systems/ros2_pubsub.rs:78](later/crates/cats/science_robotics/examples/robot_operating_systems/ros2_pubsub.rs#L78) - needs a ROS 2 environment
- [later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs:171](later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs#L171) - requires interactive example runtime

#### `text_editors`

- [bk/crates/cats/text_editors/examples/language_server_protocol/lsp_client.rs:208](bk/crates/cats/text_editors/examples/language_server_protocol/lsp_client.rs#L208) - needs rust-analyzer on the PATH

#### `web_programming`

- [bk/crates/cats/web_programming/examples/mime/request.rs:98](bk/crates/cats/web_programming/examples/mime/request.rs#L98) - needs network
- [bk/crates/cats/web_programming/examples/scraping/broken.rs:172](bk/crates/cats/web_programming/examples/scraping/broken.rs#L172) - needs network
- [bk/crates/cats/web_programming/examples/scraping/extract_links.rs:76](bk/crates/cats/web_programming/examples/scraping/extract_links.rs#L76) - needs network
- [bk/crates/cats/web_programming/examples/scraping/unique.rs:83](bk/crates/cats/web_programming/examples/scraping/unique.rs#L83) - needs network

#### `web_programming_http_client`

- [bk/crates/cats/web_programming_http_client/examples/download/download.rs:74](bk/crates/cats/web_programming_http_client/examples/download/download.rs#L74) - needs network
- [bk/crates/cats/web_programming_http_client/examples/requests/get.rs:33](bk/crates/cats/web_programming_http_client/examples/requests/get.rs#L33) - needs network
- [bk/crates/cats/web_programming_http_client/examples/requests/get1.rs:36](bk/crates/cats/web_programming_http_client/examples/requests/get1.rs#L36) - needs network
- [bk/crates/cats/web_programming_http_client/examples/requests/header.rs:87](bk/crates/cats/web_programming_http_client/examples/requests/header.rs#L87) - needs network
