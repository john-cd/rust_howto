#![allow(unused_assignments)]
// ANCHOR: example
fn main() {
    // `x` is an immutable variable.
    let x = 5;
    // Its value can't change.
    // ERROR: x = x + 1;

    // But it can be redefined (shadowed).
    // Any subsequent uses of the `x` name within the current scope will refer
    // to the new variable, and the old variable becomes inaccessible by
    // that name. Notice the `let`:
    let x = x + 1;
    // `x` is a _new_ variable that shadows the previous one.
    println!("x: {x}");

    // The type can change by shadowing.
    let x = "example";
    println!("x: {x}");

    // Let's create an inner scope (block) with curly brackets:
    {
        let x = "within scope";
        println!("x: {x}");
        assert_eq!(x, "within scope");
    }
    // When that scope is over, the inner shadowing ends and the previous `x`
    // definition applies:
    println!("x: {x}");
    assert_eq!(x, "example");

    // In contrast, a mutable variable's value can change, but its type can't.
    let mut y = 10;
    y = 20; // Valid: changing the value of `y`
    // y = "hello"; // ERROR: cannot change the type of `y`
    println!("y: {y}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
