# Cross-origin resource sharing

{{#include cors.incl.md}}

## Implement CORS {#cors}

[![tower][c-tower-badge]][c-tower] [![tower-crates.io][c-tower-crates.io-badge]][c-tower-crates.io] [![tower-github][c-tower-github-badge]][c-tower-github] [![tower-lib.rs][c-tower-lib.rs-badge]][c-tower-lib.rs]{{hi:tower}}{{hi:Async}}{{hi:Futures}}{{hi:Io}}{{hi:Non-blocking}}{{hi:Service}} [![tower-http][c-tower_http-badge]][c-tower_http] [![tower-http-crates.io][c-tower_http-crates.io-badge]][c-tower_http-crates.io] [![tower-http-github][c-tower_http-github-badge]][c-tower_http-github] [![tower-http-lib.rs][c-tower_http-lib.rs-badge]][c-tower_http-lib.rs]{{hi:tower-http}}{{hi:Async}}{{hi:Futures}}{{hi:Http}}{{hi:Io}}{{hi:Service}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/web_programming_http_server/tests/middleware/cors.rs:example}}
```

## See also

- [CORS (mozilla)][mozilla-cors]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[cors: write (P1)](https://github.com/john-cd/rust_howto/issues/510)
</div>
