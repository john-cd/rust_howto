# HTTP Types & Interfaces

{{#include http_types_and_interfaces.incl.md}}

## `http` {#http}

[![http][c~http~docs~badge]][c~http~docs]{{hi:http}}
[![http~crates.io][c~http~crates.io~badge]][c~http~crates.io]
[![http~github][c~http~github~badge]][c~http~github]
[![http~lib.rs][c~http~lib.rs~badge]][c~http~lib.rs]

The [`http`][c~http~docs]↗{{hi:http}} crate doesn't actually contain an HTTP implementation. Just types and interfaces to help interoperability.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/http_types_and_interfaces/http.rs:example}}
```

## `http-body` {#http-body}

[![http-body][c~http-body~docs~badge]][c~http-body~docs] [![http-body~crates.io][c~http-body~crates.io~badge]][c~http-body~crates.io] [![http-body~github][c~http-body~github~badge]][c~http-body~github] [![http-body~lib.rs][c~http-body~lib.rs~badge]][c~http-body~lib.rs]{{hi:http-body}}{{hi:Http}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Trait representing an asynchronous, streaming, HTTP request or response body.

## `http-body-util` {#http-body-util}

[![http-body-util][c~http-body-util~docs~badge]][c~http-body-util~docs] [![http-body-util~crates.io][c~http-body-util~crates.io~badge]][c~http-body-util~crates.io] [![http-body-util~github][c~http-body-util~github~badge]][c~http-body-util~github] [![http-body-util~lib.rs][c~http-body-util~lib.rs~badge]][c~http-body-util~lib.rs]{{hi:http-body-util}}{{hi:Http}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Combinators and adapters for HTTP request or response bodies.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write NOW](https://github.com/john-cd/rust_howto/issues/1338)
</div>
