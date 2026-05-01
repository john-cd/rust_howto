#![allow(dead_code)]
// ANCHOR: example
//! Example using `Box<T>` for recursive data structures.

/// Represents a single element in a (highly simplified) linked list.
struct Node {
    /// The value stored in this node.
    value: i32,
    /// The next node in the list, or `None` if this is the last node.
    /// Note that `Node` is a recursive data type.
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }

    /// Recursively traverses the list until it finds the last node
    /// (where next is `None`) and sets its next field to a new `Node`.
    fn append(&mut self, value: i32) {
        match self.next {
            Some(ref mut next_node) => next_node.append(value),
            None => self.next = Some(Box::new(Node::new(value))),
        }
    }

    /// Prints the values of the nodes in the list, separated by " -> ".
    fn print(&self) {
        print!("{}", self.value);
        if let Some(ref next_node) = self.next {
            print!(" -> ");
            next_node.print();
        } else {
            println!();
        }
    }
}

// The linked list has an unknown number of nodes, thus its size is not
// fixed. It could not be stored directly on the stack, because the
// compiler needs to know the size of the data type at compile time.
// By using `Box`, which has a defined size on the stack, we can create the
// `head` local variable . The actual `Node` data will be stored on the heap.
fn main() {
    let mut head = Node::new(1);
    head.append(2);
    head.append(3);
    head.append(4);

    head.print(); // Output: 1 -> 2 -> 3 -> 4.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
