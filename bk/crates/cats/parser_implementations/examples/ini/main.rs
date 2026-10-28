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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
