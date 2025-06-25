#![allow(dead_code)]
// ANCHOR: example
/// This is an asynchronous function that prints "world".
async fn say_world() {
    println!("world");
}

/// This is the main asynchronous function.
/// It demonstrates that calling an async function does not execute its body
/// immediately.
#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}
// Prints:
// hello
// world
// Example from <https://tokio.rs/tokio/tutorial/hello-tokio>
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
