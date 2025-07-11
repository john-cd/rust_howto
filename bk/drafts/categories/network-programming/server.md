# Server

{{#include server.incl.md}}

## Listen on Unused Port TCP/IP {#listen-on-unused-port}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:TCP/IP}}

In this example, the port{{hi:Port}} is displayed on the console, and the program will listen until a request is made. [`std::net::SocketAddrV4`][c~std::net::SocketAddrV4~docs]{{hi:std::net::SocketAddrV4}}⮳ assigns a random port when setting port to 0.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/server/listen_unused.rs:example}}
```

## Perform Asynchronous I/O Operations on Storage Devices {#perform-asynchronous-io-operations-on-storage-devices}

`io_uring`{{hi:io_uring}} is a [Linux][p~linux] kernel system call interface for high-performance [asynchronous][p~asynchronous] I/O operations on storage devices. It works by creating two circular buffers, called "queue rings", for storage of submission and completion of I/O requests, respectively.

### High-performance Asynchronous I/O with `glommio` {#skip1}

[![glommio][c~glommio~docs~badge]][c~glommio~docs]{{hi:glommio}}
[![glommio~crates.io][c~glommio~crates.io~badge]][c~glommio~crates.io]
[![glommio~github][c~glommio~github~badge]][c~glommio~github]
[![glommio~lib.rs][c~glommio~lib.rs~badge]][c~glommio~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}
[![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}
[![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}
[![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}}

Use [`glommio`][c~glommio~docs]⮳{{hi:glommio}} if you need {{i:io_uring support}}. Still somewhat experimental but rapidly maturing.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/server/glommio.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[server: write; add cross-links: async, web, http server](https://github.com/john-cd/rust_howto/issues/425)
</div>
