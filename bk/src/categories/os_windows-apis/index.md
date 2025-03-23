# Windows

[![cat-os::windows-apis][cat-os::windows-apis-badge]][cat-os::windows-apis]{{hi:Windows APIs}}

Bindings to Windows-specific APIs.

| Topic | Rust Crates|
| --- | --- |
| File System Operations | Use the standard `std::fs`, `path`, but also the [`windows`][c-windows]⮳{{hi:windows}} crate. |
| Process Management | Use `std::process`, [`windows`][c-windows]⮳{{hi:windows}}. |
| Networking (Sockets) | Use `std::net`, [`tokio`][c-tokio]⮳{{hi:tokio}} as usual. |
| Windows API Interaction | [`windows`][c-windows]⮳{{hi:windows}}, [`winapi`][c-winapi]⮳{{hi:winapi}} |
| Registry Access | [`winreg`][c-winreg]⮳{{hi:winreg}} |
| Services | [`windows`][c-windows]⮳{{hi:windows}} |
| COM (Component Object Model) | `com`, [`windows`][c-windows]⮳{{hi:windows}} |
| DLLs (Dynamic Link Libraries) | [`libloading`][c-libloading]⮳{{hi:libloading}} |

## Code Examples

{{#include windows.incl.md}}

## Related Topics

| Topic | Rust Crates|
| --- | --- |
| GUI Development | [`winit`][c-winit]⮳{{hi:winit}}, [`iced`][c-iced]⮳{{hi:iced}}, [`egui`][c-egui]⮳{{hi:egui}}, [`tauri`][c-tauri]⮳{{hi:tauri}}, [`fltk-rs`][c-fltk]⮳{{hi:fltk-rs}} |
| Threading | `std::thread`, [`crossbeam`][c-crossbeam]⮳{{hi:crossbeam}} |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review in depth / write](https://github.com/john-cd/rust_howto/issues/950)
</div>
