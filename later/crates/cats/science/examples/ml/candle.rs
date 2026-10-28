// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Train a linear model on synthetic data with Candle on the CPU.
//!
//! The example needs no downloaded model and uses the optional candle feature
//! of the science crate.

use candle_core::Device;
use candle_core::Result;
use candle_core::Tensor;
use candle_nn::Module;
use candle_nn::Optimizer;
use candle_nn::SGD;
use candle_nn::VarBuilder;
use candle_nn::VarMap;
use candle_nn::linear;
use candle_nn::loss;

/// Trains a one-layer model and returns the initial and final mean squared
/// loss.
///
/// # Errors
///
/// Returns an error if Candle cannot allocate tensors, run the model, or update
/// its parameters.
fn training_losses() -> Result<(f32, f32)> {
    let device = Device::Cpu;
    let inputs: Vec<f32> =
        (-10..=10).map(|value| value as f32 / 10.0).collect();
    let targets: Vec<f32> =
        inputs.iter().map(|value| 2.0 * value + 1.0).collect();
    let input = Tensor::from_vec(inputs.clone(), (inputs.len(), 1), &device)?;
    let target = Tensor::from_vec(targets, (inputs.len(), 1), &device)?;

    let variables = VarMap::new();
    let builder =
        VarBuilder::from_varmap(&variables, candle_core::DType::F32, &device);
    let model = linear(1, 1, builder)?;
    let mut optimizer = SGD::new(variables.all_vars(), 0.1)?;

    let initial_loss =
        loss::mse(&model.forward(&input)?, &target)?.to_scalar::<f32>()?;
    for _ in 0..250 {
        let prediction = model.forward(&input)?;
        let current_loss = loss::mse(&prediction, &target)?;
        optimizer.backward_step(&current_loss)?;
    }
    let final_loss =
        loss::mse(&model.forward(&input)?, &target)?.to_scalar::<f32>()?;

    Ok((initial_loss, final_loss))
}

/// Trains the synthetic linear regression model on the CPU.
fn main() -> Result<()> {
    let (initial_loss, final_loss) = training_losses()?;
    println!("Mean squared loss: {initial_loss:.4} -> {final_loss:.6}");
    assert!(
        final_loss < initial_loss,
        "training should reduce loss from {initial_loss} to {final_loss}"
    );
    assert!(
        final_loss < 0.01,
        "expected loss below 0.01 after training, got {final_loss}"
    );
    Ok(())
}
// ANCHOR_END: example

pub fn run() {
    main().expect("the Candle CPU training example failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that CPU training reduces the loss on synthetic data.
    #[test]
    fn cpu_training_reduces_synthetic_regression_loss() -> Result<()> {
        main()
    }
}
