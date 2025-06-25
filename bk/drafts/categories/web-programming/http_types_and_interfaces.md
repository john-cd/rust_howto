# HTTP Types & Interfaces

{{#include http_types_and_interfaces.incl.md}}

## `http` {#http}

[![http][c-http-badge]][c-http]{{hi:http}}
[![http-crates.io][c-http-crates.io-badge]][c-http-crates.io]
[![http-github][c-http-github-badge]][c-http-github]
[![http-lib.rs][c-http-lib.rs-badge]][c-http-lib.rs]

The [`http`][c-http]⮳{{hi:http}} crate doesn't actually contain an HTTP implementation. Just types and interfaces to help interoperability.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/http_types_and_interfaces/http.rs:example}}
```

## `http-body` {#http-body}

[![http-body][c-http_body-badge]][c-http_body] [![http-body-crates.io][c-http_body-crates.io-badge]][c-http_body-crates.io] [![http-body-github][c-http_body-github-badge]][c-http_body-github] [![http-body-lib.rs][c-http_body-lib.rs-badge]][c-http_body-lib.rs]{{hi:http-body}}{{hi:Http}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

Trait representing an asynchronous, streaming, HTTP request or response body.

## `http-body-util` {#http-body-util}

[![http-body-util][c-http_body_util-badge]][c-http_body_util] [![http-body-util-crates.io][c-http_body_util-crates.io-badge]][c-http_body_util-crates.io] [![http-body-util-github][c-http_body_util-github-badge]][c-http_body_util-github] [![http-body-util-lib.rs][c-http_body_util-lib.rs-badge]][c-http_body_util-lib.rs]{{hi:http-body-util}}{{hi:Http}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

Combinators and adapters for HTTP request or response bodies.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write NOW](https://github.com/john-cd/rust_howto/issues/1338)
</div>
