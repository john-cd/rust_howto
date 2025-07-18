# FFI - Foreign Functions Interfaces / Interop

[![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}

This chapter covers tools to help you better interface with other languages. This includes binding generators and helpful language constructs. Existing, non-idiomatic or unsafe FFI bindings may be found in [[external-ffi-bindings | External FFI Bindings]]. Idiomatic wrappers can be found in the [[api-bindings | API Bindings]] section.

## Generate FFI Bindings to C or C++ Code

- [`cbindgen`][c~cbindgen~docs]⮳{{hi:cbindgen}} generates C header files from Rust code.

{{#include generate_ffi_bindings.incl.md}}

## Calling Rust from Other Languages

While [`cbindgen`][c~cbindgen~docs]⮳{{hi:cbindgen}} helps with C, other languages might have their own FFI mechanisms. [`cpython`][c~cpython~docs]⮳{{hi:cpython}} and [`pyo3`][c~pyo3~docs]⮳{{hi:pyo3}} are common for Python.

## JavaScript (WebAssembly)

[`wasm-bindgen`][c~wasm_bindgen~docs]⮳{{hi:wasm-bindgen}} facilitates communication between Rust and JavaScript when compiling to WebAssembly.

Refer to [[wasm | WASM]].

## Node.js (NAPI)

[`neon`][c~neon~docs]⮳{{hi:neon}} makes it easy to write Node.js addons in Rust.

{{#include node.incl.md}}

## Python

- [`pyo3`][c~pyo3~docs]⮳{{hi:pyo3}}: Excellent for creating Python extensions in Rust and embedding Python in Rust. Very versatile.
- [`cpython`][c~cpython~docs]⮳{{hi:cpython}}: Lower-level bindings to the CPython interpreter. More complex, but offers more control.
- `rust-python`: Another option for Python bindings.

{{#include python.incl.md}}

## Java

{{#include java.incl.md}}

## Ruby

{{#include ruby.incl.md}}

## Dart & Flutter

{{#include flutter.incl.md}}

## Objective C

{{#include objc.incl.md}}

## Lua

{{#include lua.incl.md}}

## Erlang & Elixir

{{#include erlang_elixir.incl.md}}

## Other Languages

Many languages have their own FFI mechanisms. You'll often need to find or create bindings specific to the language you want to interact with. Sometimes, tools like SWIG can be used, but are less common with Rust than other languages.

### General Binding Generators (Less Common with Rust)

While widely used with other languages, `SWIG` (Simplified Wrapper and Interface Generator) is less frequently used with Rust due to the effectiveness of crates like [`pyo3`][c~pyo3~docs]⮳{{hi:pyo3}} and [`wasm-bindgen`][c~wasm_bindgen~docs]⮳{{hi:wasm-bindgen}}.

## Related Topics

### Build Tools

[`cargo`][c~cargo~docs]⮳{{hi:cargo}}: (Essential for building Rust projects that involve FFI or bindings.)
[`maturin`][c~maturin~docs]⮳{{hi:maturin}}: Specifically for building and distributing Python packages that include Rust extensions.

## Build System Integration (for FFI)

Usually handled with build scripts (`build.rs`) and the [`cc`][c~cc~docs]⮳{{hi:cc}} crate for compiling C/C++ code, if necessary.

- [[build_utils | Build Utils]].
- [[code_build | Code Build]].
- [[building | Building]].
- [[development-tools_build-utils | Development Tools: Build Utils]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/325)
</div>
