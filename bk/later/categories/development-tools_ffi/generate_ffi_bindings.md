# Generate Foreign Function Interface (FFI) Bindings to C or C++ Code

{{#include generate_ffi_bindings.incl.md}}

## Generate Rust code that Calls into C Libraries with `bindgen` {#bindgen}

[![bindgen][c~bindgen~docs~badge]][c~bindgen~docs]{{hi:bindgen}}
[![bindgen~crates.io][c~bindgen~crates.io~badge]][c~bindgen~crates.io]
[![bindgen~github][c~bindgen~github~badge]][c~bindgen~github]
[![bindgen~lib.rs][c~bindgen~lib.rs~badge]][c~bindgen~lib.rs]
[![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}
[![cat~external-ffi-bindings][cat~external-ffi-bindings~badge]][cat~external-ffi-bindings]{{hi:External FFI bindings}}

[`bindgen`][c~bindgen~docs]↗{{hi:bindgen}} automatically generates Rust FFI bindings to C and C++ libraries. It creates Rust code allowing you to call into a C library's functions and use its types.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/c/bindgen.rs:example}}
```

## Generates C Header Files from Rust code with `cbindgen` {#cbindgen}

[![cbindgen][c~cbindgen~docs~badge]][c~cbindgen~docs]{{hi:cbindgen}}
[![cbindgen~crates.io][c~cbindgen~crates.io~badge]][c~cbindgen~crates.io]
[![cbindgen~github][c~cbindgen~github~badge]][c~cbindgen~github]
[![cbindgen~lib.rs][c~cbindgen~lib.rs~badge]][c~cbindgen~lib.rs]
[![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}
[![cat~external-ffi-bindings][cat~external-ffi-bindings~badge]][cat~external-ffi-bindings]{{hi:External FFI bindings}}

[`cbindgen`][c~cbindgen~docs]↗{{hi:cbindgen}} generates C bindings to Rust libraries, making Rust [functions][p~functions] callable from C/C++.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/c/cbindgen.rs:example}}
```

## Interop Between C++ and Rust {#cxx}

[![cxx][c~cxx~docs~badge]][c~cxx~docs]{{hi:cxx}}
[![cxx~crates.io][c~cxx~crates.io~badge]][c~cxx~crates.io]
[![cxx~github][c~cxx~github~badge]][c~cxx~github]
[![cxx~lib.rs][c~cxx~lib.rs~badge]][c~cxx~lib.rs]
[![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}}
[![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`cxx`][c~cxx~docs]↗{{hi:cxx}} provides safe C++ <-> Rust interop by generating code for both sides. [`cxx`][c~cxx~docs]↗{{hi:cxx}} handles a lot of the boilerplate and memory management. Strongly recommended for C++ interop.

While tools like 'SWIG' exist, [`cxx`][c~cxx~docs]↗{{hi:cxx}} is generally preferred for modern C++ interop with Rust, because it's more idiomatic and safer.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/cpp/cxx.rs:example}}
```

## `uniffi` {#uniffi}

[![uniffi~website][c~uniffi~website~badge]][c~uniffi~website] [![uniffi][c~uniffi~docs~badge]][c~uniffi~docs] [![uniffi~crates.io][c~uniffi~crates.io~badge]][c~uniffi~crates.io] [![uniffi~github][c~uniffi~github~badge]][c~uniffi~github] [![uniffi~lib.rs][c~uniffi~lib.rs~badge]][c~uniffi~lib.rs]{{hi:uniffi}}{{hi:Ffi}}{{hi:Bindgen}}

[`uniffi`][c~uniffi~docs]↗{{hi:uniffi}} is a multi-language bindings generator for Rust.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/uniffi.rs:example}}
```

## FFI Caveats {#skip}

- FFI code often requires _unsafe_ blocks because the Rust compiler cannot guarantee the safety of interactions with foreign code.
- Memory Management across FFI boundaries requires careful consideration. It often involves using [`Box::into_raw`]( ){{hi: }} and `Box::from_raw` to transfer ownership of memory between Rust and C.
- [Error Handling][p~error-handling] across FFI boundaries often involves returning error codes or using [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)↗{{hi:std::result::Result}} types and converting them to C-compatible representations.

## Related Topics {#related-topics}

### C / C++ Build Integration {#skip1}

[`cc`][c~cc~docs]↗{{hi:cc}} helps with compiling C/C++ code within your Rust build script ([`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html)↗{{hi:build.rs}}). This is often necessary when working with FFI or C++ bindings. See [[build_time_tooling | Build Time Tooling]].

### Utilities {#skip2}

- [`std::ffi`](https://doc.rust-lang.org/std/ffi/index.html)↗{{hi:std::ffi}} provides types for interacting with C code, like `CString`, `CStr`, `c_char`, etc.
  - Use `std::ffi::CString`, `std::ffi::CStr` for converting between Rust [strings][p~strings] and C strings.
- [`libc`][c~libc~docs]↗{{hi:libc}} provides access to the standard C library.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[generate_ffi_bindings: write](https://github.com/john-cd/rust_howto/issues/324)

- [Rust ❤️ pre-existing C++ ♡ Existing C++](https://google.github.io/autocxx/)

</div>
