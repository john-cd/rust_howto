<!-- TODO Reviewed ??-??-2026 -->

# Web Programming

[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]

Web applications combine HTTP, asynchronous I/O, serialization, and user-interface code. Use this index to find the guide for the layer you are building.

## Services, APIs, and Network I/O

| Goal | Start here |
|---|---|
| Build an HTTP server or REST API | [[web-programming_http-server | HTTP Servers]]. The guide covers Axum, Actix Web, Hyper, middleware, GraphQL, and gRPC. |
| Send requests to an HTTP service | [[web-programming_http-client | HTTP Clients]]. The guide compares `reqwest`, `ureq`, and Hyper, and includes request and download examples. |
| Implement a GraphQL endpoint | [[_graphql | GraphQL]]. |
| Implement a gRPC service | [[_grpc | gRPC]]. |
| Use TCP or UDP without HTTP | [[network-programming | Network Programming]]. |
| Add WebSocket communication | [[web-programming_websocket | Web Programming: WebSocket]], [[asynchronous | Asynchronous Programming]] and [[network-programming | Network Programming]]; see [`tokio-tungstenite`][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}} and [`async-tungstenite`][c~async-tungstenite~docs]↗{{hi:async-tungstenite}}. |
| Add authentication or authorization | [[authentication | Authentication]]. |

## Crates by Task

| Task | Crates |
|---|---|
| Full-stack web frameworks (routing, middleware, templating) | [`axum`][c~axum~docs]↗{{hi:axum}} (built on [`tower`][c~tower~docs]↗{{hi:tower}} and [`hyper`][c~hyper~docs]↗{{hi:hyper}}), [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}} (performance), [`rocket`][c~rocket~docs]↗{{hi:rocket}} (declarative), [`warp`][c~warp~docs]↗{{hi:warp}} (lightweight) |
| HTTP clients | [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}, [`isahc`][c~isahc~docs]↗{{hi:isahc}} |
| Templating | [`minijinja`][c~minijinja~docs]↗{{hi:minijinja}}, [`tera`][c~tera~docs]↗{{hi:tera}}, [`handlebars`][c~handlebars~docs]↗{{hi:handlebars}}, [`askama`][c~askama~docs]↗{{hi:askama}} |
| Authentication middleware | [`actix-web-httpauth`][c~actix-web-httpauth~docs]↗{{hi:actix-web-httpauth}} (Actix Web), [`tower-http`][c~tower-http~docs]↗{{hi:tower-http}} (generic) |
| GraphQL | [`async-graphql`][c~async-graphql~docs]↗{{hi:async-graphql}}, [`juniper`][c~juniper~docs]↗{{hi:juniper}} |
| Static sites and full-stack WASM | [`perseus`][c~perseus~docs]↗{{hi:perseus}} (based on [`sycamore`][c~sycamore~crates.io]↗{{hi:sycamore}}), [`zola`][c~zola~docs]↗{{hi:zola}} |
| Database drivers | [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}}, [`diesel`][c~diesel~docs]↗{{hi:diesel}}, [`mongodb`][c~mongodb~docs]↗{{hi:mongodb}} |

## Browser Interfaces and Generated Pages

For browser interfaces compiled to WebAssembly, explore [`yew`][c~yew~docs]↗{{hi:yew}}, [`leptos`][c~leptos~docs]↗{{hi:leptos}}, [`seed`][c~seed~docs]↗{{hi:seed}}, and [`dominator`][c~dominator~docs]↗{{hi:dominator}}. For HTML templates, see [[template-engine | Template Engines]]. The HTTP server guide also covers [[web-programming_http-server/static_website_generators | Static Site Generators]], including Zola.

## Code Examples

### HTTP Types and Interfaces

{{#include http_types_and_interfaces.incl.md}}

### Parse and Build URLs

{{#include url.incl.md}}

### Work with Media Types (MIME)

{{#include mime.incl.md}}

### Extract Links and Scrape Web Pages

{{#include scraping.incl.md}}

## Related Topics

- [[asynchronous | Asynchronous Programming]] for async runtimes and task coordination.
- [[database | Databases]] for database drivers, connection pools, and query tools.
- [[serde | Serialization and Deserialization]] for JSON and other data formats.
- [[development-tools_testing | Testing]] for unit, integration, and end-to-end checks.

## References

- [Are we Web yet?][are-we-web-yet?~website]↗.
- [Building a crawler in Rust][blog~building-a-crawler-in-rust]↗: design and associated types.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[web-programming/index: organize; need full review; further cross link](https://github.com/john-cd/rust_howto/issues/500)

Frameworks:

- [Which Rust web framework to choose in 2022 (with code examples)][blog~kerkour-rust-web-framework-2022]↗.

QUIC:

- [quiche][c~quiche~repo]↗: Savory implementation of the QUIC transport protocol and HTTP/3.
- [Quinn - Rust network library][c~quinn~lib.rs]↗.
- [msquic][c~msquic~repo]↗: Cross-platform, C implementation of the IETF QUIC protocol, exposed to C, C++, C# and Rust..

- [Mockito - Rust web dev library][c~mockito~lib.rs]↗.

HTTP:

- [H2 - Rust web dev library][c~h2~lib.rs]↗.

OTHER:

- [webbrowser - Rust web dev library][c~webbrowser~lib.rs]↗ open URLs and local files in the web browsers available on a platform.

- [webpki-root-certs - Rust crypto library][c~webpki-root-certs~lib.rs]↗.

- [Sycamore][sycamore~website]↗.
- [sycamore][c~sycamore~repo]↗: A library for creating reactive web apps in Rust and WebAssembly.

</div>
