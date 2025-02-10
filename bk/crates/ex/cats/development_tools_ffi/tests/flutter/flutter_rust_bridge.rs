// // ANCHOR: example

// // Integrate a Rust function into a Flutter project using flutter_rust_bridge

// // Typical scenarios to combine them include:
// //
// // - When you want a UI framework for your Rust system.
// // - Use arbitrary Rust libraries in Flutter: When the desired
// // functionality only has a library in Rust, not Dart / Flutter.
// // - Need high-performance code for Flutter: Rust makes it easy and
// performant // to write multi-thread code, data-intensive operations, SIMD
// code.

// // Install Rust.
// // Install Flutter: https://flutter.dev/
// // ```sh
// // cargo install flutter_rust_bridge_codegen && flutter_rust_bridge_codegen
// // create my_app && cd my_app && flutter run
// // ```
// // You can also use: `cargo binstall flutter_rust_bridge_codegen`
// to install the binary directly.

// // Regenerate the bridge when the Rust code changes:
// // flutter_rust_bridge_codegen generate && flutter run

// // Autoexecute the code generator whenever the Rust code is changed:
// // `flutter_rust_bridge_codegen generate --watch`

// pub fn hello_from_rust(name: &str) -> String {
//     format!("Hello, {}! This is Rust speaking.", name)
// }
// // ANCHOR_END: example

// [ P1 write; add to markdown - reorg as a project using flutter_rust_bridge_codegen](https://github.com/john-cd/rust_howto/issues/1028)
