#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A bounded MPMC pipeline distributes messages to worker threads.

use std::thread;

use anyhow::anyhow;
use crossbeam_channel::bounded;

/// Sends four messages through two bounded workers and returns the results.
fn process_messages() -> anyhow::Result<Vec<u32>> {
    let (input_tx, input_rx) = bounded(1);
    let (output_tx, output_rx) = bounded(1);

    let results = thread::scope(|scope| -> anyhow::Result<Vec<u32>> {
        let workers = (0..2)
            .map(|_| {
                let receiver = input_rx.clone();
                let sender = output_tx.clone();
                scope.spawn(move || -> anyhow::Result<()> {
                    while let Ok(message) = receiver.recv() {
                        sender.send(message * 2)?;
                    }
                    Ok(())
                })
            })
            .collect::<Vec<_>>();

        drop(input_rx);
        drop(output_tx);

        let producers = (0_u32..2)
            .map(|producer| {
                let sender = input_tx.clone();
                scope.spawn(move || -> anyhow::Result<()> {
                    for item in 0_u32..2 {
                        sender.send(producer * 2 + item)?;
                    }
                    Ok(())
                })
            })
            .collect::<Vec<_>>();
        drop(input_tx);

        let mut results = output_rx.iter().collect::<Vec<_>>();
        for producer in producers {
            producer.join().map_err(|_| {
                anyhow!("a Crossbeam producer thread panicked")
            })??;
        }
        for worker in workers {
            worker
                .join()
                .map_err(|_| anyhow!("a Crossbeam worker thread panicked"))??;
        }

        results.sort_unstable();
        Ok(results)
    })?;

    Ok(results)
}

/// Runs the pipeline and verifies every input was processed once.
fn main() -> anyhow::Result<()> {
    let results = process_messages()?;
    assert_eq!(results, vec![0, 2, 4, 6]);
    println!("Processed results: {results:?}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the bounded MPMC pipeline example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that every bounded-channel message reaches one worker.
    #[test]
    fn bounded_mpmc_pipeline_processes_every_message() -> anyhow::Result<()> {
        assert_eq!(process_messages()?, vec![0, 2, 4, 6]);
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
