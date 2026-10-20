#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates basic usage of the `winnow` crate.

use winnow::ascii::digit1;
use winnow::combinator::separated;
use winnow::prelude::*;

/// Parses a list of numbers separated by commas.
fn parse_list(input: &mut &str) -> ModalResult<Vec<i32>> {
    separated(1.., digit1.parse_to::<i32>(), ",").parse_next(input)
}

fn main() -> anyhow::Result<()> {
    let mut input = "1,2,3,4,5";
    let result = parse_list(&mut input)
        .map_err(|e| anyhow::anyhow!("Parsing failed: {:?}", e))?;

    println!("Parsed numbers: {:?}", result);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
