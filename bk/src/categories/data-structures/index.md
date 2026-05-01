# Data Structures

[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

Common data structures include [[data_types | arrays]] and [[vectors | vectors]], which are contiguous block of memory that stores elements of the same data type; [[stacks_and_queues | stacks]] ("last-in, first-out" (LIFO) data structures); [[stacks_and_queues | queues]] ("first-in, first-out" (FIFO) data structures); [[linked_lists |linked lists]] (sequences of nodes, where each node contains data and a pointer to the next node); [[hashmaps | maps]] (aka dictionaries, which store key-value pairs);  [[hashmaps | sets]] (collections of unique elements); [[b-trees | trees]] (hierarchical data structures consisting of nodes with parent-child relationships); and [[graphs | graphs]].

The `std::collections` module of the standard library includes:

- [[vectors | `Vec<T>`]] (Vector) is a dynamic array that can grow or shrink as needed. It's the most commonly used collection in Rust and is similar to a dynamic array or list in other languages. It provides fast access by index.
- [[hashmaps | `HashMap<K, V>`]] (Hash Map) stores key-value pairs, allowing for efficient lookup of values based on their keys. It uses a hash function for fast average-case access.
- [[b-trees | `BTreeMap<K, V>`]] (B-Tree Map) is similar to `HashMap`, but it keeps the keys sorted. It provides ordered access to key-value pairs.
- [[hashmaps | `HashSet<T>`]] (Hash Set) stores a collection of unique elements. Used for efficiently checking membership and ensuring uniqueness.
- [[btrees | `BTreeSet<T>`]] (B-Tree Set) is similar to `HashSet`, but it keeps the elements sorted.
- [[linkedlist | `LinkedList<T>`]] is a doubly linked list. Useful for frequent insertions and deletions at arbitrary positions, but less efficient for random access.
- [[stack_and_queue | `VecDeque<T>`]] (Vector Deque) is a double-ended queue. Allows efficient insertion and deletion at both ends.
- [[binary_heaps | `BinaryHeap<T>`]] is a binary heap, often used to implement a priority queue.
- [[strings | `String`]] is a growable, UTF-8 encoded string. It's a fundamental data structure for working with text.

Refer to the [[language | Language]] and [[standard-library | Standard Library]] sections for examples of use.

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

## Additional Data Structures

- [[data_types | Data Types]].
- [[smart_pointers | Smart Pointers]].
- Immutable Data Structures: [`im`][c~im~docs]↗{{hi:im}}, [`rpds`][c~rpds~docs]↗{{hi:rpds}}. See [[functional_programming | Functional Programming]].
- Specialized Data Structures:
  - Trees and Tries: [`indextree`][c~indextree~docs]↗{{hi:indextree}}.
  - Bloom Filters: [`bloomfilter`][c~bloomfilter~docs]↗{{hi:bloomfilter}}.
  - Skip Lists: [`skiplist`][c~skiplist~docs]↗{{hi:skiplist}}.
  - Ranges as keys: [`rangemap`][c~rangemap~docs]↗{{hi:rangemap}} stores key-value pairs where keys are ranges.
  - Matrices and Tensors: [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} provides an n-dimensional array for numerical computation. See [[linear_algebra | Linear Algebra]].

## Related Topics

- [[serde | Serialization / Deserialization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[data-structures: expand](https://github.com/john-cd/rust_howto/issues/280)

- [`rust-algorithms`][rust-algorithms~repo]↗: Common data structures and algorithms in Rust.
- [PHF][c~phf~lib.rs]↗: Runtime support for perfect hash function data structures.
- [`PriorityQueue`][c~priority-queue~lib.rs]↗.
- [`elsa`][c~elsa~lib.rs]↗.
- [`equivalent`][c~equivalent~lib.rs]↗.

- [Converting between different collection types][blog~converting-between-different-collection-types]↗: from `Vec` to `HashSet` or `HashMap`.

  - [[dataframes | Dataframes]].
  - [[concurrent_data_structures | Concurrent Data Structures]].

`scroll` is a library for easily and efficiently reading/writing types from data containers like byte arrays.

</div>
