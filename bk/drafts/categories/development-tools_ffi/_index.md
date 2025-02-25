# FFI - Foreign functions interfaces / Interop

[![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}

Tools to help you better interface with other languages. This includes binding generators and helpful language constructs.

## Generate FFI bindings

{{#include generate_ffi_bindings.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[development-tools_ffi/_index: write (P2)](https://github.com/john-cd/rust_howto/issues/325)

link to pyo3 file as well

C Bindings (FFI):

[`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}}: Generates C header files from Rust code.
`std::ffi`, [`libc`][c-libc]⮳{{hi:libc}}: Standard library modules for FFI.

Python:

[`pyo3`][c-pyo3]⮳{{hi:pyo3}}: Excellent for creating Python extensions in Rust and embedding Python in Rust. Very versatile.
[`cpython`][c-cpython]⮳{{hi:cpython}}: Lower-level bindings to the CPython interpreter. More complex, but offers more control.
`rust-python`: Another option for Python bindings.

JavaScript (WebAssembly):

[`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}}: Facilitates communication between Rust and JavaScript when compiling to WebAssembly.

Node.js (NAPI):

[`neon`][c-neon]⮳{{hi:neon}}: Makes it easy to write Node.js addons in Rust.

Other Languages:

(Many languages have their own FFI mechanisms. You'll often need to find or create bindings specific to the language you want to interact with. Sometimes, tools like SWIG can be used, but are less common with Rust than other languages.)

Build Tools:

[`cargo`][c-cargo]⮳{{hi:cargo}}: (Essential for building Rust projects that involve FFI or bindings.)
[`maturin`][c-maturin]⮳{{hi:maturin}}: Specifically for building and distributing Python packages that include Rust extensions.

General Binding Generators (Less Common with Rust):

`SWIG` (Simplified Wrapper and Interface Generator): While widely used with other languages, it's less frequently used with Rust due to the effectiveness of crates like [`pyo3`][c-pyo3]⮳{{hi:pyo3}} and [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}}
</div>
