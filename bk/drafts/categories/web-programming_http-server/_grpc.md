<!-- TODO Reviewed ??-??-2026 -->
# gRPC

{{#include grpc.incl.md}}

## Implement gRPC {#tonic}

[![tonic][c~tonic~docs~badge]][c~tonic~docs]{{hi:tonic}}
[![tonic~crates.io][c~tonic~crates.io~badge]][c~tonic~crates.io]
[![tonic~repo][c~tonic~repo~badge]][c~tonic~repo]
[![tonic~lib.rs][c~tonic~lib.rs~badge]][c~tonic~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}
[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

gRPC defines remote methods and their request and response messages in a service contract. This example uses Protocol Buffers for that contract and [`tonic`][c~tonic~docs]↗{{hi:tonic}} to serve the RPCs over HTTP/2 with [`tokio`][c~tokio~docs]↗{{hi:tokio}}.

The repository's `proto/helloworld.proto` declares `Greeter.SayHello(HelloRequest) returns (HelloReply)`. The build script uses `protox` and `tonic-prost-build` to generate Rust client and server types from that contract; `tonic::include_proto!("helloworld")` includes the generated module. This avoids a separate `protoc` executable.

The package currently pairs Tonic and its Prost integration crates at 0.14.6 with Prost 0.14.4. A standalone project using the same setup can declare:

```toml
[dependencies]
tonic = "0.14.6"
tonic-prost = "0.14.6"
prost = "0.14.4"

[build-dependencies]
protox = "0.10.0"
tonic-prost-build = "0.14.6"
```

`MyGreeter` implements the generated `Greeter` trait. Each RPC receives a `Request<T>` and returns either a `Response<T>` or a `Status`; `Server::builder().add_service(...)` registers the implementation, and `.serve(addr).await` runs the server. The example binds to the IPv6 loopback address on port 50051 for local use.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/grpc/tonic.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[asynchronous | Asynchronous Programming]] for Tokio and async service methods.
- [[network-programming | Network Programming]] for socket addresses and transport basics.
- [[web-programming_http-server | HTTP Servers]] for HTTP-based web services and middleware.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[grpc: write](https://github.com/john-cd/rust_howto/issues/514)
</div>
