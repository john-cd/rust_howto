#![allow(dead_code)]
// ANCHOR: example
use std::collections::LinkedList;

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

    // Print the list (elements are printed in the order they were added):
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
    for element in list.iter_mut() {
        *element *= 2; // Double each element.
    }

    for element in &list {
        println!("{element}");
    }

    // Get the length of the list:
    println!("\nNumber of elements in the list: {}", list.len());

    // Check if the list is empty:
    println!("Is the list empty? {}", list.is_empty());

    // Clear the list:
    list.clear();
    println!("List is empty: {}", list.is_empty());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
