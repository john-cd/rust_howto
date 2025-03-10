# Send, Sync traits

{{#include send_sync.incl.md}}{{hi:Send}}{{hi:Sync}{{hi:'static}}}

## `Send` and `Sync` traits {#send-sync}

The `Send` and `Sync` [traits][p-traits] are fundamental to Rust's [concurrency][p-concurrency]. You can think of Send as "Exclusive access is thread-safe," and Sync as "Shared access is thread-safe."

A type is `Send` if it can be _transferred across thread boundaries_. Most types in Rust are `Send` by default, as long as they don't contain non-`Send` types.

`Send` allows an object to be used by two or more threads at _different times_. Thread 'A' can create and use an object, then send it to thread 'B', so thread 'B' can use the object while thread 'A' cannot. The Rust ownership model can be used to enforce this non-overlapping use. In other words, `Send` means that a type is safe to _move_ from one thread to another. If the same type also implements `Copy`, it is safe to _copy_ from one thread to another.

An important exception is `Rc`. By cloning, it allows data to have multiple owners. If one owner in thread 'A' could send the `Rc` to another thread, giving ownership to thread 'B', there could be other owners in thread 'A' that can still use the object. Since the reference count is modified non-atomically, the value of the count on the two threads may get out of sync and one thread may drop the pointed-at value while there are owners in the other thread. Therefore `Rc` does not implement `Send`.

A type is `Sync` if it is safe to be referenced from multiple threads _simultaneously_. This is trivial for immutable objects, but mutations need to be synchronized (performed in sequence with the same order being seen by all threads). This is often done using a `Mutex` or `RwLock` which allows one thread to proceed while others must wait. By enforcing a shared order of changes, these types can turn a non-`Sync` object into a `Sync` object. Another mechanism for making objects `Sync` is to use atomic types, which are essentially `Sync` primitives.

`Arc` is an `Rc` that uses an atomic type for the reference count. Hence it can be used by multiple threads without the count getting out of sync. If the data that the Arc points to is `Sync`, the entire object is `Sync`. If the data is not `Sync` (e.g. a mutable type), it can be made `Sync` using a `Mutex`. Hence the proliferation of `Arc<Mutex<T>>` types in multi-threaded Rust code.

`T` is `Sync` if and only if `&T` is `Send`.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/send_sync/send_sync.rs:example}}
```

## Existing implementations of `Send` and `Sync` {#existing-impl-send-sync}

| Traits | Types |
|---|---|
| `Send` and `Sync` | primitives; `(T1, T2)`, `[T; N]`, `&[T]`, `struct { x: T }`, `Arc`, `Vec`, `Box`, [`Option`][c-std::option::Option]⮳{{hi:Option}} (depending on underlying types); [`String`][c-std::string::ToString]⮳{{hi:String}}, `&str`; `Mutex`, `Atomic*`... |
| `!Send` and `!Sync` | `Rc`, raw pointers `*const T`,`*mut T`, types from external libraries or the operating system that are not thread safe |
| `Send` and `!Sync` | `mpsc::Receiver<T>`; `UnsafeCell`, `Cell`, `RefCell`: when a type has interior mutability, we must be sure that we mutate it from one place only, but this place can be everywhere as long as it is singular |
| `!Send` and `Sync` (rare) | `RwLockReadGuard`, `RwWriteGuard` and [`MutexGuard`][c-std::sync::MutexGuard]⮳{{hi:MutexGuard}}; `&mut T` if T is `!Send`; structs which use thread-local storage and accesses that info in `Drop` |

## Implementing `Send` and `Sync` {#implementing-send-sync}

As discussed above, `Send` and `Sync` are automatically derived [traits][p-traits]. This means that, unlike almost every other trait, if a type is composed entirely of `Send` or `Sync` types, then it is `Send` or `Sync`.

If you want to work with non-`Sync` / `Send` types like raw pointers, you should build an abstraction on which `Send` and `Sync` can be derived.

Note that, by implementing the unsafe marker [traits][p-traits] `Send` and `Sync`, you _guarantee_ that your struct can be sent across threads safely. This means the usage of `MyStruct` must not cause data races or other thread safety issues. An incorrect implementation can cause Undefined Behavior. Caveat lector!

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[Send / Sync custom implementation: finish example in playground crate](https://github.com/john-cd/rust_howto/issues/265)
[write](https://github.com/john-cd/rust_howto/issues/909)

- Credit [Understanding the Send trait](https://stackoverflow.com/questions/59428096/understanding-the-send-trait).

- Reference [https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html](https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html).
[https://nyanpasu64.gitlab.io/blog/an-unsafe-tour-of-rust-s-send-and-sync/](https://nyanpasu64.gitlab.io/blog/an-unsafe-tour-of-rust-s-send-and-sync/)

[nomicon/send-and-sync](https://doc.rust-lang.org/nomicon/send-and-sync.html)
[extensible-concurrency-sync-and-send](https://doc.rust-lang.org/stable/book/ch16-04-extensible-concurrency-sync-and-send.html)
Implementing Vec: [https://doc.rust-lang.org/nomicon/vec/vec.html](https://doc.rust-lang.org/nomicon/vec/vec.html)
[https://nyanpasu64.gitlab.io/blog/an-unsafe-tour-of-rust-s-send-and-sync/](https://nyanpasu64.gitlab.io/blog/an-unsafe-tour-of-rust-s-send-and-sync/)
[https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html](https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html)

- Need deep tech review.

- Add Send / Sync impl example. See code in playground crate:

In the following example, we define a struct with a raw pointer to some data.

We spawn multiple threads to demonstrate that `MyStruct` is `Send` and `Sync`. Each thread prints the value of the data pointer.
The data pointer remains valid and that the usage of `MyStruct` is thread-safe.
</div>
