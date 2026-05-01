#![allow(dead_code)]
// ANCHOR: example
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

fn main() {
    // Create a new `BTreeMap`:
    let mut book_ratings = BTreeMap::new();

    // Insert some book ratings:
    book_ratings.insert("The Hitchhiker's Guide to the Galaxy", 3);
    book_ratings.insert("Pride and Prejudice", 4);
    book_ratings.insert("1984", 5);
    book_ratings.insert("To Kill a Mockingbird", 4);
    book_ratings.insert("Dune", 4);

    // Print the map (elements are printed in sorted key order):
    println!("Book ratings:");
    for (book, rating) in &book_ratings {
        println!("{book}: {rating}");
    }

    // Get the rating for a specific book:
    if let Some(rating) = book_ratings.get("1984") {
        println!("Rating for 1984: {rating}");
    }

    // Check if a book is in the map:
    if book_ratings.contains_key("Pride and Prejudice") {
        println!("Pride and Prejudice is in the map.");
    }

    // Remove a book and its rating:
    if let Some(rating) = book_ratings.remove("Dune") {
        println!("Removed Dune, rating was {rating}");
    }

    // Iterate over a range of books (lexicographically between "P" and "T"):
    println!("\nRatings for books between 'P' and 'R':");
    for (book, rating) in book_ratings.range("P".."R") {
        println!("{book}: {rating}");
    }

    // Find the first and last entries:
    println!("\nFirst entry: {:?}", book_ratings.first_key_value());
    println!("Last entry: {:?}", book_ratings.last_key_value());

    // Use the `entry` API to efficiently update or insert:
    let book_title = "The Lord of the Rings";
    let new_rating = 5;
    let entry = book_ratings.entry(book_title);
    match entry {
        Entry::Occupied(mut occupied) => {
            let old_rating = occupied.get();
            println!(
                "Book '{book_title}' already exists with rating {old_rating}",
            );
            *occupied.get_mut() = new_rating; // Update in place.
            println!("Updated rating for '{book_title}' to {new_rating}");
        }
        Entry::Vacant(vacant) => {
            vacant.insert(new_rating); // Insert if it doesn't exist.
            println!("Inserted '{book_title}' with rating {new_rating}");
        }
    }
    // Most commonly you will use `or_insert` and related methods:
    book_ratings.entry("The Hitchhiker's Guide to the Galaxy")
     .and_modify(|e| { *e += 1 }) // Provides in-place mutable access to an occupied entry.
     // Ensures a value is in the entry by inserting the default if empty,
     // and returns a mutable reference to the value in the entry.
     .or_insert(5);
    println!("Updated book ratings:");
    for (book, rating) in &book_ratings {
        println!("{book}: {rating}");
    }

    // Clear the map:
    book_ratings.clear();
    println!("Map is empty: {}", book_ratings.is_empty());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
