#![allow(dead_code)]
// ANCHOR: example

fn main() {
    use std::pin::Pin;

    // In order to pin a value, we wrap a pointer to that value (of some type
    // `Ptr`) in a `Pin<Ptr>`. `Pin<Ptr>` can wrap any pointer type,
    // forming a promise that the _pointee_ will not be moved or otherwise
    // invalidated.

    // **Trivial case:** the pointee value’s type implements `Unpin`, which
    // cancels the effect of `Pin`. We can wrap any pointer to that value in
    // `Pin` directly via `Pin::new`. Here, we are safely pinning a mutable
    // reference to a `i32` on the stack.
    let mut x = 42;
    let pinned_x: Pin<&mut i32> = Pin::new(&mut x);

    // Conversely, we can unwrap the pin to get the underlying mutable reference to the value.
    let r = Pin::into_inner(pinned_x);
    assert_eq!(*r, 42);

    // If the pointee value's type does not implement `Unpin`, then Rust will
    // not let us use the `Pin::new` function directly and we'll need to
    // construct a `Pin`-wrapped pointer.

    // **General case:** Unsafely pinning any pointer without checking the
    // `Unpin` bound. The caller MUST guarantee the underlying data truly
    // does not move. Here, we have exclusive control of the `String`.
    {
        let mut y = String::from("hello");
        let _pinned_y = unsafe { Pin::new_unchecked(&mut y) };
    }

    // **Pinned Box**: The simplest way to pin a value that
    // does not implement `Unpin` is to put that value inside a `Box` and
    // then turn that `Box` into a "pinning Box" by wrapping it in a `Pin`.
    // <https://doc.rust-lang.org/std/pin/struct.Pin.html#pinning-a-value-inside-a-box>
    async fn add_one(x: u32) -> u32 {
        x + 1
    }

    // Call the async function to get a future back.
    let fut = add_one(42);

    // Pin the future inside a pinning box.
    let _pinned_fut: Pin<Box<_>> = Box::pin(fut);

    // **`pin!` Macro**:
    // `pin!` constructs a `Pin<&mut T>`.
    // Unlike `Box::pin`, this does not systematically create a new heap allocation.
    // <https://doc.rust-lang.org/std/pin/macro.pin.html>
    use core::pin::pin;

    struct Foo;

    fn stuff(_foo: Pin<&mut Foo>) {
        // …
    }

    let pinned_foo = pin!(Foo { /* … */ });
    stuff(pinned_foo);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
