<!-- TODO Reviewed ??-??-2026. -->

# Alternate WebSocket Clients

[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:WebSocket}}

{{#include websocket.incl.md}}

See the published [WebSockets chapter](https://john-cd.com/rust_howto/categories/web-programming_websocket/websocket.html) for runtime selection, the Tokio loopback recipe, and connection lifecycle and security guidance.

## Examples

### Blocking client with tungstenite {#low-level}

{{#badges tungstenite}}

Use [tungstenite][c~tungstenite~docs]↗{{hi:tungstenite}} when blocking I/O fits the application or when you need its lower-level stream APIs. This example uses the [public Postman WebSocket echo endpoint][postman-websocket-echo].

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/websocket/tungstenite.rs:example}}
```

### Asynchronous client with async-tungstenite {#async-tungstenite}

{{#badges async-tungstenite}}

[async-tungstenite][c~async-tungstenite~docs]↗{{hi:async-tungstenite}} also provides a Tokio adapter; this example uses that adapter to show the crate's API.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/websocket/async_tungstenite.rs:example}}
```

## Related Topics {#related-topics .skip}

- Web Programming<!-- Cross-book topic: bk/drafts/categories/web-programming/index.md; not included in this preview. -->.
- Web Programming: HTTP Client<!-- Cross-book topic: bk/drafts/categories/web-programming_http-client/index.md; not included in this preview. -->.
- Web Programming: HTTP Server<!-- Cross-book topic: bk/drafts/categories/web-programming_http-server/index.md; not included in this preview. -->.
- Network Programming<!-- Cross-book topic: bk/drafts/categories/network-programming/index.md; not included in this preview. -->.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
