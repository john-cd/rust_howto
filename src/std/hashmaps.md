# Hashmaps

All of the keys must have the same type as each other, and all of the values must have the same type.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // update the value

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key.

    // enumerate
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 
    scores.entry(String::from("Yellow")).or_insert(50); // Adding a Key and Value Only If a Key Isn’t Present
}
```
