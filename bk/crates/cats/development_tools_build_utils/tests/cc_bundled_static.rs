// ANCHOR: example
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
