fn foo() -> ! {
    // ! is the Never type
    panic!("This call never returns.");
}

#[test]
#[should_panic]
fn test() {
    println!("Will panic");
    foo();
}
