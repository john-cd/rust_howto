# Middleware

## Tower

[tower documentation]( https://docs.rs/tower/latest/tower/index.html )

Tower is a library of modular and reusable components for building robust networking clients and servers.

Tower provides a simple core abstraction, the `Service` trait, which represents an asynchronous function taking a request and returning either a response or an error. It can be used to model both clients and servers.

An additional abstraction, the `Layer` trait, is used to compose middleware with Services. A `Layer` is a function taking a Service of one type and returning a Service of a different type. The `ServiceBuilder` type is used to add middleware to a service by composing it with multiple Layers.
The `Layer` trait can be used to write reusable components that can be applied to very different kinds of services; for example, it can be applied to services operating on different protocols, and to both the client and server side of a network transaction.

A number of third-party libraries support `Tower` and the `Service` trait: [hyper]( https://crates.io/crates/hyper ), [tonic (gRPC)]( https://crates.io/crates/tonic ).

[Building a Tower middleware from scratch]( https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md )

## Tower HTTP

HTTP specific Tower utilities

[tower_http documentation]( https://docs.rs/tower-http/latest/tower_http/ )

```rust,ignore,noplayground
{{#include ../../deps/examples/tower_http.rs}}
```

## Alternatives

[Trillium]( https://trillium.rs/welcome )
