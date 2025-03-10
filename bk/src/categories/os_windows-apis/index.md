# Windows

[![cat-os::windows-apis][cat-os::windows-apis-badge]][cat-os::windows-apis]{{hi:Windows APIs}}

Bindings to Windows-specific APIs.

{{#include windows.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/950)

| Topic | Rust Crates|
| --- | --- |
| File System Operations | `std::fs`, `path`, [`windows`][c-windows]⮳{{hi:windows}} |
| Process Management | `std::process`, [`windows`][c-windows]⮳{{hi:windows}} |
| Networking (Sockets) | `std::net`, [`tokio`][c-tokio]⮳{{hi:tokio}} |
| Windows API Interaction | [`windows`][c-windows]⮳{{hi:windows}}, [`winapi`][c-winapi]⮳{{hi:winapi}} |
| GUI Development | [`winit`][c-winit]⮳{{hi:winit}}, [`iced`][c-iced]⮳{{hi:iced}}, [`egui`][c-egui]⮳{{hi:egui}}, [`tauri`][c-tauri]⮳{{hi:tauri}}, [`fltk-rs`][c-fltk]⮳{{hi:fltk-rs}} |
| Registry Access | [`winreg`][c-winreg]⮳{{hi:winreg}} |
| Services | [`windows`][c-windows]⮳{{hi:windows}} |
| COM (Component Object Model) | `com`, [`windows`][c-windows]⮳{{hi:windows}} |
| DLLs (Dynamic Link Libraries) | [`libloading`][c-libloading]⮳{{hi:libloading}} |
| Threading | `std::thread`, [`crossbeam`][c-crossbeam]⮳{{hi:crossbeam}} |

</div>
