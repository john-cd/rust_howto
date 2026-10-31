<!-- TODO Reviewed ??-??-2026 -->

# Windows API Bindings

{{#include windows.incl.md}}

Use the standard library for portable filesystem, process, and networking work when it covers the requirement. For a Windows-only API, choose a binding by the API surface you need and keep the platform-specific code behind `#[cfg(target_os = "windows")]`.

The [`windows`][c~windows~docs]↗{{hi:windows}} crate provides typed projections over Win32, COM, and WinRT APIs. The [`windows-sys`][windows-sys~docs] crate exposes raw C-style declarations, pointers, and ABI types. Both are part of Microsoft's [`windows-rs`][c~windows~repo]↗ project. Use `windows-sys` when a thin FFI boundary is useful and you can own its safety checks; the shape of the API, rather than assumed call overhead, is the main distinction.

[`winapi`][c~winapi~docs]↗ is an older set of raw bindings. Keep it when maintaining a project that already uses it; for new application code, prefer the maintained `windows` or `windows-sys` crates.

Enable only the API features used by the project. These examples need Windows and display a message box, so their tests are ignored by default and require a Windows desktop to run manually.

## Bind to Windows APIs with `windows` {#windows}

[![windows][c~windows~docs~badge]][c~windows~docs]{{hi:Windows}}
[![windows~crates.io][c~windows~crates.io~badge]][c~windows~crates.io]
[![windows~repo][c~windows~repo~badge]][c~windows~repo]
[![windows~lib.rs][c~windows~lib.rs~badge]][c~windows~lib.rs]
[![cat~os::windows-apis][cat~os::windows-apis~badge]][cat~os::windows-apis]{{hi:Windows APIs}}

[`windows`][c~windows~docs]↗{{hi:windows}} projects Windows API metadata into typed Rust interfaces. Enable the relevant `Win32_*` or `WinRT_*` features in Cargo instead of compiling the full API surface.

```rust,editable
{{#include ../../../crates/cats/os_windows_apis/examples/windows/windows.rs:example}}
```

## Bind to Windows APIs with `winapi` {#winapi}

{{#badges winapi external-ffi-bindings="External FFI bindings" os::windows-apis="Windows APIs" no-std="No standard library"}}

[`winapi`][c~winapi~docs]↗{{hi:winapi}} exposes raw Windows FFI declarations. Its example is included to help maintain existing code; new projects should consider `windows` or `windows-sys` first.

```rust,editable
{{#include ../../../crates/cats/os_windows_apis/examples/windows/winapi.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[os | Operating System]] covers portable process, filesystem, and environment APIs.
- [[gui | GUI Development]] covers cross-platform windowing and user interfaces.
- [[unix | Unix APIs]] covers Unix-specific system interfaces.
- [`windows-sys`][windows-sys~docs] exposes lower-level bindings for Windows APIs.
- [`windows-service`][windows-service~docs] supports Windows service applications.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
