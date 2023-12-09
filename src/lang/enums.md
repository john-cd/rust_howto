# Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },    // struct-like
    Write(String),              // tuple-like
    ChangeColor(i32, i32, i32),
}

// define methods on enums.
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let home = Message::ChangeColor(127, 0, 0);  // note the :: 
}
```

If we make an enum public, all of its variants are then public. We only need `pub` before the `enum` keyword.


## Option

Rust has no `null`

```rust,ignore
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
}
```
