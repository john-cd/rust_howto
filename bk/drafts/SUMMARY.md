
[Index of Examples](indices/examples_index.md)
[Crates (Alphabetic)](indices/crates_alphabetical.md)
[Crates (by Category)](indices/crates_by_category.md)

# Language, Standard Library, and Code Organization

- [Language](language/index.md)
  - [Rust Installation and First Steps](language/rust_install.md)
  - [Main Function](language/entrypoint.md)
  - [Data Types](language/data_types.md)
  - [Variables](language/variables.md)
  - [Constants and Statics](language/constants_and_statics.md)
  - [Ownership and Borrowing](language/ownership_borrowing.md)
  - [Slices](language/slices.md)
  - [Functions](language/functions.md)
  - [Control Flow](language/control_flow.md)
  - [Structs](language/structs.md)
  - [Enums](language/enums.md)
  - [Traits](language/traits.md)
  - [Trait Objects](language/trait_objects.md)
  - [Attributes](language/attributes.md)
  - [Generics](language/generics.md)
  - [Lifetimes](language/lifetimes.md)
  - [Pattern Matching, `if / while let`](language/match.md)
  - [Closures](language/closures.md)
  - [Iterators](language/iterators.md)
  - [Macros](language/macros.md)

  - [Strings](standard_library/strings.md)
  - [Smart Pointers](standard_library/smart_pointers.md)
  - [HashMap](standard_library/hashmap.md)
  - [Vector](standard_library/vector.md)

# Categories

- [Algorithms](categories/algorithms/index.md)
  - [Generate Random Values](categories/algorithms/randomness.md)
  - [Sort a Vector](categories/algorithms/sorting.md)

- [API bindings](categories/categories/api-bindings/index.md)

- [Architecture](categories/other/architecture/index.md)
  - [Architectural Patterns](categories/other/architecture/architectural_patterns.md)
  - [Common Architectures](categories/other/architecture/common_architectures.md)
  - [Software Architecture Process](categories/other/architecture/software_architecture_process.md)

- [Asynchronous](categories/asynchronous/index.md)
  - [Async](categories/asynchronous/async.md)
  - [Async and Traits](categories/asynchronous/async_traits.md)
  - [Tokio Async Runtime](categories/asynchronous/tokio.md)
  - [Async Channels](categories/asynchronous/async_channels.md)
  - [Streams](categories/asynchronous/streams.md)
  - [Futures crate](categories/asynchronous/futures.md)
  - [Mixing Async and Blocking Code](categories/asynchronous/async_and_blocking.md)
  - [Utilities](categories/asynchronous/async_utilities.md)

- [Authentication](categories/authentication/index.md)
  - [Basic Authentication](categories/authentication/basic_authentication.md)

- [Caching](categories/caching/index.md)
  - [In-memory Caching](categories/caching/in_memory_cache.md)

- [Command-line Interface](categories/command-line-interface/index.md)
  - [Argument Parsing](categories/command-line-interface/argument_parsing.md)
  - [ANSI Terminal](categories/command-line-interface/ansi_terminal.md)
  - [Terminal User Interfaces](categories/command-line-interface/tui.md)
  - [User Interaction](categories/command-line-interface/user_interaction.md)

- [Command-line Utilities](categories/command-line-utilities/index.md)
  - [Filesystem](categories/command-line-utilities/filesystem_cli.md)
  - [Networking](categories/command-line-utilities/networking_cli.md)
  - [Shells](categories/command-line-utilities/shells.md)

- [Compression](categories/compression/index.md)
  - [Working with Tarballs](categories/compression/tar.md)
  - [Flate2](categories/compression/compression.md)

- [Concurrency](categories/concurrency/index.md)
  - [Explicit Threads](categories/concurrency/explicit_threads.md)
  - [Threadpools](categories/concurrency/threadpool.md)
  - [Multithreading with `crossbeam`](categories/concurrency/crossbeam.md)
  - [Data Parallelism](categories/concurrency/data_parallelism.md)
  - [Message Passing and Channels](categories/concurrency/message_passing.md)
  - [Shared-state Concurrency](categories/concurrency/shared_state.md)
  - [Atomics](categories/concurrency/atomics.md)
  - [Concurrent Data Structures](categories/concurrency/concurrent_data_structures.md)
  - [Send and Sync](categories/concurrency/send_sync.md)
  - [Actors](categories/concurrency/_actors.md)

- [Configuration](categories/config/index.md)
  - [Environment Variables](categories/config/environment_variables.md)
  - [Configuration Management](categories/config/configuration.md)

- [Cryptography](categories/cryptography/index.md)
  - [Hashing](categories/cryptography/hashing.md)
  - [Password Hashing](categories/cryptography/password_hashing.md)
  - [Encryption](categories/cryptography/encryption.md)
  - [AEAD](categories/cryptography/aead.md)
  - [Signatures](categories/cryptography/signature.md)
  - [HMAC](categories/cryptography/hmac.md)
  - [Certificates](categories/cryptography/certificates.md)
  - [TLS](categories/cryptography/tls.md)
  - [Cryptographic Utilities](categories/cryptography/cryptography_utilities.md)

- [Data Structures](categories/data-structures/index.md)
  - [Maps](categories/data-structures/maps.md)
  - [B-Trees](categories/data-structures/btrees.md)
  - [Stacks and Queues](categories/data-structures/stack_and_queue.md)
  - [Binary Heaps](categories/data-structures/binaryheap.md)
  - [Linked Lists](categories/data-structures/linkedlist.md)
  - [Bitfields](categories/data-structures/bitfield.md)
  - [Graph](categories/data-structures/graph.md)
  - [Heapless Data Structures](categories/data-structures/heapless.md)
  - [Stack-allocated Arrays](categories/data-structures/stack_allocated_arrays.md)
  - [UUIDs](categories/data-structures/uuid.md)

- [Database Access](categories/database/index.md)
  - [SQLite](categories/database/sqlite.md)
  - [Postgres](categories/database/postgres.md)
  - [MSSQL](categories/database/mssql.md)
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
  - [Duration and Calculation](categories/date-and-time/duration.md)
  - [Parsing and Displaying](categories/date-and-time/parse.md)
  - [`time` Crate](categories/date-and-time/time_crate.md)

- [Development Tools](categories/development-tools/index.md)
  - [Cargo](categories/development-tools/cargo/cargo.md)
    - [Package Layout](categories/development-tools/cargo/package_layout.md)
    - [Crate Registries](categories/development-tools/cargo/crate_registries.md)
  - [Rust and Binary Installation](categories/development-tools/installation/install.md)
    - [Rustup](categories/development-tools/installation/rustup.md)
  - [Code Editing](categories/text-editors/index.md)
  - [Code Formatting](categories/development-tools/formatting/formatting.md)
  - [Documentation](categories/development-tools/documentation/documentation.md)
    - [`mdBook`](categories/development-tools/documentation/mdbook.md)
    - [Badges](categories/development-tools/documentation/badges.md)
  - [Versioning](categories/development-tools/versioning/versioning.md)
  - [Other](categories/development-tools/other/other.md)
    - [Code Building](categories/development-tools/other/code_build.md)
    - [Cross-compilation](categories/development-tools/cross-compilation/cross_compilation.md)
    - [Code Verification](categories/development-tools/other/code_verification.md)
    - [`miri`](categories/development-tools/other/miri.md)
    - [Reduce Compilation Duration](categories/development-tools/compilation/reduce_compilation_duration.md)
    - [Faster Linking](categories/development-tools/compilation/faster_linking.md)
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

- [Development Tools - Debugging, Logging](categories/development-tools_debugging/index.md)
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
  - [Low-level Profiling Tools](categories/development-tools_profiling/assembly.md)
  - [Memory Usage Analysis](categories/development-tools_profiling/memory_usage_analysis.md)

- [Development Tools - Testing](categories/development-tools_testing/index.md)
  - [Assertions](categories/development-tools_testing/assertions.md)
  - [Testing](categories/development-tools_testing/testing.md)
  - [Property-based Testing](categories/development-tools_testing/property_based_testing.md)
  - [Mocking](categories/development-tools_testing/mocking.md)
  - [Test Runners](categories/development-tools_testing/test_runners.md)
  - [Code Coverage](categories/development-tools_testing/code_coverage.md)
  - [Fuzzing](categories/development-tools_testing/fuzzing.md)

- [Encoding and Serialization](categories/encoding/index.md)
  - [Character Sets](categories/encoding/string_encoding.md)
  - [CSV Processing](categories/encoding/csv.md)
  - [Structured Data](categories/encoding/complex_encoding.md)
  - [Serialization with `serde`](categories/encoding/serde.md)
  - [Typecasting](categories/encoding/typecasts.md)
  - [Binary Encoders](categories/encoding/_binary_encoders.md)
  - [Binary Encoders, No External Schema](categories/encoding/no_external_schema.md)

- [Error Handling](categories/rust-patterns/error_handling/error_handling.md)
  - [Error Customization](categories/rust-patterns/error_handling/error_customization.md)

- [File System](categories/filesystem/index.md)
  - [File Reading & Writing](categories/filesystem/read-write.md)
  - [Directories](categories/filesystem/directories.md)
  - [Directory Traversal](categories/filesystem/directory_traversal.md)
  - [File Watching](categories/filesystem/file_watching.md)
  - [Temporary Files and Directories](categories/filesystem/tempfile.md)
  - [User Directories](categories/filesystem/user_directories.md)

- [Hardware Support](categories/hardware-support/index.md)
  - [Processor](categories/hardware-support/processor.md)
  - [Peripherals](categories/hardware-support/peripherals.md)

- [Mathematics](categories/mathematics/index.md)
  - [Linear Algebra](categories/mathematics/linear_algebra.md)
  - [Trigonometry](categories/mathematics/trigonometry.md)
  - [Complex Numbers](categories/mathematics/complex_numbers.md)
  - [Statistics](categories/mathematics/statistics.md)
  - [Additional Numeric Types](categories/mathematics/additional_numeric_types.md)

- [Memory Management](categories/memory-management/index.md)
  - [Lazy Initialization](categories/memory-management/lazy_initialization.md)
  - [Memory Allocation](categories/memory-management/memory_allocation.md)

- [Network Programming](categories/network-programming/index.md)
  - [Server](categories/network-programming/server.md)
  - [Reverse Proxy](categories/network-programming/reverse_proxy.md)

- [Operating Systems](categories/os/index.md)
  - [External Commands](categories/os/external_commands.md)
  - [Low-level System Calls](categories/os/low_level_system_calls.md)
  - [Rust OSes](categories/os/rust_os.md)

- [Operating systems - Windows](categories/os_windows-apis/index.md)
  - [Windows](categories/os_windows-apis/windows.md)

- [Parser Implementations](categories/parser-implementations/index.md)
  - [HTML](categories/parser-implementations/html.md)
  - [INI](categories/parser-implementations/ini.md)
  - [JSON](categories/parser-implementations/json.md)
  - [Markdown](categories/parser-implementations/markdown.md)
  - [TOML](categories/parser-implementations/toml.md)
  - [XML](categories/parser-implementations/xml.md)
  - [YAML](categories/parser-implementations/yaml.md)
  - [Programming Languages](categories/parser-implementations/programming_languages.md)

- [Rust Patterns](categories/rust-patterns/index.md)
  - [Creational Patterns](categories/rust-patterns/creational_patterns.md)
    - [Builder Pattern](categories/rust-patterns/builder_pattern.md)
  - [Structural Patterns](categories/rust-patterns/structural_patterns.md)
  - [Behavioral Design Patterns](categories/rust-patterns/behavioral_patterns.md)
  - [Functional Programming](categories/rust-patterns/functional_programming.md)
  - [Rust-specific Patterns](categories/rust-patterns/rust_specific_patterns.md)

- [Template Engines](categories/template-engine/index.md)
  - [Tera](categories/template-engine/tera.md)
  - [TinyTemplate](categories/template-engine/tinytemplate.md)

- [Text Editors](categories/text-editors/index.md)
  - [IDEs](categories/text-editors/ides.md)

- [Text Processing](categories/text-processing/index.md)
  - [Regular Expressions](categories/text-processing/regex.md)
  - [String Parsing](categories/text-processing/string_parsing.md)
  - [String Concatenation](categories/text-processing/string_concat.md)
  - [String Manipulation](categories/text-processing/string_manipulation.md)
  - [String Search](categories/text-processing/string_search.md)
  - [Unicode Handling](categories/text-processing/unicode.md)
  - [Diffing](categories/text-processing/diffing.md)
  - [OS and C Strings](categories/text-processing/other_strings.md)

- [Web Programming](categories/web-programming/index.md)
  - [HTTP Types and Interfaces](categories/web-programming/http_types_and_interfaces.md)
  - [Extracting Links](categories/web-programming/scraping.md)
  - [URLs](categories/web-programming/url.md)
  - [Media Types](categories/web-programming/mime.md)

- [Web Programming - HTTP Clients](categories/web-programming_http-client/index.md)
  - [HTTP clients](categories/web-programming_http-client/http_clients.md)
  - [Making Requests](categories/web-programming_http-client/requests.md)
  - [Calling a Web API](categories/web-programming_http-client/apis.md)
  - [Downloads](categories/web-programming_http-client/download.md)

- [Web Programming - HTTP Servers](categories/web-programming_http-server/index.md)
  - [Axum](categories/web-programming_http-server/_axum.md)
  - [Actix](categories/web-programming_http-server/_actix.md)
  - [Other Web Frameworks](categories/web-programming_http-server/other_frameworks.md)
  - [Static Website Generators](categories/web-programming_http-server/static_website_generators.md)
  - [Middleware](categories/web-programming_http-server/middleware.md)
  - [CORS](categories/web-programming_http-server/cors.md)
  - [Batteries-included Frameworks](categories/web-programming_http-server/_batteries-included_frameworks.md)
  - [GraphQL](categories/web-programming_http-server/_graphql.md)
  - [gRPC](categories/web-programming_http-server/_grpc.md)
  - [`hyper`](categories/web-programming_http-server/_hyper.md)

- [Written in Rust](other/written-in-rust/index.md)
  - [Development tools](other/written-in-rust/development_tools.md)
  - [Python tools](other/written-in-rust/python_tools.md)
  - [Others](other/written-in-rust/other_tools.md)

---

- [Links](links/index.md)
  - [Links](links/links.md)
  - [Example Code](links/example_code.md)
  - [Cheat Sheets](links/rust_cheatsheets.md)
  - [Blogs](links/blogs_podcasts_meetups.md)
  - [Books](links/books.md)
  - [Companies](links/companies.md)
  - [Learning](links/learning.md)

- [Contributing](contributing/index.md)
  - [Topics of Interest](contributing/topics_of_interest.md)
  - [Repository Structure](contributing/repo_structure.md)
  - [Environment Setup](contributing/dev_environment_setup.md)
  - [Editing](contributing/development_editing.md)
  - [Dev Containers and Docker](contributing/dev_container_docker.md)
  - [Optional Preprocessors](contributing/optional_preprocessors.md)
  - [API Documentation](contributing/api_documentation.md)
  - [Crate Publication](contributing/publication.md)

[Thanks](thanks.md)

---

## LATER

- [Accessibility](categories/accessibility/index.md)
  - [Enable screen readers](categories/accessibility/screen_readers.md)

- [Aerospace](categories/categories/aerospace/index.md)
  - [Aerospace](categories/categories/aerospace/aerospace.md)

- [Aerospace - Drones](categories/categories/aerospace_drones/index.md)
  - [Drones](categories/categories/aerospace_drones/drones.md)

- [Aerospace - Protocols](categories/categories/aerospace_protocols/index.md)
  - [Protocols](categories/categories/aerospace_protocols/aerospace_protocols.md)

- [Aerospace - Simulation](categories/categories/aerospace_simulation/index.md)
  - [Simulation](categories/categories/aerospace_simulation/aerospace_simulation.md)

- [Aerospace - Space Protocols](categories/categories/aerospace_space-protocols/index.md)
  - [Space Protocols](categories/categories/aerospace_space-protocols/space_protocols.md)

- [Aerospace - Unmanned Aerial Vehicles](categories/categories/aerospace_unmanned-aerial-vehicles/index.md)
  - [UAVs](categories/categories/aerospace_unmanned-aerial-vehicles/uavs.md)

- [Audio](categories/categories/multimedia_audio/index.md)
  - [Audio](categories/categories/multimedia_audio/audio.md)

- [Cloud](other/cloud/index.md)
  - [Rust on AWS](other/cloud/aws.md)
  - [Rust-native cloud development](other/cloud/rust_native_cloud_development.md)

- [Compilers](categories/compilers/index.md)
  - [Compilation duration reduction](categories/development-tools/compilation/reduce_compilation_duration.md)
  - [Incremental computation](categories/compilers/incremental_computation.md)

- [Computer vision](categories/computer-vision/index.md)
  - [Open CV](categories/computer-vision/opencv.md)

- [Cross-platform Development](other/cross-platform/index.md)
  - [Crux](other/cross-platform/crux.md)

- [Cryptocurrencies](categories/cryptography_cryptocurrencies/index.md)
  - [Cryptocurrencies](categories/cryptography_cryptocurrencies/cryptocurrencies.md)

- [Data processing](other/data-processing/index.md)
  - [CSV](other/data-processing/csv.md)
  - [Dataframes](other/data-processing/dataframes.md)
  - [Data engineering](other/data-processing/data_engineering.md)

- [Development tools - FFI](categories/development-tools_ffi/index.md)
  - [Generating FFI bindings for C and C++](categories/development-tools_ffi/generate_ffi_bindings.md)
  - [Erlang, Elixir](categories/development-tools_ffi/erlang_elixir.md)
  - [Flutter](categories/development-tools_ffi/flutter.md)
  - [Java](categories/development-tools_ffi/java.md)
  - [Lua](categories/development-tools_ffi/lua.md)
  - [Node](categories/development-tools_ffi/node.md)
  - [Objective-C](categories/development-tools_ffi/objc.md)
  - [Ruby](categories/development-tools_ffi/ruby.md)

- [Development tools - procedural macro helpers](categories/development-tools_procedural-macro-helpers/index.md)
  - [Crates for macro development](categories/development-tools_procedural-macro-helpers/write_proc_macros.md)
  - [Tools for macro development](categories/development-tools_procedural-macro-helpers/macro_tools.md)
  - [Compiling macros ahead of time](categories/development-tools_procedural-macro-helpers/compile_macros.md)

- [DevOps](other/devops/index.md)
  - [DevOps](other/devops/devops.md)
  - [Git Hooks](other/devops/version_control.md)
  - [Github Actions](other/devops/github_actions.md)
  - [Release Automation](other/devops/release_automation.md)
  - [CD/CI](other/devops/cd_ci.md)

- [Email](categories/email/index.md)
  - [Sending emails](categories/email/send_emails.md)

- [Embedded development](categories/embedded/index.md)
  - [`embassy`](categories/embedded/embassy.md)

- [Emulators](categories/emulators/index.md)
  - [Emulators](categories/emulators/emulators.md)

- [External FFI bindings](categories/external-ffi-bindings/index.md)
  - [External FFI bindings](categories/external-ffi-bindings/external_ffi_bindings.md)

- [Finance](categories/finance/index.md)
  - [Quant](categories/finance/quant.md)

- [Games](categories/games/index.md)
  - [Games](categories/games/games.md)

- [Game development](categories/game-development/index.md)
  - [Game development](categories/game-development/game_development.md)

- [Game engines](categories/game-engines/index.md)
  - [Game engines](categories/game-engines/game_engines.md)

- [Graphics](categories/graphics/index.md)

- [GPU programming](other/gpu/index.md)
  - [GPU](other/gpu/gpu.md)

- [GUI](categories/gui/index.md)
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

- [Images](categories/multimedia_images/index.md)
  - [Images](categories/multimedia_images/images.md)

- [Internationalization](categories/internationalization/index.md)
  - [Internationalization](categories/internationalization/internationalization.md)

- [Localization](categories/localization/index.md)
  - [Localization](categories/localization/localization.md)

- [Machine learning](categories/science/classical_machine_learning.md)

- [Multimedia](categories/multimedia/index.md)
  - [Multimedia](categories/multimedia/multimedia.md)

- [Multimedia encoding](categories/multimedia_encoding/index.md)
  - [Multimedia encoding](categories/multimedia_encoding/encoding.md)

- [No `alloc`](categories/no-std_no-alloc/index.md)
  - [No `alloc`](categories/no-std_no-alloc/no_alloc.md)

- [No `std`](categories/no-std/index.md)
  - [No `std`](categories/no-std/no_std.md)

- [Operating systems - FreeBSD](categories/os_freebsd-apis/index.md)
  - [FreeBSD](categories/os_freebsd-apis/freebsd.md)

- [Operating systems - Linux](categories/os_linux-apis/index.md)
  - [Linux](categories/os_linux-apis/linux.md)

- [Operating systems - macOS](categories/os_macos-apis/index.md)
  - [macOS](categories/os_macos-apis/macos.md)

- [Operating systems - Unix](categories/os_unix-apis/index.md)
  - [Unix](categories/os_unix-apis/unix.md)

- [Parsing](categories/parsing/index.md)
  - [Parsing](categories/parsing/parsing.md)

- [Rendering](categories/rendering/index.md)
  - [2D Renderers](categories/rendering/2d_renderers.md)

- [Rendering - data formats](categories/rendering_data-formats/index.md)
  - [Data formats](categories/rendering_data-formats/data_formats.md)

- [Rendering engines](categories/rendering_engine/index.md)
  - [Rendering engines](categories/rendering_engine/rendering_engines.md)

- [Rendering - graphics APIs](categories/rendering_graphics-api/index.md)
  - [Native Graphics APIs](categories/rendering_graphics-api/native_graphics_apis.md)
  - [WebGPU](categories/rendering_graphics-api/webgpu.md)

- [Robotics](categories/science_robotics/index.md)
  - [Robotics](categories/science_robotics/robot_operating_systems.md)
  - [Useful robotics tools and libraries](categories/science_robotics/robotics_frameworks.md)

- [Science](categories/science/index.md)

- [Science - Geo](categories/science_geo/index.md)
  - [Geo](categories/science_geo/geo.md)

- [Science - Neuro](categories/science_neuroscience/index.md)
  - [Neuro](categories/science_neuroscience/neuroscience.md)

- [Scripting](other/scripting/index.md)
  - [Rhai](other/scripting/rhai.md)

- [Simulation](categories/simulation/index.md)
  - [Simulation](categories/simulation/simulation.md)

- [Value formatting](categories/value-formatting/index.md)
  - [Value formatting](categories/value-formatting/number_formatting.md)

- [Video](categories/multimedia_video/index.md)
  - [Video](categories/multimedia_video/video.md)

- [Virtualization](categories/virtualization/index.md)
  - [Virtualization](categories/virtualization/virtualization.md)
  - [Containers](categories/virtualization/containers.md)

- [Visualization](categories/visualization/index.md)
  - [Visualization](categories/visualization/visualization.md)

- [Web assembly](categories/wasm/index.md)
  - [Yew](categories/wasm/yew.md)
  - [Others](categories/wasm/wasm_standalone_runtimes.md)

- [Web programming - websocket](categories/web-programming_websocket/index.md)
  - [Websocket](categories/web-programming_websocket/websocket.md)
