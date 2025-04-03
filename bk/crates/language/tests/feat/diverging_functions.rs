// ANCHOR: example

/// This function diverges, meaning it never returns.
/// It uses the `!` (Never) type to indicate this.
fn foo() -> ! {
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
