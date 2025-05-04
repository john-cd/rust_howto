#![allow(unused_imports)]
// ANCHOR: example
mod a_module {
    mod private_module {
        pub fn public_function() {}
    }

    // The following `use` declaration brings `private_module::public_function`
    // into scope, with the `public_function` name.
    // Since `use` is preceded by `pub`, the `public_function` name is public.
    pub use private_module::public_function;
    // You can rename the item that is re-exported:
    pub use private_module::public_function as another_name;
}

fn main() {
    // `private_module` is... private, therefore the inner function is not
    // accessible.
    // ERROR: a_module::private_module::public_function();

    // However, the re-exported name is accessible.
    a_module::public_function();

    // Access the re-exported item via its alternate name.
    a_module::another_name();

    // Notes:
    // - Re-exports hide implementation details and flatten the module
    //   hierarchy. `public_function` appears as if it were declared in
    //   `a_module`.
    // - `a_module`, while private, is accessible, because `main` is in the same
    //   module.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
