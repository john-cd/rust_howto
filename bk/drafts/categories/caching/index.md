<!-- TODO Reviewed ??-??-2026 -->

# Caching

[![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}}

Caching stores a copy of a value or computation result so a later request can reuse it. A cache can reduce latency or load on an expensive source, but every entry needs a key, a size or eviction policy, and a rule for how long it remains valid. The source of truth should remain authoritative: a cache hit may be stale.

## Choose a Cache by Scope

| Scope or need                                                          | Crates                                             | Typical fit                                                                                                                   |
| ---------------------------------------------------------------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Bounded in-process cache with explicit LRU behavior                    | [`lru`][c~lru~docs]↗                               | A simple cache owned by one process, where you choose the capacity and eviction policy.                                       |
| Concurrent in-process cache with expiration                            | [`moka`][c~moka~docs]↗                             | Synchronous or asynchronous applications that need bounded entries, time-to-live (TTL), or time-to-idle expiration.           |
| Concurrent in-process cache with a small API and configurable eviction | [`quick_cache`][c~quick_cache~docs]↗               | A local cache where you want sync or unsync variants and will measure its behavior against your workload.                     |
| Function memoization and cache stores                                  | [`cached`][c~cached~docs]↗                         | Caching function results with macros or selecting one of its provided stores.                                                 |
| Memory plus local disk tiers                                           | [`foyer`][c~foyer~docs]↗                           | A larger local cache whose working set should extend beyond RAM; check platform, MSRV, and project status before adopting it. |
| Shared cache service                                                   | [`redis`][c~redis~docs]↗ or [`fred`][c~fred~docs]↗ | Multiple application instances need a common Redis or Valkey service, with network and operational costs.                     |

The [in-memory cache chapter][p~in-memory-cache] includes small examples for `lru`, `moka`, and `cached`. Pick the smallest scope that meets the consistency and sharing requirements; a remote cache adds serialization, network failure, authentication, and deployment concerns.

## In-Memory Caches

{{#include in_memory_cache.incl.md}}

## Hybrid Caches

[`foyer`][c~foyer~docs]↗{{hi:foyer}} combines in-memory and disk cache tiers. It is useful when a local cache needs more capacity than RAM alone, but its extra persistence, storage, and recovery behavior should be part of the design rather than treated as a transparent map.

## Distributed Caches

[`redis`][c~redis~docs]↗{{hi:redis}} is the general-purpose Redis client; [`fred`][c~fred~docs]↗{{hi:fred}} is an asynchronous client for Redis and Valkey. A distributed cache can be shared across processes, but it can also be unavailable, slower than a local hit, or evict data under server memory pressure. Set timeouts, choose whether cache failures are fatal or fall back to the source, and avoid storing secrets without an explicit security design. See the [[nosql | NoSQL]] and [[key_value_stores | Key-Value Stores]] chapters.

## Cache Invalidation

Expiration and invalidation solve different problems. A TTL bounds how long an entry can remain in the cache; it does not make an entry fresh immediately after its source changes. For mutable data, update or invalidate the cache when the source is written, and use a TTL as a fallback for missed invalidations. Include a version or tenant in keys when values depend on changing configuration or ownership.

A common cache-aside flow is to read from the cache, fetch from the source on a miss, then insert the result with an explicit expiry. On writes, update the source first and then invalidate or replace the corresponding cache entry. If invalidation is event-driven, design for dropped or reordered events and provide a way to reconcile cache state.

For file-backed values, [`notify`][c~notify~docs]↗{{hi:notify}} can report filesystem events; treat events as a signal to invalidate or reload, not as a durable record of every change. See:

- [[file_watching | File Watching]].
- [[watching_for_changes | Watching for Changes]].

## Web Page Caching

See:

- [[web-programming | Web Programming]].
- [[web-programming_http-server | Web Programming HTTP Server]].

## Database Query Caching

Refer to:

- [[database | Database]].
- [[database-implementations | Database Implementations]].

### Serialization and Deserialization Prior to Caching

Refer to:

- [[serde | Serde]].
- [[complex_encoding | Complex Encoding]].
- [[encoding | Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  Added a cache-selection overview that compares process-local, hybrid, and distributed options, and explains cache scope, expiration, invalidation, and cache-aside flows.
</div>
