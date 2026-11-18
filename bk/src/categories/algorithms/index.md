<!-- TODO Reviewed ??-??-2026. -->

# Algorithms

[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

This category covers Rust implementations of algorithms{{hi:Algorithms}} such as sorting{{hi:Sorting}}, hashing, graph processing, and random value generation. Prefer the standard library for common operations; use a specialized crate when it adds a capability or performance your application needs.


## Random Value Generation

{{#include random_value_generation.incl.md}}

## Sorting

{{#include sorting.incl.md}}

## Hashing

{{#include hashing.incl.md}}

## Graph Algorithms

{{#include graph_algorithms.incl.md}}

## Related Topics

### Compression Algorithms

For [compression](../compression/index.md), use:

- `flate2` for deflate, gzip, and zlib compression,
- `zip` for zip files,
- `tar` for tar archives,
- `zstd` for Zstandard compression,
- `bzip2`,
- [`liblzma`][liblzma] for XZ compression (a maintained fork of `xz2`),
- `snap` for Snappy compression, and
- `brotli` for Brotli compression.

See [Compression](../compression/index.md).

### Data Structures

For common [[data-structures | data structures]] ([`Vec`][c~std::vec::Vec~docs]↗, [`HashMap`][c~std::collections::HashMap~docs]↗, etc.), you will use [`std::collections`][c~std::collections~docs]↗. You may also consider:

- [`im`][c~im~docs]↗{{hi:im}} for immutable data structures,
- `indexmap` for [maps](../data-structures/other_maps.md) that keep track of insertion order,
- `arrayvec`, `smallvec`, `tinyvec` and `heapless` for [[stack-allocated | stack-allocated data structures]],
- [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}} for [[graphs | graphs]],
- `dashmap` and `papaya` for [concurrent data structures](../concurrency/concurrent_data_structures.md),
- `parking_lot` for mutexes,
- `crossbeam-channel`, `flume`, `tokio`, and `postage` for channels.

Worth a mention: `fst` provides fast, memory-efficient, immutable set and map data structures for storing and searching (very) large collections of strings and associated values. It is based on finite state transducers (FSTs). The capabilities of ordered sets and maps mirror that of `BTreeSet` and `BTreeMap` found in the standard library. The key difference is that sets and maps in `fst` are immutable, keys are byte sequences, and values, in the case of maps, are always unsigned 64 bit integers.

See [[vectors | vector]], [[hashmaps | hash maps]], [other maps](../data-structures/other_maps.md), and [[binary_heaps | Binary Heap]] within the [[data-structures | Data Structures]] section for more details.

See [[data-structures | Data Structures]] and [Concurrent Data Structures](../concurrency/concurrent_data_structures.md).

### Recursive Algorithms

- `stacker` is a stack growth library useful when implementing deeply recursive algorithms that may accidentally blow the stack.

### Search Algorithms

For small-scale tasks, like filtering a list of items in memory, reach first for the standard library:

- `slice::binary_search*` for slices and vectors,
- `std::iter::Iterator::find` for iterators,
- `str::find`, `str::rfind` for strings.
- `argminmax` provides efficient argmin & argmax (in 1 function) with SIMD for floats and integers.
- `bytecount` counts occurrences of a given byte or UTF-8 code points in a byte slice quickly.

[meilisearch][c~meilisearch~repo]↗ and [tantivy][c~tantivy~repo]↗ are full-text search engines written in Rust, similar to what you'd expect from `Elasticsearch` or `Algolia`. See also [Search Engines](../database-implementations/rust_search_engines.md).

### String Algorithms and Text Processing

For [[strings | Strings]], use:

- [`regex`][c~regex~docs]↗{{hi:regex}} for regular expressions,
- `fancy-regex` if you need features, such as backtracking, which regex doesn't support,
- [`aho-corasick`][c~aho-corasick~docs]↗{{hi:aho-corasick}} for finding occurrences of many patterns at once,
- [`strsim`][c~strsim~docs]↗{{hi:strsim}} for string similarity.


### Cryptography

See [Cryptography](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/cryptography/index.md), [Cryptography Utilities](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/cryptography/cryptography_utilities.md), and [Encryption](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/cryptography/encryption.md).

### Data Processing

See [[data-processing | Data Processing]].

### Numerical and Mathematical Algorithms

For [[mathematics | Mathematics]], use the following crates for general numerical tasks:

- Abstracting over different number types:
  - `num-traits`,
  - [`num`][c~num~docs]↗{{hi:num}}.
- [[additional_numeric_types | Additional Numeric Types]]:
  - `num-bigint` and `rug` for big integers.
  - [`rust_decimal`][c~rust_decimal~docs]↗ for fixed-precision decimal arithmetic with a 96-bit mantissa; use `bigdecimal` when arbitrary precision is required.
  - `ordered-float` for sortable floats.
- [[complex_numbers | Complex Numbers]].
- [[linear_algebra | Linear Algebra]]:
  - [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}.
  - [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}.
- Data frames:
  - `polars`.
  - [`datafusion`][c~datafusion~docs]↗ is an extensible query engine with SQL and DataFrame APIs.
- [[statistics | Statistics]],
- [[trigonometry | Trigonometry]].
- Others
  - [`rustfft`][rustfft~crate] for Fast Fourier Transforms in Rust.
  - [`fftw`][fftw~docs], Rust bindings for the native FFTW library.
  - `roots` for numerical root finding.

### Science

See [Science](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science/index.md), [Machine Learning](https://github.com/john-cd/rust_howto/blob/develop/later/src/other/machine-learning/index.md), [Geoscience](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science_geo/index.md), [Neuroscience](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science_neuroscience/index.md), [Science Robotics](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science_robotics/index.md), and [Simulation](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/simulation/index.md).

## References

- [`rust-algorithms`][rust-algorithms~repo]↗: Common data structures and algorithms in Rust.

See also [String Concatenation](../text-processing/string_concat.md), [String Encoding](../encoding/string_encoding.md), [String Parsing](../text-processing/string_parsing.md), and [Text Processing](../text-processing/index.md).
[Compression](../compression/index.md), [cryptography](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/cryptography/index.md), [[data-structures | data structures]], [[mathematics | mathematics]], [scientific algorithms](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science/index.md), [search](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/database/search.md), [[strings | strings]], and [text processing](../text-processing/index.md) are discussed elsewhere but are briefly mentioned below.
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
