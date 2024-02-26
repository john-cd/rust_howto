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
[ex-derive-more]: standard_library/derive.md#derive-more
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

## Authentication

{{#include categories/authentication/index.incl.md}}

[ex-basic-authentication]: categories/authentication/index.md#basic-authentication

## Cloud

{{#include other/cloud/index.incl.md}}

[ex-aws]: other/cloud/index.md#aws
[ex-dapr]: other/cloud/index.md#dapr

## Command-line Interface

{{#include categories/command-line-interface/arguments.incl.md}}

{{#include categories/command-line-interface/ansi_terminal.incl.md}}

[ex-clap-basic]: categories/command-line-interface/arguments.md
[ex-ansi_term-basic]: categories/command-line-interface/ansi_terminal.md#ansi-terminal

## Command-line Utilities

{{#include categories/command-line-utilities/index.incl.md}}

[ex-cli-in-rust]: categories/command-line-utilities/index.md#command-line-utilities-written-in-rust

## Compilers

### Cross-compilation

{{#include categories/compilers/cross_compilation.incl.md}}

### Faster linking

{{#include categories/compilers/faster_linking.incl.md}}

### Compilation duration reduction

{{#include categories/compilers/reduce_compilation_duration.incl.md}}

[ex-cross]: categories/compilers/cross_compilation.md#cross
[ex-faster-linking]: categories/compilers/faster_linking.md
[ex-mold]: categories/compilers/faster_linking.md#alternative---mold-linker
[ex-dynamic-linking]: categories/compilers/reduce_compilation_duration.md#dynamic-linking
[ex-incremental-compilation]: categories/compilers/reduce_compilation_duration.md#incremental-compilation
[ex-measuring-build-times]: categories/compilers/reduce_compilation_duration.md#measuring-build-times
[ex-optimization-levels]: categories/compilers/reduce_compilation_duration.md#optimization-levels

## Compression

{{#include categories/compression/tar.incl.md}}

[ex-tar-decompress]: categories/compression/tar.md#decompress-a-tarball
[ex-tar-compress]: categories/compression/tar.md#compress-a-directory-into-tarball
[ex-tar-strip-prefix]: categories/compression/tar.md#decompress-a-tarball-while-removing-a-prefix-from-the-paths

## Computer Vision

{{#include categories/computer-vision/index.incl.md}}

[ex-open-cv]: categories/computer-vision/index.md#open-cv

## Concurrency

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

[ex-cross-platform]: other/cross_platform/index.md#crux

## Cryptography

## Encryption

{{#include categories/cryptography/encryption.incl.md}}

## Hashing

{{#include categories/cryptography/hashing.incl.md}}

[ex-sha-digest]: categories/cryptography/hashing.md#calculate-the-sha-256-digest-of-a-file
[ex-hmac]: categories/cryptography/hashing.md#sign-and-verify-a-message-with-hmac-digest
[ex-pbkdf2]: categories/cryptography/encryption.md#salt-and-hash-a-password-with-pbkdf2

## Data Processing

{{#include other/data_processing/index.incl.md}}

[ex-polars]: other/data_processing/index.md#polars
[ex-arrow]: other/data_processing/index.md#arrow
[ex-datafusion]: other/data_processing/index.md#datafusion

## Data Structures

{{#include categories/data-structures/bitfield.incl.md}}

[ex-bitflags]: categories/data-structures/bitfield.md#define-and-operate-on-a-type-represented-as-a-bitfield

## Database Access

## Date and Time

## Development Tools

### Development Tools - Build-time tooling

### Development Tools - Cargo Plugins

### Development Tools - Debugging

### Development Tools - Profiling

### Development Tools - Testing

## Encoding

## Filesystem Management

## Finance

## GPU Programming

{{#include other/gpu/index.incl.md}}

[ex-rust-gpu]: other/gpu/index.md#rust-gpu

## Hardware Support

## Mathematics

## Memory Management

## Network Programming

## Operating Systems

### Operating Systems - Windows API

## Rust Patterns

## Science

### Robotics

## Text Editors

## Text Processing

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

## HTTP Servers

## Links

{{#include refs/link-refs.md}}
