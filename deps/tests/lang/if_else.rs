// ANCHOR: example
fn main() {
    let number = 3;
    let result: u8 = if number < 5 {
        println!("Condition was true");
        5 // `if` is an expression
    } else {
        println!("Condition was false");
        6
    };
    println!("{}", result);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
