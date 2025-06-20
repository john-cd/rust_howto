// ANCHOR: example
fn main() {
    // Create a `Vec` of tuples.
    let points = vec![(0, 1), (2, 3), (4, 5)];

    // Iterate through the `Vec`. Each element is deconstructed in turn into
    // variables `x` and `y`.
    for (x, y) in points {
        println!("x: {}, y: {}", x, y);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
