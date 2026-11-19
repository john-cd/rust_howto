<!-- TODO Reviewed ??-??-2026. -->

# API Bindings

[![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}}

An API binding gives Rust code an idiomatic interface to a capability implemented elsewhere. The capability may be a native library in the same process, an operating-system service, or a remote HTTP service. Remote clients use a network protocol; they do not cross a native ABI boundary. For binding generation and maintenance workflows, see [FFI Tooling](../development-tools_ffi/index.md). For raw, unsafe, or non-idiomatic declarations, see [External FFI Bindings](../external-ffi-bindings/external_ffi_bindings.md).

## Choose an Integration Layer

| Situation | Start with | Check before adopting |
| --- | --- | --- |
| A remote HTTP service | A service client such as [`reqwest`][c~reqwest~docs], [`isahc`][c~isahc~docs], or `surf` | Runtime requirements, TLS defaults, retries, timeouts, and how status and transport errors are exposed. |
| A remote database | Its protocol client, such as [`tokio-postgres`][c~tokio-postgres~docs] or [`mongodb`][c~mongodb~docs] | Server-version support, pooling, transaction behavior, and error handling. |
| An embedded database | A local wrapper such as [`rusqlite`][c~rusqlite~docs] | Whether it bundles or links a native library, and how connections and files are managed. |
| A native library with a maintained Rust wrapper | The safe wrapper rather than its `-sys` crate | Platform support, SDK requirements, enabled features, and foreign-resource ownership. |
| An API without a suitable wrapper | A raw or generated binding behind a small safe module | ABI compatibility, pointer validity, ownership, thread safety, and link-time dependencies. |

Prefer the narrowest maintained interface that covers the required API. Keep credentials and network policy at the application boundary. Keep `unsafe` calls and foreign ownership rules inside a wrapper module.

## Find a Guide by Topic

This page is a map to the book's focused guides and working examples. Many of the crates below wrap a protocol or library; the guide explains how to select and use the wrapper for a particular task.

| Integration area | Book guide | Common crates and scope |
| --- | --- | --- |
| Cloud service clients | [Cloud](../../other/cloud/index.md), [AWS](../../other/cloud/aws.md), [Kubernetes operators](../../other/cloud/kubernetes.md#kubernetes) | AWS service clients and `kube` provide typed clients for remote services. |
| Databases | [Database](../database/index.md), [Database Implementations](../database-implementations/index.md), and [SQLite](../database/sqlite.md) | [`tokio-postgres`][c~tokio-postgres~docs], [`rusqlite`][c~rusqlite~docs], and [`mongodb`][c~mongodb~docs] cover protocol clients and an embedded database. |
| Compression | [Compression](../compression/index.md), [Compression APIs](../compression/compression.md), and [Stream Zstandard Data](../compression/compression.md#zstd-stream) | [`flate2`][c~flate2~docs] wraps several backends; [`zstd`][c~zstd~docs] provides streaming Zstandard. The guide also discusses lower-level [`zstd-safe`][c~zstd-safe~docs] and [`zstd-sys`][c~zstd-sys~docs]. |
| Computer vision | [Computer Vision](../computer-vision/index.md) | [`opencv`][c~opencv~docs] wraps the native OpenCV library and requires its SDK. |
| Cryptography and TLS | [Cryptography](../cryptography/index.md), [TLS](../cryptography/tls.md) | [`openssl`][c~openssl~docs], `native-tls`, and `rustls` offer different native and pure-Rust TLS integrations. |
| Email | [Email](../email/index.md) | `lettre` sends mail through SMTP and other configured transports. |
| Filesystem notifications | [Filesystem](../filesystem/index.md) | [`inotify`][c~inotify~docs] wraps Linux filesystem events; choose a cross-platform watcher when portability matters. |
| Graphics | [Graphics](../graphics/index.md), [Graphics APIs](../rendering_graphics-api/index.md) | Rust wrappers expose Vulkan, OpenGL, and GPU interfaces while preserving each API's ownership and synchronization rules. |
| GUI | [GUI](../gui/index.md) | [`gtk4`][c~gtk4~docs] and other toolkit crates expose native or cross-platform user-interface APIs. |
| Machine learning | [Machine Learning](../../other/machine-learning/index.md) | [`tch`][c~tch~docs] binds to LibTorch; runtime and native-library setup depends on the selected backend. |
| Multimedia | [Multimedia](../multimedia/index.md), [Audio](../multimedia_audio/index.md), [Encoding](../multimedia_encoding/index.md), [Images](../multimedia_images/index.md), and [Video](../multimedia_video/index.md) | [`ffmpeg`][c~ffmpeg~docs] and [`sdl2`][c~sdl2~docs] wrap native multimedia libraries. |
| Network sockets | [Network Programming](../network-programming/index.md), [TCP Servers](../network-programming/server.md), and [socket options recipe](../network-programming/server.md#socket2-listener) | [`socket2`][c~socket2~docs] configures a socket before converting it to a standard-library type; [`curl`][c~curl~docs] wraps libcurl. |
| Operating-system APIs | [Operating Systems](../os/index.md), the [Unix API Guide](../os_unix-apis/index.md), and a [macOS API example](../os_macos-apis/macos.md) | [`nix`][c~nix~docs] wraps many Unix calls. [`sysinfo`][c~sysinfo~docs] reports portable system information. |
| Parsing libraries | [Parsing](../parsing/index.md), [Parser Implementations](../parser-implementations/index.md) | [`libxml`][c~libxml~docs] wraps libxml2; check the native-library version and its ownership rules. |
| Language runtimes | [FFI Tooling](../development-tools_ffi/index.md), [Python Binding Guide](../development-tools_ffi/python.md), [Java Binding Guide](../development-tools_ffi/java.md), and [Objective-C Binding Guide](../development-tools_ffi/objc.md) | [`pyo3`][c~pyo3~docs], [`jni`][c~jni~docs], and [`objc2`][c~objc2~docs] connect Rust with existing language runtimes. |
| HTTP clients and APIs | [HTTP Clients](../web-programming_http-client/index.md), [HTTP API Clients](../web-programming_http-client/apis.md) | [`reqwest`][c~reqwest~docs] and [`isahc`][c~isahc~docs] provide higher-level HTTP clients; select TLS, async, and retry behavior deliberately. |
| HTTP servers | [HTTP Servers](../web-programming_http-server/index.md), [Hyper](../web-programming_http-server/hyper.md), and [GraphQL](../web-programming_http-server/graphql.md) | [`axum`][c~axum~docs], [`actix-web`][c~actix-web~docs], `warp`, and `rocket` provide HTTP server APIs. |
| Game development | [Game Development](../game-development/index.md), [Game Engines](../game-engines/index.md) | SDL and engine crates expose platform graphics, audio, input, and window APIs. |
| WebAssembly | [WebAssembly](../wasm/index.md) | [`wasm-bindgen`][c~wasm-bindgen~docs] connects Rust and JavaScript values at the Wasm boundary. |

For direct HTTP calls or a small typed client, start with [HTTP API Clients](../web-programming_http-client/apis.md). Its `reqwest` examples cover JSON responses, pagination, HTTP errors, and rate limits. A service-specific client such as `aws-sdk-s3` packages the operations and types for that service.

For operating-system-specific interfaces, see [FreeBSD](../os_freebsd-apis/index.md), [Linux](../os_linux-apis/index.md), [macOS](../os_macos-apis/index.md), [Windows](../os_windows-apis/index.md), and [Unix](../os_unix-apis/index.md).

## Safe Compression API

This example uses [`flate2`][c~flate2~docs] to compress and restore bytes with gzip. Its encoder and decoder implement Rust's I/O traits, so the caller can propagate errors without calling the underlying C API.

```rust
{{#include ../../../crates/cats/api_bindings/examples/api_bindings/gzip_round_trip.rs:example}}
```

## More Wrapper Recipes

- [socket2 recipe](../network-programming/server.md#socket2-listener) configures a TCP listener before handing it to `std::net`.
- [openssl recipe](../cryptography/tls.md#openssl-tls-client) performs a TLS handshake with certificate and host-name verification.
- [`num_threads`][c~num_threads~docs] reports the current process's thread count when the target OS supports the query; see the [thread recipe](../concurrency/explicit_threads.md#process-thread-count).
- [OpenTelemetry](../development-tools_debugging/distributed_telemetry.md) instruments traces and metrics through a shared API.
- [`libloading`][c~libloading~docs] loads a native library at runtime; follow the [External FFI recipe](../external-ffi-bindings/external_ffi_bindings.md#load-a-system-library-dynamically) and [Plugin Architecture](../../other/standalone/plugin.md) guide, and load only trusted libraries.

{{#include ../../refs/link-refs.md}}
