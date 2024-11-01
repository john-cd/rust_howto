// ANCHOR: example
fn main() {
    panic!("crash and burn");
}

#[should_panic]
// ANCHOR_END: example
#[test]
fn test() {
    main();
}
