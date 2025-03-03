# Data Structures

[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Common data structures include arrays and vectors, which are contiguous block of memory that stores elements of the same data type; linked lists (sequences of nodes, where each node contains data and a pointer to the next node); stacks ("last-in, first-out" (LIFO) data structures); queues ("first-in, first-out" (FIFO) data structures); trees (hierarchical data structures consisting of nodes with parent-child relationships); sets (collections of unique elements); and maps (aka dictionaries, which store key-value pairs).

Standard library collections include:

- `Vec<T>` (Vector) is a dynamic array that can grow or shrink as needed. It's the most commonly used collection in Rust and is similar to a dynamic array or list in other languages. Provides fast access by index.
- `String` is a growable, UTF-8 encoded string. It's a fundamental data structure for working with text.
- `HashMap<K, V>` (Hash Map) stores key-value pairs, allowing for efficient lookup of values based on their keys. Uses a hash function for fast average-case access.
- `BTreeMap<K, V>` (B-Tree Map) is similar to `HashMap`, but it keeps the keys sorted. Provides ordered access to key-value pairs.
- `HashSet<T>` (Hash Set) stores a collection of unique elements. Used for efficiently checking membership and ensuring uniqueness.
- `BTreeSet<T>` (B-Tree Set) is similar to `HashSet`, but it keeps the elements sorted.
- `LinkedList<T>` is a doubly linked list. Useful for frequent insertions and deletions at arbitrary positions, but less efficient for random access.
- `VecDeque<T>` (Vector Deque) is a double-ended queue. Allows efficient insertion and deletion at both ends.
- `BinaryHeap<T>` is a binary heap, often used to implement a priority queue.

## Stack-allocated arrays

{{#include stack_allocated_arrays.incl.md}}

## Hashmaps and friends

{{#include maps.incl.md}}

## BTreeMap and BTreeSet

{{#include btrees.incl.md}}

## Stacks and Queues

{{#include stack_and_queue.incl.md}}

## Bitflags

{{#include bitfield.incl.md}}

## Binary Heaps / Priority Queues

{{#include binaryheap.incl.md}}

## UUIDs

{{#include uuid.incl.md}}

## Heapless

{{#include heapless.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[data-structures: expand (P1)](https://github.com/john-cd/rust_howto/issues/280)

Link to stdlib pages

Additional Data Structures:

[`rangemap`][c-rangemap]⮳{{hi:rangemap}}: Stores key-value pairs where keys are ranges.
[`ndarray`][c-ndarray]⮳{{hi:ndarray}}: Provides an n-dimensional array for numerical computation.
[`roaring`][c-roaring]⮳{{hi:roaring}}: Implements compressed bitsets.

Implementing and using linked lists, deques, binary trees, and other less common but sometimes necessary structures.

Standard Library: `std::collections` (Vec, HashMap, LinkedList, BTreeMap, HashSet, etc.)
Immutable Data Structures: [`im`][c-im]⮳{{hi:im}}, [`rpds`][c-rpds]⮳{{hi:rpds}}
Specialized Data Structures:
Graphs: [`petgraph`][c-petgraph]⮳{{hi:petgraph}}, [`graph_rs`][c-graph_rs]⮳{{hi:graph_rs}}
Trees: [`indextree`][c-indextree]⮳{{hi:indextree}}, `rayon-trie`
Bloom Filters: [`bloomfilter`][c-bloomfilter]⮳{{hi:bloomfilter}}
Skip Lists: [`skiplist`][c-skiplist]⮳{{hi:skiplist}}
Serialization/Deserialization (often used with data structures): [`serde`][c-serde]⮳{{hi:serde}}

---

## `either` {#either}

[![either][c-either-badge]][c-either] [![either-crates.io][c-either-crates.io-badge]][c-either-crates.io] [![either-github][c-either-github-badge]][c-either-github] [![either-lib.rs][c-either-lib.rs-badge]][c-either-lib.rs]{{hi:either}}{{hi:Data-structure}}{{hi:No_std}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

The enum [`Either`][c-either]⮳{{hi:Either}} with variants `Left` and `Right` is a general purpose sum type with two cases.

`Either` has methods that are similar to `Option` and `Result`, and it also implements traits like `Iterator`.

Includes macros `try_left!()` and `try_right!()` to use for short-circuiting logic, similar to how the `?` operator is used with `Result`. Note that `Either` is general purpose. For describing success or error, use the regular `Result`.

</div>
