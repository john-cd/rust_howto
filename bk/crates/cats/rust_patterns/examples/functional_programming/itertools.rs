#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates the usage of the `itertools` crate.
//!
//! `itertools` provides extra iterator adaptors, functions and macros that
//! expand the `Iterator` trait.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! itertools = "0.14.0"
//! ```
use itertools::Itertools;
use itertools::assert_equal;
use itertools::chain;

fn main() {
    // `assert_equal`:
    // This function asserts that two iterables produce equal sequences.
    assert_equal("hello world".split(' '), "hello world".split(' '));

    // `chain` takes two iterables and creates a new iterator over both in
    // sequence.
    let mut result: Vec<i32> = Vec::new();
    for element in chain(&[1, 2, 3], &[4]) {
        result.push(*element);
    }
    println!("Result: {result:?}");
    assert_eq!(result, vec![1, 2, 3, 4]);

    // `cloned` creates an iterator that clones the elements of the original
    // iterator. Here, it's used to clone the bytes of the byte string
    // "abc" and check if the first element is 'a'.
    assert_eq!(itertools::cloned(b"abc").next(), Some(b'a'));

    // `dedup` removes consecutive duplicate elements from an iterator.
    // Here, it removes consecutive duplicates from the vector [1., 1., 2., 3.,
    // 3., 2., 2.].
    let data = vec![1., 1., 2., 3., 3., 2., 2.];
    itertools::assert_equal(data.into_iter().dedup(), vec![1., 2., 3., 2.]);

    // `into_group_map` groups elements of an iterator into a map based on a
    // key.
    let data = vec![(0, 10), (2, 12), (3, 13), (0, 20), (3, 33), (2, 42)];
    let lookup = data.into_iter().into_group_map();

    assert_eq!(lookup[&0], vec![10, 20]);
    assert_eq!(lookup.get(&1), None);
    assert_eq!(lookup[&2], vec![12, 42]);
    assert_eq!(lookup[&3], vec![13, 33])

    // `itertools` also offers `all`, `any`, `concat`, `fold`, "join",
    // `partition`, `sorted`, etc.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
