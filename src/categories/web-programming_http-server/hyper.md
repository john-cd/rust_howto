# Hyper

{{#include hyper.incl.md}}

## Implement an HTTP API with `hyper` {#hyper}

[![hyper][c-hyper-badge]][c-hyper]{{hi:hyper}}{{hi:HTTP server}}
[![hyper-crates.io][c-hyper-crates.io-badge]][c-hyper-crates.io]
[![hyper-github][c-hyper-github-badge]][c-hyper-github]
[![hyper-lib.rs][c-hyper-lib.rs-badge]][c-hyper-lib.rs]

`hyper` is a low-level HTTP implementation (both client and server). It implements HTTP/1 and HTTP/2. It works best with the `tokio` async runtime, but can support other runtimes.

```rust,editable
{{#include ../../../crates/ex/cats/web_programming_http_server/tests/hyper_server.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[hyper: write (P1)](https://github.com/john-cd/rust_howto/issues/515)

</div>
