// ANCHOR: example
fn generic<T>(_t: T) {
    println!("In `generic`");
}

// Explicitly specified type parameter `char` to `generic()`.
// Note the turbofish notation ::<>

fn main() {
    generic::<char>('a');
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
