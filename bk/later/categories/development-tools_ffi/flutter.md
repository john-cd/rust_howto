# Generate FFI Bindings to Flutter Code

{{#include flutter.incl.md}}

Interfacing with Flutter/Dart from Rust is primarily done using FFI (Foreign Function Interface) and requires a bit of setup on both the Rust and Dart sides.

The [`flutter_rust_bridge`][c~flutter_rust_bridge~docs]⮳{{hi:flutter_rust_bridge}} crate is the most important crate. It significantly simplifies the process of creating and managing the FFI bridge between Rust and Flutter/Dart. It's highly recommended to use it unless you have very specific low-level FFI needs.

## Rust Side {#skip1}

[`flutter_rust_bridge`][c~flutter_rust_bridge~docs]⮳{{hi:flutter_rust_bridge}}: This crate aims to simplify the FFI interaction between Dart and Rust. It handles a lot of the boilerplate and code generation. This is the strongly recommended approach.

`std::ffi`, [`libc`][c~libc~docs]⮳{{hi:libc}}: These are standard library [modules][p~modules] for lower-level FFI interaction. You'll likely use these indirectly via [`flutter_rust_bridge`][c~flutter_rust_bridge~docs]⮳{{hi:flutter_rust_bridge}}, or if you're implementing the FFI manually.

[`cbindgen`][c~cbindgen~docs]⮳{{hi:cbindgen}}: Useful if you're working with C code as an intermediary, although [`flutter_rust_bridge`][c~flutter_rust_bridge~docs]⮳{{hi:flutter_rust_bridge}} generally handles this for you.

## Dart/Flutter Side {#skip2}

- 'ffi': (Dart's FFI package for interacting with native code.)

## `flutter_rust_bridge` {#flutter_rust_bridge}

[![flutter_rust_bridge][c~flutter_rust_bridge~docs~badge]][c~flutter_rust_bridge~docs] [![flutter_rust_bridge~crates.io][c~flutter_rust_bridge~crates.io~badge]][c~flutter_rust_bridge~crates.io] [![flutter_rust_bridge~github][c~flutter_rust_bridge~github~badge]][c~flutter_rust_bridge~github] [![flutter_rust_bridge~lib.rs][c~flutter_rust_bridge~lib.rs~badge]][c~flutter_rust_bridge~lib.rs]{{hi:flutter_rust_bridge}}{{hi:Bindings}}{{hi:Ffi}}{{hi:Code-generation}}{{hi:Flutter}}{{hi:Dart}} [![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}

[`flutter_rust_bridge`][c~flutter_rust_bridge~docs]⮳{{hi:flutter_rust_bridge}} is a Flutter/Dart and Rust binding generator.

[`flutter_rust_bridge`][c~flutter_rust_bridge~docs]⮳{{hi:flutter_rust_bridge}} helps generate the necessary boilerplate code (both Rust and Dart) to bridge the FFI gap.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs:example}}
```

## Communication Between Rust and Dart {#skip3}

You'll serialize data in Rust and deserialize it in Dart, or vice-versa.

- [`serde`][c~serde~docs]⮳{{hi:serde}} can serialize and deserialize data passed between Rust and Dart.
- [`bincode`][c~bincode~docs]⮳{{hi:bincode}} is a compact binary format often used for efficient FFI communication.

For more complex interactions, you might use platform channels, but [`flutter_rust_bridge`][c~flutter_rust_bridge~docs]⮳{{hi:flutter_rust_bridge}} often abstracts this away.

## Build Tools {#skip4}

Use:

- [`cargo`][c~cargo~docs]⮳{{hi:cargo}} for building the Rust library.
- Flutter's build system.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; review in depth](https://github.com/john-cd/rust_howto/issues/1071)
</div>
