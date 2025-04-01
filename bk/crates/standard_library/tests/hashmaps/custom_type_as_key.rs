// ANCHOR: example
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
// k1 == k2 -> hash(k1) == hash(k2)
impl Hash for Student {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn main() {
    let mut grades = HashMap::new();

    // Create some students
    let alice = Student { id: 1 };
    let bob = Student { id: 2 };

    // Insert grades
    grades.insert(alice, 95);
    grades.insert(bob, 87);

    // Look up a student's grade
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
