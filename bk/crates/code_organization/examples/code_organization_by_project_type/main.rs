// ANCHOR: example
//! Library and binary code organization with modules and a prelude.

/// A library module grouping domain types and functions.
pub mod a_module {
    /// A sample configuration or service type.
    #[derive(Debug, PartialEq, Eq)]
    pub struct SuperCommonType {
        /// Identifier for the common type.
        pub name: &'static str,
    }

    /// A sample library function.
    pub fn super_common_function() -> &'static str {
        "common functionality"
    }
}

/// A prelude module re-exporting the most frequently used items.
pub mod prelude {
    pub use super::a_module::SuperCommonType;
    pub use super::a_module::super_common_function;
}

// In client or binary code, import everything from the prelude:
use prelude::*;

/// Calls re-exported functions and constructs types from the prelude.
fn main() {
    let item = SuperCommonType { name: "demo" };
    println!("{}: {}", item.name, super_common_function());
}

// ANCHOR_END: example

/// Runs the code organization by project type example.
pub fn run() {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the code organization example runs through its entry point.
    #[test]
    fn test_code_organization_by_project_type_entry_point() {
        let entry = main;
        entry();
    }

    /// Verifies that items exported through the prelude are directly
    /// accessible.
    #[test]
    fn test_prelude_exports() {
        let item = SuperCommonType { name: "test" };
        assert_eq!(item.name, "test");
        assert_eq!(super_common_function(), "common functionality");
    }
}
