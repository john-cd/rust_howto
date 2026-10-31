// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Lifetime parameters on a function.
//!
//! A lifetime describes how long a reference stays valid. When a function
//! returns a reference, the compiler must know which input it borrows from, so
//! you name the relationship with a lifetime parameter such as `'a`. It adds no
//! run-time cost; it only lets the compiler prove that a reference never
//! outlives its data.

/// This function has a lifetime parameter `'a`.
/// The lifetime parameter can be used by references within the function
/// signature, return type, or body.
///
/// In this example, the function takes two string slices, `x` and `y`, both
/// with the same lifetime `'a`. It returns a string slice that is the longer of
/// the two, also with the lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// Compares strings with different lifetimes.
fn main() {
    // We define two variables, one with a scope / lifetime
    // shorter than the other.
    let x = String::from("looooooong");
    let l;
    {
        let y = String::from("short");
        l = longest(&x, &y);
        println!("Longest: {l}");
    } // y is dropped here.
    // println!("Longest: {l}"); // ERROR: borrowed value does not live long
    // enough.
    println!("x: {x}");
    // x is valid until this line.
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the lifetime parameter function example through its entry point.
    #[test]
    fn test_lifetime_parameter_function_example_entry_point() {
        main();
    }

    /// The longer string wins; ties return the second.
    #[test]
    fn test_longest() {
        assert_eq!(longest("looooooong", "short"), "looooooong");
        assert_eq!(longest("a", "bb"), "bb");
        assert_eq!(longest("aa", "bb"), "bb");
        assert_eq!(longest("", ""), "");
    }
}
