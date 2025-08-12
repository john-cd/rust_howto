# Data Structures

[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

Common data structures include [[data_types | arrays]] and [[vectors | vectors]], which are contiguous block of memory that stores elements of the same data type; [[linkedlist |linked lists]] (sequences of nodes, where each node contains data and a pointer to the next node); [[stack_and_queue | stacks]] ("last-in, first-out" (LIFO) data structures); [[stack_and_queue | queues]] ("first-in, first-out" (FIFO) data structures); [[btrees | trees]] (hierarchical data structures consisting of nodes with parent-child relationships); sets (collections of unique elements); and [[hashmaps | maps]] (aka dictionaries, which store key-value pairs).

Standard library collections (in `std::collections`) include:

- [[vectors | `Vec<T>`]] (Vector) is a dynamic array that can grow or shrink as needed. It's the most commonly used collection in Rust and is similar to a dynamic array or list in other languages. Provides fast access by index.
- [[strings | `String`]] is a growable, UTF-8 encoded string. It's a fundamental data structure for working with text.
- [[hashmaps | `HashMap<K, V>`]] (Hash Map) stores key-value pairs, allowing for efficient lookup of values based on their keys. Uses a hash function for fast average-case access.
- [[btrees | `BTreeMap<K, V>`]] (B-Tree Map) is similar to `HashMap`, but it keeps the keys sorted. Provides ordered access to key-value pairs.
- [[hashmaps | `HashSet<T>`]] (Hash Set) stores a collection of unique elements. Used for efficiently checking membership and ensuring uniqueness.
- [[btrees | `BTreeSet<T>`]] (B-Tree Set) is similar to `HashSet`, but it keeps the elements sorted.
- [[linkedlist | `LinkedList<T>`]] is a doubly linked list. Useful for frequent insertions and deletions at arbitrary positions, but less efficient for random access.
- [[stack_and_queue | `VecDeque<T>`]] (Vector Deque) is a double-ended queue. Allows efficient insertion and deletion at both ends.
- [[binaryheap | `BinaryHeap<T>`]] is a binary heap, often used to implement a priority queue.

Refer to the [[language | Language]] and [[standard-library | Standard Library]] sections for examples of use.

## Vectors

{{#include vector.incl.md}}

## HashMap

{{#include hashmap.incl.md}}

## Strings

{{#include strings.incl.md}}

## Code Examples

The following examples cover less commonly-used data structures:

### Stack-allocated Arrays

{{#include stack_allocated_arrays.incl.md}}

### Ordered Maps, MultiMaps, and Friends

{{#include maps.incl.md}}

### BTreeMap and BTreeSet

{{#include btrees.incl.md}}

### Stacks and Queues

{{#include stack_and_queue.incl.md}}

### Linked Lists

{{#include linkedlist.incl.md}}

### Binary Heaps / Priority Queues

{{#include binaryheap.incl.md}}

### Graphs

{{#include graph.incl.md}}

### Bitflags and Fields

{{#include bitfield.incl.md}}

### UUIDs

{{#include uuid.incl.md}}

### Heapless Data Structures

{{#include heapless.incl.md}}

## Additional Data Structures

- [[data_types | Data Types]].
- [[smart_pointers | Smart Pointers]].
- Immutable Data Structures: [`im`][c~im~docs]↗{{hi:im}}, [`rpds`][c~rpds~docs]↗{{hi:rpds}}. See [[functional_programming | Functional Programming]].
- Specialized Data Structures:
  - Graphs: [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}}, [`graph-rs`][c~graph-rs~docs]↗{{hi:graph-rs}}.
  - Trees and Tries: [`indextree`][c~indextree~docs]↗{{hi:indextree}}.
  - Bloom Filters: [`bloomfilter`][c~bloomfilter~docs]↗{{hi:bloomfilter}}.
  - Skip Lists: [`skiplist`][c~skiplist~docs]↗{{hi:skiplist}}.
  - Ranges as keys: [`rangemap`][c~rangemap~docs]↗{{hi:rangemap}} stores key-value pairs where keys are ranges.
  - Matrices and Tensors: [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} provides an n-dimensional array for numerical computation.
  - [[dataframes | Dataframes]].
  - [[concurrent_data_structures | Concurrent Data Structures]].
  - Bitsets: [`roaring`][c~roaring~docs]↗{{hi:roaring}} implements compressed bitsets.

## Related Topics

- [[serde | Serialization / Deserialization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[data-structures: expand](https://github.com/john-cd/rust_howto/issues/280)

$1↗$2 [`rust-algorithms`: Common data structures and algorithms in Rust][rust-algorithms~github]↗.
- [PHF - data structures in Rust][c~phf~lib.rs]↗: Runtime support for perfect hash function data structures.
- [`PriorityQueue` - data structures in Rust][c~priority-queue~lib.rs]↗.
- [`elsa` - data structures in Rust][c~elsa~lib.rs]↗.
- [`Equivalent` - data structures in Rust][c~equivalent~lib.rs]↗.

</div>
