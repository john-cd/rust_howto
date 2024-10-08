# Rust How-to

[Introduction](index.md)
[About](_about.md)
[Index of examples](_examples_index.md)

- [Contributing](contributing/index.md)
  - [Topics of interest](contributing/topics_of_interest.md)
  - [Repository structure](contributing/repo_structure.md)
  - [Environment setup](contributing/dev_environment_setup.md)
  - [Editing](contributing/development_editing.md)
  - [Dev Containers and Docker](contributing/dev_container_docker.md)
  - [Optional preprocessors](contributing/optional_preprocessors.md)
  - [API documentation](contributing/api_documentation.md)
  - [Crate publication](contributing/publication.md)

---

- [Language](lang/index.md)
  - [Rust installation and first steps](lang/rust_install.md)
  - [Main function](lang/main.md)
  - [Simple data types](lang/simple_data_types.md)
  - [Variables and constants](lang/variables_and_constants.md)
  - [Ownership and borrowing](lang/ownership_borrowing.md)
  - [Slices](lang/slices.md)
  - [Functions](lang/functions.md)
  - [Control flow](lang/control_flow.md)
  - [Structs](lang/structs.md)
  - [Enums](lang/enums.md)
  - [Traits](lang/traits.md)
  - [Trait objects](lang/trait_objects.md)
  - [Attributes](lang/attributes.md)
  - [Generics](lang/generics.md)
  - [Lifetimes](lang/lifetimes.md)
  - [Modules and `use` keyword](lang/modules.md)
  - [Pattern matching, if / while let](lang/match.md)
  - [Closures](lang/closures.md)
  - [Iterators](lang/iterators.md)
  - [Macros](lang/macros.md)

- [Standard library](standard_library/index.md)
  - [Option](standard_library/option.md)
  - [Vectors](standard_library/vectors.md)
  - [HashMap](standard_library/hashmaps.md)
  - [Strings](standard_library/strings.md)
  - [Copy-on-write](standard_library/cow.md)
  - [Smart pointers](standard_library/smart_pointers.md)
  - [Automatic trait derivation](standard_library/derive.md)

- [Key crates](key_crates.md)
  - [Crates (alphabetic)](crates.md)
  - [Crates (by category)](_categories.md)

---

- [Algorithms](categories/algorithms/index.md)
  - [Generate random values](categories/algorithms/randomness.md)
  - [Sort a vector](categories/algorithms/sorting.md)

- [API Bindings](categories/api-bindings/_index.md)

- [Asynchronous](categories/asynchronous/index.md)
  - [Async and traits](categories/asynchronous/async_traits.md)
  - [Tokio async runtime](categories/asynchronous/tokio.md)
  - [Async channels](categories/asynchronous/async_channels.md)
  - [Streams](categories/asynchronous/streams.md)
  - [Futures crate](categories/asynchronous/futures.md)
  - [Mixing async and blocking code](categories/asynchronous/async_and_blocking.md)

- [Authentication](categories/authentication/index.md)

- [Caching](categories/caching/_index.md)

- [Cloud](other/cloud/_index.md)

- [Command-line interface](categories/command-line-interface/index.md)
  - [Argument parsing](categories/command-line-interface/arguments.md)
  - [ANSI terminal](categories/command-line-interface/ansi_terminal.md)

- [Command-line utilities](categories/command-line-utilities/_index.md)

- [Compilers](categories/compilers/_index.md)
  - [Compilation duration reduction](categories/compilers/reduce_compilation_duration.md)
  - [Faster linking](categories/compilers/faster_linking.md)
  - [Cross-compilation](categories/compilers/cross_compilation.md)

- [Compression](categories/compression/_index.md)
  - [Working with tarballs](categories/compression/tar.md)

- [Computer Vision](categories/computer-vision/_index.md)

- [Concurrency](categories/concurrency/index.md)
  - [Multi-threading](categories/concurrency/multithreading.md)
  - [Explicit threads](categories/concurrency/threads.md)
  - [Data parallelism](categories/concurrency/parallel.md)
  - [Message passing](categories/concurrency/message_passing.md)
  - [Shared-state concurrency](categories/concurrency/shared_state/index.md)
  - [Concurrent data structures](categories/concurrency/shared_state/concurrent_data_structures.md)

- [Configuration](categories/config/index.md)
  - [Environment variables](categories/config/environment_variables.md)
  - [Configuration management](categories/config/configuration.md)

- [Cross-platform Development](other/cross_platform/index.md)

- [Cryptography](categories/cryptography/index.md)
  - [Hashing](categories/cryptography/hashing.md)
  - [Encryption](categories/cryptography/encryption.md)

- [Data Processing](other/data_processing/_index.md)

- [Data structures](categories/data-structures/_index.md)
  - [Bitfield](categories/data-structures/bitfield.md)

- [Database access](categories/database/_index.md)
  - [SQLite](categories/database/sqlite.md)
  - [Postgres](categories/database/postgres.md)
  - [Query builders and ORMs](categories/database/query_builders_orms.md)

- [Database implementations](categories/database-implementations/_index.md)

- [Date and time](categories/date-and-time/index.md)
  - [Duration and calculation](categories/date-and-time/duration.md)
  - [Parsing and displaying](categories/date-and-time/parse.md)

- [Development tools](categories/development-tools/index.md)
  - [Cargo](categories/development-tools/cargo/index.md)
    - [Package layout](categories/development-tools/cargo/package_layout.md)
    - [Crate registries](categories/development-tools/cargo/crates.md)
  - [Rust and binary installation](categories/development-tools/installation/index.md)
    - [Rustup](categories/development-tools/installation/rustup.md)
  - [Code editing](categories/text-editors/_index.md)
  - [Code formatting](categories/development-tools/formatting/index.md)
  - [Documentation](categories/development-tools/documentation/index.md)
    - [mdBook](categories/development-tools/documentation/mdbook.md)
    - [Badges](categories/development-tools/documentation/badges.md)
    - [Rust playground](categories/development-tools/documentation/playground.md)
  - [Versioning](categories/development-tools/versioning/index.md)
  - [Other](categories/development-tools/other/index.md)
    - [Miri](categories/development-tools/other/miri.md)
    - [Just](categories/development-tools/other/just.md)

- [Development tools - build-time tooling](categories/development-tools_build-utils/_index.md)

- [Development tools - cargo plugins](categories/development-tools_cargo-plugins/_index.md)

- [Development tools - debugging, logging](categories/development-tools_debugging/index.md)
  - [Tracing](categories/development-tools_debugging/tracing.md)
  - [Log messages](categories/development-tools_debugging/log.md)
  - [Logging configuration](categories/development-tools_debugging/config_log.md)

- [Development tools - FFI](categories/development-tools_ffi/_index.md)

- [Development tools - procedural macro helpers](categories/development-tools_procedural-macro-helpers/_index.md)

- [Development tools - performance, profiling](categories/development-tools_profiling/_index.md)

- [Development tools - testing](categories/development-tools_testing/index.md)

- [Email](categories/email/_index.md)

- [Embedded development](categories/embedded/_index.md)

- [Encoding and serialization](categories/encoding/index.md)
  - [Character sets](categories/encoding/strings.md)
  - [CSV processing](categories/encoding/csv.md)
  - [Structured data](categories/encoding/complex.md)
  - [Serde](categories/encoding/serde.md)

- [Error handling](categories/rust-patterns/errors/index.md)
  - [Handle error variants](categories/rust-patterns/errors/handle.md)
  - [Error handling](categories/rust-patterns/errors/error_handling.md)
  - [Error customization](categories/rust-patterns/errors/error_customization.md)

- [File system](categories/filesystem/index.md)
  - [Read & write](categories/filesystem/read-write.md)
  - [Directory traversal](categories/filesystem/dir.md)

- [Finance](categories/finance/_index.md)

- [Game engines](categories/game-engines/_index.md)

- [Graphics](categories/graphics/_index.md)

- [GPU programming](other/gpu/_index.md)

- [GUI](categories/gui/_index.md)

- [Hardware support](categories/hardware-support/index.md)
  - [Processor](categories/hardware-support/processor.md)

- [Internationalization](categories/internationalization/_index.md)

- [Localization](categories/localization/_index.md)

- [Machine learning](categories/science/machine_learning/_index.md)

- [Mathematics](categories/mathematics/_index.md)
  - [Linear algebra](categories/mathematics/linear_algebra.md)
  - [Trigonometry](categories/mathematics/trigonometry.md)
  - [Complex numbers](categories/mathematics/complex_numbers.md)
  - [Statistics](categories/mathematics/statistics.md)
  - [Miscellaneous](categories/mathematics/miscellaneous.md)

- [Memory management](categories/memory-management/_index.md)
  - [Global static](categories/memory-management/global_static.md)
  - [Lazy initialization](categories/memory-management/lazy_initialization.md)

- [Multimedia](categories/multimedia/_index.md)

- [Network](categories/network-programming/index.md)
  - [Server](categories/network-programming/server.md)

- [No std](categories/no-std/_index.md)

- [Operating systems](categories/os/index.md)
  - [External commands](categories/os/external.md)

- [Other domains](other/_index.md)

- [Parsing](categories/parsing/_index.md)

- [Rendering](categories/rendering/_index.md)

- [Robotics](categories/science_robotics/_index.md)

- [Rust Patterns](categories/rust-patterns/_index.md)

- [Science](categories/science/_index.md)

- [Simulation](categories/simulation/_index.md)

- [Template engines](categories/template-engine/_index.md)

- [Text editors](categories/text-editors/_index.md)

- [Text processing](categories/text-processing/index.md)
  - [Regular expressions](categories/text-processing/regex.md)
    - [Longer regex example](categories/text-processing/regex2.md)
  - [String parsing](categories/text-processing/string_parsing.md)

- [Virtualization](categories/virtualization/_index.md)

- [Visualization](categories/visualization/_index.md)

- [Web assembly](categories/wasm/_index.md)

- [Web programming](categories/web-programming/index.md)
  - [Extracting links](categories/web-programming/scraping.md)
  - [URL](categories/web-programming/url.md)
  - [Media types](categories/web-programming/mime.md)

- [Web programming - HTTP clients](categories/web-programming_http-client/_index.md)
  - [Making requests](categories/web-programming_http-client/requests.md)
  - [Calling a web API](categories/web-programming_http-client/apis.md)
  - [Downloads](categories/web-programming_http-client/download.md)

- [Web programming - HTTP servers](categories/web-programming_http-server/index.md)
  - [Axum](categories/web-programming_http-server/axum.md)
  - [Actix](categories/web-programming_http-server/actix.md)
  - [Other web frameworks](categories/web-programming_http-server/other_frameworks.md)
  - [Static website generators](categories/web-programming_http-server/static_website_generators.md)
  - [Middleware](categories/web-programming_http-server/middleware.md)
  - [CORS](categories/web-programming_http-server/cors.md)

- [Web programming - websocket](categories/web-programming_websocket/_index.md)

- [Windows](categories/os_windows-apis/_index.md)

---

- [Links](links/index.md)
  - [Example code](links/example_code.md)
  - [Cheat sheets](links/rust_cheatsheets.md)
  - [Blogs](links/blogs.md)
  - [Books](links/books.md)
  - [Companies](links/companies.md)

[Index](word_index.md)

[Thanks](thanks.md)
