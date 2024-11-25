// ANCHOR: example
fn main() {
    panic!("Crash and burn");
}
// ANCHOR_END: example

#[should_panic]
#[test]
fn test() {
    main();
}
