//! Runs the file dialog example.
//!
//! TODO Reviewed ??-??-2026.

mod rfd;

/// Runs the file dialogs example.
fn main() -> anyhow::Result<()> {
    rfd::run()
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
