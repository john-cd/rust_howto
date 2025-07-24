#![allow(dead_code)]
// ANCHOR: example
use std::borrow::Borrow;

#[derive(Debug)]
struct UserId(u64);

// We want to borrow a `u64` from `UserId`:
impl Borrow<u64> for UserId {
    fn borrow(&self) -> &u64 {
        &self.0
    }
}

fn main() {
    use std::collections::HashMap;

    let mut login_times: HashMap<u64, String> = HashMap::new();
    login_times.insert(42, "2025-07-23T08:00Z".to_string());

    let uid = UserId(42);
    // Thanks to `Borrow<u64>`, this works:
    if let Some(time) = login_times.get(uid.borrow()) {
        println!("User 42 logged in at {time}");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO
