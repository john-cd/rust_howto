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

#[ignore = "Needs review"]
#[test]
fn test() {
    main();
}
// [finish; deal wth cc](https://github.com/john-cd/rust_howto/issues/1000)
