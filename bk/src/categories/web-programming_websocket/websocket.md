<!-- TODO Reviewed ??-??-2026. -->

# WebSockets

[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:WebSocket}}

{{#include websocket.incl.md}}

WebSockets begin with an HTTP handshake and then keep one connection open for messages in both directions. Either side can send the next message without waiting for a new request. Because a connection is long-lived, plan for timeouts, disconnections, and cleanup as part of the application protocol.

## Choose a crate that matches the runtime

| Need | Crate | Use it when |
| ---------------------------------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| Blocking client or custom I/O | [tungstenite][c~tungstenite~docs]↗{{hi:tungstenite}} | Synchronous I/O fits the application, or you need to adapt the protocol to a custom stream. |
| Tokio client or server | [tokio-tungstenite][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}} | The application already uses Tokio and needs asynchronous socket I/O. |
| Another supported async runtime | [async-tungstenite][c~async-tungstenite~docs]↗{{hi:async-tungstenite}} | The project needs an async WebSocket implementation with an adapter for its runtime. |
| A WebSocket route in an HTTP application | Your web framework | The framework already owns the HTTP server and provides WebSocket upgrade support. |

These are protocol libraries, not application protocols. Your application still defines message formats, authorization, reconnect behavior, and how clients handle server-initiated messages.

## Handle the connection lifecycle

After the handshake, read and write WebSocket messages until the peer closes the connection or an error occurs. Handle text, binary, ping, pong, and close messages deliberately; do not assume every frame contains the application message you expect. The Tokio example uses an ephemeral loopback port and runs without an external service.

For a real client or server, decide how to detect an idle or dead peer, how to perform a graceful close, and whether reconnecting is safe for each operation. Use bounded outbound queues and apply backpressure when message production can outpace the network. Retry with a limit and backoff; replaying a non-idempotent command can repeat its effect.

## Protect WebSocket connections

- Use secure WebSocket URLs in production. Enable a TLS feature on the chosen crate and verify certificates. The loopback recipe uses plain WebSocket over local TCP.
- Authenticate during the handshake or another explicit protocol step, then authorize each message. A successful connection does not grant access to every operation.
- For browser-facing servers, validate the handshake Origin against an explicit allowlist to prevent cross-site WebSocket hijacking. Origin is not authentication and may be absent or supplied by non-browser clients.
- Treat every message as untrusted input. Validate its type and fields, cap message sizes and connection counts, and rate-limit expensive operations.
- Use ping/pong or an application heartbeat with an idle timeout to notice dead peers. Log connection and protocol events without recording secrets or full message contents.

See the [OWASP WebSocket Security Cheat Sheet][owasp-websocket-security] for a security checklist.

## Examples

### Asynchronous echo client and server with Tokio {#general-purpose}

{{#badges tokio-tungstenite}}

Use [tokio-tungstenite][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}} when the application uses the [tokio][c~tokio~docs]↗{{hi:tokio}} runtime. The short example binds an ephemeral loopback listener and echoes one text message.

From `bk/crates`, run:

```console
cargo run --locked -p web_programming_websocket --example websocket
```

The client prints `Received: Hello WebSocket` after the server echoes its message.

```rust,ignore,noplayground
{{#include ../../../crates/cats/web_programming_websocket/examples/websocket/tokio_tungstenite.rs:example}}
```

### Blocking echo with Tungstenite {#low-level}

Use [`tungstenite`][c~tungstenite~docs] when synchronous I/O fits your application or you need control over the underlying stream. The example wraps an already connected TCP stream, bounds blocking reads and writes, and exchanges close frames. The echo stays on loopback.

From `bk/crates`, run:

```console
cargo run --locked -p web_programming_websocket --example websocket_blocking
```

```rust,ignore,noplayground
{{#include ../../../crates/cats/web_programming_websocket/examples/websocket/tungstenite.rs:example}}
```

### Asynchronous echo with async-tungstenite {#async-tungstenite}

[`async-tungstenite`][c~async-tungstenite~docs] offers adapters for several runtimes. This example uses the Tokio adapter. It runs the client and server together under one deadline and checks the text reply before closing the connection.

From `bk/crates`, run:

```console
cargo run --locked -p web_programming_websocket --example websocket_async_adapter
```

```rust,ignore,noplayground
{{#include ../../../crates/cats/web_programming_websocket/examples/websocket/async_tungstenite.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Web Programming](../web-programming/index.md) for HTTP fundamentals.
- [HTTP Servers](../web-programming_http-server/index.md) for building applications that handle upgrades.
- [Network Programming](../network-programming/index.md) for working with TCP.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
