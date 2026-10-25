#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Run a tiny neural-network layer on the CPU with `candle`.
//!
//! Add to `Cargo.toml`:
//! - `candle-core`: tensors and devices.
//! - `candle-nn`: layers and activation functions.

use candle_core::D;
use candle_core::Device;
use candle_core::Result;
use candle_core::Tensor;

/// Computes `softmax(x * w + b)`: a linear layer followed by a classifier.
fn main() -> Result<()> {
    // Choose between `Device::Cpu`, `Device::new_cuda(0)?` and
    // `Device::new_metal(0)?`.
    let device = Device::Cpu;

    // A batch of 2 inputs with 3 features each.
    let x = Tensor::new(&[[1f32, 2., 3.], [4., 5., 6.]], &device)?;
    // Weights map 3 features to 2 classes.
    let w = Tensor::new(&[[0.1f32, -0.1], [0.2, -0.2], [0.3, -0.3]], &device)?;
    let b = Tensor::new(&[0.0f32, 0.0], &device)?;

    // Matrix multiplication, then add the bias (broadcast over the batch).
    let logits = x.matmul(&w)?.broadcast_add(&b)?;
    assert_eq!(logits.dims(), &[2, 2]);

    // Turn logits into probabilities along the class dimension.
    let probabilities = candle_nn::ops::softmax(&logits, D::Minus1)?;
    let rows = probabilities.to_vec2::<f32>()?;
    println!("{rows:?}");

    // Each row of probabilities sums to 1.
    for row in &rows {
        assert!((row.iter().sum::<f32>() - 1.0).abs() < 1e-5);
    }
    // The first class wins, because its weights are positive.
    assert!(rows[0][0] > rows[0][1]);
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main().expect("the candle example failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
