# Hyper

{{#include hyper.incl.md}}

## Implement an HTTP API with `hyper` {#hyper}

[![hyper][c-hyper-badge]][c-hyper]{{hi:hyper}}{{hi:HTTP server}}
[![hyper-crates.io][c-hyper-crates.io-badge]][c-hyper-crates.io]
[![hyper-github][c-hyper-github-badge]][c-hyper-github]
[![hyper-lib.rs][c-hyper-lib.rs-badge]][c-hyper-lib.rs]

[`hyper`][c-hyper]⮳{{hi:hyper}} is a low-level asynchronous HTTP implementation (both client and server). It implements HTTP/1 and HTTP/2. It works best with the [`tokio`][c-tokio]⮳{{hi:tokio}} async runtime, but can support other runtimes.

[`hyper`][c-hyper]⮳{{hi:hyper}} is meant to be a building block for libraries and applications, for uses cases including:

- Low-level client libraries (e.g. `curl`, [`reqwest`][c-reqwest]⮳{{hi:reqwest}}, `aws-sdk`),
- Web server frameworks (e.g. `deno`, [`axum`][c-axum]⮳{{hi:axum}}),
- Services and Proxies (e.g. `linkerd`, [`cloudflare`][c-cloudflare]⮳{{hi:cloudflare}}, [`fastly`][fastly]⮳{{hi:fastly}} ).
- Use of bleeding-edge protocols e.g HTTP/3.

For other server use cases, please consider higher-level, easier libraries (like [`axum`][c-axum]⮳{{hi:axum}}). Reach for the [`reqwest`][c-reqwest]⮳{{hi:reqwest}} crate, if looking for a convenient HTTP client.

Note that [`hyper`][c-hyper]⮳{{hi:hyper}} developers need to bring their own async runtime (e.g. Tokio), IO, and optionally TLS. Look at the [`hyper-tls`](https://crates.io/crates/hyper-tls), [`hyper-rustls`](https://crates.io/crates/hyper-rustls) and [`hyper-openssl`](https://crates.io/crates/hyper-openssl) crates for the latter. Routing, cookies, and non-HTTP protocols are out of scope. In particular, consider the family of [`tungstenite`][c-tungstenite]⮳{{hi:tungstenite}} crates for WebSockets.

This said, we provide below an example of a HTTP server, following the recommendations of the [`hyper.rs`](http://hyper.rs/) guide:

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/tests/hyper_server.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[hyper: write (P1)](https://github.com/john-cd/rust_howto/issues/515)

Link:
[hello-world/](https://hyper.rs/guides/1/server/hello-world/)
[examples](https://github.com/hyperium/hyper/tree/master/examples)

</div>
