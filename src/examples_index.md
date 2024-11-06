# Index of Examples

## Rust language

{{#include lang/index.incl.md}}

[ex-main]: lang/main.md
[ex-simple-data-types]: lang/simple_data_types.md
[ex-variables-and-constants]: lang/variables_and_constants.md
[ex-ownership-borrowing]: lang/ownership_borrowing.md
[ex-slices]: lang/slices.md
[ex-functions]: lang/functions.md
[ex-control-flow]: lang/control_flow.md
[ex-structs]: lang/structs.md
[ex-enums]: lang/enums.md
[ex-traits]: lang/traits.md
[ex-trait-objects]: lang/trait_objects.md
[ex-attributes]: lang/attributes.md
[ex-generics]: lang/generics.md
[ex-lifetimes]: lang/lifetimes.md
[ex-modules]: lang/modules.md
[ex-match]: lang/match.md
[ex-closures]: lang/closures.md
[ex-iterators]: lang/iterators.md
[ex-macros]: lang/macros.md

## Standard Library

### Option

{{#include standard_library/option.incl.md}}

### Vector

{{#include standard_library/vectors.incl.md}}

### HashMap

{{#include standard_library/hashmaps.incl.md}}

### String

{{#include standard_library/strings.incl.md}}

### Copy-on-write

{{#include standard_library/cow.incl.md}}

### Smart Pointers

{{#include standard_library/smart_pointers.incl.md}}

### Automatic Trait Derivation

{{#include standard_library/derive.incl.md}}

[ex-convert-cow-to-str]: standard_library/cow.md#convert-cow-to-str
[ex-convert-cow-to-string]: standard_library/cow.md#convert-cow-to-string
[ex-derive]: standard_library/derive.md#automatic-trait-derivation
[ex-derive_more]: standard_library/derive.md#derive-more
[ex-hashmap]: standard_library/hashmaps.md
[ex-option-adapters]: standard_library/option.md#adapters-for-working-with-references
[ex-combinators]: standard_library/option.md#combinators
[ex-extracting-the-value-contained-in-option]: standard_library/option.md#extracting-the-value-contained-in-option
[ex-box]: standard_library/smart_pointers.md#box
[ex-rc]: standard_library/smart_pointers.md#rc
[ex-refcell]: standard_library/smart_pointers.md#refcell
[ex-string-type]: standard_library/strings.md#string-type
[ex-string-placeholders]: standard_library/strings.md#placeholders
[ex-string-concatenation]: standard_library/strings.md#string-concatenation
[ex-vec]: standard_library/vectors.md#vec

## Accessibility

{{#include categories/accessibility/index.incl.md}}

[ex-accesskit]: categories/accessibility/screen_readers.md

## Algorithms

### Random Numbers

{{#include categories/algorithms/randomness.incl.md}}

[ex-rand]: categories/algorithms/randomness.md#generate-random-numbers
[ex-rand-range]: categories/algorithms/randomness.md#generate-random-numbers-within-a-range
[ex-rand-dist]: categories/algorithms/randomness.md#generate-random-numbers-with-given-distribution
[ex-rand-custom]: categories/algorithms/randomness.md#generate-random-values-of-a-custom-type
[ex-rand-passwd]: categories/algorithms/randomness.md#create-random-passwords-from-a-set-of-alphanumeric-characters
[ex-rand-choose]: categories/algorithms/randomness.md#create-random-passwords-from-a-set-of-user-defined-characters

### Sorting

{{#include categories/algorithms/sorting.incl.md}}

[ex-sort-integers]: categories/algorithms/sorting.md
[ex-sort-floats]: categories/algorithms/sorting.md#sort-a-vector-of-floats
[ex-sort-structs]: categories/algorithms/sorting.md#sort-a-vector-of-structs

## Asynchronous

### Tokio

{{#include categories/asynchronous/tokio.incl.md}}

### Async channels

{{#include categories/asynchronous/async_channels.incl.md}}

### Async and traits

{{#include categories/asynchronous/async_traits.incl.md}}

### Streams

{{#include categories/asynchronous/streams.incl.md}}

### Futures crate

{{#include categories/asynchronous/futures.incl.md}}

### Mixing async and blocking code

{{#include categories/asynchronous/async_and_blocking.incl.md}}

[ex-async_traits]: categories/asynchronous/async_traits.md
[ex-tokio]: categories/asynchronous/tokio.md
[ex-async_channels]: categories/asynchronous/async_channels.md
[ex-streams]: categories/asynchronous/streams.md
[ex-futures]: categories/asynchronous/futures.md
[ex-mix-async-and-blocking]: categories/asynchronous/async_and_blocking.md

## Authentication

{{#include categories/authentication/index.incl.md}}

[ex-basic-authentication]: categories/authentication/index.md#basic-authentication

## Caching

{{#include categories/caching/index.incl.md}}

[ex-lru-cache]: categories/caching/lru.md

## Cloud

{{#include other/cloud/index.incl.md}}

[ex-aws]: other/cloud/aws.md
[ex-serverless]: other/cloud/serverless.md
[ex-dapr]: other/cloud/serverless.md
[ex-rust-native-cloud]: other/cloud/rust_native_cloud_development.md

## Command-line Interface

{{#include categories/command-line-interface/arguments.incl.md}}

{{#include categories/command-line-interface/ansi_terminal.incl.md}}

[ex-clap-basic]: categories/command-line-interface/arguments.md
[ex-ansi_term-basic]: categories/command-line-interface/ansi_terminal.md#ansi-terminal

## Command-line Utilities

{{#include categories/command-line-utilities/index.incl.md}}

[ex-shell-cli]: categories/command-line-utilities/shells.md
[ex-filesystem-cli]: categories/command-line-utilities/filesystem.md#
[ex-networking-cli]: categories/command-line-utilities/networking.md

## Compilers

### Cross-compilation

{{#include categories/compilers/cross_compilation.incl.md}}

### Faster linking

{{#include categories/compilers/faster_linking.incl.md}}

### Compilation duration reduction

{{#include categories/compilers/reduce_compilation_duration.incl.md}}

[ex-cross]: categories/compilers/cross_compilation.md
[ex-faster-linking]: categories/compilers/faster_linking.md
[ex-mold]: categories/compilers/faster_linking.md#alternative---mold-linker
[ex-dynamic-linking]: categories/compilers/reduce_compilation_duration.md#dynamic-linking
[ex-incremental-compilation]: categories/compilers/reduce_compilation_duration.md#incremental-compilation
[ex-measuring-build-times]: categories/compilers/reduce_compilation_duration.md#measuring-build-times
[ex-optimization-levels]: categories/compilers/reduce_compilation_duration.md#optimization-levels

## Compression

{{#include categories/compression/index.incl.md}}

[ex-tar-decompress]: categories/compression/tar.md#decompress-a-tarball
[ex-tar-compress]: categories/compression/tar.md#compress-a-directory-into-tarball
[ex-tar-strip-prefix]: categories/compression/tar.md#decompress-a-tarball-while-removing-a-prefix-from-the-paths

## Computer Vision

{{#include categories/computer-vision/index.incl.md}}

[ex-open-cv]: categories/computer-vision/opencv.md

## Concurrency

### Multi-threading

{{#include categories/concurrency/multithreading.incl.md}}

### Explicit threads

{{#include categories/concurrency/threads.incl.md}}

### Data parallelism

{{#include categories/concurrency/parallel.incl.md}}

### Message passing

{{#include categories/concurrency/message_passing.incl.md}}

### Shared-state concurrency

{{#include categories/concurrency/shared_state/index.incl.md}}

[ex-crossbeam-spawn]: categories/concurrency/threads.md#spawn-a-short-lived-thread
[ex-crossbeam-pipeline]: categories/concurrency/threads.md#create-a-parallel-pipeline
[ex-crossbeam-spsc]: categories/concurrency/threads.md#pass-data-between-two-threads
[ex-global-mut-state]: categories/concurrency/threads.md#maintain-global-mutable-state
[ex-threadpool-walk]: categories/concurrency/threads.md#calculate-sha256-sum-of-iso-files-concurrently
[ex-threadpool-fractal]: categories/concurrency/threads.md#draw-fractal-dispatching-work-to-a-thread-pool
[ex-rayon-iter-mut]: categories/concurrency/parallel.md#mutate-the-elements-of-an-array-in-parallel
[ex-rayon-any-all]: categories/concurrency/parallel.md#test-in-parallel-if-any-or-all-elements-of-a-collection-match-a-given-predicate
[ex-rayon-parallel-search]: categories/concurrency/parallel.md#search-items-using-given-predicate-in-parallel
[ex-rayon-parallel-sort]: categories/concurrency/parallel.md#sort-a-vector-in-parallel
[ex-rayon-map-reduce]: categories/concurrency/parallel.md#map-reduce-in-parallel
[ex-rayon-thumbnails]: categories/concurrency/parallel.md#generate-jpg-thumbnails-in-parallel
[ex-mutex]: categories/concurrency/shared_state/index.md#mutex
[ex-parking_lot]: categories/concurrency/shared_state/index.md#parking-lot
[ex-atomics]: categories/concurrency/shared_state/index.md#atomics
[ex-concurrent-data-structures]: categories/concurrency/shared_state/concurrent_data_structures.md

## Configuration

### Configuration Management

{{#include categories/config/configuration.incl.md}}

### Environment Variables

{{#include categories/config/environment_variables.incl.md}}

[ex-config]: categories/config/configuration.md#config
[ex-confy]: categories/config/configuration.md#confy
[ex-dotenvy]: categories/config/environment_variables.md#dotenvy
[ex-env]: categories/config/environment_variables.md#stdenv
[ex-envy]: categories/config/environment_variables.md#envy

## Cross-platform Development

{{#include other/cross_platform/index.incl.md}}

[ex-cross-platform]: other/cross_platform/crux.md

## Cryptography

### Encryption

{{#include categories/cryptography/encryption.incl.md}}

### Hashing

{{#include categories/cryptography/hashing.incl.md}}

[ex-sha-digest]: categories/cryptography/hashing.md#calculate-the-sha-256-digest-of-a-file
[ex-hmac]: categories/cryptography/hashing.md#sign-and-verify-a-message-with-hmac-digest
[ex-pbkdf2]: categories/cryptography/encryption.md#salt-and-hash-a-password-with-pbkdf2

## Data Processing

{{#include other/data_processing/index.incl.md}}

[ex-polars]: other/data_processing/dataframes.md#manipulate-data-in-a-tabular-format
[ex-read-and-write-csv-files]: other/data_processing/csv.md#read-and-write-csv-files
[ex-manipulate-csv-files-from-the-command-line]: other/data_processing/csv.md#manipulate-csv-files-from-the-command-line
[ex-arrow]: other/data_processing/data_engineering.md#develop-data-analytics-applications-that-process-columnar-data-with-arrow
[ex-datafusion]: other/data_processing/data_engineering.md#query-in-memory-data-with-datafusion

## Data Structures

{{#include categories/data-structures/index.incl.md}}

[ex-bitflags]: categories/data-structures/bitfield.md#define-and-operate-on-a-type-represented-as-a-bitfield

## Database Access

### SQLite

{{#include categories/database/sqlite.incl.md}}

### Postgres

{{#include categories/database/postgres.incl.md}}

### Query Builders and ORMs

{{#include categories/database/query_builders_orms.incl.md}}

[ex-sqlite-initialization]: categories/database/sqlite.md#create-a-sqlite-database
[ex-sqlite-insert-select]: categories/database/sqlite.md#insert-and-select-data
[ex-postgres-create-tables]: categories/database/postgres.md#create-tables-in-a-postgres-database
[ex-postgres-insert-query-data]: categories/database/postgres.md#insert-and-query-data
[ex-postgres-aggregate-data]: categories/database/postgres.md#aggregate-data
[ex-diesel]: categories/database/query_builders_orms.md#diesel
[ex-seaorm]: categories/database/query_builders_orms.md#seaorm
[ex-sqlx]: categories/database/query_builders_orms.md#sqlx
[ex-elasticsearch]: categories/database/search.md#elasticsearch

## Date and Time

### Duration and Calculation

{{#include categories/date-and-time/duration.incl.md}}

### Parsing and Displaying

{{#include categories/date-and-time/parse.incl.md}}

[ex-measure-elapsed-time]: categories/date-and-time/duration.md#measure-the-elapsed-time-between-two-code-sections
[ex-datetime-arithmetic]: categories/date-and-time/duration.md#perform-checked-date-and-time-calculations
[ex-convert-datetime-timezone]: categories/date-and-time/duration.md#convert-a-local-time-to-another-timezone
[ex-examine-date-and-time]: categories/date-and-time/parse.md#examine-the-date-and-time
[ex-convert-datetime-timestamp]: categories/date-and-time/parse.md#convert-date-to-unix-timestamp-and-vice-versa
[ex-format-datetime]: categories/date-and-time/parse.md#display-formatted-date-and-time
[ex-parse-datetime]: categories/date-and-time/parse.md#parse-string-into-datetime-struct

## Development Tools

### Cargo

{{#include categories/development-tools/cargo/index.incl.md}}

### Documentation

{{#include categories/development-tools/documentation/index.incl.md}}

### Formatting

{{#include categories/development-tools/formatting/index.incl.md}}

### Installation

{{#include categories/development-tools/installation/index.incl.md}}

### Other

{{#include categories/development-tools/other/index.incl.md}}

[ex-background-code-checker]: categories/development-tools/other/index.md#background-code-checker

### Versioning

{{#include categories/development-tools/versioning/index.incl.md}}

[ex-basic-cargo-usage]: categories/development-tools/cargo/index.md#basic-cargo-usage
[ex-cargo-toml]: categories/development-tools/cargo/index.md#cargotoml-and-lock-files
[ex-crates.io]: categories/development-tools/cargo/crates.md

[ex-documenting-your-code]: categories/development-tools/documentation/index.md#documenting-your-code
[ex-module-or-crate-level-documentation]: categories/development-tools/documentation/index.md#module-or-crate-level-documentation
[ex-mdbook]: categories/development-tools/documentation/mdbook.md
[ex-playground]: categories/development-tools/documentation/playground.md
[ex-badges]: categories/development-tools/documentation/badges.md

[ex-formatting]: categories/development-tools/formatting/index.md#rustfmt
[ex-rustfmt-config]: categories/development-tools/formatting/index.md#rustfmt-configuration
[ex-formatting-attributes]: categories/development-tools/formatting/index.md#formatting-attributes

[ex-rustup]: categories/development-tools/installation/rustup.md
[ex-cargo-install]: categories/development-tools/installation/index.md#cargo-install
[ex-cargo-binstall]: categories/development-tools/installation/index.md#cargo-binstall

[ex-api-search]: categories/development-tools/other/index.md#api-search
[ex-deployment]: categories/development-tools/other/index.md#deployment
[ex-just]: categories/development-tools/other/just.md
[ex-miri]: categories/development-tools/other/miri.md

[ex-semver-increment]: categories/development-tools/versioning/index.md#parse-and-increment-a-version-string
[ex-semver-complex]: categories/development-tools/versioning/index.md#parse-a-complex-version-string
[ex-semver-prerelease]: categories/development-tools/versioning/index.md#check-if-given-version-is-pre-release
[ex-semver-latest]: categories/development-tools/versioning/index.md#find-the-latest-version-satisfying-given-range
[ex-semver-command]: categories/development-tools/versioning/index.md#check-external-command-version-for-compatibility

## Development Tools - Build-time tooling

{{#include  categories/development-tools_build-utils/index.incl.md}}

[ex-cc-static-bundled]: categories/development-tools_build-utils/index.md#compile-and-link-statically-to-a-bundled-c-library
[ex-cc-static-bundled-cpp]: categories/development-tools_build-utils/index.md#compile-and-link-statically-to-a-bundled-c-library-1
[ex-cc-custom-defines]: categories/development-tools_build-utils/index.md#compile-a-c-library-while-setting-custom-defines

## Development Tools - Cargo Plugins

{{#include categories/development-tools_cargo-plugins/index.incl.md}}

[ex-code-coverage]: categories/development-tools_cargo-plugins/testing.md#code-coverage
[ex-fix-compiler-warnings]: categories/development-tools_cargo-plugins/code_formatting_linting.md#fix-compiler-warnings-automatically
[ex-format]: categories/development-tools_cargo-plugins/code_formatting_linting.md#format-your-code
[ex-lint]: categories/development-tools_cargo-plugins/code_formatting_linting.md#lint-your-code
[ex-security-audit]: categories/development-tools_cargo-plugins/auditing.md#audit-cargolock-files-for-crates-containing-security-vulnerabilities
[ex-templates]: categories/development-tools_cargo-plugins/writing.md#generate-a-rust-project-from-a-template
[ex-unused-dependencies]: categories/development-tools_cargo-plugins/maintenance.md#find-unused-dependencies
[ex-watch-for-changes]: categories/development-tools_cargo-plugins/watching_for_changes.md#watching-for-changes

## Development Tools - Debugging

{{#include categories/development-tools_debugging/config_log.incl.md}}

{{#include categories/development-tools_debugging/log.incl.md}}

{{#include categories/development-tools_debugging/tracing.incl.md}}

[ex-log-debug]: categories/development-tools_debugging/log.md#log-a-debug-message-to-the-console
[ex-log-error]: categories/development-tools_debugging/log.md#log-an-error-message-to-the-console
[ex-log-stdout]: categories/development-tools_debugging/log.md#log-to-stdout-instead-of-stderr
[ex-log-custom-logger]: categories/development-tools_debugging/log.md#log-messages-with-a-custom-logger
[ex-log-syslog]: categories/development-tools_debugging/log.md#log-to-the-unix-syslog
[ex-log-mod]: categories/development-tools_debugging/config_log.md#enable-log-levels-per-module
[ex-log-env-variable]: categories/development-tools_debugging/config_log.md#use-a-custom-environment-variable-to-set-up-logging
[ex-log-timestamp]: categories/development-tools_debugging/config_log.md#include-timestamp-in-log-messages
[ex-log-custom]: categories/development-tools_debugging/config_log.md#log-messages-to-a-custom-location

## Development Tools - Profiling

{{#include categories/development-tools_profiling/index.incl.md}}

## Development Tools - Testing

{{#include categories/development-tools_testing/index.incl.md}}

[ex-test-your-code]: categories/development-tools_testing/testing.md
[ex-emit-custom-message]: categories/development-tools_testing/testing.md#custom-message

## Encoding

### Character Sets

{{#include categories/encoding/strings.incl.md}}

### CSV Processing

{{#include categories/encoding/csv.incl.md}}

### Structured Data

{{#include categories/encoding/complex.incl.md}}

### Serde

{{#include categories/encoding/serde.incl.md}}

[ex-percent-encode]: categories/encoding/strings.md#percent-encode-a-string
[ex-urlencoded]: categories/encoding/strings.md#encode-a-string-as-applicationx-www-form-urlencoded
[ex-hex-encode-decode]: categories/encoding/strings.md#encode-and-decode-hex
[ex-base64]: categories/encoding/strings.md#encode-and-decode-base64
[ex-csv-read]: categories/encoding/csv.md#read-csv-records
[ex-csv-delimiter]: categories/encoding/csv.md#read-csv-records-with-different-delimiter
[ex-csv-filter]: categories/encoding/csv.md#filter-csv-records-matching-a-predicate
[ex-invalid-csv]: categories/encoding/csv.md#handle-invalid-csv-data-with-serde
[ex-serialize-csv]: categories/encoding/csv.md#serialize-records-to-csv
[ex-csv-serde]: categories/encoding/csv.md#serialize-records-to-csv-using-serde
[ex-csv-transform-column]: categories/encoding/csv.md#transform-csv-column
[ex-json-value]: categories/encoding/complex.md#serialize-and-deserialize-unstructured-json
[ex-toml-config]: categories/encoding/complex.md#deserialize-a-toml-configuration-file
[ex-byteorder-le]: categories/encoding/complex.md#read-and-write-integers-in-little-endian-byte-order

## Filesystem Management

{{#include categories/filesystem/dir.incl.md}}

{{#include categories/filesystem/read-write.incl.md}}

[ex-std-read-lines]: categories/filesystem/read-write.md#read-lines-of-strings-from-a-file
[ex-avoid-read-write]: categories/filesystem/read-write.md#avoid-writing-and-reading-from-a-same-file
[ex-random-file-access]: categories/filesystem/read-write.md#access-a-file-randomly-using-a-memory-map
[ex-file-24-hours-modified]: categories/filesystem/dir.md#find-files-that-have-been-modified-in-the-last-24-hours
[ex-find-file-loops]: categories/filesystem/dir.md#find-loops-for-a-given-path
[ex-dedup-filenames]: categories/filesystem/dir.md#recursively-find-duplicate-file-names
[ex-file-predicate]: categories/filesystem/dir.md#recursively-find-all-files-with-given-predicate
[ex-file-skip-dot]: categories/filesystem/dir.md#traverse-directories-while-skipping-dotfiles
[ex-file-sizes]: categories/filesystem/dir.md#recursively-calculate-file-sizes-at-given-depth
[ex-glob-recursive]: categories/filesystem/dir.md#find-all-png-files-recursively
[ex-glob-with]: categories/filesystem/dir.md#find-all-files-with-given-pattern-ignoring-filename-case

## Finance

{{#include categories/finance/index.incl.md}}

[ex-quant]: categories/finance/index.md

## GPU Programming

{{#include other/gpu/index.incl.md}}

[ex-rust_gpu]: other/gpu/gpu.md

## Hardware Support

{{#include categories/hardware-support/processor.incl.md}}

[ex-check-cpu-cores]: categories/hardware-support/processor.md#check-number-of-logical-cpu-cores

## Mathematics

### Linear algebra

{{#include categories/mathematics/linear_algebra.incl.md}}

### Trigonometry

{{#include categories/mathematics/trigonometry.incl.md}}

### Complex numbers

{{#include categories/mathematics/complex_numbers.incl.md}}

### Statistics

{{#include categories/mathematics/statistics.incl.md}}

### Miscellaneous

{{#include categories/mathematics/miscellaneous.incl.md}}

[ex-vector-norm]: categories/mathematics/linear_algebra.md#vector-norm
[ex-add-matrices]: categories/mathematics/linear_algebra.md#adding-matrices
[ex-multiply-matrices]: categories/mathematics/linear_algebra.md#multiplying-matrices
[ex-multiply-scalar-vector-matrix]: categories/mathematics/linear_algebra.md#multiply-a-scalar-with-a-vector-with-a-matrix
[ex-invert-matrix]: categories/mathematics/linear_algebra.md#invert-matrix
[ex-side-length]: categories/mathematics/trigonometry.md#calculating-the-side-length-of-a-triangle
[ex-tan-sin-cos]: categories/mathematics/trigonometry.md#verifying-tan-is-equal-to-sin-divided-by-cos
[ex-latitude-longitude]: categories/mathematics/trigonometry.md#distance-between-two-points-on-the-earth
[ex-create-complex]: categories/mathematics/complex_numbers.md#creating-complex-numbers
[ex-add-complex]: categories/mathematics/complex_numbers.md#adding-complex-numbers
[ex-mathematical-functions]: categories/mathematics/complex_numbers.md#mathematical-functions
[ex-central-tendency]: categories/mathematics/statistics.md#measures-of-central-tendency
[ex-standard-deviation]: categories/mathematics/statistics.md#standard-deviation
[ex-big-integers]: categories/mathematics/additional_numeric_types.md

## Memory Management

{{#include categories/memory-management/global_static.incl.md}}

{{#include categories/memory-management/lazy_initialization.incl.md}}

[ex-lazy-constant]: categories/memory-management/global_static.md#declare-lazily-evaluated-constant

## Network Programming

{{#include categories/network-programming/server.incl.md}}

[ex-random-port-tcp]: categories/network-programming/server.md#listen-on-unused-port-tcpip

## Operating Systems

{{#include categories/os/external.incl.md}}

[ex-parse-subprocess-output]: categories/os/external.md#run-an-external-command-and-process-stdout
[ex-parse-subprocess-input]: categories/os/external.md#run-an-external-command-passing-it-stdin-and-check-for-an-error-code
[ex-run-piped-external-commands]: categories/os/external.md#run-piped-external-commands
[ex-redirect-stdout-stderr-same-file]: categories/os/external.md#redirect-both-stdout-and-stderr-of-child-process-to-the-same-file
[ex-continuous-process-output]: categories/os/external.md#continuously-process-child-process-outputs
[ex-read-env-variable]: categories/os/external.md#read-environment-variable

### Operating Systems - Windows API

{{#include categories/os_windows-apis/index.incl.md}}

[ex-windows]: categories/os_windows-apis/index.md#windows

## Rust Patterns

### Error Handling

{{#include categories/rust-patterns/index.incl.md}}

[ex-error_chain-simple-error-handling]: categories/rust-patterns/errors/error_handling.md#handle-errors-correctly-in-main
[ex-error_chain-avoid-discarding]: categories/rust-patterns/errors/error_handling.md#avoid-discarding-errors-during-error-conversions
[ex-error_chain-backtrace]: categories/rust-patterns/errors/error_handling.md#obtain-backtrace-of-complex-error-scenarios

## Science

### Machine Learning

{{#include categories/science/machine_learning/index.incl.md}}

[ex-ml]: categories/science/machine_learning/index.md

### Robotics

{{#include categories/science_robotics/index.incl.md}}

[ex-robotics]: categories/science_robotics/index.md

## Scripting

{{#include other/scripting/index.incl.md}}

[ex-rhai]: other/scripting/rhai.md

## Text Editors

{{#include categories/text-editors/index.incl.md}}

[ex-ides]: categories/text-editors/ides.md

## Text Processing

{{#include categories/text-processing/regex.incl.md}}

{{#include categories/text-processing/string_parsing.incl.md}}

[ex-verify-extract-email]: categories/text-processing/regex.md#verify-and-extract-login-from-an-email-address
[ex-extract-hashtags]: categories/text-processing/regex.md#extract-a-list-of-unique-hashtags-from-a-text
[ex-phone]: categories/text-processing/regex.md#extract-phone-numbers-from-text
[ex-regex-filter-log]: categories/text-processing/regex.md#filter-a-log-file-by-matching-multiple-regular-expressions
[ex-regex-replace-named]: categories/text-processing/regex.md#replace-all-occurrences-of-one-text-pattern-with-another-pattern

[ex-unicode-graphemes]: categories/text-processing/string_parsing.md#collect-unicode-graphemes
[ex-string_parsing-from_str]: categories/text-processing/string_parsing.md#implement-the-fromstr-trait-for-a-custom-struct

### WebAssembly

{{#include categories/wasm/index.incl.md}}

[ex-yew]: categories/wasm/yew.md

## Web Programming

### Uniform Resource Locations (URL)

{{#include categories/web-programming/url.incl.md}}

### Media Types (MIME)

{{#include categories/web-programming/mime.incl.md}}

### Scraping Web Pages

{{#include categories/web-programming/scraping.incl.md}}

[ex-mime-from-string]: categories/web-programming/mime.md#get-mime-type-from-string
[ex-mime-from-filename]: categories/web-programming/mime.md#get-mime-type-from-filename
[ex-http-response-mime-type]: categories/web-programming/mime.md#parse-the-mime-type-of-a-http-response

[ex-extract-links-webpage]: categories/web-programming/scraping.md#extract-all-links-from-a-webpage-html
[ex-check-broken-links]: categories/web-programming/scraping.md#check-a-webpage-for-broken-links
[ex-extract-mediawiki-links]: categories/web-programming/scraping.md#extract-all-unique-links-from-a-mediawiki-markup

[ex-url-parse]: categories/web-programming/url.md#parse-a-url-from-a-string-to-a-url-type
[ex-url-base]: categories/web-programming/url.md#create-a-base-url-by-removing-path-segments
[ex-url-new-from-base]: categories/web-programming/url.md#create-new-urls-from-a-base-url
[ex-url-origin]: categories/web-programming/url.md#extract-the-url-origin-scheme--host--port
[ex-url-rm-frag]: categories/web-programming/url.md#remove-fragment-identifiers-and-query-pairs-from-a-url

## HTTP Clients

### Requests

{{#include categories/web-programming_http-client/requests.incl.md}}

### APIs

{{#include categories/web-programming_http-client/apis.incl.md}}

### Downloads

{{#include categories/web-programming_http-client/download.incl.md}}

[ex-rest-get]: categories/web-programming_http-client/apis.md#query-the-github-api
[ex-rest-head]: categories/web-programming_http-client/apis.md#check-if-an-api-resource-exists
[ex-rest-post]: categories/web-programming_http-client/apis.md#create-and-delete-gist-with-github-api
[ex-paginated-api]: categories/web-programming_http-client/apis.md#consume-a-paginated-restful-api
[ex-handle-rate-limited-api]: categories/web-programming_http-client/apis.md#handle-a-rate-limited-api
[ex-url-download]: categories/web-programming_http-client/download.md#download-a-file-to-a-temporary-directory
[ex-progress-with-range]: categories/web-programming_http-client/download.md#make-a-partial-download-with-http-range-headers
[ex-file-post]: categories/web-programming_http-client/download.md#post-a-file-to-paste-rs
[ex-url-basic]: categories/web-programming_http-client/requests.md#make-a-http-get-request
[ex-rest-custom-params]: categories/web-programming_http-client/requests.md#set-custom-headers-and-url-parameters-for-a-rest-request

## HTTP Servers

{{#include categories/web-programming_http-server/index.incl.md}}

### Middleware

{{#include categories/web-programming_http-server/middleware.incl.md}}

[ex-axum]: categories/web-programming_http-server/axum.md
[ex-actix]: categories/web-programming_http-server/actix.md
[ex-other-frameworks]: categories/web-programming_http-server/other_frameworks.md
[ex-middleware]: categories/web-programming_http-server/middleware.md
[ex-cors]: categories/web-programming_http-server/cors.md
[ex-static-website-generators]: categories/web-programming_http-server/static_website_generators.md
[ex-tower]: categories/web-programming_http-server/middleware.md#tower
[ex-tower_http]: categories/web-programming_http-server/middleware.md#tower-http
[ex-alternatives]: categories/web-programming_http-server/middleware.md#alternatives

## Written in Rust

{{#include other/written-in-rust/index.incl.md}}

[ex-python-tools]: other/written-in-rust/python_tools.md
[ex-dev-tools]: other/written-in-rust/development_tools.md
[ex-others]: other/written-in-rust/others.md

## Links

{{#include links/index.incl.md}}

[ex-example-code]: links/example_code.md
[ex-cheatsheets]: links/rust_cheatsheets.md
[ex-blogs]: links/blogs.md
[ex-books]: links/books.md
[ex-companies]: links/companies.md
{{#include refs/link-refs.md}}

<div class="hidden">
TODO make sure the list of examples is complete
</div>
