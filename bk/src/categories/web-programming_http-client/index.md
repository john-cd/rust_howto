# Clients

[![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]

Make HTTP network requests.

## HTTP client libraries

{{#include http_clients.incl.md}}

## Make HTTP requests

{{#include requests.incl.md}}

## Call an API

{{#include apis.incl.md}}

## Download and upload

{{#include download.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[web-programming_http-client/index: fix (P1)](https://github.com/john-cd/rust_howto/issues/505)

This table outlines common tasks for building web clients in Rust and relevant crates. Web clients in Rust can range from simple HTTP request libraries to more advanced tools for interacting with APIs or building web applications that fetch data.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| HTTP Clients (Making Requests) | `reqwest`, `isahc`, `ureq` | `reqwest` is a popular and versatile HTTP client. `isahc` is another option with a focus on performance. `ureq` is a lightweight and easy-to-use HTTP client. |
| Asynchronous HTTP Clients | (Often provided by the crates above with async features) | Most modern HTTP client crates support asynchronous requests using `async` and `await`. |

## See also

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| WebSockets | `tokio-tungstenite`, `async-tungstenite` | These crates provide WebSocket client functionality. |
| JSON Handling (Serialization/Deserialization) | `serde`, `serde_json` | `serde` is a powerful framework for serializing and deserializing data, commonly used with JSON. |
| URL Parsing & Manipulation | `url` | This crate provides tools for parsing and manipulating URLs. |
| Form Encoding | (Often handled by HTTP client crates) | Most HTTP client crates handle form encoding automatically. |
| Authentication | (Often handled by HTTP client crates or dedicated auth crates) | HTTP client crates often provide methods for basic authentication. More complex authentication schemes may require dedicated crates. |
| API Interaction (REST) | (Uses HTTP clients and serialization crates) | Interacting with REST APIs usually involves making HTTP requests using crates like `reqwest` and handling JSON responses with `serde`. |
| GraphQL Clients | `graphql_client` | This crate helps with making GraphQL queries and mutations. |
| WebAssembly (WASM) Clients (Fetching Data) | (Often uses `reqwest` or `fetch` API via `wasm-bindgen`) | In WASM contexts, you can either use `reqwest` (compiled to WASM) or interact directly with the browser's `fetch` API using `wasm-bindgen`. |
| Browser Interaction (WASM) | `wasm-bindgen` | `wasm-bindgen` is essential for interacting with browser APIs from Rust/WASM, including making network requests. |

## Key Considerations

- Synchronous vs. Asynchronous: Choose an HTTP client that supports the appropriate request style (synchronous or asynchronous). Asynchronous requests are generally preferred for web clients to avoid blocking the main thread.
- Ease of Use: `ureq` is often recommended for its simplicity, especially for basic use cases.
- Flexibility: `reqwest` is highly versatile and supports a wide range of features.
- Performance: `isahc` is often considered to be performant.
- WASM Context: In WASM, you'll either use `reqwest` compiled to WASM or interact with the browser's `fetch` API.

## Choosing the Right Crate

- Simple HTTP Requests: `ureq`
- General-Purpose HTTP Client: `reqwest`
- Performance-Focused HTTP Client: `isahc`
- WebSockets: `tokio-tungstenite`, `async-tungstenite`
- REST API Interaction: `reqwest` + `serde`
- GraphQL Client: `graphql_client`
- WASM Client (Fetching Data): `reqwest` (WASM) or `wasm-bindgen` + `fetch` API

</div>
