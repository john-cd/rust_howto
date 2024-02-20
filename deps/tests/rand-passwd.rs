use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;

#[test]
fn test() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}
