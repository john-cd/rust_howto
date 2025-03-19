// ANCHOR: example
// Define a Node struct to represent a single element in a linked list.
struct Node {
    value: i32,
    // Node is a recursive data type.
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }

    // Recursively traverses the list until it finds the last node
    // (where next is None) and sets its next field to a new Node.
    fn append(&mut self, value: i32) {
        match self.next {
            Some(ref mut next_node) => next_node.append(value),
            None => self.next = Some(Box::new(Node::new(value))),
        }
    }

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

fn main() {
    // The linked list has an unknown number of nodes,
    // thus its size is not fixed.
    // It could not be stored directly on the stack.
    // By using `Box`, which pointer to the heap has a defined size,
    // we can create the `head` local variable on the stack.
    let mut head = Node::new(1);
    head.append(2);
    head.append(3);
    head.append(4);

    head.print(); // Output: 1 -> 2 -> 3 -> 4
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
