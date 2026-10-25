<!-- TODO Reviewed ??-??-2026 -->

# Low-level Interaction with the Operating System

{{#include low_level_system_calls.incl.md}}

When the standard library does not expose an operating-system facility, choose a maintained binding at the narrowest useful level. A function from `libc` is a C-library interface; it is not necessarily a direct kernel system call. Direct `syscall` entry points and their numbers vary by operating system and architecture, so prefer a safe wrapper when one covers the operation.

## Choose an Interface

| Need                                               | Starting point                                     | Trade-off                                                                                                  |
| -------------------------------------------------- | -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Portable files, processes, environment, or sockets | [`std`][c~std~docs]↗                               | Prefer this for ordinary application code; it hides platform-specific details.                             |
| Unix system APIs with safe Rust types              | [`rustix`][c~rustix~docs]↗ or [`nix`][c~nix~docs]↗ | Use a wrapper that returns `Result` and owns or borrows file descriptors safely.                           |
| An API that a wrapper does not expose              | [`libc`][c~libc~docs]↗                             | C-compatible types and pointers require a small, reviewed `unsafe` boundary.                               |
| A raw kernel call with no library binding          | Platform-specific syscall interface                | Check the target OS and architecture documentation; do not assume syscall numbers or layouts are portable. |

Check the documented safety requirements, pointer lifetimes, ownership rules, error convention, and platform availability before calling a low-level function. Keep `unsafe` blocks small and provide a safe Rust-facing wrapper where the operation is reused.

## Call `libc` Functions {#call-libc}

[![libc][c~libc~docs~badge]][c~libc~docs]{{hi:libc}}
[![libc~crates.io][c~libc~crates.io~badge]][c~libc~crates.io]
[![libc~repo][c~libc~repo~badge]][c~libc~repo]
[![libc~lib.rs][c~libc~lib.rs~badge]][c~libc~lib.rs]
[![cat~external-ffi-bindings][cat~external-ffi-bindings~badge]][cat~external-ffi-bindings]{{hi:External FFI bindings}}
[![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

The [`libc`][c~libc~docs]↗{{hi:libc}} crate exposes platform C-library functions and types to Rust. The example converts C strings explicitly, checks null pointers, and calls a few C functions. Use the Rust standard library for ordinary output and environment access; these calls are included to show the FFI boundary.

```rust,editable
{{#include ../../../crates/cats/os/examples/low_level_system_calls/libc.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[os | Operating System]] starts with portable operating-system services.
- [[unix | Unix APIs]] covers safe wrappers for common Unix interfaces.
- [[windows | Windows APIs]] covers Windows-specific bindings.
- [[generate_ffi_bindings | FFI Bindings]] covers calling C and C++ code.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
