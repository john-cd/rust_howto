# Server

## Listen on unused port TCP/IP

[![std-badge]][std] [![cat-network-programming-badge]][cat-network-programming]

In this example, the port is displayed on the console, and the program will listen until a request is made.  `SocketAddrV4` assigns a random port when setting port to 0.

```rust,editable,no_run
{{#include ../../deps/examples/listen-unused.rs}}
```

{{#include ../refs/link-refs.md}}
