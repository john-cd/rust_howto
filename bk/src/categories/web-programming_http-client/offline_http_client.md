<!-- TODO Reviewed ??-??-2026. -->

# Make an HTTP Request Against a Local Service

Configure a reusable asynchronous [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) client with a request deadline, then build a GET request with an `Accept` header and query parameter. `send()` can return an HTTP response even for a 503; `error_for_status()` makes that status an error before JSON decoding.

```rust,noplayground
{{#include ../../../crates/cats/web_programming_http_client/examples/offline_http_client.rs:example}}
```

From `bk/crates`, run `cargo test -p web_programming_http_client --example offline_http_client`. Its in-process loopback server checks the request and returns success, a 503, and malformed JSON; no remote service is needed. The test asserts each outcome. A blocking reqwest client belongs in synchronous code; keep it off an async runtime worker thread, or use the runtime's blocking-work facility for unavoidable synchronous calls. Reuse a `Client` for repeated requests so its connection pool can help. Keep a total deadline and decide separately which failures are safe to retry.

For other async entry points, see [Pinned Async Cancellation](../../standard-library/pinned_async_cancellation.md). A [WebSocket echo](../web-programming_websocket/websocket.md) is useful when both peers need a continuing stream rather than separate HTTP request/response exchanges. The [HTTP client library guide](http_clients.md) compares higher-level clients with Hyper.
