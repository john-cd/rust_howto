// ANCHOR: example
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("{:?}", vec);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
