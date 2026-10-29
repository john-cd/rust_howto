<!-- TODO Reviewed ??-??-2026 -->

# Websocket

[![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:Websocket}}

This chapter covers communication over the WebSocket protocol.

## Choosing the Right Crate

| Need                              | Crate                                                                                                                             | Notes                                                |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| Async client or server on `tokio` | [`tokio-tungstenite`][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}}                                                          | The default choice; builds on `tungstenite`.         |
| Async on other runtimes           | [`async-tungstenite`][c~async-tungstenite~docs]↗{{hi:async-tungstenite}}                                                          | Supports `async-std`, `smol`, and `tokio`.           |
| Synchronous, blocking I/O         | [`tungstenite`][c~tungstenite~docs]↗{{hi:tungstenite}}                                                                            | The protocol implementation the others wrap.         |
| High-performance async            | [`fastwebsockets`][c~fastwebsockets~docs]↗{{hi:fastwebsockets}}                                                                   | Minimal, fast; fewer conveniences.                   |
| Server inside a web framework     | [`axum`][c~axum~docs]↗{{hi:axum}} (`ws` module), [`actix-ws`][c~actix-ws~docs]↗{{hi:actix-ws}}, [`warp`][c~warp~docs]↗{{hi:warp}} | Handshake, upgrade, and routing are handled for you. |

Prefer asynchronous crates for servers so connections do not block threads. TLS is configured through the underlying networking crate.

## Code Examples

{{#include websocket.incl.md}}

## Related Topics

- Web Programming<!-- Cross-book topic: bk/drafts/categories/web-programming/index.md; not included in this preview. -->.
- Web Programming: HTTP Server<!-- Cross-book topic: bk/drafts/categories/web-programming_http-server/index.md; not included in this preview. -->.
- Network Programming<!-- Cross-book topic: bk/drafts/categories/network-programming/index.md; not included in this preview. -->.
- [WASM](../wasm/index.md).
- [Development Tools](../../other/written-in-rust/development_tools.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
