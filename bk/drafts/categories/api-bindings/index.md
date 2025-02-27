# API Bindings

[![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}}

Idiomatic wrappers of specific APIs{{hi:APIs}} for convenient access from Rust. Includes HTTP API wrappers as well. Non-idiomatic or unsafe bindings can be found in [[external-ffi-bindings | External FFI Bindings]].

## Database Bindings

Bindings for popular databases like PostgreSQL, MySQL, and SQLite. Numerous crates exist, including [`tokio-postgres`][c-tokio_postgres]⮳{{hi:tokio-postgres}}, [`rusqlite`][c-rusqlite]⮳{{hi:rusqlite}}, [`mongodb`][c-mongodb]⮳{{hi:mongodb}}, etc.

Refer to [[database | Database]] and [[database-implementations | Database Implementations]].

## Compression APIs

Bindings to data compression libraries - for example `zlib` bindings to the zlib library: see [[compression | Compression]].

## Computer Vision APIs

See [[computer-vision | Computer Vision]].

## Email APIs

See [[email | Email]].

## Filesystem APIs

Bindings for filesystem operations and libraries, including `inotify` (bindings to the inotify API for monitoring filesystem events on Linux). See [[filesystem | Filesystem]].

## Graphics APIs

Bindings for graphics libraries such as Vulkan and OpenGL: see [[graphics | Graphics]] and [[rendering_graphics-api | Rendering: Graphics API]].

## Machine Learning APIs

Bindings for machine learning frameworks like `TensorFlow` and `PyTorch` (e.g. `tch-rs`): see [[machine-learning | Machine Learning]].

## Multimedia APIs

Examples include `ffmpeg` (bindings to the FFmpeg library) and `sdl2` (Bindings to the Simple DirectMedia Layer (SDL) library for multimedia applications). See [[multimedia | Multimedia]], [[multimedia_audio | Multimedia: Audio]], [[multimedia_encoding | Multimedia: Encoding]], [[multimedia_images | Multimedia Images]], and [[multimedia_video | Multimedia Video]].

## Networking APIs

Bindings for networking libraries and protocols, such as `curl` (bindings to the libcurl library for transferring data via URLs): see [[network-programming | Network Programming]].

## Operating System APIs

See [[os | OS]].

## Operating System-specific APIs

Bindings for OS-specific functionality and system calls: you will often use `std::os` or crates like `nix` (Unix-like API bindings) or `users` (a library to query user and group information on Unix-like systems). Refer to:

- [[os_freebsd-apis | OS Freebsd APIs]].
- [[os_linux-apis | OS Linux APIs]].
- [[os_macos-apis | OS Macos APIs]].
- [[os_unix-apis | OS Unix APIs]].
- [[os_windows-apis | OS Windows APIs]].

Use the `sysinfo` cross-platform library to fetch system information.

## Parsing APIs

Consider `libxml2` (bindings to the libxml2 library) for XML parsing. See [[parsing | Parsing]] and [[parser-implementations | Parser Implementations]].

## Programming Language Bindings

See [[development-tools_ffi | Development Tools FFI]].

TODO

## Web APIs (Client)

Consider using [`reqwest`][c-reqwest]⮳{{hi:reqwest}} (or [`isahc`][c-isahc]⮳{{hi:isahc}} or [`surf`][c-surf]⮳{{hi:surf}}). Refer to:

- [[http_clients | HTTP Clients]].
- [[web-programming_http-client | Web Programming HTTP Client]].

## Web APIs (Server)

Commonly used crates include [`axum`][c-axum]⮳{{hi:axum}}, [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`warp`][c-warp]⮳{{hi:warp}}, and [`rocket`][c-rocket]⮳{{hi:rocket}}. See:

- [[web-programming_http-server | Web Programming HTTP Server]].
- [[apis | APIs]].
- [[web-programming | Web Programming]].
- [[_graphql | GraphQL]].

## Game Development APIs

Bindings for game development libraries and engines - see [[game_development | Game Development]] and [[game-engines | Game Engines]].

## Cryptography APIs

Bindings for cryptographic libraries and protocols, such as `sodiumoxide` (a safe Rust binding to the Networking and Cryptography (NaCl / Sodium) library) or `openssl` (bindings to the OpenSSL library). See:

- [[cryptography | Cryptography]].
- [[encryption | Encryption]].
- [[hashing | Hashing]].

## WebAssembly APIs

Bindings for WebAssembly and related technologies - refer to [[wasm | WASM]].

## GUI APIs

Bindings to Graphical User Interfaces, such as `gtk-rs` (bindings to the GTK library) - see [[gui | GUI]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[api-bindings: polish (P2)](https://github.com/john-cd/rust_howto/issues/211)

</div>
