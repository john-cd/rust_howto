#![allow(dead_code)]
#![allow(unused_imports)]
// ANCHOR: example
fn private_in_parent() {}

mod private_module {
    fn also_private() {
        // `private_in_parent` is accessible,
        // since it is in a parent module.
        super::private_in_parent();
    }
    mod submodule {}
}

fn main() {
    // `private_in_parent` is accessible, since it is
    // in the same module than `main`.
    private_in_parent();

    // `private_module` is accessible as well,
    // because it is also in the same module...
    use private_module as alias;

    // ...but its private contents are not!
    // ERROR: private_module::also_private();
    // ERROR: alias::also_private();
    // ERROR: use private_module::submodule;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
