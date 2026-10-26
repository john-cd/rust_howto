<!-- TODO Reviewed ??-??-2026 -->

# WebSockets

[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~web-programming::websocket][cat~web-programming::websocket~badge]][cat~web-programming::websocket]{{hi:WebSocket}}

{{#include websocket.incl.md}}

WebSockets begin with an HTTP handshake and then keep one connection open for messages in both directions. Either side can send the next message without waiting for a new request. Because a connection is long-lived, plan for timeouts, disconnections, and cleanup as part of the application protocol.

## Choose a crate that matches the runtime

| Need                                     | Crate                                                                  | Use it when                                                                                 |
| ---------------------------------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| Blocking client or custom I/O            | [tungstenite][c~tungstenite~docs]↗{{hi:tungstenite}}                   | Synchronous I/O fits the application, or you need to adapt the protocol to a custom stream. |
| Tokio client or server                   | [tokio-tungstenite][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}} | The application already uses Tokio and needs asynchronous socket I/O.                       |
| Another supported async runtime          | [async-tungstenite][c~async-tungstenite~docs]↗{{hi:async-tungstenite}} | The project needs an async WebSocket implementation with an adapter for its runtime.        |
| A WebSocket route in an HTTP application | Your web framework                                                     | The framework already owns the HTTP server and provides WebSocket upgrade support.          |

These are protocol libraries, not application protocols. Your application still defines message formats, authorization, reconnect behavior, and how clients handle server-initiated messages.

## Handle the connection lifecycle

After the handshake, read and write WebSocket messages until the peer closes the connection or an error occurs. Handle text, binary, ping, pong, and close messages deliberately; do not assume every frame contains the application message you expect. The two Tokio-based examples start one-connection echo servers on ephemeral loopback ports, so their tests need neither an external service nor network access. The blocking example uses [Postman's public WebSocket echo endpoint][postman-websocket-echo].

For a real client or server, decide how to detect an idle or dead peer, how to perform a graceful close, and whether reconnecting is safe for each operation. Use bounded outbound queues and apply backpressure when message production can outpace the network. Retry with a limit and backoff; replaying a non-idempotent command can repeat its effect.

## Protect WebSocket connections

- Use secure WebSocket URLs in production. Enable a TLS feature on the chosen crate and verify certificates. The public echo examples use native TLS; the loopback Tokio example uses plain WebSocket over local TCP.
- Authenticate during the handshake or another explicit protocol step, then authorize each message. A successful connection does not grant access to every operation.
- For browser-facing servers, validate the handshake Origin against an explicit allowlist to prevent cross-site WebSocket hijacking. Origin is not authentication and may be absent or supplied by non-browser clients.
- Treat every message as untrusted input. Validate its type and fields, cap message sizes and connection counts, and rate-limit expensive operations.
- Use ping/pong or an application heartbeat with an idle timeout to notice dead peers. Log connection and protocol events without recording secrets or full message contents.

See the [OWASP WebSocket Security Cheat Sheet][owasp-websocket-security] for a security checklist.

## Examples

### Blocking client with tungstenite {#low-level}

[![tungstenite][c~tungstenite~docs~badge]][c~tungstenite~docs] [![tungstenite~crates.io][c~tungstenite~crates.io~badge]][c~tungstenite~crates.io] [![tungstenite~repo][c~tungstenite~repo~badge]][c~tungstenite~repo] [![tungstenite~lib.rs][c~tungstenite~lib.rs~badge]][c~tungstenite~lib.rs]{{hi:tungstenite}}

Use [tungstenite][c~tungstenite~docs]↗{{hi:tungstenite}} when blocking I/O fits the application or when you need its lower-level stream APIs.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/websocket/tungstenite.rs:example}}
```

### Asynchronous echo client and server with Tokio {#general-purpose}

[![tokio-tungstenite][c~tokio-tungstenite~docs~badge]][c~tokio-tungstenite~docs] [![tokio-tungstenite~crates.io][c~tokio-tungstenite~crates.io~badge]][c~tokio-tungstenite~crates.io] [![tokio-tungstenite~repo][c~tokio-tungstenite~repo~badge]][c~tokio-tungstenite~repo] [![tokio-tungstenite~lib.rs][c~tokio-tungstenite~lib.rs~badge]][c~tokio-tungstenite~lib.rs]{{hi:tokio-tungstenite}}

Use [tokio-tungstenite][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}} when the application uses the [tokio][c~tokio~docs]↗{{hi:tokio}} runtime. The short example binds an ephemeral loopback listener, echoes one text message, and returns the reply to its client.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/websocket/tokio_tungstenite.rs:example}}
```

### Asynchronous client with async-tungstenite {#async-tungstenite}

[![async-tungstenite][c~async-tungstenite~docs~badge]][c~async-tungstenite~docs] [![async-tungstenite~crates.io][c~async-tungstenite~crates.io~badge]][c~async-tungstenite~crates.io] [![async-tungstenite~repo][c~async-tungstenite~repo~badge]][c~async-tungstenite~repo] [![async-tungstenite~lib.rs][c~async-tungstenite~lib.rs~badge]][c~async-tungstenite~lib.rs]{{hi:async-tungstenite}}

[async-tungstenite][c~async-tungstenite~docs]↗{{hi:async-tungstenite}} also provides a Tokio adapter; this example uses that adapter to show the crate's API.

```rust,editable
{{#include ../../../crates/cats/web_programming_websocket/examples/websocket/async_tungstenite.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[web-programming | Web Programming]].
- [[web-programming_http-client | Web Programming: HTTP Client]].
- [[web-programming_http-server | Web Programming: HTTP Server]].
- [[network-programming | Network Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [websocket: write](https://github.com/john-cd/rust_howto/issues/520)
  Reviewed upstream tokio-tungstenite client and echo-server examples; updated the Tokio recipe to use `Utf8Bytes` and a tested local client/server round trip.
</div>
