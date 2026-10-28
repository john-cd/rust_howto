//! Runs the images examples.
//!
//! TODO Reviewed ??-??-2026.

mod blur;
mod convert;
mod images;
mod resize;

/// Runs the images examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    blur::run();
    convert::run()?;
    resize::run();
    images::run()
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
