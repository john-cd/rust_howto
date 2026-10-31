<!-- TODO Reviewed ??-??-2026 -->

# Unix APIs

{{#include unix.incl.md}}{{hi:Operating systems::Unix APIs}}

## Bind to Unix-specific APIs with `rustix` {#rustix}

{{#badges rustix external-ffi-bindings="External FFI bindings" os="Operating systems" os::unix-apis="Unix APIs"}}

Unix-like systems expose APIs through several overlapping layers: portable Rust standard-library APIs, POSIX interfaces, and operating-system-specific extensions. Prefer `std` for portable file, process, and networking work. When an OS interface is required, use a maintained safe wrapper such as `rustix` or `nix` where it covers the operation. These wrappers use Rust types and return errors as `Result`, reducing pointer and file-descriptor handling in application code.

Check the crate's target support and feature flags because Unix does not mean every API is available on every Unix target. Linux-only facilities such as epoll or particular flags need target-specific code. Keep platform-specific calls behind a small module and provide a portable fallback when the application requires one.

`rustix` exposes syscall-like interfaces with safe Rust types such as slices, `Result`s, and borrowed file descriptors. It also provides [`bitflags`][c~bitflags~docs]↗{{hi:bitflags}}-based flag types instead of untyped integer flags.

```rust,editable
{{#include ../../../crates/cats/os_unix_apis/examples/unix/rustix.rs:example}}
```

## Bind to Unix-specific APIs with `nix` {#nix}

{{#badges nix os::unix-apis="Unix APIs"}}

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
