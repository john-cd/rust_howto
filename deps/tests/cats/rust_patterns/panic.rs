// ANCHOR: example
fn main() {
    panic!("crash and burn");
}
// ANCHOR_END: example

#[should_panic]
#[test]
fn test() {
    main();
}
