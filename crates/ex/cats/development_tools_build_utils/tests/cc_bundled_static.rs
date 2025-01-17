// ANCHOR: example
fn main() {
    // cc::Build::new().file("src/hello.c").compile("hello");
    // // outputs `libhello.a`
}
// ANCHOR_END: example

#[test]
#[ignore = "WIP"]
fn test() {
    main();
}
// [P1 deal with cc](https://github.com/john-cd/rust_howto/issues/899)
