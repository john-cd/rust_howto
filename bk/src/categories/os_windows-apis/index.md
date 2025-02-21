# Windows

[![cat-os::windows-apis][cat-os::windows-apis-badge]][cat-os::windows-apis]{{hi:Windows APIs}}

Bindings to Windows-specific APIs.

{{#include windows.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 review](https://github.com/john-cd/rust_howto/issues/950)

| Topic | Rust Crates|
| --- | --- |
| File System Operations | `std::fs`, `path`, `windows`|
| Process Management | `std::process`, `windows`|
| Networking (Sockets) | `std::net`, `tokio`, `winnock` |
| Windows API Interaction | `windows`, `winapi` |
| GUI Development | `winit`, `iced`, `egui`, `tauri`, `fltk-rs` |
| Registry Access | `winreg` |
| Services | `winsvc`, `windows`|
| COM (Component Object Model) | `com`, `windows`|
| DLLs (Dynamic Link Libraries) | `libloading`|
| Threading | `std::thread`, `crossbeam` |

</div>
