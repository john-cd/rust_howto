# Automatic trait derivation

The `derive` attribute generates code that will implement a trait with its own default implementation on the type you’ve annotated with the derive syntax.

[Derivable traits]( https://doc.rust-lang.org/book/appendix-03-derivable-traits.html )

```rust
// on structs 
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
struct S(i32);

fn main() {
    println!("{:?}", S(0));
    println!("{}", S(1) == S(1));
}
```

You can use the `cargo-expand` utility to see the exact code that is generated for your specific type.

## Derive More

[Derive More]( https://crates.io/crates/derive_more ) derive lots of additional, commonly used traits and static methods for both structs and enums.

```rust,ignore
extern crate derive_more;

use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, From, Add)]
struct MyInt(i32);

#[derive(PartialEq, From, Into)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(PartialEq, From, Add, Display)]
enum MyEnum {
    #[display(fmt = "int: {}", _0)]
    Int(i32),
    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

fn main() {
    assert!(MyInt(11) == MyInt(5) + 6.into());
    assert!((5, 6) == Point2D { x: 5, y: 6 }.into());
    assert!(MyEnum::Int(15) == (MyEnum::Int(8) + 7.into()).unwrap());
    assert!(MyEnum::Int(15).to_string() == "int: 15");
    assert!(MyEnum::Uint(42).to_string() == "42");
    assert!(MyEnum::Nothing.to_string() == "nothing");
}
```
