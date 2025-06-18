// ANCHOR: example
fn main() {
    let x: Option<i32> = Some(5);

    if let Some(val) = x { // Declares a `val` variable and bind it to the inner vaue of `x`.
        println!("Got a value: {}", val);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
