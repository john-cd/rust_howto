<!-- TODO Reviewed ??-??-2026. -->

# Windows APIs

[![cat~os::windows-apis][cat~os::windows-apis~badge]][cat~os::windows-apis]{{hi:Windows APIs}}

Use the Rust standard library for portable file, process, and socket operations when it meets the need. For Windows-specific APIs, choose a binding crate based on the level of abstraction and the APIs a project needs.

[`windows`][c~windows~docs]↗{{hi:windows}} provides Microsoft's projected, higher-level bindings. [`windows-sys`][windows-sys~docs]↗ exposes lower-level function declarations, structs, and constants without convenience wrappers. Both are part of the [`windows-rs`][c~windows~repo]↗ project. [`winapi`][c~winapi~docs]↗ is an older binding crate, usually relevant when maintaining an existing project.

| Binding | Use it when |
| --- | --- |
| [`windows`][c~windows~docs]↗ | You want Microsoft's projected API bindings and their higher-level helpers. |
| [`windows-sys`][windows-sys~docs]↗ | You need direct FFI declarations with minimal abstraction. |
| [`winapi`][c~winapi~docs]↗ | You are maintaining code that already uses this older binding crate. |

| API area | Coverage | Relevant crates |
| --- | --- | --- |
| File and path APIs | Prefer portable standard-library operations; use Windows bindings for APIs `std` does not expose. | [`std::fs`][c~std::fs~docs]↗, [`std::path`][c~std::path~docs]↗, [`windows`][c~windows~docs]↗ |
| Process management | Use portable process APIs for ordinary child processes and Windows bindings for platform-specific controls. | [`std::process`][c~std::process~docs]↗, [`windows`][c~windows~docs]↗ |
| Networking | Use portable sockets unless a Windows-specific API is needed. | [`std::net`][c~std::net~docs]↗, [`tokio`][c~tokio~docs]↗ |
| Windows API bindings | Select projected or low-level bindings based on the abstraction and compile-time needs. | [`windows`][c~windows~docs]↗, [`windows-sys`][windows-sys~docs]↗, [`winapi`][c~winapi~docs]↗ |
| Registry access | Read and write the Windows registry. | [`winreg`][c~winreg~docs]↗ |
| Windows services | Install, manage, and implement Windows service processes. | [`windows-service`][windows-service~docs]↗ |
| COM | Consume and implement Component Object Model interfaces. | [`com`][c~com~docs]↗, [`windows`][c~windows~docs]↗ |
| Dynamic libraries | Load shared libraries and resolve symbols at runtime. | [`libloading`][c~libloading~docs]↗ |

## Binding Chapters

{{#include windows.incl.md}}

## Related Topics {#related-topics .skip}

- [Operating Systems](../os/index.md) introduces portable OS services.
- [Windows Job Objects](../os/process_management.md#job-object) controls child-process lifetimes with `windows-sys`.
- [Unix APIs](../os_unix-apis/index.md) covers Unix-specific interfaces.
- [Windows-specific standard-library extensions](https://doc.rust-lang.org/std/os/windows/) includes Windows path, handle, and process extensions.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
