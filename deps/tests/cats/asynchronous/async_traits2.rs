// ANCHOR: example
// TODO
#![allow(dead_code)]

trait Container {
    fn items(&self) -> impl Iterator<Item = u8>; // <-- return Impl in a trait
}

struct MyContainer {
    items: Vec<u8>,
}

impl Container for MyContainer {
    fn items(&self) -> impl Iterator<Item = u8> {
        self.items.iter().cloned()
    }
}

fn main() {
    let c = MyContainer {
        items: vec![1, 2, 3],
    };
    for i in c.items {
        println!("{}", i);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
