// ANCHOR: example
//! Demonstrate using a custom type as the key in a `HashMap`.

use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

// Custom struct that will be used as a key.
#[derive(Debug, PartialEq, Eq)]
struct Student {
    id: u32,
}

// `Hash` and `Eq` traits must be implemented
// in order to use the struct as a `HashMap` key.
// BEWARE: If two keys are equal, their hashes must be equal.
// This is a fundamental requirement for hash-based collections.
// k1 == k2 -> hash(k1) == hash(k2)
impl Hash for Student {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn main() {
    // Create a new HashMap to store student grades.
    let mut grades = HashMap::new();

    // Create some student instances.
    let alice = Student { id: 1 };
    let bob = Student { id: 2 };

    // Insert student grades into the HashMap.
    grades.insert(alice, 95);
    grades.insert(bob, 87);

    // Look up a student's grade using their ID.
    // Note that we create a new Student instance for lookup,
    // but it's considered the same key if the IDs match.
    let lookup_student = Student { id: 1 };
    if let Some(grade) = grades.get(&lookup_student) {
        println!("Student with ID 1 got grade: {}", grade);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
