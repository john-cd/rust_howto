# Hyper

{{#include hyper.incl.md}}

## `hyper` {#hyper}

[![hyper][c-hyper-badge]][c-hyper]{{hi:hyper}}{{hi:HTTP server}}
[![hyper-crates.io][c-hyper-crates.io-badge]][c-hyper-crates.io]
[![hyper-github][c-hyper-github-badge]][c-hyper-github]
[![hyper-lib.rs][c-hyper-lib.rs-badge]][c-hyper-lib.rs]

`hyper` is a low-level HTTP implementation (both client and server). Implements HTTP/1, and HTTP/2. Works best with the tokio async runtime, but can support other runtimes.

```rust,editable
{{#include ../../../deps/tests/categories/web_programming_http_server/hyper_server.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 write
</div>
