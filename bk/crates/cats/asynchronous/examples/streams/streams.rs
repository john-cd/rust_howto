// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Creating and consuming a `Stream` of numbers.
//!
//! A stream is the asynchronous version of an iterator: it yields many values
//! over time, awaiting between them. Examples are lines arriving on a socket,
//! events from a message queue, or the pages of a paginated API.

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the streams example through its entry point.
    #[test]
    fn test_streams_example_entry_point() {
        main();
    }

    #[tokio::test]
    async fn stream_yields_one_to_five_then_ends() {
        let mut s = count_to_five().await;
        let mut got = Vec::new();
        while let Some(n) = s.next().await {
            got.push(n);
        }
        assert_eq!(got, [1, 2, 3, 4, 5]);
        assert!(s.next().await.is_none());
    }
}
