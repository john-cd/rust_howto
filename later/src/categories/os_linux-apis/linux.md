<!-- TODO Reviewed ??-??-2026 -->

# OS - Linux APIs

{{#include linux.incl.md}}{{hi:Linux APIs}}

The index summarizes the available Linux-specific crates. These recipes show when to use them, what the host must provide, and how to keep Linux-only code separate from portable paths.

## Use Linux system calls with `rustix`

[`rustix`][c~rustix~docs] provides safe, typed wrappers for many Linux and POSIX system calls. Gate Linux-only operations with `#[cfg(target_os = "linux")]` and keep a clear fallback for other targets. For example, `rustix::process::getpid()` reads the current process ID without a direct unsafe syscall. For filesystem metadata or event watching, use the relevant `statx` or inotify wrapper and check its return value rather than assuming the requested feature exists on every kernel.

The included `statx` example creates a temporary file, writes a known message, and queries its open descriptor with `AtFlags::EMPTY_PATH`. It requests size and birth time, then checks the returned field mask before using either value. Birth time is optional even when the call succeeds. The temporary file is closed and removed explicitly; failures propagate to the caller.

`statx` requires Linux 4.11 or later and may be denied by a host's syscall policy. A missing birth-time field is reported as unavailable; syscall errors are propagated rather than replaced by zero metadata. The example and its dependencies are gated to Linux, and other targets print a message.

From `later/crates` on Linux, run `cargo run -p os_linux_apis --example linux`. The file size is 23 bytes; the birth-time output depends on the host filesystem.

## Choose an `io_uring` runtime

[`io-uring`][c~io-uring~docs] exposes Linux's submission and completion queues directly. Choose it when the application needs low-level control over operations and buffer lifetimes. [`tokio-uring`][c~tokio-uring~docs] integrates those operations with its own Tokio-based runtime; its resources transfer buffers to the kernel for an operation and return them at completion. The current `tokio-uring` documentation requires Linux 5.10 or later. Other `io_uring` operations were added in later kernels, and seccomp or host policy can also deny ring creation, so check the exact operations and deployment environment. Use an epoll-based runtime when the required kernel or sandbox does not support them.

## Call services over D-Bus with `zbus`

[`zbus`][c~zbus~docs] provides client proxies and service interfaces for D-Bus. Use the session bus for desktop-session services and the system bus for machine services; request only the bus name and methods the application needs. A D-Bus client requires an available bus and the target service's name, object path, and interface. A service must also be installed with suitable bus policy and activation metadata.

## Notify systemd when a service is ready

[`sd-notify`][c~sd-notify~docs] sends readiness and state-change messages to systemd or a compatible service manager. Send the ready notification only after the service has completed initialization and can accept work. The notification socket is provided by the service manager, so the call has no effect when the program is launched outside a compatible managed service. See the [`sd_notify` protocol][systemd~sd-notify~manual].

## Load eBPF programs with Aya

[`aya`][c~aya~docs] loads and manages eBPF programs from a userspace Rust process; the kernel verifies a program before it can run. Program types, helpers, BTF data, and attachment points depend on the running kernel and its configuration. Loading programs generally needs elevated privileges: use root in a controlled development environment or grant only the capabilities required by the program, such as `CAP_BPF` and, for some uses, `CAP_PERFMON`. Do not make a loader setuid or run it with broader privileges than necessary. See the [Aya book][aya~book] and the Linux [capabilities manual][linux~bpf~capabilities~manual].

## Example {#linux}

```rust,editable
{{#include ../../../crates/cats/os_linux_apis/examples/linux/statx_metadata.rs:example}}
```

## Related Topics {#related-topics .skip}

- Linux system programming.
- OS-level APIs and bindings.
- Cross-platform compatibility.
- Performance and low-level Rust.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [linux: write](https://github.com/john-cd/rust_howto/issues/434)
</div>
