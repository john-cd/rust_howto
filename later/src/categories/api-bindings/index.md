<!-- TODO Reviewed ??-??-2026. -->

# API Bindings

[![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}}

API bindings are idiomatic Rust wrappers for specific APIs. They let you work with Rust types, ownership, and errors when calling a native library, an operating-system service, or a remote HTTP API. Browse the recipes below by the capability you need. For raw, non-idiomatic, or unsafe interfaces, see [External FFI Bindings][api-bindings~external-ffi]. For generating and maintaining bindings, see [FFI tooling][api-bindings~ffi-tools].

## Service APIs

| API | Rust entry point | Start here |
| --- | --- | --- |
| AWS and S3-compatible object stores | `aws-sdk-s3` | [Rust on AWS][api-bindings~aws] covers typed requests, shared SDK configuration, and S3-compatible endpoints. |
| Kubernetes | `kube` | [Kubernetes Operators][api-bindings~staged-kubernetes] uses typed resources to reconcile a Deployment. |

To call an HTTP API directly or write a small typed client of your own, start with [Call an API][api-bindings~http]. Its `reqwest` recipes cover JSON responses, pagination, HTTP errors, and rate limits. Service-specific clients such as `aws-sdk-s3` package the service's operations and types.

## Native Libraries, Operating Systems, and Language Runtimes

| Crate | Book coverage |
| --- | --- |
| `flate2` | [Compression and Decompression][api-bindings~compression] includes stream and gzip file round trips. |
| `zstd` | [Stream Zstandard Data][api-bindings~zstd-stream] composes an encoder and decoder with Rust I/O and explains why the writer must finish the frame. |
| `rusqlite` | [SQLite][api-bindings~sqlite] shows how to open an embedded database, bind parameters, and read rows. |
| `opentelemetry` | [Distributed Telemetry][api-bindings~telemetry] records and exports spans and propagates trace context across HTTP. |
| `jni` | [Java][api-bindings~java] builds a Rust native library that a JVM loads and calls. |
| `pyo3` | [Python][api-bindings~python] embeds Python, exposes Rust types to the embedded interpreter, and builds an importable extension. |
| `objc2` and `objc2-foundation` | [Objective-C][api-bindings~objc] uses typed Objective-C bindings. The [macOS API example][api-bindings~macos] reads the operating-system version through Foundation. |
| `socket2` | [TCP Servers][api-bindings~tcp-server] explains setting socket options before binding and handing the socket to `std::net::TcpListener`. The code examples on that page use other APIs. |
| `openssl` | [TLS][api-bindings~tls] discusses OpenSSL as a `native-tls` backend. [Hyper][api-bindings~hyper] lists `hyper-openssl` as an adapter. Direct `openssl` use has no recipe. |
| `libloading` | [Plugin Architecture][api-bindings~plugins] discusses native symbol lookup and library lifetimes. It has no dynamic-loading example. Loading a library and trusting a symbol's type are unsafe operations. See [External FFI Bindings][api-bindings~external-ffi] for the raw boundary. |
| `zstd-safe` and `zstd-sys` | [Compression and Decompression][api-bindings~compression] mentions the `zstd-sys` backend. [`zstd-safe`][c~zstd-safe~docs] is the lower-level safe wrapper and [`zstd-sys`][c~zstd-sys~docs] exposes raw bindings. The book example uses `zstd`. |
| `num_threads` | The lockfile includes it as a dependency. [`num_threads`][c~num_threads~docs] queries the current process's thread count. There is no book recipe for it. |

<details>
<summary>Try the staged <code>flate2</code> example</summary>

This short example writes and reads gzip bytes through Rust's `Write` and `Read` traits.

```rust
{{#include ../../../crates/cats/api_bindings/examples/api_bindings/gzip_round_trip.rs:example}}
```

</details>

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
