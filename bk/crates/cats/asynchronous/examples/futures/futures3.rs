#![allow(dead_code)]
#![allow(clippy::async_yields_async)]
// ANCHOR: example
//! # Futures
//!
//! This example demonstrates various operations on futures using the `futures`
//! crate.
//!
//! It showcases how to:
//! - Map the output of a future to a different type.
//! - Chain computations onto a future using `then`.
//! - Use conditional `Either` futures.
//! - Flatten nested futures.
use anyhow::Result;
use futures::future::FutureExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a future that resolves to the value 1.
    let future_of_1 = async { 1 };

    // Map this future's output to a (possibly) different type, returning
    // a new future of the resulting type.
    let new_future = future_of_1.map(|x| x + 3);

    // Chain on a computation for when a future finished, passing the
    // result of the future to the provided closure f.
    let future_of_7 = new_future.then(|x| async move { x + 3 });
    let seven = future_of_7.await;
    println!("{seven}");
    assert_eq!(seven, 7);

    // Conditional `Either` future:
    let x = 6;
    let future = if x > 10 {
        async { true }.left_future()
    } else {
        async { false }.right_future()
    };
    let not_true: bool = future.await;
    assert!(!not_true);

    // Flatten nested futures:
    let nested_future = async { async { 1 } };
    let future = nested_future.flatten();
    let flat = future.await;
    println!("{flat}");
    assert_eq!(flat, 1);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
