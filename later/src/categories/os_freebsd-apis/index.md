<!-- TODO Reviewed ??-??-2026. -->

# OS: FreeBSD APIs

[![cat~os::freebsd-apis][cat~os::freebsd-apis~badge]][cat~os::freebsd-apis]

Use the standard library for portable filesystem, networking, process, and thread operations. For FreeBSD-specific behavior, prefer the existing `libc`, `nix`, or `rustix` bindings when they expose the needed interface; isolate direct FFI and unsafe calls behind platform configuration.

| FreeBSD interface | Rust approach | Typical use |
| --- | --- | --- |
| Capsicum | Capability-mode system calls through an available binding or a small audited FFI layer | Restrict a process to explicitly granted capabilities. |
| Jails | Jail management bindings or FreeBSD system calls | Isolate processes and services. |
| kqueue | `nix`, `mio`, or a runtime abstraction | Monitor file descriptors and events. |
| sysctl | `sysctl` bindings or `libc` | Read and, where supported, configure kernel and device state. |

Check target support and platform-specific behavior in the selected crate before relying on an API. See the [API Bindings](../api-bindings/index.md) and [External FFI Bindings](../external-ffi-bindings/index.md) categories for broader choices.

For shared crate selection and helper-process contracts, see [Native OS APIs and Process Boundaries](../../other/native-os/index.md).

## Example

{{#include freebsd.incl.md}}

## Related Topics

- [Operating Systems](https://john-cd.com/rust_howto/categories/os/index.html).
- [OS: Linux APIs](../os_linux-apis/index.md).
- [OS: macOS APIs](../os_macos-apis/index.md).
- [OS: Unix APIs](https://john-cd.com/rust_howto/categories/os_unix-apis/index.html).
- [Development Tools: FFI](../development-tools_ffi/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
