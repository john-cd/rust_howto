# Closures

```rust
fn find_emails(list: Vec<String>) -> Vec<String> {
    list.into_iter()
        .filter(|s| s.contains("@"))  // closure
        .collect()
}
```

Closure with type annotations:

```rust
let expensive_closure = |num: u32| -> u32 {  // closure can use type annotation. Multiple statements can be enclosed in a block. 
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```    

Closures can capture variables:

- by reference: &T
- by mutable reference: &mut T
- by value: T

They preferentially capture variables by reference and only go lower when required.

To force a move:

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))  // force the closure to take ownership of the values it uses
        .join()
        .unwrap();
}
```

## Closures as input parameters

```rust
// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F)
    where F: FnOnce() {     // The closure takes no input and returns nothing.
    // could also be `Fn` or `FnMut`.
    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}
```

- Fn: the closure uses the captured value by reference (&T)
- FnMut: the closure uses the captured value by mutable reference (&mut T)
- FnOnce: the closure uses the captured value by value (T)

Functions may also be used as arguments.
