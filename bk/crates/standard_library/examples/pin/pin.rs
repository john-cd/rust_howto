#![allow(dead_code)]
// ANCHOR: example
fn main() {
    use std::pin::Pin;

    // In order to pin a value, we wrap a pointer to that value (of some type
    // `Ptr`) in a `Pin<Ptr>`. `Pin<Ptr>` can wrap any pointer type,
    // forming a promise that the _pointee_ will not be moved or otherwise
    // invalidated.

    // 1. **Trivial case:** If the pointee value's type implements `Unpin`,
    //    which
    // cancels the effect of `Pin`, we can wrap any pointer to that value in
    // `Pin` directly via `Pin::new`.
    // That allows `x` to participate in `Pin`-bound APIs.
    let mut x = 42;
    let pinned_x: Pin<&mut i32> = Pin::new(&mut x);

    // Conversely, we can unwrap the pin to get the underlying mutable reference
    // to the value.
    let r = Pin::into_inner(pinned_x);
    assert_eq!(*r, 42);

    // 2. **General case:**
    // If the pointee value's type does not implement `Unpin`, then Rust will
    // not let us use the `Pin::new` function and we'll need to
    // use specialized ways, here unsafe code, to manipulate the inner value.
    //
    // The programmer MUST guarantee the underlying data truly
    // does not move in the `unsafe` code. Here, we have exclusive control of
    // the `String`.
    {
        use std::marker::PhantomPinned;
        pub struct Unpinned {
            // The `PhantomPinned` field makes the struct as `!Unpin`.
            _pin: PhantomPinned,
        }

        let mut y = Unpinned {
            _pin: PhantomPinned,
        };
        let _pinned_y = unsafe { Pin::new_unchecked(&mut y) };
    }

    // 3. **Pinned Box**: The simplest way to pin a value that
    // does not implement `Unpin` is to put that value inside a `Box` and
    // then wrapping it in a `Pin`.
    // Example fom <https://doc.rust-lang.org/std/pin/struct.Pin.html#pinning-a-value-inside-a-box>.
    async fn add_one(x: u32) -> u32 {
        x + 1
    }

    // Call the async function to get a future back.
    let fut = add_one(42); // `impl Future<Output = u32>`.

    // Pin the future inside a box.
    let pinned_box_fut: Pin<Box<_>> = Box::pin(fut);

    // Gets a shared reference to the pinned value this `Pin` points to.
    // This is a generic method to go from `&Pin<Pointer<T>>` to `Pin<&T>`.
    let _pinned_fut = pinned_box_fut.as_ref(); // `Pin<&(impl Future<Output = u32>)>`.

    // 4. **`pin!` Macro**:
    // There are some situations where it is desirable or even required (e.g.,
    // in a `#[no_std]` context) to pin a value which does not implement `Unpin`
    // to its location _on the stack_. Doing so is possible using the `pin!`
    // macro. Unlike `Box::pin`, this does not systematically create a new
    // heap allocation. <https://doc.rust-lang.org/std/pin/macro.pin.html>
    use core::pin::pin;

    struct Foo;

    fn stuff(_foo: Pin<&mut Foo>) {
        // ...
    }

    let pinned_foo = pin!(Foo { /* ... */ });
    stuff(pinned_foo);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
