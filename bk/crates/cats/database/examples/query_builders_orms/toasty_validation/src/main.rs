#[path = "../../toasty.rs"]
mod toasty;

fn main() -> anyhow::Result<()> {
    toasty::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example end to end and asserts that it succeeds.
    #[test]
    fn test_main_runs() {
        assert!(main().is_ok());
    }
}
