// TODO Reviewed ??-??-2026.
//! Computes a one-sided probability for the standard normal distribution.

// ANCHOR: example
use statrs::distribution::ContinuousCDF;
use statrs::distribution::Normal;

/// Prints the probability that a standard normal value exceeds 1.96.
fn main() {
    let probability = Normal::standard().sf(1.96);

    assert!((probability - 0.025).abs() < 0.00001);
    println!("P(Z > 1.96) = {probability:.4}");
}
// ANCHOR_END: example

/// Runs the normal-tail recipe from the chapter's example runner.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    /// Checks that the one-sided probability is approximately 2.5%.
    #[test]
    fn normal_tail_is_about_two_and_a_half_percent() {
        super::main();
    }
}
