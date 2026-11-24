// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Return-position `impl Trait` (RPIT) hides the concrete return type.
//! The function chooses one concrete type for each instantiation. Here the
//! caller can invoke the closure without naming its compiler-generated type.

/// Returns a closure that adds one to its argument.
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

/// Calls a function that returns a closure with `impl Trait`.
fn main() {
    let f = returns_closure();
    println!("{}", f(1));
}
// ANCHOR_END: example

/// Calls a function that returns a closure with `impl Trait`.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rpit example through its entry point.
    #[test]
    fn test_rpit_example_entry_point() {
        main();
    }

    /// The returned closure adds one.
    #[test]
    fn test_returns_closure() {
        let f = returns_closure();
        assert_eq!(f(1), 2);
        assert_eq!(f(-1), 0);
        assert_eq!(f(i32::MAX - 1), i32::MAX);
    }
}
