// TODO Reviewed ??-??-2026.
//! An external module nested under an inline parent.

/// Returns the value defined in the nested external module.
pub(in crate::module_forms) fn value() -> u8 {
    3
}
