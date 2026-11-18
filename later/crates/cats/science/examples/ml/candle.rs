// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Train a linear model on synthetic data with Candle on the CPU.
//!
//! The example needs no downloaded model and uses the optional candle feature
//! of the science crate.

use candle_core::DType;
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

/// Trains a reproducible CPU model and verifies held-out checkpoint predictions.
///
/// # Errors
///
/// Returns an error if Candle cannot allocate tensors, run the model, or update
/// its parameters, or if saving/loading the temporary checkpoint fails.
fn training_losses() -> Result<(f32, f32)> {
    let device = Device::Cpu;
    let inputs: Vec<f32> = (-10..=10).map(|value| value as f32 / 10.0).collect();
    let targets: Vec<f32> = inputs.iter().map(|value| 2.0 * value + 1.0).collect();
    let input = Tensor::from_vec(inputs.clone(), (inputs.len(), 1), &device)?;
    let target = Tensor::from_vec(targets, (inputs.len(), 1), &device)?;

    let mut variables = VarMap::new();
    let builder = VarBuilder::from_varmap(&variables, candle_core::DType::F32, &device);
    let model = linear(1, 1, builder)?;
    // Explicit initialization makes this small CPU demonstration reproducible.
    variables.set_one("weight", Tensor::zeros((1, 1), DType::F32, &device)?)?;
    variables.set_one("bias", Tensor::zeros(1, DType::F32, &device)?)?;
    let mut optimizer = SGD::new(variables.all_vars(), 0.1)?;

    let initial_loss = loss::mse(&model.forward(&input)?, &target)?.to_scalar::<f32>()?;
    for _ in 0..250 {
        let prediction = model.forward(&input)?;
        let current_loss = loss::mse(&prediction, &target)?;
        optimizer.backward_step(&current_loss)?;
    }
    let final_loss = loss::mse(&model.forward(&input)?, &target)?.to_scalar::<f32>()?;

    let held_out = Tensor::new(&[[2.0_f32], [3.0]], &device)?;
    let before_reload = model.forward(&held_out)?.flatten_all()?.to_vec1::<f32>()?;
    let directory = tempfile::tempdir()?;
    let checkpoint = directory.path().join("linear.safetensors");
    variables.save(&checkpoint)?;

    // Build the architecture before loading: VarMap::load updates existing variables.
    let mut restored = VarMap::new();
    let restored_model = linear(
        1,
        1,
        VarBuilder::from_varmap(&restored, DType::F32, &device),
    )?;
    restored.load(&checkpoint)?;
    let after_reload = restored_model
        .forward(&held_out)?
        .flatten_all()?
        .to_vec1::<f32>()?;
    for ((before, after), expected) in before_reload.iter().zip(&after_reload).zip([5.0, 7.0]) {
        if (before - after).abs() > 1e-6 || (after - expected).abs() > 0.01 {
            candle_core::bail!("held-out prediction or checkpoint parity failed");
        }
    }
    directory.close()?;
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

/// Runs the Candle CPU training example.
///
/// # Panics
///
/// Panics if the Candle CPU training example failed.
pub fn run() {
    main().expect("the Candle CPU training example failed");
}

/// Verifies that CPU training reduces the loss on synthetic data.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that CPU training reduces the loss on synthetic data.
    #[test]
    fn cpu_training_reduces_synthetic_regression_loss() -> Result<()> {
        main()
    }
}
