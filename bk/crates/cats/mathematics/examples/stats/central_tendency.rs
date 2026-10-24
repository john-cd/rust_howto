#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calculates the mean of a set of numbers.

/// Calculates the mean of a set of numbers.
fn main() {
    // Sample data set.
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    // Calculate the sum of the data.
    let sum = data.iter().map(|&value| f64::from(value)).sum::<f64>();
    // Get the number of elements in the data set.
    let count = data.len();

    let mean = match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    };

    println!("Mean of the data is {mean:?}");
    assert!(matches!(mean, Some(value) if (value - 5.4).abs() < 1e-12));
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calculates the mean for the example data.
    #[test]
    fn test_main() {
        main();
    }
}
