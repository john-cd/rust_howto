#![allow(dead_code)]
// ANCHOR: example

/// Return an owned type instead of a reference.
fn return_string() -> String {
    String::new()
}

/// Pass a mutable reference,
/// and modify in the function.
fn use_mut_ref(v: &mut Vec<i32>) {
    *v = vec![0; 3];
}

/// Variant: pass a mutable reference,
/// return an immutable reference.
fn use_mut_ref2(v: &mut Vec<i32>) -> &[i32] {
    *v = vec![0; 3];
    &*v
}

/// The following is possible but should be rare:
/// Use a literal.
/// This applies broadly to any item that is composed solely of literals.
fn use_literal() -> &'static str {
    let s: &'static str = "hello";
    s
}

/// Use a static variable.
fn use_static() -> &'static str {
    // Statics have by default a `'static` lifetime.
    static MY_STATIC: &str = "hello";
    MY_STATIC
}

/// Use a constant.
fn use_const() -> &'static str {
    // Constants have by default a `'static` lifetime.
    const MY_CONST: &str = "hello";
    MY_CONST
}

// You might also use `Box::leak`.

fn main() {
    let s = return_string();
    assert!(s.is_empty());

    let mut v = Vec::default();
    use_mut_ref(&mut v);
    println!("{:?}", v);
    assert_eq!(v, vec![0; 3]);

    let m = &mut v;
    println!("{:?}", use_mut_ref2(m));
    assert_eq!(v, vec![0; 3]);

    assert_eq!(use_literal(), "hello");
    assert_eq!(use_static(), "hello");
    assert_eq!(use_const(), "hello");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
