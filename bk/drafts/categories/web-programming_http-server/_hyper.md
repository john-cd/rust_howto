<!-- TODO Reviewed ??-??-2026 -->

# Hyper

{{#include _hyper.incl.md}}

## Implement an HTTP API with `hyper` {#hyper}

[![hyper][c~hyper~docs~badge]][c~hyper~docs]{{hi:hyper}}{{hi:HTTP server}}
[![hyper~crates.io][c~hyper~crates.io~badge]][c~hyper~crates.io]
[![hyper~repo][c~hyper~repo~badge]][c~hyper~repo]
[![hyper~lib.rs][c~hyper~lib.rs~badge]][c~hyper~lib.rs]

[`hyper`][c~hyper~docs]↗{{hi:hyper}} is a low-level HTTP implementation for clients and servers. It supports HTTP/1 and HTTP/2 and does not require a particular async runtime. This example uses [`tokio`][c~tokio~docs]↗{{hi:tokio}} and the `hyper-util` Tokio I/O adapter.

Hyper provides protocol and connection building blocks. An application supplies the listener, runtime, request service, and any TLS configuration. Use a higher-level framework such as [`axum`][c~axum~docs]↗{{hi:axum}} or [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}} when you want framework-provided routing and middleware.

`hyper` is a building block for low-level client libraries (for example [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}, the AWS SDK, and `curl`'s Rust bindings), web frameworks (for example [`axum`][c~axum~docs]↗{{hi:axum}} and Deno), and services and proxies (for example Linkerd, Cloudflare, and Fastly). Routing, cookies, and non-HTTP protocols are out of scope; for WebSockets, consider the [`tungstenite`][c~tungstenite~docs]↗{{hi:tungstenite}} family of crates.

The server below binds to loopback port 3000, accepts connections, adapts each Tokio stream with `TokioIo`, and serves an HTTP/1 connection. Its small service responds to `GET /`, `GET /health`, unknown paths, and other methods. It is an example server; use a framework for production routing, middleware, and deployment features.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/hyper_server.rs:example}}
```

## `hyper` TLS Support {#hyper-tls-support .skip}

[![hyper-tls~website][c~hyper-tls~website~badge]][c~hyper-tls~website] [![hyper-tls][c~hyper-tls~docs~badge]][c~hyper-tls~docs] [![hyper-tls~crates.io][c~hyper-tls~crates.io~badge]][c~hyper-tls~crates.io] [![hyper-tls~repo][c~hyper-tls~repo~badge]][c~hyper-tls~repo] [![hyper-tls~lib.rs][c~hyper-tls~lib.rs~badge]][c~hyper-tls~lib.rs]{{hi:hyper-tls}}{{hi:Http}}{{hi:Https}}{{hi:Hyper}}{{hi:Ssl}}{{hi:Tls}}

[`hyper-tls`][c~hyper-tls~crates.io]↗ is the default TLS implementation for use with Hyper (native TLS). Hyper does not configure TLS itself. TLS adapters such as `hyper-rustls`, `hyper-tls`, and `hyper-openssl` are separate crates; check that an adapter supports the same Hyper major version as your application.

{{#badges hyper-rustls}}

Rustls integration for HTTPS with Hyper, in pure Rust.

{{#badges hyper-openssl}}

TLS support for Hyper through OpenSSL.

## References {#references .skip}

- [`hyper` examples][c~hyper~examples~repo]↗.

## Related Topics {#related-topics .skip}

- [TCP Listener Servers](../network-programming/server.md) for lower-level networking.
- [Asynchronous Programming](../asynchronous/index.md) for Tokio and async I/O concepts.
- [Higher-Level HTTP Server Frameworks](index.md) for Axum, Actix Web, middleware, and routing.
- [HTTP Clients](../web-programming_http-client/index.md) for client-side requests and convenient HTTP APIs.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  hyper: fix
</div>
