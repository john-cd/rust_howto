# Algorithms

[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

This category covers Rust implementations of core algorithms{{hi:Algorithms}}, such as sorting{{hi:Sorting}} (e.g., merge sort, quicksort), searching{{hi:Searching}}, hashing{{hi:Hashing}}, and more.

The following table denotes the most common crates used for each algorithm category.

| Topic | Rust Crates | Notes |
| --- | --- | --- |
| Sorting | [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html)↗{{hi:std::cmp}}, [`itertools`][c~itertools~docs]↗{{hi:itertools}}, [`rand`][c~rand~docs]↗{{hi:rand}} | `std::cmp` provides ordering traits, [`itertools`][c~itertools~docs]↗{{hi:itertools}} for iterators, [`rand`][c~rand~docs]↗{{hi:rand}} for generating test data. No single crate is the sorting crate, as `std::slice::sort` is often sufficient. |
| Searching | [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html)↗{{hi:std::cmp}} | Binary search is available in the standard library. |
| Data Structures (General) | [`std::collections`](https://doc.rust-lang.org/std/collections/index.html)↗ for common structures ([`Vec`][c~std::vec::Vec~docs]↗, [`HashMap`][c~std::collections::HashMap~docs]↗, etc.), [`im`][c~im~docs]↗{{hi:im}}, [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}} | `std::collections` for common structures (Vec, HashMap, etc.), [`im`][c~im~docs]↗{{hi:im}} for immutable data structures, [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}} for graphs. |
| Graph Algorithms | [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}}, [`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} | [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}} is a popular graph library. [`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} provides pathfinding algorithms. |
| Dynamic Programming | Often implemented without external crates | DP is often implemented using standard library features like vectors and iterators. |
| String Algorithms | [`regex`][c~regex~docs]↗{{hi:regex}}, [`aho-corasick`][c~aho-corasick~docs]↗{{hi:aho-corasick}}, [`strsim`][c~strsim~docs]↗{{hi:strsim}} | [`regex`][c~regex~docs]↗{{hi:regex}} for regular expressions, [`aho-corasick`][c~aho-corasick~docs]↗{{hi:aho-corasick}} for multiple pattern searching, [`strsim`][c~strsim~docs]↗{{hi:strsim}} for string similarity. |
| Numerical Algorithms | [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}, [`num`][c~num~docs]↗{{hi:num}} | [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} for linear algebra, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} for N-dimensional arrays, [`num`][c~num~docs]↗{{hi:num}} for numeric traits. |
| Cryptography | [`ring`][c~ring~docs]↗{{hi:ring}}, [`rust-crypto`][c~rust-crypto~docs]↗{{hi:rust-crypto}}, [`sha2`][c~sha2~docs]↗{{hi:sha2}} | Several crates exist; choose carefully based on security needs and audit history. |

## Random Numbers

{{#include randomness.incl.md}}

## Sorting

{{#include sorting.incl.md}}

## Related Topics

| Topic | Related Links |
|---|---|
| [[compression | Compression]] | |
| [[cryptography | Cryptography]] | [[cryptography_utilities | Cryptography Utilities]], [[encryption | Encryption]], [[hashing | Hashing]], [[password_hashing | Password Hashing]] |
| [[data-processing | Data Processing]] | |
| [[data-structures | Data Structures]] | [[concurrent_data_structures | Concurrent Data Structures]] |
| [[mathematics | Mathematics]] | [[additional_numeric_types | Additional Numeric Types]], [[complex_numbers | Complex Numbers]], [[linear_algebra | Linear Algebra]], [[statistics | Statistics]], [[trigonometry | Trigonometry]]. |
| [[science | Science]] | [[_machine_learning |  Machine Learning]], [[science_geo | Geoscience]], [[science_neuroscience | Neuroscience]], [[science_robotics | Science Robotics]], [[simulation | Simulation]] |
| [[search | Search]] | [[rust_search_engines | Search Engines]] |
| [[strings | Strings]] | [[string_concat | String Concat]], [[string_encoding | String Encoding]], [[string_parsing | String Parsing]], [[text-processing | Text Processing]] |

## References

- [`rust-algorithms`: Common data structures and algorithms in Rust](https://github.com/EbTech/rust-algorithms)↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review; add graph examples](https://github.com/john-cd/rust_howto/issues/1165)

</div>
