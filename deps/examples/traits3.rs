use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        println!("* {} *", self); // can use println! because self is guaranteed to implement Display
    }
}

// String implements Display. That would not work otherwise.
impl OutlinePrint for String {}

fn main() {
    String::from("test").outline_print();
}
