// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A reader-writer lock from `parking_lot`.

use parking_lot::RwLock;

/// Reads a value concurrently, updates it exclusively, and returns it.
fn value_after_write() -> i32 {
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
        println!("Write lock is dropped here");
    }

    *lock.read()
}

/// Demonstrates simultaneous reads followed by an exclusive update.
fn main() {
    assert_eq!(value_after_write(), 6);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the write lock's update remains after its guard is dropped.
    #[test]
    fn write_lock_publishes_the_updated_value() {
        assert_eq!(value_after_write(), 6);
    }
}
