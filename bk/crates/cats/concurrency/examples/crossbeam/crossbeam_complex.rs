#![allow(dead_code)]
// ANCHOR: example
use std::thread;
use std::time::Duration;

use crossbeam_channel::bounded;

/// Complex crossbeam channel example with multiple producers, workers, and a
/// sink.
fn main() {
    // Create two bounded channels with a capacity of 1.
    // snd1/rcv1: Used for communication between the producer and workers.
    // snd2/rcv2: Used for communication between the workers and the sink.
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    // Create a new scope for spawning threads.
    crossbeam::scope(|s| {
        // Producer thread:
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // Close the channel - this is necessary to exit
            // the for-loop in the worker.
            drop(snd1);
        });

        // Parallel processing by multiple worker threads:
        for _ in 0..n_workers {
            // Clone the sender for the sink and the receiver from the source.
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            // Spawn worker threads.
            // Each worker receives messages from rcv1, processes them,
            // and sends the results to snd2.
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                // Receive until channel closes.
                for msg in recvr.iter() {
                    println!(
                        "Worker {:?} received {}.",
                        thread::current().id(),
                        msg
                    );
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        // Close the snd2 channel.
        // This is necessary to signal to the sink that no more messages will be
        // sent.
        drop(snd2);

        // Sink: receives and processes messages from the workers.
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
