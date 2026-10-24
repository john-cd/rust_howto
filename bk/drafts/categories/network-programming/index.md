<!-- TODO Reviewed ??-??-2026 -->
# Networking

[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

Network programming covers higher-level protocols{{hi:Network protocols}} such as HTTP{{hi:HTTP}} (Hypertext Transfer Protocol), SMTP{{hi:SMTP}} (Simple Mail Transfer Protocol), and SSH{{hi:SSH}}, as well as lower-level protocols such as TCP{{hi:TCP}} (Transmission Control Protocol) and UDP{{hi:UDP}} (User Datagram Protocol).

| Topic | Rust Crates |
|---|---|
| TCP and UDP sockets | Use [`std::net`][c~std::net~docs]↗{{hi:std::net}} for synchronous TCP/UDP communication. For nonblocking, event-driven I/O, [`mio`][c~mio~docs]↗{{hi:mio}} provides a lower-level interface. |
| Asynchronous networking | [`tokio`][c~tokio~docs]↗{{hi:tokio}} and [`smol`][c~smol~docs]↗{{hi:smol}} provide asynchronous runtimes and networking primitives. See [Asynchronous programming](../asynchronous/index.md) for runtime and task choices. |
| IP addresses and prefixes | [`std::net`][c~std::net~docs]↗{{hi:std::net}} provides IP address and socket address types. [`ipnet`][c~ipnet~docs]↗ adds IPv4 and IPv6 network types for working with CIDR prefixes and address ranges. |

For protocol-specific APIs, see [Email](../email/index.md), [HTTP clients](../web-programming_http-client/requests.md), and [networking command-line tools](../command-line-utilities/networking_cli.md). Email address syntax validation is separate from SMTP delivery: [`email_address`][c~email_address~crates.io] provides address parsing and validation, while [`fast_chemail`][c~fast_chemail~crates.io] is an older alternative whose repository has seen little recent activity. A syntactically valid address does not prove that a mailbox exists or can receive mail.

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
| Peer-to-peer networking | [Iroh][blog~iroh]↗ is a Rust networking project for peer-to-peer connections and data transfer. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
