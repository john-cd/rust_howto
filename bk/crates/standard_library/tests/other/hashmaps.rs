// ANCHOR: example
// Import the HashMap data structure
// from the standard library's collections module.
use std::collections::HashMap;

fn main() {
    // Create a new, empty HashMap called 'scores'.
    let mut scores = HashMap::new();

    // Insert a key-value pair into the HashMap.
    // The key is "Blue" (a `String`), and the value is 10 (an `i32`).
    scores.insert(String::from("Blue"), 10);

    // Update the value associated with the key "Blue".
    // The previous value will be replaced with 25.
    scores.insert(String::from("Blue"), 25);

    // Get the value associated with 'team_name' from the HashMap.
    let team_name = String::from("Blue");
    // .get() returns an Option<&i32>.
    // .copied() converts it to Option<i32> by copying the value.
    // .unwrap_or(0) returns the value if it exists, or 0 if it doesn't.
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    // Another way to access values:
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Team {} score: {}", team_name, s),
        None => println!("Team {} not found", team_name),
    }

    // Iterate over and print the key-value pairs in the HashMap
    // (in arbitrary order). '&scores' borrows the HashMap immutably.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Check if a key exists
    if scores.contains_key(&String::from("Blue")) {
        println!("Blue team exists");
    }

    // Getting the length
    println!("Number of teams: {}", scores.len());

    // Insert a key-value pair only if the key doesn't already exist.
    // .entry() returns an `Entry` enum, and `or_insert` inserts the value if
    // the key is absent.
    scores.entry(String::from("Yellow")).or_insert(50);

    // Remove a key and return its value if present
    if let Some(removed_value) = scores.remove(&String::from("Blue")) {
        println!("Removed Blue team with score: {}", removed_value);
    }

    // Clear the HashMap
    scores.clear();
    println!("After clearing, number of teams: {}", scores.len());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
