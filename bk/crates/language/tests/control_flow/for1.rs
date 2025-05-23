// ANCHOR: example
fn main() {
    // Iterate over an array.
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // Range - generates all numbers in sequence,
    // starting from one number and ending before another number.
    for number in (1..4).rev() {
        // Use `rev` for reverse enumeration.
        println!("{number}!");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
