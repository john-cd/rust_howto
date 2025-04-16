use similar::ChangeTag;
use similar::TextDiff;

/// Detects if two strings are identical by comparing their lines.
///
/// This function uses the `similar` crate to perform a line-by-line comparison
/// of two strings. It returns `true` if the strings are identical (i.e., no
/// differences are found), and `false` otherwise.
///
/// # Arguments
///
/// * `s1` - The first string to compare.
/// * `s2` - The second string to compare.
///
/// # Returns
///
/// `true` if the strings are identical, `false` otherwise.
pub fn detect_diff(s1: &str, s2: &str) -> bool {
    let diff = TextDiff::from_lines(s1, s2);
    diff.ratio() == 1.0
}

/// Prints the differences between two strings, line by line.
///
/// This function uses the `similar` crate to perform a line-by-line comparison
/// of two strings and prints the differences to the standard output. Each line
/// is prefixed with a sign indicating whether it was deleted (`-`), inserted
/// (`+`), or unchanged (` `).
///
/// # Arguments
///
/// * `s1` - The first string to compare.
/// * `s2` - The second string to compare.
pub fn print_diffs(s1: &str, s2: &str) {
    let diff = TextDiff::from_lines(s1, s2);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{}{}", sign, change);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_diff_identical_single_line() {
        let s1 = "hello world";
        let s2 = "hello world";
        assert!(
            detect_diff(s1, s2),
            "Strings should be detected as identical"
        );
    }

    #[test]
    fn test_detect_diff_different_single_line() {
        let s1 = "hello world";
        let s2 = "hello rust";
        assert!(
            !detect_diff(s1, s2),
            "Strings should be detected as different"
        );
    }

    #[test]
    fn test_detect_diff_identical_multi_line() {
        let s1 = "line one\nline two\nline three";
        let s2 = "line one\nline two\nline three";
        assert!(
            detect_diff(s1, s2),
            "Multi-line strings should be detected as identical"
        );
    }

    #[test]
    fn test_detect_diff_different_multi_line_insertion() {
        let s1 = "line one\nline three";
        let s2 = "line one\nline two\nline three";
        assert!(
            !detect_diff(s1, s2),
            "Multi-line strings with insertion should be different"
        );
    }

    #[test]
    fn test_detect_diff_different_multi_line_deletion() {
        let s1 = "line one\nline two\nline three";
        let s2 = "line one\nline three";
        assert!(
            !detect_diff(s1, s2),
            "Multi-line strings with deletion should be different"
        );
    }

    #[test]
    fn test_detect_diff_different_multi_line_modification() {
        let s1 = "line one\nline two\nline three";
        let s2 = "line one\nline 2\nline three";
        assert!(
            !detect_diff(s1, s2),
            "Multi-line strings with modification should be different"
        );
    }

    #[test]
    fn test_detect_diff_empty_strings() {
        let s1 = "";
        let s2 = "";
        assert!(
            detect_diff(s1, s2),
            "Empty strings should be detected as identical"
        );
    }

    #[test]
    fn test_detect_diff_one_empty_string() {
        let s1 = "hello";
        let s2 = "";
        assert!(
            !detect_diff(s1, s2),
            "One empty string should be different from non-empty"
        );

        let s1 = "";
        let s2 = "world";
        assert!(
            !detect_diff(s1, s2),
            "Non-empty string should be different from empty"
        );
    }

    #[test]
    fn test_detect_diff_trailing_newline_identical() {
        let s1 = "hello\n";
        let s2 = "hello\n";
        assert!(
            detect_diff(s1, s2),
            "Strings with identical trailing newlines should be identical"
        );
    }

    #[test]
    fn test_detect_diff_trailing_newline_different() {
        let s1 = "hello";
        let s2 = "hello\n";
        // TextDiff::from_lines treats these as different because the number of lines differs.
        assert!(
            !detect_diff(s1, s2),
            "String without trailing newline should differ from one with"
        );
    }
}
