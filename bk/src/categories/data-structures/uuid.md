# UUIDs

{{#include uuid.incl.md}}

A UUID is a unique 128-bit value, stored as 16 octets, and regularly formatted as a hex string in five groups. UUIDs are used to assign unique identifiers to entities without requiring a central allocating authority. They are particularly useful in distributed systems, though can be used in disparate areas, such as [databases][p~databases] and network protocols.

## Generate and Parse UUIDs {#generate-parse-uuid}

[![uuid][c~uuid~docs~badge]][c~uuid~docs]{{hi:uuid}}
[![uuid~crates.io][c~uuid~crates.io~badge]][c~uuid~crates.io]
[![uuid~github][c~uuid~github~badge]][c~uuid~github]
[![uuid~lib.rs][c~uuid~lib.rs~badge]][c~uuid~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`uuid`][c~uuid~docs]↗{{hi:uuid}} generates and parses UUIDs and implements a number of utility functions.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/uuid.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[uuid: write](https://github.com/john-cd/rust_howto/issues/283)

- [ulid](https://docs.rs/ulid/latest/ulid)

Universally Unique Lexicographically Sortable Identifiers.

</div>
