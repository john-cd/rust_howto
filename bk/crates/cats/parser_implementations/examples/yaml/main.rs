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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
