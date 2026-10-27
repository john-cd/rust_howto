<!-- TODO Reviewed ??-??-2026 -->

# External FFI Bindings

{{#include external_ffi_bindings.incl.md}}

## C Bindings {#ffi}

[`libc`][c~libc~docs]↗{{hi:libc}} provides definitions for C standard library functions and types.

Bindings crates expose an existing native API. Binding generators, such as `bindgen` and `cxx`, produce bindings instead; see [Generate FFI Bindings](../development-tools_ffi/generate_ffi_bindings.md) for those workflows. Prefer the standard library when it already covers your task.

On Unix, the following example calls [`libc::getpid`][ffi~getpid] and compares its result with `std::process::id()`. The `unsafe` block is small because this function takes no pointers, owns no resources, and has no caller preconditions on supported Unix targets.

```rust,editable
{{#include ../../../crates/cats/external_ffi_bindings/examples/external_ffi_bindings.rs:example}}
```

Other C functions can require valid pointers, matching allocation/deallocation functions, and an explicit check of their return value and `errno`. A declaration in `libc` does not enforce those contracts. Keep a native allocation's owner alive while Rust borrows it, and use `CStr` or `CString` when the API requires a null-terminated string.

## Use Safe Unix Wrappers {#unix-wrappers}

[`nix`][ffi~nix] wraps many Unix APIs with Rust types and `Result` errors. Enable its task-specific features, such as `process` or `fs`, and check which operating systems implement the operation. These wrappers reduce the amount of application-level `unsafe`; they do not make every Unix API portable to Windows.

## Call Windows APIs {#windows-bindings}

[`windows`][ffi~windows] provides generated Windows API projections, including Win32 and COM interfaces. [`windows-sys`][ffi~windows-sys] exposes lower-level declarations with less wrapping. Enable only the needed API feature families, gate Windows code with `#[cfg(windows)]`, and follow each function's handle, buffer, and threading contracts. Follow the API documentation for concrete calls.

## Separate Raw Bindings from Safe Wrappers {#sys-crates}

A crate named `library-sys` conventionally owns raw native declarations and linking; a companion `library` crate can expose RAII resource owners, validated arguments, and Rust errors. The suffix is a convention, not a safety guarantee. Inspect the wrapper's documented invariants before passing foreign pointers across its boundary.

For a crate that links a native library, Cargo's [`links` manifest key][ffi~cargo-links] identifies that native dependency, for example `links = "library"`. Cargo permits only one package with a given `links` value in a dependency graph. Its build script locates or builds the library and emits linker instructions and any metadata needed by immediate dependents. This avoids competing bindings packages linking incompatible copies; it does not supply a library or generate bindings by itself.

## Validate the Example {#validate-bindings}

From `later/crates`, run:

```bash
cargo test --locked -p external_ffi_bindings --all-targets
cargo clippy --locked -p external_ffi_bindings --all-targets -- -D warnings
```

Run the same commands on Unix to execute the `getpid` comparison. On Windows, the Unix code and its test compile out; the example prints the standard-library process ID instead.

## Related Topics {#related-topics .skip}

- [[external-ffi-bindings | External FFI Bindings]].
- [[development-tools_ffi | Development Tools: FFI]].
- [[api-bindings | API Bindings]].
- [[os | OS]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
