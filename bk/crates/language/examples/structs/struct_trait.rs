#![allow(dead_code)]
// ANCHOR: example
// First, define a trait (if it's not a pre-existing one):
trait SomeBehavior {
    // Method declaration (signature only, no body):
    fn do_something(&self) -> String;

    // Method with default implementation:
    fn another_action(&self, value: i32) {
        println!("The default implementation received the value: {value}");
    }

    // Associated function. In this case, it references the associated type
    // defined below. Note the path syntax `Self::<AssociatedType>`.
    fn assoc_func() -> Self::Output;

    // Associated type:
    type Output;

    // Associated constants with and without default:
    const CONSTANT: i32 = 42;
    const ANOTHER_CONST: &str;
}

// Define your struct:
struct MyStruct {
    data: i32,
    name: String,
}

// Implement the trait for your struct:
impl SomeBehavior for MyStruct {
    // Define the associated type:
    type Output = String;

    // Define the const that did not have a default:
    const ANOTHER_CONST: &str = "Another constant value";
    // Override the const that did have a default:
    const CONSTANT: i32 = 43;

    // Concrete implementation of a trait method for this struct:
    fn do_something(&self) -> String {
        format!("{} is doing something with data: {}", self.name, self.data)
    }

    // Override the default implementation of a method:
    fn another_action(&self, value: i32) {
        println!("{} received value: {}", self.name, value);
    }

    // Implement the associated function:
    fn assoc_func() -> Self::Output {
        "Associated function output".to_string()
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

    println!("{}", MyStruct::assoc_func());
    println!("{}", MyStruct::CONSTANT);
    println!("{}", MyStruct::ANOTHER_CONST);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
