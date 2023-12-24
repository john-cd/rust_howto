# Lifetimes

Prevent dangling references.

`&i32`        a reference
`&'a i32`     a reference with an explicit lifetime
`&'a mut i32` a mutable reference with an explicit lifetime

```rust,editable,ignore
let s: &'static str = "I have a static lifetime.";
```

The generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`:

```rust,editable
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let (x,y) = ("short", "looooooong");
    println!("{}", longest(x, y));
}
```

## Lifetime Annotations in Struct Definitions and methods

```rust,editable
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    let ie = ImportantExcerpt { part: "a part" };
    println!("{}", ie.level());
}
```
