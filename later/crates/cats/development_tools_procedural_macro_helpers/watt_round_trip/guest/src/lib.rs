//! Implements a procedural macro in the Watt guest.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

/// Wraps the input token stream in parentheses.
#[unsafe(no_mangle)]
pub extern "C" fn parenthesize(input: TokenStream) -> TokenStream {
    TokenTree::Group(Group::new(Delimiter::Parenthesis, input)).into()
}
// ANCHOR_END: example
