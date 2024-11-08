// ANCHOR: example
#![allow(clippy::async_yields_async)]
use anyhow::Result;
use futures::future::FutureExt;

#[tokio::main]
async fn main() -> Result<()> {
    let future_of_1 = async { 1 };

    // Map this future’s output to a (possibly) different type, returning
    // a new future of the resulting type.
    let new_future = future_of_1.map(|x| x + 3);

    // Chain on a computation for when a future finished, passing the
    // result of the future to the provided closure f.
    let future_of_7 = new_future.then(|x| async move { x + 3 });
    assert_eq!(future_of_7.await, 7);

    // Conditional `Either` future
    let x = 6;
    let future = if x > 10 {
        async { true }.left_future()
    } else {
        async { false }.right_future()
    };
    assert!(!(future.await));

    // Flatten nested futures
    let nested_future = async { async { 1 } };
    let future = nested_future.flatten();
    assert_eq!(future.await, 1);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
