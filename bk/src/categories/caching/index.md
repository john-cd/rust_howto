# Caching

Store the results of previous computations in order to reuse the results.

## LRU caching

{{#include lru.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[caching: cover Moka, etc (P2)](https://github.com/john-cd/rust_howto/issues/228)

- In-Memory Caches: [`lru`][c-lru]⮳{{hi:lru}}, [`moka`][c-moka]⮳{{hi:moka}}, [`cached`][c-cached]⮳{{hi:cached}}

- Distributed Caches: [`redis`][c-redis]⮳{{hi:redis}}
[[database | Database]]
[[nosql | Nosql]]
[[key_value_stores | Key Value Stores]]

[[database-implementations | Database Implementations]]
[[databases | Databases]]

- Cache Invalidation: Often application-specific, but crates like `notify` can help with file-based invalidation

- Serialization/Deserialization: [`serde`][c-rmp_serde]⮳{{hi:serde}} [`serde`][c-serde::Deserialize]⮳{{hi:serde}} [`serde`][c-serde]⮳{{hi:serde}} [`serde`][c-serde_ignored]⮳{{hi:serde}} [`serde`][c-serde_json::Value]⮳{{hi:serde}} [`serde`][c-serde_json::from_str]⮳{{hi:serde}} [`serde`][c-serde_json::json]⮳{{hi:serde}} [`serde`][c-serde_json::to_string]⮳{{hi:serde}} [`serde`][c-serde_json]⮳{{hi:serde}} (essential for caching complex data)

[[database | Database]]
[[database-implementations | Database Implementations]]
[[databases | Databases]]

[[serde | Serde]]
[[complex_encoding | Complex Encoding]]
[[encoding | Encoding]]

[[file_watching | File Watching]]
[[watching_for_changes | Watching for Changes]]

</div>
