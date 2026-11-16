//! A union cannot derive the book's DebugPrint trait.

#[derive(proc_macros::DebugPrint)]
union Unsupported {
    value: u32,
}

fn main() {}
