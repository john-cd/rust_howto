<!-- TODO Reviewed ??-??-2026 -->
# Hyper

{{#include hyper.incl.md}}

## Implement an HTTP API with `hyper` {#hyper}

[![hyper][c~hyper~docs~badge]][c~hyper~docs]{{hi:hyper}}{{hi:HTTP server}}
[![hyper~crates.io][c~hyper~crates.io~badge]][c~hyper~crates.io]
[![hyper~repo][c~hyper~repo~badge]][c~hyper~repo]
[![hyper~lib.rs][c~hyper~lib.rs~badge]][c~hyper~lib.rs]

[`hyper`][c~hyper~docs]↗{{hi:hyper}} is a low-level HTTP implementation for clients and servers. It supports HTTP/1 and HTTP/2 and does not require a particular async runtime. This example uses [`tokio`][c~tokio~docs]↗{{hi:tokio}} and the `hyper-util` Tokio I/O adapter.

Hyper provides protocol and connection building blocks. An application supplies the listener, runtime, request service, and any TLS configuration. Use a higher-level framework such as [`axum`][c~axum~docs]↗{{hi:axum}} or [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}} when you want framework-provided routing and middleware.

The server below binds to loopback port 3000, accepts connections, adapts each Tokio stream with `TokioIo`, and serves an HTTP/1 connection. Its small service responds to `GET /`, `GET /health`, unknown paths, and other methods. It is an example server; use a framework for production routing, middleware, and deployment features.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/hyper_server.rs:example}}
```

## `hyper` TLS Support {#hyper-tls-support .skip}

[![hyper-tls~website][c~hyper-tls~website~badge]][c~hyper-tls~website] [![hyper-tls][c~hyper-tls~docs~badge]][c~hyper-tls~docs] [![hyper-tls~crates.io][c~hyper-tls~crates.io~badge]][c~hyper-tls~crates.io] [![hyper-tls~repo][c~hyper-tls~repo~badge]][c~hyper-tls~repo] [![hyper-tls~lib.rs][c~hyper-tls~lib.rs~badge]][c~hyper-tls~lib.rs]{{hi:hyper-tls}}{{hi:Http}}{{hi:Https}}{{hi:Hyper}}{{hi:Ssl}}{{hi:Tls}}

Hyper does not configure TLS itself. TLS adapters such as `hyper-rustls`, `hyper-tls`, and `hyper-openssl` are separate crates; check that an adapter supports the same Hyper major version as your application.

[![hyper-rustls][c~hyper-rustls~docs~badge]][c~hyper-rustls~docs] [![hyper-rustls~crates.io][c~hyper-rustls~crates.io~badge]][c~hyper-rustls~crates.io] [![hyper-rustls~repo][c~hyper-rustls~repo~badge]][c~hyper-rustls~repo] [![hyper-rustls~lib.rs][c~hyper-rustls~lib.rs~badge]][c~hyper-rustls~lib.rs]{{hi:hyper-rustls}}

Rustls integration for HTTPS with Hyper.

[![hyper-openssl][c~hyper-openssl~docs~badge]][c~hyper-openssl~docs] [![hyper-openssl~crates.io][c~hyper-openssl~crates.io~badge]][c~hyper-openssl~crates.io] [![hyper-openssl~repo][c~hyper-openssl~repo~badge]][c~hyper-openssl~repo] [![hyper-openssl~lib.rs][c~hyper-openssl~lib.rs~badge]][c~hyper-openssl~lib.rs]{{hi:hyper-openssl}}

OpenSSL integration for HTTPS with Hyper.

## References {#references .skip}

- [`hyper` examples][c~hyper~examples~repo]↗.

## Related Topics {#related-topics .skip}

- [[network-programming/server | TCP Listener Servers]] for lower-level networking.
- [[asynchronous | Asynchronous Programming]] for Tokio and async I/O concepts.
- [[web-programming_http-server | Higher-Level HTTP Server Frameworks]] for Axum, Actix Web, middleware, and routing.
- [[web-programming_http-client | HTTP Clients]] for client-side requests and convenient HTTP APIs.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[hyper: fix](https://github.com/john-cd/rust_howto/issues/515)
[content audit: content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1904)
</div>
