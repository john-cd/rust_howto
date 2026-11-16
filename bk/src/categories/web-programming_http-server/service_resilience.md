<!-- TODO Reviewed ??-??-2026. -->

# Bound Retries and Concurrency with Tower

An upstream error is not permission to retry every request. This in-memory [Tower](https://docs.rs/tower/latest/tower/) service retries only an idempotent read after a transient failure, at most twice. A `ConcurrencyLimit` caps requests entering the upstream service, and one Tokio timeout covers the **whole** retry sequence. The tests use a paused clock and no network service.

```rust
{{#include ../../../crates/cats/web_programming_http_server/examples/service_resilience.rs:example}}
```

From `bk/crates`, run `cargo test -p web_programming_http_server --example service_resilience`. The tests show success after one transient failure, exhaustion after three attempts, a deadline on a stalled call, and no retry for a write or permanent error. In a real client, retry only when the operation is safe to repeat or has an idempotency key; add backoff with jitter, but keep it inside a total deadline. A concurrency limit creates backpressure when all permits are occupied. A separate rate limit or quota controls request volume over time; [governor](https://github.com/boinkor-net/governor) is one option.

[Tower's `ServiceBuilder` documentation](https://docs.rs/tower/latest/tower/builder/struct.ServiceBuilder.html#order) shows why layer order matters. The first layer added sees a request first. A timeout outside a retry layer can cap the entire attempt sequence; a timeout inside it applies to each attempt separately and can let total latency grow. Likewise, place a concurrency limit outside a queue if queued requests must count against the limit. For larger policy combinations, see [tower-resilience](https://github.com/joshrotenberg/tower-resilience). [Vector](https://github.com/vectordotdev/vector) illustrates a downstream log pipeline where bounded buffering and overload behavior also matter.
