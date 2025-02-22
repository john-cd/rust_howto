# Networking

[![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}

Deal with higher-level network protocols{{hi:Network protocols}} such as FTP{{hi:FTP}}, HTTP{{hi:HTTP}}, or SSH{{hi:SSH}}, or lower-level network protocols such as TCP{{hi:TCP}} or UDP{{hi:UDP}}.

## Server

{{#include server.incl.md}}

## Reverse Proxies

{{#include reverse_proxy.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P1 review](https://github.com/john-cd/rust_howto/issues/944)

## Key Concepts

- TCP: Transmission Control Protocol (reliable, connection-oriented).
- UDP: User Datagram Protocol (unreliable, connectionless).
- Sockets: Endpoints for network communication.
- Ports: Numbers used to identify specific applications or services on a host.
- IP addresses: Numbers used to identify devices on a network.
- HTTP: Hypertext Transfer Protocol (used for web communication).
- WebSockets: A protocol for bidirectional communication between a client and server.

## Choosing Crates

- Basic networking: `std::net` (for simple TCP/UDP communication).
- High-performance, asynchronous networking: [`tokio`][c-tokio]⮳{{hi:tokio}}.
- HTTP client: [`reqwest`][c-reqwest]⮳{{hi:reqwest}}.
- HTTP server/web framework: [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`warp`][c-warp]⮳{{hi:warp}}, or [`axum`][c-axum]⮳{{hi:axum}}.
- WebSockets: [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}.

For most common network applications, [`tokio`][c-tokio]⮳{{hi:tokio}} is essential for handling asynchronous I/O efficiently. For HTTP clients, [`reqwest`][c-reqwest]⮳{{hi:reqwest}} is a good choice. For building web servers, [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`warp`][c-warp]⮳{{hi:warp}}, or [`axum`][c-axum]⮳{{hi:axum}} are popular options. For lower-level network programming or very specific needs, you might need to explore other crates or combine crates together.

## Network Sockets (Low-Level)

- `std::net`: (Standard library) Provides basic networking functionality, including TCP and UDP sockets.

## Asynchronous Networking

- [`tokio`][c-tokio]⮳{{hi:tokio}}: A powerful and widely used asynchronous runtime. Essential for building high-performance network applications. Provides abstractions for working with sockets, streams, and other network primitives asynchronously.
- `mio`: A lower-level I/O library used by [`tokio`][c-tokio]⮳{{hi:tokio}}. You'll rarely use [`mio`][c-mio]⮳{{hi:mio}} directly unless you have very specific performance needs.

Link to:

## HTTP Clients and Servers

- [`reqwest`][c-reqwest]⮳{{hi:reqwest}}: A popular and easy-to-use HTTP client.
- [`hyper`][c-hyper]⮳{{hi:hyper}}: A low-level HTTP library. Often used to build custom HTTP clients or servers.
- [`actix-web`][c-actix_web]⮳{{hi:actix-web}}: A powerful and ergonomic web framework built on top of [`hyper`][c-hyper]⮳{{hi:hyper}}.
- [`warp`][c-warp]⮳{{hi:warp}}: A lightweight and composable web framework.
- [`axum`][c-axum]⮳{{hi:axum}}: A newer web framework with a focus on type safety and developer experience.

## WebSockets

- [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}: A WebSocket library built on [`tokio`][c-tokio]⮳{{hi:tokio}}.
- [`tungstenite`][c-tungstenite]⮳{{hi:tungstenite}}: A lower-level WebSocket library.

## Network Protocols (Specific Protocols)

Many crates exist for specific protocols (e.g., SMTP, IMAP, SSH).

## Serialization/Deserialization (Often Needed for Network Communication)

- [`serde`][c-serde]⮳{{hi:serde}}: A widely used serialization framework. Often used to serialize data before sending it over the network and deserialize it after receiving it.

</div>
