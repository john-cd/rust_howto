# API Bindings

[![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}}

Idiomatic wrappers of specific APIs{{hi:APIs}} for convenient access from Rust. Includes HTTP API wrappers as well. Non-idiomatic or unsafe bindings can be found in external FFI bindings.

## Python interop

{{#include python_interop.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[api-bindings: add other API bindings (P2)](https://github.com/john-cd/rust_howto/issues/211)

- Calling Python from Rust:

[`cpython`][c-cpython]⮳{{hi:cpython}}: Direct bindings to the CPython interpreter. Lower-level, but very powerful.
[`pyo3`][c-pyo3]⮳{{hi:pyo3}}: A popular crate for creating Python extensions in Rust, and for embedding Python in Rust. Higher-level and easier to use than cpython for many use cases.
`rust-python`: Another option for Python bindings.

- Calling Rust from Python:

[`pyo3`][c-pyo3]⮳{{hi:pyo3}}: As mentioned above, this is also excellent for exposing Rust functions and modules to Python. This is the most common approach.
`cffi`: While primarily a Python tool, it can be used in conjunction with Rust to create bindings. Less common than [`pyo3`][c-pyo3]⮳{{hi:pyo3}} for this direction.

Data Exchange:

[`serde`][c-serde]⮳{{hi:serde}}: While not strictly Python-specific, serde's serialization capabilities are often crucial for efficiently exchanging data between Rust and Python. You'd likely use [`serde_json`][c-serde_json]⮳{{hi:serde_json}} or another format in conjunction with your chosen Python binding crate.
[`numpy`][c-numpy]⮳{{hi:numpy}}: If you're working with numerical data, integrating with NumPy is often essential. Crates like ndarray in Rust can facilitate this.
Build Tools:

[`maturin`][c-maturin]⮳{{hi:maturin}}: A popular tool for building and distributing Python packages that include Rust extensions. Handles the complexities of compilation and linking.
`setuptools-rust`: Another option for integrating Rust builds into Python's setuptools.

Link to C Bindings (FFI): `std::ffi`, [`libc`][c-libc]⮳{{hi:libc}}, [`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}}
Link to Web APIs (Client): [`reqwest`][c-reqwest]⮳{{hi:reqwest}}, [`isahc`][c-isahc]⮳{{hi:isahc}}, `surf`
Link to Web APIs (Server): [`actix-web`][c-actix_web]⮳{{hi:actix-web}}, [`warp`][c-warp]⮳{{hi:warp}}, [`rocket`][c-rocket]⮳{{hi:rocket}}, [`axum`][c-axum]⮳{{hi:axum}}
Link to JSON Serialization: [`serde`][c-serde]⮳{{hi:serde}}, [`serde_json`][c-serde_json]⮳{{hi:serde_json}}
Link to XML Serialization: [`serde`][c-serde]⮳{{hi:serde}}, [`quick-xml`][c-quick_xml]⮳{{hi:quick-xml}}
Link to Protocol Buffers: [`protobuf`][c-protobuf]⮳{{hi:protobuf}}, [`prost`][c-prost]⮳{{hi:prost}}
Link to gRPC: [`tonic`][c-tonic]⮳{{hi:tonic}}, `grpc`
Link to GraphQL: [`juniper`][c-juniper]⮳{{hi:juniper}}, [`async-graphql`][c-async_graphql]⮳{{hi:async-graphql}}
Link to Database Bindings: [`tokio-postgres`][c-tokio_postgres]⮳{{hi:tokio-postgres}}, [`rusqlite`][c-rusqlite]⮳{{hi:rusqlite}}, [`mongodb`][c-mongodb]⮳{{hi:mongodb}}, etc.
Link to OS Specific APIs: (Often through `std::os` or crates)

</div>
