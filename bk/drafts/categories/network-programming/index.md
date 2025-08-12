# Networking

[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

Deal with higher-level network protocols{{hi:Network protocols}} such as FTP{{hi:FTP}} (File Transfer Protocol), HTTP{{hi:HTTP}} (Hypertext Transfer Protocol), or SSH{{hi:SSH}}, or lower-level network protocols such as TCP{{hi:TCP}} (Transmission Control Protocol) or UDP{{hi:UDP}} (User Datagram Protocol).

| Topic | Rust Crates |
|---|---|
| Basic networking, Network Sockets (Low-Level) | Use [`std::net`][c~std::net~docs]↗{{hi:std::net}} (in the standard library) for simple TCP/UDP communication, including TCP and UDP sockets. |
| High-performance, asynchronous networking | [`tokio`][c~tokio~docs]↗{{hi:tokio}} is a powerful and widely used asynchronous runtime, and essential for building high-performance network applications. It provides abstractions for working with sockets, streams, and other network primitives asynchronously.  Consider [`smol`][c~smol~docs]↗{{hi:smol}} as well. [`mio`][c~mio~docs]↗{{hi:mio}} is a lower-level I/O library used by [`tokio`][c~tokio~docs]↗{{hi:tokio}}. You'll rarely use [`mio`][c~mio~docs]↗{{hi:mio}} directly unless you have very specific performance needs. |

Many crates exist for specific protocols (e.g., SMTP, IMAP, SSH).

## Server

{{#include server.incl.md}}

## Reverse Proxies

{{#include reverse_proxy.incl.md}}

## Related Topics

| Topic | Rust Crates |
|---|---|
| HTTP client | Use [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}. |
| HTTP server / web framework | For building web servers, [`axum`][c~axum~docs]↗{{hi:axum}} or [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}} are popular options. [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}} is a powerful and ergonomic web framework built on top of [`hyper`][c~hyper~docs]↗{{hi:hyper}}. [`warp`][c~warp~docs]↗{{hi:warp}} is a lightweight and composable web framework. [`axum`][c~axum~docs]↗{{hi:axum}} is a newer web framework with a focus on type safety and developer experience. [`hyper`][c~hyper~docs]↗{{hi:hyper}} is a low-level HTTP library, often used to build custom HTTP clients or servers. |
| WebSockets | Use [`tokio-tungstenite`][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}}, a WebSocket library built on [`tokio`][c~tokio~docs]↗{{hi:tokio}}, for bidirectional communication between a client and server. [`tungstenite`][c~tungstenite~docs]↗{{hi:tungstenite}} is a lower-level WebSocket library. |
| Serialization/Deserialization | [`serde`][c~serde~docs]↗{{hi:serde}} is a widely used serialization framework, often used to serialize data before sending it over the network and deserialize it after receiving it. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; add cross-links; review; review `email_address`, `fast_chemail` NOW](https://github.com/john-cd/rust_howto/issues/944)

- [IpNet][c~ipnet~lib.rs]↗: Provides types and useful methods for working with IPv4 and IPv6 network addresses, commonly called IP prefixes.

## Peer2peer

- [Iroh (blog)](https://www.iroh.computer/blog/iroh-0-33-0-browsers-and-discovery-and-0-RTT-oh-my)

</div>
