// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A `LinkedList`, a doubly linked list.
//!
//! A linked list stores each element in a separate node pointing to the next,
//! so inserting in the middle is cheap but reading is slow and
//! cache-unfriendly. In Rust a `Vec` or `VecDeque` is nearly always faster; use
//! a list only when you need cheap splitting and joining.

use std::collections::LinkedList;

/// Adds, removes and iterates over the elements of a `LinkedList`.
fn main() {
    // Create a new `LinkedList`.
    let mut list = LinkedList::new();

    // Push elements onto the front of the list:
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    // Push elements onto the back of the list:
    list.push_back(4);
    list.push_back(5);
    assert_eq!(list.front(), Some(&1));
    assert_eq!(list.back(), Some(&5));
    assert_eq!(list.len(), 5);

    // Print the current list order, from front to back:
    println!("List elements:");
    for element in &list {
        println!("{element}");
    }

    // Get the first and last elements (without removing them):
    println!("\nFirst element: {:?}", list.front());
    println!("Last element: {:?}", list.back());

    // Pop elements from the front of the list:
    println!("\nPopping from front:");
    while let Some(element) = list.pop_front() {
        println!("Popped: {element}");
    }
    assert!(list.is_empty());

    // Add some more elements:
    list.push_front(10);
    list.push_back(20);

    // Pop elements from the back of the list:
    println!("\nPopping from back:");
    while let Some(element) = list.pop_back() {
        println!("Popped: {element}");
    }

    // Add some more elements:
    list.push_front(30);
    list.push_back(40);

    // Iterate and modify elements (requires mutable iterator):
    println!("\nIterating and modifying:");
    for element in &mut list {
        *element *= 2; // Double each element.
    }

    for element in &list {
        println!("{element}");
    }
    assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![60, 80]);

    // Get the length of the list:
    println!("\nNumber of elements in the list: {}", list.len());

    // Check if the list is empty:
    println!("Is the list empty? {}", list.is_empty());

    // Clear the list:
    list.clear();
    println!("List is empty: {}", list.is_empty());
    assert!(list.is_empty());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the linkedlist example through its entry point.
    #[test]
    fn test_linkedlist_example_entry_point() {
        main();
    }
}
