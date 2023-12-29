fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`. `iter()` only borrows `vec1` and its elements, so they can be used again
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    // `into_iter()` does move `vec2` and its elements, so they cannot be used again
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
}
