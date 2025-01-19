// ANCHOR: example

// In `Cargo.toml`
// rand = "0.8.5"
// or
// rand = { version = "0.9.0-beta.3", features = [ "thread_rng" ] }

fn main() {
    let n1: u8 = rand::random();
    let n2: u16 = rand::random();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rand::random::<u32>());
    println!("Random i32: {}", rand::random::<i32>());
    println!("Random float: {}", rand::random::<f64>());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
