<!-- TODO Reviewed ??-??-2026. -->

# OS: Linux APIs

[![cat~os::linux-apis][cat~os::linux-apis~badge]][cat~os::linux-apis]

Linux-specific interfaces are useful when a program needs kernel or service features beyond portable `std`. Prefer safe bindings and isolate platform-specific code behind `cfg(target_os = "linux")` so other targets retain a clear fallback.

| Task | Crates and interfaces | Notes |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| System calls | [`rustix`][c~rustix~docs]↗{{hi:rustix}}, [`nix`][c~nix~docs]↗{{hi:nix}}, `std::os::linux` | `rustix` and `nix` offer typed wrappers; use libc bindings only when a wrapper is unavailable. |
| Asynchronous I/O | [`io-uring`][c~io-uring~docs]↗{{hi:io-uring}}, [`tokio-uring`][c~tokio-uring~docs]↗{{hi:tokio-uring}}, [`glommio`][c~glommio~docs]↗{{hi:glommio}} | Select a runtime that matches the application's completion and cancellation model. |
| eBPF | [`aya`][c~aya~docs]↗{{hi:aya}} | Load and manage eBPF programs from Rust. |
| Desktop services | [`zbus`][c~zbus~docs]↗{{hi:zbus}} for D-Bus; systemd notification and journal bindings | Check the service and systemd versions you need to support. |
| File and device events | [`notify`][c~notify~docs]↗{{hi:notify}}; inotify; udev and evdev bindings | `notify` offers a cross-platform watcher; use the Linux interfaces for kernel-specific details. |
| Isolation and resource control | Namespaces and cgroups; see [Virtualization](../virtualization/index.md) | These interfaces require careful privilege and lifecycle handling; see the [Virtualization](../virtualization/index.md) work tracked by [issue #968](https://github.com/john-cd/rust_howto/issues/968). |

Rust's FFI can call Linux interfaces and libraries that are not exposed by `std` or a safe binding. Prefer an existing safe wrapper where possible, and keep direct `unsafe` calls small and audited.

Kernel device drivers traditionally use C APIs. The [Rust for Linux project][rust-for-linux~repo]↗ develops Rust support for Linux kernel code, including drivers, with support depending on the kernel version and subsystem. User-space programs can also communicate with device files through standard file I/O or suitable crates.

For shared crate selection and helper-process contracts, see [Native OS APIs and Process Boundaries](../../other/native-os/index.md).

## Example

{{#include linux.incl.md}}

## Related Topics

- [API Bindings](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/api-bindings/index.md) and [External FFI Bindings](../external-ffi-bindings/index.md).
- [Operating Systems](../os/index.md).
- [Virtualization](../virtualization/index.md).
- [Linux](linux.md).

For an eBPF learning path, see [Building eBPF Programs with Aya][aya~book]↗. Check the documented kernel and privilege requirements before running a program.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
