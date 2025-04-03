// ANCHOR: example

/// This function takes an integer `x` and a character `unit_label` as input.
/// It calculates a value `y` by adding 3 to `x`.
/// It then prints the value of `y` along with the `unit_label`.
/// Finally, it returns the calculated value `y`.
fn foo(x: i32, unit_label: char) -> i32 {
    let y = {
        let z = 3;
        x + z // Expression at the end of a block - no semi-colon.
    };

    println!("The value of y is: {y}{unit_label}");
    y // Returns `y` - no semi-colon.
}

fn main() {
    println!("{}", foo(1, 'm'));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
