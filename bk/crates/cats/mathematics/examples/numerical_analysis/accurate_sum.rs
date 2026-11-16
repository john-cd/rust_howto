//! Compare ordinary summation with Neumaier compensated summation.

// ANCHOR: example
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

fn compare() -> Result<(f64, f64), &'static str> {
    let values = [1.0e16, 1.0, -1.0e16];
    Ok((values.iter().sum(), compensated_sum(&values)?))
}
// ANCHOR_END: example

fn main() -> Result<(), &'static str> {
    let (ordinary, compensated) = compare()?;
    println!("ordinary: {ordinary}, compensated: {compensated}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compensation_recovers_the_small_term() {
        assert_eq!(compare(), Ok((0.0, 1.0)));
    }

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
