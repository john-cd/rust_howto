// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Compressed bitmaps with the `roaring` crate.
//!
//! A roaring bitmap stores a set of integers compactly and supports fast union
//! and intersection. Search engines and databases use it to hold the IDs of
//! documents or rows that match a condition.

use roaring::RoaringBitmap;

/// Builds roaring bitmaps and combines and queries them.
fn main() {
    // Create two bitmaps representing two sets of user IDs:
    // These are highly optimized data structures for storing sets of 32-bit
    // unsigned integers (u32).
    let mut online_users = RoaringBitmap::new();
    let mut subscribed_users = RoaringBitmap::new();

    // Add some user IDs to the bitmaps. Users with IDs 1, 5, 1000 are online.
    // Roaring bitmaps automatically handle compression, making them very
    // memory-efficient for both sparse and dense data.
    online_users.insert(1);
    online_users.insert(5);
    online_users.insert(1000);
    assert_eq!(online_users.len(), 3);
    assert!(online_users.contains(5));
    println!("Online users: {online_users:?}");
    println!("Number of online users: {}", online_users.len());

    // Users with IDs 5, 1000, 2000 are subscribed.
    subscribed_users.insert(5);
    subscribed_users.insert(1000);
    subscribed_users.insert(2000);
    assert_eq!(subscribed_users.len(), 3);
    println!("Subscribed users: {subscribed_users:?}");

    // Perform a set operation: union.
    // The '|' operator calculates the union of the two sets.
    // This finds all users who are either online OR subscribed.
    let all_relevant_users = &online_users | &subscribed_users;
    assert_eq!(all_relevant_users.len(), 4);
    assert!(all_relevant_users.contains(2000));
    println!("All relevant users (union): {all_relevant_users:?}");

    // Check for the presence of specific user IDs.
    let user_id_to_check = 99;
    if all_relevant_users.contains(user_id_to_check) {
        println!("User {user_id_to_check} is in the relevant set.");
    }

    // Intersection (&): users who are both online AND subscribed.
    let active_and_subscribed = &online_users & &subscribed_users;
    assert_eq!(active_and_subscribed.len(), 2);
    assert!(active_and_subscribed.contains(5));
    assert!(active_and_subscribed.contains(1000));
    println!("Active and subscribed (intersection): {active_and_subscribed:?}");

    // Difference (-): users who are online BUT NOT subscribed.
    let online_only = &online_users - &subscribed_users;
    assert_eq!(online_only.len(), 1);
    assert!(online_only.contains(1));
    println!("Online but not subscribed (difference): {online_only:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the roaring example through its entry point.
    #[test]
    fn test_roaring_example_entry_point() {
        main();
    }
}
