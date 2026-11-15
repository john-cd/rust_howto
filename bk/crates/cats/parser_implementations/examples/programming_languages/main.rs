//! Runs the SQL parser example when the `sqlparser` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

/// Contains the sqlparser functionality used by this parent module.
#[cfg(feature = "sqlparser")]
mod sqlparser;
/// Contains the syn functionality used by this parent module.
mod syn;

/// Runs the example if the `sqlparser` feature is enabled.
fn main() -> anyhow::Result<()> {
    syn::run()?;
    #[cfg(feature = "sqlparser")]
    {
        sqlparser::run()?;
    }
    Ok(())
}

/// Verifies that the SQL parser example when the `sqlparser` feature is enabled.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        main().expect("the example should run");
    }
}
