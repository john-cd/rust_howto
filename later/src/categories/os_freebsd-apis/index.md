<!-- TODO Reviewed ??-??-2026 -->

# OS: FreeBSD APIs

[![cat~os::freebsd-apis][cat~os::freebsd-apis~badge]][cat~os::freebsd-apis]

Use the standard library for portable filesystem, networking, process, and thread operations. For FreeBSD-specific behavior, prefer the existing `libc`, `nix`, or `rustix` bindings when they expose the needed interface; isolate direct FFI and unsafe calls behind platform configuration.

| FreeBSD interface | Rust approach                                                                          | Typical use                                                   |
| ----------------- | -------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| Capsicum          | Capability-mode system calls through an available binding or a small audited FFI layer | Restrict a process to explicitly granted capabilities.        |
| Jails             | Jail management bindings or FreeBSD system calls                                       | Isolate processes and services.                               |
| kqueue            | `nix`, `mio`, or a runtime abstraction                                                 | Monitor file descriptors and events.                          |
| sysctl            | `sysctl` bindings or `libc`                                                            | Read and, where supported, configure kernel and device state. |

Check target support and platform-specific behavior in the selected crate before relying on an API. See the [API Bindings](../api-bindings/index.md) and [External FFI Bindings](../external-ffi-bindings/index.md) categories for broader choices.

## Example

{{#include freebsd.incl.md}}

## Related Topics

- [[os | Operating Systems]].
- [[os_linux-apis | OS: Linux APIs]].
- [[os_macos-apis | OS: macOS APIs]].
- [[os_unix-apis | OS: Unix APIs]].
- [[development-tools_ffi | Development Tools: FFI]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
content audit: category index content removed by recent commits; review and restore

- Crates.io categories: [API Bindings][cat~api-bindings]↗ and [External FFI Bindings][cat~external-ffi-bindings]↗.

</div>
