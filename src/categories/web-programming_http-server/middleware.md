# Middleware

{{#include middleware.incl.md}}

## Tower

[![tower][c-tower-badge]][c-tower]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-server][cat-http-server-badge]][cat-http-server]

[`tower`][c-tower]{{hi:tower}}⮳ is a library of modular and reusable components for building robust networking clients and servers.

Tower provides a simple core abstraction, the [`tower::Service`][c-tower::Service]{{hi:tower::Service}}⮳ trait, which represents an asynchronous function taking a request and returning either a response or an error. It can be used to model both clients and servers.

An additional abstraction, the [`tower::Layer`][c-tower::Layer]{{hi:tower::Layer}}⮳ trait, is used to compose middleware with Services. A [`tower::Layer`][c-tower::Layer]{{hi:tower::Layer}}⮳ is a function taking a Service of one type and returning a Service of a different type. The [`tower::ServiceBuilder`][c-tower::ServiceBuilder]{{hi:tower::ServiceBuilder}}⮳ type is used to add middleware to a service by composing it with multiple Layers. The [`tower::Layer`][c-tower::Layer]{{hi:tower::Layer}}⮳ trait can be used to write reusable components that can be applied to very different kinds of services; for example, it can be applied to services operating on different protocols, and to both the client and server side of a network transaction.

A number of third-party libraries support [`tower`][c-tower]{{hi:tower}}⮳ and the [`tower::Service`][c-tower::Service]{{hi:tower::Service}}⮳ trait: [`hyper`][c-hyper-crates.io]{{hi:hyper}}⮳, [`tonic`][c-tonic-crates.io]{{hi:tonic}}⮳ (gRPC{{hi:gRPC}}).

[![tower-middleware-from-scratch][tower-middleware-from-scratch-badge]][tower-middleware-from-scratch]

## Tower HTTP

[![tower_http][c-tower_http-badge]][c-tower_http]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-server][cat-http-server-badge]][cat-http-server]

[`Tower HTTP`][c-tower_http]{{hi:Tower HTTP}}⮳ contains HTTP specific Tower utilities.

```rust,noplayground,ignore
{{#include ../../../deps/tests/tower_http.rs}}
```

## Alternatives

[![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-server][cat-http-server-badge]][cat-http-server]

[`trillium`][c-trillium-website]{{hi:trillium}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
<div class="hidden">
TODO:
</div>
