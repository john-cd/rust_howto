# API Bindings

[![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}}

Idiomatic wrappers of specific APIs{{hi:APIs}} for convenient access from Rust. Includes HTTP API wrappers as well. Non-idiomatic or unsafe bindings can be found in external FFI bindings.

## Python interop

{{#include python_interop.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[api-bindings: add other API bindings (P2)](https://github.com/john-cd/rust_howto/issues/211)

## Calling Python from Rust

[`cpython`][c-cpython]⮳{{hi:cpython}}: Direct bindings to the CPython interpreter. Lower-level, but very powerful.
[`pyo3`][c-pyo3]⮳{{hi:pyo3}}: A popular crate for creating Python extensions in Rust, and for embedding Python in Rust. Higher-level and easier to use than cpython for many use cases.
`rust-python`: Another option for Python bindings.

[[python_interop | Python Interop]]

## Calling Rust from Python

[`pyo3`][c-pyo3]⮳{{hi:pyo3}}: As mentioned above, this is also excellent for exposing Rust functions and modules to Python. This is the most common approach.

[[python_interop | Python Interop]]

[`cffi`][c-cffi]⮳{{hi:cffi}}: While primarily a Python tool, it can be used in conjunction with Rust to create bindings. Less common than [`pyo3`][c-pyo3]⮳{{hi:pyo3}} for this direction.

## Data Exchange

[`serde`][c-serde]⮳{{hi:serde}}: While not strictly Python-specific, serde's serialization capabilities are often crucial for efficiently exchanging data between Rust and Python. You'd likely use [`serde_json`][c-serde_json]⮳{{hi:serde_json}} or another format in conjunction with your chosen Python binding crate.
[[complex_encoding | Complex Encoding]]
[[encoding | Encoding]]
[[json | JSON]]

[`numpy`][c-numpy]⮳{{hi:numpy}}: If you're working with numerical data, integrating with NumPy is often essential. Crates like `ndarray` in Rust can facilitate this.

## Build Tools

[`maturin`][c-maturin]⮳{{hi:maturin}}: A popular tool for building and distributing Python packages that include Rust extensions. Handles the complexities of compilation and linking.
`setuptools-rust`: Another option for integrating Rust builds into Python's setuptools.

## C Bindings (FFI)

`std::ffi`, [`libc`][c-libc]⮳{{hi:libc}}, [`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}}
[[os | OS]]
[[development-tools_ffi | Development Tools FFI]]
[[external-ffi-bindings | External FFI Bindings]]
[[external_ffi_bindings | External FFI Bindings]]
[[generate_ffi_bindings | Generate FFI Bindings]]

[[http_clients | HTTP Clients]]
[[web-programming_http-client | Web Programming HTTP Client]]

## Web APIs (Client)

[`reqwest`][c-reqwest]⮳{{hi:reqwest}}, [`isahc`][c-isahc]⮳{{hi:isahc}}, [`surf`][c-surf]⮳{{hi:surf}}
[[apis | APIs]]
[[web-programming | Web Programming]]
[[web-programming_http-server | Web Programming HTTP Server]]

## Web APIs (Server)

[`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`warp`][c-warp]⮳{{hi:warp}}, [`rocket`][c-rocket]⮳{{hi:rocket}}, [`axum`][c-axum]⮳{{hi:axum}}
[[web-programming_http-server | Web Programming HTTP Server]]

## JSON Serialization

[`serde`][c-serde]⮳{{hi:serde}}, [`serde_json`][c-serde_json]⮳{{hi:serde_json}}
[[json | JSON]]

## XML Serialization

[`serde`][c-serde]⮳{{hi:serde}}, [`quick-xml`][c-quick_xml]⮳{{hi:quick-xml}}
[[xml | XML]]

## Protocol Buffers

[`protobuf`][c-protobuf]⮳{{hi:protobuf}}, [`prost`][c-prost]⮳{{hi:prost}}
[[_binary_encoders | Binary Encoders]]

## gRPC

[`tonic`][c-tonic]⮳{{hi:tonic}}, [`grpc`][c-grpc]⮳{{hi:grpc}}
[[_grpc | gRPC]]

## GraphQL

[`juniper`][c-juniper]⮳{{hi:juniper}}, [`async-graphql`][c-async_graphql]⮳{{hi:async-graphql}}
[[_graphql | GraphQL]]

## Database Bindings

[`tokio-postgres`][c-tokio_postgres]⮳{{hi:tokio-postgres}}, [`rusqlite`][c-rusqlite]⮳{{hi:rusqlite}}, [`mongodb`][c-mongodb]⮳{{hi:mongodb}}, etc.
[[database | Database]]

## OS Specific APIs

(Often through `std::os` or crates)
[[os | OS]]
[[os_freebsd-apis | OS Freebsd APIs]]
[[os_linux-apis | OS Linux APIs]]
[[os_macos-apis | OS Macos APIs]]
[[os_unix-apis | OS Unix APIs]]
[[os_windows-apis | OS Windows APIs]]

</div>
