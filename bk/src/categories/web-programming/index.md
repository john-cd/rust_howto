# Web Programming

[![cat-web-programming][cat-web-programming-badge]][cat-web-programming]

Create applications for the Web.

The following table outlines common web development tasks and relevant Rust crates. The Rust web ecosystem offers a variety of options, from low-level networking to high-level frameworks.

| Topic | Rust Crates | Notes |
|---|---|---|
| Web Frameworks (Full-Stack) | [`axum`][c-axum]⮳{{hi:axum}}, [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`rocket`][c-rocket]⮳{{hi:rocket}}, [`warp`][c-warp]⮳{{hi:warp}} | These frameworks provide tools for building web applications, including routing, middleware, templating, and more. [`actix-web`][c-actix_web]⮳{{hi:actix-web}} is known for its performance. [`axum`][c-axum]⮳{{hi:axum}} is built on top of [`tower`][c-tower]⮳{{hi:tower}} and [`hyper`][c-hyper]⮳{{hi:hyper}}. [`rocket`][c-rocket]⮳{{hi:rocket}} uses a more declarative approach. [`warp`][c-warp]⮳{{hi:warp}} is a more lightweight framework. |
| Front-end Frameworks (WASM) | [`yew`][c-yew]⮳{{hi:yew}}, [`leptos`][c-leptos]⮳{{hi:leptos}}, [`seed`][c-seed]⮳{{hi:seed}}, [`dominator`][c-dominator]⮳{{hi:dominator}} | These frameworks enable building interactive web UIs with Rust compiled to WebAssembly (WASM). They provide component-based architectures and other tools for structuring front-end applications. |
| HTTP Servers | Covered by web frameworks. | Web frameworks typically handle HTTP server functionality. |
| REST API Design | Often uses web frameworks and serialization crates like [`serde`][c-serde]⮳{{hi:serde}}. | REST APIs are typically built using web frameworks and serialization crates. |
| HTTP Clients | [`reqwest`][c-reqwest]⮳{{hi:reqwest}}, [`isahc`][c-isahc]⮳{{hi:isahc}} | These crates provide HTTP client functionality for making requests to external services. |
| Routing | Covered by web frameworks. | Web frameworks provide routing mechanisms to map requests to handlers. |
| Middleware | Often provided by web frameworks or through crates like [`tower`][c-tower]⮳{{hi:tower}}. | Middleware allows you to add functionality to the request/response pipeline. |
| Templating | [`minijinja`][c-minijinja]⮳{{hi:minijinja}}, [`tera`][c-tera]⮳{{hi:tera}}, [`handlebars`][c-handlebars]⮳{{hi:handlebars}}, [`askama`][c-askama]⮳{{hi:askama}} | Templating engines are used to generate HTML dynamically. |
| Web Authentication & Authorization | [`actix-web-httpauth`][c-actix_web_httpauth]⮳{{hi:actix-web-httpauth}} (for Actix Web), [`tower-http`][c-tower_http]⮳{{hi:tower-http}} (generic middleware) | Authentication and authorization are often handled through middleware or dedicated crates. |
| WebSockets | [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}, [`async-tungstenite`][c-async_tungstenite]⮳{{hi:async-tungstenite}} | These crates provide WebSocket support. |
| GraphQL | [`async-graphql`][c-async_graphql]⮳{{hi:async-graphql}}, [`juniper`][c-juniper]⮳{{hi:juniper}} | These crates enable building GraphQL APIs. |
| Static Site Generation (SSG) | [`perseus`][c-perseus]⮳{{hi:perseus}} (based on `sycamore`), [`zola`][c-zola]⮳{{hi:zola}} | These tools generate static websites from templates and content. |

## Choosing the Right Crates

- Full-Stack Web App: [`axum`][c-axum]⮳{{hi:axum}}, [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`rocket`][c-rocket]⮳{{hi:rocket}}, [`warp`][c-warp]⮳{{hi:warp}}.
- Front-end Web App (WASM): [`yew`][c-yew]⮳{{hi:yew}}, [`seed`][c-seed]⮳{{hi:seed}}, [`leptos`][c-leptos]⮳{{hi:leptos}}.
- REST API: Web framework + [`serde`][c-serde]⮳{{hi:serde}}.
- GraphQL API: [`async-graphql`][c-async_graphql]⮳{{hi:async-graphql}}.
- Static Site Generation: [`perseus`][c-perseus]⮳{{hi:perseus}}, [`zola`][c-zola-website]⮳{{hi:zola}}.

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
| [[asynchronous | Asynchronous]] Programming (Essential for Web) | [`tokio`][c-tokio]⮳{{hi:tokio}}, [`async-std`][c-async_std]⮳{{hi:async-std}} | These are asynchronous runtimes that are fundamental for writing efficient and scalable web applications in Rust. |
| [[databases | Databases]] | [`sqlx`][c-sqlx]⮳{{hi:sqlx}}, [`diesel`][c-diesel]⮳{{hi:diesel}}, [`mongodb`][c-mongodb]⮳{{hi:mongodb}} (drivers) | These crates provide database access for various database systems. |
| [[network-programming | Networking]] (Low-Level) | `tokio::net`, `std::net` | These modules provide low-level networking primitives. Often used by higher-level frameworks. |
| [[serde | Serialization/Deserialization]] (JSON, etc.) | [`serde`][c-serde]⮳{{hi:serde}}, [`serde_json`][c-serde_json]⮳{{hi:serde_json}}, [`serde_yml`][c-serde_yml]⮳{{hi:serde_yml}} | [`serde`][c-serde]⮳{{hi:serde}} is a powerful framework for serializing and deserializing data, often used with JSON and other formats. |
| [[development-tools_testing | Testing]] | Built-in testing framework, [`reqwest`][c-reqwest]⮳{{hi:reqwest}} for integration testing | Rust has a built-in testing framework, and [`reqwest`][c-reqwest]⮳{{hi:reqwest}} can be used for integration testing of web services. |

## References

- [Are we Web yet?][are-we-web-yet?-website]⮳.
- [Building a crawler in Rust: design and associated types][blog-building-a-crawler-in-rust]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[web-programming/index: organize; need full review; further cross link NOW](https://github.com/john-cd/rust_howto/issues/500)
</div>
