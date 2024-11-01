// ANCHOR: example
use itertools::chain;
use itertools::Itertools;

fn main() {
    // assert_equal
    // PANIC assert_equal("exceed".split('c'), "excess".split('c'));

    // Chain
    let mut result: Vec<i32> = Vec::new();

    for element in chain(&[1, 2, 3], &[4]) {
        result.push(*element);
    }
    assert_eq!(result, vec![1, 2, 3, 4]);

    // Cloned
    use itertools::cloned;

    assert_eq!(cloned(b"abc").next(), Some(b'a'));

    // dedup
    let data = vec![1., 1., 2., 3., 3., 2., 2.];
    itertools::assert_equal(data.into_iter().dedup(), vec![1., 2., 3., 2.]);

    // into_group_map
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
