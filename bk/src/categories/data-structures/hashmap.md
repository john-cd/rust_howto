# HashMap

{{#include hashmap.incl.md}}

## Store Key-Value Pairs into a HashMap {#hashmap}

[![std][c-std-badge]][c-std]{{hi:std}}{{hi:Hashmap}}

`HashMap` is a key-value data structure. It allows you to store data in an unordered collection, where each element is identified by a unique key. This makes `HashMap` an excellent choice for lookups, insertions, and deletions based on keys.

All hashmap keys{{hi:Keys}} must have the same type. All values{{hi:Values}} must have the same type.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/hashmap.rs:example}}
```

## Store Unique Items in a HashSet {#hashset}

[![std][c-std-badge]][c-std]{{hi:std}}

`HashSet` is a common data structure that stores a collection of unique items, similar to the keys of a `HashMap` but without associated values.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/hashset.rs:example}}
```

## Use a Custom Type as the Key of a HashMap {#custom-key-type}

[![std][c-std-badge]][c-std]{{hi:std}}

You can use a custom-defined type (typically a `struct`) as keys in a HashMap. It is useful when you need to associate data with complex, multi-component identifiers that don't fit naturally into a single primitive type like an integer or a string.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/custom_type_as_key.rs:example}}
```

## Use a Custom Hash Function with `HashMap` and `HashSet` {#custom-hash-function}

[![fnv][c-fnv-badge]][c-fnv] [![fnv-crates.io][c-fnv-crates.io-badge]][c-fnv-crates.io] [![fnv-github][c-fnv-github-badge]][c-fnv-github] [![fnv-lib.rs][c-fnv-lib.rs-badge]][c-fnv-lib.rs]{{hi:fnv}}

You can use a custom hash function with `HashMap` and `HashSet`. In the following, the Fowler-Noll-Vo hash function is used for better performance with short keys.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/custom_hash_function.rs:example}}
```

## Related Data Structures {#skip}

- [[btrees | B-trees]].
- [[maps | Other Maps]].

## Related Topics {#skip}

- [[concurrent_data_structures | Concurrent Data Structures]].
- [[data-structures | Data Structures]].
- [[databases | Databases]].
- [[hashing | Hashing]].
- [[key_value_stores | Key-Value Stores]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
