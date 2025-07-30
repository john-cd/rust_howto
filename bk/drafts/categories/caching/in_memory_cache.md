# In-Memory Caches

{{#include in_memory_cache.incl.md}}

Consider the following Rust crates for your in-memory [caching][p~caching] needs: [`lru`][c~lru~docs]⮳{{hi:lru}}, [`moka`][c~moka~docs]⮳{{hi:moka}}, and [`cached`][c~cached~docs]⮳{{hi:cached}}.

## Use a Least Recently Used (LRU) Cache {#lru-cache}

[![lru][c~lru~docs~badge]][c~lru~docs]{{hi:lru}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}}

[`lru`][c~lru~docs]⮳{{hi:lru}} provides a fast Least Recently Used (LRU) cache for Rust. LRU is a common data structure used to limit the size of a cache by discarding the Least Recently Used (accessed) items when the cache reaches its capacity.

```rust,editable
{{#include ../../../crates/cats/caching/examples/in_memory_cache/lru.rs:example}}
```

## `moka` {#moka}

[![moka][c~moka~docs~badge]][c~moka~docs] [![moka~crates.io][c~moka~crates.io~badge]][c~moka~crates.io] [![moka~github][c~moka~github~badge]][c~moka~github] [![moka~lib.rs][c~moka~lib.rs~badge]][c~moka~lib.rs]{{hi:moka}}{{hi:Cache}}{{hi:Concurrent}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`moka`][c~moka~docs]⮳{{hi:moka}} is a fast and concurrent cache library inspired by Java Caffeine.

```rust,editable
{{#include ../../../crates/cats/caching/examples/in_memory_cache/moka.rs:example}}
```

## `cached` {#cached}

[![cached][c~cached~docs~badge]][c~cached~docs] [![cached~crates.io][c~cached~crates.io~badge]][c~cached~crates.io] [![cached~github][c~cached~github~badge]][c~cached~github] [![cached~lib.rs][c~cached~lib.rs~badge]][c~cached~lib.rs]{{hi:cached}}{{hi:Cache}}{{hi:Disk}}{{hi:Lru}}{{hi:Memoize}}{{hi:Redis}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`cached`][c~cached~docs]⮳{{hi:cached}} provides generic cache implementations and simplified function memoization.

```rust,editable
{{#include ../../../crates/cats/caching/examples/in_memory_cache/cached.rs:example}}
```

## Related Topics {#related-topics}

- [[database | Database]].
- [[database-implementations | Database Implementations]].
- [[nosql | NoSQL]].
- [[key_value_stores | Key-Value Stores]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand; review in depth](https://github.com/john-cd/rust_howto/issues/227)
</div>
