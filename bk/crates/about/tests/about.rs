// ANCHOR: example
use rand::Rng;
fn main() {
    let mut rng = rand::rng();
    println!("Random f64: {}", rng.random::<f64>());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
