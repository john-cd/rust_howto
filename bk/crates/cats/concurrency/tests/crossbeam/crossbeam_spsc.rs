// ANCHOR: example
use std::thread;
use std::time;

use crossbeam_channel::unbounded;

/// Example of using crossbeam's unbounded channel for single-producer,
/// single-consumer (SPSC) communication.
fn main() {
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    })
    .unwrap();
    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
