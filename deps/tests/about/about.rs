// ANCHOR: example
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    println!("Random f64: {}", rng.gen::<f64>());
}
// ANCHOR_END: example
#[test]
fn test() {
    main();
}
