# Key-value Stores

{{#include key_value_stores.incl.md}}

## `heed` {#heed}

[![heed][c~heed~docs~badge]][c~heed~docs] [![heed~crates.io][c~heed~crates.io~badge]][c~heed~crates.io] [![heed~github][c~heed~github~badge]][c~heed~github] [![heed~lib.rs][c~heed~lib.rs~badge]][c~heed~lib.rs]{{hi:heed}}{{hi:Database}}{{hi:LMDB}}{{hi:Storage}}{{hi:Typed}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`heed`][c~heed~docs]↗{{hi:heed}} is a fully-typed [`LMDB`][wikipedia~LMDB]↗{{hi:LMDB}} wrapper. `LMDB` (Lightning Memory-Mapped Database) is a fast and efficient embedded [database][p~database] library that provides key/value storage.
[`heed`][c~heed~docs]↗{{hi:heed}} is a fully-typed [`LMDB`][wikipedia~LMDB]↗{{hi:LMDB}} wrapper. `LMDB` (Lightning Memory-Mapped Database) is a fast and efficient embedded [database][p~database] library that provides key/value storage.
Use [`heed`][c~heed~docs]↗{{hi:heed}} for:

- [Caching][p~caching]: App data, web pages or images.
- [Embedded][p~embedded] [databases][p~databases] for mobile apps, IoT devices, desktop applications.
- Indexing: Local [search][p~search] index or metadata storage.
- Game state persistence, [configuration][p~configuration] storage.

Advantages include speed, minimal memory footprint, [embedded][p~embedded] use, memory mapping, and ACID (Atomicity, Consistency, Isolation, Durability) properties. It is not ideal for huge datasets, complex queries, or network access.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/kv/heed.rs:example}}
```

## `rocksdb` {#rocksdb}

[![rocksdb][c~rocksdb~docs~badge]][c~rocksdb~docs] [![rocksdb~crates.io][c~rocksdb~crates.io~badge]][c~rocksdb~crates.io] [![rocksdb~github][c~rocksdb~github~badge]][c~rocksdb~github] [![rocksdb~lib.rs][c~rocksdb~lib.rs~badge]][c~rocksdb~lib.rs]{{hi:rocksdb}}{{hi:Database}}{{hi:Embedded}}{{hi:Lsm-tree}}{{hi:Persistence}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`rocksdb`][c~rocksdb~docs]↗{{hi:rocksdb}} is a Rust wrapper for Facebook's `RocksDB` embeddable database. [`RocksDB`][c~rocksdb~docs]↗{{hi:RocksDB}} is a high performance database for key-value data. Use [`rocksdb`][c~rocksdb~docs]↗{{hi:rocksdb}}:

- As the storage engine for other [databases][p~databases] (MySQL, MongoDB, TiKV...).
- For [caching][p~caching].
- To handle time-series data, indexes for [search][p~search] engines, persistent message queues.

[`RocksDB`][c~rocksdb~docs]↗{{hi:RocksDB}} is employed in stream processing frameworks like `Apache Flink` and `Kafka Streams` to maintain the state of streaming applications.

[`RocksDB`][c~rocksdb~docs]↗{{hi:RocksDB}} is great for performance, scalability, flexibility, embeddability. Avoid when dealing with complex SQL or distributed transactions.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/kv/rocksdb.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
