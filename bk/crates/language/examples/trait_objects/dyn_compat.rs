#![allow(dead_code)]
#![allow(unused_variables)]
// ANCHOR: example
//! Example of (non-)dyn-compatible traits.

use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

// Trait must not have any associated constant.
trait NotObjectSafe {
    const CONST: usize = 10;
}

// Trait must not have any associated types with generics.
trait NotObjectSafe2 {
    type Item<T>;
}

// `Sized` must not be a supertrait.
trait NotObjectSafe3
where
    Self: Sized,
{
}

trait NotObjectSafe4 {
    // Trait functions cannot have any type parameters
    // (although lifetime parameters are allowed).
    fn do_stuff<T>(&self, _t: T);
}

// Cannot use `Self` except in the type of the receiver.
trait NotObjectSafe5 {
    fn clone_me(&self) -> Self; // Returns `Self`, not object-safe if `Self` is not `Sized`.
}

// Same problem:
trait NotObjectSafe6 {
    fn clone_boxed(&self) -> Box<Self>;
}

// To make it object-safe, you might change `clone_me`
// to return a `Box<dyn ObjectSafeClone>`:
trait ObjectSafeClone {
    fn clone_boxed(&self) -> Box<dyn ObjectSafeClone>;
}

// Example struct.
struct Circle {
    radius: f64,
}

impl ObjectSafeClone for Circle {
    fn clone_boxed(&self) -> Box<dyn ObjectSafeClone> {
        Box::new(Circle {
            radius: self.radius,
        })
    }
}

// Additional examples of dyn-compatible methods.
trait TraitMethods {
    // Trait methods must have a receiver with one of the following types:
    fn by_ref(&self) {} // Equivalent to self: &Self.
    fn by_ref_mut(&mut self) {} // Equivalent to self: &mut Self.
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    #[allow(clippy::needless_lifetimes)]
    fn with_lifetime<'a>(&'a self) {} // Equivalent to self: &'a Self.
    fn nested_pin(self: Pin<Arc<Self>>) {}
    // You can also constrain functions that otherwise could not be included in
    // a dyn-compatible trait, so they do not apply to trait objects.
    // They remain accessible on implementing concrete types.
    fn associated_function()
    where
        Self: Sized,
    {
    }
    // Same thing for methods that return `Self`:
    fn returns(&self) -> Self
    where
        Self: Sized;
    fn param(&self, other: Self)
    where
        Self: Sized,
    {
    }
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
        Circle {
            radius: self.radius,
        }
    }
}

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
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
