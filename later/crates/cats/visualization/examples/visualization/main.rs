//! Runs the visualization example.
//!
//! TODO Reviewed ??-??-2026.

mod plotly;

/// Runs the visualization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    plotly::run().map_err(Into::into)
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
