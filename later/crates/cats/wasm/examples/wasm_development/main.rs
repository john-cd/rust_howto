//! Runs the host-side part of the WebAssembly development example.
//!
//! TODO Reviewed ??-??-2026.

mod wasm;

fn main() {
    wasm::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
