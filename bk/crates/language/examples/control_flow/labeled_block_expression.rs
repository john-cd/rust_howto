// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Labeled block expressions, which `break` out with a value.

/// Breaks out of a labeled block with a value.
fn main() {
    // Labeled block expressions must begin with a label, here `'block`.
    let result = 'block: {
        println!("Entering the labeled block.");
        if condition() {
            println!("About to break with value 1");
            // Break expressions within a labeled block expression must have a
            // label.
            break 'block 1;
        }
        if !condition() {
            println!("About to break with value 2");
            break 'block 2;
        }
        println!("About to return with value 3");
        3
    };
    println!("Result: {result}");
}

/// Returns the condition that decides whether to break out of the block.
fn condition() -> bool {
    false
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the labeled block expression example through its entry point.
    #[test]
    fn test_labeled_block_expression_example_entry_point() {
        main();
    }

    /// The condition is always false, so the second `break` is taken.
    #[test]
    fn test_condition_is_false() {
        assert!(!condition());
    }
}
