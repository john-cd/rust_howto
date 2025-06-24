#![allow(dead_code)]
// ANCHOR: example
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let name = "José Guimarães\r\n";
    let graphemes =
        UnicodeSegmentation::graphemes(name, true).collect::<Vec<&str>>();
    println!("{:?}", graphemes);
    assert_eq!(graphemes[3], "é");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
