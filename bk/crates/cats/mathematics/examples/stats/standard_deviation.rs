// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Computing the mean and the standard deviation of a list of integers.

/// Calculates the mean (average) of a slice of i32 integers.
///
/// # Arguments
///
/// * `data` - A slice of i32 integers.
///
/// # Returns
///
/// Returns `Some(f64)` containing the mean if the slice is not empty, otherwise
/// returns `None`.
fn mean(data: &[i32]) -> Option<f64> {
    let sum = data.iter().map(|&value| f64::from(value)).sum::<f64>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

/// Calculates the standard deviation of a slice of i32 integers.
///
/// # Arguments
///
/// * `data` - A slice of i32 integers.
///
/// # Returns
///
/// Returns `Some(f64)` containing the standard deviation if the slice is not
/// empty, otherwise returns `None`.
fn std_deviation(data: &[i32]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - f64::from(*value);

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

/// Prints the mean and standard deviation of a list.
fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let data_mean = mean(&data);
    println!("Mean is {data_mean:?}");
    assert!(matches!(data_mean, Some(value) if (value - 5.4).abs() < 1e-12));

    let data_std_deviation = std_deviation(&data);
    println!("Standard deviation is {data_std_deviation:?}");
    assert!(
        matches!(data_std_deviation, Some(value) if (value - 3.6110942).abs() < 1e-6)
    );

    let zscore = match (data_mean, data_std_deviation) {
        (Some(mean), Some(std_deviation)) => {
            let diff = f64::from(data[4]) - mean;

            Some(diff / std_deviation)
        }
        _ => None,
    };
    println!(
        "Z-score of data at index 4 (with value {}) is {zscore:?}",
        data[4]
    );
    assert!(matches!(zscore, Some(value) if (value + 0.11076978).abs() < 1e-6));
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calculates the mean, standard deviation, and z-score for the example
    /// data.
    /// Runs the standard deviation example through its entry point.
    #[test]
    fn test_standard_deviation_example_entry_point() {
        main();
    }
}
