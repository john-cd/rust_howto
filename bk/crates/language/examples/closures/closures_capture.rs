#![allow(dead_code)]
// ANCHOR: example
fn main() {
    let outer_variable = 10;

    // Define a simple closure that takes a parameter and captures
    // `outer_variable`:
    let my_closure = |x| {
        println!("Parameter x = {x}");
        println!("Outer variable = {outer_variable}"); // Captured variable.
        x + outer_variable // Optional return value.
    }; // The body can be an expression or block between `{` and `}`.

    // Call the closure:
    let result = my_closure(5);
    println!("Result from closure: {result}"); // Output: 15.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
