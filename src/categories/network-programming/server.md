# Server

{{#include server.incl.md}}

## Listen on unused port TCP/IP

[![std][std-badge]][std]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

In this example, the port is displayed on the console, and the program will listen until a request is made. `SocketAddrV4` assigns a random port when setting port to 0.

```rust,editable,no_run
{{#include ../../../deps/tests/listen-unused.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
