# Database Implementations

[![cat~database-implementations][cat~database-implementations~badge]][cat~database-implementations]{{hi:Database implementations}}

Databases allow clients to store and query large amounts of data in an efficient manner. This category is for database management systems implemented in Rust.

| Database Type | Rust crates |
|---|---|
| Embedded Databases | [`sled`][c~sled~docs]↗{{hi:sled}}: A high-performance embedded database. [`lmdb-rs`][c~lmdb-rs~docs]↗{{hi:lmdb-rs}}: Bindings to the LMDB embedded database. [`rocksdb`][c~rocksdb~docs]↗{{hi:rocksdb}}: Bindings to the RocksDB embedded database. [`tikv`](https://tikv.org)↗{{hi:tikv}}: A distributed transactional key-value database (used in TiDB). While not strictly embedded, it's often used in similar ways. |
| Key-Value Stores | [`sled`][c~sled~docs]↗{{hi:sled}} and [`lmdb-rs`][c~lmdb-rs~docs]↗{{hi:lmdb-rs}} above can also be used as key-value stores. |
| Other Databases | [`tantivy`][c~tantivy~docs]↗{{hi:tantivy}}: A full-text search engine library. [`qdrant`][c~qdrant~docs]↗{{hi:qdrant}}: A vector similarity search engine. |

## Databases Written in Rust

{{#include databases.incl.md}}

## Search Engines Written in Rust

{{#include rust_search_engines.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[database-implementations: expand](https://github.com/john-cd/rust_howto/issues/292)

## Message Queues Written in Rust

- [robustmq: RobustMQ is a next-generation, high-performance, cloud-native, converged message queue that is compatible with multiple mainstream message queuing protocols and has complete Serverless capabilities.][c~robustmq~github]
- [rocketmq-rust: Apache RocketMQ build in Rust. Faster, safer, and with lower memory usage.][c~rocketmq-rust~github]

</div>
