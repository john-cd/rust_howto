<!-- TODO Reviewed ??-??-2026 -->

# OS: macOS APIs

Use macOS-specific frameworks only when portable Rust APIs do not cover the required behavior. Prefer maintained safe bindings, keep platform code behind `cfg(target_os = "macos")`, and check the framework's deployment target.

## Frameworks and Tasks

| Framework or task | Rust bindings | Common use |
|---|---|---|
| AppKit | [`objc2-app-kit`][c~objc2-app-kit~docs]↗{{hi:objc2-app-kit}} | Native windows, application lifecycle, menus, and controls. |
| Swift interoperability | [swift-bridge][swift-bridge~repo]↗ | Generate bindings when a Swift application calls Rust. |
| Foundation | [`objc2-foundation`][c~objc2-foundation~docs]↗{{hi:objc2-foundation}} | Foundation objects and services. |
| Core Foundation | [`objc2-core-foundation`][c~objc2-core-foundation~docs]↗{{hi:objc2-core-foundation}} | Lower-level Core Foundation APIs. |
| Security and Keychain | [`objc2-security`][c~objc2-security~docs]↗{{hi:objc2-security}} | Security framework and credential storage. |
| File events | FSEvents bindings | Watch filesystem changes using the native event service. |
| App packaging | [`cargo-bundle`][c~cargo-bundle~docs]↗{{hi:cargo-bundle}} | Package a Rust application bundle for distribution. |

The older `cocoa`, `core-foundation`, and `objc` crates are legacy interfaces. For new Objective-C framework code, review the `objc2-*` crates and their framework features. The older wrapper is retained in the existing example for comparison; see [[objc | Objective-C]].

A short AppKit entry point uses the current main-thread marker API:

```rust
use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;

let marker = MainThreadMarker::new().expect("called on the main thread");
let application = NSApplication::sharedApplication(marker);
```

This API is macOS-only; initialize AppKit on the main thread and enable the corresponding `objc2-app-kit` feature in `Cargo.toml`. See the related [Objective-C bindings issue #1075](https://github.com/john-cd/rust_howto/issues/1075).

## Example

{{#include macos.incl.md}}

## Related Topics

- [[api-bindings | API Bindings]] and [[external-ffi-bindings | External FFI Bindings]].
- [[development-tools_ffi | Development Tools: FFI]].
- [[objc | Objective-C]].
- [[os | Operating Systems]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[content audit: category index content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1903)
</div>
