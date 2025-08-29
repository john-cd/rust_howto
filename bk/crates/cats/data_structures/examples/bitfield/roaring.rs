// ANCHOR: example
use roaring::RoaringBitmap;

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
    println!("Online users: {:?}", online_users);
    println!("Number of online users: {}", online_users.len());

    // Users with IDs 5, 1000, 2000 are subscribed.
    subscribed_users.insert(5);
    subscribed_users.insert(1000);
    subscribed_users.insert(2000);
    println!("Subscribed users: {:?}", subscribed_users);

    // Perform a set operation: union.
    // The '|' operator calculates the union of the two sets.
    // This finds all users who are either online OR subscribed.
    let all_relevant_users = &online_users | &subscribed_users;
    println!("All relevant users (union): {:?}", all_relevant_users);

    // Check for the presence of specific user IDs.
    let user_id_to_check = 99;
    if all_relevant_users.contains(user_id_to_check) {
        println!("User {} is in the relevant set.", user_id_to_check);
    }

    // Intersection (&): users who are both online AND subscribed.
    let active_and_subscribed = &online_users & &subscribed_users;
    println!(
        "Active and subscribed (intersection): {:?}",
        active_and_subscribed
    );

    // Difference (-): users who are online BUT NOT subscribed.
    let online_only = &online_users - &subscribed_users;
    println!("Online but not subscribed (difference): {:?}", online_only);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
