async fn say_world() {
    println!("world");
}

#[tokio::test]
async fn test() {
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
// Example from https://tokio.rs/tokio/tutorial/hello-tokio
