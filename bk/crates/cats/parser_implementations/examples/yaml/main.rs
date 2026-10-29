//! Runs the YAML parser example.
//!
//! TODO Reviewed ??-??-2026.

mod serde_saphyr;

/// Runs the yaml example.
fn main() -> anyhow::Result<()> {
    serde_saphyr::run()
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
