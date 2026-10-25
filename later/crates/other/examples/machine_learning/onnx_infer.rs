#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Run an ONNX model trained elsewhere (here, a scikit-learn logistic
//! regression) with `ort`, and hot-swap the model without a restart.

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
    version: u32,
    session: Mutex<Session>,
}

impl Model {
    fn load(env: &Environment, version: u32, bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            version,
            session: Mutex::new(
                Session::builder(env)?.commit_from_memory(bytes)?,
            ),
        })
    }
}

/// A scoring engine whose model can be replaced while it is in use.
struct Engine {
    env: Environment,
    model: ArcSwap<Model>,
}

impl Engine {
    fn new(version: u32, bytes: &[u8]) -> Result<Self> {
        let env = ort::init().build()?;
        let model = ArcSwap::from_pointee(Model::load(&env, version, bytes)?);
        Ok(Self { env, model })
    }

    /// Replaces the model. Calls in flight finish on the old one.
    fn hot_swap(&self, version: u32, bytes: &[u8]) -> Result<()> {
        self.model.store(Arc::new(Model::load(&self.env, version, bytes)?));
        Ok(())
    }

    /// Returns the model version and the probability of class 1.
    fn score(&self, features: [f32; 2]) -> Result<(u32, f32)> {
        let model = self.model.load();
        let input = Tensor::from_array(([1_usize, 2], features.to_vec()))?;
        let mut session = model
            .session
            .lock()
            .map_err(|_| anyhow!("poisoned session"))?;
        let outputs = session.run(ort::inputs!["X" => input])?;
        let (_shape, probabilities) =
            outputs["probabilities"].try_extract_tensor::<f32>()?;
        Ok((model.version, probabilities[1]))
    }
}

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

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
