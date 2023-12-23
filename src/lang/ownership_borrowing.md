# Ownership and Borrowing

## Ownership

- No garbage collector. Ownership instead.
- Each value in Rust has an owner.
- There can only be one owner at a time.

```rust,editable
fn main() {
    let s1 = String::from("hello"); // On the heap
    let s2 = s1;                    // s1 was MOVED into s2 - NOT a shallow copy - Rust invalidates s1
    // ERROR println!("{}, world!", s1);
}
```

When the owner goes out of scope, the value will be dropped.

```rust,editable
fn main() {
    {
    let s = String::from("hello");
    } // variable out of scope - Rust calls `drop`
}
```

Rust will never automatically create “deep” copies of your data. Use `clone`

```rust,editable,ignore
fn main() {
    let s3 = s2.clone();            // Deeply copy the heap data of the String, not just the stack data
}
```

If a type implements the `Copy` trait (stack-only, fixed-size values, like integers, floats, and tuples thereof), variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

```rust,editable
fn main() {
    let x = 5;  // Integer
    let y = x;  // No MOVE

    println!("x = {}, y = {}", x, y);  // OK
}
```

### Borrowing

Passing a variable to a function will move or copy, just as assignment does.
To avoid passing a value along, borrow the value:

```rust,editable
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);        // pass an immutable reference to s1

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, s1 is not dropped.
}
```

Mutable reference

```rust,editable
fn main() {
    let mut s = String::from("hello");
    change(&mut s);

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
}
```

If you have a mutable reference to a value, you can have no other simultaneous references to that value! Functions like a read/write lock.
