# Algorithms

[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}

Rust implementations of core algorithms{{hi:Algorithms}}, such as hashing{{hi:Hashing}}, sorting{{hi:Sorting}} (e.g., merge sort, quicksort), searching{{hi:Searching}}, and more.

The following table denotes the most common crates used for each algorithm category.

| Topic | Rust Crates | Notes |
| --- | --- | --- |
| Sorting | `std::cmp`, [`itertools`][c-itertools]⮳{{hi:itertools}}, [`rand`][c-rand]⮳{{hi:rand}} | `std::cmp` provides ordering traits, [`itertools`][c-itertools]⮳{{hi:itertools}} for iterators, [`rand`][c-rand]⮳{{hi:rand}} for generating test data. No single crate is the sorting crate, as `std::slice::sort` is often sufficient. |
| Searching | `std::cmp` | Binary search is available in the standard library. |
| Data Structures (General) | `std::collections`, [`im`][c-im]⮳{{hi:im}}, [`petgraph`][c-petgraph]⮳{{hi:petgraph}} | `std::collections` for common structures (Vec, HashMap, etc.), [`im`][c-im]⮳{{hi:im}} for immutable data structures, [`petgraph`][c-petgraph]⮳{{hi:petgraph}} for graphs. |
| Graph Algorithms | [`petgraph`][c-petgraph]⮳{{hi:petgraph}}, [`pathfinding`][c-pathfinding]⮳{{hi:pathfinding}} | [`petgraph`][c-petgraph]⮳{{hi:petgraph}} is a popular graph library. [`pathfinding`][c-pathfinding]⮳{{hi:pathfinding}} provides pathfinding algorithms. |
| Dynamic Programming | Often implemented without external crates | DP is often implemented using standard library features like vectors and iterators. |
| String Algorithms | [`regex`][c-regex]⮳{{hi:regex}}, [`aho-corasick`][c-aho_corasick]⮳{{hi:aho-corasick}}, [`strsim`][c-strsim]⮳{{hi:strsim}} | [`regex`][c-regex]⮳{{hi:regex}} for regular expressions, [`aho-corasick`][c-aho_corasick]⮳{{hi:aho-corasick}} for multiple pattern searching, [`strsim`][c-strsim]⮳{{hi:strsim}} for string similarity. |
| Numerical Algorithms | [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}, [`ndarray`][c-ndarray]⮳{{hi:ndarray}}, [`num`][c-num]⮳{{hi:num}} | [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} for linear algebra, [`ndarray`][c-ndarray]⮳{{hi:ndarray}} for N-dimensional arrays, [`num`][c-num]⮳{{hi:num}} for numeric traits. |
| Cryptography | [`ring`][c-ring]⮳{{hi:ring}}, [`rust-crypto`][c-rust_crypto]⮳{{hi:rust-crypto}}, [`sha2`][c-sha2]⮳{{hi:sha2}} | Several crates exist; choose carefully based on security needs and audit history. |

## Random Numbers

{{#include randomness.incl.md}}

## Sorting

{{#include sorting.incl.md}}

## Related Topics

### Compression

- [[compression | Compression]].

### Cryptography

- [[cryptography | Cryptography]].
- [[cryptography_utilities | Cryptography Utilities]].
- [[encryption | Encryption]].
- [[hashing | Hashing]].
- [[password_hashing | Password Hashing]].

### Data Processing

- [[data-processing | Data Processing]].

### Data Structures

- [[concurrent_data_structures | Concurrent Data Structures]].
- [[data-structures | Data Structures]].

### Mathematics

- [[additional_numeric_types | Additional Numeric Types]].
- [[complex_numbers | Complex Numbers]].
- [[linear_algebra | Linear Algebra]].
- [[mathematics | Mathematics]].
- [[statistics | Statistics]].
- [[trigonometry | Trigonometry]].

### Science

- [[science | Science]].
- [[_machine_learning |  Machine Learning]].
- [[science_geo | Geoscience]].
- [[science_neuroscience | Neuroscience]].
- [[science_robotics | Science Robotics]].
- [[simulation | Simulation]].

## Search

- [[search | Search]].
- [[rust_search_engines | Search Engines]].

### Strings

- [[string_concat | String Concat]].
- [[string_encoding | String Encoding]].
- [[string_parsing | String Parsing]].
- [[strings | Strings]].
- [[text-processing | Text Processing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review; add graph examples](https://github.com/john-cd/rust_howto/issues/1165)
table for related topics?
Cover [`pathfinding`][c-pathfinding]⮳{{hi:pathfinding}}
</div>
