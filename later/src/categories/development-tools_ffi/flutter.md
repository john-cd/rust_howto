<!-- TODO Reviewed ??-??-2026 -->

# Generate FFI Bindings to Flutter Code

{{#include flutter.incl.md}}

Interfacing with Flutter/Dart from Rust is primarily done using FFI (Foreign Function Interface) and requires a bit of setup on both the Rust and Dart sides.

The [`flutter_rust_bridge`][c~flutter_rust_bridge~docs]↗{{hi:flutter_rust_bridge}} crate generates Rust/Dart glue for a typed API. Use it when Rust owns application logic that a Flutter UI calls. A small existing C ABI can instead use Dart's lower-level FFI directly.

## Rust Side {#rust-side}

[`flutter_rust_bridge`][c~flutter_rust_bridge~docs]↗{{hi:flutter_rust_bridge}} handles code generation. Keep your API in ordinary functions and structs, returning explicit results for fallible operations. An `async fn` can represent asynchronous work; continuous updates can use a generated `StreamSink<T>`. Treat sink closure as cancellation and avoid keeping a producer alive after the Dart subscriber disappears. The [v2 type guide][ffi~frb-types] covers the supported types.

[`std::ffi`][c~std::ffi~docs]↗{{hi:std::ffi}} is a standard-library module, while [`libc`][c~libc~docs]↗{{hi:libc}} is a separate crate. They provide types useful for manual C ABI integration.

[`cbindgen`][c~cbindgen~docs]↗{{hi:cbindgen}}: Useful if you're working with C code as an intermediary, although [`flutter_rust_bridge`][c~flutter_rust_bridge~docs]↗{{hi:flutter_rust_bridge}} generally handles this for you.

## Dart/Flutter Side {#dart-flutter-side}

- [`dart:ffi`][ffi~dart-ffi] is Dart's native C interop library. The separate `package:ffi` supplies allocation and string helpers; neither is required as a user-designed serialization layer for a generated Rust bridge.

After generation, initialize the generated library before invoking it. Place the example API in `rust/src/api/simple.rs`, then regenerate and call the synchronous greeting from Dart:

```dart
import 'package:my_app/src/rust/api/simple.dart';
import 'package:my_app/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  final message = greet(name: 'Ada');
  final asyncMessage = await greetAsync(name: 'Ada');
  print(message);
  print(asyncMessage);

  final updates = greetStream(name: 'Ada');
  await for (final update in updates) {
    print(update);
  }
}
```

The import paths depend on the generated package name and configured output directory. In the Rust API, FRB v2 exposes the generated sink type as `crate::frb_generated::StreamSink`. The `greet_async` API returns a Dart `Future`; `greet_stream` returns a Dart `Stream` that yields each value passed to its sink. If the Dart stream closes before Rust sends an update, `StreamSink::add` returns a send error, which `greet_stream` converts into an `anyhow::Error` and returns to the bridge caller. Flutter applications should also initialize Flutter's binding before APIs that require it.

## `flutter_rust_bridge` {#flutter_rust_bridge}

[![flutter_rust_bridge][c~flutter_rust_bridge~docs~badge]][c~flutter_rust_bridge~docs] [![flutter_rust_bridge~crates.io][c~flutter_rust_bridge~crates.io~badge]][c~flutter_rust_bridge~crates.io] [![flutter_rust_bridge~repo][c~flutter_rust_bridge~repo~badge]][c~flutter_rust_bridge~repo] [![flutter_rust_bridge~lib.rs][c~flutter_rust_bridge~lib.rs~badge]][c~flutter_rust_bridge~lib.rs]{{hi:flutter_rust_bridge}}{{hi:Bindings}}{{hi:Ffi}}{{hi:Code-generation}}{{hi:Flutter}}{{hi:Dart}} [![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}

[`flutter_rust_bridge`][c~flutter_rust_bridge~docs]↗{{hi:flutter_rust_bridge}} is a Flutter/Dart and Rust binding generator.

[`flutter_rust_bridge`][c~flutter_rust_bridge~docs]↗{{hi:flutter_rust_bridge}} helps generate the necessary boilerplate code (both Rust and Dart) to bridge the FFI gap.

Install Rust and the stable Flutter SDK (which includes Dart), then install the code generator at the same version as the Rust runtime in `Cargo.toml`. The example uses `flutter_rust_bridge` 2.13.0. Follow the [v2 quickstart][ffi~frb-quickstart]:

```bash
cargo binstall --version 2.13.0 flutter_rust_bridge_codegen
flutter_rust_bridge_codegen create my_app
cd my_app
flutter_rust_bridge_codegen generate
flutter run
```

For an existing Flutter project, run `flutter_rust_bridge_codegen integrate` from its root instead of `create`. Keep the generator and runtime versions compatible and regenerate after API changes. The book's example below tests the plain API; generated application glue belongs in your Flutter project.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/flutter/flutter_rust_bridge.rs:example}}
```

## Communication Between Rust and Dart {#communication-between-rust-and-dart}

The generator converts supported Rust and Dart types at the boundary. You do not need to serialize every argument manually.

- [`serde`][c~serde~docs]↗{{hi:serde}} is useful when the application also needs an explicit storage or network format.
- [`bincode`][c~bincode~docs]↗{{hi:bincode}} is one possible Rust binary format, but Dart needs a compatible decoder if you choose to send those bytes.

For more complex interactions, you might use platform channels, but [`flutter_rust_bridge`][c~flutter_rust_bridge~docs]↗{{hi:flutter_rust_bridge}} often abstracts this away.

For manual `dart:ffi`, expose an `extern "C"` API from a Rust `cdylib`, use [`cbindgen`](generate_ffi_bindings.md#cbindgen) to describe it, and generate Dart declarations from the header with `ffigen`. Specify who allocates and frees buffers, encode errors in the ABI, and never let a Rust panic cross it. This native C ABI route is not available unchanged in a browser. [`rinf`][ffi~rinf~repo] is an alternative centered on messages between Flutter and Rust; check its current maintenance and supported platform status before adopting it.

## Build Tools {#build-tools .skip}

Use:

- [`cargo`][c~cargo~docs]↗{{hi:cargo}} for building the Rust library.
- Flutter's build system.

### Build and Test Each Platform {#flutter-platforms}

Android needs the NDK and Rust targets for the device ABIs you ship. iOS needs macOS, Xcode, and distinct device/simulator targets. Desktop builds need the host C/C++ toolchain and platform packaging; one shared library does not cover all desktop operating systems. The generated integration connects these builds to Flutter; see its [integration guide][ffi~frb-integration].

The web backend builds Rust to WebAssembly and uses browser integration rather than loading a native dynamic library. Audit dependencies for WebAssembly support and test the configured web backend separately. Do not assume a native filesystem or thread API works in a browser.

From `later/crates`, `cargo test --locked -p development_tools_ffi --example flutter` checks the plain Rust API. Add `--features flutter` to compile the `StreamSink` bridge example. These commands do not generate bindings, run Flutter, or validate mobile/web packaging. Run `flutter test` and a device/browser smoke test in the generated application as separate integration checks.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
