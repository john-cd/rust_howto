# Attributes

Attributes can take arguments with different syntaxes:

```rust,ignore
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

Inner attributes `#![attr]` apply to the item that the attribute is declared within. 


### Lint attributes

During early development, place the following attributes at the top of `main.rs` or `lib.rs`

```rust,ignore
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
// or simply #[allow(unused)]
#![allow(dead_code)]
#[allow(missing_docs)]
```

For production-ready code, replace the above by the following, for example.

```rust,ignore
#![warn(
    unused,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    rust_2018_idioms,
)]
#![deny(unreachable_pub)]  // error if violation
#![forbid(unsafe_code)]    // same as `deny` +forbids changing the lint level afterwards
```

You also apply these attributes to specific functions:

```rust
// Disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn main() {}
```

List of lint checks: `rustc -W help`. `rustc` also recognizes the tool lints for "clippy" and "rustdoc" e.g. `#![warn(clippy::pedantic)]`


## Automatic trait derivation

```rust
// on structs 
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
struct S(i32);

fn main() {
    println!("{:?}", S(0));
    println!("{}", S(1) == S(1));
}
```


## Must Use

```rust
// Must use the results of the fn; also applies to traits, structs, enums...
#[must_use]  
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() { println!("{}", add(1, 2)); }
```


## Deprecated

```rust,ignore
#[deprecated(since = "5.2.0", note = "use bar instead")]
pub fn foo() {}
```


## Conditional Compilation

[Conditional compilation]( https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute )

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
