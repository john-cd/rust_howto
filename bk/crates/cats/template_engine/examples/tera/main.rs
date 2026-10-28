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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
