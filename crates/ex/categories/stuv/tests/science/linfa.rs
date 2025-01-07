// ANCHOR: example
use linfa::dataset::Dataset;
use linfa::traits::Fit;
use linfa_linear::LinearRegression;
use linfa::prelude::Predict;

// `linfa` is a Rust library for machine learning, providing tools for tasks,
// such as clustering, regression, and classification.

// Perform linear regression
fn main() {
    // Create a dataset with features (X) and targets (y)
    let x = ndarray::array![[1.0], [2.0], [3.0], [4.0], [5.0]];
    let y = ndarray::array![1.1, 2.2, 2.8, 3.95, 5.1];

    // Create a dataset
    let dataset = Dataset::new(x, y);

    // Fit a linear regression model
    let model = LinearRegression::default().fit(&dataset).unwrap();

    // Make predictions
    let test_data = ndarray::array![[6.0], [7.0]];
    let predictions = model.predict(&test_data);

    println!("Predictions: {:?}", predictions);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
