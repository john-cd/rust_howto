# Data Structures

[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

Common data structures include [[data_types | arrays]] and [[vectors | vectors]], which are contiguous blocks of memory that store elements of the same data type; [[stacks_and_queues | stacks]] ("last-in, first-out" (LIFO) data structures); [[stacks_and_queues | queues]] ("first-in, first-out" (FIFO) data structures); [[linked_lists | linked lists]] (sequences of nodes, where each node contains data and a pointer to the next node); [[hashmaps | maps]] (aka dictionaries, which store key-value pairs); [[hashmaps | sets]] (collections of unique elements); [[b-trees | trees]] (hierarchical data structures consisting of nodes with parent-child relationships); and [[graphs | graphs]].

The `std::collections` module of the standard library includes:

- [[vectors | `Vec<T>`]] (Vector) is a dynamic array that can grow or shrink as needed. It's the most commonly used collection in Rust and is similar to a dynamic array or list in other languages. It provides fast access by index.
- [[hashmaps | `HashMap<K, V>`]] (Hash Map) stores key-value pairs, allowing for efficient lookup of values based on their keys. It uses a hash function for fast average-case access.
- [[b-trees | `BTreeMap<K, V>`]] (B-Tree Map) is similar to `HashMap`, but it keeps the keys sorted. It provides ordered access to key-value pairs.
- [[hashmaps | `HashSet<T>`]] (Hash Set) stores a collection of unique elements. Used for efficiently checking membership and ensuring uniqueness.
- [[b-trees | `BTreeSet<T>`]] (B-Tree Set) is similar to `HashSet`, but it keeps the elements sorted.
- [[linked_lists | `LinkedList<T>`]] is a doubly linked list. Useful for frequent insertions and deletions at arbitrary positions, but less efficient for random access.
- [[stacks_and_queues | `VecDeque<T>`]] (Vector Deque) is a double-ended queue. Allows efficient insertion and deletion at both ends.
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

## Perfect Hash Functions

{{#include phf.incl.md}}

## Append-only Collections

{{#include elsa.incl.md}}

## Key Equivalence with `equivalent`

[![equivalent][c~equivalent~docs~badge]][c~equivalent~docs] [![equivalent~crates.io][c~equivalent~crates.io~badge]][c~equivalent~crates.io] [![equivalent~repo][c~equivalent~repo~badge]][c~equivalent~repo]{{hi:equivalent}}

[`equivalent`][c~equivalent~docs]↗{{hi:equivalent}} provides the `Equivalent` trait, which allows comparing two values of different types that represent logically equivalent keys. This is useful when working with map APIs where you want to look up an entry using a borrowed form of the key (e.g., looking up a `String` key using a `&str`).

This pattern is used internally by [`indexmap`][c~indexmap~docs]↗{{hi:indexmap}} to allow lookups with borrowed key types without requiring an owned copy:

```rust,editable,noplayground
// The `equivalent` crate is most useful when implementing custom map types.
// For standard use, Rust's standard library already supports this pattern:
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("hello".to_string(), 1);

    // You can look up a `String` key using a `&str`:
    // (This works because `HashMap` uses the `Borrow` trait.)
    let value = map.get("hello");
    println!("{:?}", value); // Some(1)
}
```

## Converting Between Collection Types

A common task is to convert between Rust's built-in collection types. Rust's iterator API makes these conversions ergonomic:

```rust,editable
use std::collections::{HashMap, HashSet};

fn main() {
    // Vec to HashSet (removes duplicates):
    let numbers = vec![1, 2, 3, 2, 1];
    let unique: HashSet<i32> = numbers.into_iter().collect();
    println!("Unique: {:?}", unique);

    // HashSet to Vec:
    let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let mut v: Vec<i32> = set.into_iter().collect();
    v.sort(); // HashSet has no guaranteed order
    println!("Vec: {:?}", v);

    // Vec of tuples to HashMap:
    let pairs = vec![("one", 1), ("two", 2), ("three", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("Map: {:?}", map);

    // HashMap to Vec of tuples:
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    let mut pairs: Vec<(&str, i32)> = map.into_iter().collect();
    pairs.sort_by_key(|(k, _)| *k);
    println!("Pairs: {:?}", pairs);

    // Vec to HashMap (index -> value):
    let words = vec!["apple", "banana", "cherry"];
    let indexed: HashMap<usize, &str> = words.into_iter().enumerate().collect();
    println!("Indexed: {:?}", indexed);
}
```

See [Converting between different collection types][blog~converting-between-different-collection-types]↗ for more details.

## Binary Data: Reading and Writing with `scroll`

`scroll` is a library for efficiently reading and writing primitive types from byte arrays and other data containers. It is particularly useful for parsing binary protocols, file formats, and network packets.

Key features of `scroll`:
- Zero-copy reads via the `Pread` trait, which reads from a buffer at a given offset without advancing a cursor.
- Automatic endianness handling for both little-endian (`LE`) and big-endian (`BE`) byte orders.
- Support for reading into user-defined types via the `TryFromCtx` trait.

## Algorithms on Data Structures

[`rust-algorithms`][rust-algorithms~repo]↗ is a community resource providing implementations of common data structures and algorithms in Rust. It is useful for learning and reference.

## Immutable Data Structures

Immutable (persistent) data structures share structure between versions, allowing efficient copies and updates without mutating the original. They are particularly useful in functional programming styles and concurrent contexts where shared state is needed without locks.

- [`im`][c~im~docs]↗{{hi:im}}: Provides immutable versions of `HashMap`, `HashSet`, `Vector`, `OrdMap`, and `OrdSet`. Uses structural sharing for efficient persistent updates.
- [`rpds`][c~rpds~docs]↗{{hi:rpds}}: Rust Persistent Data Structures - a collection of immutable, persistent data structures including `Stack`, `Queue`, `List`, `Vector`, `HashTrieMap`, `HashTrieSet`, `RedBlackTreeMap`, and `RedBlackTreeSet`.

See [[functional_programming | Functional Programming]].

## Specialized Data Structures

- Trees and Tries: [`indextree`][c~indextree~docs]↗{{hi:indextree}} provides an arena-based tree structure where each node can have multiple children. Useful for representing hierarchical data without cycles.
- Bloom Filters: [`bloomfilter`][c~bloomfilter~docs]↗{{hi:bloomfilter}} implements a space-efficient probabilistic data structure for set membership queries. It can return false positives but never false negatives.
- Skip Lists: [`skiplist`][c~skiplist~docs]↗{{hi:skiplist}} is a data structure that allows for fast search, insertion, and deletion within an ordered sequence of elements.
- Ranges as keys: [`rangemap`][c~rangemap~docs]↗{{hi:rangemap}} stores key-value pairs where keys are ranges. Useful for tasks like mapping IP ranges to regions or time intervals to events.
- Matrices and Tensors: [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} provides an n-dimensional array for numerical computation. See [[linear_algebra | Linear Algebra]].

## Additional Data Structures

- [[data_types | Data Types]].
- [[smart_pointers | Smart Pointers]].
- [[dataframes | Dataframes]]: tabular data structures for data analysis, similar to pandas in Python.
- [[concurrent_data_structures | Concurrent Data Structures]]: thread-safe collections and lock-free data structures.

## Related Topics

- [[serde | Serialization / Deserialization]].
- [[algorithms | Algorithms]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[data-structures: expand](https://github.com/john-cd/rust_howto/issues/280)
</div>
