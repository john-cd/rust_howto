// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Represents and evaluates a tree with leaf and group nodes.
//!
//! The composite pattern treats single objects and groups of them uniformly, as
//! a tree. A file system, a user interface, or an arithmetic expression works
//! this way: a group node contains leaves and other groups.

/// A tree whose leaves are values and whose groups sum child expressions.
enum Expression {
    /// A numeric leaf value.
    Value(
        /// Literal value stored in the composite expression tree.
        i64,
    ),

    /// A group that adds its child expressions.
    Sum(
        /// Child expressions whose values are added together.
        Vec<Self>,
    ),
}

impl Expression {
    /// Recursively adds leaf values, treating an empty Sum as zero.
    ///
    /// Visits each node once and uses call-stack space proportional to tree depth.
    /// Integer addition follows the build profile's overflow-checking behavior.
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
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::Expression;
    use super::main;

    /// Verifies that the nested expression sums all values.
    #[test]
    fn nested_expression_sums_all_values() {
        let expression = Expression::Sum(vec![
            Expression::Value(20),
            Expression::Sum(vec![Expression::Value(10), Expression::Value(12)]),
        ]);

        assert_eq!(expression.evaluate(), 42);
    }

    /// Runs the example through its entry point without error.
    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
