<!-- TODO Reviewed ??-??-2026 -->

# In-Memory Caches

{{#include in_memory_cache.incl.md}}

An in-memory cache is a performance optimization, not the source of truth. Decide what makes an entry stale and how it is invalidated before selecting a cache. A process-local cache is lost when the process restarts and is not shared with other application instances.

Consider these questions when designing a cache:

- **What is the key?** Include every input that can change the result, including tenant, locale, authorization scope, or data version when relevant.
- **How long may a value be stale?** Choose an explicit time-to-live (TTL) or time-to-idle policy when freshness has a limit. Invalidate entries when writes make them obsolete.
- **How much memory may it use?** Bound entries by count or estimated weight, and account for the memory held by each key and value. An entry-count limit is not a byte limit.
- **What happens on a miss?** Concurrent misses for one key can repeat expensive work. Use single-flight/loading support where appropriate, and decide whether failures should be cached.
- **Who shares the cache?** A cache local to one process cannot provide cross-process consistency. Use a shared service when instances must observe the same values.

[`lru`][c~lru~docs]↗{{hi:lru}}, [`moka`][c~moka~docs]↗{{hi:moka}}, and [`cached`][c~cached~docs]↗{{hi:cached}} cover different needs: a small explicit LRU data structure, a concurrent cache with policies, and function memoization.

## Use a Least Recently Used (LRU) Cache {#lru-cache}

[![lru][c~lru~docs~badge]][c~lru~docs]{{hi:lru}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}}

[`lru`][c~lru~docs]↗{{hi:lru}} provides a capacity-bounded least recently used (LRU) map. It is useful when one owner controls access and wants an explicit eviction policy. A capacity limit does not add expiration or synchronization between threads; add those behaviors only if the application needs them.

```rust,editable
{{#include ../../../crates/cats/caching/examples/in_memory_cache/lru.rs:example}}
```

## `moka` {#moka}

[![moka][c~moka~docs~badge]][c~moka~docs] [![moka~crates.io][c~moka~crates.io~badge]][c~moka~crates.io] [![moka~repo][c~moka~repo~badge]][c~moka~repo] [![moka~lib.rs][c~moka~lib.rs~badge]][c~moka~lib.rs]{{hi:moka}}{{hi:Cache}}{{hi:Concurrent}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`moka`][c~moka~docs]↗{{hi:moka}} provides synchronous and asynchronous concurrent caches. Its policies include maximum capacity, expiration by time-to-live or time-to-idle, and size-aware weighting. Select the `sync` or `future` cache to match the surrounding code, and remember that maintenance and eviction are cache-policy behavior rather than durable storage.

```rust,editable
{{#include ../../../crates/cats/caching/examples/in_memory_cache/moka.rs:example}}
```

## `cached` {#cached}

[![cached][c~cached~docs~badge]][c~cached~docs] [![cached~crates.io][c~cached~crates.io~badge]][c~cached~crates.io] [![cached~repo][c~cached~repo~badge]][c~cached~repo] [![cached~lib.rs][c~cached~lib.rs~badge]][c~cached~lib.rs]{{hi:cached}}{{hi:Cache}}{{hi:Disk}}{{hi:Lru}}{{hi:Memoize}}{{hi:Redis}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`cached`][c~cached~docs]↗{{hi:cached}} provides cache types and procedural macros for memoizing function results. A memoization key is derived from the function's inputs, so only use it when equal keys reliably produce interchangeable results. Configure a size or time bound for long-running processes, and avoid caching values that depend on hidden mutable state or secrets.

```rust,editable
{{#include ../../../crates/cats/caching/examples/in_memory_cache/cached.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[database | Database]].
- [[database-implementations | Database Implementations]].
- [[nosql | NoSQL]].
- [[key_value_stores | Key-Value Stores]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  expand; review in depth
</div>
