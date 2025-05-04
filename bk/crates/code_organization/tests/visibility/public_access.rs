#![allow(dead_code)]
#![allow(unused_imports)]
// ANCHOR: example
mod a_module {
    fn private() {}
    pub fn public() {}

    pub mod public_nested_module {
        fn private() {}
        pub fn public() {}
    }

    mod private_nested_module {
        fn private() {}
        pub fn public() {}
    }
}

fn main() {
    // For an item to be accessible, all elements in the paths
    // must be accessible from the current module.
    a_module::public();

    // Here, `a_module` is accessible, despite being private,
    // because it is in the same module than `main`.

    // No access, if one or more segments in the path are inaccessible.
    // Here, `private` can't be reached from a parent.
    // ERROR: a_module::private();

    a_module::public_nested_module::public();
    // ERROR: a_module::public_nested_module::private();
    // ERROR: a_module::private_nested_module::private();
    // ERROR: a_module::private_nested_module::public();
}

// The same applies when calling from a brother module.
mod another_module {
    fn call() {
        // `super` refers to the parent and is therefore accessible from the
        // child.
        super::a_module::public();
        // ERROR: super::a_module::private();
        super::a_module::public_nested_module::public();
        // ERROR: super::a_module::private_nested_module::public();
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
