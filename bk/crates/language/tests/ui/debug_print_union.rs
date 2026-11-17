//! A union cannot derive the book's DebugPrint trait.

#[derive(proc_macros::DebugPrint)]
union Unsupported {
    /// Value stored for this value.
    value: u32,
}

/// Runs the example demonstrated by this module.
fn main() {}
