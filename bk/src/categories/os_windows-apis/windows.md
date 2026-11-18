<!-- TODO Reviewed ??-??-2026. -->

# Windows API Bindings

{{#include windows.incl.md}}

Use the standard library for portable filesystem, process, and networking work when it covers the requirement. For a Windows-only API, choose a binding by the API surface you need and keep the platform-specific code behind `#[cfg(target_os = "windows")]`.

The [`windows`][c~windows~docs]↗{{hi:windows}} crate provides typed projections over Win32, COM, and WinRT APIs. The [`windows-sys`][windows-sys~docs] crate exposes raw C-style declarations, pointers, and ABI types. Both are part of Microsoft's [`windows-rs`][c~windows~repo]↗ project. Use `windows-sys` when a thin FFI boundary is useful and you can own its safety checks; the shape of the API, rather than assumed call overhead, is the main distinction.

[`winapi`][c~winapi~docs]↗ is an older set of raw bindings. Keep it when maintaining a project that already uses it; for new application code, prefer the maintained `windows` or `windows-sys` crates.

For reusable libraries, the [windows-rs guidance](https://github.com/microsoft/windows-rs#readme) recommends focused crates for shared types and `windows-bindgen` for additional private bindings, reducing dependency weight and version conflicts. The umbrella crates remain convenient for application binaries such as these examples.

Enable only the API features used by the project. These examples need Windows and display a message box, so their tests are ignored by default and require a Windows desktop to run manually.

## Run the Examples Locally

These examples require a Windows desktop and cannot run in the browser Playground. From the repository's `bk` directory, run one command and dismiss the dialog to let it finish:

```sh
just cargo run --manifest-path crates/Cargo.toml -p os_windows_apis --example windows -- windows
just cargo run --manifest-path crates/Cargo.toml -p os_windows_apis --example windows -- winapi
```

For a standalone project, copy either snippet into `src/main.rs` and add the corresponding target-specific dependency:

```toml
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.62.2", features = ["Win32_Foundation", "Win32_UI_WindowsAndMessaging"] }
# Only needed for the legacy example:
winapi = { version = "0.3.9", features = ["winuser"] }
```

The `W` suffix in `MessageBoxW` selects UTF-16 strings. The `windows` example uses `w!` for static, NUL-terminated literals; the `winapi` example builds owned UTF-16 buffers that remain alive until the call returns. Runtime input must not contain interior NUL characters, which would truncate the text. Typed bindings still require `unsafe` for this call: the caller must satisfy the native API's pointer and lifetime contracts.

[`MessageBoxW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw) blocks until the user responds. It returns a button identifier, or zero on failure; both examples propagate the OS error immediately. Do not assume that every projected Windows function returns `Result`: check each function's documented failure convention.

## Bind to Windows APIs with `windows` {#windows}

[![windows][c~windows~docs~badge]][c~windows~docs]{{hi:Windows}}
[![windows~crates.io][c~windows~crates.io~badge]][c~windows~crates.io]
[![windows~repo][c~windows~repo~badge]][c~windows~repo]
[![windows~lib.rs][c~windows~lib.rs~badge]][c~windows~lib.rs]
[![cat~os::windows-apis][cat~os::windows-apis~badge]][cat~os::windows-apis]{{hi:Windows APIs}}

[`windows`][c~windows~docs]↗{{hi:windows}} projects Windows API metadata into typed Rust interfaces. Enable the relevant API namespace features (for example, `Win32_UI_WindowsAndMessaging`) in Cargo instead of compiling the full API surface.

```rust,ignore
{{#include ../../../crates/cats/os_windows_apis/examples/windows/windows.rs:example}}
```

## Bind to Windows APIs with `winapi` {#winapi}

{{#badges winapi external-ffi-bindings="External FFI bindings" os::windows-apis="Windows APIs" no-std="No standard library"}}

[`winapi`][c~winapi~docs]↗{{hi:winapi}} exposes raw Windows FFI declarations. Its example is included to help maintain existing code; new projects should consider `windows` or `windows-sys` first.

```rust,ignore
{{#include ../../../crates/cats/os_windows_apis/examples/windows/winapi.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Operating Systems](../os/index.md) covers portable process, filesystem, and environment APIs.
- [Windows Job Objects](../os/process_management.md#job-object) demonstrates process control with `windows-sys`.
- [Unix APIs](../os_unix-apis/index.md) covers Unix-specific system interfaces.
- [`windows-sys`][windows-sys~docs] exposes lower-level bindings for Windows APIs.
- [`windows-service`][windows-service~docs] supports Windows service applications.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
