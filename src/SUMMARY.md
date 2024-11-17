# Rust How-to

[Introduction](index.md)
[About](_about.md)
[Index of examples](examples_index.md)

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
  - [Result](standard_library/result.md)
  - [Vectors](standard_library/vectors.md)
  - [HashMap](standard_library/hashmaps.md)
  - [Strings](standard_library/strings.md)
  - [Copy-on-write](standard_library/cow.md)
  - [Smart pointers](standard_library/smart_pointers.md)
  - [AsRef](standard_library/asref.md)
  - [Automatic trait derivation](standard_library/derive.md)

- [Crates](crates/crates.md)
  - [Crates (alphabetic)](crates/crates_alphabetical.md)
  - [Crates (by category)](crates/crates_by_category.md)

---

- [Accessibility](categories/accessibility/_index.md)
  - [Enable screen readers](categories/accessibility/screen_readers.md)

- [Algorithms](categories/algorithms/index.md)
  - [Generate random values](categories/algorithms/randomness.md)
  - [Sort a vector](categories/algorithms/sorting.md)

- [API bindings](categories/api-bindings/_index.md)
  - [Interop with Python](categories/api-bindings/python_interop.md)

- [Architecture](other/architecture/architecture.md)

- [Audio](categories/multimedia_audio/_index.md)
  - [Audio](categories/multimedia_audio/audio.md)

- [Asynchronous](categories/asynchronous/index.md)
  - [Async](categories/asynchronous/async.md)
  - [Async and traits](categories/asynchronous/async_traits.md)
  - [Tokio async runtime](categories/asynchronous/tokio.md)
  - [Async channels](categories/asynchronous/async_channels.md)
  - [Streams](categories/asynchronous/streams.md)
  - [Futures crate](categories/asynchronous/futures.md)
  - [Mixing async and blocking code](categories/asynchronous/async_and_blocking.md)

- [Authentication](categories/authentication/index.md)
  - [Basic Authentication](categories/authentication/basic_authentication.md)

- [Caching](categories/caching/_index.md)
  - [LRU caching](categories/caching/lru.md)

- [Cloud](other/cloud/index.md)
  - [Rust on AWS](other/cloud/aws.md)
  - [Serverless computing](other/cloud/serverless.md)
  - [Rust-native cloud development](other/cloud/rust_native_cloud_development.md)

- [Command-line interface](categories/command-line-interface/index.md)
  - [Argument parsing](categories/command-line-interface/arguments.md)
  - [ANSI terminal](categories/command-line-interface/ansi_terminal.md)
  - [Terminal user interfaces](categories/command-line-interface/tui.md)
  - [User interaction](categories/command-line-interface/user_interaction.md)

- [Command-line utilities](categories/command-line-utilities/index.md)
  - [Filesystem](categories/command-line-utilities/filesystem.md)
  - [Networking](categories/command-line-utilities/networking.md)
  - [Shells](categories/command-line-utilities/shells.md)

- [Compilers](categories/compilers/_index.md)
  - [Compilation duration reduction](categories/compilers/reduce_compilation_duration.md)
  - [Faster linking](categories/compilers/faster_linking.md)
  - [Cross-compilation](categories/compilers/cross_compilation.md)

- [Compression](categories/compression/index.md)
  - [Working with tarballs](categories/compression/tar.md)

- [Computer vision](categories/computer-vision/_index.md)
  - [Open CV](categories/computer-vision/opencv.md)

- [Concurrency](categories/concurrency/index.md)
  - [Send and Sync](categories/concurrency/send.md)
  - [Multi-threading](categories/concurrency/multithreading.md)
  - [Explicit threads](categories/concurrency/threads.md)
  - [Data parallelism](categories/concurrency/parallel.md)
  - [Message passing](categories/concurrency/message_passing.md)
  - [Shared-state concurrency](categories/concurrency/shared_state/shared_state.md)
  - [Concurrent data structures](categories/concurrency/shared_state/concurrent_data_structures.md)
  - [Actors](categories/concurrency/_actors.md)

- [Configuration](categories/config/index.md)
  - [Environment variables](categories/config/environment_variables.md)
  - [Configuration management](categories/config/configuration.md)

- [Containers](other/containers/containers.md)

- [Cross-platform Development](other/cross_platform/index.md)
  - [Crux](other/cross_platform/crux.md)

- [Cryptography](categories/cryptography/index.md)
  - [Hashing](categories/cryptography/hashing.md)
  - [Password hashing](categories/cryptography/password_hashing.md)
  - [Encryption](categories/cryptography/encryption.md)

- [Data processing](other/data_processing/index.md)
  - [CSV](other/data_processing/csv.md)
  - [Dataframes](other/data_processing/dataframes.md)
  - [Data visualization](other/data_processing/data_visualization.md)
  - [Data engineering](other/data_processing/data_engineering.md)

- [Data structures](categories/data-structures/index.md)
  - [Bitfields](categories/data-structures/bitfield.md)
  - [Maps](categories/data-structures/maps.md)
  - [Stack-allocated arrays](categories/data-structures/stack_allocated_arrays.md)
  - [UUIDs](categories/data-structures/uuid.md)

- [Database access](categories/database/index.md)
  - [SQLite](categories/database/sqlite.md)
  - [Postgres](categories/database/postgres.md)
  - [Connection pools](categories/database/connection_pool.md)
  - [Query builders and ORMs](categories/database/query_builders_orms.md)
  - [NoSQL](categories/database/nosql.md)
  - [Search](categories/database/search.md)

- [Database implementations](categories/database-implementations/_index.md)
  - [Databases](categories/database-implementations/databases.md)
  - [Search](categories/database-implementations/search.md)

- [Date and time](categories/date-and-time/index.md)
  - [Duration and calculation](categories/date-and-time/duration.md)
  - [Parsing and displaying](categories/date-and-time/parse.md)
  - [`time` crate](categories/date-and-time/time.md)

- [Development tools](categories/development-tools/index.md)
  - [Cargo](categories/development-tools/cargo/cargo.md)
    - [Package layout](categories/development-tools/cargo/package_layout.md)
    - [Crate registries](categories/development-tools/cargo/crate_registries.md)
  - [Rust and binary installation](categories/development-tools/installation/install.md)
    - [Rustup](categories/development-tools/installation/rustup.md)
  - [Code editing](categories/text-editors/index.md)
  - [Code formatting](categories/development-tools/formatting/formatting.md)
  - [Documentation](categories/development-tools/documentation/documentation.md)
    - [`mdBook`](categories/development-tools/documentation/mdbook.md)
    - [Badges](categories/development-tools/documentation/badges.md)
  - [Versioning](categories/development-tools/versioning/versioning.md)
  - [Other](categories/development-tools/other/other.md)
    - [`miri`](categories/development-tools/other/miri.md)
    - [`just`](categories/development-tools/other/just.md)

- [Development tools - build-time tooling](categories/development-tools_build-utils/index.md)
  - [Build-time tools](categories/development-tools_build-utils/build_utils.md)

- [Development tools - cargo plugins](categories/development-tools_cargo-plugins/index.md)
  - [Writing code](categories/development-tools_cargo-plugins/writing.md)
  - [Code formatting, linting](categories/development-tools_cargo-plugins/code_formatting_linting.md)
  - [Building](categories/development-tools_cargo-plugins/building.md)
  - [Cross-compiling](categories/development-tools_cargo-plugins/cross_compiling.md)
  - [Watching for changes](categories/development-tools_cargo-plugins/watching_for_changes.md)
  - [Improving performance](categories/development-tools_cargo-plugins/performance.md)
  - [Auditing](categories/development-tools_cargo-plugins/auditing.md)
  - [Maintaining](categories/development-tools_cargo-plugins/maintaining.md)

- [Development tools - debugging, logging](categories/development-tools_debugging/index.md)
  - [Tracing](categories/development-tools_debugging/tracing.md)
  - [Log messages](categories/development-tools_debugging/log.md)
  - [Logging configuration](categories/development-tools_debugging/config_log.md)
  - [Alternatives](categories/development-tools_debugging/alternatives.md)
  - [Diagnostic functions](categories/development-tools_debugging/diagnostic_functions.md)

- [Development tools - FFI](categories/development-tools_ffi/_index.md)
  - [Generating FFI bindings](categories/development-tools_ffi/generate_ffi_bindings.md)

- [Development tools - procedural macro helpers](categories/development-tools_procedural-macro-helpers/_index.md)
  - [Crates for macro development](categories/development-tools_procedural-macro-helpers/write_proc_macros.md)
  - [Tools for macro development](categories/development-tools_procedural-macro-helpers/tools.md)
  - [Compiling macros ahead of time](categories/development-tools_procedural-macro-helpers/compile_macros.md)

- [Development tools - performance, profiling](categories/development-tools_profiling/_index.md)
  - [Benchmarking](categories/development-tools_profiling/benchmarking.md)
  - [Low-level profiling tools](categories/development-tools_profiling/assembly.md)
  - [Memory usage analysis](categories/development-tools_profiling/memory.md)

- [Development tools - testing](categories/development-tools_testing/_index.md)
  - [Testing](categories/development-tools_testing/testing.md)
  - [Fuzzing](categories/development-tools_testing/fuzzing.md)

- [DevOps](other/devops/index.md)
  - [DevOps](other/devops/devops.md)
  - [Dependency Management](other/devops/dependency_management.md)
  - [Git Hooks](other/devops/git_hooks.md)
  - [Github Actions](other/devops/github_actions.md)
  - [Release Automation](other/devops/release_automation.md)
  - [CD/CI](other/devops/cd_ci.md)

- [Email](categories/email/_index.md)
  - [Sending emails](categories/email/send_emails.md)

- [Embedded development](categories/embedded/_index.md)
  - [`embassy`](categories/embedded/embassy.md)

- [Emulators](categories/emulators/_index.md)
  - [Emulators](categories/emulators/emulators.md)

- [Encoding and serialization](categories/encoding/index.md)
  - [Character sets](categories/encoding/strings.md)
  - [CSV processing](categories/encoding/csv.md)
  - [Structured data](categories/encoding/complex.md)
  - [`serde`](categories/encoding/serde.md)
  - [Typecasting](categories/encoding/typecasts.md)

- [External FFI bindings](categories/external-ffi-bindings/_index.md)
  - [External FFI bindings](categories/external-ffi-bindings/external_ffi_bindings.md)

- [Error handling](categories/rust-patterns/error_handling.md)
  - [Error customization](categories/rust-patterns/error_customization.md)

- [File system](categories/filesystem/index.md)
  - [Read & write](categories/filesystem/read-write.md)
  - [Current working directory](categories/filesystem/cwd.md)
  - [Directory traversal](categories/filesystem/dir.md)
  - [Ignore files](categories/filesystem/ignore.md)
  - [File watching](categories/filesystem/file_watching.md)
  - [Temporary files and directories](categories/filesystem/tempfile.md)
  - [User directories](categories/filesystem/user_directories.md)

- [Finance](categories/finance/_index.md)
  - [Quant](categories/finance/quant.md)

- [Game development](categories/game-development/_index.md)
  - [Game development](categories/game-development/game_development.md)

- [Game engines](categories/game-engines/_index.md)
  - [Game engines](categories/game-engines/game_engines.md)

- [Graphics](categories/graphics/_index.md)
  - [WebGPU](categories/graphics/webgpu.md)

- [GPU programming](other/gpu/index.md)
  - [GPU](other/gpu/gpu.md)

- [GUI](categories/gui/_index.md)
  - [2D Renderers](categories/gui/2d_renderers.md)
  - [Clipboard](categories/gui/clipboard.md)
  - [File Dialogs](categories/gui/file_dialogs.md)
  - [GTK](categories/gui/gtk.md)
  - [Immediate-mode GUI](categories/gui/immediate_mode_gui.md)
  - [Other GUI](categories/gui/other_gui.md)
  - [Retained-mode GUI](categories/gui/retained_mode_gui.md)
  - [Text layout](categories/gui/text_layout.md)
  - [UI layout](categories/gui/ui_layout.md)
  - [Web-based GUI](categories/gui/web_based_gui.md)
  - [Window creation](categories/gui/window_creation.md)

- [Hardware support](categories/hardware-support/index.md)
  - [Processor](categories/hardware-support/processor.md)

- [Images](categories/multimedia_images/_index.md)
  - [Images](categories/multimedia_images/images.md)

- [Internationalization](categories/internationalization/_index.md)
  - [Internationalization](categories/internationalization/internationalization.md)

- [Localization](categories/localization/_index.md)
  - [Localization](categories/localization/localization.md)

- [Machine learning](categories/science/machine_learning.md)

- [Mathematics](categories/mathematics/index.md)
  - [Linear algebra](categories/mathematics/linear_algebra.md)
  - [Trigonometry](categories/mathematics/trigonometry.md)
  - [Complex numbers](categories/mathematics/complex_numbers.md)
  - [Statistics](categories/mathematics/statistics.md)
  - [Additional numeric types](categories/mathematics/additional_numeric_types.md)

- [Memory management](categories/memory-management/index.md)
  - [Global static](categories/memory-management/global_static.md)
  - [Lazy initialization](categories/memory-management/lazy_initialization.md)

- [Multimedia](categories/multimedia/_index.md)
  - [Multimedia](categories/multimedia/multimedia.md)

- [Multimedia encoding](categories/multimedia_encoding/_index.md)
  - [Multimedia encoding](categories/multimedia_encoding/encoding.md)

- [Network](categories/network-programming/_index.md)
  - [Server](categories/network-programming/server.md)
  - [Reverse_proxy](categories/network-programming/reverse_proxy.md)

- [No `alloc`](categories/no-std_no-alloc/_index.md)
  - [No `alloc`](categories/no-std_no-alloc/no_alloc.md)

- [No `std`](categories/no-std/_index.md)
  - [No `std`](categories/no-std/no_std.md)

- [Operating systems](categories/os/index.md)
  - [External commands](categories/os/external.md)
  - [Low-level system calls](categories/os/low_level_system_calls.md)
  - [Rust OSes](categories/os/rust_os.md)

- [Operating systems - FreeBSD](categories/os_freebsd-apis/_index.md)
  - [FreeBSD](categories/os_freebsd-apis/freebsd.md)

- [Operating systems - Linux](categories/os_linux-apis/_index.md)
  - [Linux](categories/os_linux-apis/linux.md)

- [Operating systems - macOS](categories/os_macos-apis/_index.md)
  - [macOS](categories/os_macos-apis/macos.md)

- [Operating systems - Unix](categories/os_unix-apis/_index.md)
  - [Unix](categories/os_unix-apis/unix.md)

- [Operating systems - Windows](categories/os_windows-apis/_index.md)
  - [Windows](categories/os_windows-apis/windows.md)

- [Parser implementations](categories/parser-implementations/_index.md)
  - [Parser implementations](categories/parser-implementations/parser_implementations.md)

- [Parsing](categories/parsing/_index.md)
  - [Parsing](categories/parsing/parsing.md)

- [Rendering](categories/rendering/_index.md)
  - [Rendering](categories/rendering/rendering.md)

- [Rendering - data formats](categories/rendering_data-formats/_index.md)
  - [Data formats](categories/rendering_data-formats/data_formats.md)

- [Rendering engines](categories/rendering_engine/_index.md)
  - [Rendering engines](categories/rendering_engine/rendering_engines.md)

- [Rendering - graphics APIs](categories/rendering_graphics-api/_index.md)
  - [Graphics APIs](categories/rendering_graphics-api/graphics_apis.md)

- [Robotics](categories/science_robotics/_index.md)
  - [Robotics](categories/science_robotics/robotics.md)
  - [Useful robotics tools and libraries](categories/science_robotics/useful_robotics_tools_and_libs.md)

- [Rust patterns](categories/rust-patterns/index.md)
  - [Design patterns](categories/rust-patterns/design_patterns.md)
  - [Functional programming](categories/rust-patterns/functional_programming.md)
  - [Rust idioms](categories/rust-patterns/rust_idioms.md)

- [Science](categories/science/_index.md)

- [Science - Geo](categories/science_geo/_index.md)
  - [Geo](categories/science_geo/geo.md)

- [Science - Neuro](categories/science_neuroscience/_index.md)
  - [Neuro](categories/science_neuroscience/neuro.md)

- [Scripting](other/scripting/index.md)
  - [Rhai](other/scripting/rhai.md)

- [Simulation](categories/simulation/_index.md)
  - [Simulation](categories/simulation/simulation.md)

- [Template engines](categories/template-engine/index.md)
  - [Tera](categories/template-engine/tera.md)
  - [TinyTemplate](categories/template-engine/tinytemplate.md)

- [Text editors](categories/text-editors/index.md)
  - [IDEs](categories/text-editors/ides.md)

- [Text processing](categories/text-processing/index.md)
  - [Regular expressions](categories/text-processing/regex.md)
    - [Longer regex example](categories/text-processing/regex2.md)
  - [String parsing](categories/text-processing/string_parsing.md)
  - [String concatenation](categories/text-processing/string_concat.md)

- [Value formatting](categories/value-formatting/_index.md)
  - [Value formatting](categories/value-formatting/value-formatting.md)

- [Video](categories/multimedia_video/_index.md)
  - [Video](categories/multimedia_video/video.md)

- [Virtualization](categories/virtualization/_index.md)
  - [Virtualization](categories/virtualization/virtualization.md)

- [Visualization](categories/visualization/_index.md)
  - [Visualization](categories/visualization/visualization.md)

- [Web assembly](categories/wasm/_index.md)
  - [Yew](categories/wasm/yew.md)
  - [Others](categories/wasm/others.md)

- [Web programming](categories/web-programming/index.md)
  - [Extracting links](categories/web-programming/scraping.md)
  - [URL](categories/web-programming/url.md)
  - [Media types](categories/web-programming/mime.md)

- [Web programming - HTTP clients](categories/web-programming_http-client/index.md)
  - [HTTP clients](categories/web-programming_http-client/http_clients.md)
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
  - [Batteries-included frameworks](categories/web-programming_http-server/batteries_included.md)
  - [GraphQL](categories/web-programming_http-server/graphql.md)
  - [gRPC](categories/web-programming_http-server/grpc.md)
  - [`hyper`](categories/web-programming_http-server/hyper.md)

- [Web programming - websocket](categories/web-programming_websocket/_index.md)
  - [Websocket](categories/web-programming_websocket/websocket.md)

- [Written in Rust](other/written-in-rust/index.md)
  - [Development tools](other/written-in-rust/development_tools.md)
  - [Python tools](other/written-in-rust/python_tools.md)
  - [Others](other/written-in-rust/others.md)

---

- [Links](links/index.md)
  - [Links](links/links.md)
  - [Example code](links/example_code.md)
  - [Cheat sheets](links/rust_cheatsheets.md)
  - [Blogs](links/blogs_podcasts_meetups.md)
  - [Books](links/books.md)
  - [Companies](links/companies.md)
  - [Learning](links/learning.md)

[Index](word_index.md)

[Thanks](thanks.md)

{{#include refs.incl.md}}
