<!-- TODO Reviewed ??-??-2026. -->

# Draft Migration Review

Review snapshot initiated for [issue #2284](https://github.com/john-cd/rust_howto/issues/2284). Text Processing and Parser Implementations were tracked by [issues #2288](https://github.com/john-cd/rust_howto/issues/2288) and [#2289](https://github.com/john-cd/rust_howto/issues/2289). Encoding, Configuration, Memory Management, Concurrency, Asynchronous Programming, Command-line Interface, Command-line Utilities, Development Tools - Testing, Debugging, Profiling, Build Utils, Cargo Plugins, and the remaining Development Tools chapters are tracked by [issues #2290](https://github.com/john-cd/rust_howto/issues/2290), [#2291](https://github.com/john-cd/rust_howto/issues/2291), [#2292](https://github.com/john-cd/rust_howto/issues/2292), [#2293](https://github.com/john-cd/rust_howto/issues/2293), [#2294](https://github.com/john-cd/rust_howto/issues/2294), [#2295](https://github.com/john-cd/rust_howto/issues/2295), [#2296](https://github.com/john-cd/rust_howto/issues/2296), [#2297](https://github.com/john-cd/rust_howto/issues/2297), [#2298](https://github.com/john-cd/rust_howto/issues/2298), [#2299](https://github.com/john-cd/rust_howto/issues/2299), [#2300](https://github.com/john-cd/rust_howto/issues/2300), [#2301](https://github.com/john-cd/rust_howto/issues/2301), and [#2302](https://github.com/john-cd/rust_howto/issues/2302). This inventory retains the remaining category order and readiness work.

The initial regex recipes, the `FromStr` custom-type recipe, and the complete Text Processing, Parser Implementations, Encoding, Configuration, Memory Management, Concurrency, Asynchronous Programming, Command-line Interface, Command-line Utilities, Development Tools - Testing, Development Tools - Debugging, Development Tools - Profiling, Development Tools - Build Utils, Development Tools - Cargo Plugins, and Development Tools categories are published. Closeout tests and the HTML book render have passed; the detailed evidence appears below. Other chapters remain **candidates for review**, not certified ready. The initial inventory covered 224 entries; 100 draft-summary entries remain across 23 ordered groups. Order favors deterministic local examples, then concurrency/tooling, then service/platform-dependent work. Within a category, move prerequisites and individual ready recipes first; move an overview only when its included tables reference published recipes. Existing human review markers are preserved and are not machine-checkable readiness gates.

For every candidate: inspect prose against behavior and example coverage; resolve migration blockers; move unique text and its recipe tables/reference definitions; update both summaries, related links, and the published example index. At closeout, run the owning example tests and documented commands (including failure paths and needed features), then render the book and check diagnostics, includes, and destinations. Examples already under `bk/crates` stay there. Move examples from `later` only when their dependencies/features are available in the published workspace. A complete draft overview is not evidence that every included recipe is ready.

The static review below checks chapter existence, direct and nested include paths and named Rust anchors, duplicate explicit heading IDs, and overlap with published source paths. It does not substitute for executing every candidate. Empty `TODO` searches are not readiness evidence. Chapters not listed in the draft summary are listed separately below.

## Completed First Batch

Regular Expressions: unique ASCII hashtags, US-style phone extraction, and date-pattern replacement. Published source: [regex.md](../src/categories/text-processing/regex.md); examples remain in `bk/crates/cats/text_processing/examples/regex`.

## Completed Second Migration

String Parsing: the reviewed `FromStr` recipe and its example-index entry moved to the published book under [issue #2286](https://github.com/john-cd/rust_howto/issues/2286). The existing `bk/crates/cats/text_processing/examples/string_parsing/from_str.rs` remains in place and covers valid, malformed, and entry-point cases.

## Completed Third Migration

Text Processing: promoted the remaining regex recipes and all ready chapters for concatenation, manipulation, search, analysis, syntax highlighting, diffing, and OS/C/non-UTF-8 strings. The existing Unicode and `FromStr` chapters are now under the category overview. Tests exposed and fixed a malformed verbose regex comment; the email recipe now states its limited extraction behavior, and fancy-regex input limits are explicit. All examples remain in the published `text_processing` workspace.

## Completed Fourth Migration

Parser Implementations: moved the complete overview and the HTML/CSS, INI, JSON, Markdown, TOML, XML, YAML, and programming-language parser chapters. The category contains runnable examples for the parser comparisons; existing example sources remain in `bk/crates/cats/parser_implementations`. Corrected links to published parsing, text-processing, serialization, and macro pages. Removed the WAT/WASM row that had no runnable recipe. Example tests and the final render are recorded in Validation Closeout below.

## Completed Fifth Migration

Encoding: merged Structured Data, Typecasts and Byte Layout, schema-first Binary Encoders, and Binary Encoders Without External Schemas into the existing Encoding and Serialization section. Kept the existing character encoding, CSV, Excel, and Serde guides. Corrected duplicate typecast heading IDs, cast semantics, parser links, and staging references. Examples remain in `bk/crates/cats/encoding`; example tests and final rendering are recorded in Validation Closeout below.

## Completed Sixth Migration

Configuration: promoted the overview, layered configuration recipes, and environment-variable examples. Replaced the dotenv example's process-wide environment mutation and full environment dump with a temporary fixture that reads one setting; updated cross-links to the published chapter. Examples remain in `bk/crates/cats/config`; example tests and final rendering are recorded in Validation Closeout below.

## Completed Seventh Migration

Memory Management: promoted the overview and the lazy initialization, allocator, arena, and string interning guides. Removed the unmaintained `wee_alloc` stub, added the allocation-counting recipe to the index, and corrected the `seize` example to unpublish its pointer before retirement while describing the sample as an API sketch. Updated incoming links. Examples remain in `bk/crates/cats/memory_management`; `allocator_selection` stays in its isolated package. Package tests, all three allocator selections, and final rendering are recorded in Validation Closeout below.

## Completed Eighth Migration

Concurrency: merged the draft category with the existing Atomics and Loom chapters, moved the full thread, channel, shared-state, Rayon, data-structure, `Send`/`Sync`, and actor guides, and linked the runnable examples from one category section in the example index. Corrected links, removed duplicate Crossbeam and `Arc<Mutex<T>>` index rows, excluded Kameo/Xtra discussion from the runnable example table, and removed the `arc-swap` recipe index row because there is no runnable example for it. Examples remain in `bk/crates/cats/concurrency`; tests and the final book render are recorded in Validation Closeout below.

## Completed Ninth Migration

Asynchronous Programming: moved the overview, async basics, Futures, Tokio, channels, async traits, streams, sync/async bridges, and utilities into the published category, and merged the existing Cancellation and Structured Concurrency guide into its contents. Corrected crate-table metadata, trait-compatibility wording, await-tree prose, local links, and the existing incoming links. Examples remain in `bk/crates/cats/asynchronous` and `bk/crates/cats/concurrency`; tests and final book rendering are recorded in Validation Closeout below.

## Completed Tenth Migration

Command-line Interface: moved argument parsing, ANSI terminal output, terminal user interfaces, and user interaction, along with the modular Clap builder example and its parsing tests. Updated the category and example indexes, local and incoming links, and corrected stale crate and platform notes. Examples remain in `bk/crates/cats/command_line_interface`; tests and final book rendering are recorded in Validation Closeout below.

## Completed Eleventh Migration

Command-line Utilities: moved the overview and the filesystem, networking, and shell guides, including their command recipes and category tables. Added 21 linkable recipes to the published example index. Replaced draft wiki links with links to published chapters where available, removed links to unpublished topics, and updated incoming links from the OS and Written in Rust indexes. Most examples install or run external tools; network probes and load tests need controlled targets, and packet capture/raw sockets can require platform-specific privileges. The filesystem example package passed its tests; network probes and commands requiring packet-capture privileges remain environment-dependent.

## Completed Twelfth Migration

Development Tools - Testing: moved the overview and the assertions, testing, snapshots, property-based testing, mocking, test runners, coverage, and fuzzing guides. Folded the existing mutation-testing guide into the category, indexed 31 recipes, corrected links and outdated test/fuzzer guidance, and preserved the package examples. The `afl` example is optional and platform-dependent; cargo-fuzz and coverage tools need supported toolchains. Package tests and final rendering are recorded in Validation Closeout below; AFL/cargo-fuzz remain optional toolchain checks.

## Completed Thirteenth Migration

Development Tools - Debugging: moved the debugger, debugging, tracing, logging, metrics, distributed telemetry, and diagnostic-function chapters; indexed their recipes; polished the prose, related topics, and incoming links; and corrected OpenObserve authorization handling. Default telemetry examples work without a collector; exporting to OpenObserve requires a running endpoint and may require an authorization value. Example tests and final rendering are recorded in Validation Closeout below; exporting telemetry requires a configured collector.

## Completed Fourteenth Migration

Development Tools - Profiling: moved the benchmarking, instrumentation, assembly-inspection, and memory-analysis guides, and folded the existing CI benchmark-comparison chapter into the category. Indexed 20 recipes, made the Hyperfine and flamegraph scripts produce named outputs, documented feature and host-tool prerequisites, and updated links from existing published and draft chapters. Example tests and final rendering are recorded in Validation Closeout below.

## Completed Fifteenth Migration

Development Tools - Build Utils: moved the build-script, bundled C/C++, generated-source, build-cache, and `autocfg` guides. Corrected the recipe includes and heading IDs, clarified Cargo build-script behavior, made the build-info example tolerate unavailable Git metadata, and documented the host compiler requirements. Examples remain in `bk/crates/cats/development_tools_build_utils`; package checks and final rendering are recorded in Validation Closeout below.

## Completed Sixteenth Migration

Development Tools - Cargo Plugins: moved the overview and ten chapters covering Cargo commands, dependency management, build automation, cross-compilation, watchers, performance, auditing, maintenance, and writing a Cargo subcommand. Indexed the runnable recipes, added the custom subcommand recipe to the overview, removed duplicate recipe rows, corrected broken anchors and draft-only links, and polished tool guidance. Examples remain in `bk/crates/cats/development_tools_cargo_plugins`; tests and rendering are recorded in [final closeout](#validation-closeout).

## Completed Seventeenth Migration

Development Tools: promoted the remaining 25 chapters for Cargo, package layout, registries, compilation, installation, formatting, documentation, versioning, repository automation, cross-compilation, verification, Rust source tooling, Miri, linking, and transpilation. Integrated them with the published overview and the five component categories, updated related links and references, and added table-only example-index sources where chapters contain prose. The existing examples remain in `bk/crates/cats/development_tools`; `templ` was tested in its isolated workspace. See [final closeout](#validation-closeout) for commands and results.

## 1. rust-patterns

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

## 2. standalone

Overflow: run debug and release behavior. Predicates: run the composite predicate fixture. Plugins: establish actual dynamic loading versus Any-based dispatch. Self-referential: audit pinning/unsafe invariants. Inheritance: repair the duplicate explicit anchor and explain composition/dispatch consistently before promotion.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Plugin Architecture](plugin.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Self-Referential Types](self_referential.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Integer Overflow](overflow.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Inheritance](inheritance.md) | Duplicate heading IDs: `inheritance-in-rust` |
| [Predicates](predicates.md) | No missing include/anchor found; category work and execution/render checks still required |

## 3. web-programming

Promote local URL/HTTP-type/HTML-sanitization fixtures first. Replace scraping and MIME response dependencies on live sites with local fixtures, and verify malicious HTML handling.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Web Programming](categories/web-programming/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [HTTP Types and Interfaces](categories/web-programming/http_types_and_interfaces.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Extracting Links](categories/web-programming/scraping.md) | No missing include/anchor found; category work and execution/render checks still required |
| [URLs](categories/web-programming/url.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Media Types](categories/web-programming/mime.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Sanitize HTML](categories/web-programming/html_sanitization.md) | No missing include/anchor found; category work and execution/render checks still required |

## 4. web-programming_http-client

Use a loopback server for requests, downloads, timeouts, rate limits, and retries. API recipes need mock pagination/error fixtures; gist creation/deletion requires explicit credentials and cleanup.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Web Programming - HTTP Clients](categories/web-programming_http-client/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [HTTP clients](categories/web-programming_http-client/http_clients.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Making Requests](categories/web-programming_http-client/requests.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Calling a Web API](categories/web-programming_http-client/apis.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Rate Limiting and Retries](categories/web-programming_http-client/rate_limiting_and_retries.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Downloads](categories/web-programming_http-client/download.md) | No missing include/anchor found; category work and execution/render checks still required |

## 5. web-programming_http-server

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

## 6. network-programming

Use loopback fixtures for server/proxy shutdown, forwarding, errors, and timeouts; check for conflicts with already published network recipes. The existing `network_programming` package currently fails to compile its Pingora example because three function parameters have `///` comments in attribute position (lines 53, 68, and 71); fix that before promoting this category.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Network Programming](categories/network-programming/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Server](categories/network-programming/server.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Reverse Proxy](categories/network-programming/reverse_proxy.md) | No missing include/anchor found; category work and execution/render checks still required |

## 7. database

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

## 8. database-implementations

Reconcile the published database implementation material; validate embedded search locally and distinguish application recipes from implementation surveys.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Database Implementations](categories/database-implementations/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Databases](categories/database-implementations/databases.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Search](categories/database-implementations/rust_search_engines.md) | No missing include/anchor found; category work and execution/render checks still required |

## 9. cryptography

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

## 10. authentication

Basic auth needs HTTPS context. Session/CSRF flows need request-level tests; OAuth/OIDC need mock issuers or configured providers; passkeys require browser/authenticator ceremony validation.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Authentication](categories/authentication/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Basic Authentication](categories/authentication/basic_authentication.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Session Login](categories/authentication/session_login.md) | No missing include/anchor found; category work and execution/render checks still required |
| [OAuth 2.0 Client Credentials](categories/authentication/oauth_client_credentials.md) | No missing include/anchor found; category work and execution/render checks still required |
| [OpenID Connect Authorization Code with PKCE](categories/authentication/oidc_authorization_code.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Passkeys (WebAuthn)](categories/authentication/passkeys.md) | No missing include/anchor found; category work and execution/render checks still required |

## 11. email

Parsing can use checked-in messages; sending and reading require local SMTP/IMAP fixtures or explicitly configured accounts, timeouts, and cleanup.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Email](categories/email/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Sending emails](categories/email/send_emails.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Email parsing](categories/email/email_parsing.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Reading emails](categories/email/read_emails.md) | No missing include/anchor found; category work and execution/render checks still required |

## 12. os

Run child-process/signal examples on their supported platforms with bounded subprocesses; avoid changing the parent shell or relying on machine-specific commands.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Operating Systems](categories/os/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [External Commands](categories/os/external_commands.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Process Management](categories/os/process_management.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Signals and Graceful Shutdown](categories/os/signals.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Low-level System Calls](categories/os/low_level_system_calls.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Rust OSes](categories/os/rust_os.md) | No missing include/anchor found; category work and execution/render checks still required |

## 13. os_unix-apis

Validate on supported Unix hosts with scoped process/filesystem resources; identify Linux-specific assumptions rather than claiming portability.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Operating systems - Unix](categories/os_unix-apis/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Unix](categories/os_unix-apis/unix.md) | No missing include/anchor found; category work and execution/render checks still required |

## 14. os_windows-apis

Requires a Windows host and relevant APIs/permissions; Linux compilation or prose inspection does not validate Windows runtime behavior.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Operating systems - Windows](categories/os_windows-apis/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Windows](categories/os_windows-apis/windows.md) | No missing include/anchor found; category work and execution/render checks still required |

## 15. hardware-support

Processor examples need architecture feature checks; peripherals need actual compatible devices and permissions. Keep hardware prose separate from validated runnable recipes.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Hardware Support](categories/hardware-support/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Processor](categories/hardware-support/processor.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Peripherals](categories/hardware-support/peripherals.md) | No missing include/anchor found; category work and execution/render checks still required |

## 16. text-editors

Check current editor configuration and execute the LSP examples against a local client/server. Reconcile contributor IDE setup material.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Text Editors](categories/text-editors/index.md) | No missing include/anchor found; category work and execution/render checks still required |
| [IDEs](categories/text-editors/ides.md) | No missing include/anchor found; category work and execution/render checks still required |
| [Language Server Protocol](categories/text-editors/language_server_protocol.md) | No missing include/anchor found; category work and execution/render checks still required |

## 17. graphics

Compare the graphics overview with the published graphics recipes; merge unique guidance only after checking links and identifying platform/tool requirements.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Graphics](categories/graphics/index.md) | No missing include/anchor found; category work and execution/render checks still required |

## 18. caching

Draft overview is a redirect to already published caching/in-memory cache; no chapter promotion remains. Verify destination and retire the duplicate summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Caching](categories/caching/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## 19. compression

Draft overview is a redirect to already published compression; no chapter promotion remains. Verify destination and retire the duplicate summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Compression](categories/compression/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## 20. date-and-time

This is a redirect to the published date/time chapter; verify its destination and retire the duplicate draft-summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Date and Time](categories/date-and-time/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## 21. data-structures

Compare with published collections and data-structures chapters; merge only unique guidance and repair links, rather than duplicating the category.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Data Structures](categories/data-structures/index.md) | Missing include `phf.incl.md`; Missing include `elsa.incl.md`; Published path exists: compare/merge or retire redirect; do not overwrite |

## 22. parsing

This is a redirect to the published parsing/expression parser material; verify its destinations and retire the duplicate draft-summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Parsing](categories/parsing/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## 23. template-engine

This is a redirect to published Tera/TinyTemplate recipes; verify its destinations and retire the duplicate draft-summary entry.

| Chapter in draft summary | Remaining work / blockers |
| --- | --- |
| [Template Engines](categories/template-engine/index.md) | Published path exists: compare/merge or retire redirect; do not overwrite |

## Published Upstream During PR Preparation

- [OpenAPI and JSON Schema](../src/categories/web-programming_http-server/openapi.md): already published on current `develop`; removed from the migration queue.

## Outside the Draft Summary

- [VS Code Configuration](contributing/vscode.md): reconcile with published contributor environment setup, check the current `rust-analyzer.linkedProjects` paths, and validate in VS Code before merging unique guidance. This is a chapter candidate omitted from the summary.
- [FRAGMENTS.md](FRAGMENTS.md) and [TOREVIEW.md](TOREVIEW.md): authoring/curation records, not recipe chapters. Preserve their dispositions and assess any unique source notes separately.

## Validation Closeout

- The affected book-example packages passed `cargo test --locked --all-targets` (951 passed, 14 ignored, 0 failed): `text_processing`, `parser_implementations`, `encoding`, `config@0.1.2`, `memory_management`, `concurrency`, `asynchronous`, `command_line_interface`, `filesystem`, `development_tools`, `development_tools_testing`, `development_tools_debugging`, `development_tools_profiling`, `development_tools_build_utils`, and `cargo`. The selected tests include the published regex recipes and the new profiling and Cap'n Proto fixes.
- The isolated `allocator_selection` example passed once without an allocator feature, once with `use-mimalloc`, and once with `use-jemalloc`. The isolated `templ` workspace passed 42 tests with 1 ignored.
- The HTML book rendered successfully; linkcheck reported zero incomplete links or missing destinations. The indexing plugin reports that it was built against mdBook 0.5.1 while mdBook 0.5.4 is running, the search index is large, and the optional Pandoc backend is unavailable. These did not fail the HTML or linkcheck outputs.
- Optional AFL/cargo-fuzz runs and service-dependent telemetry/network operations remain environment-dependent; their constraints are noted in the relevant category sections. A test attempt of the separate, not-yet-migrated `network_programming` package exposed its Pingora comment compile error, recorded under the Network Programming blocker. The draft preview still has missing `phf.incl.md` and `elsa.incl.md` includes in the Data Structures overview; Data Structures remains blocked until those includes are supplied and its chapter is reviewed.
