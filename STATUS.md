# STATUS

[Introduction](src/index.md) polish the intro
[About](src/_about.md) rewrite
[Contributing](src/contributing/index.md)
[Index of Examples](src/_examples_index.md) finish

- [Language](src/lang/index.md)
  - [Rust installation and first steps](src/lang/rust_install.md) polish
  - [Main function](src/lang/main.md) add description
  - [Simple data types](src/lang/simple_data_types.md) add description
  - [Variables and constants](src/lang/variables_and_constants.md) add description
  - [Ownership and borrowing](src/lang/ownership_borrowing.md)   add description
  - [Slices](src/lang/slices.md)   add description
  - [Functions](src/lang/functions.md)   add description
  - [Control flow](src/lang/control_flow.md) add description
  - [Structs](src/lang/structs.md)
  - [Enums](src/lang/enums.md)
  - [Traits](src/lang/traits.md)
  - [Trait objects](src/lang/trait_objects.md)
  - [Attributes](src/lang/attributes.md)
  - [Generics](src/lang/generics.md)
  - [Lifetimes](src/lang/lifetimes.md)
  - [Modules and `use` keyword](src/lang/modules.md)
  - [Pattern matching, if / while let](src/lang/match.md)
  - [Closures](src/lang/closures.md)
  - [Iterators](src/lang/iterators.md)
  - [Macros](src/lang/macros.md)

- [Standard library](src/standard_library/index.md)
  - [Option](src/standard_library/option.md) finish Option page map, unwrap_or
  - [Vectors](src/standard_library/vectors.md)
  - [HashMap](src/standard_library/hashmaps.md)
  - [Strings](src/standard_library/strings.md)
  - [Smart pointers](src/standard_library/smart_pointers.md)

- [Key crates](src/key_crates.md)

- [Development tools](src/categories/development-tools/index.md)
  - [Cargo](src/categories/development-tools/cargo/index.md) finish Cargo Plugins / cargo.md
  - [Package layout](src/categories/development-tools/cargo/package_layout.md)
  - [Installation](src/categories/development-tools/installation/index.md)
    - [Rustup](src/categories/development-tools/installation/rustup.md)
  - [Code editing and formatting](src/categories/development-tools/editing/index.md) add latest JetBrains tooling
  - [Compiling](src/categories/compilers/_index.md)
    - [Faster linking](src/categories/compilers/faster_linking.md)  finish faster linking
    - [Cross-compilation](src/categories/compilers/cross_compilation.md)
  - [Build-time tooling](src/categories/development-tools/build_tools.md)
  - [Testing](src/categories/development-tools/testing.md)
  - [Performance profiling](src/categories/development-tools/performance.md)
  - [Miri](src/categories/development-tools/other/miri.md)
  - [Documentation](src/categories/development-tools/documentation/index.md)
    - [mdBook](src/categories/development-tools/documenting/mdbook.md)
  - [Other](src/categories/development-tools/other.md)
    - [Other tools](src/categories/development-tools/other/other_tools.md)
    - [Crate registries](src/categories/development-tools/cargo/crates.md)
    - [Just](src/categories/development-tools/other/just.md)
  - [Versioning](src/categories/development-tools/versioning.md)

---

- [Algorithms](src/categories/algorithms/index.md)
  - [Generate Random Values](src/categories/algorithms/randomness.md)
  - [Sort a Vector](src/categories/algorithms/sorting.md)

- [Automatic trait derivation TODO](src/standard_library/derive.md)

- [Command-line applications](src/categories/command-line-interface/index.md)
  - [Argument Parsing](src/categories/command-line-interface/arguments.md)
  - [ANSI Terminal](src/categories/command-line-interface/ansi_terminal.md)

- [Compression](src/categories/compression/_index.md)
  - [Working with Tarballs](src/categories/compression/tar.md)

- [Concurrency](src/categories/concurrency/index.md)
  - [Multi-threading](src/categories/concurrency/multithreading.md)
  - [Explicit threads](src/categories/concurrency/threads.md)
  - [Data parallelism](src/categories/concurrency/parallel.md)
  - [Message passing](src/categories/concurrency/message_passing.md)
  - [Shared-state concurrency](src/categories/concurrency/shared_state/index.md)
  - [Concurrent data structures](src/concurrency/shared_state/concurrent_data_structures.md)

- [Concurrency - Async](src/concurrency/async.md)
  - [Async and traits](src/async/async_traits.md)
  - [Tokio async runtime](src/async/tokio.md) add more to tokio.md
  - [Async channels](src/async/async_channels.md) add more to async_channels
  - [Streams](src/async/streams.md)
  - [Futures crate](src/async/futures.md)
  - [Mixing async and blocking code](src/async/async_and_blocking.md)

- [Configuration TODO](src/categories/config/index.md)

- [Cross-cutting concerns TODO](src/cross_cutting_concerns.md)

- [Cryptography](src/categories/cryptography/index.md)
  - [Hashing](src/categories/cryptography/hashing.md)
  - [Encryption](src/cryptography/encryption.md)

- [Data structures](src/categories/data-structures/_index.md)
  - [Bitfield](src/categories/data-structures/bitfield.md)

- [Databases](src/categories/database/_index.md)
  - [SQLite](src/categories/database/sqlite.md)
  - [Postgres](src/categories/database/postgres.md)
  - [Query builders and ORMs](src/categories/database/query_builders_orms.md)

- [Date and time](src/categories/date-and-time/index.md)
  - [Duration and calculation](src/categories/date-and-time/duration.md)
  - [Parsing and displaying](src/categories/date-and-time/parse.md)

- [Encoding and Serialization](src/categories/encoding/index.md)
  - [Character sets](src/categories/encoding/strings.md)
  - [CSV processing](src/categories/encoding/csv.md)
  - [Structured data](src/categories/encoding/complex.md)
  - [Serde](src/categories/encoding/serde.md)

- [Error handling](src/categories/rust-patterns/_index.md) errors: color-eyre, eyre
  - [Handle error variants](src/categories/rust-patterns/errors/handle.md)
  - [Error handling](src/categories/rust-patterns/errors/error_handling.md)
  - [Error customization](src/categories/rust-patterns/errors/error_customization.md)

- [File system](src/categories/filesystem/index.md)
  - [Read & write](src/categories/filesystem/read-write.md)
  - [Directory traversal](src/categories/filesystem/dir.md)

- [Hardware support](src/categories/hardware-support/index.md)
  - [Processor](src/categories/hardware-support/processor.md)

- [Logging](src/categories/development-tools_debugging/index.md)
  - [Tracing](src/categories/development-tools_debugging/tracing.md)
  - [Log messages](src/categories/development-tools_debugging/log.md)
  - [Configure logging](src/categories/development-tools_debugging/config_log.md)

- [Mathematics](src/categories/mathematics/_index.md)
  - [Linear algebra](src/categories/mathematics/linear_algebra.md)
  - [Trigonometry](src/categories/mathematics/trigonometry.md)
  - [Complex numbers](src/categories/mathematics/complex_numbers.md)
  - [Statistics](src/categories/mathematics/statistics.md)
  - [Miscellaneous](src/categories/mathematics/miscellaneous.md)

- [Memory management](src/categories/memory-management/_index.md)
  - [Global static](src/categories/memory-management/global_static.md)
  - [Lazy initialization TODO](src/categories/memory-management/lazy_initialization.md)

- [Network](src/categories/network-programming/index.md)
  - [Server](src/categories/network-programming/server.md)

- [Operating system](src/categories/os/index.md)
  - [External commands](src/categories/os/external.md)

- [Text processing](src/categories/text-processing/index.md)
  - [Regular expressions](src/categories/text-processing/regex.md)
    - [Longer regex example](src/categories/text-processing/regex2.md)
  - [String parsing](src/categories/text-processing/string_parsing.md)

- [Web programming](src/categories/web-programming/index.md)
  - [Extracting links](src/categories/web-programming/scraping.md)
  - [URL](src/categories/web-programming/url.md)
  - [Media types](src/categories/web-programming/mime.md)
  - [Web frameworks](src/categories/web-programming/server.md)
    - [Axum](src/categories/web-programming/server/axum.md)
    - [Actix](src/categories/web-programming/actix.md)
    - [Other web frameworks](src/web/server/other_frameworks.md)
    - [Middleware](src/web/server/middleware.md)
    - [CORS](src/web/server/cors.md)
  - [Static website generators](src/web/server/static_website_generators.md)
  - [Web Clients](src/web/clients.md)
    - [Making Requests](src/clients/requests.md)
    - [Calling a Web API](src/clients/apis.md)
    - [Downloads](src/clients/download.md)
    - [Web Authentication](src/clients/authentication.md)

---

- [Other domains](src/other/domains.md)
  - [WASM](src/domains/wasm.md)
  - [GUI](src/domains/gui.md)
  - [Cross-platform applications](src/domains/cross_platform.md)
  - [Cloud](src/domains/cloud.md)
  - [Data](src/domains/data.md)
  - [ML](src/domains/ml.md)
  - [Games](src/domains/games.md)
  - [GPU programming](src/domains/gpu.md)
  - [Robotics](src/domains/robotics.md)
  - [Windows](src/domains/windows.md)

- [Links](src/links/index.md)
  - [Example code](src/links/example_code.md)
  - [Cheat sheets](src/links/rust_cheatsheets.md)
  - [Blogs](src/links/blogs.md)
  - [Books](src/links/books.md)
  - [Companies](src/links/companies.md)

---

[Crates (src/alphabetic)](src/crates.md)
[Crates (src/by category)](src/_categories.md) generate automatically
[Thanks](src/thanks.md)
