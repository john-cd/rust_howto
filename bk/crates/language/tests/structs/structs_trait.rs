// ANCHOR: example
// First, define a trait (if it's not a pre-existing one):
trait SomeBehavior {
    fn do_something(&self) -> String; // Method signature
    fn another_action(&self, value: i32);
}

// Define your struct:
struct MyStruct {
    data: i32,
    name: String,
}

// Implement the trait for your struct:
impl SomeBehavior for MyStruct {
    fn do_something(&self) -> String {
        // Concrete implementation for MyStruct
        format!("{} is doing something with data: {}", self.name, self.data)
    }

    fn another_action(&self, value: i32) {
        println!("{} received value: {}", self.name, value);
    }
}

fn main() {
    let m = MyStruct {
        data: 42,
        name: "Alice".to_string(),
    };
    // Use the `.` operator to call a method on a type that implements it:
    println!("{}", m.do_something());
    m.another_action(10);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
