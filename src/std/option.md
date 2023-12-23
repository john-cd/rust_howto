# Option

Rust has no `null`. Instead, use [Option]( https://doc.rust-lang.org/std/option/ ):

```rust,editable,ignore
enum Option<T> {
    None,
    Some(T),
}
```

Every `Option` is either `Some` and contains a value, or `None`, and does not.

```rust,editable,ignore
fn main() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
}
```

It is often used with `match`, `if let`, or `while let`:

```rust,editable
fn bake_cake(sprinkles: Option<&str>) -> String {
    let mut cake = String::from("A delicious cake base...");

    // Add required ingredients

    // Handle optional sprinkles
    if let Some(sprinkle_choice) = sprinkles {
        cake.push_str(format!(" with a sprinkle of {}", sprinkle_choice).as_str());
    } else {
        // sprinkles is None
        cake.push_str(" ready for your decorating touch!");
    }
    cake
}

fn main() { bake_cake(Some("rainbow nonpareils")); }
```

## Adapters for working with references

- `as_ref` converts from `&Option<T>` to `Option<&T>`
- `as_mut` converts from `&mut Option<T>` to `Option<&mut T>`
- `as_deref` converts from `&Option<T>` to `Option<&T::Target>`
- `as_deref_mut` converts from `&mut Option<T>` to `Option<&mut T::Target>`

## Extracting the value contained in Option

These methods extract the contained value in an Option<T> when it is the Some variant.
If the Option is None:

- `expect` panics with a provided custom message
- `unwrap` panics with a generic message
- `unwrap_or` returns the provided default value
- `unwrap_or_default` returns the default value of the type T (which must implement the `Default` trait)
- `unwrap_or_else` returns the result of evaluating the provided function

## Combinators

```rust,editable
use std::fs;

// `and_then` applies a function to the wrapped value if it's Some.
fn read_file(filename: &str) -> Option<String> {
    fs::read_to_string(filename)
        .ok() // Convert `Result` to `Option`
        .and_then(|contents| Some(contents.trim().to_string())) // Chain operations on `Some`
}

fn main() {
    let contents = read_file("poem.txt");

    // Using `match` to process the returned Option.
    match contents {
        Some(poem) => println!("{}", poem),
        None => println!("Error reading file"),
    }
}
```
