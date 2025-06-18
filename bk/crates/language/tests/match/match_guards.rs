// ANCHOR: example
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 0 => println!("Negative: {}", x),
        Some(x) if x % 2 == 0 => println!("Even: {}", x),
        Some(x) => println!("Odd positive: {}", x),
        None => (),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
