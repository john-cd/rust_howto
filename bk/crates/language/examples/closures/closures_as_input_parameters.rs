#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Closures as function arguments.

// This function demonstrates how to accept a closure as an input parameter.
// The `F` in `fn apply<F>(f: F)` is a generic type parameter, allowing
// the function to accept any type that satisfies the specified trait bounds.
/// Calls the closure `f`.
fn apply<F>(f: F)
where
    F: FnOnce(), // The closure `f` takes no input and returns nothing.
{
    f();
}

// This function demonstrates how to accept a closure that takes an input
// and returns a value.
/// Calls the closure `f` with 3 and returns its result.
fn apply_to_3<F>(f: F) -> i32
where
    // The closure takes an `i32` and returns an `i32`.
    // `Fn` closures can only take immutable references to captured variables
    // or don't capture anything at all.
    F: Fn(i32) -> i32,
{
    f(3)
}

/// Passes closures to the functions.
fn main() {
    apply(|| println!("Applied"));
    let var = 39;
    println!("`apply_to_3` result: {}", apply_to_3(|x| x + var));
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
