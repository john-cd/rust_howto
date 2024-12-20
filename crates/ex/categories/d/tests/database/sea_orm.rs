// ANCHOR: example
fn main() {
    todo!();
}
// ANCHOR_END: example

#[test]
#[ignore = "not yet implemented"]
fn require_external_svc() {
    main();
}
// [P0](https://github.com/john-cd/rust_howto/issues/715)
