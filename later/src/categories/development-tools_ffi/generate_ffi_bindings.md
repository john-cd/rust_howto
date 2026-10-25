<!-- TODO Reviewed ??-??-2026 -->

# Generate Foreign Function Interface (FFI) Bindings to C or C++ Code

{{#include generate_ffi_bindings.incl.md}}

## Choose a Binding Approach {#choose-a-binding-approach}

- Call an existing C library from Rust: use [`bindgen`][c~bindgen~docs]↗{{hi:bindgen}} to generate Rust declarations from its headers.
- Make Rust functions and types available to C: use [`cbindgen`][c~cbindgen~docs]↗{{hi:cbindgen}} to generate a C header for a Rust library.
- Call between Rust and C++: use [`cxx`][c~cxx~docs]↗{{hi:cxx}} for a reviewed bridge, or [`autocxx`][book~autocxx]↗{{hi:autocxx}} when a large existing C++ API makes handwritten bridge declarations impractical.
- Expose a Rust API to Kotlin, Swift, or Python: use [`uniffi`][c~uniffi~docs]↗{{hi:uniffi}} to generate language bindings from a cross-language interface definition.

Choose a binding generator for the direction and languages involved; these tools do not remove the need to design a stable ABI and ownership contract.

## Generate Rust code that Calls into C Libraries with `bindgen` {#bindgen}

[![bindgen][c~bindgen~docs~badge]][c~bindgen~docs]{{hi:bindgen}}
[![bindgen~crates.io][c~bindgen~crates.io~badge]][c~bindgen~crates.io]
[![bindgen~repo][c~bindgen~repo~badge]][c~bindgen~repo]
[![bindgen~lib.rs][c~bindgen~lib.rs~badge]][c~bindgen~lib.rs]
[![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}
[![cat~external-ffi-bindings][cat~external-ffi-bindings~badge]][cat~external-ffi-bindings]{{hi:External FFI bindings}}

[`bindgen`][c~bindgen~docs]↗{{hi:bindgen}} automatically generates Rust FFI bindings to C and C++ libraries. It creates Rust code allowing you to call into a C library's functions and use its types.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/c/bindgen.rs:example}}
```

## Generate C Header Files from Rust Code with `cbindgen` {#cbindgen}

[![cbindgen][c~cbindgen~docs~badge]][c~cbindgen~docs]{{hi:cbindgen}}
[![cbindgen~crates.io][c~cbindgen~crates.io~badge]][c~cbindgen~crates.io]
[![cbindgen~repo][c~cbindgen~repo~badge]][c~cbindgen~repo]
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
[![cxx~repo][c~cxx~repo~badge]][c~cxx~repo]
[![cxx~lib.rs][c~cxx~lib.rs~badge]][c~cxx~lib.rs]
[![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}}
[![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`cxx`][c~cxx~docs]↗{{hi:cxx}} provides safe C++ <-> Rust interop by generating code for both sides. [`cxx`][c~cxx~docs]↗{{hi:cxx}} handles a lot of the boilerplate and memory management. Strongly recommended for C++ interop.

The example uses a C++17 build and calls functions in both directions across the bridge.

While tools like 'SWIG' exist, [`cxx`][c~cxx~docs]↗{{hi:cxx}} is generally preferred for modern C++ interop with Rust, because it's more idiomatic and safer.

For a large existing C++ API, the [Rust ❤️ Existing C++ guide][book~autocxx]↗{{hi:autocxx}} introduces `autocxx`, which generates a `cxx` bridge from C++ headers. Review the generated surface and keep the boundary narrow.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/cpp/cxx.rs:example}}
```

## `uniffi` {#uniffi}

[![uniffi~website][c~uniffi~website~badge]][c~uniffi~website] [![uniffi][c~uniffi~docs~badge]][c~uniffi~docs] [![uniffi~crates.io][c~uniffi~crates.io~badge]][c~uniffi~crates.io] [![uniffi~repo][c~uniffi~repo~badge]][c~uniffi~repo] [![uniffi~lib.rs][c~uniffi~lib.rs~badge]][c~uniffi~lib.rs]{{hi:uniffi}}{{hi:Ffi}}{{hi:Bindgen}}

[`uniffi`][c~uniffi~docs]↗{{hi:uniffi}} is a multi-language bindings generator for Rust.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/uniffi.rs:example}}
```

## FFI Caveats {#ffi-caveats}

- Use `#[repr(C)]` on shared structs and C-compatible field types. Rust's default layout is not a stable C ABI; `repr(C)` does not make Rust-only types safe to expose. See the [Rust type-layout reference][rust-reference~type-layout].
- Define who owns, borrows, and frees each pointer at the boundary. Keep allocation and deallocation paired: for example, a pointer from `CString::into_raw` must return to Rust for `CString::from_raw`, not be freed with C's `free`.
- Use [`CString`][c~std::ffi~CString~docs] to create an owned, NUL-terminated C string and [`CStr`][c~std::ffi~CStr~docs] to borrow one. Validate pointer lifetime, NUL termination, mutability, and UTF-8 before converting data.
- An `extern "C"` boundary must not unwind. Catch Rust panics and translate them into the API's error result, or abort; use `extern "C-unwind"` only when both sides intentionally support unwinding. See the [Rustonomicon FFI and unwinding guide][rust-nomicon~ffi].
- Keep each `unsafe` operation small and document its preconditions with a nearby `// SAFETY:` comment. The caller must uphold pointer validity, alignment, lifetime, and aliasing requirements.

## Related Topics {#related-topics .skip}

### C / C++ Build Integration {#c-cpp-build-integration}

[`cc`][c~cc~docs]↗{{hi:cc}} helps with compiling C/C++ code within your Rust build script ([`build.rs`][book~cargo~build-script]↗{{hi:build.rs}}). This is often necessary when working with FFI or C++ bindings. See [[build_time_tooling | Build Time Tooling]].

### Utilities {#utilities}

- [`std::ffi`][c~std::ffi~docs]↗{{hi:std::ffi}} provides types for interacting with C code, like `CString`, `CStr`, `c_char`, etc.
  - Use `std::ffi::CString`, `std::ffi::CStr` for converting between Rust [strings][p~strings] and C strings.
- [`libc`][c~libc~docs]↗{{hi:libc}} provides access to the standard C library.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
