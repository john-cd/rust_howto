<!-- TODO Reviewed ??-??-2026. -->

# Databases Written in Rust

An embedded database engine runs in the application process and stores data locally or in memory. It avoids deploying a separate server, but persistence, transaction guarantees, and concurrency behavior vary by engine and configuration.

{{#include databases.incl.md}}

## `redb` {#redb}

{{#badges redb database-implementations="Database implementations" embedded="Embedded development"}}

[`redb`][c~redb~repo]↗ is a pure-Rust embedded key-value store with ACID transactions and a stable on-disk format.

- No external C library dependency.
- Copy-on-write transactions and strongly typed tables.
- Zero-copy reads through byte-slice references.
- Concurrent readers without blocking writers.
- One process owns a database file; share its database handle for concurrent transactions.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p database_implementations --example databases`.

```rust,editable,noplayground
{{#include ../../../crates/cats/database_implementations/examples/databases/redb.rs:example}}
```

## `sled` {#sled}

{{#badges sled algorithms=Algorithms caching=Caching concurrency=Concurrency data-structures="Data structures" database-implementations="Database implementations"}}{{hi:Sqlite}}{{hi:Redis}}{{hi:LMDB}}{{hi:Mongo}}{{hi:Rocksdb}}

[`sled`][c~sled~repo]↗ is an embedded key-value [database][p~database] with an API inspired by a thread-safe [`BTreeMap<[u8], [u8]>`][c~std~collections~BTreeMap~docs]↗. It stores data on disk and supports multiple named trees.

> **Note:** `sled` has been in beta status for a long time. The project authors caution that pre-1.0 disk formats may change between versions and require manual data migrations.

- Thread-safe access to data stored on disk, without managing the file format directly.
- Atomic single-key operations, including compare-and-swap.
- Serializable transactions across multiple keys and trees.
- Write batches, prefix subscriptions, and forward or reverse [iterators][p~iterators].
- Optional [`zstd`][c~zstd~docs]↗{{hi:zstd}} [compression][p~compression], disabled by default.

```rust,editable
{{#include ../../../crates/cats/database_implementations/examples/databases/sled.rs:example}}
```

## SurrealDB {#surrealdb}

{{#badges surrealdb data-structures="Data structures" database-implementations="Database implementations" embedded="Embedded development"}}

SurrealDB is a scalable, distributed, multi-model document-graph [database][p~database] that can run either in-process or as a standalone server.

```rust,editable,noplayground
{{#include ../../../crates/cats/database_implementations/examples/databases/surrealdb.rs:example}}
```

<div class="hidden">
  The example uses SurrealDB 4's `SurrealValue` derive and the in-memory `Mem` engine, so its test exercises the CRUD path without a server or network service.
</div>

## Choosing an Embedded Database {#choosing}

Selecting the right embedded database depends on data structure requirements, concurrency demands, and storage engine architecture.

| Engine | Model | Consider it when |
| --- | --- | --- |
| [`redb`][c~redb~repo]↗ | Embedded key-value store | You need typed tables, transactions, and a stable on-disk format. |
| [`fjall`][c~fjall~repo]↗ | Embedded key-value store using an LSM tree | You need write-oriented storage or range scans; benchmark with representative data. |
| [`sled`][c~sled~repo]↗ | Embedded key-value store | You are evaluating its API and can account for its beta status and format changes. |
| [GlueSQL][c~gluesql~repo]↗ | SQL engine with pluggable storage | You want to run SQL over an in-memory or custom key-value backend. |
| [Turso][c~turso~repo]↗ | In-process SQLite-compatible engine | You want SQLite-compatible SQL without a separate database server. |
| [SurrealDB][c~surrealdb~repo]↗ | Multi-model document and graph database | You want document and graph models through one query language. |

### Key-Value vs SQL Engines

- **B-Tree vs LSM-Tree**: Copy-on-write B-tree stores such as `redb` and LSM-tree engines such as `fjall` organize updates differently. LSM engines buffer writes in memory and compact sorted files in the background. Read latency, write amplification, and throughput depend on the workload and durability settings; benchmark representative data rather than choosing from the storage architecture alone.
- **Embedded SQL**: If your application requires relational tables, indexes, and complex JOIN queries, embedded SQL engines like `GlueSQL` or `Turso` provide SQL query parsing and execution. For traditional C-based SQLite bindings in Rust, see [SQLite][p~sqlite].

## Related Topics {#related-topics .skip}

| Topic | Chapter / Links | Description |
| --- | --- | --- |
| Database Implementations | [Database Implementations Index][ex~database-implementations~index] | Overview of databases, query engines, search systems, and messaging written in Rust. |
| Search Engines | [Search Engines Written in Rust][ex~database-implementations~search-engines] | Full-text, vector, and indexing engines implemented in Rust. |
| Database Clients | [Database Interfaces][p~database] | Client libraries and ORMs connecting to external database servers (PostgreSQL, MySQL, Redis, etc.). |
| SQLite Integration | [SQLite via `rusqlite`][p~sqlite] | Embedded SQLite database interactions using native C bindings. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
