// ANCHOR: example
fn main() {
    use itertools::assert_equal;
    use itertools::chain;
    use itertools::Itertools;

    // Assert that two iterables produce equal sequences
    assert_equal("hello world".split(' '), "hello world".split(' '));

    // Chain
    let mut result: Vec<i32> = Vec::new();

    for element in chain(&[1, 2, 3], &[4]) {
        result.push(*element);
    }
    assert_eq!(result, vec![1, 2, 3, 4]);

    // Cloned
    assert_eq!(itertools::cloned(b"abc").next(), Some(b'a'));

    // Deduplicate
    let data = vec![1., 1., 2., 3., 3., 2., 2.];
    itertools::assert_equal(data.into_iter().dedup(), vec![1., 2., 3., 2.]);

    // `into_group_map`
    let data = vec![(0, 10), (2, 12), (3, 13), (0, 20), (3, 33), (2, 42)];
    let lookup = data.into_iter().into_group_map();

    assert_eq!(lookup[&0], vec![10, 20]);
    assert_eq!(lookup.get(&1), None);
    assert_eq!(lookup[&2], vec![12, 42]);
    assert_eq!(lookup[&3], vec![13, 33])
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO
