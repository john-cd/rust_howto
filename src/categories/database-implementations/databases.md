# Databases written in Rust

{{#include databases.incl.md}}

## `sled` {#sled}

[![sled][c-sled-badge]][c-sled] [![sled-crates.io][c-sled-crates.io-badge]][c-sled-crates.io] [![sled-github][c-sled-github-badge]][c-sled-github] [![sled-lib.rs][c-sled-lib.rs-badge]][c-sled-lib.rs]{{hi:sled}}{{hi:Sqlite}}{{hi:Redis}}{{hi:Lmdb}}{{hi:Mongo}}{{hi:Rocksdb}} [![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}} [![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-database-implementations][cat-database-implementations-badge]][cat-database-implementations]{{hi:Database implementations}}

[`sled`][c-sled-github]⮳ is a high-performance, fairly low-level, embedded database. It can be thought of as a `BTreeMap<[u8], [u8]>` that stores its data on disk.

- Storage on disk, without dealing with files
- No external database
- No network costs
- API similar to a threadsafe BTreeMap<[u8], [u8]>
- Serializable (ACID) transactions for atomically reading and writing to multiple keys in multiple keyspaces
- Fully atomic single-key operations, including compare and swap
- Zero-copy reads
- Write batches
- Subscription to changes on key prefixes
- Multiple keyspaces
- Merge operators
- Forward and reverse iterators over ranges of items
- Crash-safe monotonic ID generator capable of generating 75-125 million unique ID's per second
- `zstd` compression (use the compression build feature, disabled by default)
- CPU-scalable, lock-free implementation
- Flash-optimized log-structured storage
- Uses modern b-tree techniques such as prefix encoding and suffix truncation for reducing the storage costs of long keys with shared prefixes. If keys are the same length and sequential then the system can avoid storing 99%+ of the key data in most cases, essentially acting like a learned index

```rust,editable
{{#include ../../../crates/ex/cats/database_implementations/tests/sled.rs:example}}
```

## SurrealDB {#surrealdb}

[![surrealdb][c-surrealdb-badge]][c-surrealdb]{{hi:surrealdb}}
[![surrealdb-crates.io][c-surrealdb-crates.io-badge]][c-surrealdb-crates.io]
[![surrealdb-github][c-surrealdb-github-badge]][c-surrealdb-github]
[![surrealdb-lib.rs][c-surrealdb-lib.rs-badge]][c-surrealdb-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-database-implementations][cat-database-implementations-badge]][cat-database-implementations]{{hi:Database implementations}}
[![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}}

SurrealDB is a scalable, distributed, collaborative, document-graph database, for the realtime web.

```rust,editable
{{#include ../../../crates/ex/cats/database_implementations/tests/surrealdb.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[databases: expand / write (P2)](https://github.com/john-cd/rust_howto/issues/290)

## RabbitMQ

[![lapin][c-lapin-badge]][c-lapin] [![lapin-crates.io][c-lapin-crates.io-badge]][c-lapin-crates.io] [![lapin-github][c-lapin-github-badge]][c-lapin-github] [![lapin-lib.rs][c-lapin-lib.rs-badge]][c-lapin-lib.rs]{{hi:lapin}}{{hi:Amqp}}{{hi:Futures}}{{hi:Mio}}{{hi:Rabbitmq}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

AMQP client library
</div>
