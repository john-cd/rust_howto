#![allow(dead_code)]
// ANCHOR: example
/// A struct with `name` and `age` fields.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    /// Constructor for creating a new `Person` instance.
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // Sort people by derived natural order (in this case, name and age):
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]
    );
    println!("{people:?}");

    // Sort people by age only:
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]
    );

    // You can also use `sort_by_key`:
    people.sort_by_key(|p| p.age);

    println!("{people:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
