#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates a basic usage of the `smartcore` crate for
// machine learning tasks. //!
// //! `smartcore` provides various algorithms for clustering, classification,
// // regression, and more.
// //!
// //! The following includes:
// //!
// //! 1.  Generating a synthetic dataset.
// //! 2.  Fitting a linear regression model to the dataset.
// //! 3.  Making predictions using the fitted model.
// //! 4.  Calculating the mean squared error to evaluate the model's
// performance.

// use rand::Rng;
// use smartcore::linalg::basic::matrix::DenseMatrix;
// use smartcore::linear::linear_regression::LinearRegression;
// use smartcore::metrics::mean_squared_error;
// // KNNClassifier
// // use smartcore::neighbors::knn_classifier::*;
// // Various distance metrics
// // use smartcore::metrics::distance::*;

// fn main() {
//     // Generate a dataset.
//     let x: DenseMatrix<f64> = make_blobs(100, 1, 1.0, &vec![1.0, 2.0]);
//     let y = x.iter().map(|col| col[0] * 3.0 + 1.0).collect::<Vec<f64>>();

//     // Fit a linear regression model to the dataset.
//     let lr = LinearRegression::fit(&x, &y, Default::default()).unwrap();

//     // Make predictions.
//     let y_hat = lr.predict(&x).unwrap();

//     // Calculate the mean squared error.
//     let mse = mean_squared_error(&y, &y_hat);

//     println!("Predictions: {y_hat:?}");
//     println!("Mean Squared Error: {mse}");
// }

// /// The `make_blobs` function generates random samples around the specified
// /// cluster centers, creating a dataset with the specified number of samples
// /// and feature.
// /// `n_samples` is the number of samples to generate.
// /// `n_features` is the number of features for each sample.
// /// `cluster_std` is the standard deviation of the clusters.
// /// `centers` is a vector of the center points for the clusters.
// pub fn make_blobs(
//     n_samples: usize,
//     n_features: usize,
//     cluster_std: f64,
//     centers: &Vec<f64>,
// ) -> DenseMatrix<f64> {
//     let mut rng = rand::rng()();
//     let n_centers = centers.len() / n_features;
//     let mut data = vec![0.0; n_samples * n_features];

//     for i in 0..n_samples {
//         let center = &centers
//             [(i % n_centers) * n_features..(i % n_centers + 1) * n_features];
//         for j in 0..n_features {
//             data[i * n_features + j] =
//                 rng.r#gen::<f64>() * cluster_std + center[j];
//         }
//     }

//     DenseMatrix::from_array(n_samples, n_features, &data)
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [finish; kNN example; syn data generation for linear regr.; 70/30 split; etc](https://github.com/john-cd/rust_howto/issues/837)
