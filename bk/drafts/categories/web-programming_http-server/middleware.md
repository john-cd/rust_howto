# Middleware

{{#include middleware.incl.md}}

## `tower` {#tower}

[![tower][c~tower~docs~badge]][c~tower~docs]{{hi:tower}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[`tower`][c~tower~docs]↗{{hi:tower}} is a library of modular and reusable components for building robust networking clients and servers.

Tower provides a simple core abstraction, the [`tower::Service`][c~tower::Service~docs]↗{{hi:tower::Service}} trait, which represents an asynchronous function taking a request and returning either a response or an error. It can be used to model both clients and servers.

An additional abstraction, the [`tower::Layer`][c~tower::Layer~docs]↗{{hi:tower::Layer}} trait, is used to compose middleware with Services. A [`tower::Layer`][c~tower::Layer~docs]↗{{hi:tower::Layer}} is a function taking a Service of one type and returning a Service of a different type. The [`tower::ServiceBuilder`][c~tower::ServiceBuilder~docs]↗{{hi:tower::ServiceBuilder}} type is used to add middleware to a service by composing it with multiple Layers. The [`tower::Layer`][c~tower::Layer~docs]↗{{hi:tower::Layer}} trait can be used to write reusable components that can be applied to very different kinds of services; for example, it can be applied to services operating on different protocols, and to both the client and server side of a network transaction.

A number of third-party libraries support [`tower`][c~tower~docs]↗{{hi:tower}} and the [`tower::Service`][c~tower::Service~docs]↗{{hi:tower::Service}} trait: [`hyper`][c~hyper~crates.io]↗{{hi:hyper}}, [`tonic`][c~tonic~crates.io]↗{{hi:tonic}} (gRPC{{hi:gRPC}}).

[![tower-middleware-from-scratch][tower-middleware-from-scratch~repo~badge]][tower-middleware-from-scratch~repo]

## `tower-http` {#tower-http}

[![tower-http][c~tower-http~docs~badge]][c~tower-http~docs]{{hi:tower-http}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[`Tower HTTP`][c~tower-http~docs]↗{{hi:tower-http}} contains HTTP specific Tower utilities.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/middleware/tower_http.rs:example}}
```

## Investigate Alternatives to `tower` {#alternatives}

[![trillium][c~trillium~docs~badge]][c~trillium~docs] [![trillium~crates.io][c~trillium~crates.io~badge]][c~trillium~crates.io] [![trillium~repo][c~trillium~repo~badge]][c~trillium~repo] [![trillium~lib.rs][c~trillium~lib.rs~badge]][c~trillium~lib.rs]{{hi:trillium}}{{hi:Async}}{{hi:trillium}}{{hi:Framework}}[![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}{{hi:HTTP server}}

[`trillium`][c~trillium~docs]↗{{hi:trillium}} is a modular toolkit for building async web apps

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/978)
</div>
