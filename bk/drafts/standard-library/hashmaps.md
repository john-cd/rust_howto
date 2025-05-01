# Hashmaps

{{#include hashmaps.incl.md}}

## HashMap {#hashmap}

[![std][c-std-badge]][c-std]{{hi:std}}

HashMap is a key-value data structure. It allows you to store data in an unordered collection, where each element is identified by a unique key. This makes HashMap an excellent choice for lookups, insertions, and deletions based on keys.

All of the hashmap{{hi:Hashmap}} keys{{hi:Keys}} must have the same type as each other, and all of the values{{hi:Values}} must have the same type.

```rust,editable
{{#include ../../crates/standard_library/tests/hashmaps/hashmaps.rs:example}}
```

## HashSet {#hashset}

[![std][c-std-badge]][c-std]{{hi:std}}

```rust,editable
{{#include ../../crates/standard_library/tests/hashmaps/hashset.rs:example}}
```

## HashMap and HashSet with Custom Hash Function {#custom-hash-function}

[![std][c-std-badge]][c-std]{{hi:std}}

```rust,editable
{{#include ../../crates/standard_library/tests/hashmaps/custom_hash_function.rs:example}}
```

## HashMap Using a Custom Type as the Key {#custom-key-type}

[![std][c-std-badge]][c-std]{{hi:std}}

```rust,editable
{{#include ../../crates/standard_library/tests/hashmaps/custom_type_as_key.rs:example}}
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
[hashmaps: add NOW](https://github.com/john-cd/rust_howto/issues/622).
</div>
