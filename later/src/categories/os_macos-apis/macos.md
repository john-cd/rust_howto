<!-- TODO Reviewed ??-??-2026 -->

# OS - macOS APIs

{{#include macos.incl.md}}{{hi:Operating systems::macOS APIs}}

Use portable Rust APIs when they cover the task. For native frameworks, prefer the current `objc2-*` bindings over the older `cocoa`, `core-foundation`, and `objc` wrappers. Framework APIs and their dependencies must be gated for `target_os = "macos"`.

## Call Foundation and AppKit

Use [`objc2-foundation`][c~objc2-foundation~docs] for Foundation objects and [`objc2-app-kit`][c~objc2-app-kit~docs] for native windows, menus, and application lifecycle. AppKit APIs are main-thread-only; obtain an `objc2::MainThreadMarker` on the main thread and pass it to APIs that require one. For a small system-information task, Foundation's `NSProcessInfo` exposes the operating-system version string. See the [[objc | Objective-C]] chapter and [#1075](https://github.com/john-cd/rust_howto/issues/1075) for the broader binding patterns.

The included example reads `NSProcessInfo::processInfo().operatingSystemVersionString()` through the typed, safe `objc2-foundation` methods. The bindings return retained objects, and the example copies the version into a Rust `String` before printing. It reports an empty result as an error and needs no application window, user data, or direct unsafe Objective-C calls.

The package enables only the Foundation `std`, `NSProcessInfo`, and `NSString` features explicitly. From `later/crates` on macOS, run `cargo run -p os_macos_apis --example macos`. The version string depends on the installed operating system; other hosts run a message-only fallback.

## Store credentials in the Keychain

Use [`security-framework`][c~security-framework~docs] to store and retrieve generic or internet passwords through the macOS Keychain. Identify each item by its service and account rather than keeping credentials in source files or application preferences. Keychain access can prompt the user or depend on the app's signing and access-control settings; handle access-denied and missing-item errors as normal outcomes.

## Watch filesystem changes

For a cross-platform watcher, [`notify`][c~notify~docs] uses native backends, including FSEvents on macOS. For direct FSEvents access, use the `FSEvents` APIs exposed by [`objc2-core-services`][c~objc2-core-services~docs]. The older `fsevent-sys` crate is deprecated in favor of `objc2-core-services`. Treat notifications as prompts to inspect the filesystem again; they are not a durable log of every individual file operation.

## Bundle an application

[`cargo-bundle`][c~cargo-bundle~docs] packages a Cargo executable as a macOS `.app` bundle. Add `[package.metadata.bundle]` settings for the bundle identifier, name, icon, and minimum macOS version, then run `cargo bundle --release` from the application package. Bundling does not guarantee that the app meets a developer's signing, entitlement, or notarization requirements; configure and verify those distribution steps separately.

## Build and test on macOS

Put macOS-only dependencies in `[target.'cfg(target_os = "macos")'.dependencies]` and gate the native module with `#[cfg(target_os = "macos")]`. A non-macOS fallback can keep other workspace builds working, but it does not exercise Apple frameworks. The advisory `native_platform_examples.yml` workflow runs this staging package and the Objective-C FFI example on a Mac runner. Inspect those native step results before treating the platform-specific paths as validated; the example does not test every framework discussed here.

## Example {#macos}

```rust,editable
{{#include ../../../crates/cats/os_macos_apis/examples/macos/macos.rs:example}}
```

## Related Topics {#related-topics .skip}

- macOS system APIs.
- Cocoa and native GUI development.
- Platform-specific bindings.
- Cross-platform Rust on desktop.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [macos: write](https://github.com/john-cd/rust_howto/issues/435)
</div>
