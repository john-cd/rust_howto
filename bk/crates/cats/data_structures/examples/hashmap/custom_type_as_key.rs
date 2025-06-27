#![allow(dead_code)]
// ANCHOR: example
//! Demonstrate using a custom type as the key in a `HashMap`.

use std::collections::HashMap;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

// Custom struct that will be used as a key.
// `Hash` and `Eq` traits must be implemented in order to use the struct as a
// `HashMap` key. Implement them manually or use the `derive` attribute.
#[derive(Debug, PartialEq, Eq)]
struct Student {
    id: u32,
}

// Manual implementation of `Hash` for demonstration purposes.
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
    let alice2 = Student { id: 1 }; // Same ID.
    let bob = Student { id: 2 };

    // Let's make sure the hashing relationship above stands.
    assert_eq!(alice, alice2);
    assert_eq!(calculate_hash(&alice), calculate_hash(&alice2));

    // Insert student grades into the HashMap.
    grades.insert(alice, 95);
    grades.insert(bob, 87);

    // Look up a student's grade using their ID.
    // Note that we create a new Student instance for lookup,
    // but it's considered the same key if the IDs match.
    let lookup_student = Student { id: 2 };
    if let Some(grade) = grades.get(&lookup_student) {
        println!("Student with ID 2 got grade: {grade}");
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
