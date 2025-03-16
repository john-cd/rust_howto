# Caching

[![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}}

Caching refers to storing the results of previous computations, in order to reuse the results.

## In-Memory Caches

{{#include lru.incl.md}}

## Hybrid Caches

[`foyer`][c-foyer]⮳{{hi:foyer}} is a hybrid cache for Rust.

## Distributed Caches

Consider [`redis`][c-redis]⮳{{hi:redis}}. Refer to the [[nosql | NoSQL]] and [[key_value_stores | Key Value Stores]] chapters.

## Cache Invalidation

Cache invalidation is often application-specific, but crates like [`notify`][c-notify]⮳{{hi:notify}} can help with file-based invalidation.
Refer to the following chapters:

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
TODO write
</div>
