# Smart Pointers

{{#include smart_pointers.incl.md}}

- `Rc<T>`{{hi:Rc<T>}} enables multiple owners{{hi:Multiple owners}} of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime{{hi:Borrowing}}.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.{{hi:Smart pointers}}

## `Box` {#box}

[![book-rust-box][book-rust-box-badge]][book-rust-box]{{hi:Box}} [![Rust by example - box][book-rust-by-example-box-badge]][book-rust-by-example-box] [![std][c-std-badge]][c-std]{{hi:std}}

All values in Rust are stack-allocated by default. `Box<T>` allow you to store data on the heap{{hi:Heap}} rather than the stack{{hi:Stack}}. What remains on the stack is the pointer to the heap data.

Boxes provide ownership for this allocation, and drop their contents when they go out of scope. Boxes also ensure that they never allocate more than `isize::MAX` bytes.

The `Box<T>` type is a smart pointer{{hi:Smart pointers}}, because it implements the [`std::ops::Deref`][c-std::ops::Deref]{{hi:std::ops::Deref}}⮳ trait, which allows `Box<T>` values to be treated like a reference. You can use the dereference operator{{hi:Dereference operator}} `*`{{hi:*}} or 'deref coercion' with the `.` operator to retrieve its inner value.

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
{{#include ../../crates/ex/standard_library/tests/other/box1.rs:example}}
```

## `Rc` {#rc}

[![std][c-std-badge]][c-std]{{hi:std}}

The `Rc<T>` type (for "Reference Counted") enables shared ownership of a value.{{hi:Multiple owners}}{{hi:Shared ownership}}

- `Rc` maintains a reference count of the number of owners. You can create additional references to the data using the `clone` method. Cloning an `Rc` only increments the reference count without duplicating the data. When the last owner goes out of scope, the data is automatically cleaned up (dropped).
- If you need mutability, put a `Cell` or `RefCell` inside the `Rc`.
- `Rc` automatically dereferences to `T` (via the `Deref` trait), so you can call `T`’s methods on a value of type `Rc<T>`.
- `Rc` is commonly used in data structures, such as graphs and linked lists, where multiple nodes might need to share ownership of certain nodes or data.

Keep in mind that `Rc` is not thread-safe. For concurrent scenarios, you should use `Arc` (Atomic Reference Counted), which provides similar functionality with thread safety.

```rust,editable
{{#include ../../crates/ex/standard_library/tests/other/rc.rs:example}}
```

Also of note: the `Weak` type, typically obtained via `Rc::downgrade`, allows for non-owning (weak) references to the data. This can help prevent reference cycles that could lead to memory leaks.

## `RefCell` {#refcell}

[![std][c-std-badge]][c-std]{{hi:std}}

Rust memory safety allows (i) several immutable references (`&T`) to an object `T`; or (ii) _one_ mutable reference (`&mut T`). This is enforced _at compile time_. However, sometimes it is required to have multiple references to an object and yet mutate it. `RefCell<T>` (and related types `Cell<T>` and `OnceCell<T>`) have _interior mutability_, a pattern that allows you to mutate data even when there are immutable references to it.

These types are used in scenarios involving shared state within a single thread, like GUI applications, or when creating complex data structures like graphs.

- `RefCell` keeps track of borrowing rules _at runtime_ and ensures that only one mutable or multiple immutable borrows exist at a time.
Attempts to violate borrowing rules (like having multiple mutable borrows) will cause a _panic_ at runtime. Common methods include `borrow`, `borrow_mut`, and `try_borrow`.

```rust,editable
{{#include ../../crates/ex/standard_library/tests/other/refcell.rs:example}}
```

- `RefCell<T>` (and `Cell<T>`, `OnceCell<T>`) do not implement `Sync` and are therefore _single-threaded_. The corresponding `Sync` version of `RefCell<T>` is `RwLock<T>`. Use `Mutex<T>`, `RwLock<T>`, `OnceLock<T>`, or atomic types when working with multiple threads.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

[smart_pointers: review (P1)](https://github.com/john-cd/rust_howto/issues/628)

[![book-rust-rc][book-rust-rc-badge]][book-rust-rc]{{hi:Rc}}

[![book-rust-refcell][book-rust-refcell-badge]][book-rust-refcell]{{hi:RefCell}}

- finish to rewrite Cell, OnceCell

- example: RefCell inside of Rc

## `Cell` {#cell}

`Cell<T>` is a type that provides simple, byte-wise copyable mutability. It is commonly used for types that implement the `Copy` trait, like integers and booleans. `Cell<T>` is used when you need to mutate a value without using a reference or a mutable reference. Common methods include `set`, `get`, and `replace`.

```rust,editable
{{#include ../../crates/ex/standard_library/tests/other/cell.rs:example}}
```

"`Cell<T>` implements interior mutability by moving values in and out of the cell. That is, an `&mut T` to the inner value can never be obtained, and the value itself cannot be directly obtained without replacing it with something else."

<https://doc.rust-lang.org/nightly/core/cell/index.html>

## `OnceCell` {#oncecell}

- The `OnceCell` type provides a way to define a value that will be initialized at most once. It's useful for scenarios where you want lazy initialization without the overhead of thread synchronization.

Link to memory management page and OnceCell section

</div>
