<!-- TODO Reviewed ??-??-2026 -->
# HTTP Types & Interfaces

{{#include http_types_and_interfaces.incl.md}}

## `http` {#http}

[![http][c~http~docs~badge]][c~http~docs]{{hi:http}}
[![http~crates.io][c~http~crates.io~badge]][c~http~crates.io]
[![http~repo][c~http~repo~badge]][c~http~repo]
[![http~lib.rs][c~http~lib.rs~badge]][c~http~lib.rs]

The [`http`][c~http~docs]↗{{hi:http}} crate provides the shared vocabulary used by HTTP clients, servers, and middleware. A request carries a method, URI, version, headers, extensions, and a body; a response carries a status, version, headers, extensions, and a body. The body type is generic, so these messages can hold a byte buffer, a streaming body, or an application-specific value.

These types describe messages; they do not open sockets, send requests, or implement a server. Choose a client or server such as `reqwest`, `hyper`, or `axum` to perform network I/O.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/http_types_and_interfaces/http.rs:example}}
```

Use the crate's named constants for standard header names and `HeaderName` for application-defined names. Parse untrusted values into `HeaderValue` before adding them. Use `HeaderMap::append` when a header can occur more than once, and `get_all` to inspect every value in insertion order.

<div class="hidden">

Issue #1355 expands the sample from a basic request/response builder into header construction and interpretation. It covers typed `Accept`, `Authorization`, `Content-Type`, and `Content-Length` names, a custom name, text and numeric parsing, repeated `Set-Cookie` values, invalid-value rejection, and offline tests.

</div>

## `http-body` {#http-body}

[![http-body][c~http-body~docs~badge]][c~http-body~docs] [![http-body~crates.io][c~http-body~crates.io~badge]][c~http-body~crates.io] [![http-body~repo][c~http-body~repo~badge]][c~http-body~repo] [![http-body~lib.rs][c~http-body~lib.rs~badge]][c~http-body~lib.rs]{{hi:http-body}}{{hi:Http}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Trait representing an asynchronous, streaming, HTTP request or response body.

The `Body` trait yields frames asynchronously. Data frames carry byte buffers, and HTTP/2 bodies can also carry trailers. An implementation provides an error type and may report size hints, allowing network libraries to stream large payloads without first collecting them into one allocation.

## `http-body-util` {#http-body-util}

[![http-body-util][c~http-body-util~docs~badge]][c~http-body-util~docs] [![http-body-util~crates.io][c~http-body-util~crates.io~badge]][c~http-body-util~crates.io] [![http-body-util~repo][c~http-body-util~repo~badge]][c~http-body-util~repo] [![http-body-util~lib.rs][c~http-body-util~lib.rs~badge]][c~http-body-util~lib.rs]{{hi:http-body-util}}{{hi:Http}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

[`http-body-util`][c~http-body-util~docs]↗{{hi:http-body-util}} provides adapters for the `http_body::Body` trait. `Full` and `Empty` represent a single payload or no payload; `StreamBody` adapts a stream of frames. Its `BodyExt` extension trait includes `collect`, which is useful for small, known-size responses but buffers the full body in memory.

## Related Topics {#related-topics .skip}

- HTTP clients: [[web-programming_http-client | HTTP Client]].
- HTTP servers: [[web-programming_http-server | HTTP Server]].
- Middleware often uses the `tower::Service` interface to wrap request and response handling.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1338)
</div>
