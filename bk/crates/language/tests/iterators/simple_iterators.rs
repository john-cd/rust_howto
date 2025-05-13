// ANCHOR: example
use std::iter;

fn main() {
    // Creates an iterator that yields nothing.
    let mut empty_iter = iter::empty::<i32>();
    assert_eq!(empty_iter.next(), None);

    // Creates an iterator that yields an element exactly once.
    let mut once_iter = iter::once("hello");
    assert_eq!(once_iter.next(), Some("hello"));
    assert_eq!(once_iter.next(), None);

    // Creates an iterator that lazily generates a value exactly once by
    // invoking the provided closure.
    let mut once_with_iter = iter::once_with(|| "world");
    assert_eq!(once_with_iter.next(), Some("world"));
    assert_eq!(once_with_iter.next(), None);

    // Creates a new iterator that endlessly repeats a single element.
    #[allow(clippy::manual_repeat_n)]
    let mut repeat_iter = iter::repeat(5).take(2); // Only take 2 elements of the infinite sequence.
    assert_eq!(repeat_iter.next(), Some(5));
    assert_eq!(repeat_iter.next(), Some(5));
    assert_eq!(repeat_iter.next(), None);

    // Creates a new iterator that repeats a single element a given number of
    // times.
    let mut repeat_n_iter = iter::repeat_n('a', 2);
    assert_eq!(repeat_n_iter.next(), Some('a'));
    assert_eq!(repeat_n_iter.next(), Some('a'));
    assert_eq!(repeat_n_iter.next(), None);

    // Creates a new iterator that repeats elements of type A endlessly by
    // applying the provided closure,
    let mut count = 0;
    let mut repeat_with_iter = iter::repeat_with(|| {
        count += 1;
        count * 2
    });
    assert_eq!(repeat_with_iter.next(), Some(2));
    assert_eq!(repeat_with_iter.next(), Some(4));
    assert_eq!(repeat_with_iter.next(), Some(6));

    // Creates an iterator with the provided closure.
    let mut counter = 0;
    let mut from_fn_iter = iter::from_fn(move || {
        if counter < 2 {
            counter += 1;
            Some(counter)
        } else {
            None
        }
    });
    assert_eq!(from_fn_iter.next(), Some(1));
    assert_eq!(from_fn_iter.next(), Some(2));
    assert_eq!(from_fn_iter.next(), None);

    // Creates a new iterator where each successive item is computed based on
    // the preceding one.
    let mut successors_iter =
        iter::successors(Some(1), |n| Some(n * 2)).take(3);
    assert_eq!(successors_iter.next(), Some(1));
    assert_eq!(successors_iter.next(), Some(2));
    assert_eq!(successors_iter.next(), Some(4));
    assert_eq!(successors_iter.next(), None);

    // Converts the arguments to iterators and zips them.
    let a = [1, 2, 3];
    let b = ['a', 'b'];
    let mut zip_iter = iter::zip(a, b);
    assert_eq!(zip_iter.next(), Some((1, 'a')));
    assert_eq!(zip_iter.next(), Some((2, 'b')));
    assert_eq!(zip_iter.next(), None);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
