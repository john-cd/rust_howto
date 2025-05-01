# Clients

[![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]

Make HTTP network requests.

The following table outlines common tasks for building web clients in Rust and relevant crates. Web clients can range from simple HTTP request libraries to more advanced tools for interacting with APIs or building web applications that fetch data.

- Simple HTTP Requests: [`ureq`][c-ureq]⮳{{hi:ureq}} is often recommended for its simplicity, especially for basic use cases.
- General-Purpose HTTP Client: [`reqwest`][c-reqwest]⮳{{hi:reqwest}} is highly versatile and supports a wide range of features.
- Performance-Focused HTTP Client: [`isahc`][c-isahc]⮳{{hi:isahc}} is often considered to be performant.
- REST API Interaction: [`reqwest`][c-reqwest]⮳{{hi:reqwest}} + [`serde`][c-serde]⮳{{hi:serde}}.
- When working with WASM, you'll either use [`reqwest`][c-reqwest]⮳{{hi:reqwest}} compiled to WASM or interact with the browser's 'fetch' API via [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}}.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| HTTP Clients (Making Requests) | [`reqwest`][c-reqwest]⮳{{hi:reqwest}}, [`isahc`][c-isahc]⮳{{hi:isahc}}, [`ureq`][c-ureq]⮳{{hi:ureq}} | [`reqwest`][c-reqwest]⮳{{hi:reqwest}} is a popular and versatile HTTP client. [`isahc`][c-isahc]⮳{{hi:isahc}} is another option with a focus on performance. [`ureq`][c-ureq]⮳{{hi:ureq}} is a lightweight and easy-to-use HTTP client. |

Most HTTP client crates support asynchronous requests using `async` and [`await`][c-await]⮳{{hi:await}}. Asynchronous requests are generally preferred for web clients to avoid blocking the main thread.

## HTTP Client Libraries

{{#include http_clients.incl.md}}

## Make HTTP Requests

{{#include requests.incl.md}}

## Call an API

{{#include apis.incl.md}}

## Download and Upload

{{#include download.incl.md}}

## See Also

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| WebSockets | [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}, [`async-tungstenite`][c-async_tungstenite]⮳{{hi:async-tungstenite}} | These crates provide WebSocket client functionality. |
| JSON Handling (Serialization/Deserialization) | [`serde`][c-serde]⮳{{hi:serde}}, [`serde_json`][c-serde_json]⮳{{hi:serde_json}} | [`serde`][c-serde]⮳{{hi:serde}} is a powerful framework for serializing and deserializing data, commonly used with JSON. |
| URL Parsing & Manipulation | [`url`][c-url]⮳{{hi:url}} | This crate provides tools for parsing and manipulating URLs. |
| Form Encoding | (Often handled by HTTP client crates) | Most HTTP client crates handle form encoding automatically. |
| Authentication | (Often handled by HTTP client crates or dedicated auth crates) | HTTP client crates often provide methods for basic authentication. More complex authentication schemes may require dedicated crates. |
| API Interaction (REST) | (Uses HTTP clients and serialization crates) | Interacting with REST APIs usually involves making HTTP requests using crates like [`reqwest`][c-reqwest]⮳{{hi:reqwest}} and handling JSON responses with [`serde`][c-serde]⮳{{hi:serde}}. |
| GraphQL Clients | [`graphql_client`][c-graphql_client]⮳{{hi:graphql_client}} | This crate helps with making GraphQL queries and mutations. |
| WebAssembly (WASM) Clients (Fetching Data) | (Often uses [`reqwest`][c-reqwest]⮳{{hi:reqwest}} or [`fetch`][c-fetch]⮳{{hi:fetch}} API via [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}}) | In WASM contexts, you can either use [`reqwest`][c-reqwest]⮳{{hi:reqwest}} (compiled to WASM) or interact directly with the browser's [`fetch`][c-fetch]⮳{{hi:fetch}} API using [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}}. |
| Browser Interaction (WASM) | [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} | [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} is essential for interacting with browser APIs from Rust/WASM, including making network requests. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[web-programming_http-client/index: fix, sync table and contents](https://github.com/john-cd/rust_howto/issues/505)
</div>
