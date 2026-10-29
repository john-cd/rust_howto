// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A generic function with a lifetime parameter.

/// Function with an explicit lifetime parameter.
///
/// `x` and `y` are string slices that live at least as long as the lifetime
/// `'a`. The reference returned by the longest function will have the same
/// lifetime as the lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// Finds the longer of two strings.
fn main() {
    // The compiler infers the concrete lifetime for `'a` at the call site,
    // choosing the shorter of the two input lifetimes.
    let result = longest("abcd", "xyz");
    println!("The longest string is {result}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the generic functions3 example through its entry point.
    #[test]
    fn test_generic_functions3_example_entry_point() {
        main();
    }

    /// The longer string is returned; ties return the second.
    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "xyz"), "abcd");
        assert_eq!(longest("a", "xyz"), "xyz");
        assert_eq!(longest("ab", "cd"), "cd");
        assert_eq!(longest("", ""), "");
    }
}
