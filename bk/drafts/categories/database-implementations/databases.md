<!-- TODO Reviewed ??-??-2026 -->

# Databases Written in Rust

Embedded databases written natively in Rust run inside your application process, reading and writing directly to local disk or in-memory storage. By eliminating external database servers and network latency, embedded engines simplify deployment into a single self-contained binary while retaining ACID safety guarantees.

{{#include databases.incl.md}}

## `redb` {#redb}

[![redb][c~redb~docs~badge]][c~redb~docs] [![redb~crates.io][c~redb~crates.io~badge]][c~redb~crates.io] [![redb~repo][c~redb~repo~badge]][c~redb~repo] [![redb~lib.rs][c~redb~lib.rs~badge]][c~redb~lib.rs]{{hi:redb}} [![cat~database-implementations][cat~database-implementations~badge]][cat~database-implementations]{{hi:Database implementations}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}}

[`redb`][c~redb~repo]↗ is a simple, portable, high-performance ACID-compliant embedded key-value store inspired by LMDB.

- Pure Rust implementation with zero external C dependencies.
- Fully ACID-compliant transactions using copy-on-write storage.
- Strongly typed tables with compile-time checked key and value types.
- Zero-copy reads via byte slice references.
- Concurrent reader support without blocking writers.
- Stable, crash-safe storage format. A database file has one owning process; use its shared database handle for concurrent transactions.

```rust,editable
{{#include ../../../crates/cats/database_implementations/examples/databases/redb.rs:example}}
```

## `sled` {#sled}

[![sled][c~sled~docs~badge]][c~sled~docs] [![sled~crates.io][c~sled~crates.io~badge]][c~sled~crates.io] [![sled~repo][c~sled~repo~badge]][c~sled~repo] [![sled~lib.rs][c~sled~lib.rs~badge]][c~sled~lib.rs]{{hi:sled}}{{hi:Sqlite}}{{hi:Redis}}{{hi:LMDB}}{{hi:Mongo}}{{hi:Rocksdb}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~database-implementations][cat~database-implementations~badge]][cat~database-implementations]{{hi:Database implementations}}

[`sled`][c~sled~repo]↗ is a high-performance, embedded key-value [database][p~database]. It presents an API similar to a thread-safe [`BTreeMap<[u8], [u8]>`][c~std~collections~BTreeMap~docs]↗ that stores its data on disk.

> **Note:** `sled` has been in beta status for a long time. The project authors caution that pre-1.0 disk formats may change between versions and require manual data migrations.

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
- Crash-safe monotonic ID generator capable of generating 75-125 million unique IDs per second.
- [`zstd`][c~zstd~docs]↗{{hi:zstd}} [compression][p~compression] (use the [compression][p~compression] build feature, disabled by default).
- CPU-scalable, lock-free implementation.
- Flash-optimized log-structured storage.
- Uses modern B-tree techniques such as prefix [encoding][p~encoding] and suffix truncation to reduce storage costs for long keys with shared prefixes.

```rust,editable
{{#include ../../../crates/cats/database_implementations/examples/databases/sled.rs:example}}
```

## SurrealDB {#surrealdb}

[![surrealdb][c~surrealdb~docs~badge]][c~surrealdb~docs]{{hi:surrealdb}}
[![surrealdb~crates.io][c~surrealdb~crates.io~badge]][c~surrealdb~crates.io]
[![surrealdb~repo][c~surrealdb~repo~badge]][c~surrealdb~repo]
[![surrealdb~lib.rs][c~surrealdb~lib.rs~badge]][c~surrealdb~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~database-implementations][cat~database-implementations~badge]][cat~database-implementations]{{hi:Database implementations}}
[![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}}

SurrealDB is a scalable, distributed, multi-model document-graph [database][p~database] that can run either in-process or as a standalone server.

```rust,editable
{{#include ../../../crates/cats/database_implementations/examples/databases/surrealdb.rs:example}}
```

<div class="hidden">
  The example uses SurrealDB 3's current `SurrealValue` derive and the in-memory `Mem` engine so its test exercises the full CRUD path without a server or network service.
</div>

## Choosing an Embedded Database {#choosing}

Selecting the right embedded database depends on data structure requirements, concurrency demands, and storage engine architecture.

| Engine                         | Kind               | Storage Engine                  | Best For                                                                          |
| ------------------------------ | ------------------ | ------------------------------- | --------------------------------------------------------------------------------- |
| [`redb`][c~redb~repo]↗         | Key-Value Store    | Copy-on-Write B-Tree            | Production embedded KV storage needing ACID transactions and a stable file format |
| [`fjall`][c~fjall~repo]↗       | Key-Value Store    | Log-Structured Merge Tree (LSM) | Write-heavy workloads, range scans, and high insertion throughput                 |
| [`sled`][c~sled~repo]↗         | Key-Value Store    | Lock-Free B-Tree                | High-concurrency key-value workloads (note long-standing beta status)             |
| [GlueSQL][c~gluesql~repo]↗     | SQL Engine         | Pluggable Backends              | Executing SQL queries over in-memory or custom embedded key-value backends        |
| [Turso][c~turso~repo]↗         | SQL Engine         | SQLite-Compatible Engine        | In-process SQLite-compatible SQL database engine written in Rust (formerly Limbo) |
| [SurrealDB][c~surrealdb~repo]↗ | Multi-Model Engine | KV / Document / Graph           | Embedded document and graph database with rich query language capabilities        |

### Key-Value vs SQL Engines

- **B-Tree vs LSM-Tree**: Copy-on-write B-tree stores such as `redb` and LSM-tree engines such as `fjall` organize updates differently. LSM engines buffer writes in memory and compact sorted files in the background. Read latency, write amplification, and throughput depend on the workload and durability settings; benchmark representative data rather than choosing from the storage architecture alone.
- **Embedded SQL**: If your application requires relational tables, indexes, and complex JOIN queries, embedded SQL engines like `GlueSQL` or `Turso` provide SQL query parsing and execution. For traditional C-based SQLite bindings in Rust, see [SQLite][p~sqlite].

## Related Topics {#related-topics .skip}

| Topic                    | Chapter / Links                                                              | Description                                                                                         |
| ------------------------ | ---------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Database Implementations | [Database Implementations Index][ex~database-implementations~databases]      | Overview of databases, query engines, search systems, and messaging written in Rust.                |
| Search Engines           | [Search Engines Written in Rust][ex~database-implementations~search-engines] | Full-text, vector, and indexing engines implemented in Rust.                                        |
| Database Clients         | [Database Interfaces][p~database]                                            | Client libraries and ORMs connecting to external database servers (PostgreSQL, MySQL, Redis, etc.). |
| SQLite Integration       | [SQLite via `rusqlite`][p~sqlite]                                            | Embedded SQLite database interactions using native C bindings.                                      |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
