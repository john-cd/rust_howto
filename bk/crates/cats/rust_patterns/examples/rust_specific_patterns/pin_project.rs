#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! The following demonstrates the use of the `pin_project` crate for pin
//! projection.
//!
//! Pinning keeps a value at a stable address. Projection borrows its fields
//! while preserving whether each field is pinned. This is useful for
//! address-sensitive state, including self-references and futures that borrow
//! their own state.
//!
//! Prefer a projection macro to hand-written pointer manipulation. Incorrect
//! pinning can move a value after another pointer relies on its address.

use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

use pin_project::pin_project;
use pin_project::pinned_drop;

/// The `#[pin_project]` attribute creates a projection type covering all the
/// fields of struct or enum.
// ///
/// - For the fields that use #[pin] attribute, it creates the pinned reference
///   to the field.
/// - For the other fields, it creates a normal reference to the field.
///
/// This works for tuple structs as well.
#[pin_project]
struct BasicPinned<T, U>
where
    T: Debug,
    U: Clone,
{
    /// A field that is structurally pinned.
    #[pin]
    pinned: T,
    /// A field that is not pinned.
    unpinned: U,
}

/// `#[pin_project]` also implements the following methods on the original type:
/// `fn project(self: Pin<&mut Self>) -> Projection<'_>;`
/// `fn project_ref(self: Pin<&Self>) -> ProjectionRef<'_>;`
#[allow(dead_code)]
impl<T: Debug, U: Clone> BasicPinned<T, U> {
    /// Projects the pinned value to references to its fields.
    fn method(self: Pin<&mut Self>) {
        // Return the projection type:
        let this = self.project();
        let _: Pin<&mut T> = this.pinned; // Pinned reference to the field.
        let _: &mut U = this.unpinned; // Normal reference to the field.
    }
}

// Example 1: Basic pinned struct.
/// Projects a pinned struct onto its fields.
fn example1() {
    let basic = BasicPinned {
        pinned: String::from("pinned data"),
        unpinned: 42,
    };

    let mut pinned_basic = Box::pin(basic);
    let projection = pinned_basic.as_mut().project();

    // `projection.pinned` is `Pin<&mut String>`.
    // `projection.unpinned` is `&mut i32`.
    *projection.unpinned += 1;
    println!("Updated unpinned value: {}", projection.unpinned);
}

/// ## Self-Referential Struct
///
/// This struct demonstrates how to create a self-referential struct using
/// `#[pin_project]`.
///
/// Linked Lists, Trees with Parent Pointers
#[pin_project]
struct SelfReferential {
    /// The string that `pointer` refers to.
    #[pin]
    value: String,
    // `pointer` will refer to the `value` above, making this type
    // self-referential.
    /// A pointer to `value`.
    pointer: *const String,

    // The vast majority of Rust types have no address-sensitive states.
    // These types implement the `Unpin` auto-trait, which cancels the
    // restrictive effects of `Pin` when the pointee type `T` is `Unpin`.
    // In this example, we rely upon pinning for soundness and must ensure
    // that the type is not automatically marked as `Unpin`. We do so by
    // adding a `PhantomPinned` marker field.
    /// A marker that makes the type `!Unpin`.
    #[pin]
    // <------ This `#[pin]` is required to make `Struct` to `!Unpin`.
    /// Marker that makes the type pinned.
    _marker: PhantomPinned,
    // This is equivalent to `#[pin_project(!Unpin)]`.
}

impl SelfReferential {
    /// Creates a value, whose pointer is not set yet.
    fn new(value: String) -> Self {
        Self {
            value,
            pointer: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    /// Sets the pointer to the value.
    fn init(self: Pin<&mut Self>) {
        let this = self.project();
        *this.pointer = std::ptr::from_ref(this.value.as_ref().get_ref());
    }

    /// Returns the pointer to the value.
    fn get_pointer(self: Pin<&Self>) -> *const String {
        *self.project_ref().pointer
    }
}

// Example 2: Self-referential struct.
/// Creates a self-referential struct, pins it and reads through its pointer.
fn example2() {
    // Pin the struct in a heap allocation before creating the self-reference.
    let mut pinned_self_ref =
        Box::pin(SelfReferential::new(String::from("self-referential data")));

    // Initialize the self-reference.
    pinned_self_ref.as_mut().init();

    // Access the data through the self-reference.
    let ptr = pinned_self_ref.as_ref().get_pointer();

    // SAFETY: init stored the address of value after the !Unpin struct was
    // pinned, and the pinned allocation remains alive for this dereference.
    println!("Pointer points to value: {}", unsafe { &*ptr });
}

/// A future wrapper that pins its inner future and keeps movable poll state.
#[pin_project]
struct PollCounter<F> {
    /// The inner future may be address-sensitive and must remain pinned.
    #[pin]
    future: F,
    /// The wrapper's ordinary bookkeeping does not need to be pinned.
    polls: usize,
}

impl<F: Future> Future for PollCounter<F> {
    type Output = F::Output;

    /// Counts this poll and delegates to the pinned inner future.
    fn poll(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.project();
        *this.polls += 1;
        this.future.poll(context)
    }
}

/// Demonstrates projection to a pinned future and movable bookkeeping.
fn future_projection_example() {
    let mut future = Box::pin(PollCounter {
        future: std::future::ready(42),
        polls: 0,
    });
    let mut context = Context::from_waker(Waker::noop());

    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(42));
    assert_eq!(future.as_ref().get_ref().polls, 1);
}

/// ## Pinned Enum
///
/// This enum demonstrates how to use `pin_project`
/// with an enum.
///
/// By passing an argument with the same name as the projection method
/// to the `pin_project` attribute, we can name the projection type returned
/// from the method. This allows us to use pattern matching on the projected
/// types.
#[pin_project(project = EnumProj)]
enum PinnedEnum<T, U> {
    /// A variant with a pinned field and an unpinned field.
    Variant1(#[pin] T, U),
}

impl<T: Debug, U: Debug> PinnedEnum<T, U> {
    /// Matches on the projection of the pinned enum.
    fn process(self: Pin<&mut Self>) {
        match self.project() {
            // Note the named projection type:
            EnumProj::Variant1(pinned, unpinned) => {
                println!(
                    "Processing Variant1 with pinned {pinned:?} and unpinned: {unpinned:?}"
                );
                // `pinned` is `Pin<&mut T>`,
                // `unpinned` is `&mut U`.
            }
        }
    }
}

// Example 3: Enum.
/// Projects a pinned enum onto its variant.
fn example3() {
    let mut enum_value =
        PinnedEnum::Variant1(String::from("pinned enum data"), 123);
    let mut pinned_enum = unsafe { Pin::new_unchecked(&mut enum_value) };

    pinned_enum.as_mut().process();
}

/// ## Pinned Struct with Custom Drop.
///
/// In order to correctly implement pin projections, a type's `Drop` impl must
/// not move out of any structurally pinned fields.
///
/// This struct demonstrates how to use `#[pin_project(PinnedDrop)]` to
/// create a struct with a custom `drop` implementation.
#[pin_project(PinnedDrop)]
struct PinnedWithDrop<T: Debug> {
    /// A field that is structurally pinned.
    #[pin]
    pinned_field: T,
    /// A field that is not pinned.
    unpinned_field: String,
}

/// The #[pin_project] attribute will provide a `Drop` implementation,
/// based on your implementation of the `PinnedDrop` trait:
#[pinned_drop]
impl<T: Debug> PinnedDrop for PinnedWithDrop<T> {
    fn drop(self: Pin<&mut Self>) {
        // this method takes `Pin<&mut Self>`.
        println!(
            "Dropping PinnedWithDrop with pinned_field {:?} and unpinned_field: {}",
            self.pinned_field, self.unpinned_field
        );
    }
}

// Example 4: Custom `drop`.
/// Creates a value with a custom `drop` implementation, which runs at the end
/// of the scope.
fn example4() {
    let _pinned_drop = PinnedWithDrop {
        pinned_field: 42,
        unpinned_field: String::from("will be dropped"),
    };

    println!("Created PinnedWithDrop, will be dropped at end of scope");
}

/// Runs the `pin_project` examples.
fn main() {
    example1();
    example2();
    future_projection_example();
    example3();
    example4();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exercises field, future, self-reference, enum, and drop projections.
    /// Runs the pin project example through its entry point.
    #[test]
    fn test_pin_project_example_entry_point() {
        main();
    }
}
