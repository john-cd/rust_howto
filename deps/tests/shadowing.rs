fn main() {
    let x = 5; // x is immutable

    let x = x + 1; // redefines x
    println!("{x}");

    let x = "example"; // the type can change
    println!("{x}");
}

#[test]
fn test() {
    main();
}
