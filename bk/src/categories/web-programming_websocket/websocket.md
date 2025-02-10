# Websocket

{{#include websocket.incl.md}}

## Implement Websocket using a low-level library {#low-level}

[![tungstenite][c-tungstenite-badge]][c-tungstenite]{{hi:tungstenite}}
[![tungstenite-crates.io][c-tungstenite-crates.io-badge]][c-tungstenite-crates.io]
[![tungstenite-github][c-tungstenite-github-badge]][c-tungstenite-github]
[![tungstenite-lib.rs][c-tungstenite-lib.rs-badge]][c-tungstenite-lib.rs]
[![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}
[![cat-web-programming::websocket][cat-web-programming::websocket-badge]][cat-web-programming::websocket]{{hi:WebSocket}}

Low-level crate that others build on.

```rust,editable
{{#include ../../../crates/ex/cats/web_programming_websocket/tests/tungstenite.rs:example}}
```

## Implement Websocket while using the `tokio` async runtime {#general-purpose}

[![tokio-tungstenite][c-tokio_tungstenite-badge]][c-tokio_tungstenite]{{hi:tokio-tungstenite}}
[![tokio-tungstenite-crates.io][c-tokio_tungstenite-crates.io-badge]][c-tokio_tungstenite-crates.io]
[![tokio-tungstenite-github][c-tokio_tungstenite-github-badge]][c-tokio_tungstenite-github]
[![tokio-tungstenite-lib.rs][c-tokio_tungstenite-lib.rs-badge]][c-tokio_tungstenite-lib.rs]
[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}
[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}
[![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}
[![cat-web-programming::websocket][cat-web-programming::websocket-badge]][cat-web-programming::websocket]{{hi:WebSocket}}

Use `tokio-tungstenite` if you are using the `tokio` async executor.

```rust,editable
{{#include ../../../crates/ex/cats/web_programming_websocket/tests/tokio_tungstenite.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[websocket: write (P2)](https://github.com/john-cd/rust_howto/issues/520)

[websocket: cover others e.g. async_std_tungstenite (P2)](https://github.com/john-cd/rust_howto/issues/521)

```rust,editable
{{#include ../../../crates/ex/cats/web_programming_websocket/tests/async_tungstenite.rs:example}}
```

</div>
