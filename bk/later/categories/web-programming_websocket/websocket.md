# Websocket

{{#include websocket.incl.md}}

## Implement Websocket Using a low-level Library {#low-level}

[![tungstenite][c~tungstenite~docs~badge]][c~tungstenite~docs]{{hi:tungstenite}}
[![tungstenite~crates.io][c~tungstenite~crates.io~badge]][c~tungstenite~crates.io]
[![tungstenite~repo][c~tungstenite~repo~badge]][c~tungstenite~repo]
[![tungstenite~lib.rs][c~tungstenite~lib.rs~badge]][c~tungstenite~lib.rs]
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}
[![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:WebSocket}}
[`tungstenite`][c~tungstenite~docs]↗{{hi:tungstenite}} is a low-level crate that others build on.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/tungstenite.rs:example}}
```

## Implement Websocket While Using the `tokio` Async Runtime {#general-purpose}

[![tokio-tungstenite][c~tokio-tungstenite~docs~badge]][c~tokio-tungstenite~docs]{{hi:tokio-tungstenite}}
[![tokio-tungstenite~crates.io][c~tokio-tungstenite~crates.io~badge]][c~tokio-tungstenite~crates.io]
[![tokio-tungstenite~repo][c~tokio-tungstenite~repo~badge]][c~tokio-tungstenite~repo]
[![tokio-tungstenite~lib.rs][c~tokio-tungstenite~lib.rs~badge]][c~tokio-tungstenite~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}
[![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}
[![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:WebSocket}}

Use [`tokio-tungstenite`][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}} if you are using the [`tokio`][c~tokio~docs]↗{{hi:tokio}} async executor.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/tokio-tungstenite.rs:example}}
```

## Implement Websocket While Using the `async-std` Async Runtime {#async-std}

[![async-tungstenite][c~async-tungstenite~docs~badge]][c~async-tungstenite~docs] [![async-tungstenite~crates.io][c~async-tungstenite~crates.io~badge]][c~async-tungstenite~crates.io] [![async-tungstenite~repo][c~async-tungstenite~repo~badge]][c~async-tungstenite~repo] [![async-tungstenite~lib.rs][c~async-tungstenite~lib.rs~badge]][c~async-tungstenite~lib.rs]{{hi:async-tungstenite}}{{hi:Async-std}}{{hi:Io}}{{hi:Tokio}}{{hi:Web}}{{hi:Websocket}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:WebSocket}}

Async binding for Tungstenite.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/async_tungstenite.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[web-programming | Web Programming]].
- [[web-programming_http-client | Web Programming: HTTP Client]].
- [[web-programming_http-server | Web Programming: HTTP Server]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[websocket: write](https://github.com/john-cd/rust_howto/issues/520)
</div>
