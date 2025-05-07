// ANCHOR: example
/// This example demonstrates how to use the `cc` crate to compile a C file into
/// a static library.
///
/// The `cc` crate is a build dependency that allows you to compile C/C++ code
/// as part of your Rust build process.
///
/// In this example, we're compiling `src/hello.c` into a static library named
/// `libhello.a`.
///
/// Note that this example is commented out because it requires a C file to be
/// present in the `src` directory.
fn main() {
    // cc::Build::new().file("src/hello.c").compile("hello");
    // // outputs `libhello.a`
}
// ANCHOR_END: example

#[test]
#[ignore = "Needs review"]
fn test() {
    main();
}
// [finish; deal with cc](https://github.com/john-cd/rust_howto/issues/899)
