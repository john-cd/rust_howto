use similar::ChangeTag;
use similar::TextDiff;

pub fn detect_diff(s1: &str, s2: &str) -> bool {
    let diff = TextDiff::from_lines(s1, s2);
    diff.ratio() == 1.0
}

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
