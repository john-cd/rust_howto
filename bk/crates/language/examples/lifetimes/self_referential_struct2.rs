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

        /// Overwrites the contents of `self` with `src`, fixing up the
        /// self-pointer in the process.
        ///
        /// Since `SelfRef` is `!Unpin`, it cannot be moved once pinned.
        /// However, we can still overwrite its data in-place if we have
        /// exclusive access (`&mut Self`). This requires `unsafe` to
        /// bypass the `Pin` protection.
        ///
        /// Note that the self-referential pointer must be updated to point
        /// to the new data's address within `self`, not the address it had
        /// in `src`.
        ///
        /// <https://doc.rust-lang.org/std/pin/index.html#assigning-pinned-data>
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
                let new_ptr = &unpinned_self.data as *const String;
                unpinned_self.ptr = new_ptr;
            }
        }
    }

    /// When implementing `Drop` for `!Unpin` types, we must be careful not to
    /// move `self`. Although `drop` takes `&mut self`, the `Pin` contract
    /// guarantees that the value has not been moved since it was pinned.
    ///
    /// <https://doc.rust-lang.org/std/pin/index.html#implementing-drop-for-types-with-address-sensitive-states>
    impl Drop for SelfRef {
        fn drop(&mut self) {
            // SAFETY: We must ensure that `inner_drop` does not move `self`.
            // Since we are in `drop`, the value will be invalidated anyway
            // after this call, but any code inside `inner_drop` must respect
            // the pinning invariant.
            inner_drop(unsafe { Pin::new_unchecked(self) });

            fn inner_drop(_this: Pin<&mut SelfRef>) {
                // Actual drop code would go here.
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

    assert!(&pinned.data as *const String == pinned.ptr);

    // `Pin` and `Box` implement `Debug` if the underlying type does.
    println!("{pinned:?}");

    // The inner pointee `SelfRef` struct will now never be allowed to move.
    // Meanwhile, we are free to move the smart pointer around.
    let mut still_unmoved = pinned;

    let mut other_pinned =
        SelfRef::new("I am another self-referential struct.");

    // Perform an assignment.
    still_unmoved.as_mut().assign(other_pinned.as_mut());

    assert_eq!(still_unmoved.data, "I am another self-referential struct.");
    // Verify the self-pointer was updated correctly to the target's address.
    assert!(&still_unmoved.data as *const String == still_unmoved.ptr);
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
