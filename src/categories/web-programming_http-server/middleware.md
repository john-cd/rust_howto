# Middleware

{{#include middleware.incl.md}}

## Tower

[![tower][c-tower-badge]][c-tower]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-server][cat-http-server-badge]][cat-http-server]

{{hi:Tower}}[`Tower`][c-tower]⮳ is a library of modular and reusable components for building robust networking clients and servers.

Tower provides a simple core abstraction, the {{hi:Service}}[`Service`][c-tower::Service]⮳ trait, which represents an asynchronous function taking a request and returning either a response or an error. It can be used to model both clients and servers.

An additional abstraction, the {{hi:Layer}}[`Layer`][c-tower::Layer]⮳ trait, is used to compose middleware with Services. A {{hi:Layer}}[`Layer`][c-tower::Layer]⮳ is a function taking a Service of one type and returning a Service of a different type. The {{hi:ServiceBuilder}}[`ServiceBuilder`][c-tower::ServiceBuilder]⮳ type is used to add middleware to a service by composing it with multiple Layers. The {{hi:Layer}}[`Layer`][c-tower::Layer]⮳ trait can be used to write reusable components that can be applied to very different kinds of services; for example, it can be applied to services operating on different protocols, and to both the client and server side of a network transaction.

A number of third-party libraries support {{hi:Tower}}[`Tower`][c-tower]⮳ and the {{hi:Service}}[`Service`][c-tower::Service]⮳ trait: {{hi:hyper}}[`hyper`][c-hyper-crates.io]⮳, {{hi:tonic}}[`tonic`][c-tonic-crates.io]⮳ ({{i:gRPC}}).

[![tower-middleware-from-scratch][tower-middleware-from-scratch-badge]][tower-middleware-from-scratch]

## Tower HTTP

[![tower_http][c-tower_http-badge]][c-tower_http]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-server][cat-http-server-badge]][cat-http-server]

{{hi:Tower HTTP}}[`Tower HTTP`][c-tower_http]⮳ contains HTTP specific Tower utilities.

```rust,noplayground,ignore
{{#include ../../../deps/tests/tower_http.rs}}
```

## Alternatives

[![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-server][cat-http-server-badge]][cat-http-server]

{{hi:Trillium}}[`Trillium`][c-trillium-website]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
