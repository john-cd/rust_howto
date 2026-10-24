<!-- TODO Reviewed ??-??-2026 -->

# Server

{{#include server.incl.md}}

## A TCP Server's Lifecycle {#tcp-server-lifecycle}

A TCP server binds a [`TcpListener`][c~std::net::TcpListener~docs]↗{{hi:TcpListener}} to a local socket address and waits for clients. The operating system assigns an available port when the port is `0`; query `local_addr()` to discover the selected port. Binding to the loopback address accepts connections from this machine, while binding to a non-loopback interface can expose the service to other machines. Choose the address deliberately, and configure the host firewall and network access separately.

Calling `accept()` returns a `TcpStream` for one connection and the peer's socket address. TCP provides an ordered byte stream; it does not mark application messages or preserve the boundaries of client writes. A protocol must define its own framing, such as a fixed-size header, a length prefix, or a delimiter. The example below reads UTF-8 text until the client closes its sending side, accepts one connection, and then exits. It is a small demonstration, not a multi-client service.

The example uses blocking I/O from the standard library. That is often the clearest choice for a small tool or a server that handles one connection at a time. A server that must keep many connections active can use an asynchronous runtime and spawn a task for each accepted connection; the [asynchronous programming chapter][p~asynchronous] explains the runtime and I/O model. Whichever model you use, handle connection errors and limit work per client so one slow or untrusted peer cannot hold resources indefinitely.

## Listen on Unused Port TCP/IP {#listen-on-unused-port}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:TCP/IP}}

In this example, the port{{hi:Port}} is displayed on the console, and the program will listen until a request is made. [`std::net::SocketAddrV4`][c~std::net::SocketAddrV4~docs]↗{{hi:std::net::SocketAddrV4}} assigns a random port when setting port to 0.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/server/listen_unused.rs:example}}
```

## Perform Asynchronous I/O Operations on Storage Devices {#perform-asynchronous-io-operations-on-storage-devices}

`io_uring`{{hi:io_uring}} is a [Linux][p~linux] kernel system call interface for high-performance [asynchronous][p~asynchronous] I/O operations on storage devices. It works by creating two circular buffers, called "queue rings", for storage of submission and completion of I/O requests, respectively.

### High-performance Asynchronous I/O with `glommio` {#high-performance-asynchronous-io .skip}

[![glommio][c~glommio~docs~badge]][c~glommio~docs]{{hi:glommio}}
[![glommio~crates.io][c~glommio~crates.io~badge]][c~glommio~crates.io]
[![glommio~repo][c~glommio~repo~badge]][c~glommio~repo]
[![glommio~lib.rs][c~glommio~lib.rs~badge]][c~glommio~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}
[![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}
[![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}
[![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}}

Use [`glommio`][c~glommio~docs]↗{{hi:glommio}} for {{i:io_uring support}}. Still somewhat experimental but rapidly maturing.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/server/glommio.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[asynchronous | Asynchronous Programming]] explains async runtimes and I/O.
- [[web-programming | Web Programming]] is the starting point for HTTP, WebSocket, and related application protocols.
- [[web-programming_http-server | HTTP Servers]] covers Hyper and higher-level web frameworks. Use an HTTP implementation when you need HTTP parsing, routing, and response handling instead of defining those rules over a raw TCP stream.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[server: write; add cross-links: async, web, `http` server](https://github.com/john-cd/rust_howto/issues/425)
</div>
