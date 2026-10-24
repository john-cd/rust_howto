<!-- TODO Reviewed ??-??-2026 -->

# Cross-platform Development

Cross-platform software targets more than one operating system, device, or runtime. Sharing Rust code helps, but each target still has its own toolchain, libraries, UI conventions, and platform APIs. Decide which parts to share, then keep target-specific code behind small boundaries.

## Choose what to share

### Build the same program for several targets

For command-line programs, servers, and libraries, use portable Rust APIs and build for each supported target. A separate target build produces a target-specific artifact; it does not make one binary run on every platform.

### Share behavior and keep platform UIs

When the business rules should match but the interfaces should feel native, put the shared behavior in a Rust library and connect each platform shell to it. The [Crux chapter][ex~cross-platform~develop-across-platforms] shows this Core/Shell approach.

{{#include crux.incl.md}}

### Target a browser or embedded device

For browser applications, compile Rust to WebAssembly and integrate with browser APIs as needed; see [[wasm | WebAssembly]]. Embedded projects also target specific chips and often use `no_std` libraries and hardware abstraction layers; see [[embedded | Embedded Systems]] and [[no-std | no_std]].

## Cross-compile for another target

Rust's target triple identifies the platform for which the compiler produces code. `rustup` can install a target's standard library, and Cargo can then build for it:

```sh
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu
```

Installing the Rust target does not install every tool needed to link an application. Cross-compilation may also require a target linker, system libraries, or a platform SDK. The [`cross`][c~cross~docs]↗{{hi:cross}} project can help with supported targets and their toolchains; check its current target support and prerequisites.

Read the [rustup cross-compilation guide][rustup-cross-compilation] for target setup details. A successful cross-build checks compilation for that target, but exercising platform behavior still requires a suitable runner, emulator, device, or CI host.

## Compile only the code a target needs

Use conditional compilation for code that must call a target-specific API or depend on a target-specific crate. For example, `#[cfg]` includes only the matching function in the compiled program:

```rust
#[cfg(target_os = "windows")]
fn platform_name() -> &'static str {
    "Windows"
}

#[cfg(target_os = "linux")]
fn platform_name() -> &'static str {
    "Linux"
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn platform_name() -> &'static str {
    "another target"
}

fn main() {
    println!("Running on {}", platform_name());
}
```

`target_os`, `target_arch`, and related configuration values describe the compilation target, not necessarily the machine running the compiler. The [`#[cfg]` attribute][rust-reference-conditional-compilation] removes a non-matching item before type checking. By contrast, `cfg!(...)` produces a boolean expression; it does not remove code from the build, so both sides must still compile for the current target.

Prefer portable APIs and dependencies that support all your targets. Isolate unavoidable platform differences in a small adapter, and use Cargo target-specific dependencies when a crate is only needed on some targets.

## Choose an application framework

### Web applications

WebAssembly can run Rust code in browsers, but a web app still uses browser APIs for tasks such as the DOM, storage, and networking. See the [[wasm | WebAssembly]] chapter for browser integration and framework examples.

### Desktop and mobile applications

Choose how much UI and platform integration to share. [`tauri`][c~tauri~docs]↗{{hi:tauri}} uses a web frontend with a Rust application layer; [`Wry`][wry-docs] is a lower-level WebView library. For a GTK-based interface, see [`gtk4`][c~gtk4~crates.io]↗{{hi:GTK-rs}}. On mobile, a native UI can call a shared Rust library through an FFI boundary; Crux is one way to share behavior while leaving each UI native.

[`Dioxus`][dioxus-platform-support] provides web, desktop, and mobile renderers. Their setup and platform-specific features differ, so check the current platform guide before selecting it.

## Keep operating-system APIs local

For system-specific APIs, use the smallest platform-specific layer possible. The book has guides for [[os_freebsd-apis | FreeBSD]], [[os_linux-apis | Linux]], and [[os_macos-apis | macOS]].

## Related Topics

- [[cross-platform | Cross-platform Development]].
- [[gui | GUI]].
- [[embedded | Embedded Systems]].
- [[wasm | WebAssembly]].
- [[architecture | Software Architecture]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[add / write; link to other chapters; conditional compile](https://github.com/john-cd/rust_howto/issues/583)
[content audit: content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1904)
</div>
