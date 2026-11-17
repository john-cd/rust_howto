<!-- TODO Reviewed ??-??-2026. -->

# Server

{{#include server.incl.md}}

## A TCP Server's Lifecycle {#tcp-server-lifecycle}

A TCP server binds a [`TcpListener`][c~std::net::TcpListener~docs]↗{{hi:TcpListener}} to a local socket address and waits for clients. The operating system assigns an available port when the port is `0`; query `local_addr()` to discover the selected port. Binding to the loopback address accepts connections from this machine, while binding to a non-loopback interface can expose the service to other machines. Choose the address deliberately, and configure the host firewall and network access separately.

Calling `accept()` returns a `TcpStream` for one connection and the peer's socket address. TCP provides an ordered byte stream; it does not mark application messages or preserve the boundaries of client writes. A protocol must define its own framing, such as a fixed-size header, a length prefix, or a delimiter. The example below reads UTF-8 text until the client closes its sending side, accepts one connection, and then exits. It is a small demonstration, not a multi-client service.

The example uses blocking I/O from the standard library. That is often the clearest choice for a small tool or a server that handles one connection at a time. A server that must keep many connections active can use an asynchronous runtime and spawn a task for each accepted connection; [Asynchronous Programming](../asynchronous/index.md) explains the runtime and I/O model. Whichever model you use, handle connection errors and limit work per client so one slow or untrusted peer cannot hold resources indefinitely.

## Listen on an Available TCP Port {#listen-on-unused-port}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:TCP/IP}}

The example binds to the loopback address and port `0`, so the operating system selects an available port. It prints the chosen address, accepts one connection, and displays the received text. [`std::net::SocketAddrV4`][c~std::net::SocketAddrV4~docs]↗{{hi:std::net::SocketAddrV4}} represents the IPv4 socket address.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/server/listen_unused.rs:example}}
```

## Serve HTTP with Glommio {#glommio-http-server .skip}

{{#badges glommio asynchronous=Asynchronous concurrency=Concurrency filesystem=Filesystem network-programming="Network programming" os="Operating systems"}}

[`glommio`][c~glommio~docs]↗{{hi:glommio}} is a thread-per-core runtime that uses Linux `io_uring` for asynchronous I/O. This example binds to loopback, accepts connections on a local executor, and serves a fixed HTTP response. It requires a Linux kernel with `io_uring` support (5.8 or newer is recommended) and is not available on other operating systems.

The current dependency graph includes `owned-alloc`, affected by [RUSTSEC-2026-0291](https://rustsec.org/advisories/RUSTSEC-2026-0291.html), with no fixed upstream release. This educational sample must not be copied into production services until a compatible upstream dependency path is available.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/server/glommio.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Asynchronous Programming](../asynchronous/index.md) explains async runtimes and I/O.
- [Web Programming](../web-programming/index.md) covers HTTP, WebSocket, and related application protocols.
- [HTTP Servers](../web-programming_http-server/index.md) covers Hyper and higher-level web frameworks. Use an HTTP implementation when you need HTTP parsing, routing, and response handling instead of defining those rules over a raw TCP stream.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
