# Algorithms

[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

This category covers Rust implementations of core algorithms{{hi:Algorithms}}, such as sorting{{hi:Sorting}} and random value generation, that are not covered in separate chapters.

[[compression | Compression]], [[cryptography | cryptography]], [[data-structures | data structures]], [[mathematics | mathematics]], [[science | scientific algorithms]], [[search | search]], [[strings | strings]], and [[text-processing | text processing]] are discussed elsewhere but are briefly mentioned below.

## Random Value Generation

{{#include random_value_generation.incl.md}}

## Sorting

{{#include sorting.incl.md}}

## Hashing

{{#include hashing.incl.md}}

## Related Topics

### Compression Algorithms

For [[compression | compression]], use:

- `flate2` for deflate, gzip, and zlib compression,
- `zip` for zip files,
- `tar` for tar archives,
- `zstd` for Zstandard compression,
- `bzip2`,
- `xz2`,
- `snap` for Snappy compression.
- `brotli` for Brotli compression.

### Data Structures

For common [[data-structures | data structures]] ([`Vec`][c~std::vec::Vec~docs]↗, [`HashMap`][c~std::collections::HashMap~docs]↗, etc.), you will use [`std::collections`][c~std::collections~docs]↗. You may also consider:

- [`im`][c~im~docs]↗{{hi:im}} for immutable data structures,
- `indexmap` for [[maps | maps]] that keep track of insertion order,
- `arrayvec`, `smallvec`, `tinyvec` and `heapless` for [[stack-allocated | stack-allocated data structures]],
- [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}} for [[graph | graphs]],
- `dashmap` and `papaya` for [[concurrent_data_structures | concurrent data structures]],
- `parking_lot` for mutexes,
- `crossbeam-channel`, `flume`, `tokio`, and `postage` for channels.

Worth a mention: `fst` provides fast, memory-efficient, immutable set and map data structures for storing and searching (very) large collections of strings and associated values. It is based on finite state transducers (FSTs). The capabilities of ordered sets and maps mirror that of `BTreeSet` and `BTreeMap` found in the standard library. The key difference is that sets and maps in `fst` are immutable, keys are byte sequences, and values, in the case of maps, are always unsigned 64 bit integers.

See [[vector | vector]] [[hashmap | hash maps]], [[maps | other maps]], [[binaryheap | Binary Heap]] within the [[data-structures | Data Structures]] section for more details.

### Recursive Algorithms

- `stacker` is a stack growth library useful when implementing deeply recursive algorithms that may accidentally blow the stack.

### Search Algorithms

For small-scale tasks, like filtering a list of items in memory, reach first for the standard library:

- `slice::binary_search*` for slices and vectors,
- `std::iter::Iterator::find` for iterators,
- `str::find`, `str::rfind` for strings.

- `argminmax` provides efficient argmin & argmax (in 1 function) with SIMD for floats and integers.
- `bytecount` count occurrences of a given byte, or the number of UTF-8 code points, in a byte slice, fast.

[meilisearch][c~meilisearch~repo]↗ and [tantivy][c~tantivy~repo]↗ are full-text search engines written in Rust, similar to what you'd expect from `Elasticsearch` or `Algolia`.

### String Algorithms and Text Processing

For [[strings | Strings]], use:

- [`regex`][c~regex~docs]↗{{hi:regex}} for regular expressions,
- `fancy-regex` if you need features, such as backtracking, which regex doesn't support,
- [`aho-corasick`][c~aho-corasick~docs]↗{{hi:aho-corasick}} for finding occurrences of many patterns at once,
- [`strsim`][c~strsim~docs]↗{{hi:strsim}} for string similarity.

See also [[string_concat | String Concat]], [[string_encoding | String Encoding]], [[string_parsing | String Parsing]], and [[text-processing | Text Processing]].

### Numerical and Mathematical Algorithms

For [[mathematics | Mathematics]], use the following crates for general numerical tasks:

- Abstracting over different number types:
  - `num-traits`,
  - [`num`][c~num~docs]↗{{hi:num}}.
- [[additional_numeric_types | Additional Numeric Types]]:
  - `num-bigint` and `rug` for big integers.
  - `rust_decimal` for big decimals.
  - `ordered-float` for sortable floats.
- [[complex_numbers | Complex Numbers]].
- [[linear_algebra | Linear Algebra]]:
  - [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}.
  - [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}.
- Data frames:
  - `polars`.
  - `datafusion`.
- [[statistics | Statistics]],
- [[trigonometry | Trigonometry]].
- Others
  - `rustfft` for Fast Fourier Transforms.
  - `roots` for numerical root finding.

## References

- [`rust-algorithms`][rust-algorithms~repo]↗: Common data structures and algorithms in Rust.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/1165)

review [rust-algorithms~repo][rust-algorithms~repo].

| Topic | Related Links |
|---|---|
| [[compression | Compression]] | |
| [[cryptography | Cryptography]] | [[cryptography_utilities | Cryptography Utilities]], [[encryption | Encryption]],
| [[data-processing | Data Processing]] | |
| [[data-structures | Data Structures]] | [[concurrent_data_structures | Concurrent Data Structures]] |
| [[science | Science]] | [[_machine_learning | Machine Learning]], [[science_geo | Geoscience]], [[science_neuroscience | Neuroscience]], [[science_robotics | Science Robotics]], [[simulation | Simulation]] |
| [[search | Search]] | [[rust_search_engines | Search Engines]] |
| [[strings | Strings]] | [[string_concat | String Concat]], [[string_encoding | String Encoding]], [[string_parsing | String Parsing]], [[text-processing | Text Processing]] |

See [[rust_search_engines | Search Engines]].

</div>
