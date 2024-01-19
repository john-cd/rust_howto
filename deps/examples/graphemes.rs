use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let name = "José Guimarães\r\n";
    let graphemes =
        UnicodeSegmentation::graphemes(name, true).collect::<Vec<&str>>();
    assert_eq!(graphemes[3], "é");
}
