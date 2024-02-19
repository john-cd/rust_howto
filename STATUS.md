# STATUS

[Introduction](src/intro.md) polish the intro
[About](src/about.md) rewrite
[Contributing](src/contributing.md) polish
[Index](src/index.md) rethink

- [Language](src/lang.md)
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

- [Standard library](src/standard_library.md)
  - [Option](src/standard_library/option.md) finish Option page map, unwrap_or
  - [Vectors](src/standard_library/vectors.md)
  - [HashMap](src/standard_library/hashmaps.md)
  - [Strings](src/standard_library/strings.md)
  - [Smart pointers](src/standard_library/smart_pointers.md)

- [Key crates](src/key_crates.md)

- [Development tools](src/tools.md)
  - [Cargo](src/tools/cargo.md) finish Cargo Plugins / cargo.md
  - [Package layout](src/tools/cargo/package_layout.md)
  - [Installation](src/tools/installing.md)
    - [Rustup](src/tools/installing/rustup.md)
  - [Code editing and formatting](src/tools/editing.md) add latest JetBrains tooling
  - [Compiling](src/tools/compiling.md)
    - [Faster linking](src/tools/compiling/faster_linking.md)  finish faster linking
    - [Cross-compilation](src/tools/compiling/cross_compilation.md)
  - [Build-time tooling](src/tools/build_tools.md)
  - [Testing](src/tools/testing.md)
  - [Performance profiling](src/tools/performance.md)
  - [Miri](src/tools/other/miri.md)
  - [Documentation](src/tools/documentation.md)
    - [mdBook](src/tools/documenting/mdbook.md)
  - [Other](src/tools/other.md)
    - [Other tools](src/tools/other/other_tools.md)
    - [Crate registries](src/tools/other/crates.md)
    - [Just](src/tools/other/just.md)
  - [Versioning](src/tools/versioning.md)

---

- [Algorithms](src/algorithms.md)
  - [Generate Random Values](src/algorithms/randomness.md)
  - [Sort a Vector](src/algorithms/sorting.md)

- [Automatic trait derivation TODO](src/concerns/derive.md)

- [Command-line applications](src/cli.md)
  - [Argument Parsing](src/cli/arguments.md)
  - [ANSI Terminal](src/cli/ansi_terminal.md)

- [Compression](src/compression.md)
  - [Working with Tarballs](src/compression/tar.md)

- [Concurrency](src/concurrency.md)
  - [Multi-threading](src/concurrency/multithreading.md)
  - [Explicit threads](src/concurrency/threads.md)
  - [Data parallelism](src/concurrency/parallel.md)
  - [Message passing](src/concurrency/message_passing.md)
  - [Shared-state concurrency](src/concurrency/shared_state.md)
  - [Concurrent data structures](src/concurrency/shared_state/concurrent_data_structures.md)

- [Concurrency - Async](src/concurrency/async.md)
  - [Async and traits](src/async/async_traits.md)
  - [Tokio async runtime](src/async/tokio.md) add more to tokio.md
  - [Async channels](src/async/async_channels.md) add more to async_channels
  - [Streams](src/async/streams.md)
  - [Futures crate](src/async/futures.md)
  - [Mixing async and blocking code](src/async/async_and_blocking.md)

- [Configuration TODO](src/concerns/configuration.md)

- [Cross-cutting concerns TODO](src/cross_cutting_concerns.md)

- [Cryptography](src/cryptography.md)
  - [Hashing](src/cryptography/hashing.md)
  - [Encryption](src/cryptography/encryption.md)

- [Data structures](src/data_structures.md)
  - [Bitfield](src/data_structures/bitfield.md)

- [Databases](src/databases.md)
  - [SQLite](src/databases/sqlite.md)
  - [Postgres](src/databases/postgres.md)
  - [Query builders and ORMS](src/databases/query_builders_orms.md)

- [Date and time](src/datetime.md)
  - [Duration and calculation](src/datetime/duration.md)
  - [Parsing and displaying](src/datetime/parse.md)

- [Encoding and Serialization](src/encoding.md)
  - [Character sets](src/encoding/strings.md)
  - [CSV processing](src/encoding/csv.md)
  - [Structured data](src/encoding/complex.md)
  - [Serde](src/encoding/serde.md)

- [Error handling](src/errors.md) errors: color-eyre, eyre
  - [Handle error variants](src/errors/handle.md)
  - [Error handling](src/errors/error_handling.md)
  - [Error customization](src/errors/error_customization.md)

- [File system](src/file.md)
  - [Read & write](src/file/read-write.md)
  - [Directory traversal](src/file/dir.md)

- [Hardware support](src/hardware.md)
  - [Processor](src/hardware/processor.md)

- [Logging](src/logging.md)
  - [Tracing](src/logging/tracing.md)
  - [Log messages](src/logging/log.md)
  - [Configure logging](src/logging/config_log.md)

- [Mathematics](src/mathematics.md)
  - [Linear algebra](src/mathematics/linear_algebra.md)
  - [Trigonometry](src/mathematics/trigonometry.md)
  - [Complex numbers](src/mathematics/complex_numbers.md)
  - [Statistics](src/mathematics/statistics.md)
  - [Miscellaneous](src/mathematics/miscellaneous.md)

- [Memory management](src/mem.md)
  - [Global static](src/mem/global_static.md)
  - [Lazy initialization TODO](src/mem/lazy_initialization.md)

- [Network](src/net.md)
  - [Server](src/net/server.md)

- [Operating system](src/os.md)
  - [External commands](src/os/external.md)

- [Text processing](src/text.md)
  - [Regular expressions](src/text/regex.md)
    - [Longer regex example](src/text/regex2.md)
  - [String parsing](src/text/string_parsing.md)

- [Web programming](src/web.md)
  - [Extracting links](src/web/scraping.md)
  - [URL](src/web/url.md)
  - [Media types](src/web/mime.md)
  - [Web frameworks](src/web/server.md)
    - [Axum](src/web/server/axum.md)
    - [Actix](src/web/server/actix.md)
    - [Other web frameworks](src/web/server/other_frameworks.md)
    - [Middleware](src/web/server/middleware.md)
    - [CORS](src/web/server/cors.md)
  - [Static website generators](src/web/server/static_website_generators.md)
  - [Web Clients](src/clients.md)
    - [Making Requests](src/web/clients/requests.md)
    - [Calling a Web API](src/web/clients/apis.md)
    - [Downloads](src/web/clients/download.md)
    - [Web Authentication](src/web/clients/authentication.md)

---

- [Other domains](src/domains.md)
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

- [Links](src/links.md)
  - [Example code](src/links/example_code.md)
  - [Cheat sheets](src/links/rust_cheatsheets.md)
  - [Blogs](src/links/blogs.md)
  - [Books](src/links/books.md)
  - [Companies](src/links/companies.md)

---

[Crates (src/alphabetic)](src/misc/crates.md)
[Crates (src/by category)](src/misc/categories.md) generate automatically
[Thanks](src/misc/thanks.md)
