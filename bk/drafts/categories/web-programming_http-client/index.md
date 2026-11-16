<!-- TODO Reviewed ??-??-2026. -->

# Clients

[![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]

Choose an HTTP client based on the API and runtime your application needs.
Use [`ureq`][c~ureq~docs]↗ for synchronous requests, [`reqwest`][c~reqwest~docs]↗
for a higher-level client with blocking and asynchronous APIs, or
[`hyper`][c~hyper~docs]↗ when you need control over connections and HTTP
messages. Each is demonstrated in the recipes below.

The request recipes cover headers and query parameters. The API recipes add
JSON, pagination, and status handling. The reliability recipes show when to
limit requests, retry with backoff, and honor `Retry-After`. Use the file
transfer recipes for streams and byte ranges.

## HTTP Client Libraries

{{#include http_clients.incl.md}}

## Make HTTP Requests

{{#include requests.incl.md}}

## Call an API

{{#include apis.incl.md}}

## Rate Limit and Retry

{{#include rate_limiting_and_retries.incl.md}}

## Download and Upload

{{#include download.incl.md}}

## Related Topics

| Topic | Rust Crates (Examples) | Notes |
| --- | --- | --- |
| WebSockets | [`tokio-tungstenite`][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}}, [`async-tungstenite`][c~async-tungstenite~docs]↗{{hi:async-tungstenite}} | These crates provide WebSocket client functionality. |
| JSON Handling (Serialization/Deserialization) | [`serde`][c~serde~docs]↗{{hi:serde}}, [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} | [`serde`][c~serde~docs]↗{{hi:serde}} is a powerful framework for serializing and deserializing data, commonly used with JSON. |
| URL Parsing & Manipulation | [`url`][c~url~docs]↗{{hi:url}} | This crate provides tools for parsing and manipulating URLs. |
| Form Encoding | (Often handled by HTTP client crates) | Most HTTP client crates handle form encoding automatically. |
| Authentication | (Often handled by HTTP client crates or dedicated auth crates) | HTTP client crates often provide methods for basic authentication. More complex authentication schemes may require dedicated crates. |
| API Interaction (REST) | (Uses HTTP clients and serialization crates) | Interacting with REST APIs usually involves making HTTP requests using crates like [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} and handling JSON responses with [`serde`][c~serde~docs]↗{{hi:serde}}. |
| GraphQL Clients | [`graphql_client`][c~graphql_client~docs]↗{{hi:graphql_client}} | This crate helps with making GraphQL queries and mutations. |
| WebAssembly (WASM) Clients (Fetching Data) | (Often uses [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} or [`fetch`][c~fetch~docs]↗{{hi:fetch}} API via [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}}) | In WASM contexts, use either [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} (compiled to WASM) or interact directly with the browser's [`fetch`][c~fetch~docs]↗{{hi:fetch}} API using [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}}. |
| Browser Interaction (WASM) | [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} | [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} is essential for interacting with browser APIs from Rust/WASM, including making network requests. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
