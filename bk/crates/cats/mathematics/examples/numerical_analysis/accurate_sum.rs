//! Compare ordinary summation with Neumaier compensated summation.

// ANCHOR: example
/// Sums finite values while retaining rounding errors in a separate correction.
///
/// Neumaier's algorithm adds the smaller operand back after subtracting the
/// rounded sum from the larger operand. This recovers low-order terms that
/// ordinary summation can lose when magnitudes differ greatly. It uses O(n)
/// time and O(1) extra space; the final result is still subject to f64 rounding.
///
/// # Errors
///
/// Returns an error for nonfinite inputs or a nonfinite corrected result.
fn compensated_sum(values: &[f64]) -> Result<f64, &'static str> {
    let mut sum: f64 = 0.0;
    let mut correction: f64 = 0.0;
    for &value in values {
        if !value.is_finite() {
            return Err("input must be finite");
        }
        let next = sum + value;
        correction += if sum.abs() >= value.abs() {
            (sum - next) + value
        } else {
            (value - next) + sum
        };
        sum = next;
    }
    let result = sum + correction;
    if !result.is_finite() {
        return Err("sum is outside the finite f64 range");
    }
    Ok(result)
}

/// Compares both sums on a cancellation case where ordinary summation loses 1.
fn compare() -> Result<(f64, f64), &'static str> {
    let values = [1.0e16, 1.0, -1.0e16];
    Ok((values.iter().sum(), compensated_sum(&values)?))
}
// ANCHOR_END: example

/// Prints the ordinary and compensated results for the cancellation example.
fn main() -> Result<(), &'static str> {
    let (ordinary, compensated) = compare()?;
    println!("ordinary: {ordinary}, compensated: {compensated}");
    Ok(())
}

/// Verifies recovery of small terms and rejection of nonfinite inputs.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that compensation recovers the small term.
    #[test]
    fn compensation_recovers_the_small_term() {
        assert_eq!(compare(), Ok((0.0, 1.0)));
    }

    /// Rejects NaN and infinity instead of returning an unusable sum.
    #[test]
    fn rejects_nonfinite_input() {
        assert_eq!(
            compensated_sum(&[1.0, f64::NAN]),
            Err("input must be finite")
        );
        assert_eq!(
            compensated_sum(&[f64::INFINITY]),
            Err("input must be finite")
        );
    }
}
