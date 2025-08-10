# Cross-platform Development

Cross-platform development refers to the practice of creating software applications that can run on multiple operating systems and devices (Windows, macOS, Linux, and mobile operating systems like iOS and Android) without requiring multiple codebases.

## Approaches

## Cross-compilation

For applications that do not require a graphical user interface (e.g., CLI tools, servers), simple cross-compilation may be sufficient.

- [`cargo`][c~cargo~docs]↗{{hi:cargo}} and [`rustup`][rustup~website]↗{{hi:rustup}} natively allow developers to build applications for different target platforms from a single development environment.
- [`cross`][c~cross~docs]↗{{hi:cross}} is a tool for cross-compiling Rust projects.

See [[cross_compilation | Cross Compilation]] and [[cross_compiling | Cross Compiling]].

Conditional compilation allows you to write platform-specific code, for example to call OS-specific APIs, such as [[os_freebsd-apis | FreeBSD]], [[os_linux-apis | Linux]], [[os_macos-apis | macOS]], [[os_unix-apis | Unix]] or [[os_windows-apis | Windows APIs]].

The `#[cfg(...)]` attribute is the primary mechanism for conditional compilation. It allows you to specify conditions that must be met for the code to be included in the compilation, including the target operating system (e.g., `target_os = "windows"`, `target_os = "linux"`) and/or the target architectures (e.g., `target_arch = "x86_64"`, `target_arch = "arm"`).

## Web Development

Web applications are intrinsically cross-platform, since most platforms have a web browser. You may ship your software as a web application (e.g., a single page application).

Rust can also be compiled to WebAssembly, allowing developers to run performant code in web browsers. See the [[wasm | WASM]] chapter.

## Desktop Applications

Rust can be used to build high-performance desktop applications for Windows, macOS, and Linux.

### `tauri` {#tauri}

[`tauri`][c~tauri~docs]↗{{hi:tauri}} allows you to build cross-platform desktop applications for major operating systems (Windows, macOS, Linux) using web technologies for the frontend and Rust for the backend.

### `wry` {#wry}

Wry is a cross-platform WebView rendering library. It provides a way to create cross-platform desktop applications with a focus on simplicity and performance. It is often used in conjunction with [`tauri`][c~tauri~docs]↗{{hi:tauri}}.

FIXME

### GTK

[`GTK-rs`](https://crates.io/crates/gtk4)↗{{hi:GTK-rs}} is a Rust binding for the GTK (GIMP Toolkit) library, which is widely used for creating graphical user interfaces on Linux and other platforms.

## Mobile Development

While native mobile development with Rust is nascent, Rust `can` be combined with frameworks like [`Flutter`](https://docs.flutter.dev)↗{{hi:Flutter}} to create cross-platform mobile apps. Rust can provide the backend logic, and Flutter handles the UI.

FIXME

## Embedded Systems

Rust is becoming increasingly used in embedded systems due to its performance and safety.

FIXME

## Notable Cross-Platform Application Frameworks

### `crux`

Crux is a framework for building cross-platform applications with a focus on testability and code reuse. It separates your application's logic into a Rust core that can be shared across platforms, with platform-specific "shells" for the UI.

{{#include crux.incl.md}}

### `dioxus`

Dioxus is a Rust-based framework designed for building cross-platform user interfaces. It enables developers to create applications for web, desktop, and mobile platforms using a single Rust codebase. Dioxus is designed to be familiar to developers with web development experience, utilizing concepts similar to HTML, CSS, and JavaScript. It includes features like hot reloading, server-side rendering, and support for modern web technologies. It uses a Virtual DOM for efficient UI updates. It also provides a CLI tool to aid in project creation, building, and serving.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[add / write; link to other chapters; conditional compile](https://github.com/john-cd/rust_howto/issues/583)
</div>
