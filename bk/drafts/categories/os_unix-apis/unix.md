<!-- TODO Reviewed ??-??-2026 -->

# Unix APIs

{{#include unix.incl.md}}{{hi:Operating systems::Unix APIs}}

## Bind to Unix-specific APIs with `rustix` {#rustix}

[![rustix][c~rustix~docs~badge]][c~rustix~docs]{{hi:rustix}}
[![rustix~crates.io][c~rustix~crates.io~badge]][c~rustix~crates.io]
[![rustix~repo][c~rustix~repo~badge]][c~rustix~repo]
[![rustix~lib.rs][c~rustix~lib.rs~badge]][c~rustix~lib.rs]
[![cat~external-ffi-bindings][cat~external-ffi-bindings~badge]][cat~external-ffi-bindings]{{hi:External FFI bindings}}
[![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}}
[![cat~os::unix-apis][cat~os::unix-apis~badge]][cat~os::unix-apis]{{hi:Unix APIs}}

Unix-like systems expose APIs through several overlapping layers: portable Rust standard-library APIs, POSIX interfaces, and operating-system-specific extensions. Prefer `std` for portable file, process, and networking work. When an OS interface is required, use a maintained safe wrapper such as `rustix` or `nix` where it covers the operation. These wrappers use Rust types and return errors as `Result`, reducing pointer and file-descriptor handling in application code.

Check the crate's target support and feature flags because Unix does not mean every API is available on every Unix target. Linux-only facilities such as epoll or particular flags need target-specific code. Keep platform-specific calls behind a small module and provide a portable fallback when the application requires one.

`rustix` exposes syscall-like interfaces with safe Rust types such as slices, `Result`s, and borrowed file descriptors. It also provides [`bitflags`][c~bitflags~docs]↗{{hi:bitflags}}-based flag types instead of untyped integer flags.

```rust,editable
{{#include ../../../crates/cats/os_unix_apis/examples/unix/rustix.rs:example}}
```

## Bind to Unix-specific APIs with `nix` {#nix}

[![nix][c~nix~docs~badge]][c~nix~docs]{{hi:nix}}
[![nix~crates.io][c~nix~crates.io~badge]][c~nix~crates.io]
[![nix~repo][c~nix~repo~badge]][c~nix~repo]
[![nix~lib.rs][c~nix~lib.rs~badge]][c~nix~lib.rs]
[![cat~os::unix-apis][cat~os::unix-apis~badge]][cat~os::unix-apis]{{hi:Unix APIs}}

[`nix`][c~nix~docs]↗{{hi:nix}} provides bindings to the various *nix system [functions][p~functions] (Unix, Linux, MacOS, etc.).

Use `nix` when its API and enabled features match the required interface. Its API surface and supported targets differ from `rustix`; review the crate documentation and target-specific behavior before choosing between them. Avoid raw syscalls unless neither `std` nor a maintained wrapper exposes the required operation.

```rust,editable
{{#include ../../../crates/cats/os_unix_apis/examples/unix/nix.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[os | Operating System]] introduces portable operating-system services.
- [[low_level_system_calls | Low-level system calls]] compares standard-library, wrapper, and C-library interfaces.
- [[windows | Windows APIs]] covers Windows-specific bindings.
- [[generate_ffi_bindings | FFI Bindings]] covers bindings to foreign libraries.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
