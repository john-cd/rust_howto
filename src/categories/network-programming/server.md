# Server

{{#include server.incl.md}}

## Listen on unused port TCP/IP

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}{{hi:TCP/IP}}

In this example, the port{{hi:Port}} is displayed on the console, and the program will listen until a request is made. [`std::net::SocketAddrV4`][c-std::net::SocketAddrV4]{{hi:std::net::SocketAddrV4}}⮳ assigns a random port when setting port to 0.

```rust
{{#include ../../../deps/tests/cats/network_programming/listen_unused.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
