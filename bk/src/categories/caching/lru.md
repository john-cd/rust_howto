# LRU caching

{{#include lru.incl.md}}

## Use a LRU cache {#lru-cache}

[![lru][c-lru-badge]][c-lru]{{hi:lru}} [![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}}

[`lru`][c-lru]⮳{{hi:lru}} provides a fast Least Recently Used (LRU) cache for Rust. LRU caches are a common data structure used to limit the size of a cache by discarding the Least Recently accessed (Used) items when the cache reaches its capacity.

```rust,editable
{{#include ../../../crates/cats/caching/tests/lru.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[lru: expand (P2)](https://github.com/john-cd/rust_howto/issues/227)

- In-Memory Caches: [`lru`][c-lru]⮳{{hi:lru}}, [`moka`][c-moka]⮳{{hi:moka}}, [`cached`][c-cached]⮳{{hi:cached}}

[[database | Database]]
[[nosql | Nosql]]
[[key_value_stores | Key Value Stores]]

[[database-implementations | Database Implementations]]
[[databases | Databases]]

</div>
