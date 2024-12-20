# Generate FFI bindings to C or C++ code

{{#include generate_ffi_bindings.incl.md}}

## `bindgen` {#bindgen}

[![bindgen][c-bindgen-badge]][c-bindgen]{{hi:bindgen}}
[![bindgen-crates.io][c-bindgen-crates.io-badge]][c-bindgen-crates.io]
[![bindgen-github][c-bindgen-github-badge]][c-bindgen-github]
[![bindgen-lib.rs][c-bindgen-lib.rs-badge]][c-bindgen-lib.rs]
[![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}
[![cat-external-ffi-bindings][cat-external-ffi-bindings-badge]][cat-external-ffi-bindings]{{hi:External FFI bindings}}

Automatically generates Rust FFI bindings to C and C++ libraries.

```rust,editable
{{#include ../../../crates/ex/categories/d/tests/development_tools_ffi/bindgen.rs:example}}
```

## `cbindgen` {#cbindgen}

[![cbindgen][c-cbindgen-badge]][c-cbindgen]{{hi:cbindgen}}
[![cbindgen-crates.io][c-cbindgen-crates.io-badge]][c-cbindgen-crates.io]
[![cbindgen-github][c-cbindgen-github-badge]][c-cbindgen-github]
[![cbindgen-lib.rs][c-cbindgen-lib.rs-badge]][c-cbindgen-lib.rs]
[![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}
[![cat-external-ffi-bindings][cat-external-ffi-bindings-badge]][cat-external-ffi-bindings]{{hi:External FFI bindings}}

Generate C bindings to Rust libraries

```rust,editable
{{#include ../../../crates/ex/categories/d/tests/development_tools_ffi/cbindgen.rs:example}}
```

## `cxx` {#cxx}

[![cxx][c-cxx-badge]][c-cxx]{{hi:cxx}}
[![cxx-crates.io][c-cxx-crates.io-badge]][c-cxx-crates.io]
[![cxx-github][c-cxx-github-badge]][c-cxx-github]
[![cxx-lib.rs][c-cxx-lib.rs-badge]][c-cxx-lib.rs]
[![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}}
[![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Safe C++ <-> Rust interop by generating code for both sides.

```rust,editable
{{#include ../../../crates/ex/categories/d/tests/development_tools_ffi/cxx.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[generate_ffi_bindings: write (P2)](https://github.com/john-cd/rust_howto/issues/324)

</div>
