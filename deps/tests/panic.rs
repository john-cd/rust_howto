fn main() {
    panic!("crash and burn");
}

#[should_panic]
#[test]
fn test() {
    main();
}
