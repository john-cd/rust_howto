<!-- TODO Reviewed ??-??-2026. -->

# API Bindings

[![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}}

An API binding gives Rust code a typed interface to a capability implemented elsewhere. That capability may be a native library in the same process, an operating-system service, or a remote HTTP service. Remote clients do not cross a native ABI boundary; low-level declarations for C libraries and system APIs belong in [[external-ffi-bindings | External FFI Bindings]]. Use the FFI tooling chapter when generating or maintaining bindings for an API you own.

## Choose the Integration Layer

| Situation | Start with | Check before adopting |
| --- | --- | --- |
| A remote HTTP service | A Rust client crate such as `reqwest`, `isahc`, or `surf` | Async runtime requirements, TLS defaults, retries, timeouts, and how status and transport errors are exposed. |
| A remote database server | Its protocol client, such as `tokio-postgres` or `mongodb` | Connection pooling, server-version support, transaction behavior, and how errors are exposed. |
| A database embedded in the application | A local database wrapper such as `rusqlite` | Whether the crate bundles or links the native library, and how the database file and connection are managed. |
| A native library with a maintained Rust wrapper | The safe wrapper rather than its `-sys` crate | Platform support, native SDK installation, enabled features, and whether the wrapper owns foreign resources safely. |
| An API without a suitable wrapper | The raw binding or a generated binding behind a small safe module | ABI and version compatibility, handle ownership, pointer validity, thread-safety, and link-time system dependencies. |

Prefer the narrowest maintained interface that covers the required API. Keep credentials and network policy at the application boundary, and keep `unsafe` calls and foreign ownership rules inside a wrapper module.

## Safe Compression API

This example uses [`flate2`][c~flate2~docs]↗ to compress and restore a byte
string with gzip. The encoder and decoder expose ordinary Rust I/O traits, so
callers can handle failures with `Result` without calling the underlying C API
directly:

```rust
{{#include ../../../crates/cats/api_bindings/examples/api_bindings/gzip_round_trip.rs:example}}
```

## Database Bindings

Bindings for popular databases like PostgreSQL, MySQL, and SQLite. Numerous crates exist, including [`tokio-postgres`][c~tokio-postgres~docs]↗{{hi:tokio-postgres}}, [`rusqlite`][c~rusqlite~docs]↗{{hi:rusqlite}}, [`mongodb`][c~mongodb~docs]↗{{hi:mongodb}}, etc.

Refer to [[database | Database]] and [[database-implementations | Database Implementations]].

## Compression APIs

Bindings to data compression libraries: see [[compression | Compression]].

## Computer Vision APIs

See [[computer-vision | Computer Vision]].

## Email APIs

See [[email | Email]].

## Filesystem APIs

Bindings for filesystem operations and libraries, including [`inotify`][c~inotify~docs]↗{{hi:inotify}} (bindings to the 'inotify' API for monitoring filesystem events on Linux). See [[filesystem | Filesystem]].

## Graphics APIs

Bindings for graphics libraries such as Vulkan and OpenGL: see [[graphics | Graphics]] and [[rendering_graphics-api | Rendering: Graphics API]].

## Machine Learning APIs

Bindings for machine learning frameworks like [`TensorFlow`][tensorflow~website]↗{{hi:TensorFlow}} and `PyTorch` (e.g. [`tch-rs`][c~tch~docs]↗{{hi:tch-rs}}): See [[machine-learning | Machine Learning]].

## Multimedia APIs

Examples include [`ffmpeg`][c~ffmpeg~docs]↗{{hi:ffmpeg}} (bindings to the FFmpeg library) and [`sdl2`][c~sdl2~docs]↗{{hi:sdl2}} (Bindings to the Simple DirectMedia Layer (SDL) library for multimedia applications). See [[multimedia | Multimedia]], [[multimedia_audio | Multimedia: Audio]], [[multimedia_encoding | Multimedia: Encoding]], [Multimedia Images](https://john-cd.com/rust_howto/categories/multimedia_images/index.html), and [[multimedia_video | Multimedia Video]].

## Networking APIs

Bindings for networking libraries and protocols, such as [`curl`][c~curl~docs]↗{{hi:curl}} (bindings to the libcurl library for transferring data via URLs): see [[network-programming | Network Programming]].

## Operating System APIs

See [[os | OS]].

## Operating System-specific APIs

Bindings for OS-specific functionality and system calls: you will often use [`std::os`][c~std::os~docs]↗{{hi:std::os}} or crates like [`nix`][c~nix~docs]↗{{hi:nix}} (Unix-like API bindings) or `users`{{hi:users}} (a library to query user and group information on Unix-like systems). Refer to:

- [[os_freebsd-apis | OS Freebsd APIs]].
- [[os_linux-apis | OS Linux APIs]].
- [[os_macos-apis | OS Macos APIs]].
- [[os_unix-apis | OS Unix APIs]].
- [[os_windows-apis | OS Windows APIs]].

Use the [`sysinfo`][c~sysinfo~docs]↗{{hi:sysinfo}} cross-platform library to fetch system information.

## Parsing APIs

Consider [`libxml`][c~libxml~docs]↗{{hi:libxml2}} (bindings to the libxml2 library) for XML parsing. See [[parsing | Parsing]] and [[parser-implementations | Parser Implementations]].

## Programming Language Bindings

See [[development-tools_ffi | Development Tools FFI]].

## Web APIs (Client)

Consider using [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} (or [`isahc`][c~isahc~docs]↗{{hi:isahc}} or [`surf`][c~surf~docs]↗{{hi:surf}}). Refer to:

- [[http_clients | HTTP Clients]].
- [[web-programming_http-client | Web Programming HTTP Client]].

## Web APIs (Server)

Commonly used crates include [`axum`][c~axum~docs]↗{{hi:axum}}, [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}}, [`warp`][c~warp~docs]↗{{hi:warp}}, and [`rocket`][c~rocket~docs]↗{{hi:rocket}}. See:

- [[web-programming_http-server | Web Programming HTTP Server]].
- [[apis | APIs]].
- [[web-programming | Web Programming]].
- [[_graphql | GraphQL]].

## Game Development APIs

Bindings for game development libraries and engines - see [[game_development | Game Development]] and [[game-engines | Game Engines]].

## Cryptography APIs

Bindings for cryptographic libraries and protocols, such as [`sodiumoxide`][c~sodiumoxide~docs]↗{{hi:sodiumoxide}} (a safe Rust binding to the Networking and Cryptography (NaCl / Sodium) library) or [`openssl`][c~openssl~docs]↗{{hi:openssl}} (bindings to the OpenSSL library). See:

- [[cryptography | Cryptography]].
- [[encryption | Encryption]].
- [[hashing | Hashing]].

## WebAssembly APIs

Bindings for WebAssembly and related technologies - refer to [[wasm | WASM]].

## GUI APIs

Bindings to Graphical User Interfaces, such as bindings to the GTK library - see [[gui | GUI]].

## Related Topics

- [[external-ffi-bindings | External FFI Bindings]]
- [[development-tools_ffi | Development Tools FFI]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
