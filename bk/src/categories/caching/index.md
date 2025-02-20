# Caching

Store the results of previous computations in order to reuse the results.

## LRU caching

{{#include lru.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[caching: cover Moka, etc (P2)](https://github.com/john-cd/rust_howto/issues/228)

- In-Memory Caches: lru, moka, cached
- Distributed Caches (Redis): redis
- Cache Invalidation: (Often application-specific, but crates like notify can help with file-based invalidation)
- Serialization/Deserialization: serde (essential for caching complex data)

</div>
