//! Runs the artificial intelligence examples.
//!
//! TODO Reviewed ??-??-2026.

mod astar;
mod bonsai_bt;

/// Runs the artificial intelligence examples.
fn main() -> anyhow::Result<()> {
    astar::run()?;
    bonsai_bt::run()
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
