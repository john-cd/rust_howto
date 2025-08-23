# HTTP Clients

{{#include http_clients.incl.md}}{{hi:HTTP client}}

## `reqwest` {#reqwest}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}}
[![reqwest~crates.io][c~reqwest~crates.io~badge]][c~reqwest~crates.io]
[![reqwest~repo][c~reqwest~repo~badge]][c~reqwest~repo]
[![reqwest~lib.rs][c~reqwest~lib.rs~badge]][c~reqwest~lib.rs]

[`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} is a full-fat HTTP client. It can be used in both synchronous and asynchronous code. It requires the [`tokio`][c~tokio~docs]↗{{hi:tokio}} runtime.

Multiple examples using [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} are found in [[apis | APIs]], [[download | Download]], [[requests | Requests]].

## `ureq` {#ureq}

[![ureq][c~ureq~docs~badge]][c~ureq~docs]{{hi:ureq}}
[![ureq~crates.io][c~ureq~crates.io~badge]][c~ureq~crates.io]
[![ureq~repo][c~ureq~repo~badge]][c~ureq~repo]
[![ureq~lib.rs][c~ureq~lib.rs~badge]][c~ureq~lib.rs]
[![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

[`ureq`][c~ureq~docs]↗{{hi:ureq}} is a minimal synchronous HTTP client, focused on simplicity and minimizing dependencies.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/http_clients/ureq.rs:example}}
```

## Build a HTTP Client with `hyper` {#hyper}

[![hyper~website][c~hyper~website~badge]][c~hyper~website] [![hyper][c~hyper~docs~badge]][c~hyper~docs] [![hyper~crates.io][c~hyper~crates.io~badge]][c~hyper~crates.io] [![hyper~repo][c~hyper~repo~badge]][c~hyper~repo] [![hyper~lib.rs][c~hyper~lib.rs~badge]][c~hyper~lib.rs]{{hi:hyper}}{{hi:Http}}{{hi:hyper}}{{hi:Hyperium}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[`hyper`][c~hyper~docs]↗{{hi:hyper}} is a HTTP/1 and HTTP/2 implementation (both client and server) that works best with the [`tokio`][c~tokio~docs]↗{{hi:tokio}} async runtime, but can support other runtimes.

[`hyper`][c~hyper~docs]↗{{hi:hyper}} is meant to be a low-level building block and is indeed used by higher-level libraries such as [`curl`][c~curl~docs]↗{{hi:curl}}, [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}, and `aws-sdk`{{hi:aws-sdk}}. You will likely reach for [`hyper`][c~hyper~docs]↗{{hi:hyper}} to design such tools or access to bleeding-edge features (like HTTP/3).

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/http_clients/hyper.rs:example}}
```

## References

- [`hyper` Examples][c~hyper~examples~repo]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[http_clients: expand; link to hyper.md in server.](https://github.com/john-cd/rust_howto/issues/504)
</div>
