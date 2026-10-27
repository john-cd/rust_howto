<!-- TODO Reviewed ??-??-2026 -->

# OS - FreeBSD APIs

{{#include freebsd.incl.md}}{{hi:Operating systems::FreeBSD APIs}}

Use the standard library for portable filesystem, process, and networking work. For POSIX interfaces, [`libc`][c~libc~docs], [`nix`][c~nix~docs], and [`rustix`][c~rustix~docs] provide APIs that also work on FreeBSD. Reach for FreeBSD-specific interfaces when an application needs kernel state, capability-mode confinement, jails, or kernel event notification.

## Read kernel state with `sysctl`

The `sysctl` interface exposes named kernel and device values. The [`sysctl` crate][c~sysctl~docs] provides a Rust wrapper; for example, `kern.ostype` reports the operating-system type. Prefer read-only queries in application code. Writing a tunable can require privilege and can change host-wide behavior, so follow the [FreeBSD `sysctl(2)` manual][freebsd~sysctl~manual] and document the required permissions.

This small sketch reads a value and propagates errors:

```rust,ignore
fn read_os_type() -> Result<String, Box<dyn std::error::Error>> {
    use sysctl::Sysctl;

    let mut os_type = sysctl::Ctl::new("kern.ostype")?;
    Ok(os_type.value_string()?)
}
```

The current included example uses `nix` to inspect `uname` and process IDs. A dedicated `sysctl` example and its FreeBSD-only check are tracked separately in [#817](https://github.com/john-cd/rust_howto/issues/817).

## Restrict a process with Capsicum

FreeBSD's Capsicum capability mode restricts access to global operating-system namespaces. Open the files and sockets the program needs before entering the mode, then pass those already-open descriptors to code that should run with fewer ambient privileges. Entering capability mode is irreversible for the process and inherited by child processes. Limit each descriptor's rights as well when the program only needs a subset of its operations. See the [`capsicum` crate][c~capsicum~docs] and the [FreeBSD Capsicum manual][freebsd~capsicum~manual].

Capability mode is a confinement building block, not a complete sandbox by itself: account for every inherited descriptor, system call, and child process in the design.

## Manage a jail

FreeBSD jails provide operating-system-level isolation for process, filesystem, user, and network views while sharing the host kernel. Host administrators create and configure jails with the system's jail tools; the [`jail` crate][c~jail~docs] exposes a Rust interface for managing jail operations. Creating or reconfiguring a jail requires host privileges, so keep that work in a narrowly scoped administrative tool. A jail is not a virtual machine; see the [FreeBSD Handbook's jails chapter][freebsd~jails~handbook] for setup and isolation details.

## Watch events with `kqueue`

[`kqueue`][c~kqueue~docs] registers filters for events such as readable file descriptors, filesystem changes, or process state. Use its `kevent` interface directly for FreeBSD-specific event loops, or choose a portable abstraction such as [`mio`][c~mio~docs] when one code path should also support other operating systems. See the [FreeBSD `kqueue` manual][freebsd~kqueue~manual] and check the selected crate's target support before using it.

## Build and test platform-specific code

Code that calls FreeBSD-only APIs should live behind `#[cfg(target_os = "freebsd")]`, with its dependencies declared under a matching Cargo target section. Other targets can compile a small fallback, but that does not exercise the FreeBSD path. Gate FreeBSD-specific assertions to a FreeBSD runner; the current workspace's included `nix` example has a non-FreeBSD fallback, while its planned `sysctl` test remains part of #817.

For shared POSIX operations, check the target support of `libc`, `nix`, or `rustix` before adding direct FFI. Keep unsafe calls and platform-specific behavior behind a narrow boundary.

## Example {#freebsd}

```rust,editable
{{#include ../../../crates/cats/os_freebsd_apis/examples/freebsd/freebsd.rs:example}}
```

## Related Topics {#related-topics .skip}

- UNIX-like system APIs.
- Platform-specific Rust bindings.
- System programming and compatibility.
- Application portability.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [freebsd: write](https://github.com/john-cd/rust_howto/issues/433)
</div>
