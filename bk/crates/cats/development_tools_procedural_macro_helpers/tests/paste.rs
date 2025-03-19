// ANCHOR: example
use paste::paste;

// The `paste` macro concatenate identifiers within the `create_structs` macro,
// creating structures and their implementations dynamically.

macro_rules! create_structs {
    ($($name:ident),*) => {
        paste! {
            $(
                struct [< $name Struct >] {
                    value: i32,
                }

                impl [< $name Struct >] {
                    fn new(value: i32) -> Self {
                        Self { value }
                    }

                    fn get_value(&self) -> i32 {
                        self.value
                    }
                }
            )*
        }
    }
}

create_structs!(Foo, Bar, Baz);

fn main() {
    let x = FooStruct::new(10);
    let y = BarStruct::new(20);
    let z = BazStruct::new(30);

    println!("Foo: {}", x.get_value());
    println!("Bar: {}", y.get_value());
    println!("Baz: {}", z.get_value());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
