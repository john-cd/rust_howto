//! Runs the INI parser example.
//!
//! TODO Reviewed ??-??-2026.

mod configparser_ini;
mod rust_ini;

/// Runs the ini example.
fn main() -> anyhow::Result<()> {
    configparser_ini::run()?;
    rust_ini::run()
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
