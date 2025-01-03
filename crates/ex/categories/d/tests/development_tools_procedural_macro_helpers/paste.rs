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
    let foo = FooStruct::new(10);
    let bar = BarStruct::new(20);
    let baz = BazStruct::new(30);

    println!("Foo: {}", foo.get_value());
    println!("Bar: {}", bar.get_value());
    println!("Baz: {}", baz.get_value());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/740)
