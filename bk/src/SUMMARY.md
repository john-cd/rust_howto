<!-- TODO Reviewed ??-??-2026. -->

# Summary

[Introduction](introduction.md)
[How to Read This Book](introduction/how_to_read_this_book.md)

## Core Concepts

- [Rust Installation and First Steps](rust-installation/index.md)

- [Language](language/index.md)
  - [Entry Points](language/entry_points.md)
  - [Data Types](language/data_types.md)
  - [Variables](language/variables.md)
  - [Constants and Statics](language/constants_and_statics.md)
  - [Ownership and Borrowing](language/ownership_and_borrowing.md)
  - [Lifetimes](language/lifetimes.md)
  - [Slices](language/slices.md)
  - [Control Flow](language/control_flow.md)
  - [Pattern Matching](language/pattern_matching.md)
  - [Functions](language/functions.md)
  - [Closures](language/closures.md)
  - [Structs](language/structs.md)
  - [Enumerations](language/enumerations.md)
  - [Traits](language/traits.md)
  - [`impl Trait`](language/impl_trait.md)
  - [Trait Objects and Dynamic Dispatch](language/trait_objects_and_dynamic_dispatch.md)
  - [Iterators](language/iterators.md)
  - [Generics](language/generics.md)
  - [Attributes](language/attributes.md)
  - [Macros](language/macros.md)

- [Standard Library](standard-library/index.md)
  - [Optional Values](standard-library/optional_values.md)
  - [Recoverable Error Handling](standard-library/recoverable_error_handling.md)
  - [Handle Panics at a Boundary](standard-library/panic_boundaries.md)
  - [Default Values](standard-library/default_values.md)
  - [Value Comparison](standard-library/value_comparison.md)
  - [Smart Pointers](standard-library/smart_pointers.md)
    - [Heap Storage](standard-library/heap_storage.md)
    - [Reference Counting](standard-library/reference_counting.md)
    - [Interior Mutability](standard-library/interior_mutability.md)
    - [Clone-on-Write](standard-library/clone-on-write.md)
    - [Memory Address Pinning](standard-library/memory_address_pinning.md)
    - [Pin Futures and Own Their Cancellation](standard-library/pinned_async_cancellation.md)
  - [Resource Cleanup](standard-library/resource_cleanup.md)
  - [Pointer Provenance at an FFI Boundary](standard-library/pointer_provenance.md)
  - [Conversion Traits](standard-library/conversion_traits.md)
  - [Reference-to-reference Conversions](standard-library/reference-to-reference_conversions.md)
  - [Borrowed Types](standard-library/borrowed_types.md)
  - [Automatic Trait Derivation](standard-library/automatic_trait_derivation.md)
  - [Operators](standard-library/operators.md)
  - [Dynamic Typing](standard-library/dynamic_typing.md)

- [Code Organization](code-organization/index.md)
  - [Modules and Paths](code-organization/modules_and_paths.md)
  - [Visibility](code-organization/visibility.md)
  - [`use` Declarations](code-organization/use_declarations.md)
  - [Dependencies](code-organization/dependencies.md)
  - [No Standard Library](categories/no-std/no_std.md)
  - [No Allocation](categories/no-std/no_alloc.md)
  - [Code Organization by Project Type](code-organization/code_organization_by_project_type.md)
  - [Naming Conventions](code-organization/naming_conventions.md)
  - [Configure Clippy for a Workspace](code-organization/clippy.md)

## Code Examples by Category

- [Algorithms](categories/algorithms/index.md)
  - [Random Value Generation](categories/algorithms/random_value_generation.md)
  - [Sorting](categories/algorithms/sorting.md)
  - [Hashing](categories/algorithms/hashing.md)
  - [Graph Algorithms](categories/algorithms/graph_algorithms.md)

- [Caching](categories/caching/index.md)
  - [In-memory Caching](categories/caching/in_memory_cache.md)

- [Compression](categories/compression/index.md)
  - [Working with Tarballs](categories/compression/tar.md)
  - [Compression and Decompression](categories/compression/compression.md)

- [Data Structures](categories/data-structures/index.md)
  - [Strings](categories/data-structures/strings.md)
  - [Vectors](categories/data-structures/vectors.md)
  - [HashMaps](categories/data-structures/hashmaps.md)
  - [Other Maps](categories/data-structures/other_maps.md)
  - [B-Trees](categories/data-structures/b-trees.md)
  - [Stacks and Queues](categories/data-structures/stacks_and_queues.md)
  - [Binary Heaps](categories/data-structures/binary_heaps.md)
  - [Linked Lists](categories/data-structures/linked_lists.md)
  - [Bit Arrays](categories/data-structures/bit_arrays.md)
  - [Graphs](categories/data-structures/graphs.md)
  - [Stack-allocated Data Structures](categories/data-structures/stack-allocated.md)
  - [Unique Identifiers](categories/data-structures/unique_identifiers.md)

- [Data Processing](categories/data-processing/index.md)
  - [DataFrames](categories/data-processing/dataframes.md)

- [Date and Time](categories/date-and-time/index.md)
  - [Duration and Calculation](categories/date-and-time/duration.md)
  - [Parsing and Displaying](categories/date-and-time/parse.md)
  - [`time` Crate](categories/date-and-time/time_crate.md)

- [Encoding and Serialization](categories/encoding/index.md)
  - [Character Encoding](categories/encoding/string_encoding.md)
  - [CSV Processing](categories/encoding/csv.md)
  - [Excel Spreadsheets](categories/encoding/excel.md)
  - [Serialization with `serde`](categories/encoding/serde.md)

- [File System](categories/filesystem/index.md)
  - [Paths](categories/filesystem/paths.md)
  - [File Reading and Writing](categories/filesystem/reading_and_writing_files.md)
  - [Directories](categories/filesystem/directories.md)
  - [Directory Traversal](categories/filesystem/directory_traversal.md)
  - [File Metadata](categories/filesystem/file_metadata.md)
  - [Working with the Standard Input and Output](categories/filesystem/working_with_the_standard_input_and_output.md)
  - [File Watching](categories/filesystem/file_watching.md)
  - [Temporary Files and Directories](categories/filesystem/temporary_files_and_directories.md)
  - [User Directories and Preferred Applications](categories/filesystem/user_directories_and_preferred_applications.md)
  - [Symbolic Links](categories/filesystem/symbolic_links.md)

- [Mathematics](categories/mathematics/index.md)
  - [Linear Algebra](categories/mathematics/linear_algebra.md)
  - [Trigonometry](categories/mathematics/trigonometry.md)
  - [Complex Numbers](categories/mathematics/complex_numbers.md)
  - [Statistics](categories/mathematics/statistics.md)
  - [Additional Numeric Types](categories/mathematics/additional_numeric_types.md)
  - [Numerical Analysis](categories/mathematics/numerical_analysis.md)
  - [Numerical Integration](categories/mathematics/numerical_integration.md)
  - [Optimization](categories/mathematics/optimization.md)
  - [Units of Measurement](categories/mathematics/units.md)
  - [Discrete Mathematics](categories/mathematics/discrete_mathematics.md)
  - [Graph Theory](categories/mathematics/graph_theory.md)
  - [Differential Equations](categories/mathematics/differential_equations.md)

- [Images](categories/multimedia_images/index.md)
  - [Image Processing](categories/multimedia_images/images.md)

- [Number Formatting](categories/value-formatting/number_formatting.md)

- [WebSockets](categories/web-programming_websocket/index.md)
  - [Echo Client and Server](categories/web-programming_websocket/websocket.md)

- [Parsing](categories/parsing/index.md)
  - [Parsing Utilities](categories/parsing/parsing.md)
  - [Expression Parser](categories/parsing/expression_parser.md)

- [Profiling and Benchmarking](categories/development-tools/profiling.md)

- [Date and Time](categories/date-and-time/jiff_dst.md)
- [Register Commands Explicitly or with Inventory](categories/rust-patterns/plugin_registration.md)
- [Find Missing Assertions with Mutation Testing](categories/development-tools/mutation_testing.md)
- [Test an AWS Lambda Handler Locally](categories/cloud/aws_lambda.md)
- [Connect Two Iroh Endpoints on Loopback](categories/network-programming/iroh_loopback.md)
- [Regular Expressions](categories/text-processing/regex.md)
- [String Parsing](categories/text-processing/string_parsing.md)
- [Count Unicode Text for Display](categories/text-processing/unicode.md)
- [Make an HTTP Request Against a Local Service](categories/web-programming_http-client/offline_http_client.md)
- [Cancellation and Structured Concurrency](categories/asynchronous/structured_concurrency.md)
- [Atomics](categories/concurrency/atomics.md)
- [Model-Checking with Loom](categories/concurrency/loom.md)
- [HTTP Servers](categories/web-programming_http-server/json_patch.md)
  - [OpenAPI and JSON Schema](categories/web-programming_http-server/openapi.md)
  - [Bound Retries and Concurrency with Tower](categories/web-programming_http-server/service_resilience.md)

- [Diagnose Rust Build Time](categories/development-tools/build_time_diagnosis.md)

- [Compare Benchmark Results in CI](categories/development-tools/benchmark_regressions.md)

- [Template Engines](categories/template-engine/index.md)
  - [Tera](categories/template-engine/tera.md)
  - [TinyTemplate](categories/template-engine/tinytemplate.md)

## Resources

- [Crate Selection](crate-selection/index.md)
  - [Current State of the Art](crate-selection/current_state_of_the_art.md)
  - [Generate Crate Indices Offline](crate-selection/crate_indices.md)

- [Links](links/index.md)
  - [Rust Cheat Sheets](links/rust_cheatsheets.md)
  - [Learning Rust](links/learning_rust.md)
  - [Books](links/books.md)
  - [Blogs and Forums](links/blogs_and_forums.md)
  - [Example Code](links/example_code.md)
  - [Videos and Podcasts](links/videos_and_podcasts.md)
  - [Meetings](links/meetings.md)
  - [Rust Companies and Jobs](links/rust_companies_and_jobs.md)

## Indices

- [Index](word_index.md)
- [Index of Examples](indices/examples_index.md)
- [Crates (Alphabetic)](indices/crates_alphabetical.md)
- [Crates (by Category)](indices/crates_by_category.md)

## Appendices

- [Privacy](appendices/privacy.md)
- [Contributing](appendices/contributing/index.md)
  - [Topics of Interest](appendices/contributing/topics_of_interest.md)
  - [Repository Structure](appendices/contributing/repository_structure.md)
  - [Development Environment Setup](appendices/contributing/development_environment_setup.md)
  - [Book Editing and Example Code Development](appendices/contributing/book_editing_and_example_code_development.md)
  - [API Documentation](appendices/contributing/api_documentation.md)
  - [mdBook Pre-processors](appendices/contributing/mdBook_pre-processors.md)
  - [Dev Containers and Docker](appendices/contributing/dev_container_and_docker.md)
  - [Documentation Generation](appendices/contributing/documentation_generation.md)
  - [Publication to `crates.io`](appendices/contributing/publication_to_crates.io.md)

[Thanks](appendices/thanks.md)

{{#include refs.incl.md}}

<div class="hidden">
</div>
