#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A `BTreeMap`, an ordered map, storing book ratings.

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

/// Populates a new `BTreeMap` with sample book ratings.
fn populate_map() -> BTreeMap<&'static str, u32> {
    let mut book_ratings = BTreeMap::new();
    book_ratings.insert("The Hitchhiker's Guide to the Galaxy", 3);
    book_ratings.insert("Pride and Prejudice", 4);
    book_ratings.insert("1984", 5);
    book_ratings.insert("To Kill a Mockingbird", 4);
    book_ratings.insert("Dune", 4);
    book_ratings
}

/// Queries and modifies ratings in a `BTreeMap`.
fn query_and_modify(book_ratings: &mut BTreeMap<&'static str, u32>) {
    if let Some(rating) = book_ratings.get("1984") {
        println!("Rating for 1984: {rating}");
    }

    if book_ratings.contains_key("Pride and Prejudice") {
        println!("Pride and Prejudice is in the map.");
    }

    if let Some(rating) = book_ratings.remove("Dune") {
        println!("Removed Dune, rating was {rating}");
    }

    let book_title = "The Lord of the Rings";
    let new_rating = 5;
    match book_ratings.entry(book_title) {
        Entry::Occupied(mut occupied) => {
            *occupied.get_mut() = new_rating;
        }
        Entry::Vacant(vacant) => {
            vacant.insert(new_rating);
        }
    }

    book_ratings
        .entry("The Hitchhiker's Guide to the Galaxy")
        .and_modify(|e| *e += 1)
        .or_insert(5);
}

/// Queries range and clears the map.
fn range_and_clear(book_ratings: &mut BTreeMap<&'static str, u32>) {
    println!("\nRatings for books between 'P' and 'R':");
    for (book, rating) in book_ratings.range("P".."R") {
        println!("{book}: {rating}");
    }

    println!("\nFirst entry: {:?}", book_ratings.first_key_value());
    println!("Last entry: {:?}", book_ratings.last_key_value());

    book_ratings.clear();
    println!("Map is empty: {}", book_ratings.is_empty());
}

/// Creates, fills, queries and iterates over a `BTreeMap`.
fn main() {
    let mut book_ratings = populate_map();

    println!("Book ratings:");
    for (book, rating) in &book_ratings {
        println!("{book}: {rating}");
    }

    query_and_modify(&mut book_ratings);
    range_and_clear(&mut book_ratings);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the btreemap example through its entry point.
    #[test]
    fn test_btreemap_example_entry_point() {
        main();
    }

    #[test]
    fn test_populate_map() {
        let map = populate_map();
        assert_eq!(map.len(), 5);
        assert_eq!(map.get("1984"), Some(&5));
    }

    #[test]
    fn test_query_and_modify() {
        let mut map = populate_map();
        query_and_modify(&mut map);
        assert_eq!(map.get("Dune"), None);
        assert_eq!(map.get("The Lord of the Rings"), Some(&5));
        assert_eq!(map.get("The Hitchhiker's Guide to the Galaxy"), Some(&4));
    }

    #[test]
    fn test_range_and_clear() {
        let mut map = populate_map();
        range_and_clear(&mut map);
        assert!(map.is_empty());
    }
}
