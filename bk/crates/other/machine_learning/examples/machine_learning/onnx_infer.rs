// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Run an ONNX model trained elsewhere (here, a scikit-learn logistic
//! regression) with `ort`, and hot-swap the model without a restart.
//!
//! ONNX is an open format for trained machine-learning models, so a model built
//! in Python (scikit-learn, PyTorch) can run in Rust. Inference means using a
//! trained model to predict from new data, as opposed to training it. It needs
//! the ONNX Runtime library.

use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Result;
use anyhow::anyhow;
use arc_swap::ArcSwap;
use ort::environment::Environment;
use ort::session::Session;
use ort::value::Tensor;

/// A loaded model. `Session::run` needs `&mut`, so the session is behind a
/// mutex: use one session per worker (or a pool) when you need parallelism.
struct Model {
    /// Protocol version.
    version: u32,

    /// Session value.
    session: Mutex<Session>,
}

impl Model {
    /// Loads an ONNX model from the provided bytes and records its version.
    ///
    /// # Errors
    ///
    /// Returns an error if initializing the ONNX runtime or loading the model fails.
    fn load(env: &Environment, version: u32, bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            version,
            session: Mutex::new(Session::builder(env)?.commit_from_memory(bytes)?),
        })
    }
}

/// A scoring engine whose model can be replaced while it is in use.
struct Engine {
    /// Env value.
    env: Environment,

    /// Model value.
    model: ArcSwap<Model>,
}

impl Engine {
    /// Initializes the ONNX runtime and loads a model from the supplied bytes.
    ///
    /// # Arguments
    ///
    /// * `version` - Model version recorded with the loaded model.
    /// * `bytes` - Serialized ONNX model data to load.
    ///
    /// # Errors
    ///
    /// Returns an error if initializing the ONNX runtime or loading the model fails.
    fn new(version: u32, bytes: &[u8]) -> Result<Self> {
        let env = ort::init().build()?;
        let initial = Model::load(&env, version, bytes)?;
        Self::score_model(&initial, [0.0, 0.0])?;
        let model = ArcSwap::from_pointee(initial);
        Ok(Self { env, model })
    }

    /// Validates a replacement before publishing it; in-flight calls retain the old model.
    ///
    /// # Errors
    ///
    /// Returns an error if the candidate cannot load or score the expected input.
    /// The previously published model remains available on failure.
    fn hot_swap(&self, version: u32, bytes: &[u8]) -> Result<()> {
        let candidate = Arc::new(Model::load(&self.env, version, bytes)?);
        Self::score_model(&candidate, [0.0, 0.0])?;
        self.model.store(candidate);
        Ok(())
    }

    /// Returns the model version and class-1 probability from one model snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error for non-finite input, inference failure, or invalid output.
    fn score(&self, features: [f32; 2]) -> Result<(u32, f32)> {
        let model = self.model.load_full();
        Self::score_model(&model, features)
    }

    /// Validates and scores a model without publishing it to other readers.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid features, runtime failure, or unexpected output.
    fn score_model(model: &Model, features: [f32; 2]) -> Result<(u32, f32)> {
        anyhow::ensure!(
            features.iter().all(|value| value.is_finite()),
            "features must be finite"
        );
        let input = Tensor::from_array(([1_usize, 2], features.to_vec()))?;
        let mut session = model
            .session
            .lock()
            .map_err(|_| anyhow!("poisoned session"))?;
        let outputs = session.run(ort::inputs!["X" => input])?;
        let output = outputs
            .get("probabilities")
            .ok_or_else(|| anyhow!("model has no probabilities output"))?;
        let (shape, probabilities) = output.try_extract_tensor::<f32>()?;
        anyhow::ensure!(
            shape.as_ref() == [1, 2],
            "expected probabilities shaped [1, 2], got {shape:?}"
        );
        anyhow::ensure!(
            probabilities.len() == 2
                && probabilities
                    .iter()
                    .all(|p| p.is_finite() && (0.0..=1.0).contains(p)),
            "invalid class probabilities"
        );
        Ok((model.version, probabilities[1]))
    }
}

/// Runs inference with a loaded ONNX model, then swaps to another model version.
///
/// # Errors
///
/// Returns an error if model initialization, inference, or the model hot swap fails.
fn main() -> Result<()> {
    // Models exported with `skl2onnx` (see `models/make_models.py`).
    let model_a = include_bytes!("models/logreg_a.onnx");
    let model_b = include_bytes!("models/logreg_b.onnx");

    let engine = Engine::new(1, model_a)?;
    let (version, p) = engine.score([1.5, 1.5])?;
    assert_eq!(version, 1);
    assert!((p - 0.942).abs() < 0.01);

    // Deploy a new model version, without restarting:
    engine.hot_swap(2, model_b)?;
    let (version, p) = engine.score([1.5, 1.5])?;
    assert_eq!(version, 2);
    assert!((p - 0.058).abs() < 0.01);
    Ok(())
}
// ANCHOR_END: example

/// Runs inference with a loaded ONNX model, then swaps to another model version.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> Result<()> {
    main()
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the onnx infer example through its entry point.
    #[test]
    fn test_onnx_infer_example_entry_point() -> Result<()> {
        main()
    }
    /// Keeps the previous model available after a replacement fails to load.
    #[test]
    fn rejected_replacement_preserves_the_model() -> Result<()> {
        let engine = Engine::new(1, include_bytes!("models/logreg_a.onnx"))?;
        let before = engine.score([1.5, 1.5])?;
        assert!(engine.hot_swap(2, b"invalid ONNX model").is_err());
        assert_eq!(engine.score([1.5, 1.5])?, before);
        assert!(engine.score([f32::NAN, 0.0]).is_err());
        Ok(())
    }
    /// Rejects a valid ONNX graph whose output name violates the scoring contract.
    #[test]
    fn rejects_incompatible_replacement_before_publication() -> Result<()> {
        let bytes = include_bytes!("models/logreg_a.onnx");
        let engine = Engine::new(1, bytes)?;
        let before = engine.score([1.5, 1.5])?;
        // Renaming every occurrence preserves graph references and protobuf lengths.
        let mut renamed = bytes.to_vec();
        let offsets: Vec<_> = renamed
            .windows(b"probabilities".len())
            .enumerate()
            .filter_map(|(i, value)| (value == b"probabilities").then_some(i))
            .collect();
        assert!(!offsets.is_empty());
        for offset in offsets {
            renamed[offset] = b'P';
        }
        Model::load(&engine.env, 2, &renamed)?;
        assert!(engine.hot_swap(2, &renamed).is_err());
        assert_eq!(engine.score([1.5, 1.5])?, before);
        Ok(())
    }

    /// An acquired snapshot continues scoring with its own version after publication.
    #[test]
    fn old_snapshot_survives_a_successful_swap() -> Result<()> {
        let engine = Engine::new(1, include_bytes!("models/logreg_a.onnx"))?;
        let old = engine.model.load_full();
        let before = Engine::score_model(&old, [1.5, 1.5])?;
        engine.hot_swap(2, include_bytes!("models/logreg_b.onnx"))?;
        assert_eq!(Engine::score_model(&old, [1.5, 1.5])?, before);
        assert_eq!(engine.score([1.5, 1.5])?.0, 2);
        Ok(())
    }
}
