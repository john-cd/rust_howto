// ANCHOR: example
fn main() {
    // TODO P1
    todo!();
}
// ANCHOR_END: example

#[test]
#[ignore = "not yet implemented"]
fn test() {
    main();
}
