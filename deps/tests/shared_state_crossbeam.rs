use crossbeam_utils::atomic::AtomicCell;

#[test]
fn test() {
    let a = AtomicCell::new(7);
    let v = a.into_inner();

    assert_eq!(v, 7);
}
