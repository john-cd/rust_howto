// // ANCHOR: example

// // Integrate a Rust function into a Flutter project using
// // `flutter_rust_bridge`.
// // Typical scenarios to combine them include:
// //
// // - When you want a UI framework for your Rust system.
// // - Use arbitrary Rust libraries in Flutter: When the desired
// // functionality only has a library in Rust, not Dart / Flutter.
// // - Need high-performance code for Flutter: Rust makes it easy and
// // performant to write multi-thread code, data-intensive operations.
// //
// // 1. Install Rust.
// // 2. Install [Flutter](https://flutter.dev).
// // 3. Install the flutter_rust_bridge_codegen crate and create an app.
// // ```sh
// // cargo install flutter_rust_bridge_codegen && flutter_rust_bridge_codegen
// // create my_app && cd my_app && flutter run
// // ```
// // You can also use: `cargo binstall flutter_rust_bridge_codegen`
// // to install the binary directly.
// // 4. Regenerate the bridge when the Rust code changes:
// // ```sh
// // flutter_rust_bridge_codegen generate && flutter run
// // ```
// // 5. Autoexecute the code generator whenever the Rust code is changed:
// // `flutter_rust_bridge_codegen generate --watch`

// /// This function greets the user with a personalized message.
// pub fn hello_from_rust(name: &str) -> String {
//     format!("Hello, {}! This is Rust speaking.", name)
// }
// // ANCHOR_END: example

// // [finish; reorg as a project using flutter_rust_bridge_codegen](https://github.com/john-cd/rust_howto/issues/1028)
