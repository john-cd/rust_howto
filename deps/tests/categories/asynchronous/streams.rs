// ANCHOR: example
use futures::Stream;
use futures::stream;
use futures::stream::StreamExt;

async fn count_to_five() -> impl Stream<Item = u32> {
    stream::iter(1..=5)
}

#[tokio::main]
async fn main() {
    let mut stream = count_to_five().await;
    // `for` loops are not usable with Streams, but for imperative-style
    // code, `while let` and the `next`/`try_next` functions can be used:
    while let Some(num) = stream.next().await {
        println!("{}", num);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
