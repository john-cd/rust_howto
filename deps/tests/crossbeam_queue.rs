use crossbeam_queue::ArrayQueue;

#[test]
fn test() {
    let q = ArrayQueue::new(2);
    assert_eq!(q.push('a'), Ok(()));
    assert_eq!(q.push('b'), Ok(()));
    assert_eq!(q.push('c'), Err('c'));
    assert_eq!(q.pop(), Some('a'));
}
