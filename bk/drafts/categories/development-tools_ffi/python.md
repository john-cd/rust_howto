# Python Interop

{{#include python.incl.md}}

## Interop with Python {#pyo3}

[![pyo3][c-pyo3-badge]][c-pyo3] [![pyo3-crates.io][c-pyo3-crates.io-badge]][c-pyo3-crates.io] [![pyo3-github][c-pyo3-github-badge]][c-pyo3-github] [![pyo3-lib.rs][c-pyo3-lib.rs-badge]][c-pyo3-lib.rs]{{hi:pyo3}}{{hi:Cpython}}{{hi:Ffi}}{{hi:pyo3}}{{hi:Python}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}

[`pyo3`][c-pyo3]⮳{{hi:pyo3}} is a Rust crate that enables seamless integration between Rust and Python. It allows developers to write Python extensions in Rust, leveraging Rust's performance and safety benefits, or to embed Python within Rust applications. Pyo3 provides tools for managing Python objects, handling data exchange, and defining Python classes and modules in Rust.

[`pyo3`][c-pyo3-website]⮳ supports both calling python code from Rust and exposing Rust code to Python.

```rust,editable
{{#include ../../../crates/cats/api_bindings/tests/pyo3.rs:example}}
```

Use Rust from Python:

```rust
{{#include ../../../crates/cats/api_bindings/tests/use_rust_from_python.rs}}
```

## Calling Rust from Python

[`pyo3`][c-pyo3]⮳{{hi:pyo3}}: As mentioned above, this is also excellent for exposing Rust functions and modules to Python. This is the most common approach.

[[python_interop | Python Interop]]

[`cffi`][c-cffi]⮳{{hi:cffi}}: While primarily a Python tool, it can be used in conjunction with Rust to create bindings. Less common than [`pyo3`][c-pyo3]⮳{{hi:pyo3}} for this direction.

## Calling Python from Rust

[`cpython`][c-cpython]⮳{{hi:cpython}}: Direct bindings to the CPython interpreter. Lower-level, but very powerful.
[`pyo3`][c-pyo3]⮳{{hi:pyo3}}: A popular crate for creating Python extensions in Rust, and for embedding Python in Rust. Higher-level and easier to use than cpython for many use cases.
`rust-python`: Another option for Python bindings.

## Data Exchange

[`serde`][c-serde]⮳{{hi:serde}}: While not strictly Python-specific, serde's serialization capabilities are often crucial for efficiently exchanging data between Rust and Python. You'd likely use [`serde_json`][c-serde_json]⮳{{hi:serde_json}} or another format in conjunction with your chosen Python binding crate.

See:

- [[complex_encoding | Complex Encoding]].
- [[encoding | Encoding]].
- [[json | JSON]].

## Numerical Data

[`numpy`][c-numpy]⮳{{hi:numpy}}: If you're working with numerical data, integrating with NumPy is often essential. Crates like `ndarray` in Rust can facilitate this.

## Build Tools

[`maturin`][c-maturin]⮳{{hi:maturin}}: A popular tool for building and distributing Python packages that include Rust extensions. Handles the complexities of compilation and linking.
`setuptools-rust`: Another option for integrating Rust builds into Python's setuptools.

See [[development-tools_build-utils | Build Utils]] and [[code_build | Code Build]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[python_interop: write](https://github.com/john-cd/rust_howto/issues/210)

</div>
