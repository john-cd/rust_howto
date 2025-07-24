
# Operators

{{#include ops.incl.md}}

## Overload Operators like `+`, `-`, `[]` {#operator-overloading}

[![std][c~std~docs~badge]][c~std~docs]

Operator overloading allows you to define how operators like `+`, `-`, `*`, and `/` behave for your custom types. This is done by implementing the corresponding traits from the `std::ops` module.

Common overloaded operators include [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)⮳, [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)⮳ and [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html)⮳.

For example, to overload the `+` operator for a custom type, you would implement the `Add` trait:

```rust,editable
{{#include ../../crates/standard_library/examples/ops/ops.rs:example}}
```

## Add Function-like Behavior to a Custom Type {#function-like-behavior}

The `std::ops` module also provides traits for function-like / closure-like behavior: `Fn`, `FnMut`, and `FnOnce`. You can make your own types callable like functions by implementing these traits. This is _unstable_ and requires nightly Rust:

```rust,editable
#![feature(unboxed_closures, fn_traits)]

struct Adder {
    offset: i32,
}

use std::ops::FnOnce;

impl FnOnce<(i32,)> for Adder {
    type Output = i32;

    extern "rust-call" fn call_once(self, args: (i32,)) -> i32 {
        // args is a 1-tuple: `(i32,)`.
        self.offset + args.0
    }
}
use std::ops::FnMut;

impl FnMut<(i32,)> for Adder {
    extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> i32 {
        self.offset + args.0
    }
}
use std::ops::Fn;

impl Fn<(i32,)> for Adder {
    extern "rust-call" fn call(&self, args: (i32,)) -> i32 {
        self.offset + args.0
    }
}

fn main() {
    let add_five = Adder { offset: 5 };

    // Call the type like a function:
    assert_eq!(add_five(3), 8);
    println!("5 + 3 = {}", add_five(3)); // prints "5 + 3 = 8".
}
```

## Related Topics {#related-topics}

- [[data_structures | Data Structures]].
- [[drop | Drop]].
- [[traits | Traits]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
