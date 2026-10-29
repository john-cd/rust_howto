//! Runs the `tera` template example.
//!
//! TODO Reviewed ??-??-2026.

mod tera;

/// Runs the tera example.
fn main() -> anyhow::Result<()> {
    tera::run()?;
    Ok(())
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
