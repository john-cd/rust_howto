// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example using `Box<T>` for recursive data structures.

/// Represents a single element in a (highly simplified) linked list.
struct Node {
    /// The value stored in this node.
    value: i32,
    /// The next node in the list, or `None` if this is the last node.
    /// Note that `Node` is a recursive data type.
    next: Option<Box<Self>>,
}

impl Node {
    /// Creates a node without a successor.
    fn new(value: i32) -> Self {
        Self { value, next: None }
    }

    /// Recursively traverses the list until it finds the last node
    /// (where next is `None`) and sets its next field to a new `Node`.
    fn append(&mut self, value: i32) {
        match self.next {
            Some(ref mut next_node) => next_node.append(value),
            None => self.next = Some(Box::new(Self::new(value))),
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
/// Builds a linked list and prints it.
fn main() {
    let mut head = Node::new(1);
    head.append(2);
    head.append(3);
    head.append(4);

    head.print(); // Output: 1 -> 2 -> 3 -> 4.
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_recursive_example_entry_point() {
        main();
    }

    fn values(head: &Node) -> Vec<i32> {
        let mut out = vec![head.value];
        let mut current = &head.next;
        while let Some(node) = current {
            out.push(node.value);
            current = &node.next;
        }
        out
    }

    #[test]
    fn test_new_node_has_no_successor() {
        let node = Node::new(1);
        assert_eq!(node.value, 1);
        assert!(node.next.is_none());
    }

    #[test]
    fn test_append_adds_nodes_at_the_end() {
        let mut head = Node::new(1);
        head.append(2);
        head.append(3);
        head.append(4);
        assert_eq!(values(&head), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_print_does_not_panic_on_long_lists() {
        let mut head = Node::new(0);
        for i in 1..100 {
            head.append(i);
        }
        assert_eq!(values(&head).len(), 100);
        head.print();
    }
}
