// ANCHOR: example
use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::thread_rng;

fn main() {
    let rand_string: String = thread_rng()
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
