#![allow(dead_code)]
// ANCHOR: example
async fn do_something() {
    println!("hello, world!");
}

fn main() {
    let future = do_something();
    // Futures are lazy - nothing is happening
    // until driven to completion by `.await`, `block_on`...

    // `block_on` blocks the current thread until the provided future has
    // run to completion. Other executors provide more complex
    // behavior, like scheduling multiple futures onto the same
    // thread. See `Tokio`.
    futures::executor::block_on(future);
    // `future` is run and "hello, world!" is printed.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
