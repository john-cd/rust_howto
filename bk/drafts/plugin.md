# Plugin Architecture

## Create a Plugin System {#plugins}

[![std][c~std~docs~badge]][c~std~docs]

```rust,editable
{{#include ../../crates/standard_library/examples/any/plugin.rs:example}}
```

For a true plugin architecture where plugins are compiled as separate shared libraries (`.so` on Linux, `.dll` on Windows, `.dylib` on macOS) and loaded at runtime, you would typically:

- Use the [`libloading`][c~libloading~crates.io]↗{{hi:libloading}} crate, which provides safe FFI (Foreign Function Interface) wrappers to dynamically load shared libraries and resolve symbols (functions).
- Define a C-compatible ABI: Because Rust's internal ABI is not stable across different compiler versions or even minor changes, you should define your plugin interface using [`#[repr(C)]`][book~rust-reference~c-representation]↗{{hi:repr}} structs and [`extern "C"`][keyword~extern]↗{{hi:extern "C"}} functions.
- Define an entry point: Each plugin [`.so`][.so-files]↗{{hi:.so}}/[`.dll`][.dll-files]↗{{hi:.dll}} would export a specific [`extern "C"`][keyword~extern]↗{{hi:extern "C"}} function that the host calls to get a `Box<dyn Plugin>`.
- Version Management: Even with [`extern "C"`][keyword~extern]↗{{hi:extern "C"}}, you need robust versioning for your common plugin interface crate to prevent issues if host and plugins are compiled with different versions of the interface. Crates like [`abi_stable`][c~abi_stable~docs]↗{{hi:abi_stable}} can help with this by providing more robust ABI compatibility checks.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include refs/link-refs.md}}

<div class="hidden">
TODO write
</div>
