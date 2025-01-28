// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use candle_core::DType;
// use candle_core::Device;
// use candle_core::Tensor;
// use candle_core::error::Result;
// use candle_nn::Module;
// use candle_nn::Optimizer;
// use candle_nn::VarBuilder;
// use candle_nn::VarMap;
// use candle_nn::Linear;
// use candle_nn::linear;

// // Add to your `Cargo.toml` as needed:
// // candle-core. Core Datastructures and DataTypes.
// // candle-nn. Building blocks for Neural Nets.
// // candle-datasets. Rust access to commonly used Datasets like MNIST.
// // candle-examples. Examples of Candle in Use.
// // candle-onnx. Loading and using ONNX models.
// // candle-pyo3. Access to Candle from Python.
// // candle-transformers. Candle implemntation of many published transformer
// models. // safetensors
// // tokenizers

// const DEVICE: Device = Device::Cpu;

// fn main() -> anyhow::Result<()> {
//     // Define model architecture
//     struct LinearModel {
//         linear: Linear,
//     }

//     // Define a module with the `forward` method using a single argument.
//     impl Module for LinearModel {
//         fn forward(&self, x: &Tensor) -> Result<Tensor> {
//             Ok(self.linear.forward(x)?)
//         }
//     }

//     impl LinearModel {
//         fn parameters(&self) -> Vec<(String, VarBuilder)> {
//             vec![("linear.weight".to_string(), self.linear.weight.clone())]
//         }
//     }

//     // Create model instance
//     let model = LinearModel {
//         linear: linear(1, 1, DType::F32)?,
//     };

//     // Generate some sample data
//     let x_train = Tensor::of_slice(&[1.0f32, 2.0, 3.0], (3, 1), DEVICE)?;
//     let y_train = Tensor::of_slice(&[2.0f32, 4.0, 6.0], (3, 1), DEVICE)?;

//     // Define optimizer
//     let mut optimizer = dyn Optimizer::Adam { lr: 0.01 };

//     // Training loop
//     for epoch in 0..100 {
//         // Forward pass
//         let y_pred = model.forward(&x_train)?;

//         // Compute loss (e.g., Mean Squared Error)
//         let loss = (&(y_pred - &y_train)).powf(2.0)?.mean()?;

//         // Backward pass
//         loss.backward();

//         // Update model parameters
//         optimizer.step(&mut VarMap::from_module(&model));

//         // Print progress (optional)
//         if epoch % 10 == 0 {
//             println!("Epoch: {}, Loss: {}", epoch, loss);
//         }
//     }

//     // Make predictions
//     let x_test = Tensor::of_slice(&[4.0f32], (1, 1), DEVICE)?;
//     let y_pred = model.forward(&x_test)?;

//     println!("Prediction: {:?}", y_pred);

//     Ok(())
// }

// #[test]
// fn test() {

//     let a = Tensor::arange(0f32, 6f32, &Device::Cpu)?.reshape((2, 3))?;
//     let b = Tensor::arange(0f32, 12f32, &Device::Cpu)?.reshape((3, 4))?;
//     let _c = a.matmul(&b)?;

//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/835) review https://huggingface.github.io/candle/index.html
// // https://github.com/ToluClassics/candle-tutorial
