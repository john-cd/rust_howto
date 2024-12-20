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
// [need for tokio.rs example? P0](https://github.com/john-cd/rust_howto/issues/95)
