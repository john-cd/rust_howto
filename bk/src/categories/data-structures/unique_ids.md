# Unique Identifiers

{{#include unique_ids.incl.md}}

## Generate and Parse UUIDs {#generate-parse-uuid}

[![uuid][c~uuid~docs~badge]][c~uuid~docs]{{hi:uuid}}
[![uuid~crates.io][c~uuid~crates.io~badge]][c~uuid~crates.io]
[![uuid~repo][c~uuid~repo~badge]][c~uuid~repo]
[![uuid~lib.rs][c~uuid~lib.rs~badge]][c~uuid~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

A UUID is a unique 128-bit value, stored as 16 octets, and regularly formatted as a hex string in five groups. UUIDs are used to assign unique identifiers to entities without requiring a central allocating authority. They are particularly useful in distributed systems, though can be used in disparate areas, such as [[databases]] and network protocols.

[`uuid`][c~uuid~docs]↗{{hi:uuid}} generates and parses UUIDs and implements a number of utility functions.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/unique_ids/uuid.rs:example}}
```

## Generate and Parse ULIDs {#ulid}

[![ulid][c~ulid~docs~badge]][c~ulid~docs] [![ulid~crates.io][c~ulid~crates.io~badge]][c~ulid~crates.io] [![ulid~repo][c~ulid~repo~badge]][c~ulid~repo] [![ulid~lib.rs][c~ulid~lib.rs~badge]][c~ulid~lib.rs]{{hi:ulid}}{{hi:Identifier}}{{hi:Sortable}}{{hi:ulid}}{{hi:Uuid}}

ULID stands for Universally Unique Lexicographically Sortable Identifier, which is a type of unique identifier that is 128 bits long and can be lexicographically sorted in order of creation. It is designed to avoid some limitations of UUIDs, such as being more compact and allowing for easier sorting. Of the 128-bits, the first 48 are a Unix timestamp in milliseconds. The remaining 80 are random. The first 48 provide for lexicographic sorting and the remaining 80 ensure that the identifier is unique. Canonically, a ULID is represented as a 26 character Crockford Base32-encoded string. [ulid][c~ulid~docs]↗ implements ULIDs in Rust:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/unique_ids/ulid.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
