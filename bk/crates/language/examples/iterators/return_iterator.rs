#![allow(dead_code)]
// ANCHOR: example
/// Return an iterator from a function using the `impl Iterator<Item = T>`
/// syntax as the return type, where `T` is the type of the items the iterator
/// will yield.
fn count_up_to(max: i32) -> impl Iterator<Item = i32> {
    // In this case, we convert a range into an iterator,
    // then transform it further.
    (0..max).map(|x| x + 1)
}
// You could also return a boxed trait object e.g.,
// `Box<dyn Iterator<Item = T>>`.

fn main() {
    let my_iterator = count_up_to(5);

    for number in my_iterator {
        println!("{}", number);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
