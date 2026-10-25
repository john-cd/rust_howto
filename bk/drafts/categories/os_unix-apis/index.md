<!-- TODO Reviewed ??-??-2026 -->

# OS - Unix APIs

[![cat~os::unix-apis][cat~os::unix-apis~badge]][cat~os::unix-apis]{{hi:Operating systems::Unix APIs}}

Rust programs can use portable standard-library APIs, safe wrappers for operating-system interfaces, or low-level C bindings. Start with `std` when it meets the requirement; when using a Unix-specific API, check target support and prefer a safe wrapper such as [`rustix`][c~rustix~docs]↗{{hi:rustix}} or [`nix`][c~nix~docs]↗{{hi:nix}} where available.

| Area                | Portable starting point                                           | Unix-specific options                                                                                        |
| ------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| Files and paths     | [`std::fs`][c~std::fs~docs]↗ and [`std::path`][c~std::path~docs]↗ | `rustix` or `nix` for file descriptors and OS-specific operations                                            |
| Processes and pipes | [`std::process`][c~std::process~docs]↗                            | `nix` for Unix process and descriptor APIs                                                                   |
| Sockets             | [`std::net`][c~std::net~docs]↗                                    | [`socket2`][c~socket2~docs]↗ for lower-level socket options                                                  |
| Terminal control    | `std::io`                                                         | [`crossterm`][c~crossterm~docs]↗ for cross-platform terminal control; `termion` for Unix terminal interfaces |
| Time                | [`std::time`][c~std::time~docs]↗                                  | Use an OS interface only when a system clock or timer API is required                                        |
| Users and groups    | No portable standard-library API                                  | `nix` where supported; use `libc` only when a safe wrapper does not expose the needed operation              |
| Signals             | No portable standard-library API                                  | [`signal-hook`][c~signal-hook~docs]↗ or `nix`, according to the required signal behavior                     |

Raw [`libc`][c~libc~docs]↗ calls expose platform-specific details and unsafe contracts. Keep them behind a small interface, check the target-specific documentation, and propagate OS errors instead of assuming an operation is available on every Unix-like system.

## Unix API Examples

The [[unix | Unix APIs]] chapter compares [`rustix`][c~rustix~docs]↗ and [`nix`][c~nix~docs]↗ and includes working examples.

{{#include unix.incl.md}}

## Related Topics {#related-topics .skip}

- [[os | Operating System]].
- [[linux | Linux APIs]].
- [[macos | macOS APIs]].
- [[windows | Windows APIs]].
- [[low_level_system_calls | Low-level System Calls]].
- [[generate_ffi_bindings | FFI Bindings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
