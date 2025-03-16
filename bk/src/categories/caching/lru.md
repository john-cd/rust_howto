# In-Memory Caches

{{#include lru.incl.md}}

Consider the following Rust crates for your in-memory [caching][p-caching] needs: [`lru`][c-lru]⮳{{hi:lru}}, [`moka`][c-moka]⮳{{hi:moka}}, and [`cached`][c-cached]⮳{{hi:cached}}.

## Use a Least Recently Used (LRU) Cache {#lru-cache}

[![lru][c-lru-badge]][c-lru]{{hi:lru}} [![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}}

[`lru`][c-lru]⮳{{hi:lru}} provides a fast Least Recently Used (LRU) cache for Rust. LRU is a common data structure used to limit the size of a cache by discarding the Least Recently Used (accessed) items when the cache reaches its capacity.

```rust,editable
{{#include ../../../crates/cats/caching/tests/lru.rs:example}}
```

## `moka` {#moka}

[![moka][c-moka-badge]][c-moka] [![moka-crates.io][c-moka-crates.io-badge]][c-moka-crates.io] [![moka-github][c-moka-github-badge]][c-moka-github] [![moka-lib.rs][c-moka-lib.rs-badge]][c-moka-lib.rs]{{hi:moka}}{{hi:Cache}}{{hi:Concurrent}} [![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[`moka`][c-moka]⮳{{hi:moka}} is a fast and concurrent cache library inspired by Java Caffeine.

{{#example moka}}

## `cached` {#cached}

[![cached][c-cached-badge]][c-cached] [![cached-crates.io][c-cached-crates.io-badge]][c-cached-crates.io] [![cached-github][c-cached-github-badge]][c-cached-github] [![cached-lib.rs][c-cached-lib.rs-badge]][c-cached-lib.rs]{{hi:cached}}{{hi:Cache}}{{hi:Disk}}{{hi:Lru}}{{hi:Memoize}}{{hi:Redis}} [![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

[`cached`][c-cached]⮳{{hi:cached}} provides generic cache implementations and simplified function memoization.

{{#example cached}}

## Related Topics {#skip}

- [[database | Database]].
- [[database-implementations | Database Implementations]].
- [[nosql | NoSQL]].
- [[key_value_stores | Key-Value Stores]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand](https://github.com/john-cd/rust_howto/issues/227)
rename file
review in depth
</div>
