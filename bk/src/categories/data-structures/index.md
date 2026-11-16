<!-- TODO Reviewed ??-??-2026. -->

# Data Structures

[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

Common data structures include [[data_types | arrays]] and [[vectors | vectors]], which store elements of the same type contiguously; [[stacks_and_queues | stacks]] ("last-in, first-out" (LIFO) data structures); [[stacks_and_queues | queues]] ("first-in, first-out" (FIFO) data structures); [[linked_lists | linked lists]] (sequences of nodes that contain data and links to neighboring nodes); [[hashmaps | maps]] (dictionaries that store key-value pairs); [[hashmaps | sets]] (collections of unique elements); [[b-trees | trees]] (hierarchical data structures with parent-child relationships); and [[graphs | graphs]].

The standard library provides these common collections. `Vec` is defined in `std::vec` and `String` in `std::string`; neither is part of `std::collections`:

- [[vectors | `Vec<T>`]] (Vector) is a dynamic array that can grow or shrink as needed. It's the most commonly used collection in Rust and is similar to a dynamic array or list in other languages. It provides fast access by index.
- [[hashmaps | `HashMap<K, V>`]] (Hash Map) stores key-value pairs, allowing for efficient lookup of values based on their keys. It uses a hash function for fast average-case access.
- [[b-trees | `BTreeMap<K, V>`]] (B-Tree Map) is similar to `HashMap`, but it keeps the keys sorted. It provides ordered access to key-value pairs.
- [[hashmaps | `HashSet<T>`]] (Hash Set) stores a collection of unique elements. Used for efficiently checking membership and ensuring uniqueness.
- [[b-trees | `BTreeSet<T>`]] (B-Tree Set) is similar to `HashSet`, but it keeps the elements sorted.
- [[linked_lists | `LinkedList<T>`]] is a doubly linked list with efficient operations at either end. Stable Rust does not provide the cursor API needed for efficient insertions and deletions at arbitrary positions; indexed access also takes O(n) time.
- [[stacks_and_queues | `VecDeque<T>`]] (Vector Deque) is a double-ended queue. Allows efficient insertion and deletion at both ends.
- [[binary_heaps | `BinaryHeap<T>`]] is a binary heap, often used to implement a priority queue.
- [[strings | `String`]] is a growable, UTF-8 encoded string. It's a fundamental data structure for working with text.

Refer to the [[language | Language]] and [[standard-library | Standard Library]] sections for examples of use.

Start with the operation that dominates your workload: indexing for `Vec`, work at both ends for `VecDeque`, sorted traversal for a B-tree, and priority retrieval for `BinaryHeap`. The sections below link to examples of those operations.

## Strings

{{#include strings.incl.md}}

## Vectors

{{#include vectors.incl.md}}

## Stacks and Queues

{{#include stacks_and_queues.incl.md}}

## Stack-allocated Data Structures

{{#include stack-allocated.incl.md}}

## Hashmaps

{{#include hashmaps.incl.md}}

## B-Trees

{{#include b-trees.incl.md}}

## HashMap's Friends: `IndexMap`, `MultiMap`, and `SlotMap`

{{#include other_maps.incl.md}}

## Binary Heaps / Priority Queues

{{#include binary_heaps.incl.md}}

## Graphs

{{#include graphs.incl.md}}

## Linked Lists

{{#include linked_lists.incl.md}}

## Bit Arrays

{{#include bit_arrays.incl.md}}

## Unique Identifiers

{{#include unique_identifiers.incl.md}}

## Perfect Hash Maps

The [`phf`][c~phf~docs]↗{{hi:phf}} crate generates maps and sets for keys known at compile time. It supports `Map`, `Set`, `OrderedMap`, and `OrderedSet`, with corresponding `phf_map!`, `phf_set!`, `phf_ordered_map!`, and `phf_ordered_set!` macros. The generated lookup tables need no runtime construction.

### Use a Compile-time Perfect Hash Map {#phf-map}

{{#include phf.incl.md}}

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/phf/perfect_hash_map.rs:example}}
```

## Append-only Collections with `elsa`

[`elsa`][c~elsa~docs]↗{{hi:elsa}} provides append-only collections such as `FrozenVec` and `FrozenMap`. They allow insertion through shared references but do not support removing or replacing entries. Stored values must implement `StableDeref` so references to their targets remain valid as the collection grows. For example, `Box<T>` keeps its allocation stable even if the vector reallocates and moves its `Box` values; the vector itself may still reallocate.

### Use Append-only Frozen Collections {#elsa-frozen-vec}

{{#include elsa.incl.md}}

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/elsa/frozen_vec.rs:example}}
```

`FrozenMap<K, Box<V>>` uses the same stable-target property for values:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/elsa/frozen_map.rs:example}}
```

## Additional Data Structures

- [[data_types | Data Types]].
- [[smart_pointers | Smart Pointers]].
- Immutable Data Structures: [`im`][c~im~docs]↗{{hi:im}}, [`rpds`][c~rpds~docs]↗{{hi:rpds}}. See [Functional Programming](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/rust-patterns/functional_programming.md).
- Specialized Data Structures:
  - Trees and Tries: [`indextree`][c~indextree~docs]↗{{hi:indextree}}.
  - Bloom Filters: [`bloomfilter`][c~bloomfilter~docs]↗{{hi:bloomfilter}}.
  - Skip Lists: [`skiplist`][c~skiplist~docs]↗{{hi:skiplist}}.
  - Ranges as keys: [`rangemap`][c~rangemap~docs]↗{{hi:rangemap}} stores key-value pairs where keys are ranges.
  - Matrices and Tensors: [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} provides an n-dimensional array for numerical computation. See [[linear_algebra | Linear Algebra]].

## Related Topics

- [Serialization / Deserialization](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/encoding/serde.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

## See Also

- [`rust-algorithms`][rust-algorithms~repo]↗: Common data structures and algorithms in Rust.
- [PHF][c~phf~lib.rs]↗: Runtime support for perfect hash function data structures.
- [`PriorityQueue`][c~priority-queue~lib.rs]↗.
- [`elsa`][c~elsa~lib.rs]↗.
- [`equivalent`][c~equivalent~lib.rs]↗.
- [Converting between different collection types][blog~converting-between-different-collection-types]↗: from `Vec` to `HashSet` or `HashMap`.
- [[dataframes | Dataframes]].
- [Concurrent Data Structures](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/concurrency/concurrent_data_structures.md).
- [`scroll`][c~scroll~docs]↗: easily and efficiently read/write types from data containers like byte arrays.
