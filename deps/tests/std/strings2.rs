fn main() {
    // ANCHOR: example
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);
    // ANCHOR_END: example
}

#[test]
fn test() {
    main();
}