#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Creating and consuming a `Stream` of numbers.

use futures::Stream;
use futures::stream;
use futures::stream::StreamExt as _;

/// Creates a stream that yields the numbers 1 through 5.
async fn count_to_five() -> impl Stream<Item = u32> {
    stream::iter(1..=5)
}

/// Prints each number of the stream.
#[tokio::main]
async fn main() {
    let mut stream = count_to_five().await;
    // `for` loops are not usable with Streams, but for imperative-style
    // code, `while let` and the `next`/`try_next` functions can be used:
    while let Some(num) = stream.next().await {
        println!("{num}");
    }
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
