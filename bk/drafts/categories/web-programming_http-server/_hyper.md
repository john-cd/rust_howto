# Hyper

{{#include hyper.incl.md}}

## Implement an HTTP API with `hyper` {#hyper}

[![hyper][c~hyper~docs~badge]][c~hyper~docs]{{hi:hyper}}{{hi:HTTP server}}
[![hyper~crates.io][c~hyper~crates.io~badge]][c~hyper~crates.io]
[![hyper~github][c~hyper~github~badge]][c~hyper~github]
[![hyper~lib.rs][c~hyper~lib.rs~badge]][c~hyper~lib.rs]

[`hyper`][c~hyper~docs]↗{{hi:hyper}} is a low-level asynchronous HTTP implementation (both client and server). It implements HTTP/1 and HTTP/2. It works best with the [`tokio`][c~tokio~docs]↗{{hi:tokio}} async runtime, but can support other runtimes.

[`hyper`][c~hyper~docs]↗{{hi:hyper}} is meant to be a building block for libraries and applications, for uses cases including:

- Low-level client libraries (e.g. [`curl`][c~curl~docs]↗{{hi:curl}}, [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}, `aws-sdk`).
- Web server frameworks (e.g. [`deno`][c~deno~docs]↗{{hi:deno}}, [`axum`][c~axum~docs]↗{{hi:axum}}).
- Services and Proxies (e.g. `linkerd`, [`cloudflare`][c~cloudflare~docs]↗{{hi:cloudflare}}, [`fastly`][fastly]↗{{hi:fastly}} ).
- Use of bleeding-edge protocols e.g HTTP/3.

For other server use cases, please consider higher-level, easier libraries (like [`axum`][c~axum~docs]↗{{hi:axum}}). Reach for the [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} crate, if looking for a convenient HTTP client.

Note that [`hyper`][c~hyper~docs]↗{{hi:hyper}} developers need to bring their own async runtime (e.g. Tokio), IO, and optionally TLS. Look at the [`hyper-tls`](https://crates.io/crates/hyper-tls), [`hyper-rustls`](https://crates.io/crates/hyper-rustls) and [`hyper-openssl`][hyper-openssl] crates for the latter. Routing, cookies, and non-HTTP protocols are out of scope. In particular, consider the family of [`tungstenite`][c~tungstenite~docs]↗{{hi:tungstenite}} crates for WebSockets.

This said, we provide below an example of a HTTP server, following the recommendations of the [`hyper.rs`][c~hyper~website] guide:

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/hyper_server.rs:example}}
```

## `hyper` TLS Support {#skip}

[![hyper-tls~website][c~hyper-tls~website~badge]][c~hyper-tls~website] [![hyper-tls][c~hyper-tls~docs~badge]][c~hyper-tls~docs] [![hyper-tls~crates.io][c~hyper-tls~crates.io~badge]][c~hyper-tls~crates.io] [![hyper-tls~github][c~hyper-tls~github~badge]][c~hyper-tls~github] [![hyper-tls~lib.rs][c~hyper-tls~lib.rs~badge]][c~hyper-tls~lib.rs]{{hi:hyper-tls}}{{hi:Http}}{{hi:Https}}{{hi:Hyper}}{{hi:Ssl}}{{hi:Tls}}

Default TLS implementation for use with `hyper`.

[![hyper-rustls][c~hyper-rustls~docs~badge]][c~hyper-rustls~docs] [![hyper-rustls~crates.io][c~hyper-rustls~crates.io~badge]][c~hyper-rustls~crates.io] [![hyper-rustls~github][c~hyper-rustls~github~badge]][c~hyper-rustls~github] [![hyper-rustls~lib.rs][c~hyper-rustls~lib.rs~badge]][c~hyper-rustls~lib.rs]{{hi:hyper-rustls}}

Rustls + [`hyper`][c~hyper~docs]↗{{hi:hyper}} integration for pure rust HTTPS.

[![hyper-openssl][c~hyper-openssl~docs~badge]][c~hyper-openssl~docs] [![hyper-openssl~crates.io][c~hyper-openssl~crates.io~badge]][c~hyper-openssl~crates.io] [![hyper-openssl~github][c~hyper-openssl~github~badge]][c~hyper-openssl~github] [![hyper-openssl~lib.rs][c~hyper-openssl~lib.rs~badge]][c~hyper-openssl~lib.rs]{{hi:hyper-openssl}}

Hyper TLS support via OpenSSL.

## References {#references}

- [`hyper` examples][c~hyper~examples~github]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[hyper: fix](https://github.com/john-cd/rust_howto/issues/515)
</div>
