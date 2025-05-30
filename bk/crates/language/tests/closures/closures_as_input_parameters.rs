#![allow(dead_code)]
// ANCHOR: example
// This function demonstrates how to accept a closure as an input parameter.
// The `F` in `fn apply<F>(f: F)` is a generic type parameter, allowing
// the function to accept any type that satisfies the specified trait bounds.
fn apply<F>(f: F)
where
    F: FnOnce(), // The closure `f` takes no input and returns nothing.
{
    f();
}

// This function demonstrates how to accept a closure that takes an input
// and returns a value.
fn apply_to_3<F>(f: F) -> i32
where
    // The closure takes an `i32` and returns an `i32`.
    // `Fn` closures can only take immutable references to captured variables
    // or don't capture anything at all.
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    apply(|| println!("Applied"));
    let var = 39;
    println!("`apply_to_3` result: {}", apply_to_3(|x| x + var));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
