# External FFI Bindings

[![cat-external-ffi-bindings][cat-external-ffi-bindings-badge]][cat-external-ffi-bindings]{{hi:External FFI bindings}}

{{#include external_ffi_bindings.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[external-ffi-bindings/_index: fix (P2)](https://github.com/john-cd/rust_howto/issues/356)

## Foreign Function Interface (FFI) Basics

`std::ffi` (provides types for interacting with C code, like `CString`, `CStr`, `c_char`, etc.)

## C Bindings

[`libc`][c-libc]⮳{{hi:libc}} (provides definitions for C standard library functions and types)

## Generating C Header Files

[`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}}  (generates C header files from Rust code, crucial for making Rust functions callable from C)

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

While [`cbindgen`][c-cbindgen]⮳{{hi:cbindgen}}  helps with C, other languages might have their own FFI mechanisms. [`cpython`][c-cpython]⮳{{hi:cpython}} and [`pyo3`][c-pyo3]⮳{{hi:pyo3}} are common for Python.

</div>
