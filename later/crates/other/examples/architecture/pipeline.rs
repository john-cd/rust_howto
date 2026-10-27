#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Pipeline architecture: stages connected by channels, each on its own thread.

use std::sync::mpsc;
use std::thread;

/// Runs three stages (parse, square, collect) connected by channels.
fn main() {
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

    // Feed the pipeline, then close it by dropping the sender.
    for text in ["1", "2", "oops", "3"] {
        raw_tx.send(text).expect("pipeline is gone");
    }
    drop(raw_tx);

    // Stage 3: collect the results (here on the main thread).
    let results: Vec<i64> = sq_rx.iter().collect();
    parse.join().expect("parse stage panicked");
    square.join().expect("square stage panicked");

    assert_eq!(results, vec![1, 4, 9]);
    println!("{results:?}");
}
// ANCHOR_END: example

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
}
