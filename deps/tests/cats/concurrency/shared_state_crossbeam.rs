// ANCHOR: example
use crossbeam_utils::atomic::AtomicCell;

fn main() {
    let a = AtomicCell::new(7);
    let v = a.into_inner();

    assert_eq!(v, 7);
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
