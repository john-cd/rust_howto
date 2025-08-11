# Web Programming

[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]

Create applications for the Web.

The following table outlines common web development tasks and relevant Rust crates. The Rust web ecosystem offers a variety of options, from low-level networking to high-level frameworks.

| Topic | Rust Crates | Notes |
|---|---|---|
| Web Frameworks (Full-Stack) | [`axum`][c~axum~docs]↗{{hi:axum}}, [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}}, [`rocket`][c~rocket~docs]↗{{hi:rocket}}, [`warp`][c~warp~docs]↗{{hi:warp}} | These frameworks provide tools for building web applications, including routing, middleware, templating, and more. [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}} is known for its performance. [`axum`][c~axum~docs]↗{{hi:axum}} is built on top of [`tower`][c~tower~docs]↗{{hi:tower}} and [`hyper`][c~hyper~docs]↗{{hi:hyper}}. [`rocket`][c~rocket~docs]↗{{hi:rocket}} uses a more declarative approach. [`warp`][c~warp~docs]↗{{hi:warp}} is a more lightweight framework. |
| Front-end Frameworks (WASM) | [`yew`][c~yew~docs]↗{{hi:yew}}, [`leptos`][c~leptos~docs]↗{{hi:leptos}}, [`seed`][c~seed~docs]↗{{hi:seed}}, [`dominator`][c~dominator~docs]↗{{hi:dominator}} | These frameworks enable building interactive web UIs with Rust compiled to WebAssembly (WASM). They provide component-based architectures and other tools for structuring front-end applications. |
| HTTP Servers | Covered by web frameworks. | Web frameworks typically handle HTTP server functionality. |
| REST API Design | Often uses web frameworks and serialization crates like [`serde`][c~serde~docs]↗{{hi:serde}}. | REST APIs are typically built using web frameworks and serialization crates. |
| HTTP Clients | [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}, [`isahc`][c~isahc~docs]↗{{hi:isahc}} | These crates provide HTTP client functionality for making requests to external services. |
| Routing | Covered by web frameworks. | Web frameworks provide routing mechanisms to map requests to handlers. |
| Middleware | Often provided by web frameworks or through crates like [`tower`][c~tower~docs]↗{{hi:tower}}. | Middleware allows you to add functionality to the request/response pipeline. |
| Templating | [`minijinja`][c~minijinja~docs]↗{{hi:minijinja}}, [`tera`][c~tera~docs]↗{{hi:tera}}, [`handlebars`][c~handlebars~docs]↗{{hi:handlebars}}, [`askama`][c~askama~docs]↗{{hi:askama}} | Templating engines are used to generate HTML dynamically. |
| Web Authentication & Authorization | [`actix-web-httpauth`][c~actix-web-httpauth~docs]↗{{hi:actix-web-httpauth}} (for Actix Web), [`tower-http`][c~tower-http~docs]↗{{hi:tower-http}} (generic middleware) | Authentication and authorization are often handled through middleware or dedicated crates. |
| WebSockets | [`tokio-tungstenite`][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}}, [`async-tungstenite`][c~async-tungstenite~docs]↗{{hi:async-tungstenite}} | These crates provide WebSocket support. |
| GraphQL | [`async-graphql`][c~async-graphql~docs]↗{{hi:async-graphql}}, [`juniper`][c~juniper~docs]↗{{hi:juniper}} | These crates enable building GraphQL APIs. |
| Static Site Generation (SSG) | [`perseus`][c~perseus~docs]↗{{hi:perseus}} (based on [`sycamore`](https://crates.io/crates/sycamore)↗{{hi:sycamore}}), [`zola`][zola~website]↗{{hi:zola}} | These tools generate static websites from templates and content. |

## Choosing the Right Crates

- Full-Stack Web App: [`axum`][c~axum~docs]↗{{hi:axum}}, [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}}, [`rocket`][c~rocket~docs]↗{{hi:rocket}}, [`warp`][c~warp~docs]↗{{hi:warp}}.
- Front-end Web App (WASM): [`yew`][c~yew~docs]↗{{hi:yew}}, [`seed`][c~seed~docs]↗{{hi:seed}}, [`leptos`][c~leptos~docs]↗{{hi:leptos}}.
- REST API: Web framework + [`serde`][c~serde~docs]↗{{hi:serde}}.
- GraphQL API: [`async-graphql`][c~async-graphql~docs]↗{{hi:async-graphql}}.
- Static Site Generation: [`perseus`][c~perseus~docs]↗{{hi:perseus}}, [`zola`][zola~website]↗{{hi:zola}}.

## Code Examples

### HTTP Types & Interfaces

{{#include http_types_and_interfaces.incl.md}}

### Manipulate Uniform Resource Locations (URLs)

{{#include url.incl.md}}

### Handle Media Types (MIME)

{{#include mime.incl.md}}

### Web Page Scraping

{{#include scraping.incl.md}}

### HTTP Clients

See [[web-programming_http-client | Web Programming: HTTP Client]].

### HTTP Servers

See [[web-programming_http-server | Web Programming: HTTP Server]].

### Web Authentication & Authorization

See [[authentication | Authentication]].

### GraphQL

See [[_graphql | GraphQL]].

### WebSockets

See [[web-programming_websocket | Web Programming: Websocket]].

## Related Topics

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| [[asynchronous | Asynchronous]] Programming (Essential for Web) | [`tokio`][c~tokio~docs]↗{{hi:tokio}}, [`async-std`][c~async-std~docs]↗{{hi:async-std}} | These are asynchronous runtimes that are fundamental for writing efficient and scalable web applications in Rust. |
| [[databases | Databases]] | [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}}, [`diesel`][c~diesel~docs]↗{{hi:diesel}}, [`mongodb`][c~mongodb~docs]↗{{hi:mongodb}} (drivers) | These crates provide database access for various database systems. |
| [[network-programming | Networking]] (Low-Level) | [`tokio::net`](https://docs.rs/tokio/latest/tokio/net)↗{{hi:tokio::net}}, [`std::net`](https://doc.rust-lang.org/std/net/index.html)↗{{hi:std::net}} | These modules provide low-level networking primitives. Often used by higher-level frameworks. |
| [[serde | Serialization/Deserialization]] (JSON, etc.) | [`serde`][c~serde~docs]↗{{hi:serde}}, [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}}, [`serde_yml`][c~serde_yml~docs]↗{{hi:serde_yml}} | [`serde`][c~serde~docs]↗{{hi:serde}} is a powerful framework for serializing and deserializing data, often used with JSON and other formats. |
| [[development-tools_testing | Testing]] | Built-in testing framework, [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} for integration testing | Rust has a built-in testing framework, and [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} can be used for integration testing of web services. |

## References

- [Are we Web yet?][are-we-web-yet?~website]↗.
- [Building a crawler in Rust: design and associated types][blog~building-a-crawler-in-rust]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[web-programming/index: organize; need full review; further cross link NOW](https://github.com/john-cd/rust_howto/issues/500)

Frameworks

- [Which Rust web framework to choose in 2022 (with code examples)](https://kerkour.com/rust-web-framework-2022)

QUIC

- [quiche: Savory implementation of the QUIC transport protocol and HTTP/3](https://github.com/cloudflare/quiche)
- [Quinn — Rust network library](https://lib.rs/crates/quinn)
- [msquic: Cross-platform, C implementation of the IETF QUIC protocol, exposed to C, C++, C# and Rust.](https://github.com/microsoft/msquic)

- [Mockito — Rust web dev library](https://lib.rs/crates/mockito)

HTTP

- [H2 — Rust web dev library](https://lib.rs/crates/h2)

OTHER

- [webbrowser — Rust web dev library](https://lib.rs/crates/webbrowser)  open URLs and local files in the web browsers available on a platform

- [webpki-root-certs — Rust crypto library](https://lib.rs/crates/webpki-root-certs)

- [Sycamore](https://sycamore.dev/)
- [sycamore: A library for creating reactive web apps in Rust and WebAssembly](https://github.com/sycamore-rs/sycamore)

</div>
