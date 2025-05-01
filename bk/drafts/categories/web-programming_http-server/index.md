# Web Serving

[![cat-web-programming::http-server][cat-web-programming::http-server-badge]][cat-web-programming::http-server]{{hi:HTTP server}}

Serve data over HTTP.

The following table outlines common tasks for building web servers in Rust and relevant crates. The Rust web server ecosystem offers a variety of options, from low-level networking to high-level frameworks.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Web Frameworks (Full-Stack) | [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`axum`][c-axum]⮳{{hi:axum}}, [`rocket`][c-rocket]⮳{{hi:rocket}}, [`warp`][c-warp]⮳{{hi:warp}}, [`tower-web`][c-tower_web]⮳{{hi:tower-web}} (less actively maintained) | These frameworks provide tools for building complete web applications, including routing, middleware, request handling, templating, and more. [`actix-web`][c-actix_web]⮳{{hi:actix-web}} is known for its performance. [`axum`][c-axum]⮳{{hi:axum}} is built on top of [`tower`][c-tower]⮳{{hi:tower}} and [`hyper`][c-hyper]⮳{{hi:hyper}}. [`rocket`][c-rocket]⮳{{hi:rocket}} uses a more declarative approach. [`warp`][c-warp]⮳{{hi:warp}} is a more lightweight framework. [`tower-web`][c-tower_web]⮳{{hi:tower-web}} is a framework built on top of [`tower`][c-tower]⮳{{hi:tower}}, but is less actively maintained. |
| HTTP Servers (Low-Level) | [`hyper`][c-hyper]⮳{{hi:hyper}} | [`hyper`][c-hyper]⮳{{hi:hyper}} is a low-level HTTP library that can be used to build custom web servers. Most frameworks use [`hyper`][c-hyper]⮳{{hi:hyper}} under the hood. |
| Routing and Handling Requests | Provided by web frameworks. | Web frameworks handle routing to map incoming requests to the appropriate handlers. |
| Middleware | [`tower`][c-tower]⮳{{hi:tower}}, often integrated into frameworks. | [`tower`][c-tower]⮳{{hi:tower}} is a crate for building composable middleware. Many web frameworks integrate with [`tower`][c-tower]⮳{{hi:tower}} or provide their own middleware systems. |
| Request Handling | Handled by web frameworks. | Web frameworks provide mechanisms for handling incoming requests, parsing data, and generating responses. |
| Response Generation | Handled by web frameworks. | Web frameworks provide ways to construct and send HTTP responses. |
| REST API Design | Often uses web frameworks and serialization crates. | REST APIs are typically built using web frameworks and serialization crates. |
| Static File Serving | Often provided by web frameworks or through crates like `serve-static`. | Serving static files is a common task, often handled by the framework or a dedicated crate. |

## Choosing the Right Crates

- Full-Stack Web App: [`axum`][c-axum]⮳{{hi:axum}}, [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`rocket`][c-rocket]⮳{{hi:rocket}}, [`warp`][c-warp]⮳{{hi:warp}}.
- REST API: Web framework + [`serde`][c-serde]⮳{{hi:serde}}.
- GraphQL API: [`juniper`][c-juniper]⮳{{hi:juniper}}, [`async-graphql`][c-async_graphql]⮳{{hi:async-graphql}}.
- Low-Level HTTP Server (Custom): [`hyper`][c-hyper]⮳{{hi:hyper}}.

Choose a web framework that fits your project's complexity and requirements. [`actix-web`][c-actix_web]⮳{{hi:actix-web}} is popular for performance-sensitive applications. [`axum`][c-axum]⮳{{hi:axum}} is a good choice for those familiar with [`tower`][c-tower]⮳{{hi:tower}} and [`hyper`][c-hyper]⮳{{hi:hyper}}. [`rocket`][c-rocket]⮳{{hi:rocket}} is a good choice for smaller projects or if you prefer a more declarative approach. [`warp`][c-warp]⮳{{hi:warp}} is very lightweight.

If your web server needs to interact with a database, choose a framework compatible with the appropriate database driver. Plan your authentication and authorization strategy in advance. You should also consider which templating engine (like [`Tera`][c-tera]⮳{{hi:Tera}} or [`Handlebars`][c-handlebars]⮳{{hi:Handlebars}}) works best with your web framework.

## Frameworks

### Axum

{{#include _axum.incl.md}}

### Actix Web

{{#include _actix.incl.md}}

### Batteries-included Frameworks

{{#include _batteries-included_frameworks.incl.md}}

### Other Frameworks

{{#include other_frameworks.incl.md}}

### Low-level Serving with `hyper`

{{#include _hyper.incl.md}}

## Middleware

Middleware is a powerful way to add functionality to your web server.

{{#include middleware.incl.md}}

### Cross-origin Resource Sharing

{{#include cors.incl.md}}

## Static Website Generators

{{#include static_website_generators.incl.md}}

## GraphQL

{{#include _graphql.incl.md}}

## gRPC

{{#include _grpc.incl.md}}

## See Also

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Asynchronous Programming (Essential for Web Servers) | [`tokio`][c-tokio]⮳{{hi:tokio}}, [`smol`][c-smol]⮳{{hi:smol}} | These asynchronous runtimes are fundamental for building efficient and scalable web servers in Rust. [`tokio`][c-tokio]⮳{{hi:tokio}} is the most widely used. |
| Networking (Low-Level) | `tokio::net`, `std::net` | These modules provide low-level networking primitives. Often used by higher-level frameworks. |
| Templating | [`minijinja`][c-minijinja]⮳{{hi:minijinja}}, [`tera`][c-tera]⮳{{hi:tera}}, [`handlebars`][c-handlebars]⮳{{hi:handlebars}}, [`askama`][c-askama]⮳{{hi:askama}} | Templating engines are used to generate HTML dynamically. |
| Serialization/Deserialization (JSON, etc.) | [`serde`][c-serde]⮳{{hi:serde}}, [`serde_json`][c-serde_json]⮳{{hi:serde_json}}, [`serde_yml`][c-serde_yml]⮳{{hi:serde_yml}} | [`serde`][c-serde]⮳{{hi:serde}} is a powerful framework for serializing and deserializing data, often used with JSON and other formats. |
| Databases | [`sqlx`][c-sqlx]⮳{{hi:sqlx}}, [`diesel`][c-diesel]⮳{{hi:diesel}}, [`mongodb`][c-mongodb]⮳{{hi:mongodb}} (drivers) | These crates provide database access for various database systems. |
| Authentication & Authorization | Often handled through middleware. | Authentication and authorization are often implemented using middleware or dedicated crates, sometimes provided by the framework. |
| WebSockets | [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}, [`async-tungstenite`][c-async_tungstenite]⮳{{hi:async-tungstenite}} | These crates provide WebSocket support. |
| GraphQL | [`juniper`][c-juniper]⮳{{hi:juniper}}, [`async-graphql`][c-async_graphql]⮳{{hi:async-graphql}} | These crates enable building GraphQL APIs. |
| Testing | Built-in testing framework, crates like [`reqwest`][c-reqwest]⮳{{hi:reqwest}} for integration testing. | Rust has a built-in testing framework, and crates like [`reqwest`][c-reqwest]⮳{{hi:reqwest}} can be used for integration testing of web servers. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write / review in depth NOW](https://github.com/john-cd/rust_howto/issues/977)

- [Cot](https://cot.rs)
- [Salvo](https://salvo.rs)

</div>
