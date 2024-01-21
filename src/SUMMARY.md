# Rust How-to

[Introduction](intro.md)
[About](about.md)

- [Rust installation and first steps](install.md)

- [Language](lang/rust_language.md)
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

- [Standard library](standard_library.md)
  - [Option](standard_library/option.md)
  - [Vectors](standard_library/vectors.md)
  - [HashMap](standard_library/hashmaps.md)
  - [Strings](standard_library/strings.md)
  - [Smart pointers](standard_library/smart_pointers.md)

- [Key crates](key/key_crates.md)

- [Algorithms](algorithms.md)
  - [Generate Random Values](algorithms/randomness.md)
  - [Sort a Vector](algorithms/sorting.md)

- [Automatic derivation](concerns/derive.md)

- [Command Line](cli2.md)
  - [CLI](cli.md)
  - [Argument Parsing](cli/arguments.md)
  - [ANSI Terminal](cli/ansi_terminal.md)

- [Compression](compression.md)
  - [Working with Tarballs](compression/tar.md)

- [Concurrency](concurrency.md)
- [Concurrency](concurrency2.md)
  - [Multi-threading](concurrency/multithreading.md)
  - [Explicit Threads](concurrency/threads.md)
  - [Data Parallelism](concurrency/parallel.md)
  - [Message passing](concurrency/message_passing.md)
  - [Shared-state concurrency](concurrency/shared_state.md)
  - [Concurrent data structures](concurrency/shared_state/concurrent_data_structures.md)
  - [Async](concurrency/async.md)
    - [Async and traits](concurrency/async/async_traits.md)
    - [Tokio async runtime](concurrency/async/tokio.md)
    - [Async channels](concurrency/async/async_channels.md)
    - [Streams](concurrency/async/streams.md)
    - [Futures crate](concurrency/async/futures.md)
    - [Mixing Async and Blocking](concurrency/async/async_and_blocking.md)

- [Configuration](concerns/configuration.md)

- [Cross-cutting concerns](concerns/cross_cutting_concerns.md)

- [Cryptography](cryptography.md)
  - [Hashing](cryptography/hashing.md)
  - [Encryption](cryptography/encryption.md)

- [Data Structures](data_structures.md)
  - [Bitfield](data_structures/bitfield.md)

- [Databases](databases.md)
- [Database](database.md)
  - [SQLite](database/sqlite.md)
  - [Postgres](database/postgres.md)

- [Date and Time](datetime2.md)
  - [Duration and Calculation](datetime/duration.md)
  - [Parsing and Displaying](datetime/parse.md)

- [Encoding](encoding.md)
  - [Character Sets](encoding/strings.md)
  - [CSV processing](encoding/csv.md)
  - [Structured Data](encoding/complex.md)
  - [Serialization (Serde)](key/serialization.md)

- [Error Handling](errors.md)
  - [Handle Error Variants](errors/handle.md)
  - [Error handling](errors/error_handling.md)
  - [Error customization](errors/error_customization.md)

- [File System](file.md)
  - [Read & Write](file/read-write.md)
  - [Directory Traversal](file/dir.md)

- [Hardware Support](hardware.md)
  - [Processor](hardware/processor.md)

- [Lazy initialization](concerns/lazy_initialization.md)

- [Logging / tracing](logging/logging.md)
- [Logging](logging.md)
  - [Log Messages](logging/log.md)
  - [Configure Logging](logging/config_log.md)

- [Mathematics](mathematics.md)
  - [Linear Algebra](mathematics/linear_algebra.md)
  - [Trigonometry](mathematics/trigonometry.md)
  - [Complex Numbers](mathematics/complex_numbers.md)
  - [Statistics](mathematics/statistics.md)
  - [Miscellaneous](mathematics/miscellaneous.md)

- [Memory Management](mem.md)
  - [Global Static](mem/global_static.md)

- [Network](net.md)
  - [Server](net/server.md)

- [Operating System](os.md)
  - [External Command](os/external.md)

- [Science](science.md)

- [Text](text.md)
  - [Regex](text/regex.md)

- [Text Processing](text.md)
  - [Regular Expressions](text/regex2.md)
  - [String Parsing](text/string_parsing.md)

- [Time and date](datetime.md)

- [Web Programming](web2.md)
- [Web](web.md)
  - [Extracting Links](web/scraping.md)
  - [URL](web/url.md)
  - [Media Types](web/mime.md)
  - [Server](web/server.md)
    - [Axum](web/server/axum.md)
    - [Actix](web/server/actix.md)
    - [Other web frameworks](web/server/other_frameworks.md)
    - [Static website generators](web/server/static_website_generators.md)
    - [Middleware](web/server/middleware.md)
    - [CORS](web/server/cors.md)

- [Web Clients](clients.md)
  - [HTTP clients](web/http_clients.md)
  - [Making Requests](web/clients/requests.md)
  - [Calling a Web API](web/clients/apis.md)
  - [Downloads](web/clients/download.md)
  - [Web Authentication](web/clients/authentication.md)

- [Other domains](domains.md)
  - [WASM](domains/wasm.md)
  - [GUI](domains/gui.md)
  - [Cross-platform](domains/cross_platform.md)
  - [Cloud](domains/cloud.md)
  - [Data](domains/data.md)
  - [ML](domains/ml.md)
  - [Games](domains/games.md)
  - [GPU programming](domains/gpu.md)
  - [Robotics](domains/robotics.md)
  - [Windows](domains/windows.md)

---

- [Development Tools](development_tools.md)
- [Tools](tools.md)
  - [Cargo](tools/cargo.md)
    - [Package layout](tools/cargo/package_layout.md)
  - [Installing](tools/installing.md)
    - [Rustup](tools/installing/rustup.md)
  - [Editing](tools/editing.md)
  - [Compiling](tools/compiling.md)
    - [Faster linking](tools/compiling/faster_linking.md)
    - [Cross-compilation](tools/compiling/cross_compilation.md)
  - [Build Time Tooling](tools/build_tools.md)
  - [Testing](tools/testing.md)
  - [Performance](tools/performance.md)
  - [Documentation](tools/documentation.md)
    - [mdBook](tools/documenting/mdbook.md)
  - [Other](tools/other.md)
    - [Other tools](tools/other/other_tools.md)
    - [Crate registries](tools/other/crates.md)
    - [Miri](tools/other/miri.md)
    - [Just](tools/other/just.md)
  - [Versioning](tools/versioning.md)

- [Links](links.md)
  - [Example code](links/example_code.md)
  - [Cheat sheets](links/rust_cheatsheets.md)
  - [Blogs](links/blogs.md)
  - [Books](links/books.md)
  - [Companies](links/companies.md)

---

[Categories](misc/categories.md)
[Crates](misc/crates.md)
[Thanks](misc/thanks.md)
