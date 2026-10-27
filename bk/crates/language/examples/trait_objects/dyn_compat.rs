#![allow(dead_code)]
#![allow(unused_variables)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example of (non-)dyn-compatible traits.

use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

// Trait must not have any associated constant.
/// A trait that is not dyn compatible, because it has an associated constant.
trait NotObjectSafe {
    /// An associated constant.
    const CONST: usize = 10;
}

// Trait must not have any associated types with generics.
/// A trait that is not dyn compatible, because it has a generic associated
/// type.
trait NotObjectSafe2 {
    /// A generic associated type.
    type Item<T>;
}

// `Sized` must not be a supertrait.
/// A trait that is not dyn compatible, because it requires `Self: Sized`.
trait NotObjectSafe3
where
    Self: Sized,
{
}

/// A trait that is not dyn compatible, because a method has a type parameter.
trait NotObjectSafe4 {
    // Trait functions cannot have any type parameters
    // (although lifetime parameters are allowed).
    /// A method with a type parameter.
    fn do_stuff<T>(&self, _t: T);
}

// Cannot use `Self` except in the type of the receiver.
/// A trait that is not dyn compatible, because a method returns `Self`.
trait NotObjectSafe5 {
    /// Returns a copy of `self`.
    fn clone_me(&self) -> Self; // Returns `Self`, not object-safe if `Self` is not `Sized`.
}

// Same problem:
/// A trait that is not dyn compatible, because a method returns `Box<Self>`.
trait NotObjectSafe6 {
    /// Returns a boxed copy of `self`.
    fn clone_boxed(&self) -> Box<Self>;
}

// To make it object-safe, you might change `clone_me`
// to return a `Box<dyn ObjectSafeClone>`:
/// A dyn-compatible trait that can clone itself.
trait ObjectSafeClone {
    /// Returns a boxed copy of `self`, as a trait object.
    fn clone_boxed(&self) -> Box<dyn ObjectSafeClone>;
}

// Example struct.
/// A circle.
struct Circle {
    /// The radius of the circle.
    radius: f64,
}

impl ObjectSafeClone for Circle {
    fn clone_boxed(&self) -> Box<dyn ObjectSafeClone> {
        Box::new(Self {
            radius: self.radius,
        })
    }
}

// Additional examples of dyn-compatible methods.
/// A trait with the receivers that dyn-compatible methods can have.
trait TraitMethods {
    // Trait methods must have a receiver with one of the following types:
    /// Takes `self` by reference.
    fn by_ref(&self) {} // Equivalent to self: &Self.

    /// Takes `self` by mutable reference.
    fn by_ref_mut(&mut self) {} // Equivalent to self: &mut Self.

    /// Takes `self` in a `Box`.
    fn by_box(self: Box<Self>) {}

    /// Takes `self` in an `Rc`.
    fn by_rc(self: Rc<Self>) {}

    /// Takes `self` in an `Arc`.
    fn by_arc(self: Arc<Self>) {}

    /// Takes `self` in a `Pin`.
    fn by_pin(self: Pin<&Self>) {}

    /// Takes `self` by reference with an explicit lifetime.
    #[allow(clippy::needless_lifetimes)]
    fn with_lifetime<'a>(&'a self) {} // Equivalent to self: &'a Self.

    /// Takes `self` in a `Pin<Arc<Self>>`.
    fn nested_pin(self: Pin<Arc<Self>>) {}

    // You can also constrain functions that otherwise could not be included in
    // a dyn-compatible trait, so they do not apply to trait objects.
    // They remain accessible on implementing concrete types.
    /// An associated function, which is excluded from trait objects.
    fn associated_function()
    where
        Self: Sized,
    {
    }

    // Same thing for methods that return `Self`:
    /// Returns `Self`, so it is excluded from trait objects.
    fn returns(&self) -> Self
    where
        Self: Sized;

    /// Takes another `Self`, so it is excluded from trait objects.
    fn param(&self, other: Self)
    where
        Self: Sized,
    {
    }

    /// Has a type parameter, so it is excluded from trait objects.
    fn typed<T>(&self, x: T)
    where
        Self: Sized,
    {
    }
}

impl TraitMethods for Circle {
    fn returns(&self) -> Self
    where
        Self: Sized,
    {
        Self {
            radius: self.radius,
        }
    }
}

/// Shows which traits can be used as trait objects.
fn main() {
    // These cause a compile error:
    // ERROR: the trait `NotObjectSafe` is not dyn compatible.
    // let obj: Box<dyn NotObjectSafe>;
    // let obj: Box<dyn NotObjectSafe2>;
    // let obj: Box<dyn NotObjectSafe3>;
    // let obj: Box<dyn NotObjectSafe4>;
    // let obj: Box<dyn NotObjectSafe5>;
    // let obj: Box<dyn NotObjectSafe6>;

    // But you can do:
    let obj: &dyn ObjectSafeClone = &Circle { radius: 1.0 };
    let _cloned_obj: Box<dyn ObjectSafeClone> = obj.clone_boxed();

    let mut obj: Box<dyn TraitMethods> = Box::new(Circle { radius: 1.0 });
    obj.by_ref();
    obj.by_ref_mut();
    obj.by_box();

    // Functions with a `where Self: Sized` constraints cannot be called on a
    // trait object...
    // obj.returns();
    // ERROR: the `returns` method cannot be invoked on a trait object.
    // ...but can be called on an implementing type:
    let c = Circle { radius: 1.0 };
    let _ = c.returns();
    <Circle as TraitMethods>::associated_function();
    println!("dyn compatibility examples: dyn-compatible traits demonstrated");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the dyn compat example through its entry point.
    #[test]
    fn test_dyn_compat_example_entry_point() {
        main();
    }
}
