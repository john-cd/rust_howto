// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Types that can implement a trait.

// An example trait (with a method with a default implementation):
/// A trait with a default method.
trait Tr {
    /// Does nothing.
    fn foo(&self) {
        println!("foo");
    }
}

/// An enum.
enum MyEnum {
    /// The first variant.
    A,
    /// The second variant.
    B,
}

// Implement a trait for an enum:
impl Tr for MyEnum {}

/// A union.
#[repr(C)]
union MyUnion {
    /// A 32-bit unsigned integer.
    f1: u32,
    /// A 32-bit float, which shares its memory with `f1`.
    f2: f32,
}

// Implement a trait for a union:
impl Tr for MyUnion {}

// Implement a trait for a primitive type:
impl Tr for u32 {}

// Implement a trait for a slice:
impl Tr for [u32] {}

// Implement a trait for an array:
impl Tr for [u32; 3] {}

// Implement a trait for a tuple:
impl Tr for (u32, f64) {}

// Implement a trait for a function pointer:
impl Tr for fn() {}

// Implement a trait for a reference type:
impl Tr for &u32 {}

/// Calls the trait method on several types.
fn main() {
    let e = MyEnum::A;
    e.foo();
    MyEnum::B.foo();
    let u = MyUnion { f1: 42 };
    u.foo();
    let s: &[u32] = &[1, 2, 3, 4];
    s.foo();
    let a: [u32; 3] = [1, 2, 3];
    a.foo();
    let t = (1, 2.0);
    t.foo();
    /// Prints the label used by the function-pointer example.
    fn bar() {
        println!("bar");
    }
    // Function pointers can be created via a coercion from both function items
    // and non-capturing, non-async closures.
    let f: fn() = bar;
    f.foo();
    f();
    let r = &42;
    r.foo();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the trait types example through its entry point.
    #[test]
    fn test_trait_types_example_entry_point() {
        main();
    }

    /// Every kind of type got the default method.
    #[test]
    fn test_default_method_on_each_type() {
        MyEnum::A.foo();
        MyUnion { f1: 1 }.foo();
        42_u32.foo();
        let s: &[u32] = &[1];
        s.foo();
        [1_u32, 2, 3].foo();
        (1_u32, 2.0_f64).foo();
        let f: fn() = || {};
        f.foo();
        let r = &7_u32;
        r.foo();
    }
    // Output-only example: the default method only prints.
}
