# Middleware

{{#include middleware.incl.md}}

## Tower

[![tower][tower-badge]][tower]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-server][cat-http-server-badge]][cat-http-server]

[`Tower`][tower]⮳ is a library of modular and reusable components for building robust networking clients and servers.

Tower provides a simple core abstraction, the [`Service`][tower::Service]⮳ trait, which represents an asynchronous function taking a request and returning either a response or an error. It can be used to model both clients and servers.

An additional abstraction, the [`Layer`][tower::Layer]⮳ trait, is used to compose middleware with Services. A [`Layer`][tower::Layer]⮳ is a function taking a Service of one type and returning a Service of a different type. The [`ServiceBuilder`][tower::ServiceBuilder]⮳ type is used to add middleware to a service by composing it with multiple Layers. The [`Layer`][tower::Layer]⮳ trait can be used to write reusable components that can be applied to very different kinds of services; for example, it can be applied to services operating on different protocols, and to both the client and server side of a network transaction.

A number of third-party libraries support [`Tower`][tower]⮳ and the [`Service`][tower::Service]⮳ trait: [`hyper`][hyper-crate]⮳, [`tonic`][tonic-crate]⮳ ({{i:gRPC}}).

[![tower-middleware-from-scratch][tower-middleware-from-scratch-badge]][tower-middleware-from-scratch]

## Tower HTTP

[![tower-http][tower-http-badge]][tower-http]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-server][cat-http-server-badge]][cat-http-server]

[`Tower HTTP`][tower-http]⮳ contains HTTP specific Tower utilities.

```rust,noplayground,ignore
{{#include ../../../deps/tests/tower_http.rs}}
```

## Alternatives

[![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-server][cat-http-server-badge]][cat-http-server]

[`Trillium`][trillium-website]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
