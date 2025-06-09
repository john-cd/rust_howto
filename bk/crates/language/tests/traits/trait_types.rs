#![allow(dead_code)]
// ANCHOR: example

// An example trait (with a method with a default implementation):
trait Tr {
    fn foo(&self) {
        println!("foo");
    }
}

enum MyEnum {
    A,
    B,
}

// Implement a trait for an enum:
impl Tr for MyEnum {}

#[repr(C)]
union MyUnion {
    f1: u32,
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

fn main() {
    let e = MyEnum::A;
    e.foo();
    let u = MyUnion { f1: 42 };
    u.foo();
    let s: &[u32] = &[1, 2, 3, 4];
    s.foo();
    let a: [u32; 3] = [1, 2, 3];
    a.foo();
    let t = (1, 2.0);
    t.foo();
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

#[test]
fn test() {
    main();
}
