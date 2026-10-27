//! Demonstrates dependency-free numerical integration.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
/// Errors returned by the composite integration methods.
#[derive(Debug, PartialEq, Eq)]
enum IntegrationError {
    /// The lower or upper integration bound is not finite.
    NonFiniteBounds,
    /// The requested number of subintervals is zero.
    ZeroSubintervals,
    /// Simpson's rule was given an odd number of subintervals.
    OddSimpsonSubintervalCount,
    /// The interval width cannot be represented by a finite nonzero step.
    UnrepresentableStep,
    /// The integrand returned a non-finite sample or received a non-finite
    /// point.
    NonFiniteSample,
    /// The weighted sum or final estimate is not finite.
    NonFiniteEstimate,
}

impl std::fmt::Display for IntegrationError {
    /// Formats the integration error for a user-facing diagnostic.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::NonFiniteBounds => "integration bounds must be finite",
            Self::ZeroSubintervals => "the subinterval count must be positive",
            Self::OddSimpsonSubintervalCount => {
                "Simpson's rule requires an even subinterval count"
            }
            Self::UnrepresentableStep => {
                "the step size is not representable as a finite nonzero value"
            }
            Self::NonFiniteSample => {
                "the integrand must produce finite samples at finite points"
            }
            Self::NonFiniteEstimate => "the integral estimate is not finite",
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for IntegrationError {}

/// Validates a finite interval and computes its step size.
fn checked_step(
    lower: f64,
    upper: f64,
    subintervals: usize,
    require_even: bool,
) -> Result<f64, IntegrationError> {
    if !lower.is_finite() || !upper.is_finite() {
        return Err(IntegrationError::NonFiniteBounds);
    }

    if subintervals == 0 {
        return Err(IntegrationError::ZeroSubintervals);
    }

    if require_even && subintervals % 2 != 0 {
        return Err(IntegrationError::OddSimpsonSubintervalCount);
    }

    let step = (upper - lower) / subintervals as f64;
    if !step.is_finite() || (lower != upper && step == 0.0) {
        return Err(IntegrationError::UnrepresentableStep);
    }

    Ok(step)
}

/// Evaluates the integrand and rejects a non-finite point or sample.
fn checked_sample<F>(integrand: &F, point: f64) -> Result<f64, IntegrationError>
where
    F: Fn(f64) -> f64,
{
    if !point.is_finite() {
        return Err(IntegrationError::NonFiniteSample);
    }

    let sample = integrand(point);
    if !sample.is_finite() {
        return Err(IntegrationError::NonFiniteSample);
    }

    Ok(sample)
}

/// Approximates a definite integral with the composite trapezoidal rule.
///
/// The bounds must be finite and `subintervals` must be positive. For a
/// sufficiently smooth integrand, halving the step size reduces the leading
/// truncation error by about four while floating-point error remains small.
fn integrate_trapezoidal<F>(
    integrand: F,
    lower: f64,
    upper: f64,
    subintervals: usize,
) -> Result<f64, IntegrationError>
where
    F: Fn(f64) -> f64,
{
    let step = checked_step(lower, upper, subintervals, false)?;
    if lower == upper {
        return Ok(0.0);
    }

    let mut weighted_sum = checked_sample(&integrand, lower)? / 2.0
        + checked_sample(&integrand, upper)? / 2.0;
    for index in 1..subintervals {
        let point = lower + step * index as f64;
        weighted_sum += checked_sample(&integrand, point)?;
    }

    let estimate = step * weighted_sum;
    if !estimate.is_finite() {
        return Err(IntegrationError::NonFiniteEstimate);
    }

    Ok(estimate)
}

/// Approximates a definite integral with composite Simpson's 1/3 rule.
///
/// The bounds must be finite and `subintervals` must be positive and even.
/// For a sufficiently smooth integrand, halving the step size reduces the
/// leading truncation error by about sixteen while floating-point error remains
/// small.
fn integrate_simpson<F>(
    integrand: F,
    lower: f64,
    upper: f64,
    subintervals: usize,
) -> Result<f64, IntegrationError>
where
    F: Fn(f64) -> f64,
{
    let step = checked_step(lower, upper, subintervals, true)?;
    if lower == upper {
        return Ok(0.0);
    }

    let mut weighted_sum =
        checked_sample(&integrand, lower)? + checked_sample(&integrand, upper)?;
    for index in 1..subintervals {
        let point = lower + step * index as f64;
        let weight = if index % 2 == 0 { 2.0 } else { 4.0 };
        weighted_sum += weight * checked_sample(&integrand, point)?;
    }

    let estimate = (step / 3.0) * weighted_sum;
    if !estimate.is_finite() {
        return Err(IntegrationError::NonFiniteEstimate);
    }

    Ok(estimate)
}

/// Prints trapezoidal and Simpson estimates for the integral of `x²` on `[0,
/// 1]`.
fn main() {
    let integrand = |x: f64| x * x;
    let trapezoidal = integrate_trapezoidal(integrand, 0.0, 1.0, 100)
        .expect("finite bounds and samples should produce a finite trapezoidal estimate");
    let simpson = integrate_simpson(integrand, 0.0, 1.0, 100).expect(
        "finite bounds and samples should produce a finite Simpson estimate",
    );

    println!("∫₀¹ x² dx ≈ {trapezoidal:.8} (trapezoidal)");
    println!("∫₀¹ x² dx ≈ {simpson:.8} (Simpson's rule)");
    println!("exact value: {:.8}", 1.0 / 3.0);
}
// ANCHOR_END: example

/// Runs the example from the book's example workspace.
pub fn run() {
    main();
}

/// Verifies the trapezoidal rule converges for a known quadratic integral.
#[test]
fn trapezoidal_rule_converges_for_a_quadratic() {
    let coarse = integrate_trapezoidal(|x| x * x, 0.0, 1.0, 8)
        .expect("the coarse finite partition should integrate");
    let fine = integrate_trapezoidal(|x| x * x, 0.0, 1.0, 16)
        .expect("the refined finite partition should integrate");
    let exact = 1.0 / 3.0;

    assert!((fine - exact).abs() < (coarse - exact).abs() / 3.9);
}

/// Verifies Simpson's rule integrates a quadratic to floating-point precision.
#[test]
fn simpson_rule_integrates_a_quadratic() {
    let estimate = integrate_simpson(|x| x * x, 0.0, 1.0, 8).expect(
        "Simpson's rule should integrate a quadratic with an even partition",
    );

    assert!((estimate - 1.0 / 3.0).abs() < 1e-14);
}

/// Verifies Simpson's rule rejects an odd subinterval count.
#[test]
fn simpson_rule_requires_an_even_partition() {
    let result = integrate_simpson(|x| x, 0.0, 1.0, 3);

    assert_eq!(result, Err(IntegrationError::OddSimpsonSubintervalCount));
}

/// Verifies the methods reject non-finite input bounds and samples.
#[test]
fn integration_rejects_non_finite_bounds_and_samples() {
    let invalid_bounds = integrate_trapezoidal(|x| x, f64::NAN, 1.0, 2);
    let invalid_sample = integrate_simpson(|_| f64::INFINITY, 0.0, 1.0, 2);

    assert_eq!(invalid_bounds, Err(IntegrationError::NonFiniteBounds));
    assert_eq!(invalid_sample, Err(IntegrationError::NonFiniteSample));
}

/// Verifies swapping finite bounds negates the integral estimate.
#[test]
fn integration_preserves_orientation_of_bounds() {
    let forward = integrate_trapezoidal(|x| x, 0.0, 1.0, 16)
        .expect("the forward interval should integrate");
    let reverse = integrate_trapezoidal(|x| x, 1.0, 0.0, 16)
        .expect("the reversed interval should integrate");

    assert!((forward + reverse).abs() < 1e-15);
}
