//! Runs the `pest` parser example.
//!
//! TODO Reviewed ??-??-2026.

mod pest;

/// Runs the pest example.
fn main() -> anyhow::Result<()> {
    pest::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn main_runs() {
        main().expect("the example should run");
    }
}
