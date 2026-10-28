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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
