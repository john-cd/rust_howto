<!-- TODO Reviewed 11-17-2026. -->

# Key-value Stores

{{#include key_value_stores.incl.md}}

## `heed` {#heed}

{{#badges heed data-structures="Data structures" database="Database interfaces"}}{{hi:Database}}{{hi:LMDB}}{{hi:Storage}}{{hi:Typed}}

[`heed`][c~heed~docs]↗{{hi:heed}} is a fully typed wrapper for [`LMDB`][wikipedia~LMDB]↗{{hi:LMDB}}, an embedded key-value database with memory-mapped storage and ACID transactions.
Use [`heed`][c~heed~docs]↗{{hi:heed}} for:

- [Caching][p~caching]: App data, web pages or images.
- Embedded databases for mobile apps, IoT devices, and desktop applications.
- Indexing: Local [search][p~search] index or metadata storage.
- Game state persistence, [configuration][p~configuration] storage.

Advantages include speed, minimal memory footprint, embedded use, memory mapping, and ACID (Atomicity, Consistency, Isolation, Durability) properties. It is not ideal for huge datasets, complex queries, or network access.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/kv/heed.rs:example}}
```

## `rocksdb` {#rocksdb}

{{#badges rocksdb database="Database interfaces"}}{{hi:Database}}{{hi:Embedded}}{{hi:Lsm-tree}}{{hi:Persistence}}

[`rocksdb`][c~rocksdb~docs]↗{{hi:rocksdb}} is a Rust wrapper for Facebook's `RocksDB` embeddable database. [`RocksDB`][c~rocksdb~docs]↗{{hi:RocksDB}} is a high performance database for key-value data. Use [`rocksdb`][c~rocksdb~docs]↗{{hi:rocksdb}}:

- As the storage engine for other databases (including MySQL, MongoDB, and TiKV).
- For [caching][p~caching].
- To handle time-series data, indexes for [search][p~search] engines, persistent message queues.

[`RocksDB`][c~rocksdb~docs]↗{{hi:RocksDB}} is employed in stream processing frameworks like `Apache Flink` and `Kafka Streams` to maintain the state of streaming applications.

[`RocksDB`][c~rocksdb~docs]↗{{hi:RocksDB}} is great for performance, scalability, flexibility, embeddability. Avoid when dealing with complex SQL or distributed transactions.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/kv/rocksdb.rs:example}}
```

## Related Topics {#related-topics .skip}

- [NoSQL and Friends](nosql.md).
- [Connection Pools](connection_pool.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
