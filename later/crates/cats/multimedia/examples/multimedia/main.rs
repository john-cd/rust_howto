//! Runs the multimedia example.
//!
//! TODO Reviewed ??-??-2026.

mod multimedia1;

/// Runs the multimedia example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    multimedia1::run()
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
