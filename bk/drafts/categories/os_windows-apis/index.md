# Windows

[![cat~os::windows-apis][cat~os::windows-apis~badge]][cat~os::windows-apis]{{hi:Windows APIs}}

Bindings to Windows-specific APIs.

| Topic | Rust Crates|
| --- | --- |
| File System Operations | Use the standard `std::fs`, `path`, but also the [`windows`][c~windows~docs]⮳{{hi:windows}} crate. |
| Process Management | Use `std::process`, [`windows`][c~windows~docs]⮳{{hi:windows}}. |
| Networking (Sockets) | Use `std::net`, [`tokio`][c~tokio~docs]⮳{{hi:tokio}} as usual. |
| Windows API Interaction | [`windows`][c~windows~docs]⮳{{hi:windows}}, [`winapi`][c~winapi~docs]⮳{{hi:winapi}} |
| Registry Access | [`winreg`][c~winreg~docs]⮳{{hi:winreg}} |
| Services | [`windows`][c~windows~docs]⮳{{hi:windows}} |
| COM (Component Object Model) | `com`, [`windows`][c~windows~docs]⮳{{hi:windows}} |
| DLLs (Dynamic Link Libraries) | [`libloading`][c~libloading~docs]⮳{{hi:libloading}} |

## Code Examples

{{#include windows.incl.md}}

## Related Topics

| Topic | Rust Crates|
| --- | --- |
| GUI Development | [`winit`][c~winit~docs]⮳{{hi:winit}}, [`iced`][c~iced~docs]⮳{{hi:iced}}, [`egui`][c~egui~docs]⮳{{hi:egui}}, [`tauri`][c~tauri~docs]⮳{{hi:tauri}}, [`fltk-rs`][c~fltk~docs]⮳{{hi:fltk-rs}} |
| Threading | `std::thread`, [`crossbeam`][c~crossbeam~docs]⮳{{hi:crossbeam}} |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review in depth / write](https://github.com/john-cd/rust_howto/issues/950)
</div>
