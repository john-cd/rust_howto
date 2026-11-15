//! Runs the `pest` parser example.
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates `pest` parsing a flat arithmetic expression
mod pest;

/// Runs the pest example.
fn main() -> anyhow::Result<()> {
    pest::run()
}

/// Verifies that the `pest` parser example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        main().expect("the example should run");
    }
}
