#![allow(dead_code)]
// ANCHOR: example
//! `static` Example.

// Statics are in SCREAMING_SNAKE_CASE, must declare their type, must be
// initialized:
static APPLICATION_NAME: &str = "My Awesome App";
static VERSION: u32 = 1;

fn main() {
    println!("Welcome to {} v{}", APPLICATION_NAME, VERSION);

    // Statics do not `Drop`.
    static POD: PrintOnDrop = PrintOnDrop("This message does not appear.");
}

// A struct that implements a `Drop` destructor.
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
