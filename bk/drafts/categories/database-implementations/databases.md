# Databases Written in Rust

{{#include databases.incl.md}}

## `sled` {#sled}

[![sled][c~sled~docs~badge]][c~sled~docs] [![sled~crates.io][c~sled~crates.io~badge]][c~sled~crates.io] [![sled~github][c~sled~github~badge]][c~sled~github] [![sled~lib.rs][c~sled~lib.rs~badge]][c~sled~lib.rs]{{hi:sled}}{{hi:Sqlite}}{{hi:Redis}}{{hi:LMDB}}{{hi:Mongo}}{{hi:Rocksdb}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~database-implementations][cat~database-implementations~badge]][cat~database-implementations]{{hi:Database implementations}}

[`sled`][c~sled~github]↗ is a high-performance, fairly low-level, embedded [database][p~database]. It can be thought of as a [`BTreeMap<[u8], [u8]>`][c~std~collections~BTreeMap~docs]↗ that stores its data on disk.

- Storage on disk, without dealing with files.
- No external database.
- No network costs.
- API similar to a thread-safe `BTreeMap<[u8], [u8]>`.
- Serializable (ACID) transactions for atomically reading and writing to multiple keys in multiple keyspaces.
- Fully atomic single-key operations, including compare and swap.
- Zero-copy reads.
- Write batches.
- Subscription to changes on key prefixes.
- Multiple keyspaces.
- Merge operators.
- Forward and reverse [iterators][p~iterators] over ranges of items.
- Crash-safe monotonic ID generator capable of generating 75-125 million unique ID's per second.
- [`zstd`][c~zstd~docs]↗{{hi:zstd}} [compression][p~compression] (use the [compression][p~compression] build feature, disabled by default).
- CPU-scalable, lock-free implementation.
- Flash-optimized log-structured storage.
- Uses modern b-tree techniques such as prefix [encoding][p~encoding] and suffix truncation for reducing the storage costs of long keys with shared prefixes. If keys are the same length and sequential then the system can avoid storing 99%+ of the key data in most cases, essentially acting like a learned index.

```rust,editable
{{#include ../../../crates/cats/database_implementations/examples/databases/sled.rs:example}}
```

## SurrealDB {#surrealdb}

[![surrealdb][c~surrealdb~docs~badge]][c~surrealdb~docs]{{hi:surrealdb}}
[![surrealdb~crates.io][c~surrealdb~crates.io~badge]][c~surrealdb~crates.io]
[![surrealdb~github][c~surrealdb~github~badge]][c~surrealdb~github]
[![surrealdb~lib.rs][c~surrealdb~lib.rs~badge]][c~surrealdb~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~database-implementations][cat~database-implementations~badge]][cat~database-implementations]{{hi:Database implementations}}
[![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}}

SurrealDB is a scalable, distributed, collaborative, document-graph [database][p~database], for the realities web.

```rust,editable
{{#include ../../../crates/cats/database_implementations/examples/databases/surrealdb.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[databases: expand / write](https://github.com/john-cd/rust_howto/issues/290)
</div>
