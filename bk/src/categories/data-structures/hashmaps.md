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

[![fnv][c~fnv~docs~badge]][c~fnv~docs] [![fnv~crates.io][c~fnv~crates.io~badge]][c~fnv~crates.io] [![fnv~repo][c~fnv~repo~badge]][c~fnv~repo] [![fnv~lib.rs][c~fnv~lib.rs~badge]][c~fnv~lib.rs]{{hi:fnv}}

You can use a custom hash function with [`HashMap`][c~std::collections::HashMap~docs]↗ and [`HashSet`][c~std::collections::HashSet~docs]↗. In the following, the {{i:Fowler-Noll-Vo hash function}} is used for better performance with short keys.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/custom_hash_function.rs:example}}
```

## Choose a Faster Hasher: `ahash` and `rustc-hash` {#fast-hashmap}

[![ahash][c~ahash~docs~badge]][c~ahash~docs] [![ahash~crates.io][c~ahash~crates.io~badge]][c~ahash~crates.io] [![ahash~repo][c~ahash~repo~badge]][c~ahash~repo] [![ahash~lib.rs][c~ahash~lib.rs~badge]][c~ahash~lib.rs]{{hi:ahash}}{{hi:AHashMap}}{{hi:Performance}} [![rustc-hash][c~rustc-hash~docs~badge]][c~rustc-hash~docs] [![rustc-hash~crates.io][c~rustc-hash~crates.io~badge]][c~rustc-hash~crates.io] [![rustc-hash~repo][c~rustc-hash~repo~badge]][c~rustc-hash~repo] [![rustc-hash~lib.rs][c~rustc-hash~lib.rs~badge]][c~rustc-hash~lib.rs]{{hi:rustc-hash}}{{hi:FxHashMap}}{{hi:Hash}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The example counts words with three maps: the standard library's [`HashMap`][c~std::collections::HashMap~docs]↗, [`ahash::AHashMap`][c~ahash~docs]↗{{hi:AHashMap}} and [`rustc_hash::FxHashMap`][c~rustc-hash~docs]↗{{hi:FxHashMap}}. The trade-offs:

- `HashMap` uses SipHash, which resists HashDoS (attacker-chosen keys that all collide). Keep it for keys that come from untrusted input.
- `AHashMap` is much faster on strings and other larger keys, and is randomly seeded by default.
- `FxHashMap` is the fastest for small integer or enum keys. It is deterministic (not seeded) and has weak distribution for some key patterns.

[`hashbrown`][c~hashbrown~docs]↗{{hi:hashbrown}} (the table behind the standard library's `HashMap`) and [`foldhash`][c~foldhash~docs]↗{{hi:foldhash}} are other options. Measure with [`criterion`][c~criterion~docs]↗{{hi:criterion}} before switching.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/hashmap/fast_hashmap.rs:example}}
```

## Related Data Structures {#related-data-structures .skip}

- [[btrees | B-trees]].
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

[accuracy review: data structures](https://github.com/john-cd/rust_howto/issues/1892)

</div>
