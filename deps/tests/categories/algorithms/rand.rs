// ANCHOR: example
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Note: we use a raw identifier to stay compatible with Rust edition 2024.
    // `gen` is a keyword in the 2024 edition.
    let n1: u8 = rng.r#gen();
    let n2: u16 = rng.r#gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.r#gen::<u32>());
    println!("Random i32: {}", rng.r#gen::<i32>());
    println!("Random float: {}", rng.r#gen::<f64>());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
