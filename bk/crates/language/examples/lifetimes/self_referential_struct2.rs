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

        // Although it is not valid to swap data or assign through a `Pin<Ptr>`
        // since it would reuse the pinned object's memory, it is possible to
        // assign validly by implementing a function with special care.
        // We unpack the `Pin`, update values, and manually fix up the pointer.

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

    // The `drop` function takes `&mut self`, but this is called even if
    // that `self` has been pinned! Implementing `Drop` for a type with
    // address-sensitive states requires some care.
    // We use `inner_drop` with a `Pin<&mut Self>` to make sure that you do
    // not accidentally use `self` in a way that is in conflict with pinning's
    // invariants inside the drop logic.
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

pub mod structural_pinning {
    use std::pin::Pin;

    pub struct Struct {
        pub structural_field: i32,
        pub unpinned_field: i32,
    }

    impl Struct {
        // Structural Pinning: Pinning is "structural" for this field, meaning
        // that if the struct is pinned, then so is the field.
        // It allows writing a projection that creates a `Pin<&mut Field>`.
        pub fn structural_field(self: Pin<&mut Self>) -> Pin<&mut i32> {
            // SAFETY: This is okay because `structural_field` is pinned when
            // `self` is.
            unsafe { self.map_unchecked_mut(|s| &mut s.structural_field) }
        }

        // Non-structural Pinning: We explicitly choose not to expose a
        // `Pin<&mut Field>`, so we don't need to be careful about other
        // code moving out of that field. It provides a projection
        // method that turns `Pin<&mut Struct>` into `&mut Field`.
        pub fn unpinned_field(self: Pin<&mut Self>) -> &mut i32 {
            // SAFETY: This is okay because `unpinned_field` is never considered
            // pinned, therefore we do not need to uphold any
            // pinning guarantees for this field.
            unsafe { &mut self.get_unchecked_mut().unpinned_field }
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

    use structural_pinning::Struct;
    let s = Struct {
        structural_field: 1,
        unpinned_field: 2,
    };
    let mut pinned_struct = std::pin::pin!(s);

    // We can project a structurally pinned field to a Pin<&mut Field>
    let mut sf = pinned_struct.as_mut().structural_field();
    *sf = 10;

    // We can project a non-structurally pinned field to a &mut Field
    let uf = pinned_struct.as_mut().unpinned_field();
    *uf = 20;

    assert_eq!(*pinned_struct.as_mut().structural_field(), 10);
    assert_eq!(*pinned_struct.as_mut().unpinned_field(), 20);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
