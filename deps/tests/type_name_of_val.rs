fn get_iter() -> impl Iterator<Item = i32> {
    [1, 2, 3].into_iter()
}

#[test]
fn test() {
    let iter = get_iter();
    // NEW in Rust 1.76
    let iter_name = std::any::type_name_of_val(&iter);
    let sum: i32 = iter.sum();
    println!("The sum of the `{iter_name}` is {sum}.");
}
