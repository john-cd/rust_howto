// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to sample from a normal distribution.
//!
//! The `Normal` struct represents a normal distribution with a given mean and
//! standard deviation. The `sample` method is used to draw a random sample from
//! the distribution.

use rand_distr::Distribution as _;
use rand_distr::Normal;
use rand_distr::NormalError;

/// Sample from a normal distribution with mean 2.0 and standard deviation 3.0:
fn main() -> Result<(), NormalError> {
    let mut rng = rand::rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{v} is from a N(2, 3) distribution");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), NormalError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_distr_example_entry_point() {
        main().expect("the example should succeed");
    }

    #[test]
    fn infinite_standard_deviation_is_rejected() {
        assert!(Normal::new(2.0, f64::INFINITY).is_err());
    }

    #[test]
    fn sample_mean_is_close_to_the_distribution_mean() {
        let mut rng = rand::rng();
        let normal = Normal::new(2.0, 3.0).expect("valid parameters");
        let n = 20_000;
        let mean =
            (0..n).map(|_| normal.sample(&mut rng)).sum::<f64>() / f64::from(n);
        assert!((mean - 2.0).abs() < 0.5, "mean was {mean}");
    }
}
