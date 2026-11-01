<!-- TODO Reviewed ??-??-2026 -->

# Unique Identifiers

{{#include unique_identifiers.incl.md}}

## Generate and Parse UUIDs {#generate-parse-uuid}

{{#badges uuid wasm=WebAssembly data-structures="Data structures" no-std="No standard library" parser-implementations="Parser implementations"}}{{hi:Guid}}{{hi:Unique}}{{hi:uuid}}

A UUID is a unique 128-bit value, stored as 16 octets, and regularly formatted as a hex string in five groups. UUIDs are used to assign unique identifiers to entities without requiring a central allocating authority. They are particularly useful in distributed systems, though can be used in disparate areas, such as [[databases]] and network protocols.

[`uuid`][c~uuid~docs]↗{{hi:uuid}} generates and parses UUIDs and implements a number of utility functions.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/unique_ids/uuid.rs:example}}
```

## Generate and Parse ULIDs {#ulid}

{{#badges ulid}}{{hi:Identifier}}{{hi:Sortable}}{{hi:ulid}}{{hi:Uuid}}

ULID stands for Universally Unique Lexicographically Sortable Identifier. Like a UUID, it stores a 128-bit value. Its canonical text form uses 26 Crockford Base32 characters, compared with 36 characters for a hyphenated UUID, so it is more compact only in text form. The first 48 bits encode a Unix timestamp in milliseconds, and the remaining 80 bits are random to reduce collision risk. UUIDv7 is another time-ordered identifier format; the [`uuid`][c~uuid~docs]↗ crate supports it when its `v7` feature is enabled. The [`ulid`][c~ulid~docs]↗ crate implements ULIDs in Rust:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/unique_ids/ulid.rs:example}}
```

## Generate Compact UUIDs with `short-uuid` {#short-uuid}

{{#badges short-uuid data-structures=}}

[`short-uuid`][c~short-uuid~docs]↗{{hi:short-uuid}} encodes a UUID in a shorter, URL-safe string. It converts losslessly to and from a regular [`uuid::Uuid`][c~uuid~docs]↗. Use it for identifiers that appear in URLs or are typed by people. For time-ordered identifiers, consider UUID version 7 or a ULID (see above); for compact random identifiers without UUID structure, consider `nanoid`.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/unique_ids/short_uuid.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[maps | Maps]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: data structures
</div>
