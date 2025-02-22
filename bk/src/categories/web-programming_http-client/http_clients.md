# HTTP clients

{{#include http_clients.incl.md}}{{hi:HTTP client}}

## `reqwest` {#reqwest}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}}
[![reqwest-crates.io][c-reqwest-crates.io-badge]][c-reqwest-crates.io]
[![reqwest-github][c-reqwest-github-badge]][c-reqwest-github]
[![reqwest-lib.rs][c-reqwest-lib.rs-badge]][c-reqwest-lib.rs]

[`reqwest`][c-reqwest]⮳{{hi:reqwest}} is a full-fat HTTP client. It can be used in both synchronous and asynchronous code. It requires the [`tokio`][c-tokio]⮳{{hi:tokio}} runtime.

## `ureq` {#ureq}

[![ureq][c-ureq-badge]][c-ureq]{{hi:ureq}}
[![ureq-crates.io][c-ureq-crates.io-badge]][c-ureq-crates.io]
[![ureq-github][c-ureq-github-badge]][c-ureq-github]
[![ureq-lib.rs][c-ureq-lib.rs-badge]][c-ureq-lib.rs]
[![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]{{hi:HTTP client}}

[`ureq`][c-ureq]⮳{{hi:ureq}} is a minimal synchronous HTTP client, focused on simplicity and minimizing dependencies.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/tests/other/ureq.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

## Build a HTTP client with `hyper` {#hyper}

[![hyper][c-hyper-badge]][c-hyper]{{hi:hyper}}{{hi:HTTP client}}{{hi:HTTP server}}
[![hyper-crates.io][c-hyper-crates.io-badge]][c-hyper-crates.io]
[![hyper-github][c-hyper-github-badge]][c-hyper-github]
[![hyper-lib.rs][c-hyper-lib.rs-badge]][c-hyper-lib.rs]

[`hyper`][c-hyper]⮳{{hi:hyper}} is a HTTP/1 and HTTP/2 implementation (both client and server) that works best with the [`tokio`][c-tokio]⮳{{hi:tokio}} async runtime, but can support other runtimes.

[`hyper`][c-hyper]⮳{{hi:hyper}} is meant to be a low-level building block and is indeed used by higher-level libraries such as [`curl`][c-curl]⮳{{hi:curl}}, [`reqwest`][c-reqwest]⮳{{hi:reqwest}}, and `aws-sdk`. You will likely reach for [`hyper`][c-hyper]⮳{{hi:hyper}} to design such tools or access to bleeding-edge features (like HTTP/3).

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/tests/other/hyper.rs:example}}
```

[http_clients: expand (P1)](https://github.com/john-cd/rust_howto/issues/504)
See hyper.md in server.
[hyper.rs](http://hyper.rs/)
[examples](https://github.com/hyperium/hyper/tree/master/examples)
</div>
