<!-- TODO Reviewed ??-??-2026. -->

# Cross-platform Development

Cross-platform software targets more than one operating system, device, or runtime. Sharing Rust code helps, but each target still has its own toolchain, libraries, UI conventions, and platform APIs. Decide which parts to share, then keep target-specific code behind small boundaries.

## Choose what to share

### Build the same program for several targets

For command-line programs, servers, and libraries, use portable Rust APIs and build for each supported target. A separate target build produces a target-specific artifact; it does not make one binary run on every platform.

### Share behavior and keep platform UIs

When the business rules should match but the interfaces should feel native, put the shared behavior in a Rust library and connect each platform shell to it. The [Crux chapter][ex~cross-platform~develop-across-platforms] shows this Core/Shell approach.

{{#include crux.incl.md}}

### Target a browser or embedded device

For browser applications, compile Rust to WebAssembly and integrate with browser APIs as needed; see [WebAssembly](../../categories/wasm/index.md). Embedded projects also target specific chips and often use `no_std` libraries and hardware abstraction layers; see [Embedded Systems](../../categories/embedded/index.md) and [no_std](https://github.com/john-cd/rust_howto/blob/86dcf0197f8d6810877a6021ffedcf84aa9230c8/bk/src/categories/no-std/no_std.md). Crates such as [`cortex-m`][c~cortex-m~docs]↗{{hi:cortex-m}}, [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} and [`rtic`][c~rtic~docs]↗{{hi:rtic}} are common starting points.

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

WebAssembly can run Rust code in browsers, but a web app still uses browser APIs for tasks such as the DOM, storage, and networking. See the [WebAssembly](../../categories/wasm/index.md) chapter for browser integration and framework examples.

### Desktop and mobile applications

Choose which parts you want in common across targets.

| What to share | Common crates | What to check |
| --- | --- | --- |
| A web interface and a Rust application layer | [`tauri`][c~tauri~docs], or [`wry`][wry-docs] for direct WebView integration | Check the target's WebView, permissions, and installer. See [Web-based desktop GUIs](../../categories/gui/index.md#web-based-guis). |
| A Rust component model | [`dioxus`][dioxus-platform-support] | Check the feature set and prerequisites of the selected desktop, mobile, or web renderer. |
| A desktop interface drawn from Rust | [`egui`][c~egui~docs], [`iced`][c~iced~docs], or [`slint`][c~slint~docs] | Compare the [GUI recipes](../../categories/gui/index.md) and check keyboard input, accessibility, and packaging on each target. |
| Application rules behind a native UI | [`crux_core`][c~crux_core~docs], or a Rust library with generated bindings | Keep the UI in the platform shell or connect a [Flutter interface](../../categories/development-tools_ffi/flutter.md). Smoke check the binding and lifecycle on a real target. |

## Keep operating-system APIs local

For system-specific APIs, use the smallest platform-specific layer possible. The book has guides for [FreeBSD](../../categories/os_freebsd-apis/index.md), [Linux](../../categories/os_linux-apis/index.md), and [macOS](../../categories/os_macos-apis/index.md).

## Related Topics

- [Native OS APIs](../native-os/index.md) for platform bindings and ownership.
- [GUI](../../categories/gui/index.md).
- [Embedded Systems](../../categories/embedded/index.md).
- [WebAssembly](../../categories/wasm/index.md).
- [Software Architecture](https://github.com/john-cd/rust_howto/blob/c9d457b66ed12377a6ff5e0e1377af71cc9dde0b/bk/src/other/architecture/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
