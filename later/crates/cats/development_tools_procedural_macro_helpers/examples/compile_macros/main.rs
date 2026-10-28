//! Runs the ahead-of-time macro helper example.

mod watt;

/// Runs the `watt` token-transformation example.
fn main() -> anyhow::Result<()> {
    watt::run()
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
