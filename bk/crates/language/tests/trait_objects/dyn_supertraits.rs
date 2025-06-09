// ANCHOR: example
// Supertrait.
trait Message {
    fn message(&self, msg: &str) -> String;
}

// Subtrait.
trait Greet: Message {
    fn greet(&self) -> String {
        self.message("Hello.")
    }
}

// An example struct that implements `Greet`:
struct Person {
    name: String,
}

impl Greet for Person {}

// The struct must also implement the supertrait:
impl Message for Person {
    fn message(&self, msg: &str) -> String {
        format!("{}: {msg}", self.name)
    }
}

// This function takes a trait object.
fn say_hello_and_goodbye(entity: &dyn Greet) {
    println!("{}", entity.greet());

    // Because `Greet` has `Message` as a supertrait,
    // we can also call `Message`'s methods.
    println!("{}", entity.message("Goodbye."));
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };

    // You can directly pass a reference to `person`,
    // because it implements `Greet`.
    say_hello_and_goodbye(&person);

    // You can also explicitly create a trait object reference:
    let greet_obj: &dyn Greet = &person;
    say_hello_and_goodbye(greet_obj);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
