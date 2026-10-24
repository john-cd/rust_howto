// ANCHOR: example
//! Prints an expression with `dbg!` and continues with its value.

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
    /// Verifies that `dbg!` returns the value of its expression.
    #[test]
    fn dbg_returns_the_expression_value() {
        let value = 2 + 2;

        assert_eq!(dbg!(value), 4);
    }
}
