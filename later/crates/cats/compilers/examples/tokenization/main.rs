//! Runs the lexer example.

mod tokenization;

/// Runs the lexer example.
fn main() {
    tokenization::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
