// ANCHOR: example
fn main() {}
// ANCHOR_END: example

#[test]
#[ignore = "later"]
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/814)
