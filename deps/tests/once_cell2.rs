use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

// must be static, not const
static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

#[test]
fn test() {
    println!("{:?}", GLOBAL_DATA.lock().unwrap());
}
