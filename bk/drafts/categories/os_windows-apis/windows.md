# Windows API Bindings

{{#include windows.incl.md}}

## Bind to Windows APIs with `windows` {#windows}

[![windows][c~windows~docs~badge]][c~windows~docs]{{hi:Windows}}
[![windows~crates.io][c~windows~crates.io~badge]][c~windows~crates.io]
[![windows~repo][c~windows~repo~badge]][c~windows~repo]
[![windows~lib.rs][c~windows~lib.rs~badge]][c~windows~lib.rs]
[![cat~os::windows-apis][cat~os::windows-apis~badge]][cat~os::windows-apis]{{hi:Windows APIs}}

[`windows`][c~windows~docs]↗{{hi:windows}} is the official Microsoft-provided crate for interacting with Windows APIs.

```rust,editable
{{#include ../../../crates/cats/os_windows_apis/examples/windows/windows.rs:example}}
```

## Bind to Windows APIs with `winapi` {#winapi}

[![winapi][c~winapi~docs~badge]][c~winapi~docs]{{hi:winapi}}
[![winapi~crates.io][c~winapi~crates.io~badge]][c~winapi~crates.io]
[![winapi~repo][c~winapi~repo~badge]][c~winapi~repo]
[![winapi~lib.rs][c~winapi~lib.rs~badge]][c~winapi~lib.rs]
[![cat~external-ffi-bindings][cat~external-ffi-bindings~badge]][cat~external-ffi-bindings]{{hi:External FFI bindings}}
[![cat~os::windows-apis][cat~os::windows-apis~badge]][cat~os::windows-apis]{{hi:Windows APIs}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`winapi`][c~winapi~docs]↗{{hi:winapi}} provides older binding to the Windows APIs. Unofficial, but more complete than [`windows-rs`][c~windows~docs]↗{{hi:windows-rs}}.

```rust,editable
{{#include ../../../crates/cats/os_windows_apis/examples/windows/winapi.rs:example}}
```

## See Also {#see-also .skip}

Native Windows [GUI][p~gui]: [![native-windows-gui~repo][c~native-windows-gui~repo~badge]][c~native-windows-gui~repo]{{hi:native-windows-gui}}

[`native-windows-gui`][c~native-windows-gui~docs]↗{{hi:native-windows-gui}}

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[windows: write; add examples for "windows" and `winapi`](https://github.com/john-cd/rust_howto/issues/438)
</div>
