# Smart Pointers

{{#include smart_pointers.incl.md}}

- `{{i:Rc<T>}}` enables {{i:multiple owners}} of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable {{i:borrows checked at runtime}}.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

## Box

[![book-rust-box][book-rust-box-badge]][book-rust-box]  [![std][c-std-badge]][c-std]

`Box<T>` allow you to store data on the {{i:heap}} rather than the {{i:stack}}. What remains on the stack is the pointer to the heap data.

The `{{i:Box}}<T>` type is a {{i:smart pointer}} because it implements the [`{{i:Deref}}`][c-std::ops::Deref]⮳ trait, which allows `Box<T>` values to be treated like references. Implementing the [`{{i:Deref}}`][c-std::ops::Deref]⮳ trait allows you to customize the behavior of the {{i:dereference operator}} `{{i:*}}`.

Use when

- you have a type whose size can’t be known at compile time
- you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type.

## Rc

[![std][c-std-badge]][c-std]

The `Rc<T>` type keeps track of the number of references to data on the heap so that data can have {{i:multiple owners}}.

## RefCell

[![std][c-std-badge]][c-std]

The `RefCell<T>` type with its {{i:interior mutability}} gives us a type that we can use when we need an {{i:immutable type}} but need to change an {{i:inner value}} of that type; it also enforces the borrowing rules at runtime instead of at compile time.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
