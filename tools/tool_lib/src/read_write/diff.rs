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
// [unit tests](https://github.com/john-cd/rust_howto/issues/1370)
