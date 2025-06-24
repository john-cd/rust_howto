#![allow(dead_code)]
// ANCHOR: example
/// Example of how to use `cc::Build` to define preprocessor macros.
fn main() {
    // cc::Build::new()
    //     // Define a string macro.
    //     .define("APP_NAME", "\"foo\"")
    //     // Define a string macro with a value from an environment variable.
    //     .define(
    //         "VERSION",
    //         format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
    //     )
    //     // Define a macro without a value.
    //     .define("WELCOME", None)
    //     .file("src/foo.c")
    //     .compile("foo");
}
// ANCHOR_END: example

#[ignore = "Needs review"]
#[test]
fn test() {
    main();
}
// [finish; deal wth cc](https://github.com/john-cd/rust_howto/issues/1000)
