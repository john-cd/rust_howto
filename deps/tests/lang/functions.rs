// ANCHOR: example
fn foo(x: i32, unit_label: char) -> i32 {
    let y = {
        let z = 3;
        x + z // expression at the end of a block - no semi-colon
    };

    println!("The value of y is: {y}{unit_label}");
    y // returns y - no semi-colon
}

fn main() {
    println!("{}", foo(1, 'm'));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
