// TODO Reviewed ??-??-2026
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

    #[test]
    fn test_remove_empty_lines_empty_input() {
        let (lines, result) = remove_empty_lines("");
        assert!(lines.is_empty());
        assert_eq!(result, "");
    }

    #[test]
    fn test_remove_empty_lines_only_whitespace() {
        let (lines, result) = remove_empty_lines("   \n\t\n  \r\n ");
        assert!(lines.is_empty());
        assert_eq!(result, "");
    }

    #[test]
    fn test_remove_empty_lines_no_empty_lines() {
        let input = "line 1\nline 2\nline 3";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
        assert_eq!(result, "line 1\nline 2\nline 3");
    }

    #[test]
    fn test_remove_empty_lines_leading_trailing_interspersed() {
        let input = "\n  \nHello\n\n\t\nWorld\n   \n";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["Hello", "World"]);
        assert_eq!(result, "Hello\nWorld");
    }

    #[test]
    fn test_remove_empty_lines_preserves_indentation() {
        let input = "  indented line 1  \n\tindented line 2";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["  indented line 1  ", "\tindented line 2"]);
        assert_eq!(result, "  indented line 1  \n\tindented line 2");
    }

    #[test]
    fn test_remove_empty_lines_crlf() {
        let input = "line 1\r\n\r\nline 2\r\n   \r\nline 3";
        let (lines, result) = remove_empty_lines(input);
        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
        assert_eq!(result, "line 1\nline 2\nline 3");
    }
}
