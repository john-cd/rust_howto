#![allow(dead_code)]
// ANCHOR: example
use parking_lot::RwLock;

fn main() {
    // Create a new RwLock with an initial value of 5.
    let lock = RwLock::new(5);

    {
        println!("Many reader locks can be held at once");
        let r1 = lock.read();
        let r2 = lock.read();
        assert_eq!(*r1, 5);
        // Multiple read locks can be held concurrently.
        assert_eq!(*r2, 5);
        println!("Read locks are dropped at this point");
    }

    {
        println!("Only one write lock may be held, however");
        let mut w = lock.write();
        // Only one write lock can be held at a time.
        *w += 1;
        assert_eq!(*w, 6);
        println!("Write lock is dropped here");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
