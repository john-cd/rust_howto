//! Starts the host-side Embassy task example.
// TODO Reviewed ??-??-2026.

mod embassy;

// ANCHOR: example
/// Runs the bounded host example.
fn main() -> anyhow::Result<()> {
    embassy::run()
}
// ANCHOR_END: example


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
