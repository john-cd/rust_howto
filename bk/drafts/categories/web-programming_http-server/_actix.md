<!-- TODO Reviewed ??-??-2026 -->

# Actix Web

{{#include _actix.incl.md}}

## Create a web Server with Actix Web {#actix}

[![actix-web~website][c~actix-web~website~badge]][c~actix-web~website] {{#badges actix-web}} {{hi:Actix}} {{hi:Async}} {{hi:Framework}} {{hi:Http}} {{hi:Web}} {{#categories asynchronous=Asynchronous,network-programming="Network programming",web-programming::http-server="HTTP server",web-programming::websocket=WebSocket}}

[`actix-web`][c~actix-web~docs]↗{{hi:actix-web}} is an asynchronous HTTP framework. A handler is an async function that returns a response or another value implementing `Responder`; the `#[get]` macro in this example registers a handler for `GET /`. `HttpServer::new` takes an application factory, `.bind` selects the listening socket, and `.run().await` starts serving requests. Actix Web creates an application instance for each worker, so use the application factory to configure per-worker services and state.

The example binds to `127.0.0.1:8080` for local access. The accompanying test initializes the `App` directly and exercises the handler without opening a network listener. Select a framework by the APIs, middleware, runtime, and integration points your application needs; compare performance only with measurements for the workload you intend to serve.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/actix_web.rs:example}}
```

## References {#references .skip}

- [Actix examples][c~actix~examples~repo]↗.
- [Practical Rust Web Development - API Rest][blog~practical-rust]↗.
- ["Real world" example with Actix Web][realworld~actix-fullstack~repo]↗.
- ["Real world" example with Actix Web and Diesel][realworld~v1-rust-actix-web-diesel~repo]↗.

## Related Topics {#related-topics .skip}

- [[asynchronous | Asynchronous Programming]] for async runtimes and task execution.
- [[network-programming | Network Programming]] for TCP listeners and lower-level sockets.
- [[web-programming_http-server | HTTP Servers]] for Axum, Hyper, GraphQL, gRPC, and middleware.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  actix: organize / write
</div>
