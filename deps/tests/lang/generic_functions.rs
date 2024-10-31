fn generic<T>(_t: T) {
    println!("got t");
}

// Explicitly specified type parameter `char` to `generic()`.
// Note the turbofish notation ::<>

fn main() {
    generic::<char>('a');
}

#[test]
fn test() {
    main();
}
