// ANCHOR: example
//! Example of a slice as a function argument.

fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    // Note: immutable (and mutable) slices implement `IntoIterator`.
    // The iterator yields references to the slice elements.
    for &element in slice {
        sum += element;
    }
    sum
}

fn main() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    // `Vec<T>` implements `Deref<Target = [T]>`.
    // Therefore, you can simply pass a `&Vec` to a function
    // that accepts `&[T]`.
    let _ = sum(&vector);

    let slice: &[i32] = &vector[..];
    let _ = sum(slice);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
