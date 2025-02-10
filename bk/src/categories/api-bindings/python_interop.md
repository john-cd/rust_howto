# Python Interop

{{#include python_interop.incl.md}}

## Interop with Python {#pyo3}

[![pyo3][c-pyo3-badge]][c-pyo3] [![pyo3-crates.io][c-pyo3-crates.io-badge]][c-pyo3-crates.io] [![pyo3-github][c-pyo3-github-badge]][c-pyo3-github] [![pyo3-lib.rs][c-pyo3-lib.rs-badge]][c-pyo3-lib.rs]{{hi:pyo3}}{{hi:Cpython}}{{hi:Ffi}}{{hi:pyo3}}{{hi:Python}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}

[`pyo3`][c-pyo3-website]⮳ supports both calling python code from Rust and exposing Rust code to Python.

```rust,editable
{{#include ../../../crates/ex/cats/api_bindings/tests/pyo3.rs:example}}
```

Use Rust from Python:

```rust
{{#include ../../../crates/ex/cats/api_bindings/tests/use_rust_from_python.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[python_interop: write (P2)](https://github.com/john-cd/rust_howto/issues/210)

</div>
