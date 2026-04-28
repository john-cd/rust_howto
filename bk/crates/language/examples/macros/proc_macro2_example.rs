#![allow(unused)]
// ANCHOR: example
// Import our custom procedural macro that was built using `proc-macro2`.
use proc_macros::replace_ident;

fn main() {
    // The `replace_ident` macro replaces any occurrence of `old_ident`
    // with `new_ident` inside its token stream.
    replace_ident! {
        let old_ident = "Hello, proc-macro2!";
        println!("{new_ident}");
        assert_eq!(new_ident, "Hello, proc-macro2!");
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
