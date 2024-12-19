// ANCHOR: example
#[tokio::main]
async fn main() {
    // This is running on Tokio. We may not block here.

    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a thread where blocking is fine.
        println!("Inside spawn_blocking");
    });

    blocking_task.await.unwrap();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
