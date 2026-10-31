<!-- TODO Reviewed ??-??-2026 -->

# Hash Maps and Sets {#hashmaps-and-sets}

{{#include hashmaps.incl.md}}

## Store Key-Value Pairs into a HashMap {#hashmap}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

[`HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}} is a key-value data structure. It allows storing data in an unordered collection, where each element is identified by a unique key. This makes `HashMap` an excellent choice for lookups, insertions, and deletions based on keys.

All hashmap keys{{hi:Keys}} must have the same type. All values{{hi:Values}} must have the same type.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/hashmap.rs:example}}
```

## Store Unique Items in a HashSet {#hashset}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

[`HashSet`][c~std::collections::HashSet~docs]↗{{hi:std::collections::HashSet}} is a common data structure that stores a collection of unique items, similar to the keys of a `HashMap` but without associated values.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/hashset.rs:example}}
```

## Use a Custom Type as the Key of a HashMap {#custom-key-type}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

You can use a custom-defined type (typically a `struct`) as keys in a HashMap. It is useful to associate data with complex, multi-component identifiers that don't fit naturally into a single primitive type like an integer or a string.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/custom_type_as_key.rs:example}}
```

## Use a Custom Hash Function with `HashMap` and `HashSet` {#custom-hash-function}

{{#badges fnv}}

You can use a custom hash function with [`HashMap`][c~std::collections::HashMap~docs]↗ and [`HashSet`][c~std::collections::HashSet~docs]↗. In the following, the {{i:Fowler-Noll-Vo hash function}} is used for better performance with short keys.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/custom_hash_function.rs:example}}
```

## Choose a Faster Hasher: `ahash` and `rustc-hash` {#fast-hashmap}

{{#badges ahash}} {{hi:AHashMap}} {{hi:Performance}} {{#badges rustc-hash}} {{hi:FxHashMap}} {{hi:Hash}} {{#categories data-structures="Data structures"}}

The example counts words with three maps: the standard library's [`HashMap`][c~std::collections::HashMap~docs]↗, [`ahash::AHashMap`][c~ahash~docs]↗{{hi:AHashMap}} and [`rustc_hash::FxHashMap`][c~rustc-hash~docs]↗{{hi:FxHashMap}}. The trade-offs:

- `HashMap` uses SipHash, which resists HashDoS (attacker-chosen keys that all collide). Keep it for keys that come from untrusted input.
- `AHashMap` is much faster on strings and other larger keys, and is randomly seeded by default.
- `FxHashMap` is the fastest for small integer or enum keys. It is deterministic (not seeded) and has weak distribution for some key patterns.

[`hashbrown`][c~hashbrown~docs]↗{{hi:hashbrown}} (the table behind the standard library's `HashMap`) and [`foldhash`][c~foldhash~docs]↗{{hi:foldhash}} are other options. Measure with [`criterion`][c~criterion~docs]↗{{hi:criterion}} before switching.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/fast_hashmap.rs:example}}
```

## Related Data Structures {#related-data-structures .skip}

- [[b-trees | B-trees]].
- [[maps | Other Maps]].

## Related Topics {#related-topics .skip}

- [[data-structures | Data Structures]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- [[concurrent_data_structures | Concurrent Data Structures]].
- [[databases | Databases]].
- [[hashing | Hashing]].
- [[key_value_stores | Key-Value Stores]].

accuracy review: data structures

</div>
