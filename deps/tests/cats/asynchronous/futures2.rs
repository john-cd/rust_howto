// ANCHOR: example
use futures::join;

async fn foo(i: u32) -> u32 {
    i
}

#[tokio::main]
async fn main() {
    // The `join!` macro polls multiple futures simultaneously, returning
    // a tuple of all results once complete.
    assert_eq!(join!(foo(1), foo(2)), (1, 2));
    // `join!` is variadic, so you can pass any number of futures

    // `join_all` create a future which represents a collection of the
    // outputs of the futures given.
    let futures = vec![foo(1), foo(2), foo(3)];
    assert_eq!(futures::future::join_all(futures).await, [1, 2, 3]);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
