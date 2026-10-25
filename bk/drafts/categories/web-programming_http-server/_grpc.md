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

The integration test binds a Tokio listener to an available local port, sends a generated client request, checks the greeting, and stops the server with a one-shot shutdown signal. It does not depend on a fixed port or a timing delay.

## Serve gRPC to Browsers with gRPC-Web {#grpc-web}

[![tonic-web][c~tonic-web~docs~badge]][c~tonic-web~docs]{{hi:tonic-web}} [![tonic][c~tonic~docs~badge]][c~tonic~docs]{{hi:tonic}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

Browsers cannot make the HTTP/2 trailers-based calls that gRPC needs. [gRPC-Web][grpc-web~repo]↗ is a variant of the protocol that browsers can speak with `fetch`. [`tonic-web`][c~tonic-web~docs]↗{{hi:tonic-web}} provides `GrpcWebLayer`, which translates gRPC-Web requests into gRPC for the same `tonic` service, so one port serves native and browser clients.

Two details matter. First, `Server::builder().accept_http1(true)` is required, because gRPC-Web may arrive over HTTP/1.1. Second, a page served from another origin needs CORS: add a [`tower-http`][c~tower-http~docs]↗{{hi:tower-http}} `CorsLayer` that allows the request headers and exposes `grpc-status` and `grpc-message` (see [[cors | Cross-origin Resource Sharing]]). The example mirrors any origin to stay short; allow only your own origins in production.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/grpc/grpc_web.rs:example}}
```

The test sends a hand-framed gRPC-Web request (a five-byte length prefix followed by the protobuf message) to the service and checks the reply, so no browser is needed. In the browser, the [`tonic-web-wasm-client`][c~tonic-web-wasm-client~docs]↗{{hi:tonic-web-wasm-client}} crate provides the matching client; see [[leptos | Leptos]].

**gRPC, gRPC-Web, or something else?**

- Use plain gRPC between servers: it supports all four streaming modes.
- Use gRPC-Web for browser clients. It supports unary and server-streaming calls, but not client or bidirectional streaming; if you need those, put a proxy such as Envoy in front of the service instead.
- If the API is small and public, a JSON API described with OpenAPI may be simpler for outside developers; see [[openapi | OpenAPI and JSON Schema]].

## Generate Code Without a System `protoc` {#protoc}

[![tonic-prost-build][c~tonic-prost-build~docs~badge]][c~tonic-prost-build~docs]{{hi:tonic-prost-build}} [![protox][c~protox~docs~badge]][c~protox~docs]{{hi:protox}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[`tonic-prost-build`][c~tonic-prost-build~docs]↗{{hi:tonic-prost-build}} normally runs the `protoc` compiler from a build script, which fails on machines that do not have it. There are three ways to avoid a system install:

- **A pure-Rust compiler.** [`protox`][c~protox~docs]↗{{hi:protox}} parses `.proto` files in Rust. This book's build script uses it: `protox::compile(["proto/helloworld.proto"], ["proto"])` returns a file descriptor set, and `tonic_prost_build::compile_fds(descriptors)` generates the Rust code.
- **A prebuilt `protoc`.** [`protoc-bin-vendored`][c~protoc-bin-vendored~docs]↗{{hi:protoc-bin-vendored}} ships `protoc` binaries for common platforms. Point `PROTOC` at `protoc_bin_vendored::protoc_bin_path()` before compiling. It is fast to build, but it downloads binaries with the crate.
- **A `protoc` built from source.** [`protobuf-src`][c~protobuf-src~docs]↗{{hi:protobuf-src}} compiles `protoc` with your C++ toolchain: slow, but no prebuilt binaries are involved.

Whichever you choose, honour a `PROTOC` environment variable set by the user, and tell Cargo when to rerun the script with `cargo:rerun-if-changed=proto/helloworld.proto` (and `cargo:rerun-if-env-changed=PROTOC`). This book's `build.rs` also compiles `proto/echo.proto` with the vendored variant, so the code below is built and tested with the book. `grpc/echo.rs` includes the generated code and checks that the message, client, and server types exist:

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/build.rs:vendored_protoc}}
```

## Related Topics {#related-topics .skip}

- [[asynchronous | Asynchronous Programming]] for Tokio and async service methods.
- [[network-programming | Network Programming]] for socket addresses and transport basics.
- [[web-programming_http-server | HTTP Servers]] for HTTP-based web services and middleware.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[gRPC-Web and codegen written for issue #1945](https://github.com/john-cd/rust_howto/issues/1945)

  [grpc: write](https://github.com/john-cd/rust_howto/issues/514)
</div>
