#![allow(dead_code)]
// ANCHOR: example
// In e.g. `lib.rs`:
mod a_module {
    // Conservative use of `pub(crate)`:
    // Even if `a_module` is accidentally made fully public,
    // this function will remain visible only to this crate.
    pub(crate) fn visible_in_crate_only() {}
}

// We can also explictly mark the top module as `pub(crate)`:
pub(crate) mod b_module {
    pub fn visible_in_crate_only() {}
}

fn main() {
    a_module::visible_in_crate_only();
    b_module::visible_in_crate_only();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
