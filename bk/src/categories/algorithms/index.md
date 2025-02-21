# Algorithms

[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}

Rust implementations of core algorithms{{hi:Algorithms}}, such as hashing{{hi:Hashing}}, sorting{{hi:Sorting}}, searching{{hi:Searching}}, and more.

## Random Numbers

{{#include randomness.incl.md}}

## Sorting

{{#include sorting.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 review; add graph examples; link to text processing and

Sorting and Searching: Implementing or using standard sorting algorithms (merge sort, quicksort) and searching algorithms (binary search).

| Topic | Rust Crates | Notes |
| --- | --- | --- |
| Sorting | `std::cmp`, `itertools`, `rand` | `std::cmp` provides ordering traits, itertools for iterators, rand for generating test data. No single crate is the sorting crate, as `std::slice::sort` is often sufficient. |
| Searching | std::cmp | Binary search is available in the standard library. |
| Data Structures (General) | `std::collections`, `im`, `petgraph` | `std::collections` for common structures (Vec, HashMap, etc.), `im` for immutable data structures, `petgraph` for graphs. |
| Graph Algorithms | `petgraph`, `pathfinding` | `petgraph` is a popular graph library. `pathfinding` provides pathfinding algorithms. |
| Dynamic Programming | (Often implemented without external crates) | DP is often implemented using standard library features like vectors and iterators. |
| String Algorithms | `regex`, `aho-corasick`, `strsim` | `regex` for regular expressions, `aho-corasick` for multiple pattern searching, `strsim` for string similarity. |
| Numerical Algorithms | `nalgebra`, `ndarray`, `num` | `nalgebra` for linear algebra, `ndarray` for N-dimensional arrays, `num` for numeric traits.
| Cryptography | `ring`, `rust-crypto`, `sha2` | Several crates exist; choose carefully based on security needs and audit history. |

</div>
