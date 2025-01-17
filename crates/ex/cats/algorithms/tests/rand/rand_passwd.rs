// ANCHOR: example
use rand::Rng;
use rand::distr::Alphanumeric;
use rand::rng;

fn main() {
    let rand_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("{}", rand_string);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
