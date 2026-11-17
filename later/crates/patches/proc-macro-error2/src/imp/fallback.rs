//! This implementation uses self-written stable facilities.
//!
//! TODO Reviewed ??-??-2026.

use std::cell::RefCell;

use crate::abort_now;
use crate::check_correctness;
use crate::diagnostic::Diagnostic;
use crate::diagnostic::Level;

/// Handles the abort if dirty behavior for this component.
pub fn abort_if_dirty() {
    check_correctness();
    ERR_STORAGE.with(|storage| {
        if !storage.borrow().is_empty() {
            abort_now()
        }
    });
}

/// Releases resources that are no longer needed by the operation.
pub(crate) fn cleanup() -> Vec<Diagnostic> {
    ERR_STORAGE.with(|storage| storage.replace(Vec::new()))
}

/// Handles the emit diagnostic behavior for this component.
pub(crate) fn emit_diagnostic(diag: Diagnostic) {
    if diag.level == Level::Error {
        ERR_STORAGE.with(|storage| storage.borrow_mut().push(diag));
    }
}

thread_local! {

    /// Shared `err storage` used by this module.
    static ERR_STORAGE: RefCell<Vec<Diagnostic>> = const { RefCell::new(Vec::new()) };
}
