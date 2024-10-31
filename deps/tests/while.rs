fn main() {
    let mut number = 5;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
}

#[test]
fn test() {
    main();
}
