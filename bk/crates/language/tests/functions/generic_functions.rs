// ANCHOR: example
fn generic<T>(_t: T) {
    // This is a generic function that can take any type.
    println!("In `generic`");
}

// We explicitly specify the type parameter `char` for `generic()`.
// Note the use of the 'turbofish' notation: `::<>`
fn main() {
    generic::<char>('a');
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
