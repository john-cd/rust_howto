# Lifetimes

Prevent dangling references.

```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

let s: &'static str = "I have a static lifetime.";
```

The generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {}
```
Lifetime Annotations in Struct Definitions and methods:

```rust
struct ImportantExcerpt<'a> { 
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {}
```
