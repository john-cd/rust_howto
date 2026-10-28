#[path = "../../toasty.rs"]
mod toasty;

fn main() -> anyhow::Result<()> {
    toasty::run()
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
