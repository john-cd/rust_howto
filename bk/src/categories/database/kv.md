# Key-value stores

{{#include kv.incl.md}}

## `heed` {#heed}

[![heed][c-heed-badge]][c-heed] [![heed-crates.io][c-heed-crates.io-badge]][c-heed-crates.io] [![heed-github][c-heed-github-badge]][c-heed-github] [![heed-lib.rs][c-heed-lib.rs-badge]][c-heed-lib.rs]{{hi:heed}}{{hi:Database}}{{hi:Lmdb}}{{hi:Storage}}{{hi:Typed}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

A fully typed LMDB (mdb.master) wrapper with minimum overhead. LMDB is a high performant, light-weight, embedded key-value database library.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/kv/heed.rs:example}}
```

## `rocksdb` {#rocksdb}

[![rocksdb][c-rocksdb-badge]][c-rocksdb] [![rocksdb-crates.io][c-rocksdb-crates.io-badge]][c-rocksdb-crates.io] [![rocksdb-github][c-rocksdb-github-badge]][c-rocksdb-github] [![rocksdb-lib.rs][c-rocksdb-lib.rs-badge]][c-rocksdb-lib.rs]{{hi:rocksdb}}{{hi:Database}}{{hi:Embedded}}{{hi:Lsm-tree}}{{hi:Persistence}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

Rust wrapper for Facebook's RocksDB embeddable database. RocksDB is a high performance database for key-value data.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/rocksdb/rocksdb.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

[P1 organize / write](https://github.com/john-cd/rust_howto/issues/1066)

</div>
