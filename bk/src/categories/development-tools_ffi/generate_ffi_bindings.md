# Generate FFI bindings to C or C++ code

{{#include generate_ffi_bindings.incl.md}}

## `bindgen` {#bindgen}

[![bindgen][c-bindgen-badge]][c-bindgen]{{hi:bindgen}}
[![bindgen-crates.io][c-bindgen-crates.io-badge]][c-bindgen-crates.io]
[![bindgen-github][c-bindgen-github-badge]][c-bindgen-github]
[![bindgen-lib.rs][c-bindgen-lib.rs-badge]][c-bindgen-lib.rs]
[![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}
[![cat-external-ffi-bindings][cat-external-ffi-bindings-badge]][cat-external-ffi-bindings]{{hi:External FFI bindings}}

[`bindgen`][c-bindgen]⮳{{hi:bindgen}} automatically generates Rust FFI bindings to C and C++ libraries.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/c/bindgen.rs:example}}
```

## `cbindgen` {#cbindgen}

[![cbindgen][c-cbindgen-badge]][c-cbindgen]{{hi:cbindgen}}
[![cbindgen-crates.io][c-cbindgen-crates.io-badge]][c-cbindgen-crates.io]
[![cbindgen-github][c-cbindgen-github-badge]][c-cbindgen-github]
[![cbindgen-lib.rs][c-cbindgen-lib.rs-badge]][c-cbindgen-lib.rs]
[![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}
[![cat-external-ffi-bindings][cat-external-ffi-bindings-badge]][cat-external-ffi-bindings]{{hi:External FFI bindings}}

[`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}} generates C bindings to Rust libraries.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/c/cbindgen.rs:example}}
```

## Interop between C++ and Rust {#cxx}

[![cxx][c-cxx-badge]][c-cxx]{{hi:cxx}}
[![cxx-crates.io][c-cxx-crates.io-badge]][c-cxx-crates.io]
[![cxx-github][c-cxx-github-badge]][c-cxx-github]
[![cxx-lib.rs][c-cxx-lib.rs-badge]][c-cxx-lib.rs]
[![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}}
[![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

`cxx` provides safe C++ <-> Rust interop by generating code for both sides.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/cpp/cxx.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[generate_ffi_bindings: write (P2)](https://github.com/john-cd/rust_howto/issues/324)

## `uniffi` {#uniffi}

[![uniffi-website][c-uniffi-website-badge]][c-uniffi-website] [![uniffi][c-uniffi-badge]][c-uniffi] [![uniffi-crates.io][c-uniffi-crates.io-badge]][c-uniffi-crates.io] [![uniffi-github][c-uniffi-github-badge]][c-uniffi-github] [![uniffi-lib.rs][c-uniffi-lib.rs-badge]][c-uniffi-lib.rs]{{hi:uniffi}}{{hi:Ffi}}{{hi:Bindgen}}

[`uniffi`][c-uniffi]⮳{{hi:uniffi}} is a multi-language bindings generator for Rust.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/uniffi.rs:example}}
```

</div>
