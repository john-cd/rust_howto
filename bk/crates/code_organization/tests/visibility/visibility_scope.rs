// ANCHOR: example
mod a {
    pub mod b {
        pub(crate) fn visible_in_crate() {}
        pub(super) fn visible_in_parent_module() {}
        pub mod c {
            pub(in super::super) fn visible_in_a() {}
        }
    }

    #[allow(dead_code)]
    fn use_in_a() {
        b::visible_in_crate();
        b::visible_in_parent_module();
        b::c::visible_in_a();
    }
}

mod d {
    #[allow(dead_code)]
    fn use_in_d() {
        super::a::b::visible_in_crate();
        // ERROR: super::a::b::visible_in_parent_module();
        // ERROR: super::a::b::c::visible_in_a();
    }
}

fn main() {
    a::b::visible_in_crate();
    // ERROR: a::b::visible_in_parent_module();
    // ERROR: a::b::c::visible_in_a();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
