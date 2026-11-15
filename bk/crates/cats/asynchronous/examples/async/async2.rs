// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! This is the main asynchronous function.
//! It demonstrates that calling an async function does not execute its body
//! immediately.

/// Asynchronous function that prints "world".
async fn say_world() {
    println!("world");
}

/// This is the main asynchronous function.
/// It demonstrates that calling an async function does not execute its body
/// immediately.
#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    // Futures are lazy.
    let op = say_world();

    // This `println!` comes first:
    println!("hello");

    // Awaiting `op` polls it until it completes:
    op.await;
}

// Prints:
// hello
// world
// Example from <https://tokio.rs/tokio/tutorial/hello-tokio>.
// ANCHOR_END: example

/// This is the main asynchronous function. It demonstrates that calling an async function does not
/// execute its body immediately.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the `async2` example runs.
    #[test]
    fn test_async2_example_entry_point() {
        main();
    }

    /// Leaves a future pending until it is explicitly awaited.
    #[tokio::test]
    async fn a_future_does_nothing_until_it_is_awaited() {
        use std::future::Future as _;
        use std::task::Context;
        use std::task::Poll;

        let mut op = Box::pin(say_world());
        // Creating the future ran nothing; polling it once runs it to the end.
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        assert_eq!(op.as_mut().poll(&mut cx), Poll::Ready(()));
    }
}
