// ANCHOR: example
fn main() {
    todo!();
}
// ANCHOR_END: example

#[test]
#[ignore = "not yet implemented"]
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/697)