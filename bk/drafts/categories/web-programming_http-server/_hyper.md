# Hyper

{{#include hyper.incl.md}}

## Implement an HTTP API with `hyper` {#hyper}

[![hyper][c-hyper-badge]][c-hyper]{{hi:hyper}}{{hi:HTTP server}}
[![hyper-crates.io][c-hyper-crates.io-badge]][c-hyper-crates.io]
[![hyper-github][c-hyper-github-badge]][c-hyper-github]
[![hyper-lib.rs][c-hyper-lib.rs-badge]][c-hyper-lib.rs]

[`hyper`][c-hyper]⮳{{hi:hyper}} is a low-level asynchronous HTTP implementation (both client and server). It implements HTTP/1 and HTTP/2. It works best with the [`tokio`][c-tokio]⮳{{hi:tokio}} async runtime, but can support other runtimes.

[`hyper`][c-hyper]⮳{{hi:hyper}} is meant to be a building block for libraries and applications, for uses cases including:

- Low-level client libraries (e.g. [`curl`][c-curl]⮳{{hi:curl}}, [`reqwest`][c-reqwest]⮳{{hi:reqwest}}, `aws-sdk`).
- Web server frameworks (e.g. [`deno`][c-deno]⮳{{hi:deno}}, [`axum`][c-axum]⮳{{hi:axum}}).
- Services and Proxies (e.g. `linkerd`, [`cloudflare`][c-cloudflare]⮳{{hi:cloudflare}}, [`fastly`][fastly]⮳{{hi:fastly}} ).
- Use of bleeding-edge protocols e.g HTTP/3.

For other server use cases, please consider higher-level, easier libraries (like [`axum`][c-axum]⮳{{hi:axum}}). Reach for the [`reqwest`][c-reqwest]⮳{{hi:reqwest}} crate, if looking for a convenient HTTP client.

Note that [`hyper`][c-hyper]⮳{{hi:hyper}} developers need to bring their own async runtime (e.g. Tokio), IO, and optionally TLS. Look at the [`hyper-tls`](https://crates.io/crates/hyper-tls), [`hyper-rustls`](https://crates.io/crates/hyper-rustls) and [`hyper-openssl`][hyper-openssl] crates for the latter. Routing, cookies, and non-HTTP protocols are out of scope. In particular, consider the family of [`tungstenite`][c-tungstenite]⮳{{hi:tungstenite}} crates for WebSockets.

This said, we provide below an example of a HTTP server, following the recommendations of the [`hyper.rs`][c-hyper-website] guide:

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/tests/hyper_server.rs:example}}
```

## `hyper` TLS Support {#skip}

[![hyper-tls-website][c-hyper_tls-website-badge]][c-hyper_tls-website] [![hyper-tls][c-hyper_tls-badge]][c-hyper_tls] [![hyper-tls-crates.io][c-hyper_tls-crates.io-badge]][c-hyper_tls-crates.io] [![hyper-tls-github][c-hyper_tls-github-badge]][c-hyper_tls-github] [![hyper-tls-lib.rs][c-hyper_tls-lib.rs-badge]][c-hyper_tls-lib.rs]{{hi:hyper-tls}}{{hi:Http}}{{hi:Https}}{{hi:Hyper}}{{hi:Ssl}}{{hi:Tls}}

Default TLS implementation for use with `hyper`.

[![hyper-rustls][c-hyper_rustls-badge]][c-hyper_rustls] [![hyper-rustls-crates.io][c-hyper_rustls-crates.io-badge]][c-hyper_rustls-crates.io] [![hyper-rustls-github][c-hyper_rustls-github-badge]][c-hyper_rustls-github] [![hyper-rustls-lib.rs][c-hyper_rustls-lib.rs-badge]][c-hyper_rustls-lib.rs]{{hi:hyper-rustls}}

Rustls + `hyper` integration for pure rust HTTPS.

[![hyper-openssl][c-hyper_openssl-badge]][c-hyper_openssl] [![hyper-openssl-crates.io][c-hyper_openssl-crates.io-badge]][c-hyper_openssl-crates.io] [![hyper-openssl-github][c-hyper_openssl-github-badge]][c-hyper_openssl-github] [![hyper-openssl-lib.rs][c-hyper_openssl-lib.rs-badge]][c-hyper_openssl-lib.rs]{{hi:hyper-openssl}}

Hyper TLS support via OpenSSL.

## References {#skip}

- [`hyper` examples][c-hyper-examples-github]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[hyper: fix](https://github.com/john-cd/rust_howto/issues/515)
</div>
