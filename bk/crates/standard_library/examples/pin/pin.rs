// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Pinning values with `Pin` and `PhantomPinned`.
//!
//! `Pin` promises that a value will not be moved in memory. Futures made by
//! `async` blocks can hold references into themselves, so they must stay put
//! while polled. `PhantomPinned` marks a type that must not be moved.

use std::marker::PhantomPinned;
use std::pin::Pin;

/// A type that cannot be moved once pinned.
struct PinnedField {
    /// Marker that makes the type `!Unpin`.
    _pin: PhantomPinned,
}

/// A struct with a pinned field.
struct PinnedParent {
    /// The structurally pinned field.
    field: PinnedField,
    /// Marker that makes the type `!Unpin`.
    _pin: PhantomPinned,
}

impl PinnedParent {
    /// Returns a pinned mutable reference to the field.
    fn field(self: Pin<&mut Self>) -> Pin<&mut PinnedField> {
        // SAFETY: `field` is structurally pinned: this API never exposes it as
        // an unpinned mutable reference or moves it out of the parent.
        unsafe { self.map_unchecked_mut(|parent| &mut parent.field) }
    }
}

/// Pins values on the stack and on the heap.
fn main() {
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
    println!("Pinned value: {}", *r);

    // 2. **General case:**
    // If the pointee value's type does not implement `Unpin`, then Rust will
    // not let us use `Pin::new`; creating a pin requires an unsafe operation.
    //
    // The programmer MUST guarantee the underlying data truly
    // does not move in the `unsafe` code. Here, we have exclusive control of
    // `y`.
    {
        /// Demonstrates a type that does not implement `Unpin`.
        struct NotUnpin {
            // The `PhantomPinned` field makes the struct `!Unpin`.
            /// Pinning marker used by the example.
            _pin: PhantomPinned,
        }

        let mut y = NotUnpin {
            _pin: PhantomPinned,
        };
        let _pinned_y = unsafe { Pin::new_unchecked(&mut y) };
    }

    // A projection can preserve pinning for a field declared structurally
    // pinned. Once the parent is pinned, this method returns a pinned borrow of
    // its `!Unpin` field.
    let mut parent = Box::pin(PinnedParent {
        field: PinnedField {
            _pin: PhantomPinned,
        },
        _pin: PhantomPinned,
    });
    let _pinned_field: Pin<&mut PinnedField> = parent.as_mut().field();

    // 3. **Pinned Box**: The simplest way to pin a value that
    // does not implement `Unpin` is to put that value inside a `Box` and
    // then wrapping it in a `Pin`.
    // Example from <https://doc.rust-lang.org/std/pin/struct.Pin.html#pinning-a-value-inside-a-box>.
    /// Returns the input incremented by one when awaited.
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
    // heap allocation. <https://doc.rust-lang.org/std/pin/macro.pin.html>.
    use core::pin::pin;

    /// Type used to demonstrate pinning a mutable reference.
    struct Foo;

    /// Accepts a pinned mutable reference to a `Foo` value.
    fn stuff(_foo: Pin<&mut Foo>) {
        // ...
    }

    let pinned_foo = pin!(Foo { /* ... */ });
    stuff(pinned_foo);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use std::future::Future as _;
    use std::task::Context;
    use std::task::Poll;
    use std::task::Waker;

    use super::*;

    #[test]
    fn test_pin_example_entry_point() {
        main();
    }

    #[test]
    fn test_pin_new_for_unpin_types() {
        let mut x = 42;
        let mut pinned: Pin<&mut i32> = Pin::new(&mut x);
        *pinned = 43;
        pinned.set(44);
        assert_eq!(*Pin::into_inner(pinned), 44);
    }

    #[test]
    fn test_pinned_parent_projects_to_its_field() {
        let mut parent = Box::pin(PinnedParent {
            field: PinnedField {
                _pin: PhantomPinned,
            },
            _pin: PhantomPinned,
        });
        let field_ptr: *const PinnedField = &raw const parent.field;
        let projected = parent.as_mut().field();
        assert!(std::ptr::eq(&raw const *projected, field_ptr));
    }

    #[test]
    fn test_pinned_future_can_be_polled() {
        async fn add_one(x: u32) -> u32 {
            x + 1
        }
        let mut fut = Box::pin(add_one(42));
        let mut cx = Context::from_waker(Waker::noop());
        assert_eq!(fut.as_mut().poll(&mut cx), Poll::Ready(43));
    }
}
