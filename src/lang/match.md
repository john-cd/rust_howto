# Match, if let, while let

```rust,ignore
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,   
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {                   // pattern binding to value 
            println!("State quarter from {:?}!", state);
            25
        }
        // if needed, use catchall _ =>
    }
}

fn main() {}
```

```rust
// struct pattern matching
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),  // Ignoring all fields of a Point except for x by using ..
    }
}
```

Patterns accept ` 1 | 2` for or, `1..=5` for inclusive range, `if x % 2 == 0` guards, @-binding `Message::Hello { id: id_variable @ 3..=7,}`.


```rust
fn main() {
    // if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        }
}
```

```rust
fn main() {
    // while let
    let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
}
```

## See Also

[Pattern matching]( https://doc.rust-lang.org/book/ch18-00-patterns.html )
