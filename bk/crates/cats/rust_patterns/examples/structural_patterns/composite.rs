// ANCHOR: example
//! Represents and evaluates a tree with leaf and group nodes.

/// A tree whose leaves are values and whose groups sum child expressions.
enum Expression {
    /// A numeric leaf value.
    Value(i64),
    /// A group that adds its child expressions.
    Sum(Vec<Self>),
}

impl Expression {
    /// Evaluates a leaf or recursively combines a group's children.
    fn evaluate(&self) -> i64 {
        match self {
            Self::Value(value) => *value,
            Self::Sum(children) => children.iter().map(Self::evaluate).sum(),
        }
    }
}

/// Builds and evaluates a small composite expression tree.
fn main() -> anyhow::Result<()> {
    let expression = Expression::Sum(vec![
        Expression::Value(20),
        Expression::Sum(vec![Expression::Value(10), Expression::Value(12)]),
    ]);

    println!("Total: {}", expression.evaluate());
    Ok(())
}
// ANCHOR_END: example

/// Runs the composite tree example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::Expression;
    use super::main;

    /// Verifies a composite sums its nested leaf values recursively.
    /// Runs the composite example through its entry point.
    #[test]
    fn nested_expression_sums_all_values() {
        let expression = Expression::Sum(vec![
            Expression::Value(20),
            Expression::Sum(vec![Expression::Value(10), Expression::Value(12)]),
        ]);

        assert_eq!(expression.evaluate(), 42);
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
