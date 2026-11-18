<!-- TODO Reviewed ??-??-2026. -->

# Native OS APIs and Process Boundaries

Use `std` for portable files, processes, synchronization, and paths. Reach for a
platform crate when the operation requires facilities beyond `std`; keep the native
implementation behind target-specific dependencies and `cfg` gates. A fallback that
prints "unsupported platform" verifies portability of the package, not the native API.

## Choose the Native Boundary

| Platform or task | Common crates | Recipe and support boundary |
| --- | --- | --- |
| Unix file descriptors and system calls | `rustix`, `nix`, `libc` | [Linux metadata](../../categories/os_linux-apis/linux.md) uses rustix's safe statx wrapper. Nix offers additional Unix abstractions; libc exposes raw ABI calls and requires manual safety reasoning. |
| Linux services and asynchronous I/O | `zbus`, `io-uring`, `tokio-uring` | [Linux APIs](../../categories/os_linux-apis/linux.md) explains the service/runtime boundaries; these integrations are not tested by statx. |
| Windows APIs | `windows`, `windows-sys` | The [published Windows category](https://github.com/john-cd/rust_howto/blob/86dcf0197f8d6810877a6021ffedcf84aa9230c8/bk/src/categories/os_windows-apis/index.md) owns Windows recipes; no duplicate is staged here. |
| macOS frameworks | `objc2`, `objc2-foundation`, `objc2-app-kit`, `core-foundation`, `security-framework` | [macOS APIs](../../categories/os_macos-apis/macos.md) introduces framework access. AppKit's main-thread requirements and Keychain entitlements are separate from Foundation queries. |
| FreeBSD kernel state and events | `sysctl`, `nix`, `libc` | [FreeBSD APIs](../../categories/os_freebsd-apis/freebsd.md) covers a sysctl query; kqueue, jails, and Capsicum need their own runtime checks. |
| Application change notifications | `notify`, `mio`, `polling` | Start with the [published file watcher](https://github.com/john-cd/rust_howto/blob/db7033a53e8c5f6cfdc66ad0a9c89105319ebe30/bk/src/categories/filesystem/file_watching.md) for path changes. Use an I/O readiness API when waiting for file descriptors or sockets to become ready. |

Prefer an owned descriptor/handle wrapper over a raw integer or pointer. Document
which side closes it and what happens after partial initialization. Match error codes
to the operation; permission denial, unsupported kernel features, and a missing path
are different failures. Kernel version, filesystem, privileges, sandbox policy, and
framework availability can all affect a native call.

## Launch a Trusted Helper with an Explicit Environment

This portable example launches the current executable as a fixed helper. It supplies
an absolute executable, a temporary working directory, separate arguments, null stdin,
and a minimal environment; it checks both successful output and a requested failure.
It changes neither the parent's current directory nor its environment.

```rust,ignore,noplayground
{{#include ../../../crates/cats/os_linux_apis/examples/process_environment.rs:example}}
```

```sh
just cargo run --manifest-path later/crates/Cargo.toml --locked -p os_linux_apis --example process_environment
just cargo test --manifest-path later/crates/Cargo.toml --locked -p os_linux_apis --example process_environment
```

The package contains the Linux statx recipe too, but this helper example is portable.
Its child produces a tiny fixed amount of output and terminates immediately. `output()`
buffers stdout/stderr and waits: arbitrary tools need output limits, deadlines,
cancellation, and process-tree cleanup. Do not use this fixture as an untrusted-code
sandbox. Clearing environment variables does not restrict filesystem or network access.
See [`Command`](https://doc.rust-lang.org/std/process/struct.Command.html) for inheritance
and argument handling. An absolute program path avoids relying on a cleared PATH.

On Windows the example retains `SystemRoot` when present. Real helpers may require
additional explicit runtime or locale settings; add only the settings their documented
contract needs. Passing arguments separately avoids shell interpretation, but the child
program can still interpret its own options: use an end-of-options marker when supported.

## Verify on the Actual Target

Run Linux examples on Linux, macOS examples on macOS, and FreeBSD examples on FreeBSD.
Record the executed native test step and final source revision. Cross-compilation and
host fallback execution are useful checks with narrower claims. Keep privileged or
state-changing examples isolated, state their cleanup steps, and avoid requiring
administrator/root access for introductory read-only recipes.

## Related Topics

- [Linux APIs](../../categories/os_linux-apis/index.md).
- [macOS APIs](../../categories/os_macos-apis/index.md).
- [FreeBSD APIs](../../categories/os_freebsd-apis/index.md).
- [Cross-platform Development](../cross-platform/index.md).
