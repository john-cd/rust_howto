// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Pipeline architecture: stages connected by channels, each on its own thread.

use std::sync::mpsc;
use std::thread;

/// Parses and squares input concurrently, returning the collected results.
fn run_pipeline(input: Vec<&'static str>) -> Vec<i64> {
    let (raw_tx, raw_rx) = mpsc::sync_channel::<&str>(8);
    let (num_tx, num_rx) = mpsc::sync_channel::<i64>(8);
    let (sq_tx, sq_rx) = mpsc::sync_channel::<i64>(8);

    // Stage 1: parse text, dropping invalid input.
    let parse = thread::spawn(move || {
        for text in raw_rx {
            if let Ok(n) = text.parse::<i64>() {
                num_tx.send(n).expect("next stage is gone");
            }
        }
    });

    // Stage 2: transform.
    let square = thread::spawn(move || {
        for n in num_rx {
            sq_tx.send(n * n).expect("next stage is gone");
        }
    });

    // Produce input concurrently so a full output queue cannot block the
    // producer before the main thread begins receiving results.
    let producer = thread::spawn(move || {
        for text in input {
            raw_tx.send(text).expect("pipeline is gone");
        }
    });

    // Stage 3: collect the results (here on the main thread).
    let results: Vec<i64> = sq_rx.iter().collect();
    producer.join().expect("producer stage panicked");
    parse.join().expect("parse stage panicked");
    square.join().expect("square stage panicked");
    results
}

/// Demonstrates parsing, transformation, and collection through the pipeline.
fn main() {
    let results = run_pipeline(vec!["1", "2", "oops", "3"]);
    assert_eq!(results, vec![1, 4, 9]);
    println!("{results:?}");
}
// ANCHOR_END: example

/// Runs the bounded-channel pipeline example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the pipeline example through its entry point.
    #[test]
    fn test_pipeline_example_entry_point() {
        main();
    }

    /// Checks that input larger than the buffers completes in order.
    #[test]
    fn processes_more_items_than_channel_capacity() {
        let results = run_pipeline(vec!["2"; 100]);
        assert_eq!(results, vec![4; 100]);
    }
}
