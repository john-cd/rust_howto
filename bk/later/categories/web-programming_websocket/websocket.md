# Websocket

{{#include websocket.incl.md}}

## Implement Websocket Using a low-level Library {#low-level}

[![tungstenite][c~tungstenite~docs~badge]][c~tungstenite~docs]{{hi:tungstenite}}
[![tungstenite~crates.io][c~tungstenite~crates.io~badge]][c~tungstenite~crates.io]
[![tungstenite~github][c~tungstenite~github~badge]][c~tungstenite~github]
[![tungstenite~lib.rs][c~tungstenite~lib.rs~badge]][c~tungstenite~lib.rs]
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}
[![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:WebSocket}}
[`tungstenite`][c~tungstenite~docs]⮳{{hi:tungstenite}} is a low-level crate that others build on.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/tungstenite.rs:example}}
```

## Implement Websocket While Using the `tokio` Async Runtime {#general-purpose}

[![tokio-tungstenite][c~tokio_tungstenite~docs~badge]][c~tokio_tungstenite~docs]{{hi:tokio-tungstenite}}
[![tokio-tungstenite~crates.io][c~tokio_tungstenite~crates.io~badge]][c~tokio_tungstenite~crates.io]
[![tokio-tungstenite~github][c~tokio_tungstenite~github~badge]][c~tokio_tungstenite~github]
[![tokio-tungstenite~lib.rs][c~tokio_tungstenite~lib.rs~badge]][c~tokio_tungstenite~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}
[![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}
[![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:WebSocket}}

Use [`tokio-tungstenite`][c~tokio_tungstenite~docs]⮳{{hi:tokio-tungstenite}} if you are using the [`tokio`][c~tokio~docs]⮳{{hi:tokio}} async executor.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/tokio_tungstenite.rs:example}}
```

## Implement Websocket While Using the `async-std` Async Runtime {#async-std}

[![async-tungstenite][c~async_tungstenite~docs~badge]][c~async_tungstenite~docs] [![async-tungstenite~crates.io][c~async_tungstenite~crates.io~badge]][c~async_tungstenite~crates.io] [![async-tungstenite~github][c~async_tungstenite~github~badge]][c~async_tungstenite~github] [![async-tungstenite~lib.rs][c~async_tungstenite~lib.rs~badge]][c~async_tungstenite~lib.rs]{{hi:async-tungstenite}}{{hi:Async-std}}{{hi:Io}}{{hi:Tokio}}{{hi:Web}}{{hi:Websocket}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:WebSocket}}

Async binding for Tungstenite.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/async_tungstenite.rs:example}}
```

## Related Topics {#skip}

- [[web-programming | Web Programming]].
- [[web-programming_http-client | Web Programming: HTTP Client]].
- [[web-programming_http-server | Web Programming: HTTP Server]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[websocket: write](https://github.com/john-cd/rust_howto/issues/520)
</div>
