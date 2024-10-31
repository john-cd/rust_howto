fn main() {
    let number = 3;
    let result = if number < 5 {
        // condition must return a bool; `if` is an expression
        println!("condition was true");
        5
    } else {
        println!("condition was false");
        6
    };
    println!("{}", result);
}

#[test]
fn test() {
    main();
}
