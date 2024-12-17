# Windows API bindings

{{#include windows.incl.md}}

## Bind to Windows APIs with `windows` {#windows}

[![windows][c-windows-badge]][c-windows]{{hi:Windows}}
[![windows-crates.io][c-windows-crates.io-badge]][c-windows-crates.io]
[![windows-github][c-windows-github-badge]][c-windows-github]
[![windows-lib.rs][c-windows-lib.rs-badge]][c-windows-lib.rs]
[![cat-os::windows-apis][cat-os::windows-apis-badge]][cat-os::windows-apis]{{hi:Windows APIs}}

The official Microsoft-provided crate for interacting with windows APIs

```rust,editable
{{#include ../../../deps/tests/categories/os_windows_apis/windows.rs:example}}
```

## Bind to Windows APIs with `winapi` {#winapi}

[![winapi][c-winapi-badge]][c-winapi]{{hi:winapi}}
[![winapi-crates.io][c-winapi-crates.io-badge]][c-winapi-crates.io]
[![winapi-github][c-winapi-github-badge]][c-winapi-github]
[![winapi-lib.rs][c-winapi-lib.rs-badge]][c-winapi-lib.rs]
[![cat-external-ffi-bindings][cat-external-ffi-bindings-badge]][cat-external-ffi-bindings]{{hi:External FFI bindings}}
[![cat-os::windows-apis][cat-os::windows-apis-badge]][cat-os::windows-apis]{{hi:Windows APIs}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Older binding to the windows APIs. Unofficial, but more complete than windows-rs

```rust,editable
{{#include ../../../deps/tests/categories/os_windows_apis/winapi.rs:example}}
```

## See also

Native Windows GUI: [![native_windows_gui-github][c-native_windows_gui-github-badge]][c-native_windows_gui-github]{{hi:native-windows-gui}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[windows: write; add examples for "windows" and winapi (P2)](https://github.com/john-cd/rust_howto/issues/438)

</div>
