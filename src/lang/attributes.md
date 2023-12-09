# Attributes

Attributes can take arguments with different syntaxes:

```rust,ignore
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

Inner attributes: `#![attr]`


### Useful module-wide attributes during early coding

```rust,ignore
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
```


## Derive

```rust
// on structs 
#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct S;

fn main() {}
```

## Other Examples

```rust
// Must use the results of the fn
#[must_use]  
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() { println!("{}", add(1, 2)); }
```

```rust
// Disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn main() {}
```

## Conditional Compilation

```rust
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {                  // alternative: use cfg!
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```


## See Also

[Attributes reference]( https://doc.rust-lang.org/reference/attributes.html )

[Rust by example - attributes]( https://doc.rust-lang.org/rust-by-example/attribute.html )
