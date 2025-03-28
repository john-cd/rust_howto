// ANCHOR: example
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // Update the value
    scores.insert(String::from("Blue"), 25);

    let team_name = String::from("Blue");
    // Get an Option<i32> rather than an Option<&i32>, then unwrap_or to
    // set score to zero if scores doesn't have an entry for the key.
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    // Enumerate
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Adding a Key and Value only if a Key isn't present
    scores.entry(String::from("Yellow")).or_insert(50);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
