// ANCHOR: example
fn foo() -> ! {
    // ! is the Never type
    panic!("This call never returns.");
}

fn main() {
    println!("Will panic");
    foo();
}
// ANCHOR_END: example

#[should_panic]
#[test]
fn test() {
    main();
}
