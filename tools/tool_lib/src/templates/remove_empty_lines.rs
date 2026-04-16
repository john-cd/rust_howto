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
pub fn remove_empty_lines(text: &str) -> (Vec<String>, String) {
    let lines = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|str| str.to_string())
        .collect::<Vec<String>>();
    (lines.clone(), lines.join("\n"))
}
