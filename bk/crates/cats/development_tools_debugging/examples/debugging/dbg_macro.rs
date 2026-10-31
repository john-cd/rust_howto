// ANCHOR: example
//! Prints an expression with `dbg!` and continues with its value.
//!
//! `dbg!` prints the expression, its value and the file and line to standard
//! error, and returns the value, so you can wrap any expression without
//! changing behaviour. It is meant for temporary debugging; remove it before
//! committing.

/// Demonstrates that `dbg!` prints an expression and returns it.
fn main() -> anyhow::Result<()> {
    let width = 5;
    let height = 3;
    let area = dbg!(width * height);
    println!("Area: {area}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the `dbg!` example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::main;

    /// Verifies that `dbg!` returns the value of its expression.
    #[test]
    fn dbg_returns_the_expression_value() {
        let value = 2 + 2;

        assert_eq!(dbg!(value), 4);
    }

    /// Exercises the user-facing example entry point.
    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
