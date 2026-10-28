//! Runs the `yew` web framework example.
//!
//! TODO Reviewed ??-??-2026.

mod yew;

/// Runs the yew example.
fn main() {
    yew::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
