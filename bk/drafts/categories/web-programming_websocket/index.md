# Websocket

[![cat-web-programming::websocket][cat-web-programming::websocket-badge]][cat-web-programming::websocket]{{hi:Websocket}}

This chapter covers communication over the WebSocket protocol.

## Key Considerations

The Rust WebSocket ecosystem is relatively stable, with [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}} being a very popular and well-maintained option for asynchronous use cases.

- If you're using a web framework ([`warp`][c-warp]⮳{{hi:warp}}, [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`axum`][c-axum]⮳{{hi:axum}}, etc.), check if it provides built-in WebSocket support, as this often simplifies integration.
- Asynchronous WebSockets are generally preferred over synchronous ones for web applications to avoid blocking the main thread.

## Choosing the Right Crate

The following table outlines common tasks for working with WebSockets in Rust, along with relevant crates.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| WebSocket Clients | [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}, [`async-tungstenite`][c-async_tungstenite]⮳{{hi:async-tungstenite}}, [`tungstenite`][c-tungstenite]⮳{{hi:tungstenite}} | Use [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}} when using the [`tokio`][c-tokio]⮳{{hi:tokio}} async runtime, [`async-tungstenite`][c-async_tungstenite]⮳{{hi:async-tungstenite}} for the [`async-std`][c-async_std]⮳{{hi:async-std}} runtime. [`tungstenite`][c-tungstenite]⮳{{hi:tungstenite}} is a lower-level crate. |
| WebSocket Servers | [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}, [`async-tungstenite`][c-async_tungstenite]⮳{{hi:async-tungstenite}}, [`warp`][c-warp]⮳{{hi:warp}} (with WebSocket support), [`actix-web`][c-actix_web]⮳{{hi:actix-web}} (with WebSocket support), [`axum`][c-axum]⮳{{hi:axum}} (with WebSocket support) | [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}} and [`async-tungstenite`][c-async_tungstenite]⮳{{hi:async-tungstenite}} can be used to build custom WebSocket servers. Web frameworks like [`warp`][c-warp]⮳{{hi:warp}}, [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, and [`axum`][c-axum]⮳{{hi:axum}} also provide built-in support for handling WebSockets. |
| Asynchronous WebSockets | [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}, [`async-tungstenite`][c-async_tungstenite]⮳{{hi:async-tungstenite}} | These crates are designed for asynchronous WebSocket communication, which is essential for efficient web applications. |
| Synchronous WebSockets | [`tungstenite`][c-tungstenite]⮳{{hi:tungstenite}} | [`tungstenite`][c-tungstenite]⮳{{hi:tungstenite}} provides a synchronous API for WebSockets. Less common for web applications but might be useful in other contexts. |
| Handshake & Upgrading | (Handled by the WebSocket crates) | The WebSocket crates handle the initial handshake and protocol upgrade process. |
| Message Handling (Text & Binary) | (Handled by the WebSocket crates) | WebSocket crates provide methods for sending and receiving both text and binary messages. |
| Error Handling | (Handled by the WebSocket crates) | WebSocket crates include mechanisms for handling errors that may occur during communication. |
| Connection Management | (Often handled by the application logic using the WebSocket crates) | Managing connections, including reconnecting and handling disconnections, is typically handled by the application logic using the WebSocket crate's API. |
| Integration with Web Frameworks | (Supported by frameworks like [`warp`][c-warp]⮳{{hi:warp}}, [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`axum`][c-axum]⮳{{hi:axum}}) | Many web frameworks provide easy integration with WebSockets, allowing you to handle WebSocket connections alongside regular HTTP requests. |
| TLS/SSL Support | (Often handled by the underlying networking libraries) | TLS/SSL encryption for WebSockets is usually handled by the underlying networking libraries used by the WebSocket crate. |

## Code Examples

{{#include websocket.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/979)

</div>
