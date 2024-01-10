use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        println!("* {} *", self);
        // We can use `println!` here,
        // because `self` is guaranteed to implement `Display`
    }
}

// String implements Display. That would not work otherwise.
impl OutlinePrint for String {}

fn main() {
    String::from("test").outline_print();
}

// BEWARE: supertrait are NOT inheritance!
