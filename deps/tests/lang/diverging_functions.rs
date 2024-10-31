fn foo() -> ! {
    // ! is the Never type
    panic!("This call never returns.");
}

fn main() {
    println!("Will panic");
    foo();
}

#[should_panic]
#[test]
fn test() {
    main();
}
