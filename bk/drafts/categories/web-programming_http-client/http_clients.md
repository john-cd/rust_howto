<!-- TODO Reviewed ??-??-2026 -->
# HTTP Clients

{{#include http_clients.incl.md}}{{hi:HTTP client}}

## `reqwest` {#reqwest}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}}
[![reqwest~crates.io][c~reqwest~crates.io~badge]][c~reqwest~crates.io]
[![reqwest~repo][c~reqwest~repo~badge]][c~reqwest~repo]
[![reqwest~lib.rs][c~reqwest~lib.rs~badge]][c~reqwest~lib.rs]

[`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} is a high-level HTTP client with asynchronous and blocking APIs. The asynchronous `Client` runs on an async runtime; this package uses [`tokio`][c~tokio~docs]↗{{hi:tokio}}. The blocking API is enabled by the `blocking` feature and should not be called from inside an async runtime.

For a single request, the top-level `reqwest::get` and `reqwest::blocking::get` helpers are convenient. For repeated requests, create a `Client` once and reuse it so connections can be pooled. Configure timeouts and other client-wide behavior with `Client::builder`; call `error_for_status()` when an HTTP error status should become a Rust error, since receiving a response is separate from deciding whether its status is successful.

### Asynchronous GET {#reqwest-async-get}

This example uses the asynchronous API to print a response. For structured JSON, custom headers, pagination, and more request patterns, see [[apis | APIs]], [[requests | Requests]], and [[download | Download]].

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/requests/get1.rs:example}}
```

### Blocking GET {#reqwest-blocking-get}

Use the blocking API in synchronous code. Keep it off async runtime worker threads; if a synchronous operation must be called from async code, use the runtime's blocking-work facility.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/requests/get.rs:example}}
```

## `ureq` {#ureq}

[![ureq][c~ureq~docs~badge]][c~ureq~docs]{{hi:ureq}}
[![ureq~crates.io][c~ureq~crates.io~badge]][c~ureq~crates.io]
[![ureq~repo][c~ureq~repo~badge]][c~ureq~repo]
[![ureq~lib.rs][c~ureq~lib.rs~badge]][c~ureq~lib.rs]
[![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

[`ureq`][c~ureq~docs]↗{{hi:ureq}} is a synchronous HTTP client for code that does not need to await requests. `ureq::get` is convenient for one-off calls. For repeated calls or shared settings such as timeouts, create and reuse a `ureq::Agent`; it owns reusable client configuration and connections. It does not require an async runtime.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/http_clients/ureq.rs:example}}
```

## Build an HTTP Client with `hyper` {#hyper}

[![hyper~website][c~hyper~website~badge]][c~hyper~website] [![hyper][c~hyper~docs~badge]][c~hyper~docs] [![hyper~crates.io][c~hyper~crates.io~badge]][c~hyper~crates.io] [![hyper~repo][c~hyper~repo~badge]][c~hyper~repo] [![hyper~lib.rs][c~hyper~lib.rs~badge]][c~hyper~lib.rs]{{hi:hyper}}{{hi:Http}}{{hi:hyper}}{{hi:Hyperium}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[`hyper`][c~hyper~docs]↗{{hi:hyper}} implements HTTP/1 and HTTP/2 and exposes low-level client and server connection APIs. Hyper 1 moved the higher-level pooled client to [`hyper-util`][c~hyper-util~docs]↗{{hi:hyper-util}}. Choose that lower-level stack when you need to control the connection and protocol integration; choose a higher-level client such as `reqwest` for common application requests. The example below opens one HTTP/1 connection with Tokio. It uses plain HTTP and does not configure TLS for HTTPS.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/http_clients/hyper.rs:example}}
```

## References {#references .skip}

- [`hyper` Examples][c~hyper~examples~repo]↗.

## Related Topics {#related-topics .skip}

- [[asynchronous | Asynchronous Programming]] for async runtimes and I/O.
- [[network-programming | Network Programming]] for TCP and lower-level connections.
- [[web-programming_http-server | HTTP Servers]] for server-side Hyper, Axum, and Actix Web examples.
- [Hyper server implementation][p~hyper] for the matching server-side connection example.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[http_clients: expand; link to hyper.md in server.](https://github.com/john-cd/rust_howto/issues/504)
</div>
