# Caching

Store the results of previous computations, in order to reuse the results.

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

## Web page caching

See:

- [[web-programming | Web Programming]].
- [[web-programming_http-server | Web Programming HTTP Server]].

## Database query caching

Refer to:

- [[database | Database]].
- [[database-implementations | Database Implementations]].

### Serialization and Deserialization prior to Caching

Refer to:

- [[serde | Serde]].
- [[complex_encoding | Complex Encoding]].
- [[encoding | Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[cover Moka, etc](https://github.com/john-cd/rust_howto/issues/228)
write
</div>
