# Smart Pointers

{{#include smart_pointers.incl.md}}

- `Rc<T>`{{hi:Rc<T>}} enables multiple owners{{hi:Multiple owners}} of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime{{hi:Borrowing}}.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.{{hi:Smart pointers}}

## `Box` {#box}

[![book-rust-box][book-rust-box-badge]][book-rust-box]{{hi:Box}} [![Rust by example - box][book-rust-by-example-box-badge]][book-rust-by-example-box] [![std][c-std-badge]][c-std]{{hi:std}}

All values in Rust are stack allocated by default. `Box<T>` allow you to store data on the heap{{hi:Heap}} rather than the stack{{hi:Stack}}. What remains on the stack is the pointer to the heap data.

Boxes provide ownership for this allocation, and drop their contents when they go out of scope. Boxes also ensure that they never allocate more than `isize::MAX` bytes.

The `Box<T>` type is a smart pointer{{hi:Smart pointers}}, because it implements the [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ trait, which allows `Box<T>` values to be treated like a reference. You can use the dereference operator{{hi:Dereference operator}} `*`{{hi:*}} or deref coercion with the `.` operator to retrieve its inner value.

```rust,editable
let boxed: Box<u8> = Box::new(1);
let _val: u8 = *boxed;
let boxed = Box::new("example");
// Deref coercion: equivalent to (*boxed.deref()).len()
let _val = boxed.len();
```

Use `Box<T>` when

- you have a dynamically sized type, whose size can’t be known at compile time,
- you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type,
- you don't want to rely on stack space.

```rust,editable
{{#include ../../deps/tests/standard_library/box1.rs:example}}
```

## `Rc` {#rc}

[![std][c-std-badge]][c-std]{{hi:std}}

The `Rc<T>` type keeps track of the number of references{{hi:References}} to data on the heap so that data can have multiple owners{{hi:Multiple owners}}.

```rust,editable
{{#include ../../deps/tests/standard_library/rc.rs:example}}
```

## `RefCell` {#refcell}

[![std][c-std-badge]][c-std]{{hi:std}}

The `RefCell<T>` type with its interior mutability{{hi:Interior mutability}} gives us a type that we can use when we need an immutable type{{hi:Immutable type}} but need to change an inner value{{hi:Inner value}} of that type; it also enforces the borrowing rules at runtime instead of at compile time.

```rust,editable
{{#include ../../deps/tests/standard_library/refcell.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[smart_pointers: review (P1)](https://github.com/john-cd/rust_howto/issues/628)

</div>
