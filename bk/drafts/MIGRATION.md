<!-- TODO Reviewed ??-??-2026. -->

# Draft Migration Review

Review snapshot for [issue #2284](https://github.com/john-cd/rust_howto/issues/2284), based on `develop` at `c2678fd7a` plus the regex promotion. The issue owns follow-up status; this is the requested source inventory, not a second live task tracker.

The three regex recipes and the `FromStr` custom-type recipe are now published. Other chapters below are **candidates for review**, not certified ready. The initial inventory covered 224 entries; 223 draft-summary entries remain across 38 ordered groups. Order favors deterministic local examples, then concurrency/tooling, then service/platform-dependent work. Within a category, move prerequisites and individual ready recipes first; move an overview only when its included tables reference published recipes. Existing human review markers are preserved and are not machine-checkable readiness gates.

For every candidate: inspect prose against behavior; run the owning example tests and documented commands (including failure paths and needed features); move unique text and its recipe tables/reference definitions; update both summaries and the published example index; render the book and check diagnostics, includes, and destinations. Examples already under `bk/crates` stay there. Move examples from `later` only when their dependencies/features are available in the published workspace. A complete draft overview is not evidence that every included recipe is ready.

The static review below checks chapter existence, direct and nested include paths and named Rust anchors, duplicate explicit heading IDs, and overlap with published source paths. It does not substitute for executing every candidate. Empty `TODO` searches are not readiness evidence. Chapters not listed in the draft summary are listed separately below.

## Completed First Batch

Regular Expressions: unique ASCII hashtags, US-style phone extraction, and date-pattern replacement. Published source: [regex.md](../src/categories/text-processing/regex.md); examples remain in `bk/crates/cats/text_processing/examples/regex`. The draft chapter retains email, log filtering, fancy-regex, the longer example, and engine-selection guidance.

## Completed Second Migration

String Parsing: the reviewed `FromStr` recipe and its example-index entry moved to the published book under [issue #2286](https://github.com/john-cd/rust_howto/issues/2286). The existing `bk/crates/cats/text_processing/examples/string_parsing/from_str.rs` remains in place; it already contains valid, malformed, and entry-point test cases. Tests and book rendering were not run for this documentation-only move.

## 1. text-processing

Finish the remaining regex recipes individually: the full regex test run fails in `email::tests::test_email_example_entry_point` (expected `Some("I❤email")`, got `None`); a `///` line was inserted inside the verbose regex literal and must be corrected. Email extraction must not claim full email validation; supply a log fixture for filtering; explain the longer example and bound fancy-regex inputs. Remove the duplicate Longer Regex row and repair its undefined reference. Then consider concatenation, manipulation, search, diffing, other strings, text analysis, and highlighting; run each target and inspect its output. Unicode is already published and absent from this draft summary.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Text Processing](categories/text-processing/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Regular Expressions — Remaining Recipes](categories/text-processing/regex.md) | Published path exists: compare/merge or retire redirect; do not overwrite; Only remaining recipes; duplicate/undefined Longer Regex table reference |
| [String Concatenation](categories/text-processing/string_concat.md) | No missing include/anchor found; category work and execution/render checks still required |
| [String Manipulation](categories/text-processing/string_manipulation.md) | No missing include/anchor found; category work and execution/render checks still required |
| [String Search](categories/text-processing/string_search.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Text Analysis](categories/text-processing/text_analysis.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Syntax Highlighting](categories/text-processing/syntax_highlighting.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Text Diffing](categories/text-processing/diffing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [OS and C Strings](categories/text-processing/other_strings.md) | No missing include/anchor found; category work and execution/render checks still required |

## 2. parser-implementations

Test deterministic parser fixtures, malformed inputs, and feature choices; reconcile any already published parser material before moving overview tables.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Parser Implementations](categories/parser-implementations/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [HTML](categories/parser-implementations/html.md) | No missing include/anchor found; category work and execution/render checks still required |
| [INI](categories/parser-implementations/ini.md) | No missing include/anchor found; category work and execution/render checks still required |
| [JSON](categories/parser-implementations/json.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Markdown](categories/parser-implementations/markdown.md) | No missing include/anchor found; category work and execution/render checks still required |
| [TOML](categories/parser-implementations/toml.md) | No missing include/anchor found; category work and execution/render checks still required |
| [XML](categories/parser-implementations/xml.md) | No missing include/anchor found; category work and execution/render checks still required |
| [YAML](categories/parser-implementations/yaml.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Programming Languages](categories/parser-implementations/programming_languages.md) | No missing include/anchor found; category work and execution/render checks still required |

## 3. encoding

Merge with the published encoding category. Validate binary round trips and document format compatibility; do not replace published character encoding, CSV, Excel, or serde chapters.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Encoding and Serialization](categories/encoding/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |
| [Structured Data](categories/encoding/complex_encoding.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Typecasting](categories/encoding/typecasts.md) | Duplicate heading IDs: `Casting` |
| [Binary Encoders](categories/encoding/_binary_encoders.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Binary Encoders, No External Schema](categories/encoding/no_external_schema.md) | No missing include/anchor found; category work and execution/render checks still required |

## 4. config

Test environment changes with isolation and configuration precedence using temporary fixtures.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Configuration](categories/config/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Environment Variables](categories/config/environment_variables.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Configuration Management](categories/config/configuration.md) | No missing include/anchor found; category work and execution/render checks still required |

## 5. memory-management

Validate ownership, unsafe boundaries, allocation claims, and interning lifetimes. Check allocator_selection as its own workspace and retain required feature variants.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Memory Management](categories/memory-management/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Lazy Initialization](categories/memory-management/lazy_initialization.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Memory Allocation](categories/memory-management/memory_allocation.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Arena Allocation](categories/memory-management/arena_allocation.md) | No missing include/anchor found; category work and execution/render checks still required |
| [String Interning](categories/memory-management/string_interning.md) | No missing include/anchor found; category work and execution/render checks still required |

## 6. concurrency

Start with spawn/join and scoped threads, then channels/shared state, Rayon, atomics, Loom, and actors. Exercise bounded completion, synchronization invariants, and file/image fixtures; compile-only checks do not establish race freedom.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Concurrency](categories/concurrency/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Explicit Threads](categories/concurrency/explicit_threads.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Threadpools](categories/concurrency/threadpool.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Multithreading with `crossbeam`](categories/concurrency/crossbeam.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Data Parallelism](categories/concurrency/data_parallelism.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Message Passing and Channels](categories/concurrency/message_passing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Shared-state Concurrency](categories/concurrency/shared_state.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Concurrent Data Structures](categories/concurrency/concurrent_data_structures.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Send and Sync](categories/concurrency/send_sync.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Actors](categories/concurrency/_actors.md) | No missing include/anchor found; category work and execution/render checks still required |

## 7. asynchronous

Establish futures/runtime basics before traits, channels, streams, blocking bridges, and structured cancellation. Test shutdown, task completion, and error propagation without timing-sensitive assertions.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Asynchronous](categories/asynchronous/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Async](categories/asynchronous/async.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Async and Traits](categories/asynchronous/async_traits.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Tokio Async Runtime](categories/asynchronous/tokio.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Async Channels](categories/asynchronous/async_channels.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Streams](categories/asynchronous/streams.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Futures crate](categories/asynchronous/futures.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Mixing Async and Blocking Code](categories/asynchronous/async_and_blocking.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Utilities](categories/asynchronous/async_utilities.md) | No missing include/anchor found; category work and execution/render checks still required |

## 8. command-line-interface

Promote argument parsing first; snapshot help and completions. ANSI, TUI, and user-interaction recipes require terminal/output inspection and bounded interactive demos.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Command-line Interface](categories/command-line-interface/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Argument Parsing](categories/command-line-interface/argument_parsing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [ANSI Terminal](categories/command-line-interface/ansi_terminal.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Terminal User Interfaces](categories/command-line-interface/tui.md) | No missing include/anchor found; category work and execution/render checks still required |
| [User Interaction](categories/command-line-interface/user_interaction.md) | No missing include/anchor found; category work and execution/render checks still required |

## 9. command-line-utilities

Check installation and real CLI invocations with small fixtures. Networking tools require a controlled local target; shell setup/completion needs a supported shell.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Command-line Utilities](categories/command-line-utilities/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Filesystem](categories/command-line-utilities/filesystem_cli.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Networking](categories/command-line-utilities/networking_cli.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Shells](categories/command-line-utilities/shells.md) | No missing include/anchor found; category work and execution/render checks still required |

## 10. development-tools_testing

Use tiny example projects to demonstrate actual failures as well as passing tests. Coverage, Miri, and fuzzing need their documented tools/toolchains and bounded runs.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Development Tools - Testing](categories/development-tools_testing/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Assertions](categories/development-tools_testing/assertions.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Testing](categories/development-tools_testing/testing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Snapshot Testing](categories/development-tools_testing/snapshot_testing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Property-based Testing](categories/development-tools_testing/property_based_testing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Mocking](categories/development-tools_testing/mocking.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Test Runners](categories/development-tools_testing/test_runners.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Code Coverage](categories/development-tools_testing/code_coverage.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Fuzzing](categories/development-tools_testing/fuzzing.md) | No missing include/anchor found; category work and execution/render checks still required |

## 11. development-tools_debugging

Verify logging/tracing output locally, then debugger sessions and telemetry exports. Distributed telemetry requires a collector fixture or a documented service prerequisite.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Development Tools - Debugging, Logging](categories/development-tools_debugging/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Debuggers](categories/development-tools_debugging/debugger.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Debugging](categories/development-tools_debugging/debugging.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Tracing](categories/development-tools_debugging/tracing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Alternatives](categories/development-tools_debugging/tracing_alternatives.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Log Messages](categories/development-tools_debugging/log.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Logging Configuration](categories/development-tools_debugging/config_log.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Metrics](categories/development-tools_debugging/metrics.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Distributed Telemetry](categories/development-tools_debugging/distributed_telemetry.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Diagnostic Functions](categories/development-tools_debugging/diagnostic_functions.md) | No missing include/anchor found; category work and execution/render checks still required |

## 12. development-tools_profiling

Reconcile the published profiling overview. Verify benchmark/profiler invocations and output; instrumentation, assembly, and memory profiling depend on host permissions and installed tools.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Development Tools - Performance, Profiling](categories/development-tools_profiling/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Benchmarking](categories/development-tools_profiling/benchmarking.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Instrumentation Profiling](categories/development-tools_profiling/instrumentation_profiling.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Low-level Profiling Tools](categories/development-tools_profiling/assembly.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Memory Usage Analysis](categories/development-tools_profiling/memory_usage_analysis.md) | No missing include/anchor found; category work and execution/render checks still required |

## 13. development-tools_build-utils

Check generated artifacts, build-script rerun behavior, and autocfg probes in a minimal crate; benchmark cache claims only with measured evidence.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Development Tools - Build-time Tooling](categories/development-tools_build-utils/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Build-time Tools](categories/development-tools_build-utils/build_time_tooling.md) | Duplicate heading IDs: `cargo-toml` |
| [`autocfg`](categories/development-tools_build-utils/autocfg.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Build Cache](categories/development-tools_build-utils/build_cache.md) | No missing include/anchor found; category work and execution/render checks still required |

## 14. development-tools_cargo-plugins

Install and exercise pinned commands on a disposable crate. Keep auditing limitations explicit and test the custom Cargo plugin end to end.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Development Tools - Cargo Plugins](categories/development-tools_cargo-plugins/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Writing Code](categories/development-tools_cargo-plugins/code_writing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Code Formatting, Linting](categories/development-tools_cargo-plugins/code_formatting_linting.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Dependency Management](categories/development-tools_cargo-plugins/dependency_management.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Building](categories/development-tools_cargo-plugins/building.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Cross-compiling](categories/development-tools_cargo-plugins/cross_compiling.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Watching for Changes](categories/development-tools_cargo-plugins/watching_for_changes.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Improving Performance](categories/development-tools_cargo-plugins/performance.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Auditing](categories/development-tools_cargo-plugins/auditing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Maintaining](categories/development-tools_cargo-plugins/maintaining.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Write a Cargo Plugin](categories/development-tools_cargo-plugins/write_cargo_plugin.md) | No missing include/anchor found; category work and execution/render checks still required |

## 15. development-tools

Merge with existing contributor/setup/profiling material. Validate each command with the current pinned toolchain, platform, and fixture; remove duplicated guidance before moving category overviews.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Development Tools](categories/development-tools/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Cargo](categories/development-tools/cargo/cargo.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Package Layout](categories/development-tools/cargo/package_layout.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Crate Registries](categories/development-tools/cargo/crate_registries.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Build Rust Code](categories/development-tools/compilation/building.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Rust and Binary Installation](categories/development-tools/installation/install.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Rustup](categories/development-tools/installation/rustup.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Code Formatting](categories/development-tools/formatting/formatting.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Documentation](categories/development-tools/documentation/documentation.md) | No missing include/anchor found; category work and execution/render checks still required |
| [`mdBook`](categories/development-tools/documentation/mdbook.md) | No missing include/anchor found; category work and execution/render checks still required |
| [`mdbook-utils`](categories/development-tools/documentation/mdbook_utils.md) | No missing include/anchor found; category work and execution/render checks still required |
| [`templ`](categories/development-tools/documentation/templ.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Badges](categories/development-tools/documentation/badges.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Versioning](categories/development-tools/versioning/versioning.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Repository Automation](categories/development-tools/automation/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [`xtask` Project Checks](categories/development-tools/automation/xtask.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Git History with `gix`](categories/development-tools/automation/git.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Other](categories/development-tools/other/other.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Cross-compilation](categories/development-tools/cross-compilation/cross_compilation.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Code Verification](categories/development-tools/other/code_verification.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Analyze and Generate Rust Code](categories/development-tools/other/rust_source.md) | No missing include/anchor found; category work and execution/render checks still required |
| [`miri`](categories/development-tools/other/miri.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Faster Linking](categories/development-tools/compilation/faster_linking.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Reduce Compilation Duration](categories/development-tools/compilation/reduce_compilation_duration.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Transpilers](categories/development-tools/transcompilation/transpilers.md) | No missing include/anchor found; category work and execution/render checks still required |

## 16. rust-patterns

Merge error-handling overlap with published Result/Option chapters. Verify examples and their ownership/error contracts; prioritize builder/composition before broad pattern catalogs.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Error Handling](categories/rust-patterns/error_handling/error_handling.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Error Customization](categories/rust-patterns/error_handling/error_customization.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Rust Patterns](categories/rust-patterns/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Creational Patterns](categories/rust-patterns/creational_patterns.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Builder Pattern](categories/rust-patterns/builder_pattern.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Structural Patterns](categories/rust-patterns/structural_patterns.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Behavioral Design Patterns](categories/rust-patterns/behavioral_patterns.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Functional Programming](categories/rust-patterns/functional_programming.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Rust-specific Patterns](categories/rust-patterns/rust_specific_patterns.md) | No missing include/anchor found; category work and execution/render checks still required |

## 17. standalone

Overflow: run debug and release behavior. Predicates: run the composite predicate fixture. Plugins: establish actual dynamic loading versus Any-based dispatch. Self-referential: audit pinning/unsafe invariants. Inheritance: repair the duplicate explicit anchor and explain composition/dispatch consistently before promotion.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Plugin Architecture](plugin.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Self-Referential Types](self_referential.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Integer Overflow](overflow.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Inheritance](inheritance.md) | Duplicate heading IDs: `inheritance-in-rust` |
| [Predicates](predicates.md) | No missing include/anchor found; category work and execution/render checks still required |

## 18. web-programming

Promote local URL/HTTP-type/HTML-sanitization fixtures first. Replace scraping and MIME response dependencies on live sites with local fixtures, and verify malicious HTML handling.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Web Programming](categories/web-programming/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [HTTP Types and Interfaces](categories/web-programming/http_types_and_interfaces.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Extracting Links](categories/web-programming/scraping.md) | No missing include/anchor found; category work and execution/render checks still required |
| [URLs](categories/web-programming/url.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Media Types](categories/web-programming/mime.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Sanitize HTML](categories/web-programming/html_sanitization.md) | No missing include/anchor found; category work and execution/render checks still required |

## 19. web-programming_http-client

Use a loopback server for requests, downloads, timeouts, rate limits, and retries. API recipes need mock pagination/error fixtures; gist creation/deletion requires explicit credentials and cleanup.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Web Programming - HTTP Clients](categories/web-programming_http-client/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [HTTP clients](categories/web-programming_http-client/http_clients.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Making Requests](categories/web-programming_http-client/requests.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Calling a Web API](categories/web-programming_http-client/apis.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Rate Limiting and Retries](categories/web-programming_http-client/rate_limiting_and_retries.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Downloads](categories/web-programming_http-client/download.md) | No missing include/anchor found; category work and execution/render checks still required |

## 20. web-programming_http-server

Test bounded loopback server/client flows. Verify middleware, CORS preflight, TLS fixtures, schema generation, GraphQL, and gRPC including generated code before promoting framework comparisons.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Web Programming - HTTP Servers](categories/web-programming_http-server/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Internationalized API Errors](categories/web-programming_http-server/i18n.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Axum](categories/web-programming_http-server/_axum.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Actix](categories/web-programming_http-server/_actix.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Other Web Frameworks](categories/web-programming_http-server/other_frameworks.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Static Website Generators](categories/web-programming_http-server/static_website_generators.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Middleware](categories/web-programming_http-server/middleware.md) | No missing include/anchor found; category work and execution/render checks still required |
| [CORS](categories/web-programming_http-server/cors.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Axum Add-ons](categories/web-programming_http-server/axum_extras.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Batteries-included Frameworks](categories/web-programming_http-server/_batteries-included_frameworks.md) | No missing include/anchor found; category work and execution/render checks still required |
| [GraphQL](categories/web-programming_http-server/_graphql.md) | No missing include/anchor found; category work and execution/render checks still required |
| [gRPC](categories/web-programming_http-server/_grpc.md) | No missing include/anchor found; category work and execution/render checks still required |
| [`hyper`](categories/web-programming_http-server/_hyper.md) | No missing include/anchor found; category work and execution/render checks still required |

## 21. network-programming

Use loopback fixtures for server/proxy shutdown, forwarding, errors, and timeouts; check for conflicts with already published network recipes.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Network Programming](categories/network-programming/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Server](categories/network-programming/server.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Reverse Proxy](categories/network-programming/reverse_proxy.md) | No missing include/anchor found; category work and execution/render checks still required |

## 22. database

Start with embedded SQLite/key-value fixtures, then pools/ORMs. External databases, Oracle/MSSQL clients, and AMQP require provisioned services, credentials, and cleanup; run nested toasty_validation independently.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Database Access](categories/database/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [SQLite](categories/database/sqlite.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Postgres](categories/database/postgres.md) | No missing include/anchor found; category work and execution/render checks still required |
| [MSSQL](categories/database/mssql.md) | No missing include/anchor found; category work and execution/render checks still required |
| [MySQL](categories/database/mysql.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Oracle](categories/database/oracle.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Connection Pools](categories/database/connection_pool.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Query Builders and ORMs](categories/database/query_builders_orms.md) | No missing include/anchor found; category work and execution/render checks still required |
| [NoSQL](categories/database/nosql.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Key-Value Stores](categories/database/key_value_stores.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Search](categories/database/search.md) | No missing include/anchor found; category work and execution/render checks still required |
| [AMQP](categories/database/amqp.md) | No missing include/anchor found; category work and execution/render checks still required |

## 23. database-implementations

Reconcile the published database implementation material; validate embedded search locally and distinguish application recipes from implementation surveys.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Database Implementations](categories/database-implementations/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Databases](categories/database-implementations/databases.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Search](categories/database-implementations/rust_search_engines.md) | No missing include/anchor found; category work and execution/render checks still required |

## 24. cryptography

Review security claims and demonstrate authentication/tamper failures, nonce/key handling, password parameters, and certificate validation. Supply local TLS/certificate fixtures; passing a happy path alone is insufficient.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Cryptography](categories/cryptography/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Password Hashing](categories/cryptography/password_hashing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Encryption](categories/cryptography/encryption.md) | No missing include/anchor found; category work and execution/render checks still required |
| [AEAD](categories/cryptography/aead.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Protect Secrets](categories/cryptography/secrets.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Signatures](categories/cryptography/signature.md) | No missing include/anchor found; category work and execution/render checks still required |
| [HMAC](categories/cryptography/hmac.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Certificates](categories/cryptography/certificates.md) | No missing include/anchor found; category work and execution/render checks still required |
| [TLS](categories/cryptography/tls.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Cryptographic Utilities](categories/cryptography/cryptography_utilities.md) | No missing include/anchor found; category work and execution/render checks still required |

## 25. authentication

Basic auth needs HTTPS context. Session/CSRF flows need request-level tests; OAuth/OIDC need mock issuers or configured providers; passkeys require browser/authenticator ceremony validation.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Authentication](categories/authentication/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Basic Authentication](categories/authentication/basic_authentication.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Session Login](categories/authentication/session_login.md) | No missing include/anchor found; category work and execution/render checks still required |
| [OAuth 2.0 Client Credentials](categories/authentication/oauth_client_credentials.md) | No missing include/anchor found; category work and execution/render checks still required |
| [OpenID Connect Authorization Code with PKCE](categories/authentication/oidc_authorization_code.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Passkeys (WebAuthn)](categories/authentication/passkeys.md) | No missing include/anchor found; category work and execution/render checks still required |

## 26. email

Parsing can use checked-in messages; sending and reading require local SMTP/IMAP fixtures or explicitly configured accounts, timeouts, and cleanup.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Email](categories/email/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Sending emails](categories/email/send_emails.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Email parsing](categories/email/email_parsing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Reading emails](categories/email/read_emails.md) | No missing include/anchor found; category work and execution/render checks still required |

## 27. os

Run child-process/signal examples on their supported platforms with bounded subprocesses; avoid changing the parent shell or relying on machine-specific commands.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Operating Systems](categories/os/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [External Commands](categories/os/external_commands.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Process Management](categories/os/process_management.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Signals and Graceful Shutdown](categories/os/signals.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Low-level System Calls](categories/os/low_level_system_calls.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Rust OSes](categories/os/rust_os.md) | No missing include/anchor found; category work and execution/render checks still required |

## 28. os_unix-apis

Validate on supported Unix hosts with scoped process/filesystem resources; identify Linux-specific assumptions rather than claiming portability.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Operating systems - Unix](categories/os_unix-apis/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Unix](categories/os_unix-apis/unix.md) | No missing include/anchor found; category work and execution/render checks still required |

## 29. os_windows-apis

Requires a Windows host and relevant APIs/permissions; Linux compilation or prose inspection does not validate Windows runtime behavior.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Operating systems - Windows](categories/os_windows-apis/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Windows](categories/os_windows-apis/windows.md) | No missing include/anchor found; category work and execution/render checks still required |

## 30. hardware-support

Processor examples need architecture feature checks; peripherals need actual compatible devices and permissions. Keep hardware prose separate from validated runnable recipes.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Hardware Support](categories/hardware-support/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Processor](categories/hardware-support/processor.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Peripherals](categories/hardware-support/peripherals.md) | No missing include/anchor found; category work and execution/render checks still required |

## 31. text-editors

Check current editor configuration and execute the LSP examples against a local client/server. Reconcile contributor IDE setup material.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Text Editors](categories/text-editors/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [IDEs](categories/text-editors/ides.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Language Server Protocol](categories/text-editors/language_server_protocol.md) | No missing include/anchor found; category work and execution/render checks still required |

## 32. graphics

Compare the graphics overview with the published graphics recipes; merge unique guidance only after checking links and identifying platform/tool requirements.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Graphics](categories/graphics/index.md) | No missing include/anchor found; category work and execution/render checks still required |

## 33. caching

Draft overview is a redirect to already published caching/in-memory cache; no chapter promotion remains. Verify destination and retire the duplicate summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Caching](categories/caching/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## 34. compression

Draft overview is a redirect to already published compression; no chapter promotion remains. Verify destination and retire the duplicate summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Compression](categories/compression/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## 35. date-and-time

This is a redirect to the published date/time chapter; verify its destination and retire the duplicate draft-summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Date and Time](categories/date-and-time/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## 36. data-structures

Compare with published collections and data-structures chapters; merge only unique guidance and repair links, rather than duplicating the category.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Data Structures](categories/data-structures/index.md) | Missing include `phf.incl.md`; Missing include `elsa.incl.md`; Published path exists: compare/merge or retire redirect; do not overwrite |

## 37. parsing

This is a redirect to the published parsing/expression parser material; verify its destinations and retire the duplicate draft-summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Parsing](categories/parsing/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## 38. template-engine

This is a redirect to published Tera/TinyTemplate recipes; verify its destinations and retire the duplicate draft-summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Template Engines](categories/template-engine/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## Published Upstream During PR Preparation

- [Atomics](../src/categories/concurrency/atomics.md): already published on current `develop`; removed from the migration queue.
- [Model-Checking with Loom](../src/categories/concurrency/loom.md): already published on current `develop`; removed from the migration queue.
- [Cancellation and Structured Concurrency](../src/categories/asynchronous/structured_concurrency.md): already published on current `develop`; removed from the migration queue.
- [OpenAPI and JSON Schema](../src/categories/web-programming_http-server/openapi.md): already published on current `develop`; removed from the migration queue.

## Outside the Draft Summary

- [Unicode](categories/text-processing/unicode.md): recipe already published as Count Unicode Text for Display; compare retained draft notes and retire duplication without republishing the recipe.
- [VS Code Configuration](contributing/vscode.md): reconcile with published contributor environment setup, check the current `rust-analyzer.linkedProjects` paths, and validate in VS Code before merging unique guidance. This is a chapter candidate omitted from the summary.
- [FRAGMENTS.md](FRAGMENTS.md) and [TOREVIEW.md](TOREVIEW.md): authoring/curation records, not recipe chapters. Preserve their dispositions and assess any unique source notes separately.

## First-Batch Validation

- All three documented `cargo run -p text_processing --example regex -- <recipe>` commands pass their assertions (`hashtags`, `phone`, `replace`).
- `just cargo test --manifest-path bk/crates/Cargo.toml --locked -p text_processing --example regex -- --skip email::tests::test_email_example_entry_point`: 20 pass, one existing failing draft test excluded. This includes case-sensitive deduplication, extraction boundaries, unchanged borrowed replacements, and invalid-calendar-pattern replacement tests.
- Clippy for `text_processing --example regex` with `-D warnings`, pinned Rustfmt checks, scoped dprint/Markdownlint, repository hooks, and the example-reference index check pass.
- The full regex target fails in the unchanged draft email recipe. Its verbose regex literal contains a `///` documentation line as pattern text; repair that literal and its validation claim before considering promotion.
- `just bb` stops at existing missing dependency references in `bk/src/refs/crate-refs.md`; the same check fails on unchanged `develop`. These include Iroh, jiff, unicode-width, and local plugin packages. Review generated references rather than publishing placeholder crates.io links for local packages.
- The underlying `just bash mdbook-build` renders HTML with the configured preprocessors. The promoted page and example-index links, included Rust snippets, local destinations, and the rendered word-index fragments pass inspection. The configured linkcheck backend also passes. The pinned indexing plugin reports its existing mdBook 0.5.1/0.5.4 version warning; optional Pandoc output is skipped because its renderer is unavailable.
- `just book drafts` renders the draft preview, but logs missing `phf.incl.md` and `elsa.incl.md` in the data-structures overview; a successful exit does not make that chapter ready.
