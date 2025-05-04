// ANCHOR: example
// Define a module inline.
// Use the path syntax `module_name::item_name` to access items within.
mod my_module {
    // Items defined inside the module are private by default.
    fn private_function() {
        println!("Called private_function inside my_module.");
    }
    // Use `pub` to make items visible outside the module.
    pub fn public_function() {
        println!("Called public_function inside my_module.");
        private_function(); // One can use / call private items within the same module.
    }

    // Define a nested module.
    // Nested modules are private by default.
    // We make both the module and its inner function public to make the latter
    // accessible.
    pub mod nested_public_module {
        pub fn nested_public() {
            println!("Called nested_public inside nested_module.");
        }
    }

    // Private nested module.
    mod nested_private_module {
        #[allow(dead_code)]
        pub fn nested_public_in_private_module() {}
    }

    // This function calls another in its parent module.
    pub fn call_func_in_parent_module() {
        // `super` in a path refers to the parent module.
        super::private_in_parent_module();
    }
}

// This (private) function can be called from child modules.
fn private_in_parent_module() {
    println!("Called private_in_parent_module.");
}

fn main() {
    // Call a public function within a child module.
    // Note the relative path syntax: `<module_name>::<item_name>`.
    my_module::public_function();

    // my_module::private_function(); // ERROR: private_function is private.

    // Paths can include multiple segments to reach nested modules.
    my_module::nested_public_module::nested_public();

    // Note that both the nested module and its inner function had to be public
    // to be reachable:
    // my_module::nested_private_module::nested_public();
    // ERROR: module `nested_private_module` is private.

    // `call_func_in_parent_module` calls a (private) function
    // in its parent module.
    my_module::call_func_in_parent_module();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
