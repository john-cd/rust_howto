//! Runs the clipboard example.
//!
//! TODO Reviewed ??-??-2026.

mod arboard;

/// Runs the clipboard example.
fn main() -> anyhow::Result<()> {
    arboard::run()
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
