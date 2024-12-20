// ANCHOR: example
use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

// Must be static, not const
static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

fn main() {
    println!("{:?}", GLOBAL_DATA.lock().unwrap());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
