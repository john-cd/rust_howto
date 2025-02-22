# Database Implementations

[![cat-database-implementations][cat-database-implementations-badge]][cat-database-implementations]{{hi:Database implementations}}

Databases allow clients to store and query large amounts of data in an efficient manner. This category is for database management systems implemented in Rust.

## Databases written in Rust

{{#include databases.incl.md}}

## Search engines written in Rust

{{#include rust_search_engines.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[database-implementations: expand (P2)](https://github.com/john-cd/rust_howto/issues/292)

Embedded Databases:

[`sled`][c-sled]⮳{{hi:sled}}: A high-performance embedded database.
[`lmdb-rs`][c-lmdb_rs]⮳{{hi:lmdb-rs}}: Bindings to the LMDB embedded database.
[`rocksdb`][c-rocksdb]⮳{{hi:rocksdb}}: Bindings to the RocksDB embedded database (though RocksDB itself is written in C++).
`tikv`: A distributed transactional key-value database (used in TiDB). While not strictly embedded, it's often used in similar ways.

Key-Value Stores:

[`sled`][c-sled]⮳{{hi:sled}} and [`lmdb-rs`][c-lmdb_rs]⮳{{hi:lmdb-rs}} above can also be used as key-value stores.

Other Databases (Partially or Primarily Rust):

[`tantivy`][c-tantivy]⮳{{hi:tantivy}}: A full-text search engine library.
[`qdrant`][c-qdrant]⮳{{hi:qdrant}}: A vector similarity search engine.

</div>
