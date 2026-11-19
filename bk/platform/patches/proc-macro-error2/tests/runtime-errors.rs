// TODO Reviewed ??-??-2026.

use proc_macro_error2::*;

/// Panics with the expected message when an error is emitted outside macro expansion.
#[test]
#[should_panic = "proc-macro-error2 API cannot be used outside of"]
fn missing_attr_emit() {
    emit_call_site_error!("You won't see me");
}

/// Panics with the expected message when expansion is aborted outside a macro.
#[test]
#[should_panic = "proc-macro-error2 API cannot be used outside of"]
fn missing_attr_abort() {
    abort_call_site!("You won't see me");
}
