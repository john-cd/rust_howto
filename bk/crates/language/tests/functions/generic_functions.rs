// ANCHOR: example

// This is a generic function that can take any type.
// The type parameter `T` is written between < and >.
// It often can be inferred from the argument passed to the function.
fn generic<T>(t: T) {
    println!("Argument: {t}; type: {}", std::any::type_name::<T>());
}

/// A generic function can take multiple type parameters.
///
/// This function takes two arguments of potentially different types:
fn take_two<T, U>(a: T, b: U) {
    println!(
        "Arguments: {a} {b}; types: {} {}",
        std::any::type_name::<T>(),
        std::any::type_name::<U>()
    );
}

fn main() {
    // The type parameter `T` is inferred to be `char` in this case.
    generic('a');

    // The type parameter `T` is inferred to be `i32` in this case.
    generic(1);

    // We can also explicitly specify the type parameter `char` for `generic()`.
    // Note the use of the 'turbofish' notation: `::<>`
    generic::<char>('a');

    take_two(Some(3), "4");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
