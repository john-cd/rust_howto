# Generate FFI bindings to Flutter code

{{#include flutter.incl.md}}

## `flutter_rust_bridge` {#flutter_rust_bridge}

[![flutter_rust_bridge][c-flutter_rust_bridge-badge]][c-flutter_rust_bridge] [![flutter_rust_bridge-crates.io][c-flutter_rust_bridge-crates.io-badge]][c-flutter_rust_bridge-crates.io] [![flutter_rust_bridge-github][c-flutter_rust_bridge-github-badge]][c-flutter_rust_bridge-github] [![flutter_rust_bridge-lib.rs][c-flutter_rust_bridge-lib.rs-badge]][c-flutter_rust_bridge-lib.rs]{{hi:flutter_rust_bridge}}{{hi:Bindings}}{{hi:Ffi}}{{hi:Code-generation}}{{hi:Flutter}}{{hi:Dart}} [![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}

[`flutter_rust_bridge`][c-flutter_rust_bridge]⮳{{hi:flutter_rust_bridge}} is a Flutter/Dart and Rust binding generator.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/flutter/flutter_rust_bridge.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write](https://github.com/john-cd/rust_howto/issues/1071)

Interfacing with Flutter/Dart from Rust is primarily done using FFI (Foreign Function Interface) and requires a bit of setup on both the Rust and Dart sides.

Rust Side:

flutter_rust_bridge: This crate aims to simplify the FFI interaction between Dart and Rust. It handles a lot of the boilerplate and code generation. This is the strongly recommended approach.

std::ffi, libc: (These are standard library modules for lower-level FFI interaction. You'll likely use these indirectly via flutter_rust_bridge, or if you're implementing the FFI manually.)

cbindgen: (Useful if you're working with C code as an intermediary, although flutter_rust_bridge generally handles this for you.)

Dart/Flutter Side:

ffi: (Dart's FFI package for interacting with native code.)
Communication/Serialization:

serde: (Essential for serializing and deserializing data passed between Rust and Dart.)
bincode: (A compact binary format often used for efficient FFI communication.)
Build Tools:

cargo: (For building the Rust library.)
Flutter's build system: (Handles the Dart/Flutter side and integrates with the Rust build.)
Key Concepts and Workflow:

FFI: You'll define functions in Rust that are callable from Dart.
Code Generation: flutter_rust_bridge helps generate the necessary boilerplate code (both Rust and Dart) to bridge the FFI gap.
Data Serialization: You'll serialize data in Rust (e.g., using serde) and deserialize it in Dart, or vice-versa.
Platform Channels (if needed): For more complex interactions, you might use platform channels, but flutter_rust_bridge often abstracts this away.
The flutter_rust_bridge crate is the most important crate here. It significantly simplifies the process of creating and managing the FFI bridge between Rust and Flutter/Dart. It's highly recommended to use it unless you have very specific low-level FFI needs.
</div>
