//! Removing empty lines from text.
//!
//! TODO Reviewed ??-??-2026.

/// Removes empty lines from a given string slice.
///
/// This function takes a string slice as input, splits it into lines,
/// filters out any lines that are empty or contain only whitespace,
/// and then joins the remaining lines back together with a single newline
/// character between them.
///
/// # Arguments
///
/// * `text` - A string slice (`&str`) from which to remove empty lines.
///
/// # Returns
///
/// A `Vec` of non-empty lines, and a `String` containing the text with all empty lines removed.
///
/// # Example
///
/// ```
/// use tool_lib::remove_empty_lines;
/// let input = "Hello, world!\n\nThis is a test.\n   \nAnother line.";
/// let expected = "Hello, world!\nThis is a test.\nAnother line.";
/// let (_, result) = remove_empty_lines(input);
/// assert_eq!(result, expected);
/// ```
#[must_use]
pub fn remove_empty_lines(text: &str) -> (Vec<String>, String) {
    let lines = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(std::borrow::ToOwned::to_owned)
        .collect::<Vec<String>>();
    (lines.clone(), lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that empty input produces no lines and an empty output string.
    #[test]
    fn test_remove_empty_lines_empty_input() {
        let (lines, result) = remove_empty_lines("");
        assert!(lines.is_empty());
        assert_eq!(result, "");
    }

    /// Verifies that input containing only whitespace produces no output lines.
    #[test]
    fn test_remove_empty_lines_only_whitespace() {
        let (lines, result) = remove_empty_lines("   \n\t\n  \r\n ");
        assert!(lines.is_empty());
        assert_eq!(result, "");
    }

    /// Verifies that non-empty input lines are preserved in order.
    #[test]
    fn test_remove_empty_lines_no_empty_lines() {
        let input = "line 1\nline 2\nline 3";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
        assert_eq!(result, "line 1\nline 2\nline 3");
    }

    /// Verifies that leading, trailing, and interspersed blank lines are removed.
    #[test]
    fn test_remove_empty_lines_leading_trailing_interspersed() {
        let input = "\n  \nHello\n\n\t\nWorld\n   \n";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["Hello", "World"]);
        assert_eq!(result, "Hello\nWorld");
    }

    /// Verifies that indentation and trailing spaces in non-empty lines are preserved.
    #[test]
    fn test_remove_empty_lines_preserves_indentation() {
        let input = "  indented line 1  \n\tindented line 2";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["  indented line 1  ", "\tindented line 2"]);
        assert_eq!(result, "  indented line 1  \n\tindented line 2");
    }

    /// Verifies that CRLF line endings are normalized to LF in the output.
    #[test]
    fn test_remove_empty_lines_crlf() {
        let input = "line 1\r\n\r\nline 2\r\n   \r\nline 3";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
        assert_eq!(result, "line 1\nline 2\nline 3");
    }

    /// Verifies that a single line without a trailing newline is preserved.
    #[test]
    fn test_remove_empty_lines_single_line_no_newline() {
        let input = "single line string";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["single line string"]);
        assert_eq!(result, "single line string");
    }

    /// Verifies that mixed LF and CRLF separators are normalized consistently.
    #[test]
    fn test_remove_empty_lines_mixed_line_endings() {
        let input = "line 1\r\n\nline 2\r\n\r\n   \nline 3";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
        assert_eq!(result, "line 1\nline 2\nline 3");
    }

    /// Verifies that Unicode whitespace-only lines are removed.
    #[test]
    fn test_remove_empty_lines_unicode_whitespace() {
        // \u{00A0} is non-breaking space, \u{2003} is em space, \u{3000} is ideographic space
        let input = "header\n\u{00A0}\n\u{2003}\nbody\n\u{3000}\nfooter";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["header", "body", "footer"]);
        assert_eq!(result, "header\nbody\nfooter");
    }

    /// Verifies that Unicode text remains unchanged while blank lines are removed.
    #[test]
    fn test_remove_empty_lines_unicode_content() {
        let input = "🦀 Rust\n\n  \n🚀 Fast\n   \n✨ Safe";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["🦀 Rust", "🚀 Fast", "✨ Safe"]);
        assert_eq!(result, "🦀 Rust\n🚀 Fast\n✨ Safe");
    }

    /// Verifies that input with only newline characters produces no output lines.
    #[test]
    fn test_remove_empty_lines_only_newlines() {
        let (lines, result) = remove_empty_lines("\n\n\n\r\n\r\n");
        assert!(lines.is_empty());
        assert_eq!(result, "");
    }

    /// Verifies that multiple trailing newlines are stripped and do not affect non-empty content.
    #[test]
    fn test_remove_empty_lines_trailing_newlines() {
        let input = "Hello world\n\n\n\r\n";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["Hello world"]);
        assert_eq!(result, "Hello world");
    }

    /// Verifies that multiple consecutive blank and whitespace-only lines are removed.
    #[test]
    fn test_remove_empty_lines_consecutive_blank_lines() {
        let input = "Header\n\n\n   \n\t\n\nFooter";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["Header", "Footer"]);
        assert_eq!(result, "Header\nFooter");
    }
}
