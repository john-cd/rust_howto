# Web Programming

[![cat-web-programming][cat-web-programming-badge]][cat-web-programming]

Create applications for the Web.

## Manipulate uniform resource locations (URLs)

{{#include url.incl.md}}

## Handle media types (MIME)

{{#include mime.incl.md}}

## Scrape Web Pages

{{#include scraping.incl.md}}

## See also

[Are we Web yet?][are-we-web-yet?-website]⮳

[Building a crawler in Rust: design and associated types][blog-building-a-crawler-in-rust]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[web-programming/index: cover http crate (P1)](https://github.com/john-cd/rust_howto/issues/500)
[web-programming/index: organize (P1)](https://github.com/john-cd/rust_howto/issues/499)
[web-programming_http-client/index: fix (P1)](https://github.com/john-cd/rust_howto/issues/505)

## HTTP types & interfaces

[![http][c-http-badge]][c-http]{{hi:http}}
[![http-crates.io][c-http-crates.io-badge]][c-http-crates.io]
[![http-github][c-http-github-badge]][c-http-github]
[![http-lib.rs][c-http-lib.rs-badge]][c-http-lib.rs]

The [`http`][c-http]⮳{{hi:http}} crate doesn't actually contain an HTTP implementation. Just types and interfaces to help interoperability.

---

This table outlines common web development tasks and relevant Rust crates. The Rust web ecosystem offers a variety of options, from low-level networking to high-level frameworks.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Web Frameworks (Full-Stack) | [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`axum`][c-axum]⮳{{hi:axum}}, [`rocket`][c-rocket]⮳{{hi:rocket}}, [`warp`][c-warp]⮳{{hi:warp}} | These frameworks provide tools for building web applications, including routing, middleware, templating, and more. [`actix-web`][c-actix_web]⮳{{hi:actix-web}} is known for its performance. [`axum`][c-axum]⮳{{hi:axum}} is built on top of [`tower`][c-tower]⮳{{hi:tower}} and [`hyper`][c-hyper]⮳{{hi:hyper}}. [`rocket`][c-rocket]⮳{{hi:rocket}} uses a more declarative approach. [`warp`][c-warp]⮳{{hi:warp}} is a more lightweight framework. |
| Front-end Frameworks (WASM) | [`yew`][c-yew]⮳{{hi:yew}}, [`seed`][c-seed]⮳{{hi:seed}}, [`leptos`][c-leptos]⮳{{hi:leptos}}, [`dominator`][c-dominator]⮳{{hi:dominator}} | These frameworks enable building interactive web UIs with Rust compiled to WebAssembly (WASM). They provide component-based architectures and other tools for structuring front-end applications. |
| Asynchronous Programming (Essential for Web) | [`tokio`][c-tokio]⮳{{hi:tokio}}, [`async-std`][c-async_std]⮳{{hi:async-std}} | These are asynchronous runtimes that are fundamental for writing efficient and scalable web applications in Rust. |
| Networking (Low-Level) | `tokio::net`, `std::net` | These modules provide low-level networking primitives. Often used by higher-level frameworks. |
| HTTP Clients | [`reqwest`][c-reqwest]⮳{{hi:reqwest}}, [`isahc`][c-isahc]⮳{{hi:isahc}} | These crates provide HTTP client functionality for making requests to external services. |
| HTTP Servers | (Covered by web frameworks) | Web frameworks typically handle HTTP server functionality. |
| Routing | (Covered by web frameworks) | Web frameworks provide routing mechanisms to map requests to handlers. |
| Middleware | (Often provided by web frameworks or through crates like [`tower`][c-tower]⮳{{hi:tower}}) | Middleware allows you to add functionality to the request/response pipeline. |
| Templating | [`minijinja`][c-minijinja]⮳{{hi:minijinja}}, [`tera`][c-tera]⮳{{hi:tera}}, [`handlebars`][c-handlebars]⮳{{hi:handlebars}}, [`askama`][c-askama]⮳{{hi:askama}} | Templating engines are used to generate HTML dynamically. |
| Serialization/Deserialization (JSON, etc.) | [`serde`][c-serde]⮳{{hi:serde}}, [`serde_json`][c-serde_json]⮳{{hi:serde_json}}, [`serde_yaml`][c-serde_yaml]⮳{{hi:serde_yaml}} | [`serde`][c-serde]⮳{{hi:serde}} is a powerful framework for serializing and deserializing data, often used with JSON and other formats. |
| Databases | [`sqlx`][c-sqlx]⮳{{hi:sqlx}}, [`diesel`][c-diesel]⮳{{hi:diesel}}, [`mongodb`][c-mongodb]⮳{{hi:mongodb}} (drivers) | These crates provide database access for various database systems. |
| Authentication & Authorization | [`actix-web-httpauth`][c-actix_web_httpauth]⮳{{hi:actix-web-httpauth}} (for Actix Web), [`tower-http`][c-tower_http]⮳{{hi:tower-http}} (generic middleware) | Authentication and authorization are often handled through middleware or dedicated crates. |
| WebSockets | [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}, [`async-tungstenite`][c-async_tungstenite]⮳{{hi:async-tungstenite}} | These crates provide WebSocket support. |
| GraphQL | [`juniper`][c-juniper]⮳{{hi:juniper}}, [`async-graphql`][c-async_graphql]⮳{{hi:async-graphql}} | These crates enable building GraphQL APIs. |
| REST API Design | (Often uses web frameworks and serialization crates) | REST APIs are typically built using web frameworks and serialization crates. |
| Static Site Generation (SSG) | [`perseus`][c-perseus]⮳{{hi:perseus}}, [`zola`][c-zola]⮳{{hi:zola}} | These tools generate static websites from templates and content. |
| Testing | (Built-in testing framework, [`reqwest`][c-reqwest]⮳{{hi:reqwest}} for integration testing) | Rust has a built-in testing framework, and [`reqwest`][c-reqwest]⮳{{hi:reqwest}} can be used for integration testing of web services. |

## Key Considerations

- Full-Stack vs. Front-end: Decide whether you need a full-stack framework or just a front-end framework for WASM.
- Asynchronous Programming: Asynchronous programming is essential for web development in Rust.
- Performance: Rust's performance is a major advantage for web development.
- Community & Ecosystem: The Rust web ecosystem is growing rapidly, so explore different options and choose the crates that best suit your needs and coding style.

## Choosing the Right Crates

- Full-Stack Web App: [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`axum`][c-axum]⮳{{hi:axum}}, [`rocket`][c-rocket]⮳{{hi:rocket}}, [`warp`][c-warp]⮳{{hi:warp}}
- Front-end Web App (WASM): [`yew`][c-yew]⮳{{hi:yew}}, [`seed`][c-seed]⮳{{hi:seed}}, [`leptos`][c-leptos]⮳{{hi:leptos}}
- REST API: Web framework + [`serde`][c-serde]⮳{{hi:serde}}
- GraphQL API: [`juniper`][c-juniper]⮳{{hi:juniper}}, [`async-graphql`][c-async_graphql]⮳{{hi:async-graphql}}
- Static Site Generation: [`perseus`][c-perseus]⮳{{hi:perseus}}, [`zola`][c-zola]⮳{{hi:zola}}

</div>
