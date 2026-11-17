<!-- TODO Reviewed ??-??-2026. -->

# HTTP Clients

{{#include http_clients.incl.md}}{{hi:HTTP client}}

## `reqwest` {#reqwest}

{{#badges reqwest}}

[`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} is a high-level HTTP client with asynchronous and blocking APIs. The asynchronous `Client` runs on an async runtime; this package uses [`tokio`][c~tokio~docs]↗{{hi:tokio}}. The blocking API is enabled by the `blocking` feature and should not be called from inside an async runtime.

For a single request, the top-level `reqwest::get` and `reqwest::blocking::get` helpers are convenient. For repeated requests, create a `Client` once and reuse it so connections can be pooled. Configure timeouts and other client-wide behavior with `Client::builder`; call `error_for_status()` when an HTTP error status should become a Rust error, since receiving a response is separate from deciding whether its status is successful.

### GET Requests

For a complete async GET, see [Make an HTTP Request](requests.md#make-a-http-get-request). Use the blocking API in synchronous code, outside async runtime workers.

## `ureq` {#ureq}

{{#badges ureq web-programming::http-client="HTTP client"}}

[`ureq`][c~ureq~docs]↗{{hi:ureq}} is a synchronous HTTP client for code that does not need to await requests. `ureq::get` is convenient for one-off calls. For repeated calls or shared settings such as timeouts, create and reuse a `ureq::Agent`; it owns reusable client configuration and connections. It does not require an async runtime.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/http_clients/ureq.rs:example}}
```

## Build an HTTP Client with `hyper` {#hyper}

[![hyper~website][c~hyper~website~badge]][c~hyper~website] {{#badges hyper}} {{hi:Http}} {{hi:hyper}} {{hi:Hyperium}} {{#categories network-programming="Network programming",web-programming::http-client="HTTP client",web-programming::http-server="HTTP server"}}

[`hyper`][c~hyper~docs]↗{{hi:hyper}} implements HTTP/1 and HTTP/2 and exposes low-level client and server connection APIs. Hyper 1 moved the higher-level pooled client to [`hyper-util`][c~hyper-util~docs]↗{{hi:hyper-util}}. Choose that lower-level stack when you need to control the connection and protocol integration; choose a higher-level client such as `reqwest` for common application requests. The example below opens one HTTP/1 connection with Tokio. It uses plain HTTP and does not configure TLS for HTTPS.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/http_clients/hyper.rs:example}}
```

## References {#references .skip}

- [`hyper` Examples][c~hyper~examples~repo]↗.

## Related Topics {#related-topics .skip}

- [Asynchronous Programming](../asynchronous/index.md) for async runtimes and I/O.
- [HTTP Servers](../web-programming_http-server/json_patch.md) for server-side Hyper, Axum, and Actix Web examples.
- [WebSockets](../web-programming_websocket/index.md) for long-lived, bidirectional connections.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
