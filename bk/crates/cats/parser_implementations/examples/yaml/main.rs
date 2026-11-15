//! Runs the YAML parser example.
//!
//! TODO Reviewed ??-??-2026.

/// Serialize a Rust value to YAML and deserialize it with `serde-saphyr`.
mod serde_saphyr;

/// Runs the yaml example.
fn main() -> anyhow::Result<()> {
    serde_saphyr::run()
}

/// Verifies that the YAML parser example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        main().expect("the example should run");
    }
}
