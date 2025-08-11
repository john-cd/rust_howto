# gRPC

{{#include grpc.incl.md}}

## Implement gRPC {#tonic}

[![tonic][c~tonic~docs~badge]][c~tonic~docs]{{hi:tonic}}
[![tonic~crates.io][c~tonic~crates.io~badge]][c~tonic~crates.io]
[![tonic~github][c~tonic~github~badge]][c~tonic~github]
[![tonic~lib.rs][c~tonic~lib.rs~badge]][c~tonic~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}
[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

[`tonic`][c~tonic~docs]↗{{hi:tonic}} implements gRPC over HTTP/2 with full support for asynchronous code. It works with [`tokio`][c~tokio~docs]↗{{hi:tokio}}.

[`gRPC`][grpc~website]↗{{hi:gRPC}} is an open-source high [performance][p~performance] Remote Procedure Call (RPC) framework that can run in any environment. It can efficiently connect services in and across data centers with pluggable support for load balancing, [tracing][p~tracing], health checking and [authentication][p~authentication]. It is also applicable in the "last mile" of distributed computing to connect devices, mobile applications and browsers to backend services.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/grpc/tonic.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[grpc: write](https://github.com/john-cd/rust_howto/issues/514)
</div>
