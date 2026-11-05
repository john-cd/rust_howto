//! Tera filters.
//!
//! TODO Reviewed ??-??-2026.

// Filters are functions with the
// fn(Value, HashMap<String, Value>) -> Result<Value>
// definition
//
// Register with:
// t.register_filter("do_nothing", do_nothing_filter);
// t.register_filter("upper", string::upper);

use std::collections::HashMap;

use tera::Result;
use tera::Value;

/// Replaces all `-` characters with `_` in a string.
///
/// If the input is not a string, it is returned unchanged.
pub fn underscored(val: &Value, _context: &HashMap<String, Value>) -> Result<Value> {
    match val {
        Value::String(s) => Ok(Value::String(s.replace("-", "_"))),
        _ => Ok(val.to_owned()),
    }
}

/// Replaces `-` with `--`, `_` with `__`, and ` ` with `_` in a string.
///
/// If the input is not a string, it is returned unchanged.
pub fn shielded(val: &Value, _context: &HashMap<String, Value>) -> Result<Value> {
    if let Some(v) = val.as_str() {
        Ok(Value::String(
            v.replace("-", "--").replace("_", "__").replace(" ", "_"),
        ))
    } else {
        Ok(val.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the `underscored` filter replaces all hyphens with underscores.
    #[test]
    fn underscored_replaces_hyphens_with_underscores() {
        let input = Value::String("my-crate-name".to_string());
        let context = HashMap::new();
        let result = underscored(&input, &context).expect("underscored filter should succeed");
        assert_eq!(result, Value::String("my_crate_name".to_string()));
    }

    /// Verifies that the `underscored` filter leaves strings without hyphens unchanged.
    #[test]
    fn underscored_leaves_strings_without_hyphens_unchanged() {
        let input = Value::String("already_underscored".to_string());
        let context = HashMap::new();
        let result = underscored(&input, &context).expect("underscored filter should succeed");
        assert_eq!(result, Value::String("already_underscored".to_string()));

        let empty = Value::String(String::new());
        let empty_result = underscored(&empty, &context).expect("empty string should succeed");
        assert_eq!(empty_result, Value::String(String::new()));
    }

    /// Verifies that the `underscored` filter leaves non-string values unchanged.
    #[test]
    fn underscored_ignores_non_string_values() {
        let context = HashMap::new();

        let num = Value::from(42);
        let num_result = underscored(&num, &context).expect("number should succeed");
        assert_eq!(num_result, num);

        let boolean = Value::Bool(true);
        let bool_result = underscored(&boolean, &context).expect("boolean should succeed");
        assert_eq!(bool_result, boolean);

        let null_val = Value::Null;
        let null_result = underscored(&null_val, &context).expect("null should succeed");
        assert_eq!(null_result, null_val);
    }

    /// Verifies that the `shielded` filter escapes hyphens, underscores, and spaces.
    #[test]
    fn shielded_escapes_special_characters_for_shields_io() {
        let input = Value::String("badge-name_with spaces-and_chars".to_string());
        let context = HashMap::new();
        let result = shielded(&input, &context).expect("shielded filter should succeed");
        assert_eq!(
            result,
            Value::String("badge--name__with_spaces--and__chars".to_string())
        );
    }

    /// Verifies that the `shielded` filter leaves strings without special characters unchanged.
    #[test]
    fn shielded_leaves_unaffected_strings_unchanged() {
        let input = Value::String("simplebadge".to_string());
        let context = HashMap::new();
        let result = shielded(&input, &context).expect("shielded filter should succeed");
        assert_eq!(result, Value::String("simplebadge".to_string()));

        let empty = Value::String(String::new());
        let empty_result = shielded(&empty, &context).expect("empty string should succeed");
        assert_eq!(empty_result, Value::String(String::new()));
    }

    /// Verifies that the `shielded` filter leaves non-string values unchanged.
    #[test]
    fn shielded_ignores_non_string_values() {
        let context = HashMap::new();

        let num = Value::from(100);
        let num_result = shielded(&num, &context).expect("number should succeed");
        assert_eq!(num_result, num);

        let boolean = Value::Bool(false);
        let bool_result = shielded(&boolean, &context).expect("boolean should succeed");
        assert_eq!(bool_result, boolean);

        let null_val = Value::Null;
        let null_result = shielded(&null_val, &context).expect("null should succeed");
        assert_eq!(null_result, null_val);
    }
}
