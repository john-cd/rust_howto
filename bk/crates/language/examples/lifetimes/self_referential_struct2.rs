// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A small self-referential type whose pointer is initialized after pinning.
//!
//! Pinning does not create or validate self-references. The constructor below
//! uses `unsafe` to establish the pointer invariant after the value has a stable
//! address, then keeps the fields private so safe callers cannot break it.

mod selfref {
    use std::marker::PhantomPinned;
    use std::pin::Pin;
    use std::ptr::NonNull;

    /// A string together with a pointer to its own `String` field.
    pub struct SelfRef {
        data: String,
        data_ptr: Option<NonNull<String>>,
        _pin: PhantomPinned,
    }

    impl SelfRef {
        /// Allocates the value at a stable address and initializes its pointer.
        pub fn new(text: &str) -> Pin<Box<Self>> {
            let mut value = Box::pin(Self {
                data: text.to_owned(),
                data_ptr: None,
                _pin: PhantomPinned,
            });

            let data_ptr = NonNull::from(&value.as_ref().get_ref().data);

            // SAFETY: `Box::pin` has fixed the pointee's address. No reference
            // to the incomplete value escapes before this field is initialized.
            unsafe {
                value.as_mut().get_unchecked_mut().data_ptr = Some(data_ptr);
            }

            value
        }

        /// Reads the data through the stored self-pointer.
        pub fn text(self: Pin<&Self>) -> &str {
            let this = self.get_ref();
            let data_ptr = this
                .data_ptr
                .expect("SelfRef::new initializes the self-pointer");

            // SAFETY: The pointer is set by `new` to `this.data`. `SelfRef` is
            // `!Unpin`, the fields are private, and this API exposes no way to
            // move or mutate `data` while the value is pinned.
            unsafe { data_ptr.as_ref().as_str() }
        }

        /// Checks whether the stored pointer still addresses this value's data.
        pub fn pointer_matches_data(self: Pin<&Self>) -> bool {
            let this = self.get_ref();
            this.data_ptr
                .is_some_and(|pointer| std::ptr::eq(pointer.as_ptr(), &this.data))
        }
    }
}

/// Creates, pins, reads, and moves the handle to a self-referential value.
fn main() {
    use selfref::SelfRef;

    let value = SelfRef::new("I stay at the same address.");
    assert_eq!(value.as_ref().text(), "I stay at the same address.");
    assert!(value.as_ref().pointer_matches_data());

    // Moving `Pin<Box<SelfRef>>` moves the owning pointer, not its heap value.
    let moved_handle = value;
    assert_eq!(moved_handle.as_ref().text(), "I stay at the same address.");
    assert!(moved_handle.as_ref().pointer_matches_data());
}
// ANCHOR_END: example

/// Creates, pins, reads, and moves the handle to a self-referential value.
pub fn run() {
    main();
}

/// Verifies the pinned pointer invariant and the runnable example.
#[cfg(test)]
mod tests {
    use super::selfref::SelfRef;

    /// Verifies that the example entry point succeeds.
    #[test]
    fn self_referential_example_runs() {
        super::main();
    }

    /// Reads through the self-pointer before and after moving the `Pin<Box<_>>`.
    #[test]
    fn pointer_stays_with_the_pinned_value() {
        let value = SelfRef::new("stable");
        let address = value.as_ref().pointer_matches_data();
        assert!(address);
        assert_eq!(value.as_ref().text(), "stable");

        let moved_handle = value;
        assert!(moved_handle.as_ref().pointer_matches_data());
        assert_eq!(moved_handle.as_ref().text(), "stable");
    }
}
