# Smart Pointers

- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime. 
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.


## Box

`Box<T>` allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data. 

The `Box<T>` type is a smart pointer because it implements the Deref trait, which allows `Box<T>` values to be treated like references.
Implementing the Deref trait allows you to customize the behavior of the dereference operator `*`.

Use when
- you have a type whose size can’t be known at compile time
- you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type.

[Box]( https://doc.rust-lang.org/book/ch15-01-box.html )


## Rc

The `Rc<T>` type keeps track of the number of references to data on the heap so that data can have multiple owners. 


## RefCell

The `RefCell<T>` type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.
