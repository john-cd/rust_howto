// ANCHOR: example
//!

#[allow(dead_code)]
trait NotObjectSafe {
    fn clone_me(&self) -> Self; // Returns Self, not object-safe if `Self` is not `Sized`.
}

// To make it object-safe, you might change `clone_me` to return a
// `Box<dyn NotObjectSafe>`:
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
        }) // Assuming Circle is Cloneable.
    }
}

fn main() {
    // This causes a compile error:
    // let obj: Box<dyn NotObjectSafe>; // ERROR: the trait `NotObjectSafe` is
    // not dyn compatible.

    // But you can do:
    let obj: &dyn ObjectSafeClone = &Circle { radius: 1.0 };
    let _cloned_obj: Box<dyn ObjectSafeClone> = obj.clone_boxed();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
