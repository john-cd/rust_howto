#![allow(dead_code)]
// ANCHOR: example
//! Example of a self-referential struct using pinning.

mod selfref {

    use std::marker::PhantomPinned;
    use std::pin::Pin;

    // Self-referential struct.
    //
    // To create a `!Unpin` type on stable Rust,
    // embed `std::marker::PhantomPinned`.
    // This marker tells the compiler your type must never move once pinned.
    #[derive(Debug)]
    pub struct SelfRef {
        pub data: String,
        // Raw pointer to the field above.
        // You will often use `NotNull` as well.
        pub ptr: *const String,
        // The `PhantomPinned` field makes `SelfRef: !Unpin`.
        _pin: PhantomPinned,
    }

    impl SelfRef {
        pub fn new(txt: &str) -> Pin<Box<Self>> {
            // Allocate on heap and construct a new `Pin<Box<T>>`.
            // Since the type does not implement `Unpin`, then the newly created
            // struct will be pinned in memory and unable to be moved.
            let mut boxed: Pin<Box<Self>> = Box::pin(SelfRef {
                data: txt.to_string(),
                ptr: std::ptr::null(), /* Initially provide a dummy value to
                                        * the pointer, since we don't know
                                        * the address yet. */
                _pin: PhantomPinned,
            });
            // Constructing and pinning of the `Box` can also be done in two
            // steps: `Box::into_pin(Box::new(x))`.

            // SAFETY: We have exclusive access to `boxed`.
            let self_ptr: *const String = &boxed.data;
            unsafe {
                // Get a mutable reference to the pinned struct.
                // - `as_mut` goes from `&mut Pin<Pointer<T>>` to `Pin<&mut T>`
                //   (here `Pointer` is a `Box`).
                // - `get_unchecked_mut` gets a mutable reference to what's
                //   inside of the `Pin`.
                let mut_ref: &mut SelfRef =
                    Pin::get_unchecked_mut(boxed.as_mut());
                // Assign the right address.
                mut_ref.ptr = self_ptr;
            }
            boxed
        }

        // FIXME explain

        // Copies the contents of `src` into `self`, fixing up the self-pointer
        // in the process.
        // <https://doc.rust-lang.org/std/pin/index.html#assigning-pinned-data>
        pub fn assign(self: Pin<&mut Self>, src: Pin<&mut Self>) {
            unsafe {
                // Unwraps the `Pin<Ptr>`, returning the underlying `Ptr`.
                let unpinned_self = Pin::into_inner_unchecked(self);
                let unpinned_src = Pin::into_inner_unchecked(src);

                *unpinned_self = Self {
                    data: unpinned_src.data.clone(),
                    ptr: std::ptr::null(),
                    _pin: PhantomPinned,
                };

                // Adjust the self-pointer:
                let new_ptr = unpinned_src.data.as_ptr() as *const String;
                unpinned_self.ptr = new_ptr;
            }
        }
    }

    // FIXME explain
    // <https://doc.rust-lang.org/std/pin/index.html#implementing-drop-for-types-with-address-sensitive-states>
    impl Drop for SelfRef {
        fn drop(&mut self) {
            // `new_unchecked` is okay because we know this value is never used
            // again after being dropped.
            inner_drop(unsafe { Pin::new_unchecked(self) });

            fn inner_drop(_this: Pin<&mut SelfRef>) {
                // Actual drop code goes here.
            }
        }
    }
}

fn main() {
    use std::pin::Pin;

    use selfref::*;

    let pinned: Pin<Box<SelfRef>> =
        SelfRef::new("I am a self-referential struct.");

    assert!(std::ptr::addr_eq(&pinned.data, pinned.ptr));

    // `Pin` and `Box` implement `Debug` if the underlying type does.
    println!("{pinned:?}");

    // The inner pointee `SelfRef` struct will now never be allowed to move.
    // Meanwhile, we are free to move the smart pointer around.
    let mut _still_unmoved = pinned;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [cover <https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning>](https://github.com/john-cd/rust_howto/issues/1407)
