// ANCHOR: example
fn main() {
    let x = 3;
    match x {
        1 | 2 => println!("one or two"),
        3 | 4 => println!("three or four"),
        _ => println!("something else"),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
