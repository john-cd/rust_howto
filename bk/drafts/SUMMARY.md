<!-- TODO Reviewed ??-??-2026. -->

# Categories

<!-- Ordered migration review and remaining work: MIGRATION.md. -->

- [Authentication](categories/authentication/index.md)
  - [Basic Authentication](categories/authentication/basic_authentication.md)
  - [Session Login](categories/authentication/session_login.md)
  - [OAuth 2.0 Client Credentials](categories/authentication/oauth_client_credentials.md)
  - [OpenID Connect Authorization Code with PKCE](categories/authentication/oidc_authorization_code.md)
  - [Passkeys (WebAuthn)](categories/authentication/passkeys.md)

- [Caching](categories/caching/index.md)

- [Command-line Utilities](categories/command-line-utilities/index.md)
  - [Filesystem](categories/command-line-utilities/filesystem_cli.md)
  - [Networking](categories/command-line-utilities/networking_cli.md)
  - [Shells](categories/command-line-utilities/shells.md)

- [Compression](categories/compression/index.md)

- [Cryptography](categories/cryptography/index.md)
  - [Password Hashing](categories/cryptography/password_hashing.md)
  - [Encryption](categories/cryptography/encryption.md)
  - [AEAD](categories/cryptography/aead.md)
  - [Protect Secrets](categories/cryptography/secrets.md)
  - [Signatures](categories/cryptography/signature.md)
  - [HMAC](categories/cryptography/hmac.md)
  - [Certificates](categories/cryptography/certificates.md)
  - [TLS](categories/cryptography/tls.md)
  - [Cryptographic Utilities](categories/cryptography/cryptography_utilities.md)

- [Database Access](categories/database/index.md)
  - [SQLite](categories/database/sqlite.md)
  - [Postgres](categories/database/postgres.md)
  - [MSSQL](categories/database/mssql.md)
  - [MySQL](categories/database/mysql.md)
  - [Oracle](categories/database/oracle.md)
  - [Connection Pools](categories/database/connection_pool.md)
  - [Query Builders and ORMs](categories/database/query_builders_orms.md)
  - [NoSQL](categories/database/nosql.md)
  - [Key-Value Stores](categories/database/key_value_stores.md)
  - [Search](categories/database/search.md)
  - [AMQP](categories/database/amqp.md)

- [Database Implementations](categories/database-implementations/index.md)
  - [Databases](categories/database-implementations/databases.md)
  - [Search](categories/database-implementations/rust_search_engines.md)

- [Date and Time](categories/date-and-time/index.md)

- [Data Structures](categories/data-structures/index.md)

- [Development Tools](categories/development-tools/index.md)
  - [Cargo](categories/development-tools/cargo/cargo.md)
    - [Package Layout](categories/development-tools/cargo/package_layout.md)
    - [Crate Registries](categories/development-tools/cargo/crate_registries.md)
  - [Build Rust Code](categories/development-tools/compilation/building.md)
  - [Rust and Binary Installation](categories/development-tools/installation/install.md)
    - [Rustup](categories/development-tools/installation/rustup.md)
  - [Code Formatting](categories/development-tools/formatting/formatting.md)
  - [Documentation](categories/development-tools/documentation/documentation.md)
    - [`mdBook`](categories/development-tools/documentation/mdbook.md)
    - [`mdbook-utils`](categories/development-tools/documentation/mdbook_utils.md)
    - [`templ`](categories/development-tools/documentation/templ.md)
    - [Badges](categories/development-tools/documentation/badges.md)
  - [Versioning](categories/development-tools/versioning/versioning.md)
  - [Repository Automation](categories/development-tools/automation/index.md)
    - [`xtask` Project Checks](categories/development-tools/automation/xtask.md)
    - [Git History with `gix`](categories/development-tools/automation/git.md)
  - [Other](categories/development-tools/other/other.md)
    - [Cross-compilation](categories/development-tools/cross-compilation/cross_compilation.md)
    - [Code Verification](categories/development-tools/other/code_verification.md)
    - [Analyze and Generate Rust Code](categories/development-tools/other/rust_source.md)
    - [`miri`](categories/development-tools/other/miri.md)
    - [Faster Linking](categories/development-tools/compilation/faster_linking.md)
    - [Reduce Compilation Duration](categories/development-tools/compilation/reduce_compilation_duration.md)
    - [Transpilers](categories/development-tools/transcompilation/transpilers.md)

- [Development Tools - Build-time Tooling](categories/development-tools_build-utils/index.md)
  - [Build-time Tools](categories/development-tools_build-utils/build_time_tooling.md)
  - [`autocfg`](categories/development-tools_build-utils/autocfg.md)
  - [Build Cache](categories/development-tools_build-utils/build_cache.md)

- [Development Tools - Cargo Plugins](categories/development-tools_cargo-plugins/index.md)
  - [Writing Code](categories/development-tools_cargo-plugins/code_writing.md)
  - [Code Formatting, Linting](categories/development-tools_cargo-plugins/code_formatting_linting.md)
  - [Dependency Management](categories/development-tools_cargo-plugins/dependency_management.md)
  - [Building](categories/development-tools_cargo-plugins/building.md)
  - [Cross-compiling](categories/development-tools_cargo-plugins/cross_compiling.md)
  - [Watching for Changes](categories/development-tools_cargo-plugins/watching_for_changes.md)
  - [Improving Performance](categories/development-tools_cargo-plugins/performance.md)
  - [Auditing](categories/development-tools_cargo-plugins/auditing.md)
  - [Maintaining](categories/development-tools_cargo-plugins/maintaining.md)
  - [Write a Cargo Plugin](categories/development-tools_cargo-plugins/write_cargo_plugin.md)

- [Development Tools - Debugging, Logging](categories/development-tools_debugging/index.md)
  - [Debuggers](categories/development-tools_debugging/debugger.md)
  - [Debugging](categories/development-tools_debugging/debugging.md)
  - [Tracing](categories/development-tools_debugging/tracing.md)
  - [Alternatives](categories/development-tools_debugging/tracing_alternatives.md)
  - [Log Messages](categories/development-tools_debugging/log.md)
  - [Logging Configuration](categories/development-tools_debugging/config_log.md)
  - [Metrics](categories/development-tools_debugging/metrics.md)
  - [Distributed Telemetry](categories/development-tools_debugging/distributed_telemetry.md)
  - [Diagnostic Functions](categories/development-tools_debugging/diagnostic_functions.md)

- [Development Tools - Performance, Profiling](categories/development-tools_profiling/index.md)
  - [Benchmarking](categories/development-tools_profiling/benchmarking.md)
  - [Instrumentation Profiling](categories/development-tools_profiling/instrumentation_profiling.md)
  - [Low-level Profiling Tools](categories/development-tools_profiling/assembly.md)
  - [Memory Usage Analysis](categories/development-tools_profiling/memory_usage_analysis.md)

- [Development Tools - Testing](categories/development-tools_testing/index.md)
  - [Assertions](categories/development-tools_testing/assertions.md)
  - [Testing](categories/development-tools_testing/testing.md)
  - [Snapshot Testing](categories/development-tools_testing/snapshot_testing.md)
  - [Property-based Testing](categories/development-tools_testing/property_based_testing.md)
  - [Mocking](categories/development-tools_testing/mocking.md)
  - [Test Runners](categories/development-tools_testing/test_runners.md)
  - [Code Coverage](categories/development-tools_testing/code_coverage.md)
  - [Fuzzing](categories/development-tools_testing/fuzzing.md)

- [Email](categories/email/index.md)
  - [Sending emails](categories/email/send_emails.md)
  - [Email parsing](categories/email/email_parsing.md)
  - [Reading emails](categories/email/read_emails.md)


- [Error Handling](categories/rust-patterns/error_handling/error_handling.md)
  - [Error Customization](categories/rust-patterns/error_handling/error_customization.md)

- [Graphics](categories/graphics/index.md)

- [Hardware Support](categories/hardware-support/index.md)
  - [Processor](categories/hardware-support/processor.md)
  - [Peripherals](categories/hardware-support/peripherals.md)

- [Network Programming](categories/network-programming/index.md)
  - [Server](categories/network-programming/server.md)
  - [Reverse Proxy](categories/network-programming/reverse_proxy.md)

- [Operating Systems](categories/os/index.md)
  - [External Commands](categories/os/external_commands.md)
  - [Process Management](categories/os/process_management.md)
  - [Signals and Graceful Shutdown](categories/os/signals.md)
  - [Low-level System Calls](categories/os/low_level_system_calls.md)
  - [Rust OSes](categories/os/rust_os.md)

- [Operating systems - Unix](categories/os_unix-apis/index.md)
  - [Unix](categories/os_unix-apis/unix.md)

- [Operating systems - Windows](categories/os_windows-apis/index.md)
  - [Windows](categories/os_windows-apis/windows.md)

- [Parsing](categories/parsing/index.md)


- [Rust Patterns](categories/rust-patterns/index.md)
  - [Creational Patterns](categories/rust-patterns/creational_patterns.md)
    - [Builder Pattern](categories/rust-patterns/builder_pattern.md)
  - [Structural Patterns](categories/rust-patterns/structural_patterns.md)
  - [Behavioral Design Patterns](categories/rust-patterns/behavioral_patterns.md)
  - [Functional Programming](categories/rust-patterns/functional_programming.md)
  - [Rust-specific Patterns](categories/rust-patterns/rust_specific_patterns.md)

- [Template Engines](categories/template-engine/index.md)

- [Text Editors](categories/text-editors/index.md)
  - [IDEs](categories/text-editors/ides.md)
  - [Language Server Protocol](categories/text-editors/language_server_protocol.md)


- [Web Programming](categories/web-programming/index.md)
  - [HTTP Types and Interfaces](categories/web-programming/http_types_and_interfaces.md)
  - [Extracting Links](categories/web-programming/scraping.md)
  - [URLs](categories/web-programming/url.md)
  - [Media Types](categories/web-programming/mime.md)
  - [Sanitize HTML](categories/web-programming/html_sanitization.md)

- [Web Programming - HTTP Clients](categories/web-programming_http-client/index.md)
  - [HTTP clients](categories/web-programming_http-client/http_clients.md)
  - [Making Requests](categories/web-programming_http-client/requests.md)
  - [Calling a Web API](categories/web-programming_http-client/apis.md)
  - [Rate Limiting and Retries](categories/web-programming_http-client/rate_limiting_and_retries.md)
  - [Downloads](categories/web-programming_http-client/download.md)

- [Web Programming - HTTP Servers](categories/web-programming_http-server/index.md)
  - [Internationalized API Errors](categories/web-programming_http-server/i18n.md)
  - [Axum](categories/web-programming_http-server/_axum.md)
  - [Actix](categories/web-programming_http-server/_actix.md)
  - [Other Web Frameworks](categories/web-programming_http-server/other_frameworks.md)
  - [Static Website Generators](categories/web-programming_http-server/static_website_generators.md)
  - [Middleware](categories/web-programming_http-server/middleware.md)
  - [CORS](categories/web-programming_http-server/cors.md)
  - [Axum Add-ons](categories/web-programming_http-server/axum_extras.md)
  - [Batteries-included Frameworks](categories/web-programming_http-server/_batteries-included_frameworks.md)
  - [GraphQL](categories/web-programming_http-server/_graphql.md)
  - [gRPC](categories/web-programming_http-server/_grpc.md)
  - [`hyper`](categories/web-programming_http-server/_hyper.md)

- [Plugin Architecture](plugin.md)

- [Self-Referential Types](self_referential.md)

- [Integer Overflow](overflow.md)

- [Inheritance](inheritance.md)

- [Predicates](predicates.md)
