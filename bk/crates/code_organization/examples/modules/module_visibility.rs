#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Visibility of items in modules.

// This file is an (implicit) module.

// A private function in this top module.
/// A private function in the top-level module.
fn private_in_top_module() {
    println!("Called private_in_top_module.");
}

// Let's define an inner module inline with the `mod` keyword.
// Modules are private by default.
/// A private module.
mod my_module {
    // Items defined inside the module are also private by default.
    /// A private function; visible in this module and its descendants.
    fn private_function() {
        println!("Called private_function inside my_module.");
    }

    // Use `pub` to make items public.
    /// A public function.
    pub fn public_function() {
        println!("Called public_function inside my_module.");
    }

    // Define a public nested module with the `pub` keyword.
    // We make both the module and its inner function public.
    /// A public nested module.
    pub mod nested_public_module {
        /// A public function of a public nested module.
        pub fn nested_public() {
            println!("Called nested_public inside nested_module.");
        }
    }

    // Private nested module.
    /// A private nested module.
    mod nested_private_module {
        /// A public function of a private module; not reachable from outside
        /// it.
        #[allow(dead_code)]
        pub fn nested_public_in_private_module() {}
    }

    // This function calls another in its parent module.
    // This is always allowed, even if private.
    /// Calls a private function of the parent module.
    pub fn call_func_in_parent_module() {
        // `super` in a path refers to the parent module.
        super::private_in_top_module();
    }
}

/// Calls items of the modules, according to their visibility.
fn main() {
    // Items can access other items in the _same_ module, even if private.
    private_in_top_module();

    // In the same way, `my_module` is _private_, but _accessible_ from
    // `main()`. Here, we define an alias:
    use my_module as alias;

    // Private items are inaccessible from a parent module.
    // my_module::private_function(); // ERROR: private_function is private.

    // Call a public function within a child module.
    // Note the relative path syntax: `<module_name>::<item_name>` to access
    // items within a module.
    my_module::public_function();

    // We can also use the alias:
    alias::public_function();

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

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
