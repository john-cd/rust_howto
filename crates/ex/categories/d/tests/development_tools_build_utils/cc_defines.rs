// ANCHOR: example
fn main() {
    // cc::Build::new()
    //     .define("APP_NAME", "\"foo\"")
    //     .define(
    //         "VERSION",
    //         format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
    //     )
    //     .define("WELCOME", None)
    //     .file("src/foo.c")
    //     .compile("foo");
}
// ANCHOR_END: example

// [ P1 deal wth cc](https://github.com/john-cd/rust_howto/issues/1000)
#[ignore]
#[test]
fn test() {
    main();
}
