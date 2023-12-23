# Functions

```rust,editable
fn foo(x: i32, unit_label: char) -> i32 {
    let y = {
        let z = 3;
        x + z       // expression at the end of a block - no semi-colon
    };

    println!("The value of y is: {y}{unit_label}");
    y               // returns y - no semi-colon
}

fn main() {}
```

The unit type `()` (void in some languages) is the default return type when no type is given for a function.
It could be omitted: `fn log(message: &str) { ... }`

## Generic functions

```rust,editable
fn generic<T>(_s: T) {}

// Explicitly specified type parameter `char` to `generic()`. Note the turbofish notation ::<>
fn main() {
    generic::<char>('a');
}
```

```rust,editable
fn generic<T: ?Sized>(t: &T) {  // By default, generic functions will work only on types that have a known size at compile time. Use ?Sized to relax that.
    // t must be some kind of pointer: &, Rc, Box...
}

fn main() {}
```

## Function pointers

```rust,editable
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {   // function pointer - see also Closures
    f(arg) + f(arg)
}

fn main() {}
```

## Diverging functions

Diverging functions never return.

```rust,editable,ignore
fn foo() -> ! {  // empty type
    panic!("This call never returns.");
}

fn main() {}
```
