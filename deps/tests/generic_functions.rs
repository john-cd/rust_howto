fn generic<T>(_t: T) {
    println!("got t");
}

// Explicitly specified type parameter `char` to `generic()`.
// Note the turbofish notation ::<>
#[test]
fn test() {
    generic::<char>('a');
}
