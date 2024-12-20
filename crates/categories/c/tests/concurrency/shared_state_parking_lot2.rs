// ANCHOR: example
use parking_lot::RwLock;

fn main() {
    let lock = RwLock::new(5);

    {
        println!("Many reader locks can be held at once");
        let r1 = lock.read();
        let r2 = lock.read();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
        println!("Read locks are dropped at this point");
    }

    {
        println!("Only one write lock may be held, however");
        let mut w = lock.write();
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
