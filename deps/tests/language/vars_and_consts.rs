#![allow(unused)]
// ANCHOR: example

fn main() {
    // `const` is set to a constant expression; the type must be annotated
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let apples = 5; // Immutable variable

    let mut guess = String::new(); // Mutable variable
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
