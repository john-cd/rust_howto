# FFI - Foreign functions interfaces / Interop

[![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}

Tools to help you better interface with other languages. This includes binding generators and helpful language constructs.

## Generate FFI bindings to C or C++ code

[`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}}: Generates C header files from Rust code.
`std::ffi`, [`libc`][c-libc]⮳{{hi:libc}}: Standard library modules for FFI.

{{#include generate_ffi_bindings.incl.md}}

## JavaScript (WebAssembly)

[`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} facilitates communication between Rust and JavaScript when compiling to WebAssembly.

Refer to [[wasm | WASM]].

## Node.js (NAPI)

[`neon`][c-neon]⮳{{hi:neon}} makes it easy to write Node.js addons in Rust.

{{#include node.incl.md}}

## Python

- [`pyo3`][c-pyo3]⮳{{hi:pyo3}}: Excellent for creating Python extensions in Rust and embedding Python in Rust. Very versatile.
- [`cpython`][c-cpython]⮳{{hi:cpython}}: Lower-level bindings to the CPython interpreter. More complex, but offers more control.
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

`SWIG` (Simplified Wrapper and Interface Generator): While widely used with other languages, it's less frequently used with Rust due to the effectiveness of crates like [`pyo3`][c-pyo3]⮳{{hi:pyo3}} and [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}}

## Related Topics

### Build Tools

[`cargo`][c-cargo]⮳{{hi:cargo}}: (Essential for building Rust projects that involve FFI or bindings.)
[`maturin`][c-maturin]⮳{{hi:maturin}}: Specifically for building and distributing Python packages that include Rust extensions.

- [[build_utils | Build Utils]]
- [[code_build | Code Build]]
- [[building | Building]]
- [[development-tools_build-utils | Development Tools: Build Utils]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[development-tools_ffi/_index: write (P2)](https://github.com/john-cd/rust_howto/issues/325)

## Foreign Function Interface (FFI) Basics

`std::ffi` (provides types for interacting with C code, like `CString`, `CStr`, `c_char`, etc.)

## Generating C Header Files

[`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}} (generates C header files from Rust code, crucial for making Rust functions callable from C)

## Memory Management across FFI boundaries

Requires careful consideration; often involves using Box::into_raw and Box::from_raw to transfer ownership of memory between Rust and C

## String Conversion

`std::ffi::CString`, `std::ffi::CStr` for converting between Rust strings and C strings

## Error Handling (across FFI boundaries)

Often involves returning error codes or using Result types and converting them to C-compatible representations

## Build System Integration (for FFI)

Usually handled with build scripts (`build.rs`) and the [`cc`][c-cc]⮳{{hi:cc}} crate for compiling C/C++ code, if necessary.

## Safety (unsafe blocks)

FFI code often requires unsafe blocks because the Rust compiler cannot guarantee the safety of interactions with foreign code.

## Calling Rust from other languages

While [`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}} helps with C, other languages might have their own FFI mechanisms. [`cpython`][c-cpython]⮳{{hi:cpython}} and [`pyo3`][c-pyo3]⮳{{hi:pyo3}} are common for Python.

## C Bindings (FFI)

`std::ffi`, [`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}}

See [[external-ffi-bindings | External FFI Bindings]]

</div>
