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

// TODO P1 deal wth cc
#[ignore]
#[test]
fn test() {
    main();
}
