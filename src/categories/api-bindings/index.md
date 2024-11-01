# API Bindings

Idiomatic wrappers of specific APIs{{hi:APIs}} for convenient access from Rust. Includes HTTP API wrappers as well. Non-idiomatic or unsafe bindings can be found in external FFI bindings.

{{#include index.incl.md}}

## Interop

[![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}}

### Python Interop

[![pyo3][c-pyo3-badge]][c-pyo3]{{hi:pyo3}}  [pyo3 website][c-pyo3-website]⮳  [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]

Supports both calling python code from Rust and exposing Rust code to Python

```rust
{{#include ../../../deps/tests/cats/api_bindings/pyo3.rs:example}}
```

### Rust tools for Python

[![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]

[![pyOxidizer][c-pyoxidizer-badge]][c-pyoxidizer-github]{{hi:pyOxidizer}}⮳

[Ruff][c-ruff-website]{{hi:ruff}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: write
</div>
