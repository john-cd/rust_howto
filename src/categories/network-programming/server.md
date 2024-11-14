# Server

{{#include server.incl.md}}

## Listen on unused port TCP/IP {#listen-on-unused-port}

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}{{hi:TCP/IP}}

In this example, the port{{hi:Port}} is displayed on the console, and the program will listen until a request is made. [`std::net::SocketAddrV4`][c-std::net::SocketAddrV4]{{hi:std::net::SocketAddrV4}}⮳ assigns a random port when setting port to 0.

```rust
{{#include ../../../deps/tests/cats/network_programming/listen_unused.rs:example}}
```

## Perform asynchronous I/O operations on storage devices {#perform-asynchronous-io-operations-on-storage-devices}

`io_uring` is a Linux kernel system call interface for high-performance asynchronous I/O operations on storage devices. It works by creating two circular buffers, called "queue rings", for storage of submission and completion of I/O requests, respectively.

### glommio {#glommio}

[![glommio][c-glommio-badge]][c-glommio]{{hi:glommio}}
[![glommio-crates.io][c-glommio-crates.io-badge]][c-glommio-crates.io]
[![glommio-github][c-glommio-github-badge]][c-glommio-github]
[![glommio-lib.rs][c-glommio-lib.rs-badge]][c-glommio-lib.rs]
[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}
[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}
[![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}
[![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}
[![cat-os][cat-os-badge]][cat-os]{{hi:Operating systems}}

Use `glommio` if you need {{i:io_uring support}}. Still somewhat experimental but rapidly maturing.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
