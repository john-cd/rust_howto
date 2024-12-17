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
// [add example for Send / Sync (P0)](https://github.com/john-cd/rust_howto/issues/93)
